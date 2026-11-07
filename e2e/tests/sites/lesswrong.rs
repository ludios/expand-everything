use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
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
    /// How many of the `shown` comments are collapsed (showing no body).
    collapsed: usize,
    /// How many comments the page truncates to one line.
    one_line: usize,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

/// A JavaScript expression for the [`Comments`].
fn comments_expression() -> String {
    format!(
        r#"({{
            shown: document.querySelectorAll(".CommentsItem-root").length,
            collapsed: document.querySelectorAll(".CommentsItem-root:not(:has(.CommentBody-root))").length,
            one_line: document.querySelectorAll(".SingleLineComment-truncatedHighlight").length,
            clicked: {CLICKED_TEXTS},
        }})"#
    )
}

#[tokio::test]
async fn post_comments_are_expanded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, POST).await?;
        // LessWrong renders the comments a few seconds after the page loads.
        // The settling time is long enough to catch a userscript that
        // collapses comments after expanding them.
        wait_for(
            &tab,
            "the userscript to expand every comment truncated to one line, and to collapse no comment",
            &comments_expression(),
            Duration::from_secs(30),
            Duration::from_secs(3),
            |c: &Comments| {
                c.shown > 0 && c.collapsed == 0 && c.one_line == 0 && !c.clicked.is_empty()
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

#[tokio::test]
async fn all_posts_are_listed() -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct Posts {
        /// How many posts the list shows.
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
                    listed: list?.querySelectorAll("article").length ?? 0,
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
            |p: &Posts| {
                p.listed > 0
                    && !p.more
                    && p.clicked.iter().any(|text| text.starts_with("Load More"))
            },
        )
        .await?;
        Ok(())
    })
    .await
}
