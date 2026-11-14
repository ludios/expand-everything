use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A live coverage page with [`LONG_POSTS`] long posts, each of which it
/// cuts off above a "Show more" button.
const LIVE: &str = "https://www.nytimes.com/live/2024/07/07/world/france-election-2024";

/// How many long posts [`LIVE`] has.
const LONG_POSTS: usize = 10;

/// What a test waits for.
#[derive(Debug, Deserialize)]
struct Page {
    /// How many posts have a button that expands or collapses the post.
    long_posts: usize,
    /// How many of those posts are expanded.
    expanded_posts: usize,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

#[tokio::test]
async fn long_posts_are_expanded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, LIVE).await?;
        // The button below a long post names, in `aria-controls`, the element
        // that holds the rest of the post. The page doesn't display that
        // element until the button is clicked, which also changes the
        // button's `aria-expanded` to "true" and its text to "Show less".
        let expression = format!(
            r#"(() => {{
                const buttons = [...document.querySelectorAll('button[data-testid="Show-More"]')];
                const expanded = (b) => b.getAttribute("aria-expanded") === "true"
                    && b.textContent === "Show less"
                    && document.getElementById(b.getAttribute("aria-controls"))?.checkVisibility();
                return {{
                    long_posts: buttons.length,
                    expanded_posts: buttons.filter(expanded).length,
                    clicked: {CLICKED_TEXTS},
                }};
            }})()"#
        );
        wait_for(
            &tab,
            "the userscript to expand every long post by clicking each \"Show more\" once",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |p: &Page| {
                p.long_posts == LONG_POSTS
                    && p.expanded_posts == LONG_POSTS
                    && p.clicked == ["Show more"; LONG_POSTS]
            },
        )
        .await?;
        Ok(())
    })
    .await
}
