//! IMDb shows a visitor who isn't logged in only a few "Featured reviews" and
//! a "Sign in to see all reviews" card, so every test here loads its page as
//! a user who is logged in.

use anyhow::{Context, Result, ensure};
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{add_cookies, open, wait_for, with_chrome};
use serde::Deserialize;
use std::path::Path;
use std::time::Duration;

/// The reviews of a film that has only 8 of them. The page hides some
/// because of spoilers, each behind a "Spoiler" button, and cuts off the
/// lengthy ones, each under a "See more" button.
const FEW_REVIEWS: &str = "https://www.imdb.com/title/tt0809535/reviews/";

/// The reviews of a film that has thousands of them. The page shows the first
/// 25 above a "25 more" button, which adds the next 25 to the page.
const MANY_REVIEWS: &str = "https://www.imdb.com/title/tt6710474/reviews/";

/// What a test waits for.
#[derive(Debug, Deserialize)]
struct Page {
    /// How many reviews the page has.
    reviews: usize,
    /// How many reviews the page doesn't show the text of (as for a review
    /// hidden because of spoilers).
    hidden_reviews: usize,
    /// How many reviews the page cuts off.
    cut_off: usize,
    /// Whether the page has a "25 more" button that can be clicked (it can't
    /// be while reviews are loading).
    more_offered: bool,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

impl Page {
    /// How many of the userscript's clicks were on an element whose text is
    /// `text`.
    fn clicks_on(&self, text: &str) -> usize {
        self.clicked.iter().filter(|t| *t == text).count()
    }
}

/// A JavaScript expression for the [`Page`].
fn page_expression() -> String {
    // IMDb puts each review's text in an `ipc-html-content-inner-div`, which
    // a review hidden because of spoilers lacks. A text that isn't hidden is
    // in a `review-overflow`, which may limit its height; over the end of a
    // text that it cuts off (and of every text, until the page's scripts
    // have measured it) is the "See more" button, an `ipc-overflowText-overlay`.
    format!(
        r#"(() => {{
            const reviews = [...document.querySelectorAll("article.user-review-item")];
            const texts = [...document.querySelectorAll('article.user-review-item [data-testid="review-overflow"]')];
            const more = document.querySelector('[data-testid="tturv-pagination"] .single-page-see-more-button > button');
            return {{
                reviews: reviews.length,
                hidden_reviews: reviews.filter((r) => !r.querySelector(".ipc-html-content-inner-div")?.checkVisibility()).length,
                cut_off: texts.filter((t) => t.querySelector(".ipc-overflowText-overlay") !== null || t.scrollHeight > t.clientHeight).length,
                more_offered: more !== null && !more.disabled,
                clicked: {CLICKED_TEXTS},
            }};
        }})()"#
    )
}

/// Open `url` as the user whose cookies.txt file the environment variable
/// `E2E_IMDB_COOKIES` names, and wait until `done` has been true of the
/// [`Page`] for 3 seconds in a row.
///
/// `what` describes what `done` waits for, for the error message.
///
/// Returns an error if the cookies don't log in to IMDb, or if 30 seconds
/// elapse first.
async fn expect(url: &str, what: &str, done: impl Fn(&Page) -> bool) -> Result<()> {
    let cookies = std::env::var_os("E2E_IMDB_COOKIES").context(
        "E2E_IMDB_COOKIES must be the path of a cookies.txt file of a logged-in IMDb session",
    )?;
    with_chrome(async |browser| {
        add_cookies(browser, Path::new(&cookies)).await?;
        let tab = open(browser, url).await?;
        let logged_in: bool = tab
            .evaluate_main(
                r#"document.querySelector(".imdb-header__account-toggle--logged-in") !== null"#,
            )
            .await?;
        ensure!(
            logged_in,
            "the cookies in E2E_IMDB_COOKIES don't log in to IMDb"
        );
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
#[ignore = "needs E2E_IMDB_COOKIES, the path of a cookies.txt file of a logged-in IMDb session"]
async fn lengthy_reviews_are_expanded() -> Result<()> {
    expect(
        FEW_REVIEWS,
        "the userscript to click \"See more\" on every lengthy review",
        |p| p.reviews > 0 && p.cut_off == 0 && p.clicks_on("See more") > 0,
    )
    .await
}

#[tokio::test]
#[ignore = "needs E2E_IMDB_COOKIES, the path of a cookies.txt file of a logged-in IMDb session"]
async fn spoiler_reviews_are_shown() -> Result<()> {
    expect(
        FEW_REVIEWS,
        "the userscript to click \"Spoiler\" on every review hidden because of spoilers",
        |p| p.reviews > 0 && p.hidden_reviews == 0 && p.clicks_on("Spoiler") > 0,
    )
    .await
}

#[tokio::test]
#[ignore = "needs E2E_IMDB_COOKIES, the path of a cookies.txt file of a logged-in IMDb session"]
async fn more_reviews_are_loaded_until_150() -> Result<()> {
    // The userscript must click "25 more" just once for each 25 reviews,
    // although IMDb may disable the button only some time after a click.
    expect(
        MANY_REVIEWS,
        "the userscript to click \"25 more\" 5 times, which makes 150 reviews, and then leave \
         it alone, and to show and expand every review",
        |p| {
            p.reviews == 150
                && p.more_offered
                && p.clicks_on("25 more") == 5
                && p.hidden_reviews == 0
                && p.cut_off == 0
        },
    )
    .await
}
