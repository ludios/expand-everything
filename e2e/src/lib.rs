//! A harness for testing `expand.user.js` against live sites, in Chrome driven
//! by zendriver.
//!
//! Chrome gets the userscript injected into every document (see
//! [`userscript`]), as if a userscript manager had it installed. A test opens
//! a page with [`open`], and waits with [`wait_for`] until the page shows what
//! the userscript should have expanded, and [`userscript::CLICKED_TEXTS`]
//! shows that it was the userscript that clicked.

pub mod userscript;

use anyhow::{Context, Result, bail};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::fmt::Debug;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;
use userscript::{HARNESS_GLOBAL, Start, Userscript};
use zendriver::{Browser, Cookie, ObserverError, PausedSession, Tab, TargetObserver};

/// How long to wait between two evaluations in [`wait_for`].
const POLL_INTERVAL: Duration = Duration::from_millis(250);

/// How long [`open`] waits for the userscript to start running.
const START_TIMEOUT: Duration = Duration::from_secs(30);

/// Registers the scripts made by [`Userscript`] on each page and iframe
/// target once zendriver attaches to it, so that they run in every document
/// that the target loads after that. A target that Chrome creates after
/// launch is paused until then; the main tab already has its first document
/// (`about:blank`), but zendriver's `launch` returns only after this
/// registration, so every page that a test loads gets the scripts.
#[derive(Debug)]
struct InjectUserscript {
    /// The scripts, in the order in which they run in each document.
    scripts: [String; 2],
}

#[async_trait::async_trait]
impl TargetObserver for InjectUserscript {
    fn name(&self) -> &'static str {
        "expand-everything-userscript"
    }

    async fn on_target_attached(&self, session: PausedSession<'_>) -> Result<(), ObserverError> {
        if !matches!(session.target_info.kind.as_str(), "page" | "iframe") {
            return Ok(());
        }
        session.call("Page.enable", json!({})).await?;
        for source in &self.scripts {
            session
                .call(
                    "Page.addScriptToEvaluateOnNewDocument",
                    json!({ "source": source }),
                )
                .await?;
        }
        Ok(())
    }
}

/// Launch Chrome with `expand.user.js` injected into every document (where
/// the userscript starts when `start` says), run `work` with it, and then
/// close Chrome, even if `work` returned an error.
///
/// Chrome is headless unless the environment variable `E2E_HEADFUL` is set.
/// It asks sites for English (US), because the userscript and the tests look
/// for some buttons by their English text.
///
/// Returns what `work` returns, or the error from loading the userscript or
/// launching Chrome. An error from closing Chrome is only printed, because it
/// doesn't make `work`'s result wrong, and because zendriver kills Chrome's
/// process once the `Browser` is dropped.
pub async fn with_chrome_starting<T>(
    start: Start,
    work: impl AsyncFnOnce(&Browser) -> Result<T>,
) -> Result<T> {
    let userscript = Userscript::load()?;
    let observer = InjectUserscript {
        scripts: [
            userscript.harness_script(start),
            userscript.wrapped_script(),
        ],
    };
    let browser = Browser::builder()
        .headless(std::env::var_os("E2E_HEADFUL").is_none())
        .lang("en-US")
        .observer(Arc::new(observer))
        .launch()
        .await
        .context("launching Chrome")?;
    let result = work(&browser).await;
    if let Err(error) = browser.close().await {
        eprintln!("closing Chrome failed: {error}");
    }
    result
}

/// [`with_chrome_starting`], with the userscript starting once the DOM has
/// been parsed, as it does in Violentmonkey.
pub async fn with_chrome<T>(work: impl AsyncFnOnce(&Browser) -> Result<T>) -> Result<T> {
    with_chrome_starting(Start::DocumentEnd, work).await
}

/// The cookies in `text`, the contents of a cookies.txt file in the Netscape
/// format, which curl and browser extensions write. Each line that isn't
/// blank and doesn't start with `#` is a cookie, as seven tab-separated
/// fields: domain, whether subdomains get the cookie too (`TRUE` or
/// `FALSE`), path, whether only HTTPS requests get it (`TRUE` or `FALSE`),
/// when it expires (in seconds since the Unix epoch, or 0 for a cookie that
/// expires once the browser closes), name, and value. A line that starts with
/// `#HttpOnly_` is a cookie too, one that JavaScript can't read.
///
/// Returns an error that names the first line that isn't a comment, a blank
/// line, or a cookie.
fn parse_cookies_txt(text: &str) -> Result<Vec<Cookie>> {
    let flag = |field: &str| match field {
        "TRUE" => Ok(true),
        "FALSE" => Ok(false),
        _ => bail!("{field:?} is neither TRUE nor FALSE"),
    };
    let mut cookies = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let mut http_only = false;
        let mut line = line;
        if let Some(rest) = line.strip_prefix("#HttpOnly_") {
            http_only = true;
            line = rest;
        } else if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let parse = || -> Result<Cookie> {
            let fields: Vec<&str> = line.split('\t').collect();
            let [domain, subdomains, path, secure, expires, name, value] = fields[..] else {
                bail!("it has {} tab-separated fields instead of 7", fields.len());
            };
            let domain = domain.trim_start_matches('.');
            let expires: f64 = expires
                .parse()
                .ok()
                .filter(|e: &f64| e.is_finite() && *e >= 0.0)
                .with_context(|| format!("the expiry {expires:?} is neither 0 nor a time"))?;
            Ok(Cookie {
                name: name.to_string(),
                value: value.to_string(),
                // Chrome gives a cookie whose domain starts with "." to the
                // domain's subdomains too, and any other cookie only to the
                // host that is its domain.
                domain: if flag(subdomains)? {
                    format!(".{domain}")
                } else {
                    domain.to_string()
                },
                path: path.to_string(),
                expires: (expires != 0.0).then_some(expires),
                http_only,
                secure: flag(secure)?,
                ..Default::default()
            })
        };
        cookies.push(parse().with_context(|| format!("line {} isn't a cookie", index + 1))?);
    }
    Ok(cookies)
}

/// Give `browser` the cookies in the cookies.txt file at `path` (see
/// [`parse_cookies_txt`]), such as to load pages as a user who is logged in.
///
/// Returns an error if the file can't be read or parsed, or if Chrome
/// refuses a cookie.
pub async fn add_cookies(browser: &Browser, path: &Path) -> Result<()> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let cookies =
        parse_cookies_txt(&text).with_context(|| format!("parsing {}", path.display()))?;
    browser
        .cookies()
        .set_many(cookies)
        .await
        .with_context(|| format!("giving Chrome the cookies in {}", path.display()))
}

/// Poll the JavaScript `expression` in the main world of the page in `tab`
/// until `done` has returned true for every value of `expression` during one
/// uninterrupted interval of length `settle` (so that a value that is only
/// briefly right, such as a page between two loads of comments, doesn't end
/// the wait).
///
/// `what` describes what is awaited, for the error message.
///
/// Returns the last value of `expression`, or an error that shows the last
/// value that an evaluation of `expression` gave (or the error that it gave) if `timeout`
/// elapses first.
pub async fn wait_for<T: DeserializeOwned + Debug>(
    tab: &Tab,
    what: &str,
    expression: &str,
    timeout: Duration,
    settle: Duration,
    done: impl Fn(&T) -> bool,
) -> Result<T> {
    let deadline = Instant::now() + timeout;
    let mut done_since = None;
    let mut last = None;
    loop {
        // An evaluation that is still running at the deadline is abandoned.
        let Ok(value) = tokio::time::timeout_at(deadline, tab.evaluate_main::<T>(expression)).await
        else {
            break;
        };
        let now = Instant::now();
        match value {
            Ok(v) if done(&v) => {
                let since = *done_since.get_or_insert(now);
                if now - since >= settle {
                    return Ok(v);
                }
                last = Some(Ok(v));
            }
            other => {
                done_since = None;
                last = Some(other);
            }
        }
        if now >= deadline {
            break;
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
    let last = match last {
        Some(Ok(value)) => format!("{value:?}"),
        Some(Err(error)) => format!("an error: {error}"),
        None => "missing, because no evaluation finished".to_string(),
    };
    let settling = match done_since {
        Some(since) => format!(
            " (which was right, but only for {:?} of {settle:?})",
            Instant::now() - since
        ),
        None => String::new(),
    };
    bail!("waited {timeout:?} for {what}, but the last value was {last}{settling}")
}

/// Load `url` in the main tab of `browser`, and wait for the userscript to
/// start running in it. Every call uses the same tab, so a second call
/// replaces the page of the first.
///
/// Returns the tab, or an error if the page doesn't load, no `@match` line
/// allows the page's URL, the harness script didn't run exactly once, or the
/// userscript doesn't start within `START_TIMEOUT` or throws while starting.
pub async fn open(browser: &Browser, url: &str) -> Result<Tab> {
    let tab = browser.main_tab();
    tab.goto(url)
        .await
        .with_context(|| format!("loading {url}"))?;
    let expression = format!(
        "(({{ injections, matched, ran, error }}) => ({{ injections, matched, ran, error, url: location.href, title: document.title, ready_state: document.readyState }}))(window.{HARNESS_GLOBAL} ?? {{}})"
    );
    #[derive(Debug, serde::Deserialize)]
    struct State {
        injections: Option<u32>,
        matched: Option<bool>,
        ran: Option<bool>,
        error: Option<String>,
        url: String,
        #[allow(dead_code, reason = "shown in the error message")]
        title: String,
        #[allow(dead_code, reason = "shown in the error message")]
        ready_state: String,
    }
    let state = wait_for(
        &tab,
        "the userscript to start",
        &expression,
        START_TIMEOUT,
        Duration::ZERO,
        |s: &State| s.ran == Some(true) || s.matched == Some(false),
    )
    .await?;
    if state.matched == Some(false) {
        bail!("no @match line in the userscript allows {}", state.url);
    }
    if let Some(error) = &state.error {
        bail!(
            "the userscript threw while starting in {}: {error}",
            state.url
        );
    }
    if state.injections != Some(1) {
        bail!(
            "the harness script didn't run exactly once in {} (did zendriver attach to the tab \
             more than once?): {state:?}",
            state.url
        );
    }
    Ok(tab)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cookies_txt_parses() {
        let text = "# Netscape HTTP Cookie File\n\
            \n\
            .example.com\tTRUE\t/\tTRUE\t1825613861\ta\t1\n\
            #HttpOnly_example.com\tFALSE\t/x\tFALSE\t0\t__Host-b\tc d\n";
        let cookies = parse_cookies_txt(text).unwrap();
        assert_eq!(
            cookies,
            [
                Cookie {
                    name: "a".into(),
                    value: "1".into(),
                    domain: ".example.com".into(),
                    path: "/".into(),
                    expires: Some(1825613861.0),
                    http_only: false,
                    secure: true,
                    ..Default::default()
                },
                Cookie {
                    name: "__Host-b".into(),
                    value: "c d".into(),
                    domain: "example.com".into(),
                    path: "/x".into(),
                    expires: None,
                    http_only: true,
                    secure: false,
                    ..Default::default()
                },
            ]
        );
    }

    #[test]
    fn malformed_cookies_txt_is_an_error() {
        for text in [
            "example.com\tFALSE\t/\tFALSE\t0\tname\n",
            "example.com\tYES\t/\tFALSE\t0\tname\tvalue\n",
            "example.com\tFALSE\t/\tFALSE\tnever\tname\tvalue\n",
            "example.com\tFALSE\t/\tFALSE\t-2\tname\tvalue\n",
            "example.com\tFALSE\t/\tFALSE\tinf\tname\tvalue\n",
        ] {
            let error = parse_cookies_txt(text).unwrap_err();
            assert!(
                format!("{error:#}").starts_with("line 1 isn't a cookie"),
                "{text:?}"
            );
        }
    }
}
