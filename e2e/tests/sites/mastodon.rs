use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A post that has a content warning, and whose replies mostly have one too.
/// Mastodon covers the text of such a post until the "Show post" button of
/// the post's content warning is clicked.
const THREAD: &str = "https://mastodon.social/@Gargron/117588712201630497";

/// A lower bound on how many posts on the page of [`THREAD`] have a content
/// warning. The page had 26 of them when the test was written; replies can
/// still be added and deleted.
const MIN_WARNED_POSTS: usize = 20;

/// What the test waits for.
#[derive(Debug, Deserialize)]
struct Page {
    /// How many posts have a content warning.
    warned_posts: usize,
    /// How many of those posts show their text.
    shown_posts: usize,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

#[tokio::test]
async fn posts_with_a_content_warning_are_expanded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, THREAD).await?;
        // A content warning names, in `aria-controls`, the element that holds
        // the text of its post. While the post is collapsed, that element is
        // `inert`, and its `::after` covers it.
        let expression = format!(
            r#"(() => {{
                const warnings = [...document.querySelectorAll("article div[aria-controls]")];
                const shown = (w) => {{
                    const text = document.getElementById(w.getAttribute("aria-controls"));
                    return text !== null
                        && !text.inert
                        && getComputedStyle(text, "::after").visibility === "hidden";
                }};
                return {{
                    warned_posts: warnings.length,
                    shown_posts: warnings.filter(shown).length,
                    clicked: {CLICKED_TEXTS},
                }};
            }})()"#
        );
        // A second click on a button would collapse its post again, so the
        // userscript must click each button exactly once.
        wait_for(
            &tab,
            "the userscript to expand every post that has a content warning by clicking each \"Show post\" once",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |p: &Page| {
                p.warned_posts >= MIN_WARNED_POSTS
                    && p.shown_posts == p.warned_posts
                    && p.clicked.len() == p.warned_posts
                    && p.clicked.iter().all(|t| t == "Show post")
            },
        )
        .await?;
        Ok(())
    })
    .await
}
