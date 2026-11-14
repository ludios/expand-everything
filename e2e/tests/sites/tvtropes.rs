use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A page that has two folders below one "open/close all folders" label, and
/// 15 notes, all of them in the folders.
const ONE_TOGGLE_PAGE: &str =
    "https://tvtropes.org/pmwiki/pmwiki.php/OvershadowedByControversy/WebOriginal";

/// A page that has 16 folders and two "open/close all folders" labels, each
/// of which opens or closes all 16 folders.
const TWO_TOGGLES_PAGE: &str = "https://tvtropes.org/pmwiki/pmwiki.php/Franchise/Godzilla";

/// A page that has two folders and no "open/close all folders" label.
const NO_TOGGLE_PAGE: &str = "https://tvtropes.org/pmwiki/pmwiki.php/Creator/AgathaChristie";

/// What a test of folders waits for.
#[derive(Debug, Deserialize)]
struct Folders {
    /// How many folders the article has.
    folders: usize,
    /// How many folders the article shows the contents of.
    open_folders: usize,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

/// Check that the userscript opens every folder of the article at `url`.
async fn check_folders_are_opened(url: &str) -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, url).await?;
        // The contents of a folder are in a `.folder`, which the page hides
        // until the folder is opened.
        let expression = format!(
            r##"(() => {{
                const folders = [...document.querySelectorAll("#main-article .folder")];
                return {{
                    folders: folders.length,
                    open_folders: folders.filter((f) => f.checkVisibility()).length,
                    clicked: {CLICKED_TEXTS},
                }};
            }})()"##
        );
        // A click on a label that is already open closes folders, so every
        // folder stays open only if the userscript makes no such click.
        wait_for(
            &tab,
            "the userscript to open every folder",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |p: &Folders| p.folders > 0 && p.open_folders == p.folders && !p.clicked.is_empty(),
        )
        .await?;
        Ok(())
    })
    .await
}

#[tokio::test]
async fn folders_are_opened() -> Result<()> {
    check_folders_are_opened(ONE_TOGGLE_PAGE).await
}

#[tokio::test]
async fn folders_stay_open_on_a_page_with_two_toggles() -> Result<()> {
    check_folders_are_opened(TWO_TOGGLES_PAGE).await
}

#[tokio::test]
async fn folders_are_opened_on_a_page_without_a_toggle() -> Result<()> {
    check_folders_are_opened(NO_TOGGLE_PAGE).await
}

/// What the test of notes waits for.
#[derive(Debug, Deserialize)]
struct Notes {
    /// How many notes the article has.
    notes: usize,
    /// How many notes the article shows the text of.
    shown_notes: usize,
    /// Whether TV Tropes itself shows the text of every note, as it does for
    /// a visitor who has turned that preference on.
    shown_by_site: bool,
}

#[tokio::test]
async fn notes_are_shown() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, ONE_TOGGLE_PAGE).await?;
        // The text of a note is in an `.inlinefolder`, which the page hides
        // until the label before it (usually "note") is clicked. The
        // userscript shows the text without a click, so the test checks
        // instead that TV Tropes didn't show it: the page's script does that
        // only if `<html>` has the class "notes-open".
        let expression = r##"(() => {
            const notes = [...document.querySelectorAll("#main-article .inlinefolder[isnote]")];
            return {
                notes: notes.length,
                shown_notes: notes.filter((n) => n.checkVisibility()).length,
                shown_by_site: document.documentElement.classList.contains("notes-open"),
            };
        })()"##;
        wait_for(
            &tab,
            "the userscript to show the text of every note",
            expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |p: &Notes| p.notes > 0 && p.shown_notes == p.notes && !p.shown_by_site,
        )
        .await?;
        Ok(())
    })
    .await
}
