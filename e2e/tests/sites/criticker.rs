use anyhow::{Result, ensure};
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{wait_for, with_login};
use serde::Deserialize;
use std::time::Duration;
use zendriver::Tab;

/// A film whose page lists its ratings, 100 at a time. If the account has
/// "Compact view" switched on, the list cuts off each minireview before a
/// "More" link. Without the cookies of a logged-in session, the tests'
/// Chrome gets a Cloudflare challenge instead of the page.
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
    /// Whether the list of ratings is still the one that [`mark_list_as_old`]
    /// marked.
    old_list: bool,
    /// The page's address.
    href: String,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

impl Ratings {
    /// Whether the page has at least one minireview and shows every
    /// minireview in full, and the userscript has clicked a "More".
    fn expanded(&self) -> bool {
        self.reviews > 0
            && self.cut_off == 0
            && self.more_links == 0
            && self.clicked.iter().any(|text| text == "More")
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
                old_list: document.querySelector(".ratings_list[data-e2e-old]") !== null,
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

/// Wait until the userscript has expanded every minireview of the page in
/// `tab` (see [`Ratings::expanded`]).
///
/// Returns the [`Ratings`], or an error if 30 seconds elapse first.
async fn expect_expanded(tab: &Tab) -> Result<Ratings> {
    expect(
        tab,
        "every minireview to be shown in full, after the userscript clicked \"More\"",
        Ratings::expanded,
    )
    .await
}

/// Mark the list of ratings of the page in `tab`, so that
/// [`Ratings::old_list`] is true until Criticker replaces the list.
async fn mark_list_as_old(tab: &Tab) -> Result<()> {
    tab.evaluate_main::<bool>(
        r#"(() => {
            document.querySelector(".ratings_list").dataset.e2eOld = "";
            return true;
        })()"#,
    )
    .await?;
    Ok(())
}

/// Add an element to the page in `tab`, `times` times, each in a task of its
/// own, so that a `MutationObserver` of the whole page is called back `times`
/// times.
async fn change_page(tab: &Tab, times: usize) -> Result<()> {
    tab.evaluate_main::<bool>(format!(
        r#"(async () => {{
            for (let i = 0; i < {times}; i++) {{
                document.body.append(document.createElement("span"));
                await new Promise((resolve) => setTimeout(resolve, 0));
            }}
            return true;
        }})()"#
    ))
    .await?;
    Ok(())
}

/// Open [`FILM`] as the user of the Criticker session that [`with_login`]
/// finds the cookies of, and run `work` with the tab.
///
/// Returns what `work` returns, or an error if the page doesn't open, the
/// cookies don't log in, or the account doesn't have "Compact view" switched
/// on.
async fn with_film(work: impl AsyncFnOnce(&Tab) -> Result<()>) -> Result<()> {
    // The link that signs the user out
    let logged_in = r#"document.querySelector('a[href$="/signout/"]')"#;
    with_login("criticker", FILM, logged_in, async |tab| {
        let compact_view: bool = tab
            .evaluate_main(r#"document.querySelector(".tip_ratings_toggle.on") !== null"#)
            .await?;
        ensure!(
            compact_view,
            "the account that the cookies log in doesn't have \"Compact view\" (above a film's \
             ratings) switched on"
        );
        work(tab).await
    })
    .await
}

#[tokio::test]
#[ignore = "needs a login (see `with_login`)"]
async fn minireviews_are_expanded_when_logged_in() -> Result<()> {
    with_film(async |tab| {
        expect_expanded(tab).await?;
        Ok(())
    })
    .await
}

#[tokio::test]
#[ignore = "needs a login (see `with_login`)"]
async fn address_does_not_gain_a_fragment_when_logged_in() -> Result<()> {
    // Each "More" is a link to "#", which a click navigates to unless
    // something prevents that, as Criticker's script does once it has
    // started handling clicks.
    with_film(async |tab| {
        expect(
            tab,
            "every minireview to be shown in full, after the userscript clicked \"More\", without \
             the address changing",
            |r| r.expanded() && r.href == FILM,
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
#[ignore = "needs a login (see `with_login`)"]
async fn minireviews_are_expanded_after_searching_when_logged_in() -> Result<()> {
    with_film(async |tab| {
        let before = expect_expanded(tab).await?;
        // More changes than a userscript that stops observing after 20
        // changes would see. (In a browser that isn't headless, the page
        // changes more by itself.)
        change_page(tab, 25).await?;
        mark_list_as_old(tab).await?;
        // Criticker replaces the list with a list of only the ratings whose
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
            "Criticker to replace the list of ratings, and every minireview of the new list to be \
             shown in full, after the userscript clicked \"More\" again",
            |r| !r.old_list && r.expanded() && r.clicked.len() > before.clicked.len(),
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
#[ignore = "needs a login (see `with_login`)"]
async fn minireview_cut_off_by_the_user_stays_cut_off_when_logged_in() -> Result<()> {
    with_film(async |tab| {
        expect_expanded(tab).await?;
        // "Show less" is under each expanded minireview (in a wide window,
        // as headless Chrome's is).
        let clicked: bool = tab
            .evaluate_main(
                r#"(() => {
                    const less = [...document.querySelectorAll(".rating_card.review.expanded .ratingcard_compact_less > a")].find((a) => a.checkVisibility());
                    less?.click();
                    return less !== undefined;
                })()"#,
            )
            .await?;
        ensure!(clicked, "the page shows no \"Show less\" to click");
        // The userscript looks at the page again when the page changes.
        change_page(tab, 1).await?;
        expect(
            tab,
            "the one minireview that \"Show less\" was clicked on to stay cut off",
            |r| r.cut_off == 1 && r.more_links == 1,
        )
        .await?;
        Ok(())
    })
    .await
}
