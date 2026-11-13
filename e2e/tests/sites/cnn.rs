use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A live news page that cuts off each long update above a "Read more".
const LIVE_NEWS: &str =
    "https://www.cnn.com/2026/11/12/business/live-news/microsoft-global-outage-intl-hnk";

/// What a test waits for.
#[derive(Debug, Deserialize)]
struct Page {
    /// Whether the page shows in full what the test expects the userscript
    /// to have expanded.
    expanded: bool,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

/// A JavaScript expression for the [`Page`], given `expanded`, a JavaScript
/// expression for [`Page::expanded`].
fn page_expression(expanded: &str) -> String {
    format!("({{ expanded: {expanded}, clicked: {CLICKED_TEXTS} }})")
}

#[tokio::test]
async fn long_updates_are_expanded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, LIVE_NEWS).await?;
        // CNN adds the classes "live-story-post--collapsible" and
        // "live-story-post--collapsed" to each update that it cuts off, and
        // removes only "live-story-post--collapsed" when it expands one.
        let expression = page_expression(
            r##"(() => {
                const updates = [...document.querySelectorAll("article.liveStoryPost")];
                return updates.some((u) => u.classList.contains("live-story-post--collapsible"))
                    && !updates.some((u) => u.classList.contains("live-story-post--collapsed"));
            })()"##,
        );
        wait_for(
            &tab,
            "the userscript to click \"Read more\" on every long update",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |p: &Page| p.expanded && p.clicked.iter().any(|text| text == "Read more"),
        )
        .await?;
        Ok(())
    })
    .await
}
