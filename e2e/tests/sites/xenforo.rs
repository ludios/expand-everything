use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A page of a thread on a site that runs XenForo 2.3. The last of its 5
/// quotes (of a post by "Rohirm") is taller than [`COLLAPSED_HEIGHT`].
const V2_3_THREAD: &str = "https://www.hwinfo.com/forum/threads/prometheus-adapter-for-hwinfo-grafana-dashboard.6281/page-5";

/// A page of a thread on a site that runs a XenForo before 2.2, in which the
/// "Click to expand..." link of a quote lacks the `js-expandLink` class and
/// the `role` attribute that XenForo 2.3 gives it. 5 of the page's 25 quotes
/// are taller than [`COLLAPSED_HEIGHT`].
const OLDER_THREAD: &str =
    "https://forum.allaboutcircuits.com/threads/the-case-against-quantum-computing.154610/page-2";

/// The height, in pixels, above which XenForo's default style cuts off the
/// text of a quote that isn't expanded.
const COLLAPSED_HEIGHT: u32 = 150;

/// What a test of quotes waits for.
#[derive(Debug, Deserialize)]
struct Quotes {
    /// The height, in pixels, that the page shows of the text of the quote
    /// that it shows the most of.
    tallest: u32,
    /// How many quotes the page cuts off: it shows less than the whole height
    /// of their text.
    cut_off: usize,
    /// How many "Click to expand..." links the page shows.
    shown_links: usize,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

/// Check that the userscript expands every quote of the thread page at `url`,
/// which must have a quote that is taller than [`COLLAPSED_HEIGHT`].
async fn check_quotes_are_expanded(url: &str) -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, url).await?;
        let expression = format!(
            r##"(() => {{
                const texts = [...document.querySelectorAll(".bbCodeBlock--expandable .bbCodeBlock-expandContent")];
                const links = [...document.querySelectorAll(".bbCodeBlock-expandLink")];
                return {{
                    tallest: Math.max(0, ...texts.map((t) => t.clientHeight)),
                    cut_off: texts.filter((t) => t.scrollHeight > t.clientHeight + 1).length,
                    shown_links: links.filter((l) => l.checkVisibility()).length,
                    clicked: {CLICKED_TEXTS},
                }};
            }})()"##
        );
        // A quote taller than `COLLAPSED_HEIGHT` shows that XenForo still
        // cuts quotes off, and that the userscript expanded one.
        wait_for(
            &tab,
            "the userscript to expand every quote",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |q: &Quotes| {
                q.tallest > COLLAPSED_HEIGHT
                    && q.cut_off == 0
                    && q.shown_links == 0
                    && q.clicked.iter().any(|text| text == "Click to expand...")
            },
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
