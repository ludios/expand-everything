use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome, with_login};
use serde::Deserialize;
use std::time::Duration;

/// A book whose page cuts off the text about its author and many of its
/// reviews, each above a "Show more" button.
const BOOK: &str =
    "https://www.goodreads.com/book/show/931984.The_Presentation_of_Self_in_Everyday_Life";

/// A book whose page cuts off its description, the text about its author,
/// and some of its reviews, each above a "Show more" button.
const DESCRIBED_BOOK: &str = "https://www.goodreads.com/book/show/39736150-the-little-typer";

/// A book whose page cuts off its description, the text about its author,
/// and many of its reviews, each above a "Show more" button, and hides a few
/// reviews because of spoilers, each above a "Show full review" button.
const SPOILED_BOOK: &str = "https://www.goodreads.com/book/show/7260188-mockingjay";

/// The list of a user's reviews, which cuts off each long review above a
/// "...more" link. Goodreads redirects a visitor who isn't logged in to its
/// sign-in page.
const REVIEW_LIST: &str = "https://www.goodreads.com/review/list/35831082?utf8=%E2%9C%93&order=d&sort=review&view=reviews&per_page=infinite";

/// What a test waits for.
#[derive(Debug, Deserialize)]
struct Page {
    /// How many reviews the page has expanded.
    expanded_reviews: usize,
    /// Whether the page has expanded the text about the author.
    author_expanded: bool,
    /// Whether the page has expanded the book description.
    description_expanded: bool,
    /// How many texts the page still cuts off.
    cut_off: usize,
    /// How many reviews the page doesn't show the text of (as for a review
    /// hidden because of spoilers).
    hidden_reviews: usize,
    /// How many passages of reviews are marked as spoilers.
    spoiler_passages: usize,
    /// How many of the [`Page::spoiler_passages`] the page hides.
    hidden_passages: usize,
    /// How far the page is scrolled down, in CSS pixels.
    scroll_y: f64,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

impl Page {
    /// Whether one of the userscript's clicks was on an element whose text
    /// is `text`.
    fn clicked_on(&self, text: &str) -> bool {
        self.clicked.iter().any(|t| t == text)
    }
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
                author_expanded: expanded('.AuthorPreview ~ .TruncatedContent > [data-testid="contentContainer"]'),
                description_expanded: expanded('[data-testid="description"] [data-testid="contentContainer"]'),
                cut_off: texts.filter((t) => !t.classList.contains(EXPANDED) && t.scrollHeight > t.clientHeight).length,
                hidden_reviews: [...document.querySelectorAll(".ReviewCard")].filter((c) => !c.querySelector(".ReviewText__content")?.checkVisibility()).length,
                spoiler_passages: document.querySelectorAll(".ReviewCard input.spoiler__control").length,
                hidden_passages: document.querySelectorAll(".ReviewCard input.spoiler__control:not(:checked)").length,
                scroll_y: window.scrollY,
                clicked: {CLICKED_TEXTS},
            }};
        }})()"#
    )
}

/// Open `url`, and wait until `done` has been true of the [`Page`] for 3
/// seconds in a row.
///
/// `what` describes what `done` waits for, for the error message.
///
/// Returns an error if 30 seconds elapse first.
async fn expect(url: &str, what: &str, done: impl Fn(&Page) -> bool) -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, url).await?;
        wait_for(
            &tab,
            what,
            &page_expression(),
            Duration::from_secs(30),
            Duration::from_secs(3),
            done,
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn reviews_and_author_are_expanded() -> Result<()> {
    expect(
        BOOK,
        "the userscript to click \"Show more\" on the text about the author and on every long review",
        |p| {
            p.cut_off == 0
                && p.author_expanded
                && p.expanded_reviews > 0
                && p.clicked_on("Show more")
        },
    )
    .await
}

#[tokio::test]
async fn description_and_reviews_are_expanded() -> Result<()> {
    expect(
        DESCRIBED_BOOK,
        "the userscript to click \"Show more\" on the book description and on every long review",
        |p| {
            p.cut_off == 0
                && p.description_expanded
                && p.expanded_reviews > 0
                && p.clicked_on("Show more")
        },
    )
    .await
}

#[tokio::test]
async fn page_does_not_scroll() -> Result<()> {
    // Goodreads focuses each text that "Show more" expands, which scrolls
    // the page to the text unless the userscript prevents the focusing.
    expect(
        SPOILED_BOOK,
        "the userscript to show every review hidden because of spoilers, and to click \"Show more\" \
         on the book description, the text about the author, and every long review, without the \
         page scrolling down",
        |p| {
            p.cut_off == 0
                && p.hidden_reviews == 0
                && p.description_expanded
                && p.author_expanded
                && p.expanded_reviews > 0
                && p.scroll_y == 0.0
                && p.clicked_on("Show full review")
                && p.clicked_on("Show more")
        },
    )
    .await
}

#[tokio::test]
async fn spoiler_reviews_are_shown_and_expanded() -> Result<()> {
    // Goodreads hides a review whose text is marked as having spoilers with
    // `display: none`, so `cut_off` doesn't count it until it is shown.
    expect(
        SPOILED_BOOK,
        "the userscript to click \"Show full review\" on every review hidden because of spoilers, \
         and \"Show more\" on every long review",
        |p| {
            p.hidden_reviews == 0
                && p.cut_off == 0
                && p.expanded_reviews > 0
                && p.clicked_on("Show full review")
                && p.clicked_on("Show more")
        },
    )
    .await
}

#[tokio::test]
async fn spoiler_passages_are_shown() -> Result<()> {
    // Goodreads blurs a passage that is marked as a spoiler until the
    // checkbox before it is checked.
    expect(
        SPOILED_BOOK,
        "the userscript to check the checkbox before every passage that is marked as a spoiler",
        |p| {
            p.spoiler_passages > 0
                && p.hidden_passages == 0
                && p.clicked_on(
                    "The following text has been marked spoiler. Toggle checkbox to reveal or hide.",
                )
        },
    )
    .await
}

#[tokio::test]
#[ignore = "needs a login (see `with_login`)"]
async fn review_list_reviews_are_expanded_when_logged_in() -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct List {
        /// How many reviews the page has with a "...more" link (which says
        /// "(less)" once clicked).
        links: usize,
        /// How many reviews the page still cuts off.
        cut_off: usize,
        clicked: Vec<String>,
    }
    // Goodreads redirects a visitor who isn't logged in to its sign-in page.
    let logged_in = r#"location.pathname.startsWith("/review/list/")"#;
    with_login("goodreads", REVIEW_LIST, logged_in, async |tab| {
        // Each review has its text twice: cut off in `freeTextContainer{id}`,
        // and in full in `freeText{id}`, which "...more" shows instead.
        let expression = format!(
            r#"(() => {{
                const links = [...document.querySelectorAll('a[data-text-id^="review"]')];
                return {{
                    links: links.length,
                    cut_off: links.filter((a) => !document.getElementById(`freeText${{a.dataset.textId}}`)?.checkVisibility()).length,
                    clicked: {CLICKED_TEXTS},
                }};
            }})()"#
        );
        wait_for(
            tab,
            "the userscript to click \"...more\" on every long review",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(3),
            |l: &List| {
                l.links > 0 && l.cut_off == 0 && l.clicked.iter().any(|text| text == "...more")
            },
        )
        .await?;
        Ok(())
    })
    .await
}
