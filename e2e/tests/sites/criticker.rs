use anyhow::{Context, Result, ensure};
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{add_cookies, open, wait_for, with_chrome};
use serde::Deserialize;
use std::path::Path;
use std::time::Duration;
use zendriver::Tab;

/// A film whose page lists its ratings, 100 at a time, in the compact view
/// (if the account has "Compact view" switched on), which cuts off each
/// minireview before a "More" link. Without the cookies of a logged-in
/// session, the tests' Chrome gets a Cloudflare challenge instead of the
/// page.
const FILM: &str = "https://www.criticker.com/film/Come-and-See/";

/// What a test waits for.
#[derive(Debug, Deserialize)]
struct Ratings {
    /// How many of the listed ratings have a minireview.
    reviews: usize,
    /// How many of the [`Ratings::reviews`] the page doesn't show in full.
    cut_off: usize,
    /// How many "More" links the page shows.
    more_links: usize,
    /// The page's address.
    href: String,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

impl Ratings {
    /// Whether the page shows every minireview in full, and has at least one.
    fn expanded(&self) -> bool {
        self.reviews > 0 && self.cut_off == 0 && self.more_links == 0
    }
}

/// A JavaScript expression for the [`Ratings`].
fn ratings_expression() -> String {
    // A rating with a minireview is a `.rating_card.review`, whose full text
    // is in `.ratingcard_minireview`. The compact view either hides that text
    // (in a wide window) or keeps it on one line that it clips (in a narrow
    // window), until "More" is clicked.
    format!(
        r#"(() => {{
            const cards = [...document.querySelectorAll(".rating_card.review")].filter((c) => c.checkVisibility());
            const cut_off = (card) => {{
                const text = card.querySelector(".ratingcard_minireview");
                return !text?.checkVisibility() || text.scrollWidth > text.clientWidth;
            }};
            return {{
                reviews: cards.length,
                cut_off: cards.filter(cut_off).length,
                more_links: [...document.querySelectorAll(".ratingcard_compact_more > a")].filter((a) => a.checkVisibility()).length,
                href: location.href,
                clicked: {CLICKED_TEXTS},
            }};
        }})()"#
    )
}

/// Wait until `done` has been true of the [`Ratings`] of the page in `tab`
/// for 3 seconds in a row.
///
/// `what` describes what `done` waits for, for the error message.
///
/// Returns those [`Ratings`], or an error if 30 seconds elapse first.
async fn expect(tab: &Tab, what: &str, done: impl Fn(&Ratings) -> bool) -> Result<Ratings> {
    wait_for(
        tab,
        what,
        &ratings_expression(),
        Duration::from_secs(30),
        Duration::from_secs(3),
        done,
    )
    .await
}

/// Open [`FILM`] as the user whom the cookies in the file named by the
/// environment variable `E2E_CRITICKER_COOKIES` log in, and run `work` with
/// the tab.
///
/// Returns what `work` returns, or an error if the variable isn't set or the
/// cookies don't log in.
async fn with_film(work: impl AsyncFnOnce(&Tab) -> Result<()>) -> Result<()> {
    let cookies = std::env::var_os("E2E_CRITICKER_COOKIES").context(
        "E2E_CRITICKER_COOKIES must be the path of a cookies.txt file of a logged-in Criticker session",
    )?;
    with_chrome(async |browser| {
        add_cookies(browser, Path::new(&cookies)).await?;
        let tab = open(browser, FILM).await?;
        let logged_in: bool = tab
            .evaluate_main(r#"document.querySelector('a[href$="/signout/"]') !== null"#)
            .await?;
        ensure!(
            logged_in,
            "the cookies in E2E_CRITICKER_COOKIES don't log in to Criticker"
        );
        work(&tab).await
    })
    .await
}

#[tokio::test]
#[ignore = "needs E2E_CRITICKER_COOKIES, the path of a cookies.txt file of a logged-in Criticker session"]
async fn minireviews_are_expanded_when_logged_in() -> Result<()> {
    with_film(async |tab| {
        expect(
            tab,
            "the userscript to click \"More\" on every minireview",
            |r| r.expanded() && r.clicked.iter().any(|text| text == "More"),
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
#[ignore = "needs E2E_CRITICKER_COOKIES, the path of a cookies.txt file of a logged-in Criticker session"]
async fn address_does_not_gain_a_fragment_when_logged_in() -> Result<()> {
    // Each "More" is a link to "#", which a click navigates to unless
    // something prevents that, as Criticker's script does once it has
    // started handling clicks.
    with_film(async |tab| {
        expect(
            tab,
            "the userscript to click \"More\" on every minireview, without the address changing",
            |r| r.expanded() && r.href == FILM && r.clicked.iter().any(|text| text == "More"),
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
#[ignore = "needs E2E_CRITICKER_COOKIES, the path of a cookies.txt file of a logged-in Criticker session"]
async fn minireviews_are_expanded_after_searching_when_logged_in() -> Result<()> {
    with_film(async |tab| {
        let before = expect(
            tab,
            "the userscript to click \"More\" on every minireview",
            |r| r.expanded() && r.clicked.iter().any(|text| text == "More"),
        )
        .await?;
        // Criticker replaces the list with one of the ratings whose
        // minireview or user name has "the" in it. (Choosing "Reviews"
        // instead of "All" would replace the list too, but Criticker
        // remembers that choice for the account.)
        tab.evaluate_main::<bool>(
            r##"(() => {
                document.querySelector("#ratings_search").value = "the";
                document.querySelector("#textbutton_ratings_search button").click();
                return true;
            })()"##,
        )
        .await?;
        expect(
            tab,
            "the page to list more minireviews than before the search, and the userscript to \
             click \"More\" on every one of them",
            |r| {
                r.expanded() && r.reviews > before.reviews && r.clicked.len() > before.clicked.len()
            },
        )
        .await?;
        Ok(())
    })
    .await
}
