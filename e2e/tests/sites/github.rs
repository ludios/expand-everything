use anyhow::{Context, Result, ensure};
use expand_everything_e2e::userscript::{CLICKED_TEXTS, OBSERVING};
use expand_everything_e2e::{add_cookies, open, wait_for, with_chrome};
use serde::Deserialize;
use std::path::Path;
use std::time::Duration;
use zendriver::{Browser, Tab};

/// A pull request whose timeline GitHub cuts off twice above "N hidden
/// items", and which has a comment collapsed below "1 similar comment" and
/// two comments minimized as off-topic (which only a logged-in user can
/// expand).
const PULL_REQUEST: &str = "https://github.com/rust-lang/rust/pull/95035";

/// A link to a comment in the middle of an issue with 512 comments. GitHub
/// first shows the first comments, the linked comment, and the last
/// comments, and above and below the linked comment, "N remaining items" and
/// a "Load more" button that loads 150 more at a time.
const ISSUE_WITH_MANY_COMMENTS: &str =
    "https://github.com/rust-lang/rust/issues/57640#issuecomment-456702793";

/// An issue with 37 comments, of which five are minimized as spam or
/// off-topic, and some of which are first hidden behind "Load more".
const ISSUE_WITH_MINIMIZED_COMMENTS: &str =
    "https://github.com/JustAnotherArchivist/snscrape/issues/634";

/// A pull request with 11 review threads, of which GitHub collapses the 7
/// that are outdated or resolved.
const PULL_REQUEST_WITH_OUTDATED_THREADS: &str = "https://github.com/NixOS/nixpkgs/pull/194310";

/// A user's profile, whose activity ends in a "Show more activity" button
/// that changes the URL when clicked.
const PROFILE: &str = "https://github.com/A";

/// A commit with 424 comments, of which GitHub first shows 12, below a "Load
/// more comments" button that loads more of them each time it is clicked.
const COMMIT: &str =
    "https://github.com/PolyMC/PolyMC/commit/ccf282593dcdbe189c99b81b8bc90cb203aed3ee";

/// A discussion that GitHub cuts off above "19 hidden items", and whose
/// threads of replies it cuts off below "Show N previous replies".
const DISCUSSION: &str = "https://github.com/orgs/community/discussions/12032";

/// An issue whose description and two comments each have a `<details>`.
const ISSUE_WITH_DETAILS: &str = "https://github.com/rust-lang/rust/issues/165052";

/// A JavaScript expression for whether no part of the timeline of a pull
/// request or a discussion is still cut off above "N hidden items".
const NO_HIDDEN_ITEMS: &str = r##"document.querySelector("#js-progressive-timeline-item-container form.ajax-pagination-form") === null"##;

/// A JavaScript expression for whether no part of the timeline of an issue is
/// still cut off at "N remaining items" (above or below a linked comment).
const NO_REMAINING_ITEMS: &str =
    r#"document.querySelector('[data-testid^="issue-timeline-load-more-container-"]') === null"#;

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
async fn issue_comments_are_all_loaded() -> Result<()> {
    with_chrome(async |browser| {
        // GitHub replaces "Load more" with a new button each time it has
        // loaded more comments, so more than one click shows that the
        // userscript clicked each new button.
        let tab = open(browser, ISSUE_WITH_MANY_COMMENTS).await?;
        expect_expanded(
            &tab,
            "the userscript to click \"Load more\" until no comment remains to be loaded",
            NO_REMAINING_ITEMS,
            Duration::from_secs(2),
            |clicked| clicked.iter().filter(|t| *t == "Load more").count() >= 2,
        )
        .await
    })
    .await
}

#[tokio::test]
async fn minimized_issue_comments_are_expanded() -> Result<()> {
    with_chrome(async |browser| {
        // A minimized comment's "show comment" is only the tooltip of an icon
        // button.
        let tab = open(browser, ISSUE_WITH_MINIMIZED_COMMENTS).await?;
        expect_expanded(
            &tab,
            "the userscript to load every comment, and to click \"show comment\" on every minimized one",
            &format!(
                r#"{NO_REMAINING_ITEMS} && ![...document.querySelectorAll('[data-testid="comment-header"] button[aria-labelledby]')]
                    .some((b) => document.getElementById(b.getAttribute("aria-labelledby"))?.textContent === "show comment")"#
            ),
            Duration::from_secs(2),
            |clicked| clicked_on(clicked, "show comment"),
        )
        .await
    })
    .await
}

#[tokio::test]
async fn outdated_review_threads_are_expanded() -> Result<()> {
    with_chrome(async |browser| {
        // A review thread's chevron is an icon button whose tooltip says
        // "Comment thread".
        let tab = open(browser, PULL_REQUEST_WITH_OUTDATED_THREADS).await?;
        expect_expanded(
            &tab,
            "the userscript to expand every collapsed review thread",
            &format!(
                r#"{NO_HIDDEN_ITEMS} && (() => {{
                    const threads = [...document.querySelectorAll("review-thread-collapsible")];
                    return threads.length > 0 && threads.every((t) => t.classList.contains("open"));
                }})()"#
            ),
            Duration::from_secs(2),
            |clicked| clicked_on(clicked, "Comment thread"),
        )
        .await
    })
    .await
}

#[tokio::test]
async fn profile_activity_is_not_expanded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, PROFILE).await?;
        // That the userscript observes the page shows that the GitHub block
        // runs.
        expect_expanded(
            &tab,
            "the userscript to observe the page, but to leave \"Show more activity\" unclicked, and the URL unchanged",
            &format!(
                r#"{OBSERVING} && location.href === "{PROFILE}"
                    && [...document.querySelectorAll("button.ajax-pagination-btn")].some((b) => b.innerText === "Show more activity")"#
            ),
            Duration::from_secs(5),
            |clicked| !clicked_on(clicked, "Show more activity"),
        )
        .await
    })
    .await
}

#[tokio::test]
async fn commit_comments_are_all_loaded() -> Result<()> {
    with_chrome(async |browser| {
        // The comments start with a heading (for screen readers only) that
        // says how many there are, such as "424 commit comments".
        let tab = open(browser, COMMIT).await?;
        expect_expanded(
            &tab,
            "the userscript to click \"Load more comments\" until every comment is loaded",
            r##"(() => {
                const comments = document.querySelector("#comments.react-comments-container");
                return comments?.querySelector(":scope > button") === null
                    && comments.querySelectorAll('[id^="commitcomment-"]').length === parseInt(comments.querySelector("h2")?.textContent);
            })()"##,
            Duration::from_secs(2),
            |clicked| clicked_on(clicked, "Load more comments"),
        )
        .await
    })
    .await
}

#[tokio::test]
async fn discussion_replies_are_all_shown() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, DISCUSSION).await?;
        expect_expanded(
            &tab,
            "the userscript to load every hidden item, and to click every \"Show N previous replies\"",
            &format!(
                r#"{NO_HIDDEN_ITEMS} && document.querySelector(".discussion-nested-comment-paging-form") === null"#
            ),
            Duration::from_secs(2),
            |clicked| {
                clicked
                    .iter()
                    .any(|t| t.starts_with("Show ") && t.contains(" previous repl"))
            },
        )
        .await
    })
    .await
}

/// Open [`ISSUE_WITH_DETAILS`] in `browser`, and wait for the userscript to
/// open every `<details>` in the description and the comments.
///
/// Returns the tab of the issue.
async fn open_issue_with_details(browser: &Browser) -> Result<Tab> {
    let tab = open(browser, ISSUE_WITH_DETAILS).await?;
    expect_expanded(
        &tab,
        "the userscript to open every <details> in the description and the comments",
        r#"(() => {
            const details = [...document.querySelectorAll(".markdown-body details")];
            return details.length >= 3 && details.every((d) => d.open);
        })()"#,
        Duration::from_secs(2),
        |clicked| clicked_on(clicked, "Program output"),
    )
    .await?;
    Ok(tab)
}

#[tokio::test]
async fn details_are_opened() -> Result<()> {
    with_chrome(async |browser| {
        open_issue_with_details(browser).await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn details_closed_by_the_user_stay_closed_after_going_to_a_fragment() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open_issue_with_details(browser).await?;
        tab.evaluate_main::<bool>(
            r#"(() => {
                document.querySelector(".markdown-body details").open = false;
                location.hash = "e2e-fragment";
                return true;
            })()"#,
        )
        .await?;
        wait_for(
            &tab,
            "the <details> that the test closed to stay closed",
            r#"document.querySelector(".markdown-body details").open"#,
            Duration::from_secs(10),
            Duration::from_secs(3),
            |open: &bool| !open,
        )
        .await?;
        Ok(())
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
