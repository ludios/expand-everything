use anyhow::Result;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A page of a thread on a site that runs XenForo 2.3. The page has 5
/// quotes. XenForo cuts off the last one, which quotes a post by "Rohirm".
const V2_3_THREAD: &str = "https://www.hwinfo.com/forum/threads/prometheus-adapter-for-hwinfo-grafana-dashboard.6281/page-5";

/// A page of a thread on a site that runs a XenForo before 2.2. The page has
/// 25 quotes, 5 of which XenForo cuts off.
const OLDER_THREAD: &str =
    "https://forum.allaboutcircuits.com/threads/the-case-against-quantum-computing.154610/page-2";

/// What a test of quotes waits for.
#[derive(Debug, Deserialize)]
struct Quotes {
    /// How many quotes the page cuts off: it shows less than the whole
    /// height of their text.
    cut_off: usize,
    /// How many quotes the page cuts off while they lack the class
    /// "is-expanded" (which the expression takes away from a quote only
    /// while it measures the quote).
    cut_off_unless_expanded: usize,
    /// How many quotes show their "Click to expand..." link.
    shown_links: usize,
}

/// Check that the userscript expands every quote of the thread page at `url`,
/// at least one of which XenForo must cut off.
async fn check_quotes_are_expanded(url: &str) -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, url).await?;
        // The userscript expands a quote without a click, by giving it the
        // class "is-expanded". A quote that is cut off without that class
        // shows that the site still cuts quotes off, and therefore that the
        // userscript did the expanding. The userscript doesn't observe
        // changes of classes, so taking the class away for the measurement
        // doesn't make the userscript do anything.
        let expression = r##"(() => {
            const quotes = [...document.querySelectorAll(".bbCodeBlock--expandable")];
            const cut_off = (quote) => {
                const text = quote.querySelector(".bbCodeBlock-expandContent");
                return text !== null && text.scrollHeight > text.clientHeight + 1;
            };
            const cut_off_unless_expanded = (quote) => {
                const expanded = quote.classList.contains("is-expanded");
                quote.classList.remove("is-expanded");
                const result = cut_off(quote);
                quote.classList.toggle("is-expanded", expanded);
                return result;
            };
            return {
                cut_off: quotes.filter(cut_off).length,
                cut_off_unless_expanded: quotes.filter(cut_off_unless_expanded).length,
                shown_links: quotes.filter((quote) => quote.querySelector(".bbCodeBlock-expandLink")?.checkVisibility()).length,
            };
        })()"##;
        wait_for(
            &tab,
            "the userscript to expand every quote that XenForo cuts off",
            expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |q: &Quotes| q.cut_off_unless_expanded > 0 && q.cut_off == 0 && q.shown_links == 0,
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn quotes_are_expanded() -> Result<()> {
    check_quotes_are_expanded(V2_3_THREAD).await
}

#[tokio::test]
async fn quotes_are_expanded_on_xenforo_before_2_2() -> Result<()> {
    check_quotes_are_expanded(OLDER_THREAD).await
}
