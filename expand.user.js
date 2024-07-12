// ==UserScript==
// @name        Expand Everything
// @description Click the "show more" links to expand all the text on a page
// @namespace   ludios
// @author      ludios
// @license     AGPL-3.0; https://www.gnu.org/licenses/agpl-3.0.en.html
// @grant       none
// @version     2.8.8
//
// @match       https://www.goodreads.com/book/show/*
// @match       https://www.imdb.com/title/*/reviews*
// @match       https://www.youtube.com/*
// @match       https://*.linkedin.com/*
// @match       https://tvtropes.org/*
// @match       https://news.ycombinator.com/*
// @match       https://github.com/*
// @match       https://gist.github.com/*
// @match       https://www.quora.com/*
// @match       https://old.reddit.com/*
// @match       https://www.lesswrong.com/*
// @match       https://cohost.org/*
// @match       https://nextdoor.com/*
// @match       https://store.steampowered.com/*
// @match       https://www.patreon.com/*
// @match       https://twitter.com/*
// @match       https://www.google.com/search?*
// @match       https://www.nytimes.com/*
//
// Substack sites
// @match       https://substack.com/*
// @match       https://*.substack.com/*
// @match       https://www.platformer.news/*
// @match       https://www.henrikkarlsson.xyz/*
// @match       https://www.experimental-history.com/*
// @match       https://www.astralcodexten.com/*
// @match       https://www.computerenhance.com/*
// @match       https://www.tracingwoodgrains.com/*
//
// Mastodon sites
// @match       https://mastodon.social/*
// @match       https://social.pixie.town/*
//
// XenForo sites
// @match       https://www.hwinfo.com/forum/*
//
// StackExchange sites
// Copied from https://stackexchange.com/sites?view=list#traffic
// @match       https://3dprinting.stackexchange.com/*
// @match       https://academia.stackexchange.com/*
// @match       https://ai.stackexchange.com/*
// @match       https://alcohol.stackexchange.com/*
// @match       https://android.stackexchange.com/*
// @match       https://anime.stackexchange.com/*
// @match       https://apple.stackexchange.com/*
// @match       https://arduino.stackexchange.com/*
// @match       https://area51.stackexchange.com/*
// @match       https://askubuntu.com/*
// @match       https://astronomy.stackexchange.com/*
// @match       https://aviation.stackexchange.com/*
// @match       https://bicycles.stackexchange.com/*
// @match       https://bioacoustics.stackexchange.com/*
// @match       https://bioinformatics.stackexchange.com/*
// @match       https://biology.stackexchange.com/*
// @match       https://bitcoin.stackexchange.com/*
// @match       https://blender.stackexchange.com/*
// @match       https://boardgames.stackexchange.com/*
// @match       https://bricks.stackexchange.com/*
// @match       https://buddhism.stackexchange.com/*
// @match       https://cardano.stackexchange.com/*
// @match       https://chemistry.stackexchange.com/*
// @match       https://chess.stackexchange.com/*
// @match       https://chinese.stackexchange.com/*
// @match       https://christianity.stackexchange.com/*
// @match       https://civicrm.stackexchange.com/*
// @match       https://codegolf.stackexchange.com/*
// @match       https://codereview.stackexchange.com/*
// @match       https://coffee.stackexchange.com/*
// @match       https://communitybuilding.stackexchange.com/*
// @match       https://computergraphics.stackexchange.com/*
// @match       https://conlang.stackexchange.com/*
// @match       https://cooking.stackexchange.com/*
// @match       https://craftcms.stackexchange.com/*
// @match       https://crafts.stackexchange.com/*
// @match       https://crypto.stackexchange.com/*
// @match       https://cs.stackexchange.com/*
// @match       https://cseducators.stackexchange.com/*
// @match       https://cstheory.stackexchange.com/*
// @match       https://datascience.stackexchange.com/*
// @match       https://dba.stackexchange.com/*
// @match       https://devops.stackexchange.com/*
// @match       https://diy.stackexchange.com/*
// @match       https://drones.stackexchange.com/*
// @match       https://drupal.stackexchange.com/*
// @match       https://dsp.stackexchange.com/*
// @match       https://earthscience.stackexchange.com/*
// @match       https://ebooks.stackexchange.com/*
// @match       https://economics.stackexchange.com/*
// @match       https://electronics.stackexchange.com/*
// @match       https://elementaryos.stackexchange.com/*
// @match       https://ell.stackexchange.com/*
// @match       https://emacs.stackexchange.com/*
// @match       https://engineering.stackexchange.com/*
// @match       https://english.stackexchange.com/*
// @match       https://eosio.stackexchange.com/*
// @match       https://es.stackoverflow.com/*
// @match       https://esperanto.stackexchange.com/*
// @match       https://ethereum.stackexchange.com/*
// @match       https://expatriates.stackexchange.com/*
// @match       https://expressionengine.stackexchange.com/*
// @match       https://fitness.stackexchange.com/*
// @match       https://freelancing.stackexchange.com/*
// @match       https://french.stackexchange.com/*
// @match       https://gamedev.stackexchange.com/*
// @match       https://gaming.stackexchange.com/*
// @match       https://gardening.stackexchange.com/*
// @match       https://genealogy.stackexchange.com/*
// @match       https://german.stackexchange.com/*
// @match       https://gis.stackexchange.com/*
// @match       https://graphicdesign.stackexchange.com/*
// @match       https://ham.stackexchange.com/*
// @match       https://hardwarerecs.stackexchange.com/*
// @match       https://hermeneutics.stackexchange.com/*
// @match       https://hinduism.stackexchange.com/*
// @match       https://history.stackexchange.com/*
// @match       https://homebrew.stackexchange.com/*
// @match       https://hsm.stackexchange.com/*
// @match       https://interpersonal.stackexchange.com/*
// @match       https://iot.stackexchange.com/*
// @match       https://iota.stackexchange.com/*
// @match       https://islam.stackexchange.com/*
// @match       https://italian.stackexchange.com/*
// @match       https://ja.stackoverflow.com/*
// @match       https://japanese.stackexchange.com/*
// @match       https://joomla.stackexchange.com/*
// @match       https://judaism.stackexchange.com/*
// @match       https://korean.stackexchange.com/*
// @match       https://languagelearning.stackexchange.com/*
// @match       https://latin.stackexchange.com/*
// @match       https://law.stackexchange.com/*
// @match       https://lifehacks.stackexchange.com/*
// @match       https://linguistics.stackexchange.com/*
// @match       https://literature.stackexchange.com/*
// @match       https://magento.stackexchange.com/*
// @match       https://martialarts.stackexchange.com/*
// @match       https://math.stackexchange.com/*
// @match       https://matheducators.stackexchange.com/*
// @match       https://mathematica.stackexchange.com/*
// @match       https://mathoverflow.net/*
// @match       https://mattermodeling.stackexchange.com/*
// @match       https://mechanics.stackexchange.com/*
// @match       https://medicalsciences.stackexchange.com/*
// @match       https://meta.stackexchange.com/*
// @match       https://monero.stackexchange.com/*
// @match       https://money.stackexchange.com/*
// @match       https://movies.stackexchange.com/*
// @match       https://music.stackexchange.com/*
// @match       https://musicfans.stackexchange.com/*
// @match       https://mythology.stackexchange.com/*
// @match       https://networkengineering.stackexchange.com/*
// @match       https://opendata.stackexchange.com/*
// @match       https://opensource.stackexchange.com/*
// @match       https://or.stackexchange.com/*
// @match       https://outdoors.stackexchange.com/*
// @match       https://parenting.stackexchange.com/*
// @match       https://patents.stackexchange.com/*
// @match       https://pets.stackexchange.com/*
// @match       https://philosophy.stackexchange.com/*
// @match       https://photo.stackexchange.com/*
// @match       https://physics.stackexchange.com/*
// @match       https://pm.stackexchange.com/*
// @match       https://poker.stackexchange.com/*
// @match       https://politics.stackexchange.com/*
// @match       https://portuguese.stackexchange.com/*
// @match       https://proofassistants.stackexchange.com/*
// @match       https://psychology.stackexchange.com/*
// @match       https://pt.stackoverflow.com/*
// @match       https://puzzling.stackexchange.com/*
// @match       https://quant.stackexchange.com/*
// @match       https://quantumcomputing.stackexchange.com/*
// @match       https://raspberrypi.stackexchange.com/*
// @match       https://retrocomputing.stackexchange.com/*
// @match       https://reverseengineering.stackexchange.com/*
// @match       https://robotics.stackexchange.com/*
// @match       https://rpg.stackexchange.com/*
// @match       https://ru.stackoverflow.com/*
// @match       https://rus.stackexchange.com/*
// @match       https://russian.stackexchange.com/*
// @match       https://salesforce.stackexchange.com/*
// @match       https://scicomp.stackexchange.com/*
// @match       https://scifi.stackexchange.com/*
// @match       https://security.stackexchange.com/*
// @match       https://serverfault.com/*
// @match       https://sharepoint.stackexchange.com/*
// @match       https://sitecore.stackexchange.com/*
// @match       https://skeptics.stackexchange.com/*
// @match       https://softwareengineering.stackexchange.com/*
// @match       https://softwarerecs.stackexchange.com/*
// @match       https://solana.stackexchange.com/*
// @match       https://sound.stackexchange.com/*
// @match       https://space.stackexchange.com/*
// @match       https://spanish.stackexchange.com/*
// @match       https://sports.stackexchange.com/*
// @match       https://sqa.stackexchange.com/*
// @match       https://stackapps.com/*
// @match       https://stackexchange.com/*
// @match       https://stackoverflow.com/*
// @match       https://stats.stackexchange.com/*
// @match       https://stellar.stackexchange.com/*
// @match       https://substrate.stackexchange.com/*
// @match       https://superuser.com/*
// @match       https://sustainability.stackexchange.com/*
// @match       https://tex.stackexchange.com/*
// @match       https://tezos.stackexchange.com/*
// @match       https://tor.stackexchange.com/*
// @match       https://travel.stackexchange.com/*
// @match       https://tridion.stackexchange.com/*
// @match       https://ukrainian.stackexchange.com/*
// @match       https://unix.stackexchange.com/*
// @match       https://ux.stackexchange.com/*
// @match       https://vegetarianism.stackexchange.com/*
// @match       https://vi.stackexchange.com/*
// @match       https://video.stackexchange.com/*
// @match       https://webapps.stackexchange.com/*
// @match       https://webmasters.stackexchange.com/*
// @match       https://woodworking.stackexchange.com/*
// @match       https://wordpress.stackexchange.com/*
// @match       https://workplace.stackexchange.com/*
// @match       https://worldbuilding.stackexchange.com/*
// @match       https://writing.stackexchange.com/*
//
// ==/UserScript==

const loc = window.location.href;

function queryElements(selector, callback) {
  const elements = document.querySelectorAll(selector);
  elements.forEach(element => callback(element));
}

const logPrefix = "Expand Everything: ";
let pageCounter = 0;
let mutations = 0;
let observer = null;
let lastHref = location.href;
let locationInterval = null;

// Observe some selectors and run a callback for each selected element.
function observe(maxMutations, selectors, callback) {
  if (observer !== null) {
    throw new Error(`observe(...) called more than once`);
  }

  observer = new MutationObserver(() => {
    mutations++;
    if (mutations >= maxMutations) {
      console.log(`${logPrefix}disconnecting MutationObserver after ${mutations} mutations to avoid slowing down the page`);
      observer.disconnect();
    }
    for (const selector of selectors) {
      queryElements(selector, callback);
    }
  });

  function reobserve() {
    // Process elements that were present before MutationObserver
    for (const selector of selectors) {
      queryElements(selector, callback);
    }

    // Start observing
    observer.observe(document.documentElement, {
      childList: true,
      subtree: true,
    });
  }

  reobserve();

  function navigated() {
    console.log(`${logPrefix}navigated, resetting alreadyClicked and MutationObserver`);
    observer.disconnect();
    resetAlreadyClicked();
    mutations = 0;
    pageCounter = 0;
    reobserve();
  }

  function startPollingLocation() {
    if (locationInterval !== null) {
      throw new Error(`locationInterval already exists: ${locationInterval}`);
    }
    locationInterval = setInterval(
      () => {
        if (location.href !== lastHref) {
          lastHref = location.href;
          navigated();
        }
      },
      1000
    );
  }

  function stopPollingLocation() {
    if (locationInterval === null) {
      throw new Error(`locationInterval === null`);
    }
    clearInterval(locationInterval);
    locationInterval = null;
  }

  // So far, only Chromium-based browsers have Navigation API: https://caniuse.com/mdn-api_navigation
  if (window.navigation && navigation.addEventListener) {
    console.log(`${logPrefix}using Navigation API to detect location changes`);
    navigation.addEventListener('navigate', (_ev) => {
      navigated();
    });
  } else {
    // https://stackoverflow.com/questions/34999976/detect-changes-on-the-url
    // suggests that setInterval is the best way to do it when we lack Navigation API.
    console.log(`${logPrefix}using setInterval(..., 1000) to detect location changes`);
    startPollingLocation();
    // Not necessary for correctness, but to save power, kill our setInterval when the page isn't visible.
    document.addEventListener("visibilitychange", function() {
      if (document.hidden) {
        console.log(`${logPrefix}page has become hidden, stopping setInterval`);
        stopPollingLocation();
      } else {
        console.log(`${logPrefix}page has become visible, starting setInterval`);
        startPollingLocation();
      }
    });
  }
}

let alreadyClicked;
function resetAlreadyClicked() {
  alreadyClicked = new WeakMap();
}
resetAlreadyClicked();

// Click on something if it hasn't already been clicked.
function clickIfUnclicked(el) {
  if (alreadyClicked.get(el)) {
    return;
  }
  alreadyClicked.set(el, true);
  el.click();
}

// Test page: https://www.goodreads.com/book/show/931984.The_Presentation_of_Self_in_Everyday_Life
// Expected: all the lengthy user book reviews are expanded
//
// Test page: https://www.goodreads.com/book/show/39736150-the-little-typer
// Expected: all the lengthy user book reviews are expanded
if (loc.startsWith("https://www.goodreads.com/book/show/")) {
  observe(1000, [
    'button[aria-label="Tap to show more review"]',
    'button[aria-label="Tap to show more about the author"]',
  ], el => {
    clickIfUnclicked(el);
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
        console.log(`${logPrefix}page counter: ${pageCounter}`);
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
//
// Test page (logged in): https://www.youtube.com/
// Expected: after clicking the profile icon in the top-right, the modal/menu works normally (it is not immediately closed)
if (loc.startsWith("https://www.youtube.com/")) {
  observe(200, [
    // Video description "show more"
    'div#description > div#description-inner > #description-inline-expander > .button.ytd-text-inline-expander#expand',
    // Video comments "show more" and replies
    'div.ytd-comment-replies-renderer#expander .more-button#more-replies',
  ], el => {
    // We can't just el.click() unconditionally because that causes menus/modals to lose focus
    // and disappear.
    //
    // We can't use clickIfUnclicked due to YouTube oddities after navigating the SPA (the video
    // description doesn't expand after clicking too early), but conveniently YouTube sets "hidden"
    // on things that don't need to be clicked any more.
    if (!el.hidden) {
      el.click();
    }
  });
}

// Test page: https://www.linkedin.com/in/daraweiss (logged out)
// Expected: "show more" is clicked in the profile sections, revealing all text
//
// Test page: https://www.linkedin.com/in/daraweiss (logged in)
// Expected: "show more" is clicked in the profile sections, revealing all text, browser is _not_ redirected to another page like https://www.linkedin.com/feed/update/urn:li:activity:7150642313440985089/
//
// Test page: https://www.linkedin.com/pulse/how-scaleways-object-storage-failed-me-files-lost-lack-di-battista-uvyme/
// Expected: comments at the bottom are expanded; "…see more" is not visible
if (loc.startsWith("https://www.linkedin.com/")) {
  observe(100, [
    // "show more" in profile sections
    //'.inline-show-more-text__link-container-collapsed > .inline-show-more-text__button',
    // "…see more" in the comments at the bottom of posts
    'button.feed-shared-inline-show-more-text__see-more-less-toggle.see-more',
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
//
// Test page: https://gurwinder.substack.com/p/tiktok-may-be-a-chinese-bio-weapon/comments
// Test page: https://mrgirl.substack.com/p/the-destiny-report/comments
// Expected: "Load more" button at the bottom is clicked; all comments are shown
if (
  window.location.host.endsWith(".substack.com") ||
  window.location.host === "www.platformer.news" ||
  window.location.host === "www.henrikkarlsson.xyz" ||
  window.location.host === "www.experimental-history.com" ||
  window.location.host === "www.astralcodexten.com" ||
  window.location.host === "www.computerenhance.com" ||
  window.location.host === "www.tracingwoodgrains.com"
) {
  observe(100, [
    // "No thanks" button; clicking removes the overlay
    'button.maybe-later',
    // "Expand full comment"
    'div.comment-body:not(.expanded) > div.show-all-toggle > div.show-all-toggle-label',
    // "Load more" at the end of a comments page
    'button.button.collapsed-reply',
  ], el => {
    el.click();
  });
}

// Test page: https://substack.com/@meaningness/note/c-59538921
// Expected: all comments are expanded; "See more..." is not visible
if (loc.startsWith("https://substack.com/")) {
  observe(100, [
    // "See more..."
    'div.pencraft[class*=_seeMoreText_] > span.pencraft > a.pencraft',
  ], el => {
    if (el.innerText === "See more...") {
      el.click();
    }
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
//
// Test page: https://news.ycombinator.com/threads?id=huppeldepup (or 'next' to the page with the collapsed 'I used to not worry about climate change. Now I do...')
// Expected: all comments are shown; no '[N more]' links visible
if (loc.startsWith("https://news.ycombinator.com/")) {
  // No need for MutationObserver
  queryElements('a.togg.clicky', el => {
    if (el.innerText.endsWith(" more]")) {
      el.click();
    }
  });
}

// Test page: https://github.com/rust-lang/rust/pull/95035
// Expected: all comments are loaded; "N hidden items" button is not visible; "1 similar comment" is not visible
//
// Test page: https://github.com/rust-lang/rust/issues/57640
// Expected: all comments are loaded; "N hidden items" button is not visible
//
// Test page: https://github.com/JustAnotherArchivist/snscrape/issues/634
// Expected: spam/outdated/duplicate/off-topic comments are expanded
//
// Test page: https://github.com/vitejs/vite/issues/8237
// Expected: minimized comments are expanded
//
// Test page: https://github.com/NixOS/nixpkgs/pull/194310
// Expected: outdated review comments are expanded
//
// Test page: https://github.com/A
// Expected: activity is not expanded; URL does not start changing
//
// Test page: https://github.com/PolyMC/PolyMC/commit/ccf282593dcdbe189c99b81b8bc90cb203aed3ee
// Expected: all comments are loaded; "Load more comments..." button is not visible
if (loc.startsWith("https://github.com/")) {
  observe(1000, [
    // "N hidden items; Load more..."
    //
    // We don't want all button.ajax-pagination-btn because clicking button.ajax-pagination-btn
    // on a profile page e.g. https://github.com/A causes the URL to start changing.
    '#js-progressive-timeline-item-container button.ajax-pagination-btn',
    // "Load more comments..." on a commit page
    '#all_commit_comments button.ajax-pagination-btn',
    // "N similar comments"
    'summary.pagination-loader-container > .Details-content--closed',
    // "Show comment"
    'div.minimized-comment > details:not([open]) > summary > div > .Details-content--closed',
    // "Show resolved"
    'summary[role="button"] > div > span.Details-content--closed',
    // "Load diff" button on PRs, where some diffs are collapsed because
    // "Large diffs are not rendered by default" and "This file was deleted".
    // This is disabled by default because some pages are just too large, e.g.
    // https://github.com/sveltejs/svelte/commit/fe8a9ce31d5fb662b3953b318621ea364992e014
    //'button[data-view-component="true"].load-diff-button',
  ], el => {
    clickIfUnclicked(el);
  });
}

// Test page: https://gist.github.com/ivan/5095670735ba941a6090a69fce4183df
// Expected: all comments are loaded; "Load earlier comments..." is not visible
// Expected: when restarting Chrome, tab is not redirected to a /load_comments page
if (loc.startsWith("https://gist.github.com/")) {
  // Wait for the JavaScript to load, because we don't want a button click to navigate us to /load_comments
  window.onload = () => {
    observe(5000, [
      // "Load earlier comments..."
      'form.ajax-pagination-form.js-ajax-pagination[action$="/load_comments"] > button.ajax-pagination-btn'
    ], el => {
      clickIfUnclicked(el);
    });
  }
}

// Test page: https://stackoverflow.com/questions/59156473/what-is-the-difference-between-async-move-and-async-move
// Expected: all comments are loaded; "Show N more comments" is not visible
//
// Test page: https://meta.stackexchange.com/questions/333965/firing-mods-and-forced-relicensing-is-stack-exchange-still-interested-in-cooper?rq=1
// Expected: all comments are loaded; "Show N more comments" is not visible
//
// Test page: https://meta.stackexchange.com/questions/277369/a-terms-of-service-update-restricting-companies-that-scrape-your-profile-informa?rq=1#
// Expected: all comments are loaded; "Show N more comments" is not visible
//
// Test page: https://astronomy.stackexchange.com/questions/51544/is-the-solar-core-hard#
// Expected: all comments are loaded; "Show N more comments" is not visible
if (window.StackExchange) {
  observe(100, [
    // "Show N more comments"
    'a.js-show-link:not(.dno)',
  ], el => {
    el.click();
  });
}

// Test page: https://www.quora.com/Which-of-the-big-four-Google-Microsoft-Amazon-Facebook-tech-companies-have-the-most-selective-hiring-process-for-software-engineers?share=1
// Expected: answers are fully visible; "Continue Reading" is not visible
if (loc.startsWith("https://www.quora.com/")) {
  observe(100, [
    // "Continue Reading"
    '.puppeteer_test_read_more_button',
  ], el => {
    clickIfUnclicked(el);
  });
}

// Test page: https://old.reddit.com/r/Windows10/comments/umf10d/should_windows_10_support_be_extended_beyond_2025/
// Expected: all comments expanded; "[+]" is not visible; "comment score below threshold" is not visible
//
// Test page: https://old.reddit.com/r/nextfuckinglevel/comments/108nsqn/student_creates_a_diy_projection_map_light_show/
// Expected: all comments expanded; "[+]" is not visible; "comment score below threshold" is not visible
if (loc.startsWith("https://old.reddit.com/")) {
  observe(200, [
    // Collapsed comments
    'a.expand[href="javascript:void(0)"][onclick="return togglecomment(this)"]',
    // For "load more comments", use https://github.com/honestbleeps/Reddit-Enhancement-Suite
    // and enable "Never Ending Comments", "Load Child Comments" in the extension options;
    // click "save options".
  ], el => {
    if (el.innerText == "[+]") {
      clickIfUnclicked(el);
    }
  });
}

// Test page: https://www.lesswrong.com/posts/uKp6tBFStnsvrot5t/what-dall-e-2-can-and-cannot-do
// Expected: all comments expanded; no one-line previews
//
// Test page: https://www.lesswrong.com/users/blueiris
// Expected: all downvoted comments are expanded; no [+]
//
// Test page: https://www.lesswrong.com/users/blueiris2
// Expected: all downvoted comments are expanded; no [+]; "Load more" at the end is not visible
if (loc.startsWith("https://www.lesswrong.com/")) {
  observe(100, [
    // Comments truncated to a single line
    '.SingleLineComment-truncatedHighlight',
    // Fully collapsed comments
    'a.CommentsItemMeta-collapse',
    // "Load more" at the end of a /users/ page
    'a.LoadMore-root',
  ], el => {
    if (el.classList.contains("CommentsItemMeta-collapse")) {
      // Click only if [+], not [-]
      if (el.innerText == "[+]") {
        clickIfUnclicked(el);
      }
    } else {
      // .SingleLineComment-truncatedHighlight or a.LoadMore-root
      clickIfUnclicked(el);
    }
  });
}

// Test page: https://cohost.org/mcc/post/718898-andi-s-game-of-the-y
// Expected: the full post is shown; "read more" is not visible
if (loc.startsWith("https://cohost.org/")) {
  observe(100, ['div.overflow-hidden[data-testid="post-body"] > div > a.cursor-pointer.font-bold.text-cherry'], el => {
    if (el.innerText == "read more") {
      clickIfUnclicked(el);
    }
  });
}

// Test page: https://nextdoor.com/p/bSFkfd2Smd5c?view=detail
// Expected: all comments are shown
//
// Test page: https://nextdoor.com/
// Expected: on timeline posts, all posts are expanded
if (loc.startsWith("https://nextdoor.com/")) {
  observe(200, [
    // "See N more comments"
    'button.see-previous-comments-button-paged',
    // "See more" at the end of a post on the timeline
    'button.truncate-view-more-link',
    // "See more" at the end of a comment
    'a.truncate-view-more-link',
  ], el => {
    clickIfUnclicked(el);
  });
}

// Test page: https://store.steampowered.com/app/1147890/Bonfire_Peaks/
// Expected: review text is expanded on all reviews
if (loc.startsWith("https://store.steampowered.com/")) {
  observe(100, ['div.view_more > a[href="#"][onclick]'], el => {
    el.click();
  });
}

// Test page: https://www.hwinfo.com/forum/threads/prometheus-adapter-for-hwinfo-grafana-dashboard.6281/page-5
// Expected: "Rohirm said:" quote at the bottom is expanded; "Click to expand..." is not visible
if (loc.startsWith("https://www.hwinfo.com/forum/")) {
  observe(10, [
    // "Click to expand..."
    'div.js-expandLink > a[role="button"]'
  ], el => {
    el.click();
  });
}

// Test page: https://www.patreon.com/comprehensiblejapanese/posts
// Expected: all posts expanded; "Continue reading" is not visible
if (loc.startsWith("https://www.patreon.com/")) {
  observe(10, [
    // "Continue reading"
    'div[class][data-tag="post-content-collapse"] > div[class] > button[class]'
  ], el => {
    el.click();
  });
}

// Test page: https://social.pixie.town/@joepie91/110978280642849045
// Expected: all toots expanded; "SHOW MORE" is not visible
if (
  window.location.host === "mastodon.social" ||
  window.location.host === "social.pixie.town"
) {
  observe(20, [
    // "SHOW MORE"
    'div[data-spoiler="folded"] button.status__content__spoiler-link'
  ], el => {
    clickIfUnclicked(el);
  });
}

// Test page: https://twitter.com/AChillGhost/status/1761774191920005343
// Expected: all replies expanded; "Show replies" is not visible
if (loc.startsWith("https://twitter.com/")) {
  observe(1000, [
    // "Show replies"
    'div[role="button"] > div > div > div[style] > span[class^="css-"][style="text-overflow: unset;"]'
  ], el => {
    if (el.innerText === "Show replies") {
      el.click();
    }
  });
}

// Test page: https://www.google.com/search?q=Pacific+West+Home+Improvements
// Expected: after clicking '54 Google reviews', all the reviews are expanded and the "More" links are not visible
if (loc.startsWith("https://www.google.com/search")) {
  observe(100, [
    'a.review-more-link[role="button"][aria-expanded="false"]'
  ], el => {
    el.click();
  });
}

// Test page: https://www.nytimes.com/live/2024/07/07/world/france-election-2024
// Expected: All mini articles expanded; "Show more" buttons are not visible
if (loc.startsWith("https://www.nytimes.com/")) {
  observe(20, [
    // "Show more"
    'button[class][data-testid="Show-More"][type="button"][aria-hidden="true"]'
  ], el => {
    clickIfUnclicked(el);
  });
}
