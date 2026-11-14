//! Reading `expand.user.js` and turning it into the scripts that the harness
//! injects into each new document, imitating what Violentmonkey does with it.
//!
//! Violentmonkey runs a script at `document-end` (unless `@run-at` says
//! otherwise), in each document whose URL a `@match` line allows. The injected
//! scripts do the same (unless a test asks for [`Start::AfterLoad`]), and
//! [`Userscript::parse`] refuses a metadata key or a `@match` pattern that
//! they don't imitate.
//!
//! The injected scripts always run the userscript in the page's main world.
//! Violentmonkey does that by default too, but if a page's Content Security
//! Policy blocks it, Violentmonkey falls back to its content script's isolated
//! world, where page globals (such as `window.StackExchange`) are missing. The
//! tests don't cover that fallback.

use anyhow::{Context, Result, bail, ensure};
use std::path::PathBuf;

/// The name of the global (in the page's main world) through which the
/// harness script reports to the tests. See [`Userscript::harness_script`].
macro_rules! harness_global {
    () => {
        "__expand_everything_e2e"
    };
}

/// The name of the global (in the page's main world) through which the
/// harness script reports to the tests. See [`Userscript::harness_script`].
pub const HARNESS_GLOBAL: &str = harness_global!();

/// A JavaScript expression for the texts of the elements that the userscript
/// has clicked, as they were just before each click (see
/// [`Userscript::harness_script`]), in the order of the clicks.
pub const CLICKED_TEXTS: &str = concat!("window.", harness_global!(), ".clicks.map((c) => c.text)");

/// The `sourceURL` of the injected userscript. Stack traces name the
/// userscript's frames with it, which is how the harness tells a `click()`
/// call made directly by the userscript from one made by the page (even one
/// made by a page's click handler while the userscript's click is being
/// dispatched).
const USERSCRIPT_SOURCE_URL: &str = "expand.user.js";

/// The JavaScript source of `install_harness`, which the harness script calls
/// (see [`Userscript::harness_script`]).
const HARNESS_JS: &str = include_str!("harness.js");

/// A JavaScript expression for whether the userscript is watching the page
/// for elements to click (see `observing` in [`Userscript::harness_script`]).
pub const OBSERVING: &str = concat!("window.", harness_global!(), ".observing");

/// When the injected scripts start the userscript in a document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Start {
    /// Once the DOM has been parsed (Violentmonkey's default `@run-at
    /// document-end`).
    DocumentEnd,
    /// After the document's `load` event, as when a userscript manager gets
    /// to a document only after the document has loaded.
    AfterLoad,
}

/// The metadata keys whose meaning the injected scripts imitate.
const SUPPORTED_KEYS: &[&str] = &[
    "name",
    "description",
    "namespace",
    "author",
    "license",
    "version",
    "grant",
    "match",
];

/// A userscript, with the regexes (in a syntax that both JavaScript's
/// `RegExp` and Rust's `regex` accept) that its `@match` lines become.
#[derive(Debug)]
pub struct Userscript {
    source: String,
    match_regexes: Vec<String>,
}

/// The `@key value` lines of the metadata block in `source`, in order. As in
/// Violentmonkey, a line is one only if it starts with `// @` (after any
/// whitespace), and every other line is ignored.
///
/// Returns an error if `source` has no complete `==UserScript==` block.
fn metadata(source: &str) -> Result<Vec<(&str, &str)>> {
    let mut lines = source.lines().map(str::trim);
    ensure!(
        lines.any(|line| line == "// ==UserScript=="),
        "no `// ==UserScript==` line"
    );
    let mut entries = Vec::new();
    for line in lines {
        if line == "// ==/UserScript==" {
            return Ok(entries);
        }
        let Some(entry) = line.strip_prefix("// @") else {
            continue;
        };
        let (key, value) = entry.split_once(char::is_whitespace).unwrap_or((entry, ""));
        entries.push((key, value.trim()));
    }
    bail!("no `// ==/UserScript==` line")
}

/// `text`, with every character that is special in a regex escaped, for both
/// JavaScript's `RegExp` and Rust's `regex`.
fn escape_regex(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for c in text.chars() {
        if "\\^$.|?*+()[]{}".contains(c) {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    escaped
}

/// Convert a `@match` pattern to a regex that matches the URLs (without
/// their `#fragment`) that the pattern allows, following Chrome's
/// [match patterns](https://developer.chrome.com/docs/extensions/develop/concepts/match-patterns),
/// which Violentmonkey's `@match` follows:
///
/// - The scheme is `http`, `https`, or `*` (either of those two).
/// - The host is `*` (any host), `*.` followed by a domain (that domain or
///   any of its subdomains), or a host to match exactly, in lowercase. A URL
///   with any port matches.
/// - The path (which includes the query string) is a glob in which `*`
///   matches any characters, and which ends with `*`.
///
/// Returns an error for anything else, such as `<all_urls>`, a pattern with a
/// port or a `#`, or a path that doesn't end with `*` (where Violentmonkey and
/// Chrome disagree about the query string), which the tests don't need yet.
pub fn match_pattern_to_regex(pattern: &str) -> Result<String> {
    let Some((scheme, rest)) = pattern.split_once("://") else {
        bail!("@match {pattern:?} has no `://`");
    };
    let scheme = match scheme {
        "*" => "https?",
        "http" | "https" => scheme,
        _ => bail!("@match {pattern:?} has an unsupported scheme"),
    };
    ensure!(
        !rest.contains('#'),
        "@match {pattern:?} has a `#`, which isn't supported"
    );
    let Some(path_start) = rest.find('/') else {
        bail!("@match {pattern:?} has no path");
    };
    let (host, path) = rest.split_at(path_start);
    ensure!(
        !host.contains(':'),
        "@match {pattern:?} has a port, which isn't supported"
    );
    ensure!(
        host == host.to_ascii_lowercase(),
        "@match {pattern:?} has capital letters in its host, which isn't supported"
    );
    ensure!(
        path.ends_with('*'),
        "@match {pattern:?} has a path that doesn't end with `*`, which isn't supported"
    );
    let domain = host.strip_prefix("*.").unwrap_or(host);
    ensure!(
        host == "*" || (!domain.is_empty() && !domain.contains('*')),
        "@match {pattern:?} has an empty host or a `*` in the middle of its host"
    );
    let host = if host == "*" {
        "[^/:]+".to_string()
    } else if host.starts_with("*.") {
        format!("(?:[^/:]+\\.)?{}", escape_regex(domain))
    } else {
        escape_regex(host)
    };
    let path = path
        .split('*')
        .map(escape_regex)
        .collect::<Vec<_>>()
        .join(".*");
    Ok(format!("^{scheme}://{host}(?::[0-9]+)?{path}$"))
}

impl Userscript {
    /// Parse the userscript `source`, and check that the injected scripts can
    /// imitate Violentmonkey for each of its metadata keys.
    ///
    /// Returns an error if the metadata block is missing or malformed, has a
    /// key that isn't in `SUPPORTED_KEYS`, grants anything but `none`, or has
    /// no `@match` lines.
    pub fn parse(source: String) -> Result<Self> {
        let mut grants = Vec::new();
        let mut match_regexes = Vec::new();
        for (key, value) in metadata(&source)? {
            ensure!(
                SUPPORTED_KEYS.contains(&key),
                "the harness doesn't imitate Violentmonkey's handling of @{key}; teach userscript.rs about it"
            );
            match key {
                "grant" => grants.push(value),
                "match" => match_regexes.push(match_pattern_to_regex(value)?),
                _ => {}
            }
        }
        ensure!(
            grants == ["none"],
            "the harness runs the userscript in the page's main world, as Violentmonkey does only for `@grant none`, but the grants are {grants:?}"
        );
        ensure!(!match_regexes.is_empty(), "no @match lines");
        Ok(Self {
            source,
            match_regexes,
        })
    }

    /// Read and parse the userscript file named by the environment variable
    /// `E2E_USERSCRIPT` (to test another copy, such as an older release), or
    /// else `expand.user.js` in the directory above this crate.
    pub fn load() -> Result<Self> {
        let path = match std::env::var_os("E2E_USERSCRIPT") {
            Some(path) => PathBuf::from(path),
            None => PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../expand.user.js"),
        };
        let source = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        Self::parse(source).with_context(|| format!("parsing {}", path.display()))
    }

    /// Whether a `@match` line allows `url` (which must not have a fragment).
    #[cfg(test)]
    fn matches(&self, url: &str) -> bool {
        self.match_regexes
            .iter()
            .any(|re| regex::Regex::new(re).unwrap().is_match(url))
    }

    /// The script to inject before [`Userscript::wrapped_script`]. In each
    /// document, it creates `window[HARNESS_GLOBAL]`, an object with:
    ///
    /// - `injections`: how many times this script has run in the document,
    ///   which [`crate::open`] requires to be 1.
    /// - `matched`: whether a `@match` line allows the document's URL, which
    ///   decides whether the userscript runs.
    /// - `ran`: whether the userscript has started running.
    /// - `error`: the exception (as text) that the userscript's top-level code
    ///   threw, if it threw one.
    /// - `clicks`: one `{element, text, time}` for each `click()` call that the
    ///   userscript made directly, where `text` is the element's `innerText`
    ///   just before the click (or, if that is empty, as for an icon button,
    ///   the element's `aria-label` or the text of the elements that its
    ///   `aria-labelledby` names), trimmed and cut to 200 UTF-16 code units
    ///   (where the cut splits a character, its first half becomes U+FFFD,
    ///   which CDP's JSON can carry), and `time` is `performance.now()`.
    /// - `observing`: whether the userscript's last direct call of a
    ///   `MutationObserver`'s `observe` or `disconnect` was to `observe`.
    /// - `start`: a function that runs its argument (the userscript, as a
    ///   function) if `matched` is true, at the time that `when` (the
    ///   argument of this Rust method) names. For [`Start::DocumentEnd`],
    ///   like Violentmonkey, it waits for `DOMContentLoaded` on `window`, so
    ///   that the page's listeners on `document` run first. For
    ///   [`Start::AfterLoad`], it waits until every listener of the `load`
    ///   event has run.
    pub fn harness_script(&self, when: Start) -> String {
        let global = serde_json::to_string(HARNESS_GLOBAL).expect("strings serialize");
        let regexes = serde_json::to_string(&self.match_regexes).expect("strings serialize");
        let source_url = serde_json::to_string(USERSCRIPT_SOURCE_URL).expect("strings serialize");
        let after_load = when == Start::AfterLoad;
        format!(
            "(() => {{\n{HARNESS_JS}\ninstall_harness({global}, {regexes}, {source_url}, {after_load});\n}})();\n//# sourceURL=e2e-harness.js\n"
        )
    }

    /// The userscript, wrapped in a function that the harness's `start` (see
    /// [`Userscript::harness_script`]) runs.
    ///
    /// The userscript is the whole body of that function, so that its
    /// top-level names stay out of the page's global scope, and a
    /// `"use strict"` at its top would apply. The wrapper's opening goes on
    /// the userscript's first line, so that line numbers in stack traces are
    /// the line numbers in `expand.user.js`.
    pub fn wrapped_script(&self) -> String {
        format!(
            "window.{HARNESS_GLOBAL}.start(() => {{ {}\n}});\n//# sourceURL={USERSCRIPT_SOURCE_URL}\n",
            self.source
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_patterns_follow_chrome() {
        let cases = [
            (
                "https://*.substack.com/*",
                "https://foo.substack.com/p/x/comments",
                true,
            ),
            ("https://*.substack.com/*", "https://substack.com/", true),
            (
                "https://*.substack.com/*",
                "https://a.b.substack.com/",
                true,
            ),
            (
                "https://*.substack.com/*",
                "https://notsubstack.com/",
                false,
            ),
            (
                "https://*.substack.com/*",
                "http://foo.substack.com/",
                false,
            ),
            (
                "https://*.substack.com/*",
                "https://foo.substack.com.evil.com/",
                false,
            ),
            (
                "https://*.substack.com/*",
                "https://foo.substack.com:8443/x",
                true,
            ),
            (
                "https://www.google.com/search?*",
                "https://www.google.com/search?q=a",
                true,
            ),
            (
                "https://www.google.com/search?*",
                "https://www.google.com/searchX?q=a",
                false,
            ),
            (
                "https://www.google.com/search?*",
                "https://www.google.com/maps",
                false,
            ),
            (
                "https://www.imdb.com/title/*/reviews*",
                "https://www.imdb.com/title/tt1/reviews?a",
                true,
            ),
            (
                "https://www.imdb.com/title/*/reviews*",
                "https://www.imdb.com/title/tt1/",
                false,
            ),
            ("*://example.com/a.b*", "http://example.com/a.b", true),
            ("*://example.com/a.b*", "https://example.com/aXb", false),
            ("https://*/*", "https://x.org/", true),
            ("https://evil.com/*", "https://evil.com.example/", false),
        ];
        for (pattern, url, expected) in cases {
            let re = regex::Regex::new(&match_pattern_to_regex(pattern).unwrap()).unwrap();
            assert_eq!(re.is_match(url), expected, "{pattern} vs {url}");
        }
    }

    #[test]
    fn unsupported_match_patterns_are_errors() {
        for pattern in [
            "https://x.com/page",
            "https://X.com/*",
            "https://x.com/*#a",
            "<all_urls>",
            "ftp://x.com/*",
            "https://x.com",
            "https://x.com:80/*",
            "https://a*.com/*",
        ] {
            assert!(match_pattern_to_regex(pattern).is_err(), "{pattern}");
        }
    }

    #[test]
    fn expand_user_js_parses_and_matches_the_tested_sites() {
        let script = Userscript::load().unwrap();
        assert!(script.matches("https://dispatchesfromtheautismwars.substack.com/p/x/comments"));
        assert!(script.matches("https://substack.com/@x/note/c-1"));
        assert!(script.matches("https://www.youtube.com/watch?v=x"));
        assert!(script.matches("https://store.steampowered.com/app/1/X/"));
        assert!(script.matches("https://www.lesswrong.com/users/x"));
        assert!(script.matches("https://mastodon.social/@x/1"));
        assert!(script.matches("https://github.com/rust-lang/rust/issues/57640"));
        assert!(script.matches("https://gist.github.com/x/5095670735ba941a6090a69fce4183df"));
        assert!(script.matches("https://www.goodreads.com/book/show/1-x"));
        assert!(script.matches("https://www.cnn.com/2026/11/12/business/live-news/x"));
        assert!(script.matches("https://edition.cnn.com/2026/11/12/business/live-news/x"));
        assert!(!script.matches("https://www.cnn.com/2026/11/12/business/x"));
        assert!(script.matches("https://www.nytimes.com/live/2024/07/07/world/x"));
        assert!(script.matches("https://tvtropes.org/pmwiki/pmwiki.php/X/Y"));
        assert!(!script.matches("https://example.com/"));
    }

    #[test]
    fn unsupported_metadata_is_an_error() {
        let parse = |meta: &str| {
            Userscript::parse(format!("// ==UserScript==\n{meta}\n// ==/UserScript==\n"))
        };
        assert!(parse("// @grant none\n// @match https://x.com/*").is_ok());
        assert!(
            parse("// @grant none\n// @match https://x.com/*\n// @run-at document-start").is_err()
        );
        assert!(parse("// @grant GM_xmlhttpRequest\n// @match https://x.com/*").is_err());
        assert!(parse("// @match https://x.com/*").is_err());
        assert!(parse("// @grant none").is_err());
    }

    #[test]
    fn metadata_lines_follow_violentmonkey() {
        let source = "// ==UserScript==\n\n// @grant  none\n//@run-at document-start\n//  @match https://y.com/*\n  // @match https://x.com/*\nlet x;\n// ==/UserScript==\n// @match https://z.com/*\n";
        let script = Userscript::parse(source.to_string()).unwrap();
        assert!(script.matches("https://x.com/"));
        assert!(!script.matches("https://y.com/"));
        assert!(!script.matches("https://z.com/"));
    }
}
