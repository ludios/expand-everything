use anyhow::Result;
use expand_everything_e2e::userscript::HARNESS_GLOBAL;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A game whose page has reviews that are cut off above a "Read More" button.
const GAME: &str = "https://store.steampowered.com/app/1147890/Bonfire_Peaks/";

/// An Early Access game whose page cuts off the Early Access information
/// above a "READ MORE" link.
const EARLY_ACCESS_GAME: &str = "https://store.steampowered.com/app/1592280/Selaco/";

/// A game whose page cuts off its Awards and its "About This Game" above a
/// "READ MORE" button each.
const AWARDED_GAME: &str = "https://store.steampowered.com/app/1145360/Hades/";

/// The state of the page, as far as one kind of button or link on it goes.
#[derive(Debug, Deserialize)]
struct Buttons {
    /// How many of the buttons or links there are, whether shown or not.
    total: usize,
    /// How many of the buttons or links are shown (with `display`, and the
    /// `display` of every ancestor, not `none`).
    shown: usize,
    /// For each click that the userscript made on one of the buttons or links
    /// (whatever its text), the text that it had just before the click, in
    /// the order of the clicks.
    clicked: Vec<String>,
}

/// A JavaScript expression, evaluated after the JavaScript statements
/// `before`, for the [`Buttons`] that match the CSS selector `css`. If `text`
/// is given, [`Buttons::total`] and [`Buttons::shown`] count only the buttons
/// whose text is `text`.
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
                clicked: window.{HARNESS_GLOBAL}.clicks.filter((c) => c.element.matches({css})).map((c) => c.text),
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
        // The link's text is in capitals only through CSS, and once the link
        // is hidden, its `innerText` is no longer in capitals, so the
        // expression doesn't check the text.
        let expression = buttons_expression("", "a.morebutton", None);
        wait_for(
            &tab,
            "the userscript to click \"READ MORE\" on the Early Access information",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(1),
            |b: &Buttons| {
                b.total > 0 && b.shown == 0 && b.clicked.iter().any(|text| text == "READ MORE")
            },
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
        // The "READ MORE" of the Awards, and the "READ MORE" of "About This Game".
        let expression = buttons_expression(
            "",
            "#awardsTable .game_page_autocollapse_readmore, #aboutThisGame ~ .game_page_autocollapse_fade > .game_page_autocollapse_readmore",
            None,
        );
        wait_for(
            &tab,
            "the userscript to click \"READ MORE\" on the Awards and on \"About This Game\"",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(1),
            |b: &Buttons| b.total == 2 && b.shown == 0 && b.clicked.len() == 2,
        )
        .await?;
        Ok(())
    })
    .await
}
