use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A live coverage page that cuts off each of its 10 long posts above a
/// "Show more" button.
const LIVE: &str = "https://www.nytimes.com/live/2024/07/07/world/france-election-2024";

/// What a test waits for.
#[derive(Debug, Deserialize)]
struct Page {
    /// How many posts the page can cut off.
    long_posts: usize,
    /// How many of the [`Page::long_posts`] the page shows in full.
    expanded_posts: usize,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

#[tokio::test]
async fn long_posts_are_expanded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, LIVE).await?;
        // The button below a long post names, in `aria-controls`, the element
        // with the rest of the post, which the page hides until the button is
        // clicked.
        let expression = format!(
            r##"(() => {{
                const buttons = [...document.querySelectorAll('button[data-testid="Show-More"]')];
                const shown = (b) => document.getElementById(b.getAttribute("aria-controls"))?.checkVisibility() === true;
                return {{
                    long_posts: buttons.length,
                    expanded_posts: buttons.filter(shown).length,
                    clicked: {CLICKED_TEXTS},
                }};
            }})()"##
        );
        wait_for(
            &tab,
            "the userscript to click \"Show more\" below every long post",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |p: &Page| {
                p.long_posts > 0
                    && p.expanded_posts == p.long_posts
                    && p.clicked.iter().any(|text| text == "Show more")
            },
        )
        .await?;
        Ok(())
    })
    .await
}
