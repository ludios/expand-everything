use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A gist with more than 1300 comments, of which GitHub first shows the
/// newest 60, below a "Load earlier comments..." button that loads more of
/// them each time it is clicked.
const GIST: &str = "https://gist.github.com/ivan/5095670735ba941a6090a69fce4183df";

/// What the test waits for.
#[derive(Debug, Deserialize)]
struct Page {
    /// The URL of the page, which is no longer [`GIST`] if a click on the
    /// button navigated to the `/load_comments` page.
    url: String,
    /// Whether the page still has the button's form, which GitHub removes
    /// once it has loaded the oldest comments.
    cut_off: bool,
    /// How many comments the page shows.
    comments: usize,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

#[tokio::test]
async fn comments_are_all_loaded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, GIST).await?;
        // GitHub replaces the button with a new one each time it has loaded
        // more comments, so more than one click shows that the userscript
        // clicked each new button. Loading every comment takes more than 40
        // clicks, which is why the timeout is long.
        wait_for(
            &tab,
            "the userscript to click \"Load earlier comments...\" until every comment is loaded",
            &format!(
                r#"({{
                    url: location.href,
                    cut_off: document.querySelector('form[action$="/load_comments"]') !== null,
                    comments: document.querySelectorAll('[id^="gistcomment-"]').length,
                    clicked: {CLICKED_TEXTS},
                }})"#
            ),
            Duration::from_secs(120),
            Duration::from_secs(2),
            |p: &Page| {
                let load_clicks = p
                    .clicked
                    .iter()
                    .filter(|t| *t == "Load earlier comments...");
                p.url == GIST && !p.cut_off && p.comments > 0 && load_clicks.count() >= 2
            },
        )
        .await?;
        Ok(())
    })
    .await
}
