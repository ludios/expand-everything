// ==UserScript==
// @name        expand-everything
// @namespace   ludios
// @match       https://www.goodreads.com/book/show/*
// @match       https://www.imdb.com/title/*/reviews*
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
function observe(selectors, callback) {
  // For elements present before MutationObserver
  for (let selector of selectors) {
    queryElements(selector, callback);
  }

  const observer = new MutationObserver(() => {
    for (let selector of selectors) {
      queryElements(selector, callback);
    }
  });

  observer.observe(document.documentElement, {
    //attributes: true,
    childList: true,
    //characterData: true,
    subtree: true,
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
  observe(['a[data-text-id][onclick^="swapContent("]'], el => {
    if (el.innerText == "...more") {
      el.click();
    }
  });
}

// Test page: https://www.imdb.com/title/tt0809535/reviews?ref_=tt_urv
// Expected: all the user film reviews are expanded and not cut off
// Expected: all the "Warning: Spoilers" reviews are showing
if (loc.startsWith("https://www.imdb.com/title/")) {
  observe(['.ipl-expander:not(.ipl-expander--expanded) > div > div'], el => {
    // Avoid MutationObserver loop: imdb adds .ipl-expander--expanded to the element some time after you click.
    clickIfUnclicked(el);
  });
}
