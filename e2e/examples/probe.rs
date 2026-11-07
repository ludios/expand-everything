//! Look at a page as the userscript leaves it, to write or fix a test.
//!
//!     cargo run --example probe -- URL [--wait SECONDS] [--html FILE] [EXPRESSION...]
//!
//! Opens URL in Chrome with the userscript injected, waits SECONDS (default
//! 10), and then prints each click that the userscript made and the JSON value
//! of each JavaScript EXPRESSION (evaluated in the page's main world). With
//! `--html`, first saves the page's DOM to FILE, to study it without loading
//! the page again.

use anyhow::{Context, Result, bail};
use expand_everything_e2e::userscript::HARNESS_GLOBAL;
use expand_everything_e2e::{open, with_chrome};
use std::time::Duration;

/// What the command line asks for.
struct Args {
    url: String,
    wait: Duration,
    html: Option<String>,
    expressions: Vec<String>,
}

/// Parse the command line (see the module docs).
fn parse_args() -> Result<Args> {
    let mut args = std::env::args().skip(1);
    let mut url = None;
    let mut wait = Duration::from_secs(10);
    let mut html = None;
    let mut expressions = Vec::new();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--wait" => {
                let seconds = args.next().context("--wait needs a value")?;
                wait = Duration::try_from_secs_f64(seconds.parse().context("parsing --wait")?)
                    .context("--wait needs a number of seconds")?;
            }
            "--html" => html = Some(args.next().context("--html needs a value")?),
            _ if arg.starts_with("--") => bail!("unknown option {arg}"),
            _ if url.is_none() => url = Some(arg),
            _ => expressions.push(arg),
        }
    }
    let Some(url) = url else {
        bail!("usage: probe URL [--wait SECONDS] [--html FILE] [EXPRESSION...]");
    };
    Ok(Args {
        url,
        wait,
        html,
        expressions,
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = parse_args()?;
    with_chrome(async |browser| {
        let tab = open(browser, &args.url).await?;
        tokio::time::sleep(args.wait).await;
        if let Some(path) = &args.html {
            std::fs::write(path, tab.content().await?).with_context(|| format!("writing {path}"))?;
        }
        let clicks: serde_json::Value = tab
            .evaluate_main(format!(
                "window.{HARNESS_GLOBAL}.clicks.map((c) => [Math.round(c.time), c.element.tagName + '.' + c.element.className, c.text])"
            ))
            .await?;
        println!("clicks: {}", serde_json::to_string_pretty(&clicks)?);
        for expression in &args.expressions {
            let value: serde_json::Value = tab.evaluate_main(expression).await?;
            println!("{expression}\n=> {}", serde_json::to_string_pretty(&value)?);
        }
        Ok(())
    })
    .await
}
