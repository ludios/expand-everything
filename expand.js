// ==UserScript==
// @name        expand-everything
// @namespace   ludios
// @match       https://www.goodreads.com/book/show/*
// @match       https://www.imdb.com/title/*/reviews*
// @match       https://www.youtube.com/watch*
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

// Observe some selectors and run a callback for each selected element.
function observe(maxMutations, selectors, callback) {
  // For elements present before MutationObserver
  for (let selector of selectors) {
    queryElements(selector, callback);
  }

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
    observer.observe(document.documentElement, {
      childList: true,
      subtree: true,
    });
  }

  reobserve();

  navigation.addEventListener('navigate', ev => {
    console.log('navigated, resetting MutationObserver');
    mutations = 0;
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
// Note: we do not press the "Load more" button on e.g.
// https://www.imdb.com/title/tt6710474/reviews?ref_=tt_urv
// because there might be far too many reviews.
if (loc.startsWith("https://www.imdb.com/title/")) {
  observe(100, ['.ipl-expander:not(.ipl-expander--expanded) > div > div'], el => {
    // Avoid MutationObserver loop: imdb adds .ipl-expander--expanded
    // to the element some time _after_ you click.
    clickIfUnclicked(el);
  });
}

if (loc.startsWith("https://www.youtube.com/")) {
  observe(100, [
    // Video description "show more"
    '.button.ytd-text-inline-expander#expand',
    // Video comments "show more" and replies
    '.more-button',
  ], el => {
    el.click();
  });
}
