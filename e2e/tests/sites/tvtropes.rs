use anyhow::Result;
use expand_everything_e2e::userscript::CLICKED_TEXTS;
use expand_everything_e2e::{open, wait_for, with_chrome};
use serde::Deserialize;
use std::time::Duration;

/// A page that has two folders below an "open/close all folders" label.
const FOLDERS_PAGE: &str =
    "https://tvtropes.org/pmwiki/pmwiki.php/OvershadowedByControversy/WebOriginal";

/// What a test waits for.
#[derive(Debug, Deserialize)]
struct Page {
    /// How many folders the page has.
    folders: usize,
    /// How many of the [`Page::folders`] the page shows the contents of.
    open_folders: usize,
    /// The texts of the userscript's clicks (see [`CLICKED_TEXTS`]).
    clicked: Vec<String>,
}

#[tokio::test]
async fn folders_are_opened() -> Result<()> {
    with_chrome(async |browser| {
        let tab = open(browser, FOLDERS_PAGE).await?;
        // The contents of a folder are in a `.folder`, which the page hides
        // until the folder is opened.
        let expression = format!(
            r##"(() => {{
                const folders = [...document.querySelectorAll(".folder")];
                return {{
                    folders: folders.length,
                    open_folders: folders.filter((f) => f.checkVisibility()).length,
                    clicked: {CLICKED_TEXTS},
                }};
            }})()"##
        );
        wait_for(
            &tab,
            "the userscript to open every folder by clicking \"open/close all folders\"",
            &expression,
            Duration::from_secs(30),
            Duration::from_secs(2),
            |p: &Page| {
                p.folders > 0
                    && p.open_folders == p.folders
                    && p.clicked == ["open/close all folders"]
            },
        )
        .await?;
        Ok(())
    })
    .await
}
