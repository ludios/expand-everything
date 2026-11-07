use anyhow::Result;
use expand_everything_e2e::userscript::{CLICKED_TEXTS, HARNESS_GLOBAL};
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A post with about 300 comments, many of which LessWrong truncates to one
/// line.
const POST: &str =
    "https://www.lesswrong.com/posts/uKp6tBFStnsvrot5t/what-dall-e-2-can-and-cannot-do";

/// A user page that shows the user's 11 comments, 8 of which LessWrong
/// collapses because they were downvoted.
const USER_WITH_DOWNVOTED_COMMENTS: &str = "https://www.lesswrong.com/users/blueiris2";

/// A user page that lists the user's posts (28 in November 2026), 7 at a
/// time, above a "Load More" link.
const USER_WITH_POSTS: &str = "https://www.lesswrong.com/users/valentine";

/// What a test of comments waits for.
#[derive(Debug, Deserialize)]
struct Comments {
    /// How many comments the page shows, whether expanded or collapsed (not
    /// counting the comments that it truncates to one line).
    shown: usize,
    /// How many of the `shown` comments are collapsed: they show neither a
    /// body nor the notice that replaces the body of a deleted comment.
    collapsed: usize,
    /// How many comments the page truncates to one line.
    one_line: usize,
    /// How many of the userscript's clicks were on a comment truncated to one
    /// line.
    one_line_clicks: usize,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

/// A JavaScript expression for the [`Comments`].
fn comments_expression() -> String {
    format!(
        r#"({{
            shown: document.querySelectorAll(".CommentsItem-root").length,
            collapsed: document.querySelectorAll(".CommentsItem-root:not(:has(.CommentBody-root, .CommentDeletedMetadata-root))").length,
            one_line: document.querySelectorAll(".SingleLineComment-root").length,
            one_line_clicks: window.{HARNESS_GLOBAL}.clicks.filter((c) => c.element.closest(".SingleLineComment-root")).length,
            clicked: {CLICKED_TEXTS},
        }})"#
    )
}

#[tokio::test]
async fn post_comments_are_expanded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, POST).await?;
        wait_for(
            &tab,
            "the userscript to expand every comment truncated to one line, and to collapse no comment",
            &comments_expression(),
            Duration::from_secs(30),
            Duration::from_secs(2),
            |c: &Comments| {
                c.shown > 0
                    && c.collapsed == 0
                    && c.one_line == 0
                    && c.one_line_clicks > 0
                    && !c.clicked.iter().any(|text| text == "-")
            },
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn downvoted_comments_are_expanded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, USER_WITH_DOWNVOTED_COMMENTS).await?;
        wait_for(
            &tab,
            "the userscript to click the \"+\" of every collapsed comment",
            &comments_expression(),
            Duration::from_secs(30),
            Duration::from_secs(2),
            |c: &Comments| {
                c.shown > 0 && c.collapsed == 0 && c.clicked.iter().any(|text| text == "+")
            },
        )
        .await?;
        Ok(())
    })
    .await
}

/// The number of posts in a list, according to the last text in `clicked`
/// (the texts of the userscript's clicks) of the form "Load More
/// (LOADED/TOTAL)", or None if there is no such text.
fn total_posts(clicked: &[String]) -> Option<usize> {
    let text = clicked
        .iter()
        .rev()
        .find(|text| text.starts_with("Load More ("))?;
    let (_, total) = text.strip_suffix(')')?.split_once('/')?;
    total.parse().ok()
}

#[tokio::test]
async fn all_posts_are_listed() -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct Posts {
        /// How many different posts the list shows.
        listed: usize,
        /// Whether the list ends in a "Load More" link or in the spinner that
        /// replaces the link while LessWrong loads more posts.
        more: bool,
        clicked: Vec<String>,
    }
    with_chrome(async |browser| {
        let tab = open(browser, USER_WITH_POSTS).await?;
        let expression = format!(
            r#"(() => {{
                const list = document.querySelector(".ProfilePageAllPostsTabUnshared-postsList");
                return {{
                    listed: new Set([...(list?.querySelectorAll("article a[href^='/posts/']") ?? [])].map((a) => a.pathname)).size,
                    more: list?.querySelector("a.LoadMore-root, .Loading-spinner") != null,
                    clicked: {CLICKED_TEXTS},
                }};
            }})()"#
        );
        wait_for(
            &tab,
            "the userscript to click \"Load More\" until the list shows every post",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |p: &Posts| !p.more && total_posts(&p.clicked) == Some(p.listed),
        )
        .await?;
        Ok(())
    })
    .await
}
