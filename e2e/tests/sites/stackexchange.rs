use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A Stack Overflow question whose accepted answer has 14 comments. Stack
/// Overflow first shows only the 5 that it ranks as top comments, below a
/// "14 Comments" heading with a button labelled "Show all comments". A click
/// on that button shows all 14 and changes its label to "Collapse comments",
/// so a second click would hide them all.
const TOP_COMMENTS_PAGE: &str = "https://stackoverflow.com/questions/59156473/what-is-the-difference-between-async-move-and-async-move";

/// The `id` of the element that holds the comments of the accepted answer on
/// [`TOP_COMMENTS_PAGE`].
const TOP_COMMENTS_ID: &str = "follow-ups-container-59158908";

/// The `id` of the element of a comment on that answer that isn't a top
/// comment.
const OTHER_COMMENT_ID: &str = "follow-up-104563957";

/// What the test of [`TOP_COMMENTS_PAGE`] waits for.
#[derive(Debug, Deserialize)]
struct TopComments {
    /// The heading above the answer's comments, such as "14 Comments".
    heading: String,
    /// How many comments the answer shows.
    shown: usize,
    /// Whether the answer shows the comment whose `id` is
    /// [`OTHER_COMMENT_ID`].
    other_comment_shown: bool,
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
                    heading: comments?.querySelector("h2")?.innerText ?? "",
                    shown: comments?.querySelectorAll('[id^="follow-up-"]').length ?? 0,
                    other_comment_shown: document.getElementById("{OTHER_COMMENT_ID}") !== null,
                    cut_off: document.querySelectorAll('button[aria-label="Show all comments"]').length,
                    clicked: {CLICKED_TEXTS},
                }};
            }})()"#
        );
        wait_for(
            &tab,
            "the userscript to click \"Show all comments\" once, and the answer to show as many comments as its heading says",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |p: &TopComments| {
                p.heading == format!("{} Comments", p.shown)
                    && p.other_comment_shown
                    && p.cut_off == 0
                    && p.clicked == ["Show all comments"]
            },
        )
        .await?;
        Ok(())
    })
    .await
}
