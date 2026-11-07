use anyhow::{Context, Result};
use expand_everything_e2e::userscript::{CLICKED_TEXTS, OBSERVING};
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A video with a description longer than YouTube shows at first, and
/// comments with replies.
const VIDEO: &str = "https://www.youtube.com/watch?v=ben9qDbrLYU";

/// The state of the description of the video on the page.
#[derive(Debug, Deserialize)]
struct Description {
    url: String,
    /// Whether the page shows the whole description.
    expanded: bool,
    clicked: Vec<String>,
    observing: bool,
}

/// A JavaScript expression for a [`Description`].
fn description_expression() -> String {
    format!(
        r##"({{
            url: location.href,
            expanded: document.querySelector("#description-inline-expander")?.hasAttribute("is-expanded") ?? false,
            clicked: {CLICKED_TEXTS},
            observing: {OBSERVING},
        }})"##
    )
}

/// How many of `texts` are "more", the text of the button that expands a
/// description.
fn more_clicks(texts: &[String]) -> usize {
    texts.iter().filter(|text| *text == "more").count()
}

#[tokio::test]
async fn description_is_expanded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, VIDEO).await?;
        wait_for(
            &tab,
            "the userscript to click \"more\" below the description",
            &description_expression(),
            Duration::from_secs(30),
            Duration::from_secs(1),
            |d: &Description| d.expanded && more_clicks(&d.clicked) == 1,
        )
        .await?;
        Ok(())
    })
    .await
}

/// After the page changes to another video without loading a new document,
/// the userscript expands the new video's description, even though it had
/// stopped watching the page before then: this is what the userscript's
/// watching of navigations is for.
#[tokio::test]
async fn description_is_expanded_after_navigating_to_another_video() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, VIDEO).await?;
        // Loading the comments makes the page change enough for the
        // userscript to stop watching it.
        let expression = format!(
            r#"(document.querySelector("ytd-comments#comments")?.scrollIntoView(), {})"#,
            description_expression()
        );
        wait_for(
            &tab,
            "the userscript to click \"more\" below the first description, and stop watching the page",
            &expression,
            Duration::from_secs(60),
            Duration::from_secs(1),
            |d: &Description| d.expanded && more_clicks(&d.clicked) == 1 && !d.observing,
        )
        .await?;
        let related = tab
            .find()
            .css(r#"#related a[href^="/watch?v="]"#)
            .timeout(Duration::from_secs(30))
            .one()
            .await
            .context("finding a related video")?;
        related.click().await.context("clicking a related video")?;
        // Two clicks on "more" are recorded only if the page is still the
        // first document, so the navigation didn't load a new one.
        wait_for(
            &tab,
            "the userscript to click \"more\" below the second description",
            &description_expression(),
            Duration::from_secs(30),
            Duration::from_secs(1),
            |d: &Description| {
                !d.url.starts_with(VIDEO) && d.expanded && more_clicks(&d.clicked) == 2 && d.observing
            },
        )
        .await?;
        Ok(())
    })
    .await
}

/// The userscript stops watching the page soon after it loads, so it opens
/// the replies of only the top comments; check the first comment that has
/// replies.
#[tokio::test]
async fn replies_of_first_comment_are_opened() -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct State {
        /// How many comments (not counting replies) the page shows.
        #[allow(dead_code, reason = "shown in the error message")]
        comments: usize,
        /// Whether the "N replies" button of the first comment with replies
        /// is hidden, which YouTube does once it has been clicked.
        button_hidden: Option<bool>,
        /// How many replies the page shows below that comment.
        replies: Option<usize>,
        clicked: Vec<String>,
    }
    with_chrome(async |browser| {
        let tab = open(browser, VIDEO).await?;
        // YouTube loads the comments once they are near the viewport, so
        // each evaluation scrolls to them.
        let expression = format!(
            r##"(() => {{
                document.querySelector("ytd-comments#comments")?.scrollIntoView();
                const threads = [...document.querySelectorAll("ytd-comment-thread-renderer")];
                const first = threads.find((t) => t.querySelector("#more-replies"));
                return {{
                    comments: threads.length,
                    button_hidden: first?.querySelector("#more-replies").hidden ?? null,
                    replies: first?.querySelectorAll("ytd-comment-replies-renderer ytd-comment-view-model").length ?? null,
                    clicked: {CLICKED_TEXTS},
                }};
            }})()"##
        );
        wait_for(
            &tab,
            "the userscript to open the replies of the first comment with replies",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(1),
            |s: &State| {
                s.button_hidden == Some(true)
                    && s.replies.is_some_and(|n| n > 0)
                    && s.clicked.iter().any(|text| text.ends_with(" replies") || text.ends_with(" reply"))
            },
        )
        .await?;
        Ok(())
    })
    .await
}
