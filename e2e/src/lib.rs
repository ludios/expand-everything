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
/// until `done` returns true for each of its values for `settle` (so that a
/// value that is only briefly right, such as a page between two loads of
/// comments, doesn't end the wait).
///
/// `what` describes what is awaited, for the error message.
///
/// Returns the last value of `expression`, or an error that shows the last
/// value (or the last error from evaluating `expression`) if `timeout` passed
/// first.
pub async fn wait_for<T: DeserializeOwned + Debug>(
    tab: &Tab,
    what: &str,
    expression: &str,
    timeout: Duration,
    settle: Duration,
    done: impl Fn(&T) -> bool,
) -> Result<T> {
    let start = Instant::now();
    let mut done_since = None;
    loop {
        let value = tab.evaluate_main::<T>(expression).await;
        let now = Instant::now();
        match &value {
            Ok(v) if done(v) => {
                let since = *done_since.get_or_insert(now);
                if now - since >= settle {
                    return Ok(value?);
                }
            }
            _ => done_since = None,
        }
        if now - start >= timeout {
            bail!("waited {timeout:?} for {what}, but the last value was {value:?}");
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

/// Load `url` in a new tab of `browser`, and wait for the userscript to
/// start running in it.
///
/// Not the main tab, which zendriver attaches to twice: the scripts are then
/// injected twice into each document, and the second attachment's injections
/// race with the first navigation. (The new tab is attached once, and paused
/// until the scripts are registered.)
///
/// Returns the tab, or an error if the page doesn't load, no `@match` line
/// allows the page's URL, the scripts were injected more than once, or the
/// userscript doesn't start within `START_TIMEOUT`.
pub async fn open(browser: &Browser, url: &str) -> Result<Tab> {
    let tab = browser.new_tab().await.context("opening a tab")?;
    tab.goto(url)
        .await
        .with_context(|| format!("loading {url}"))?;
    let expression = format!(
        "(({{ injections, matched, ran }}) => ({{ injections, matched, ran, url: location.href, title: document.title, ready_state: document.readyState }}))(window.{HARNESS_GLOBAL} ?? {{}})"
    );
    #[derive(Debug, serde::Deserialize)]
    struct State {
        injections: Option<u32>,
        matched: Option<bool>,
        ran: Option<bool>,
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
    if state.injections != Some(1) {
        bail!(
            "the scripts were injected more than once into {}: {state:?}",
            state.url
        );
    }
    Ok(tab)
}
