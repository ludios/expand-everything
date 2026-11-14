use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A gist with more than 650 comments, of which GitHub first shows the
/// newest 30, below a "Load earlier comments..." button that loads 30 more
/// each time it is clicked.
const GIST: &str = "https://gist.github.com/ivan/5095670735ba941a6090a69fce4183df";

/// The `id` of the element of the oldest comment on [`GIST`].
const OLDEST_COMMENT_ID: &str = "gistcomment-4423762";

/// What the test waits for.
#[derive(Debug, Deserialize)]
struct Page {
    /// The URL of the page. (A click on the button that GitHub's JavaScript
    /// doesn't handle submits the button's form, which navigates to a
    /// `/load_comments` URL.)
    url: String,
    /// Whether the page still has the button's form. GitHub removes it once
    /// it has loaded the oldest comments, and also if loading fails with a
    /// 404.
    cut_off: bool,
    /// Whether the page shows the comment whose `id` is
    /// [`OLDEST_COMMENT_ID`].
    oldest_comment_shown: bool,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

#[tokio::test]
async fn comments_are_all_loaded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, GIST).await?;
        // GitHub replaces the button with a new one each time it has loaded
        // more comments, so a second click shows that the userscript also
        // clicked a new button. Loading every comment takes more than 20
        // clicks, each of which waits for GitHub's response, so the timeout
        // is long.
        wait_for(
            &tab,
            "the userscript to click \"Load earlier comments...\" until the oldest comment is loaded",
            &format!(
                r#"({{
                    url: location.href,
                    cut_off: document.querySelector('form[action$="/load_comments"]') !== null,
                    oldest_comment_shown: document.getElementById("{OLDEST_COMMENT_ID}") !== null,
                    clicked: {CLICKED_TEXTS},
                }})"#
            ),
            Duration::from_secs(120),
            Duration::from_secs(2),
            |p: &Page| {
                let load_clicks = p.clicked.iter().filter(|t| *t == "Load earlier comments...");
                p.url == GIST && !p.cut_off && p.oldest_comment_shown && load_clicks.count() >= 2
            },
        )
        .await?;
        Ok(())
    })
    .await
}
