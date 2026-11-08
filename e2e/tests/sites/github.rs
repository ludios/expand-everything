use anyhow::{Context, Result, ensure};
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{add_cookies, open, wait_for, with_chrome};
use serde::Deserialize;
use std::path::Path;
use std::time::Duration;
use zendriver::Tab;

/// A pull request whose timeline GitHub cuts off twice above "N hidden
/// items", and which has a comment collapsed below "1 similar comment" and
/// two comments minimized as off-topic (which only a logged-in user can
/// expand).
const PULL_REQUEST: &str = "https://github.com/rust-lang/rust/pull/95035";

/// A user's profile, whose activity ends in a "Show more activity" button
/// that changes the URL when clicked.
const PROFILE: &str = "https://github.com/A";

/// A JavaScript expression for whether no part of the timeline of a pull
/// request or a discussion is still cut off above "N hidden items".
const NO_HIDDEN_ITEMS: &str = r##"document.querySelector("#js-progressive-timeline-item-container form.ajax-pagination-form") === null"##;

/// What a test waits for.
#[derive(Debug, Deserialize)]
struct Page {
    /// Whether the page shows in full what the test expects the userscript
    /// to have expanded.
    expanded: bool,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

/// Wait until, in the page in `tab`, the JavaScript expression `expanded` has
/// been true, and `clicks_done` (given the texts of the userscript's clicks)
/// has returned true, for an uninterrupted `settle`.
///
/// `what` describes what is awaited, for the error message.
///
/// Returns an error if that doesn't happen within 60 seconds.
async fn expect_expanded(
    tab: &Tab,
    what: &str,
    expanded: &str,
    settle: Duration,
    clicks_done: impl Fn(&[String]) -> bool,
) -> Result<()> {
    wait_for(
        tab,
        what,
        &format!("({{ expanded: {expanded}, clicked: {CLICKED_TEXTS} }})"),
        Duration::from_secs(60),
        settle,
        |p: &Page| p.expanded && clicks_done(&p.clicked),
    )
    .await?;
    Ok(())
}

/// Whether one of `clicked` (the texts of the userscript's clicks) is `text`.
fn clicked_on(clicked: &[String], text: &str) -> bool {
    clicked.iter().any(|t| t == text)
}

/// Whether `text` says how many items GitHub hides, as "N hidden items" does.
fn is_hidden_items(text: &str) -> bool {
    text.strip_suffix(" hidden items")
        .is_some_and(|n| n.parse::<u32>().is_ok())
}

#[tokio::test]
async fn pull_request_hidden_items_and_similar_comments_are_expanded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, PULL_REQUEST).await?;
        expect_expanded(
            &tab,
            "the userscript to click every \"N hidden items\" and \"1 similar comment\"",
            &format!(
                r#"{NO_HIDDEN_ITEMS} && (() => {{
                    const similar = [...document.querySelectorAll("summary.pagination-loader-container")];
                    return similar.length > 0 && similar.every((s) => s.parentElement.open);
                }})()"#
            ),
            Duration::from_secs(2),
            |clicked| {
                clicked.iter().any(|t| is_hidden_items(t)) && clicked_on(clicked, "1 similar comment")
            },
        )
        .await
    })
    .await
}

#[tokio::test]
async fn profile_activity_is_not_expanded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, PROFILE).await?;
        expect_expanded(
            &tab,
            "the userscript to leave \"Show more activity\" unclicked, and the URL unchanged",
            &format!(
                r#"location.href === "{PROFILE}"
                    && [...document.querySelectorAll("button.ajax-pagination-btn")].some((b) => b.innerText === "Show more activity")"#
            ),
            Duration::from_secs(5),
            |clicked| clicked.is_empty(),
        )
        .await
    })
    .await
}

#[tokio::test]
#[ignore = "needs E2E_GITHUB_COOKIES, the path of a cookies.txt file of a logged-in GitHub session"]
async fn minimized_pull_request_comments_are_expanded_when_logged_in() -> Result<()> {
    let cookies = std::env::var_os("E2E_GITHUB_COOKIES").context(
        "E2E_GITHUB_COOKIES must be the path of a cookies.txt file of a logged-in GitHub session",
    )?;
    with_chrome(async |browser| {
        add_cookies(browser, Path::new(&cookies)).await?;
        let tab = open(browser, PULL_REQUEST).await?;
        let login: String = tab
            .evaluate_main(r#"document.querySelector('meta[name="user-login"]')?.content ?? """#)
            .await?;
        ensure!(
            !login.is_empty(),
            "the cookies in E2E_GITHUB_COOKIES don't log in to GitHub"
        );
        expect_expanded(
            &tab,
            "the userscript to load every hidden item, and to click \"Show comment\" on every minimized comment",
            &format!(
                r#"{NO_HIDDEN_ITEMS}
                    && document.querySelector("div.minimized-comment > details") !== null
                    && document.querySelector("div.minimized-comment > details:not([open])") === null"#
            ),
            Duration::from_secs(2),
            |clicked| clicked_on(clicked, "Show comment"),
        )
        .await
    })
    .await
}
