//! End-to-end tests of `expand.user.js` on live sites, one module per site.
//!
//! Each test loads a page with the userscript injected, waits until the page
//! shows what the userscript should have expanded, and checks that the
//! userscript did the clicking (so that a test doesn't pass only because the
//! site stopped hiding anything).

mod substack;
