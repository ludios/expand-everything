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
/// the DOM has been parsed handles a click on one, by requesting the post's
/// comments. Until that script is ready, a click only navigates to "#".
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
    /// How many "Show N more comments" links the page shows.
    cut_off: usize,
    /// How many of the comments whose `id`s are [`LOADED_COMMENT_IDS`] the
    /// page has.
    loaded_comments: usize,
    /// How many clicks the userscript made.
    clicks: usize,
    /// How many elements the userscript clicked.
    clicked_elements: usize,
}

/// Open [`LINKS_PAGE`] in a Chrome whose userscript starts when `start` says,
/// and wait for the userscript to load the comments of both posts without
/// navigating to "#", and for `also` to be true.
async fn expect_loaded_comments(start: Start, also: fn(&Links) -> bool) -> Result<()> {
    with_chrome_starting(start, async |browser| {
        let tab = open(browser, LINKS_PAGE).await?;
        let expression = format!(
            r#"(() => {{
                const clicks = window.{HARNESS_GLOBAL}.clicks;
                return {{
                    title: document.title,
                    url: location.href,
                    cut_off: [...document.querySelectorAll("a.js-show-link")].filter((a) => a.checkVisibility()).length,
                    loaded_comments: {LOADED_COMMENT_IDS:?}.filter((id) => document.getElementById(id) !== null).length,
                    clicks: clicks.length,
                    clicked_elements: new Set(clicks.map((c) => c.element)).size,
                }};
            }})()"#
        );
        wait_for(
            &tab,
            "the userscript to load the comments of both posts",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |p: &Links| {
                p.url == LINKS_PAGE
                    && p.cut_off == 0
                    && p.loaded_comments == LOADED_COMMENT_IDS.len()
                    && p.clicked_elements == 2
                    && also(p)
            },
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn comments_are_all_loaded_without_navigating() -> Result<()> {
    expect_loaded_comments(Start::DocumentEnd, |_| true).await
}

/// By the `load` event, the page's script handles a click, so the userscript
/// must not click a link a second time (the page would request the comments
/// again).
#[tokio::test]
async fn each_link_is_clicked_once_by_a_userscript_that_starts_after_the_load_event() -> Result<()>
{
    expect_loaded_comments(Start::AfterLoad, |p| p.clicks == p.clicked_elements).await
}
