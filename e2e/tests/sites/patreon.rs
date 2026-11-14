use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// The page of a creator who last posted in 2018. One of its public posts
/// ("New reward tier: Evernotes firehose") is long, so the page cuts it off
/// under a fade, above a "View post" link that leads to the post's own page.
const CREATOR_PAGE: &str = "https://www.patreon.com/gwern";

/// The page of a creator with thousands of posts, of which the page at
/// first shows 20, above a "See more posts" button that adds 20 more per
/// click.
const BUSY_CREATOR_PAGE: &str = "https://www.patreon.com/amandapalmer";

/// The page of a public post from 2015 with 19 comments, of which the page
/// at first shows 12. Three of the comments are long (two of them among the
/// first 12), so the page cuts each of them off, above a "Show more" button.
const POST_PAGE: &str = "https://www.patreon.com/posts/beautiful-to-2746322";

/// The page of a public post from 2015 with about 150 comments, of which the
/// page shows about 10 more for each click on "Load more comments". A few of
/// them are replies that the page shows only after a click on a button like
/// "View 3 replies" below the comment that they reply to.
const MANY_COMMENTS_PAGE: &str = "https://www.patreon.com/posts/new-statesman-is-2440676";

/// A JavaScript function that takes a box that Patreon cuts text off with (a
/// `div[data-is-measured]`), and returns whether the box is shorter than the
/// text in it, by more than what rounding explains.
const IS_CUT_OFF: &str = "((box) => box.getBoundingClientRect().height + 1 < box.firstElementChild.getBoundingClientRect().height)";

#[tokio::test]
async fn long_public_post_on_creator_page_is_shown_whole() -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct Posts {
        /// How many posts the page has in a box that can cut the post off.
        long: usize,
        /// How many of those posts the page cuts off.
        cut_off: usize,
        /// How many of those posts the page shows a fade over.
        faded: usize,
        /// How many of those posts have the height that the userscript sets.
        height_set: usize,
    }
    with_chrome(async |browser| {
        let tab = open(browser, CREATOR_PAGE).await?;
        // In the box, the post's text is the first child, and the fade is
        // the other one.
        let expression = format!(
            r#"(() => {{
                const boxes = [...document.querySelectorAll('div[data-tag="post-content"]')];
                const visible = (el) => el.checkVisibility({{ opacityProperty: true }});
                return {{
                    long: boxes.length,
                    cut_off: boxes.filter({IS_CUT_OFF}).length,
                    faded: boxes.filter((box) => [...box.children].slice(1).some(visible)).length,
                    height_set: boxes.filter((box) => box.style.height === "auto").length,
                }};
            }})()"#
        );
        // The userscript makes no click here, so `height_set` is what shows
        // that the userscript did this, and not a page that stopped cutting
        // posts off.
        wait_for(
            &tab,
            "the userscript to show every long post whole, without a fade",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(3),
            |p: &Posts| p.long > 0 && p.cut_off == 0 && p.faded == 0 && p.height_set == p.long,
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn posts_are_loaded_until_there_are_100() -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct Posts {
        /// How many posts the page shows.
        shown: usize,
        clicked: Vec<String>,
    }
    with_chrome(async |browser| {
        let tab = open(browser, BUSY_CREATOR_PAGE).await?;
        let expression = format!(
            r#"({{
                shown: document.querySelectorAll('div[data-tag="post-card"]').length,
                clicked: {CLICKED_TEXTS},
            }})"#
        );
        // A userscript that didn't stop at 100 posts would load the next 20
        // within the 5 seconds.
        wait_for(
            &tab,
            "the userscript to click \"See more posts\" until the page shows 100 posts, and no more",
            &expression,
            Duration::from_secs(60),
            Duration::from_secs(5),
            |p: &Posts| {
                (100..120).contains(&p.shown) && p.clicked.iter().any(|text| text == "See more posts")
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
    struct Comments {
        /// How many comments the page has in a box that can cut the comment
        /// off.
        long: usize,
        /// How many of those comments the page cuts off.
        cut_off: usize,
        clicked: Vec<String>,
    }
    with_chrome(async |browser| {
        let tab = open(browser, POST_PAGE).await?;
        let expression = format!(
            r#"(() => {{
                const boxes = [...document.querySelectorAll('div[data-tag="comment-body-collapsible"]')];
                return {{
                    long: boxes.length,
                    cut_off: boxes.filter({IS_CUT_OFF}).length,
                    clicked: {CLICKED_TEXTS},
                }};
            }})()"#
        );
        wait_for(
            &tab,
            "the userscript to click \"Show more\" below every long comment",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(3),
            |c: &Comments| {
                c.long > 0 && c.cut_off == 0 && c.clicked.iter().any(|text| text == "Show more")
            },
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn all_comments_are_loaded() -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct Comments {
        /// How many "Load more comments" buttons the page has.
        load_more: usize,
        /// How many buttons like "View 3 replies" the page has.
        view_replies: usize,
        /// How many comments (including replies) the page shows.
        shown: usize,
        clicked: Vec<String>,
    }
    with_chrome(async |browser| {
        let tab = open(browser, MANY_COMMENTS_PAGE).await?;
        let expression = format!(
            r#"({{
                load_more: document.querySelectorAll('button[data-tag="loadMoreCommentsCta"]').length,
                view_replies: [...document.querySelectorAll('div[data-tag="content-card-comment-thread-container"] button')].filter((b) => /^View .*repl/.test(b.innerText)).length,
                shown: document.querySelectorAll('div[data-tag="comment-row"]').length,
                clicked: {CLICKED_TEXTS},
            }})"#
        );
        // More than 100 comments are shown only after many clicks.
        wait_for(
            &tab,
            "the userscript to load every comment and every reply",
            &expression,
            Duration::from_secs(90),
            Duration::from_secs(3),
            |c: &Comments| {
                c.load_more == 0
                    && c.view_replies == 0
                    && c.shown > 100
                    && c.clicked.iter().any(|text| text == "Load more comments")
                    && c.clicked.iter().any(|text| text.ends_with(" replies"))
            },
        )
        .await?;
        Ok(())
    })
    .await
}
