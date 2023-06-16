// ==UserScript==
// @name        Expand Everything
// @description Click the "show more" links to expand all the text on a page
// @namespace   ludios
// @author      ludios
// @license     AGPL-3.0; https://www.gnu.org/licenses/agpl-3.0.en.html
// @grant       none
// @version     1.7.0
//
// @match       https://www.goodreads.com/book/show/*
// @match       https://www.imdb.com/title/*/reviews*
// @match       https://www.youtube.com/*
// @match       https://*.linkedin.com/*
// @match       https://*.substack.com/*
// @match       https://tvtropes.org/*
// @match       https://news.ycombinator.com/*
// @match       https://github.com/*
// @match       https://www.quora.com/*
// @match       https://old.reddit.com/*
// @match       https://www.lesswrong.com/*
// @match       https://cohost.org/*
// @match       https://nextdoor.com/*
// @match       https://store.steampowered.com/*
//
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
//
// Test page: https://www.goodreads.com/book/show/39736150-the-little-typer
// Expected: all the lengthy user book reviews are expanded
if (loc.startsWith("https://www.goodreads.com/book/show/")) {
  observe(1000, ['button[aria-label="Tap to show more review"]'], el => {
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
//
// Test page: https://gurwinder.substack.com/p/tiktok-may-be-a-chinese-bio-weapon/comments
// Test page: https://mrgirl.substack.com/p/the-destiny-report/comments
// Expected: "Load more" button at the bottom is clicked; all comments are shown
if (window.location.host.endsWith(".substack.com")) {
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

// Test page: https://github.com/rust-lang/rust/pull/95035
// Expected: all comments are loaded; "N hidden items" button is not visible; "1 similar comment" is not visible
//
// Test page: https://github.com/rust-lang/rust/issues/57640
// Expected: all comments are loaded; "N hidden items" button is not visible
//
// Test page: https://github.com/JustAnotherArchivist/snscrape/issues/634
// Expected: spam/outdated/duplicate/off-topic comments are expanded
//
// Test page: https://github.com/NixOS/nixpkgs/pull/194310
// Expected: outdated review comments are expanded
//
// Test page: https://github.com/A
// Expected: activity is not expanded; URL does not start changing
if (loc.startsWith("https://github.com/")) {
  observe(1000, [
    // "N hidden items; Load more..."
    //
    // We don't want all button.ajax-pagination-btn because clicking button.ajax-pagination-btn
    // on a profile page e.g. https://github.com/A causes the URL to start changing.
    '#js-progressive-timeline-item-container button.ajax-pagination-btn',
    // "N similar comments"
    'summary.pagination-loader-container > .Details-content--closed',
    // "Show comment"
    'div.minimized-comment > details > summary > div > .Details-content--closed',
    // "Show resolved"
    'summary[role="button"] > div > span.Details-content--closed',
  ], el => {
    clickIfUnclicked(el);
  });
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
if (loc.startsWith("https://www.lesswrong.com/")) {
  observe(100, [
    // Collapsed comments
    '.SingleLineComment-truncatedHighlight',
  ], el => {
    clickIfUnclicked(el);
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
