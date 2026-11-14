//! End-to-end tests of `expand.user.js` on live sites, one module per site.
//!
//! Each test loads a page with the userscript injected, waits until the page
//! shows what the userscript should have expanded, and checks that the
//! userscript did the clicking (so that a test doesn't pass only because the
//! site stopped hiding anything). A test of what the userscript must leave
//! alone checks instead that the userscript observes the page but doesn't
//! click it.

mod bloomberg;
mod cnn;
mod criticker;
mod gist;
mod github;
mod goodreads;
mod imdb;
mod lesswrong;
mod mastodon;
mod nytimes;
mod patreon;
mod steam;
mod substack;
mod tvtropes;
mod youtube;
