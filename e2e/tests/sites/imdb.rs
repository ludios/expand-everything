//! In place of a film's reviews, IMDb shows a visitor who isn't logged in
//! only a few "Featured reviews" and a "Sign in to see all reviews" card,
//! plus the one review that the page's address names (if it names one). So
//! the tests of a film's reviews load their page as a user who is logged in.

use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome, with_login};
use serde::Deserialize;
use std::time::Duration;
use zendriver::Tab;

/// A lengthy review, on the page of its film's reviews. The page cuts the
/// review off under a "See more" button.
const FEATURED_REVIEW: &str = "https://www.imdb.com/title/tt0809535/reviews/?featured=rw2027839";

/// [`FEATURED_REVIEW`] on the German pages, which IMDb serves under `/de/`.
/// The "See more" button is "Mehr anzeigen" there.
const GERMAN_FEATURED_REVIEW: &str =
    "https://www.imdb.com/de/title/tt0809535/reviews/?featured=rw2027839";

/// The reviews of a film that has only 8 of them. The page hides some
/// because of spoilers, each behind a "Spoiler" button, and cuts off the
/// lengthy ones, each under a "See more" button.
const FEW_REVIEWS: &str = "https://www.imdb.com/title/tt0809535/reviews/";

/// The reviews of a film that has thousands of them. The page shows the first
/// 25 above a "25 more" button, which adds the next 25 to the page.
const MANY_REVIEWS: &str = "https://www.imdb.com/title/tt6710474/reviews/";

/// Who a test loads its page as.
enum Visitor {
    /// Someone who isn't logged in to IMDb.
    LoggedOut,
    /// The user of the IMDb session that [`with_login`] finds the cookies of.
    LoggedIn,
}

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
    /// The sum of the heights of the texts of the reviews that the page
    /// doesn't hide, in CSS pixels, including the parts that it cuts off.
    text_height: f64,
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
                text_height: texts.reduce((sum, t) => sum + t.scrollHeight, 0),
                more_offered: more !== null && !more.disabled,
                clicked: {CLICKED_TEXTS},
            }};
        }})()"#
    )
}

/// Open `url` as `visitor`, and run `work` with the tab that shows the page.
///
/// Returns what `work` returns, or an error if [`Visitor::LoggedIn`]'s
/// cookies don't log in to IMDb (see [`with_login`]).
async fn with_page(
    url: &str,
    visitor: Visitor,
    work: impl AsyncFnOnce(&Tab) -> Result<()>,
) -> Result<()> {
    if let Visitor::LoggedIn = visitor {
        // The header's account menu, as it is for a user who is logged in
        let logged_in = r#"document.querySelector(".imdb-header__account-toggle--logged-in")"#;
        return with_login("imdb", url, logged_in, work).await;
    }
    with_chrome(async |browser| {
        let tab = open(browser, url).await?;
        work(&tab).await
    })
    .await
}

/// Wait until `done` has been true of the [`Page`] in `tab` for 3 seconds in
/// a row.
///
/// `what` describes what `done` waits for, for the error message.
///
/// Returns the last [`Page`], or an error if 30 seconds elapse first.
async fn expect(tab: &Tab, what: &str, done: impl Fn(&Page) -> bool) -> Result<Page> {
    wait_for(
        tab,
        what,
        &page_expression(),
        Duration::from_secs(30),
        Duration::from_secs(3),
        done,
    )
    .await
}

#[tokio::test]
async fn featured_review_is_expanded() -> Result<()> {
    with_page(FEATURED_REVIEW, Visitor::LoggedOut, async |tab| {
        expect(
            tab,
            "the userscript to click \"See more\" on the review",
            |p| p.reviews == 1 && p.cut_off == 0 && p.clicks_on("See more") > 0,
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn featured_review_is_expanded_on_the_german_pages() -> Result<()> {
    with_page(GERMAN_FEATURED_REVIEW, Visitor::LoggedOut, async |tab| {
        expect(
            tab,
            "the userscript to click \"Mehr anzeigen\" on the review",
            |p| p.reviews == 1 && p.cut_off == 0 && p.clicks_on("Mehr anzeigen") > 0,
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn expanded_review_is_not_cut_off_once_narrower() -> Result<()> {
    // IMDb limits the height of an expanded text to the height that the text
    // had when it was expanded, so the userscript must remove the limit.
    with_page(FEATURED_REVIEW, Visitor::LoggedOut, async |tab| {
        let expanded = expect(
            tab,
            "the userscript to click \"See more\" on the review",
            |p| p.reviews == 1 && p.cut_off == 0 && p.clicks_on("See more") > 0,
        )
        .await?;
        // The text of a narrower review has more lines, as it has in a
        // narrower window.
        tab.evaluate_main::<String>(
            r#"document.querySelector("article.user-review-item").style.width = "300px""#,
        )
        .await?;
        expect(
            tab,
            "the review to get taller than it was when it was expanded, without being cut off",
            |p| p.reviews == 1 && p.cut_off == 0 && p.text_height > expanded.text_height,
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
#[ignore = "needs a login (see `with_login`)"]
async fn lengthy_reviews_are_expanded() -> Result<()> {
    with_page(FEW_REVIEWS, Visitor::LoggedIn, async |tab| {
        expect(
            tab,
            "the userscript to click \"See more\" on every lengthy review",
            |p| p.reviews > 0 && p.cut_off == 0 && p.clicks_on("See more") > 0,
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
#[ignore = "needs a login (see `with_login`)"]
async fn spoiler_reviews_are_shown() -> Result<()> {
    with_page(FEW_REVIEWS, Visitor::LoggedIn, async |tab| {
        expect(
            tab,
            "the userscript to click \"Spoiler\" on every review hidden because of spoilers",
            |p| p.reviews > 0 && p.hidden_reviews == 0 && p.clicks_on("Spoiler") > 0,
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
#[ignore = "needs a login (see `with_login`)"]
async fn more_reviews_are_loaded_until_150() -> Result<()> {
    // The userscript must click "25 more" just once for each 25 reviews that
    // it loads, although IMDb may disable the button only some time after a
    // click.
    with_page(MANY_REVIEWS, Visitor::LoggedIn, async |tab| {
        expect(
            tab,
            "the userscript to click \"25 more\" 5 times, which makes 150 reviews, and then leave \
             it alone, and to leave no review hidden or cut off",
            |p| {
                p.reviews == 150
                    && p.more_offered
                    && p.clicks_on("25 more") == 5
                    && p.hidden_reviews == 0
                    && p.cut_off == 0
            },
        )
        .await?;
        Ok(())
    })
    .await
}
