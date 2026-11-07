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

/// The state of the buttons or links of one kind on the page.
#[derive(Debug, Deserialize)]
struct Buttons {
    /// How many there are, shown or not.
    total: usize,
    /// How many are shown.
    shown: usize,
    clicked: Vec<String>,
}

/// A JavaScript expression for the [`Buttons`] that match the CSS selector
/// `css` (and whose text is `text`, if given), evaluated after `before` (a
/// statement).
fn buttons_expression(before: &str, css: &str, text: Option<&str>) -> String {
    let css = serde_json::to_string(css).expect("strings serialize");
    let text = serde_json::to_string(&text).expect("strings serialize");
    format!(
        r#"(() => {{
            {before}
            const text = {text};
            const buttons = [...document.querySelectorAll({css})].filter((b) => text === null || b.innerText === text);
            return {{
                total: buttons.length,
                shown: buttons.filter((b) => b.checkVisibility()).length,
                clicked: {CLICKED_TEXTS},
            }};
        }})()"#
    )
}

#[tokio::test]
async fn reviews_are_expanded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, GAME).await?;
        // Steam loads the reviews once they are near the viewport, so each
        // evaluation scrolls to them.
        let expression = buttons_expression(
            r##"document.querySelector("#app_reviews_hash")?.scrollIntoView(); window.scrollBy(0, 600);"##,
            "#app_reviews_hash button",
            Some("Read More"),
        );
        wait_for(
            &tab,
            "the userscript to click \"Read More\" on every long review",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |b: &Buttons| {
                b.total > 0 && b.shown == 0 && b.clicked.iter().any(|text| text == "Read More")
            },
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
        // Once hidden, the link's text isn't in capitals, so it's not checked.
        let expression = buttons_expression("", "a.morebutton", None);
        wait_for(
            &tab,
            "the userscript to click \"READ MORE\" on the Early Access information",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(1),
            |b: &Buttons| b.total > 0 && b.shown == 0 && b.clicked == ["READ MORE"],
        )
        .await?;
        Ok(())
    })
    .await
}
