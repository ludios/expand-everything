use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A publication's home page, which a new visitor sees under a subscribe
/// overlay with a "No thanks" button.
const HOME_PAGE: &str = "https://shimmeringvoid.substack.com/";

/// The comments page of a post with about 100 comments, of which the page at
/// first shows about 30, above a "Load More" button. The publication is on a
/// domain of its own, not on a subdomain of substack.com, so that a test runs
/// the userscript on such a domain.
const COMMENTS_PAGE: &str =
    "https://www.gurwinder.blog/p/tiktok-may-be-a-chinese-bio-weapon/comments";

/// The comments page of a post with more than 200 comments, which the page
/// shows about 30 at a time, above a "Load More" button. A few of the comments
/// are long, so the page cuts each of them off, above a "See more" button.
const LONG_COMMENTS_PAGE: &str = "https://mrgirl.substack.com/p/the-destiny-report/comments";

/// A note with a few long comments. The page cuts off each long comment
/// above a "See more" link.
const NOTE_PAGE: &str = "https://substack.com/@meaningness/note/c-59538921";

/// The end of one of the long comments on [`NOTE_PAGE`]. The page doesn't
/// have this text until that comment is expanded.
const NOTE_COMMENT_END: &str = "Your advice is completely inactionable as stated.";

#[tokio::test]
async fn subscribe_overlay_is_dismissed() -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct State {
        /// How many subscribe overlays the page has.
        overlays: usize,
        clicked: Vec<String>,
    }
    with_chrome(async |browser| {
        let tab = open(browser, HOME_PAGE).await?;
        let expression = format!(
            r#"({{ overlays: document.querySelectorAll("div.intro-popup").length, clicked: {CLICKED_TEXTS} }})"#
        );
        wait_for(
            &tab,
            "the userscript to click \"No thanks\" and the page to remove the overlay",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |s: &State| {
                s.overlays == 0 && s.clicked.iter().any(|text| text == "No thanks")
            },
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn comments_page_loads_all_comments() -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct State {
        /// How many "Load More" buttons the page has.
        load_more: usize,
        /// How many comments (including replies) the page shows.
        shown: usize,
        /// How many comments the post has, according to the page's data.
        total: usize,
        clicked: Vec<String>,
    }
    with_chrome(async |browser| {
        let tab = open(browser, COMMENTS_PAGE).await?;
        let expression = format!(
            r#"({{
                load_more: document.querySelectorAll("div.comment-list-items + button").length,
                shown: document.querySelectorAll("div.comment").length,
                total: window._preloads.post.comment_count,
                clicked: {CLICKED_TEXTS},
            }})"#
        );
        wait_for(
            &tab,
            "the userscript to click \"Load More\" until every comment is shown",
            &expression,
            Duration::from_secs(60),
            Duration::from_secs(3),
            |s: &State| {
                s.load_more == 0
                    && s.shown >= s.total
                    && s.clicked.iter().any(|text| text == "Load More")
            },
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn long_comments_are_expanded() -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct State {
        /// How many "Load More" buttons the page has.
        load_more: usize,
        /// How many comments the page cuts off.
        cut_off: usize,
        /// How many comments the page has expanded.
        expanded: usize,
        clicked: Vec<String>,
    }
    with_chrome(async |browser| {
        let tab = open(browser, LONG_COMMENTS_PAGE).await?;
        let expression = format!(
            r#"({{
                load_more: document.querySelectorAll("div.comment-list-items + button").length,
                cut_off: document.querySelectorAll("div.comment-body:not(.expanded)").length,
                expanded: document.querySelectorAll("div.comment-body.expanded").length,
                clicked: {CLICKED_TEXTS},
            }})"#
        );
        wait_for(
            &tab,
            "the userscript to load every comment and click \"See more\" below every long one",
            &expression,
            Duration::from_secs(60),
            Duration::from_secs(3),
            |s: &State| {
                s.load_more == 0
                    && s.cut_off == 0
                    && s.expanded > 0
                    && s.clicked.iter().any(|text| text == "See more")
            },
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn long_comments_on_a_note_are_expanded() -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct State {
        /// How many "See more" links the page has.
        cut_off: usize,
        /// Whether the page shows [`NOTE_COMMENT_END`].
        end_shown: bool,
        clicked: Vec<String>,
    }
    with_chrome(async |browser| {
        let tab = open(browser, NOTE_PAGE).await?;
        let expression = format!(
            r#"({{
                cut_off: document.querySelectorAll("div[class*=seeMoreText-]").length,
                end_shown: document.body.innerText.includes("{NOTE_COMMENT_END}"),
                clicked: {CLICKED_TEXTS},
            }})"#
        );
        wait_for(
            &tab,
            "the userscript to click \"See more\" below every long comment",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |s: &State| {
                s.cut_off == 0 && s.end_shown && s.clicked.iter().any(|text| text == "See more")
            },
        )
        .await?;
        Ok(())
    })
    .await
}
