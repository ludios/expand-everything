use anyhow::Result;
use expand_everything_e2e::userscript::{CLICKED_TEXTS, HARNESS_GLOBAL, Start};
use expand_everything_e2e::{open, wait_for, with_chrome, with_chrome_starting};
use serde::Deserialize;
use std::time::Duration;

/// A Stack Overflow question whose accepted answer has 14 comments. Stack
/// Overflow first shows only the 5 that it ranks as top comments, below a
/// "14 Comments" heading with an icon button whose `aria-label` is "Show all
/// comments". A click on that button shows all 14 and changes its
/// `aria-label` to "Collapse comments", so a second click would hide them
/// all. (Stack Overflow's scripts call this an experiment, so it may send
/// some visitors the "Show N more comments" link of [`LINKS_PAGE`] instead.)
const TOP_COMMENTS_PAGE: &str = "https://stackoverflow.com/questions/59156473/what-is-the-difference-between-async-move-and-async-move";

/// The `id` of the element that holds the comments of the accepted answer on
/// [`TOP_COMMENTS_PAGE`].
const TOP_COMMENTS_ID: &str = "follow-ups-container-59158908";

/// What the test of [`TOP_COMMENTS_PAGE`] waits for.
#[derive(Debug, Deserialize)]
struct TopComments {
    /// The page's title, which shows whether the site sent the question.
    #[allow(dead_code, reason = "shown in the error message")]
    title: String,
    /// The heading above the answer's comments, such as "14 Comments".
    heading: String,
    /// How many of the answer's comments are visible.
    shown: usize,
    /// How many "Show all comments" buttons the page has.
    cut_off: usize,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

#[tokio::test]
async fn all_comments_are_shown_where_stack_overflow_shows_only_the_top_ones() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, TOP_COMMENTS_PAGE).await?;
        let expression = format!(
            r#"(() => {{
                const comments = document.getElementById("{TOP_COMMENTS_ID}");
                return {{
                    title: document.title,
                    heading: comments?.querySelector("h2")?.innerText ?? "",
                    shown: [...(comments?.querySelectorAll('[id^="follow-up-"]') ?? [])].filter((c) => c.checkVisibility()).length,
                    cut_off: document.querySelectorAll('button[aria-label="Show all comments"]').length,
                    clicked: {CLICKED_TEXTS},
                }};
            }})()"#
        );
        wait_for(
            &tab,
            "the userscript to click \"Show all comments\", and the answer to show as many comments as its heading says",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |p: &TopComments| {
                p.shown > 5
                    && p.heading == format!("{} Comments", p.shown)
                    && p.cut_off == 0
                    && p.clicked.iter().any(|t| t == "Show all comments")
            },
        )
        .await?;
        Ok(())
    })
    .await
}

/// A Super User question that cuts off the comments of two posts: it has
/// "Show 7 more comments" below the question, and "Show 2 more comments" below
/// an answer. Each is a link to "#". A script that the page loads only after
/// the DOM has been parsed handles a click on one, by requesting
/// `/posts/POST_ID/comments`, and hides the link once the comments have
/// loaded. Until that script is ready, a click only navigates to "#".
const LINKS_PAGE: &str = "https://superuser.com/questions/681575/any-way-to-get-list-of-functions-defined-in-zsh-like-alias-command-for-aliases";

/// The `id`s of the elements of the last comment on each of the two posts.
/// The page has each only after it has loaded the post's comments.
const LOADED_COMMENT_IDS: [&str; 2] = ["comment-869378", "comment-3032175"];

/// What the tests of [`LINKS_PAGE`] wait for.
#[derive(Debug, Deserialize)]
struct Links {
    /// The page's title, which shows whether the site sent the question.
    #[allow(dead_code, reason = "shown in the error message")]
    title: String,
    /// The URL of the page.
    url: String,
    /// How many "Show N more comments" links are visible.
    cut_off: usize,
    /// How many of the comments whose `id`s are [`LOADED_COMMENT_IDS`] the
    /// page has.
    loaded_comments: usize,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
    /// How many different elements the userscript clicked.
    clicked_elements: usize,
    /// How many requests for a post's comments the page made. (Chrome
    /// records only the page's first 250 requests; these are made early.)
    comment_requests: usize,
}

/// Open [`LINKS_PAGE`] in a Chrome whose userscript starts when `start` says,
/// and wait until the userscript has loaded the comments of both posts, with
/// one click on each link, one request for each click, and no navigation to
/// "#".
async fn expect_loaded_comments(start: Start) -> Result<()> {
    with_chrome_starting(start, async |browser| {
        let tab = open(browser, LINKS_PAGE).await?;
        let expression = format!(
            r#"(() => {{
                const requests = performance.getEntriesByType("resource");
                return {{
                    title: document.title,
                    url: location.href,
                    cut_off: [...document.querySelectorAll("a.js-show-link")].filter((a) => a.checkVisibility()).length,
                    loaded_comments: {LOADED_COMMENT_IDS:?}.filter((id) => document.getElementById(id) !== null).length,
                    clicked: {CLICKED_TEXTS},
                    clicked_elements: new Set(window.{HARNESS_GLOBAL}.clicks.map((c) => c.element)).size,
                    comment_requests: requests.filter((r) => /\/posts\/\d+\/comments/.test(r.name)).length,
                }};
            }})()"#
        );
        wait_for(
            &tab,
            "the userscript to load the comments of both posts, with one click and one request for each",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |p: &Links| {
                let texts_are_right = p
                    .clicked
                    .iter()
                    .all(|t| t.starts_with("Show ") && t.contains(" more comment"));
                !p.url.contains('#')
                    && p.cut_off == 0
                    && p.loaded_comments == LOADED_COMMENT_IDS.len()
                    && p.clicked.len() >= LOADED_COMMENT_IDS.len()
                    && texts_are_right
                    && p.clicked_elements == p.clicked.len()
                    && p.comment_requests == p.clicked.len()
            },
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn comments_are_all_loaded_with_one_request_for_each_post() -> Result<()> {
    expect_loaded_comments(Start::DocumentEnd).await
}

#[tokio::test]
async fn comments_are_all_loaded_by_a_userscript_that_starts_after_the_load_event() -> Result<()> {
    expect_loaded_comments(Start::AfterLoad).await
}
