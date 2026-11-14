use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A thread on a site that runs Mastodon 5.0. Each of its [`V5_POSTS`] posts
/// has a content warning, and its first post has an image that is marked as
/// sensitive.
const V5_THREAD: &str = "https://mastodon.social/@spacerog/112974029330927616";

/// How many posts [`V5_THREAD`] has.
const V5_POSTS: usize = 14;

/// The page that shows only the first post of [`V5_THREAD`], for other sites
/// to embed.
const V5_EMBED: &str = "https://mastodon.social/@spacerog/112974029330927616/embed";

/// A thread on a site that runs Mastodon 4.7. Each of its [`V4_POSTS`] posts
/// has a content warning, and [`V4_MEDIA`] of them have an animation that is
/// marked as sensitive.
const V4_THREAD: &str = "https://mathstodon.xyz/@j_bertolotti/110436005325555159";

/// How many posts [`V4_THREAD`] has.
const V4_POSTS: usize = 7;

/// How many posts of [`V4_THREAD`] have an animation.
const V4_MEDIA: usize = 5;

/// A reply on a site that runs glitch-soc (a fork of Mastodon), which the page
/// shows below the post that it replies to. Both posts have a content
/// warning. Unlike Mastodon, glitch-soc shows whom a collapsed post mentions
/// (here, the reply mentions the author of the other post).
const GLITCH_REPLY: &str = "https://infosec.exchange/@plasticbiker/112974302351350325";

/// A JavaScript expression for a [`Page`] (without `clicked`) of Mastodon 5.0.
///
/// A content warning names, in `aria-controls`, the element that holds the
/// text of its post. While the post is collapsed, that element is `inert`, and
/// its `::after` covers it. The parent of a media gallery is covered in the
/// same way until the media is shown.
const V5_STATE: &str = r#"(() => {
    const uncovered = (e) => !e.inert
        && e.checkVisibility()
        && getComputedStyle(e, "::after").visibility === "hidden";
    const warnings = [...document.querySelectorAll("article div[aria-controls]")];
    const media = [...document.querySelectorAll("article .media-gallery")];
    const buttons = (text) => [...document.querySelectorAll("button")]
        .filter((b) => b.checkVisibility() && text.test(b.innerText.trim())).length;
    return {
        warnings: warnings.length,
        shown_posts: warnings.filter((w) => {
            const post = document.getElementById(w.getAttribute("aria-controls"));
            return post !== null && uncovered(post);
        }).length,
        media: media.length,
        shown_media: media.filter((m) => uncovered(m.parentElement)
            && m.querySelector("img, video")?.checkVisibility()).length,
        show_post_buttons: buttons(/^Show post$/),
        show_media_buttons: buttons(/^Show media$|Click to show$/),
    };
})()"#;

/// A JavaScript expression for a [`Page`] (without `clicked`) of Mastodon
/// before 5.0, or of glitch-soc.
///
/// A collapsed post has neither its text nor its media. A media gallery has a
/// `.spoiler-button` instead of its media until the media is shown.
const V4_STATE: &str = r#"(() => {
    const warnings = [...document.querySelectorAll(".content-warning")];
    const media = [...document.querySelectorAll(".media-gallery")];
    const buttons = (text) => [...document.querySelectorAll("button")]
        .filter((b) => b.checkVisibility() && text.test(b.innerText.trim())).length;
    return {
        warnings: warnings.length,
        shown_posts: warnings.filter((w) => w.parentElement
            .querySelector(":scope > .status__content > .status__content__text")
            ?.checkVisibility()).length,
        media: media.length,
        shown_media: media.filter((m) => m.querySelector(".spoiler-button") === null
            && m.querySelector("img, video")?.checkVisibility()).length,
        show_post_buttons: buttons(/^Show more$/),
        show_media_buttons: buttons(/Click to show$/),
    };
})()"#;

/// What a test waits for.
#[derive(Debug, Deserialize)]
struct Page {
    /// How many posts have a content warning.
    warnings: usize,
    /// How many of those posts show their text.
    shown_posts: usize,
    /// How many media galleries the posts have.
    media: usize,
    /// How many of those galleries show their media.
    shown_media: usize,
    /// How many buttons that would show a post are displayed.
    show_post_buttons: usize,
    /// How many buttons that would show media are displayed.
    show_media_buttons: usize,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

impl Page {
    /// How many of the userscript's clicks were on an element whose text
    /// ends with `text`.
    fn clicks(&self, text: &str) -> usize {
        self.clicked.iter().filter(|t| t.ends_with(text)).count()
    }
}

/// Load `url` and wait until `done` holds for its [`Page`].
///
/// `state` is [`V5_STATE`] or [`V4_STATE`], whichever reads the web interface
/// of the site of `url`. `what` describes what is awaited, for the error
/// message.
async fn check(url: &str, state: &str, what: &str, done: impl Fn(&Page) -> bool) -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, url).await?;
        wait_for(
            &tab,
            what,
            &format!("({{ ...{state}, clicked: {CLICKED_TEXTS} }})"),
            Duration::from_secs(30),
            Duration::from_secs(2),
            done,
        )
        .await?;
        Ok(())
    })
    .await
}

/// Whether `p` shows the text of each of its `posts` posts, after exactly one
/// click by the userscript per post, on a button whose text ends with
/// `button`. (A second click on a button would collapse its post again.)
fn posts_are_expanded(p: &Page, posts: usize, button: &str) -> bool {
    p.warnings == posts
        && p.shown_posts == posts
        && p.show_post_buttons == 0
        && p.clicks(button) == posts
}

/// Whether `p` shows the media of each of its `galleries` media galleries,
/// after exactly one click by the userscript per gallery, on a button whose
/// text ends with `button`.
fn media_is_shown(p: &Page, galleries: usize, button: &str) -> bool {
    p.media == galleries
        && p.shown_media == galleries
        && p.show_media_buttons == 0
        && p.clicks(button) == galleries
}

#[tokio::test]
async fn posts_are_expanded_on_mastodon_5() -> Result<()> {
    check(
        V5_THREAD,
        V5_STATE,
        "the userscript to expand every post by clicking each \"Show post\" once",
        |p| posts_are_expanded(p, V5_POSTS, "Show post"),
    )
    .await
}

#[tokio::test]
async fn embedded_post_is_expanded_on_mastodon_5() -> Result<()> {
    check(
        V5_EMBED,
        V5_STATE,
        "the userscript to expand the post by clicking \"Show post\" once",
        |p| posts_are_expanded(p, 1, "Show post"),
    )
    .await
}

#[tokio::test]
async fn sensitive_media_is_shown_on_mastodon_5() -> Result<()> {
    check(
        V5_THREAD,
        V5_STATE,
        "the userscript to show the image by clicking \"Show media\" once",
        |p| media_is_shown(p, 1, "Show media"),
    )
    .await
}

#[tokio::test]
async fn posts_are_expanded_on_mastodon_4() -> Result<()> {
    check(
        V4_THREAD,
        V4_STATE,
        "the userscript to expand every post by clicking each \"Show more\" once",
        |p| posts_are_expanded(p, V4_POSTS, "Show more"),
    )
    .await
}

#[tokio::test]
async fn sensitive_media_is_shown_on_mastodon_4() -> Result<()> {
    check(
        V4_THREAD,
        V4_STATE,
        "the userscript to show every animation by clicking each \"Click to show\" once",
        |p| media_is_shown(p, V4_MEDIA, "Click to show"),
    )
    .await
}

#[tokio::test]
async fn posts_are_expanded_on_glitch_soc() -> Result<()> {
    check(
        GLITCH_REPLY,
        V4_STATE,
        "the userscript to expand both posts by clicking each \"Show more\" once",
        |p| posts_are_expanded(p, 2, "Show more"),
    )
    .await
}
