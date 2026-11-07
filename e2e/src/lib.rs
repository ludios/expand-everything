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
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;
use userscript::{HARNESS_GLOBAL, Userscript};
use zendriver::{Browser, ObserverError, PausedSession, Tab, TargetObserver};

/// How long to wait between two evaluations in [`wait_for`].
const POLL_INTERVAL: Duration = Duration::from_millis(250);

/// How long [`open`] waits for the userscript to start running.
const START_TIMEOUT: Duration = Duration::from_secs(30);

/// Registers the scripts made by [`Userscript`] on each new page and iframe
/// target, before the target runs anything, so that they run in every
/// document that the target loads.
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
///
/// Chrome is headless unless the environment variable `E2E_HEADFUL` is set.
/// It asks sites for English (US), because the userscript and the tests look
/// for some buttons by their English text.
///
/// Returns what `work` returns, or the error from loading the userscript or
/// launching Chrome. An error from closing Chrome is only printed, because it
/// doesn't make `work`'s result wrong, and because zendriver kills Chrome's
/// process once the `Browser` is dropped.
pub async fn with_chrome<T>(work: impl AsyncFnOnce(&Browser) -> Result<T>) -> Result<T> {
    let userscript = Userscript::load()?;
    let observer = InjectUserscript {
        scripts: [userscript.harness_script(), userscript.wrapped_script()],
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
/// start running in it.
///
/// Returns the tab, or an error if the page doesn't load, no `@match` line
/// allows the page's URL, the scripts were injected more than once, or the
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
            "the scripts were injected more than once into {}: {state:?}",
            state.url
        );
    }
    Ok(tab)
}
