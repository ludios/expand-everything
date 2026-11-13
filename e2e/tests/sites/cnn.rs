use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A live news page that cuts off each long update above a "Read more", and
/// that hides its older updates until "Load more Live Updates" is clicked.
const LIVE_NEWS: &str =
    "https://www.cnn.com/2026/11/12/business/live-news/microsoft-global-outage-intl-hnk";

/// [`LIVE_NEWS`], linked to its oldest update, which is long. CNN shows only
/// the pinned update and the linked one, and hides "Load more Live Updates".
const LIVE_NEWS_OLDEST_UPDATE: &str = "https://www.cnn.com/2026/11/12/business/live-news/microsoft-global-outage-intl-hnk?post-id=cmweh3n9k003027nubrwoaxfe";

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

#[tokio::test]
async fn all_updates_are_shown() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, LIVE_NEWS).await?;
        // CNN gives the class "live-story-post--hidden-for-button" to each
        // update that it hides until "Load more Live Updates" is clicked, and
        // hides the button once no update is hidden.
        let expression = page_expression(
            r##"document.querySelector("article.live-story-post--hidden-for-button") === null
                && document.querySelector("button.live-story__load-more-updates")?.checkVisibility() === false"##,
        );
        wait_for(
            &tab,
            "the userscript to click \"Load more Live Updates\" until every update is shown",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |p: &Page| p.expanded && p.clicked.iter().any(|text| text == "Load more Live Updates"),
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn hidden_load_more_is_not_clicked() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, LIVE_NEWS_OLDEST_UPDATE).await?;
        let expression = page_expression(
            r##"(() => {
                const updates = [...document.querySelectorAll("article.liveStoryPost")].filter((u) => u.checkVisibility());
                return updates.some((u) => u.classList.contains("live-story-post--collapsible"))
                    && !updates.some((u) => u.classList.contains("live-story-post--collapsed"));
            })()"##,
        );
        wait_for(
            &tab,
            "the userscript to click \"Read more\" on the linked update, and not to click \"Load more Live Updates\"",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(3),
            |p: &Page| {
                p.expanded
                    && p.clicked.iter().any(|text| text == "Read more")
                    && !p.clicked.iter().any(|text| text == "Load more Live Updates")
            },
        )
        .await?;
        Ok(())
    })
    .await
}
