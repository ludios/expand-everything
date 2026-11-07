use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A game whose page has reviews that are cut off above a "Read More" button.
const GAME: &str = "https://store.steampowered.com/app/1147890/Bonfire_Peaks/";

/// An Early Access game whose page cuts off the Early Access information
/// above a "READ MORE" link.
const EARLY_ACCESS_GAME: &str = "https://store.steampowered.com/app/1592280/Selaco/";

/// A game whose page cuts off two sections, the Awards section and the "About
/// This Game" section, each above its own "READ MORE".
const AWARDED_GAME: &str = "https://store.steampowered.com/app/1145360/Hades/";

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
async fn reviews_are_expanded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, GAME).await?;
        // Steam loads the reviews once they are near the viewport, so each
        // evaluation scrolls to them. The class names of a review don't say
        // whether it is expanded, so the expression checks that every "Read
        // More" is hidden.
        let expression = page_expression(
            r##"(() => {
                document.querySelector("#app_reviews_hash")?.scrollIntoView();
                window.scrollBy(0, 600);
                const buttons = [...document.querySelectorAll("#app_reviews_hash button")].filter((b) => b.innerText === "Read More");
                return buttons.length > 0 && buttons.every((b) => !b.checkVisibility());
            })()"##,
        );
        wait_for(
            &tab,
            "the userscript to click \"Read More\" on every long review",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |p: &Page| p.expanded && p.clicked.iter().any(|text| text == "Read More"),
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn early_access_information_is_expanded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, EARLY_ACCESS_GAME).await?;
        // Only the onclick of "READ MORE" sets the max-height of the Early
        // Access information to `none` (and hides the link). Steam also hides
        // the link, without the onclick, if the information is short.
        let expression = page_expression(
            r##"document.querySelector("#devnotes_expander")?.style.maxHeight === "none"
                && document.querySelector("#devnotes_more")?.checkVisibility() === false"##,
        );
        wait_for(
            &tab,
            "the userscript to click \"READ MORE\" on the Early Access information",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(1),
            |p: &Page| p.expanded && p.clicked.iter().any(|text| text == "READ MORE"),
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn awards_and_description_are_expanded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, AWARDED_GAME).await?;
        // Steam replaces the class "collapsed" of a section's wrapper with
        // "expanded" (which hides the section's "READ MORE") only when it
        // expands a section that it had cut off.
        let expression = page_expression(
            r##"["#awardsTable .game_page_autocollapse", "#aboutThisGame"]
                .map((css) => document.querySelector(css))
                .every((section) => section?.parentElement.classList.contains("expanded") && section.scrollHeight <= section.clientHeight)"##,
        );
        wait_for(
            &tab,
            "the userscript to click \"READ MORE\" on the Awards and on \"About This Game\"",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(1),
            |p: &Page| p.expanded && p.clicked.iter().any(|text| text == "READ MORE"),
        )
        .await?;
        Ok(())
    })
    .await
}
