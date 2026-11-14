use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// The page of an author who has thousands of stories. It lists the latest 20
/// above a "Load more" button, which adds the next 20 to the page. Bloomberg
/// lists at most 400 stories there: after that, the button stays but adds
/// nothing.
const PROLIFIC_AUTHOR: &str =
    "https://www.bloomberg.com/opinion/authors/ARbTQlRLRjE/matthew-s-levine";

/// What the test waits for.
#[derive(Debug, Deserialize)]
struct Page {
    /// How many stories the page lists.
    stories: usize,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

#[tokio::test]
async fn load_more_is_clicked_until_it_adds_no_more_stories() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, PROLIFIC_AUTHOR).await?;
        // Each story on the page has one headline.
        let expression = format!(
            r#"({{
                stories: document.querySelectorAll('[data-component="headline"]').length,
                clicked: {CLICKED_TEXTS},
            }})"#
        );
        // 19 clicks add 380 stories to the first 20, and the userscript stops
        // after 2 more clicks that add nothing. A click made before the page
        // has fetched the next stories adds nothing either, and makes the
        // page fetch them again; 25 allows for a few of those. Had the
        // userscript not stopped, it would exceed 25 clicks within the 10
        // seconds.
        wait_for(
            &tab,
            "the userscript to click \"Load more\" until the page lists about 400 stories, and then to stop",
            &expression,
            Duration::from_secs(120),
            Duration::from_secs(10),
            |p: &Page| {
                let clicks = p.clicked.iter().filter(|text| *text == "Load more").count();
                p.stories >= 380 && (19..=25).contains(&clicks)
            },
        )
        .await?;
        Ok(())
    })
    .await
}
