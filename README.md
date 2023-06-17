# Expand Everything

*Load more!*, *show more!*, *read more!* - these are the cursed buttons that
website owners make you click before you can see all the information that you
came to read. Expand Everything fixes this by clicking them for you.

<b><a href="#demo">Demo</a></b>.

Efforts are made to avoid slowing things down after the initial load. We use
`MutationObserver` and `querySelectorAll` to watch for the elements that
we need to automatically click, but crucially:
  1) The MutationObserver is disconnected after e.g. 200 mutations to avoid
     slowing things down.
  2) The MutationObserver is reinstalled after an SPA navigation (because
     there might be new elements to click on).

     We use [Navigation API](https://caniuse.com/mdn-api_navigation) when
     available to watch for location changes; otherwise, we fall back to
     `setInterval(..., 1000)`, but stop polling when the page is not visible.

## Install

This is distributed as a userscript, so you need a browser extension that runs userscripts.

[Violentmonkey](https://violentmonkey.github.io/) can be installed in Firefox and Chromium-based browsers. [Userscripts](https://apps.apple.com/us/app/userscripts/id1463298887) works in Safari.

Then, visit [expand.user.js](https://raw.githubusercontent.com/ludios/expand-everything/master/expand.user.js) and _Confirm installation_.

(Alternatively, create a new script and paste in the entire <code>expand.user.js</code> source.)

## Supported sites

- https://github.com/ ("Load more" all the issue comments, expand minimized and similar comments, expand outdated review comments)
- https://www.youtube.com/ (expand video descriptions and comments, load more comments)
- https://stackoverflow.com/ and all other Stack Exchange sites (show all comments)
- https://\*.substack.com/ (dismiss subscribe overlay, expand comment text)
- https://www.goodreads.com/ (expand book review text)
- https://old.reddit.com/ (expand comments below threshold, expand deleted comments)
  - For "load more comments", you'll need [Reddit Enhancement Suite](https://github.com/honestbleeps/Reddit-Enhancement-Suite) and its "Never Ending Comments", "Load Child Comments" options.
- https://news.ycombinator.com/ (expand collapsed comments)
- https://www.quora.com/ (expand answer text)
- https://www.imdb.com/ (expand film review text, load more reviews)
- https://\*.linkedin.com/ (expand text in profile sections)
- https://www.lesswrong.com/ (expand collapsed comments)
- https://tvtropes.org/ (open all folders)
- https://cohost.org/ (expand posts on actual post pages)
- https://nextdoor.com/ (expand posts and comments)
- https://store.steampowered.com/ (expand review text)

For test pages, see the "Test page:" comments in expand.user.js.

## Demo

The video description and comment replies are expanded automatically.

<img src="https://user-images.githubusercontent.com/4458/246577842-20f194f1-05d2-421e-afbf-cb392c9c4a31.gif">
