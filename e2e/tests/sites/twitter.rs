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
const LONG_REPLIES_POST: &str = "https://x.com/ledwards/status/1916218168626012356";

/// A post (which replies to no other post) whose picture its author flagged
/// as sensitive. For an account whose setting "Display media that may
/// contain sensitive content" is off, X blurs the picture, under "Content
/// warning: Sensitive content" and a "Show" button.
const FLAGGED_PICTURE_POST: &str = "https://x.com/KalebPrime/status/1486262773743038464";

/// How far [`long_replies_are_expanded`] scrolls down at least, in CSS
/// pixels: far enough to pass several long replies of [`LONG_REPLIES_POST`].
const LONG_REPLIES_DEPTH: u32 = 4000;

/// A JavaScript expression that is truthy once the page shows that the
/// cookies log in: once it has the button that opens the menu of the user's
/// account.
const LOGGED_IN: &str =
    r#"document.querySelector('[data-testid="SideNav_AccountSwitcher_Button"]')"#;

/// A JavaScript function that takes a text, and returns how many `button`
/// elements on the page have that text (after trimming).
const BUTTONS_WITH_TEXT: &str = r#"((text) => [...document.querySelectorAll("button")].filter((b) => b.innerText.trim() === text).length)"#;

/// Load `url` logged in to X (see [`with_login`]), and then wait (see
/// [`wait_for`]) until `done` has held for 3 seconds for the value of the
/// JavaScript `expression`.
///
/// `logged_in` is the `logged_in` of [`with_login`], such as [`LOGGED_IN`].
/// `what` describes what is awaited, for the error message. `timeout` is
/// how long to wait once the page shows that the cookies log in.
///
/// Returns an error if the page doesn't open, doesn't show that the cookies
/// log in, or doesn't satisfy `done` in time.
async fn check<T: DeserializeOwned + Debug>(
    url: &str,
    logged_in: &str,
    what: &str,
    expression: &str,
    timeout: Duration,
    done: impl Fn(&T) -> bool,
) -> Result<()> {
    with_login("twitter", url, logged_in, async |tab| {
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
        /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
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
        LOGGED_IN,
        "the userscript to click \"Show replies\" until none is left",
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
        /// How many cells of the conversation below the "Probable spam"
        /// heading (and above the next heading, if any) show a reply.
        spam: usize,
        /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
        clicked: Vec<String>,
    }
    // X shows the end of the conversation only once it is near the
    // viewport, so each evaluation asks to scroll to the bottom of the page,
    // until the "Probable spam" heading exists. From then on, it scrolls the
    // heading into view, so that X keeps the heading and the spam below it in
    // the DOM.
    let expression = format!(
        r#"(() => {{
            const cells = [...document.querySelectorAll('[data-testid="primaryColumn"] [data-testid="cellInnerDiv"]')];
            const heading_text = (cell) => cell.querySelector('h2[role="heading"]')?.innerText.trim() ?? null;
            const spam_heading = cells.find((cell) => heading_text(cell) === "Probable spam");
            if (spam_heading === undefined) {{
                window.scrollTo(0, document.documentElement.scrollHeight);
            }} else {{
                spam_heading.scrollIntoView();
            }}
            const shown = {{ visibilityProperty: true, opacityProperty: true }};
            let section = null;
            let spam = 0;
            for (const cell of cells) {{
                if (heading_text(cell) !== null) {{
                    section = heading_text(cell);
                }} else if (section === "Probable spam" && cell.querySelector("article")?.checkVisibility(shown)) {{
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
        LOGGED_IN,
        "the userscript to click \"Show probable spam\", and the page to show at least one reply below \"Probable spam\" (and no \"Show probable spam\" button)",
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
        /// How many "Show more" buttons (by their `data-testid`) the page has.
        show_more: usize,
        /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
        clicked: Vec<String>,
    }
    // X keeps only the replies near the viewport in the DOM, so a "Show more"
    // that the userscript misses must stop the scrolling (instead of leaving
    // the DOM): an evaluation scrolls half a screen further down only if the
    // page has no "Show more" button, until LONG_REPLIES_DEPTH.
    let expression = format!(
        r#"(() => {{
            const show_more = document.querySelectorAll('button[data-testid="tweet-text-show-more-link"]').length;
            if (show_more === 0 && window.scrollY < {LONG_REPLIES_DEPTH}) {{
                window.scrollBy(0, window.innerHeight / 2);
            }}
            return {{
                scroll_y: window.scrollY,
                show_more,
                clicked: {CLICKED_TEXTS},
            }};
        }})()"#
    );
    check(
        LONG_REPLIES_POST,
        LOGGED_IN,
        &format!(
            "the userscript to click \"Show more\" on every long reply in the top \
             {LONG_REPLIES_DEPTH} CSS pixels of the page"
        ),
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

#[tokio::test]
#[ignore = "needs a login (see `with_login`), to an account whose setting \"Display media that may contain sensitive content\" is off"]
async fn flagged_picture_is_shown() -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct Page {
        /// How many pictures the post has.
        pictures: usize,
        /// How many of those pictures are blurred.
        blurred: usize,
        /// How many content warnings the post shows.
        warnings: usize,
        /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
        clicked: Vec<String>,
    }
    // The page's first article is the post, because the post replies to no
    // other post. X blurs a picture with a `filter` on an element between
    // the picture and the article.
    let expression = format!(
        r#"(() => {{
            const post = document.querySelector("article");
            const pictures = [...(post?.querySelectorAll('[data-testid="tweetPhoto"]') ?? [])];
            const blurred = (picture) => {{
                for (let e = picture; e !== post; e = e.parentElement) {{
                    if (getComputedStyle(e).filter.includes("blur(")) {{
                        return true;
                    }}
                }}
                return false;
            }};
            return {{
                pictures: pictures.length,
                blurred: pictures.filter(blurred).length,
                warnings: [...(post?.querySelectorAll("span") ?? [])].filter((s) => s.innerText.startsWith("Content warning:")).length,
                clicked: {CLICKED_TEXTS},
            }};
        }})()"#
    );
    // An account whose setting "Display media that may contain sensitive
    // content" is on gets the picture without a content warning. The page
    // has the account's settings in the inline script that sets
    // `window.__INITIAL_STATE__` (a global that X deletes once it starts).
    let logged_in = format!(
        r#"({LOGGED_IN}) && [...document.scripts].some((s) => s.textContent.includes('"display_sensitive_media":false'))"#
    );
    check(
        FLAGGED_PICTURE_POST,
        &logged_in,
        "the userscript to click \"Show\" on the content warning, and the page to show the picture unblurred",
        &expression,
        Duration::from_secs(30),
        |p: &Page| {
            p.pictures > 0
                && p.blurred == 0
                && p.warnings == 0
                && p.clicked.iter().any(|text| text == "Show")
        },
    )
    .await
}
