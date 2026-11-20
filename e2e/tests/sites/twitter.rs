use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{wait_for, with_login};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::time::Duration;

/// A post whose conversation has 6 replies (to other replies) that X hides,
/// each below a "Show replies" button, among the replies that X shows
/// without scrolling.
const HIDDEN_REPLIES_POST: &str = "https://x.com/AChillGhost/status/1761774191920005343";

/// A post at the end of whose conversation, below about 4000 CSS pixels of
/// replies, X hides the replies that it deems probable spam, below a "Show
/// probable spam" button. A click on that button replaces the button with
/// a "Probable spam" heading, below which X shows those replies.
const PROBABLE_SPAM_POST: &str = "https://x.com/Aella_Girl/status/1811478649964843329";

/// A post whose conversation has many long replies, each of which X cuts
/// off above a "Show more" button. The first of them is about one screen
/// below the top of the page.
const LONG_REPLIES_POST: &str = "https://x.com/terronk/status/1916218168626012356";

/// How far [`long_replies_are_expanded`] scrolls down, in CSS pixels: far
/// enough to pass several long replies of [`LONG_REPLIES_POST`].
const LONG_REPLIES_DEPTH: u32 = 4000;

/// A JavaScript expression that is truthy once the page shows that the
/// cookies log in: once it has the button that switches between the user's
/// accounts.
const LOGGED_IN: &str =
    r#"document.querySelector('[data-testid="SideNav_AccountSwitcher_Button"]')"#;

/// A JavaScript function that takes a text, and returns how many buttons on
/// the page have that text.
const BUTTONS_WITH_TEXT: &str = r#"((text) => [...document.querySelectorAll("button")].filter((b) => b.innerText.trim() === text).length)"#;

/// Load `url` logged in to X (see [`with_login`]), and wait until `done`
/// has held for 3 seconds for the value of the JavaScript `expression`.
///
/// `what` describes what is awaited, for the error message. `timeout` is
/// how long to wait.
async fn check<T: DeserializeOwned + Debug>(
    url: &str,
    what: &str,
    expression: &str,
    timeout: Duration,
    done: impl Fn(&T) -> bool,
) -> Result<()> {
    with_login("twitter", url, LOGGED_IN, async |tab| {
        wait_for(tab, what, expression, timeout, Duration::from_secs(3), done).await?;
        Ok(())
    })
    .await
}

#[tokio::test]
#[ignore = "needs a login (see `with_login`)"]
async fn hidden_replies_are_shown() -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct Page {
        /// How many "Show replies" buttons the page has.
        show_replies: usize,
        clicked: Vec<String>,
    }
    let expression = format!(
        r#"({{
            show_replies: {BUTTONS_WITH_TEXT}("Show replies"),
            clicked: {CLICKED_TEXTS},
        }})"#
    );
    check(
        HIDDEN_REPLIES_POST,
        "the userscript to click every \"Show replies\"",
        &expression,
        Duration::from_secs(30),
        |p: &Page| p.show_replies == 0 && p.clicked.iter().any(|text| text == "Show replies"),
    )
    .await
}

#[tokio::test]
#[ignore = "needs a login (see `with_login`)"]
async fn probable_spam_is_shown() -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct Page {
        /// How many "Show probable spam" buttons the page has.
        show_spam: usize,
        /// How many replies the page shows below the "Probable spam"
        /// heading (and above the next heading, if any).
        spam: usize,
        clicked: Vec<String>,
    }
    // X shows the end of the conversation only once it is near the
    // viewport, so each evaluation scrolls to the bottom of the page.
    let expression = format!(
        r#"(() => {{
            window.scrollTo(0, document.documentElement.scrollHeight);
            let section = null;
            let spam = 0;
            for (const cell of document.querySelectorAll('[data-testid="primaryColumn"] [data-testid="cellInnerDiv"]')) {{
                const heading = cell.querySelector('h2[role="heading"]');
                if (heading !== null) {{
                    section = heading.innerText.trim();
                }} else if (section === "Probable spam" && cell.querySelector("article")?.checkVisibility()) {{
                    spam++;
                }}
            }}
            return {{
                show_spam: {BUTTONS_WITH_TEXT}("Show probable spam"),
                spam,
                clicked: {CLICKED_TEXTS},
            }};
        }})()"#
    );
    check(
        PROBABLE_SPAM_POST,
        "the userscript to click \"Show probable spam\", and the page to show the spam",
        &expression,
        Duration::from_secs(60),
        |p: &Page| {
            p.show_spam == 0
                && p.spam > 0
                && p.clicked.iter().any(|text| text == "Show probable spam")
        },
    )
    .await
}

#[tokio::test]
#[ignore = "needs a login (see `with_login`)"]
async fn long_replies_are_expanded() -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct Page {
        /// How far the page is scrolled down, in CSS pixels.
        scroll_y: f64,
        /// How many "Show more" buttons the page has.
        show_more: usize,
        clicked: Vec<String>,
    }
    // Each evaluation scrolls one screen further down, until
    // LONG_REPLIES_DEPTH, so that X shows the long replies down to there.
    let expression = format!(
        r#"(() => {{
            if (window.scrollY < {LONG_REPLIES_DEPTH}) {{
                window.scrollBy(0, window.innerHeight);
            }}
            return {{
                scroll_y: window.scrollY,
                show_more: document.querySelectorAll('button[data-testid="tweet-text-show-more-link"]').length,
                clicked: {CLICKED_TEXTS},
            }};
        }})()"#
    );
    check(
        LONG_REPLIES_POST,
        "the userscript to click \"Show more\" on every long reply down to LONG_REPLIES_DEPTH",
        &expression,
        Duration::from_secs(30),
        |p: &Page| {
            p.scroll_y >= f64::from(LONG_REPLIES_DEPTH)
                && p.show_more == 0
                && p.clicked.iter().any(|text| text == "Show more")
        },
    )
    .await
}
