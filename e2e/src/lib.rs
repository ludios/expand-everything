//! A harness for testing `expand.user.js` against live sites, in Chrome driven
//! by zendriver.
//!
//! Chrome gets the userscript injected into every document (see
//! [`userscript`]), as if a userscript manager had it installed. A test opens
//! a page with [`open`], and waits with [`wait_for`] until the page shows what
//! the userscript should have expanded, and [`userscript::CLICKED_TEXTS`]
//! shows that it was the userscript that clicked. A test of a page that a
//! site shows only to a visitor who is logged in uses [`with_login`].

pub mod userscript;

use anyhow::{Context, Result, bail};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;
use userscript::{HARNESS_GLOBAL, Start, Userscript};
use zendriver::{Browser, Cookie, ObserverError, PausedSession, Persona, Tab, TargetObserver};

/// How long to wait between two evaluations in [`wait_for`].
const POLL_INTERVAL: Duration = Duration::from_millis(250);

/// How long [`open`] waits for the userscript to start running.
const START_TIMEOUT: Duration = Duration::from_secs(30);

/// How long [`with_login`] waits for the page to show the session that the
/// test needs. A site may first serve a challenge page, which then loads the
/// page itself.
const LOGIN_TIMEOUT: Duration = Duration::from_secs(30);

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

/// Launch Chrome with `expand.user.js` injected into every document, run
/// `work` with it, and then close Chrome, even if `work` returned an error.
/// In each document, the userscript starts at the time that `start` names.
///
/// Chrome is headless unless the environment variable `E2E_HEADFUL` is set.
/// It asks sites for English (US), because the userscript and the tests look
/// for some buttons by their English text. So that a site doesn't see the
/// timezone contradict the IP address, zendriver asks ip-api.com for the
/// timezone of the IP address that connects to ip-api.com, and Chrome reports
/// that timezone; if that request fails, zendriver only logs a warning, and
/// Chrome reports the machine's timezone.
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
        // Without a locale here, `geo_auto` would also set the language of
        // the IP address's country.
        .persona_overlay(Persona::builder().locale("en-US").build())
        .geo_auto()
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

/// The cookies.txt file (see [`parse_cookies_txt`]) of a session on `site`,
/// in the cookies directory `dir`: `dir/site/cookies.txt`, or
/// `dir/site/ACCOUNT/cookies.txt` for any name ACCOUNT.
///
/// Returns an error unless exactly one such file exists, and `site` is a
/// name made of ASCII letters, digits, and `-`.
fn find_cookies_txt(dir: &Path, site: &str) -> Result<PathBuf> {
    // Any other name could be a path that leads out of `dir`.
    let name_like = |c: char| c.is_ascii_alphanumeric() || c == '-';
    if site.is_empty() || !site.chars().all(name_like) {
        bail!("{site:?} isn't the name of a site's directory in the cookies directory");
    }
    let site_dir = dir.join(site);
    let entries = std::fs::read_dir(&site_dir).with_context(|| {
        format!(
            "listing {}, which must have the cookies of a session on {site}, as cookies.txt or \
             ACCOUNT/cookies.txt",
            site_dir.display()
        )
    })?;
    let mut found = Vec::new();
    if site_dir.join("cookies.txt").is_file() {
        found.push(site_dir.join("cookies.txt"));
    }
    for entry in entries {
        let file = entry?.path().join("cookies.txt");
        if file.is_file() {
            found.push(file);
        }
    }
    if found.len() > 1 {
        found.sort();
        bail!(
            "{} has more than one cookies.txt file, so which session to use is unknown: {found:?}",
            site_dir.display()
        );
    }
    let Some(file) = found.pop() else {
        bail!(
            "{0} has neither cookies.txt nor ACCOUNT/cookies.txt",
            site_dir.display()
        );
    };
    Ok(file)
}

/// The cookies.txt file (in the Netscape format) of a session on `site`,
/// such as `"github"`: `site/cookies.txt`, or `site/ACCOUNT/cookies.txt` for
/// any name ACCOUNT, in the cookies directory. The cookies directory is
/// `~/.cache/cookies`, unless the environment variable `E2E_COOKIES_DIR` is
/// set, which then names it.
///
/// Returns an error unless the cookies directory has exactly one such file,
/// and `site` is a name made of ASCII letters, digits, and `-`.
pub fn cookies_txt(site: &str) -> Result<PathBuf> {
    let mut dir = None;
    if let Some(named) = std::env::var_os("E2E_COOKIES_DIR") {
        dir = Some(PathBuf::from(named));
    } else if let Some(home) = std::env::var_os("HOME") {
        dir = Some(Path::new(&home).join(".cache/cookies"));
    }
    let dir = dir.context("neither E2E_COOKIES_DIR nor HOME is set")?;
    find_cookies_txt(&dir, site).with_context(|| {
        format!(
            "finding the cookies to log in with, in the cookies directory {} (which is \
             ~/.cache/cookies, unless the environment variable E2E_COOKIES_DIR names another)",
            dir.display()
        )
    })
}

/// Launch Chrome as [`with_chrome`] does, give it the cookies of a session
/// on `site` (see [`cookies_txt`]), load `url` in it with [`open`], wait
/// until the page shows the session that `work` needs, and run `work` with
/// the tab.
///
/// `logged_in` is a JavaScript expression, evaluated in the page's main
/// world, that is truthy once the page shows that session: that the cookies
/// log in, and whatever else `work` needs of their account, such as a
/// membership.
///
/// Returns what `work` returns, or an error if the cookies aren't found or
/// can't be given to Chrome, `url` doesn't open, or `logged_in` isn't truthy
/// within `LOGIN_TIMEOUT`.
pub async fn with_login<T>(
    site: &str,
    url: &str,
    logged_in: &str,
    work: impl AsyncFnOnce(&Tab) -> Result<T>,
) -> Result<T> {
    #[derive(Debug, serde::Deserialize)]
    struct Page {
        logged_in: bool,
        #[allow(dead_code, reason = "shown in the error message")]
        url: String,
        #[allow(dead_code, reason = "shown in the error message")]
        title: String,
    }
    let cookies = cookies_txt(site)?;
    with_chrome(async |browser| {
        add_cookies(browser, &cookies).await?;
        let tab = open(browser, url).await.with_context(|| {
            format!(
                "opening {url} with the cookies in {} (if no @match line allows the page, the \
                 site may have redirected to a sign-in page, because the cookies don't log in)",
                cookies.display()
            )
        })?;
        wait_for(
            &tab,
            &format!(
                "`{logged_in}` to be truthy in the page (if it never is, the cookies in {} don't \
                 log in, or their account lacks something that the test needs)",
                cookies.display()
            ),
            &format!(
                "({{ logged_in: Boolean(({logged_in})), url: location.href, title: document.title }})"
            ),
            LOGIN_TIMEOUT,
            Duration::ZERO,
            |page: &Page| page.logged_in,
        )
        .await?;
        work(&tab).await
    })
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Make a new directory for the test named `test` (which no other test
    /// of this process may name), with the files at `files` (paths in the
    /// directory) in it, and return its path.
    fn cookies_dir_with(test: &str, files: &[&str]) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("e2e-{test}-{}", std::process::id()));
        // An earlier process with the same id may have left the directory
        // behind.
        let _ = std::fs::remove_dir_all(&dir);
        for file in files {
            let path = dir.join(file);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, "").unwrap();
        }
        dir
    }

    #[test]
    fn cookies_txt_of_a_site_is_found() {
        let files = ["a/someone/cookies.txt", "b/cookies.txt", "b/notes.txt"];
        let dir = cookies_dir_with("found", &files);
        assert_eq!(find_cookies_txt(&dir, "a").unwrap(), dir.join(files[0]));
        assert_eq!(find_cookies_txt(&dir, "b").unwrap(), dir.join(files[1]));
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn missing_or_ambiguous_cookies_txt_is_an_error() {
        let files = [
            "two/x/cookies.txt",
            "two/y/cookies.txt",
            "both/cookies.txt",
            "both/x/cookies.txt",
            "none/x/other.txt",
        ];
        let dir = cookies_dir_with("errors", &files);
        for (site, expected) in [
            ("two", "has more than one cookies.txt file"),
            ("both", "has more than one cookies.txt file"),
            ("none", "has neither cookies.txt nor ACCOUNT/cookies.txt"),
            ("absent", "listing "),
            ("", "isn't the name of a site's directory"),
            ("two/x", "isn't the name of a site's directory"),
            ("..", "isn't the name of a site's directory"),
        ] {
            let error = format!("{:#}", find_cookies_txt(&dir, site).unwrap_err());
            assert!(error.contains(expected), "{site}: {error}");
        }
        std::fs::remove_dir_all(dir).unwrap();
    }

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
