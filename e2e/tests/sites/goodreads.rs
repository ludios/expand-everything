use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A book whose page cuts off the text about its author and many of its
/// reviews, each above a "Show more" button.
const BOOK: &str =
    "https://www.goodreads.com/book/show/931984.The_Presentation_of_Self_in_Everyday_Life";

/// What a test waits for.
#[derive(Debug, Deserialize)]
struct Page {
    /// How many reviews the page has expanded.
    expanded_reviews: usize,
    /// Whether the page has expanded the text about the author.
    author_expanded: bool,
    /// How many texts the page still cuts off.
    cut_off: usize,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

/// A JavaScript expression for the [`Page`].
fn page_expression() -> String {
    // Goodreads puts each text that it may cut off in a `contentContainer`,
    // cuts the text off by limiting the container's height, and shows
    // "Show more" if the text is taller than that. Clicking "Show more" gives
    // the container the class `TruncatedContent__text--expanded`.
    format!(
        r#"(() => {{
            const EXPANDED = "TruncatedContent__text--expanded";
            const expanded = (css) => document.querySelector(css)?.classList.contains(EXPANDED) ?? false;
            const texts = [...document.querySelectorAll('[data-testid="contentContainer"]')];
            return {{
                expanded_reviews: document.querySelectorAll(`.ReviewCard .${{EXPANDED}}`).length,
                author_expanded: expanded('.PageSection > .TruncatedContent > [data-testid="contentContainer"]'),
                cut_off: texts.filter((t) => !t.classList.contains(EXPANDED) && t.scrollHeight > t.clientHeight).length,
                clicked: {CLICKED_TEXTS},
            }};
        }})()"#
    )
}

#[tokio::test]
async fn reviews_and_author_are_expanded() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, BOOK).await?;
        wait_for(
            &tab,
            "the userscript to click \"Show more\" on the text about the author and on every long review",
            &page_expression(),
            Duration::from_secs(30),
            Duration::from_secs(2),
            |p: &Page| {
                p.cut_off == 0
                    && p.author_expanded
                    && p.expanded_reviews > 0
                    && p.clicked.iter().any(|text| text == "Show more")
            },
        )
        .await?;
        Ok(())
    })
    .await
}
