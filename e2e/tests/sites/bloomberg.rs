//! Each page here lists stories above a "Load more" button, which adds the
//! next ones to the page. Bloomberg stops at the 20th batch: after that, the
//! button stays but adds nothing.

use anyhow::Result;
use expand_everything_e2e::userscript::HARNESS_GLOBAL;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// The page of an author who has thousands of stories, in batches of 20.
const PROLIFIC_AUTHOR: &str =
    "https://www.bloomberg.com/opinion/authors/ARbTQlRLRjE/matthew-s-levine";

/// The latest stories of all authors, in batches of 25. The page requests
/// them from another API than an author's page does. Stories published while
/// the batches load shift the later batches, so the page ends up with a few
/// more or fewer stories than 500.
const LATEST: &str = "https://www.bloomberg.com/latest";

/// What a test waits for.
#[derive(Debug, Deserialize)]
struct Page {
    /// How many stories the page lists.
    stories: usize,
    /// How many times the userscript clicked "Load more".
    clicks: usize,
    /// How many milliseconds ago the userscript last clicked anything, or
    /// `None` if it hasn't clicked.
    idle_ms: Option<f64>,
}

/// Open `url`, and wait until the page lists at least `min_stories` stories
/// (more than 19 batches have) and the userscript has stopped clicking
/// "Load more".
///
/// Returns an error if 120 seconds elapse first.
async fn expect_all_batches(url: &str, min_stories: usize) -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, url).await?;
        // Each story on the page has one headline.
        let expression = format!(
            r#"(() => {{
                const clicks = window.{HARNESS_GLOBAL}.clicks;
                return {{
                    stories: document.querySelectorAll('[data-component="headline"]').length,
                    clicks: clicks.filter((c) => c.text === "Load more").length,
                    idle_ms: clicks.length > 0 ? performance.now() - clicks.at(-1).time : null,
                }};
            }})()"#
        );
        // 19 clicks add the other 19 batches, and the userscript stops after
        // 2 more clicks that add nothing. A few clicks more are allowed,
        // because a click that comes before the page is ready adds nothing
        // either. While the userscript hasn't stopped, it clicks again within
        // about 2 seconds.
        wait_for(
            &tab,
            &format!("the userscript to click \"Load more\" until the page lists at least {min_stories} stories, and then to stop"),
            &expression,
            Duration::from_secs(120),
            Duration::ZERO,
            |p: &Page| {
                p.stories >= min_stories
                    && (19..=30).contains(&p.clicks)
                    && p.idle_ms.is_some_and(|ms| ms >= 10_000.0)
            },
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn all_batches_of_an_author_are_listed() -> Result<()> {
    expect_all_batches(PROLIFIC_AUTHOR, 400).await
}

#[tokio::test]
async fn all_batches_of_the_latest_stories_are_listed() -> Result<()> {
    expect_all_batches(LATEST, 490).await
}
