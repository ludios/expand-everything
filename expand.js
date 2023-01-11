// ==UserScript==
// @name        expand-everything
// @namespace   ludios
// @match       https://www.goodreads.com/book/show/*
// @match       https://www.imdb.com/title/*/reviews*
// @match       https://www.youtube.com/*
// @match       https://www.linkedin.com/*
// @match       https://*.substack.com/*
// @match       https://tvtropes.org/*
// @match       https://news.ycombinator.com/*
// @grant       none
// @version     0.1
// @author      ludios
// @description Click the "show more" links to expand all the text on a page
// ==/UserScript==

const loc = window.location.href;

function queryElements(selector, callback) {
  const elements = document.querySelectorAll(selector);
  elements.forEach(element => callback(element));
}

let pageCounter = 0;

// Observe some selectors and run a callback for each selected element.
function observe(maxMutations, selectors, callback) {
  let mutations = 0;
  const observer = new MutationObserver(() => {
    mutations++;
    if (mutations >= maxMutations) {
      console.log(`disconnecting MutationObserver after ${mutations} mutations to avoid slowing down the page`);
      observer.disconnect();
    }
    for (let selector of selectors) {
      queryElements(selector, callback);
    }
  });

  function reobserve() {
    // For elements present before MutationObserver
    for (let selector of selectors) {
      queryElements(selector, callback);
    }

    observer.observe(document.documentElement, {
      childList: true,
      subtree: true,
    });
  }

  reobserve();

  navigation.addEventListener('navigate', ev => {
    console.log('navigated, resetting MutationObserver');
    mutations = 0;
    pageCounter = 0;
    reobserve();
  });
}

// Click on something if it hasn't already been clicked.
const alreadyClicked = new WeakMap();
function clickIfUnclicked(el) {
  if (alreadyClicked.get(el)) {
    return;
  }
  alreadyClicked.set(el, true);
  el.click();
}

// Test page: https://www.goodreads.com/book/show/931984.The_Presentation_of_Self_in_Everyday_Life
// Expected: all the lengthy user book reviews are expanded
if (loc.startsWith("https://www.goodreads.com/book/show/")) {
  observe(100000, ['a[data-text-id][onclick^="swapContent("]'], el => {
    if (el.innerText == "...more") {
      el.click();
    }
  });
}

// Test page: https://www.imdb.com/title/tt0809535/reviews?ref_=tt_urv
// Expected: all the user film reviews are expanded and not cut off
// Expected: all the "Warning: Spoilers" reviews are showing
//
// Test page: https://www.imdb.com/title/tt6710474/reviews?ref_=tt_urv
// Expected: all the user film reviews are expanded and not cut off
// Expected: all the "Warning: Spoilers" reviews are showing
// Expected: "Load more" is clicked 5 times but not more
//
// (We don't want to click "Load more" so many times that we slow down
// the page and also risk getting banned.)
if (loc.startsWith("https://www.imdb.com/title/")) {
  observe(100, [
    // Longer film reviews by users and reviews with spoilers
    '.ipl-expander:not(.ipl-expander--expanded) > div > div',
    // "Load more" button at the end of a set of reviews
    'button.ipl-load-more__button',
  ], el => {
    if (el.tagName == "BUTTON") {
      if (pageCounter < 5) {
        pageCounter++;
        console.log(`page counter: ${pageCounter}`);
        el.click();
      }
    } else {
      // Avoid MutationObserver loop: imdb adds .ipl-expander--expanded
      // to the element some time _after_ you click.
      clickIfUnclicked(el);
    }
  });
}

// Test page: https://www.youtube.com/watch?v=ben9qDbrLYU
// Test page: https://www.youtube.com/ followed by click on a video
// Expected: video description is expanded
// Expected: "show more" in the comments is clicked, for the loaded comments
// Expected: replies for the top-most comments are clicked
// Expected: the MutationObserver is disconnected soon after page load to avoid slowing things down
if (loc.startsWith("https://www.youtube.com/")) {
  observe(100, [
    // Video description "show more"
    'div#description > div#description-inner > #description-inline-expander > .button.ytd-text-inline-expander#expand',
    // Video comments "show more" and replies
    '.more-button',
  ], el => {
    el.click();
  });
}

// Test page: https://www.linkedin.com/in/daraweiss
// Expected: "show more" is clicked in the profile sections, revealing all text
if (loc.startsWith("https://www.linkedin.com/")) {
  observe(100, [
    '.inline-show-more-text__button',
  ], el => {
    el.click();
  });
}

// Test page: https://shimmeringvoid.substack.com/
// Test page: https://jenniferdaniel.substack.com/
// Expected: the subscribe modal with "No thanks" is bypassed
//
// Test page: https://philo.substack.com/p/scarcity-truthers/comments#comment-6543771
// Test page: https://philo.substack.com/p/scarcity-truthers/comment/6543771
// Expected: "Expand full comment" is clicked; the full text of the comment is shown
if (window.location.host.endsWith(".substack.com")) {
  observe(100, [
    // "No thanks" button; clicking removes the overlay
    'button.maybe-later',
    // "Expand full comment"
    'div.comment-body:not(.expanded) > div.show-all-toggle > div.show-all-toggle-label',
  ], el => {
    el.click();
  });
}

// Test page: https://tvtropes.org/pmwiki/pmwiki.php/OvershadowedByControversy/WebOriginal
// Expected: all folders are opened
if (loc.startsWith("https://tvtropes.org/")) {
  observe(10, [
    // "open/close all folders"
    '.toggle-all-folders-button:not(.is-open)',
  ], el => {
    clickIfUnclicked(el);
  });
}

// Test page: https://news.ycombinator.com/item?id=34282033 as a logged-in user with showdead enabled
// Expected: all comments are shown; no '[N more]' links visible
if (loc.startsWith("https://news.ycombinator.com/item")) {
  // No need for MutationObserver
  queryElements('a.togg.clicky', el => {
    if (el.innerText.endsWith(" more]")) {
      el.click();
    }
  });
}
