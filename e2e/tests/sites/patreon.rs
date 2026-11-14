use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// The page of a creator who last posted in 2019. One of its public posts
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
/// Two of the comments have replies, which the page shows without a click,
/// above a "Collapse replies" button.
const POST_PAGE: &str = "https://www.patreon.com/posts/beautiful-to-2746322";

/// The page of a public post from 2015 that has 153 comments, by Patreon's
/// count. The page at first shows 14 of them, above a "Load more comments"
/// button, and each click that the page accepts loads about 10 more. Once
/// that button has loaded all that it can, the page shows 145 comments and
/// 3 buttons like "View 3 replies", which load 6 more comments in all.
const MANY_COMMENTS_PAGE: &str = "https://www.patreon.com/posts/new-statesman-is-2440676";

/// A JavaScript function that takes a box that Patreon can cut text off with
/// (a `div[data-is-measured]`), and returns whether the box shows less than
/// the whole text: whether the box has no height (as when it is hidden), or
/// is shorter than the text in it by more than 1 CSS pixel (a difference of
/// up to 1 pixel can come from the rounding of the two heights).
const IS_CUT_OFF: &str = "((box) => { const height = box.getBoundingClientRect().height; return height === 0 || height + 1 < box.firstElementChild.getBoundingClientRect().height; })";

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
        clicked: Vec<String>,
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
                    clicked: {CLICKED_TEXTS},
                }};
            }})()"#
        );
        // The userscript must make no click here (the page has nothing to
        // click), so `height_set` is what shows that the userscript did
        // this, and not a page that stopped cutting posts off.
        wait_for(
            &tab,
            "the userscript to show every long post whole, without a fade",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(3),
            |p: &Posts| {
                p.long > 0
                    && p.cut_off == 0
                    && p.faded == 0
                    && p.height_set == p.long
                    && p.clicked.is_empty()
            },
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
        // Each click adds 20 posts to the ones that the page showed at
        // first. So more than 20 posts per click means that every click has
        // added its posts: the userscript didn't click again at 100 posts.
        // Fewer than 120 posts means that it didn't click twice for a batch.
        wait_for(
            &tab,
            "the userscript to load posts until the page shows 100, and then stop",
            &expression,
            Duration::from_secs(60),
            Duration::from_secs(5),
            |p: &Posts| {
                (100..120).contains(&p.shown)
                    && p.shown > 20 * p.clicked.len()
                    && !p.clicked.is_empty()
                    && p.clicked.iter().all(|text| text == "See more posts")
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
        /// How many "Load more comments" buttons the page has.
        load_more: usize,
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
                    load_more: document.querySelectorAll('button[data-tag="loadMoreCommentsCta"]').length,
                    clicked: {CLICKED_TEXTS},
                }};
            }})()"#
        );
        wait_for(
            &tab,
            "the userscript to load every comment and expand every long one",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(3),
            |c: &Comments| {
                let expected = ["Show more", "Load more comments"];
                let show_more = c.clicked.iter().filter(|text| *text == "Show more").count();
                c.long > 0
                    && c.cut_off == 0
                    && c.load_more == 0
                    && show_more == c.long
                    && c.clicked.iter().all(|text| expected.contains(&text.as_str()))
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
        /// How many different comments those are.
        distinct: usize,
        clicked: Vec<String>,
    }
    with_chrome(async |browser| {
        let tab = open(browser, MANY_COMMENTS_PAGE).await?;
        let expression = format!(
            r#"(() => {{
                const ids = [...document.querySelectorAll('div[data-tag="comment-row"]')].map((row) => row.id);
                return {{
                    load_more: document.querySelectorAll('button[data-tag="loadMoreCommentsCta"]').length,
                    view_replies: [...document.querySelectorAll('div[data-tag="content-card-comment-thread-container"] button')].filter((b) => /^View .*repl/.test(b.innerText)).length,
                    shown: ids.length,
                    distinct: new Set(ids).size,
                    clicked: {CLICKED_TEXTS},
                }};
            }})()"#
        );
        // 145 comments are shown only after about 13 clicks on "Load more
        // comments".
        wait_for(
            &tab,
            "the userscript to load every comment and every reply",
            &expression,
            Duration::from_secs(90),
            Duration::from_secs(3),
            |c: &Comments| {
                let expected = ["Load more comments", "Show more"];
                c.load_more == 0
                    && c.view_replies == 0
                    && c.shown >= 145
                    && c.distinct == c.shown
                    && c.clicked.iter().any(|text| text == "Load more comments")
                    && c.clicked.iter().any(|text| text.starts_with("View "))
                    && c.clicked.iter().all(|text| {
                        expected.contains(&text.as_str()) || text.starts_with("View ")
                    })
            },
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn replies_that_the_user_collapses_stay_collapsed() -> Result<()> {
    #[derive(Debug, Deserialize)]
    struct Comments {
        /// How many "Load more comments" buttons the page has.
        load_more: usize,
        /// How many "Collapse replies" buttons the page has.
        collapse: usize,
        /// How many buttons like "View 3 replies" the page has.
        view_replies: usize,
        /// How many comments (including replies) the page shows.
        shown: usize,
        clicked: Vec<String>,
    }
    with_chrome(async |browser| {
        let tab = open(browser, POST_PAGE).await?;
        let expression = format!(
            r#"(() => {{
                const texts = [...document.querySelectorAll('div[data-tag="content-card-comment-thread-container"] button')].map((b) => b.innerText);
                return {{
                    load_more: document.querySelectorAll('button[data-tag="loadMoreCommentsCta"]').length,
                    collapse: texts.filter((text) => text === "Collapse replies").length,
                    view_replies: texts.filter((text) => /^View .*repl/.test(text)).length,
                    shown: document.querySelectorAll('div[data-tag="comment-row"]').length,
                    clicked: {CLICKED_TEXTS},
                }};
            }})()"#
        );
        let loaded = wait_for(
            &tab,
            "the userscript to load every comment, with the replies shown",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(3),
            |c: &Comments| c.load_more == 0 && c.collapse > 0 && c.view_replies == 0,
        )
        .await?;
        // The harness doesn't record this click, which the userscript
        // doesn't make.
        tab.evaluate_main::<bool>(
            r#"(() => {
                const buttons = [...document.querySelectorAll('div[data-tag="content-card-comment-thread-container"] button')];
                buttons.find((b) => b.innerText === "Collapse replies").click();
                return true;
            })()"#,
        )
        .await?;
        // The page replaces "Collapse replies" by a button like "View 3
        // replies", which the userscript must not click: it must make no
        // click after the ones that it had made.
        wait_for(
            &tab,
            "the replies that \"Collapse replies\" hid to stay hidden",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(5),
            |c: &Comments| {
                c.view_replies == 1
                    && c.collapse == loaded.collapse - 1
                    && c.shown < loaded.shown
                    && c.clicked == loaded.clicked
            },
        )
        .await?;
        Ok(())
    })
    .await
}
