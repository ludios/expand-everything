// ==UserScript==
// @name        expand-everything
// @namespace   ludios
// @match       https://www.goodreads.com/book/show/*
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

  let mutations = 0;
  const observer = new MutationObserver(() => {
    console.log(`mutations: ${mutations}`);
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


if (loc.startsWith("https://www.goodreads.com/book/show/")) {
  observe(['a[data-text-id][onclick^="swapContent("]'], el => {
  if (el.innerText == "...more") {
    el.click();
  }
  });
}
