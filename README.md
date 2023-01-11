# Expand Everything

Expand Everything is a userscript that clicks on all the "show more" links
on a page, so that you don't have to click everything before you can read,
print, copy, or archive (e.g. SingleFile) the entire page.

## Install

1. Install [Violentmonkey](https://violentmonkey.github.io/) in any Chromium-based browser (this userscript uses the [navigate](https://developer.mozilla.org/en-US/docs/Web/API/Navigation/navigate_event) event).
2. Create a new userscript.
3. Copy the entire [expand.js source](https://raw.githubusercontent.com/ludios/expand-everything/master/expand.js) and paste it in.
4. Save.

## Supported sites

- https://github.com/ ("Load more" all the issue comments, expand minimized comments, expand outdated review comments)
- https://www.youtube.com/ (expand video descriptions and comments, load more comments)
- https://stackoverflow.com/ and all other Stack Exchange sites (show all comments)
- https://\*.substack.com/ (dismiss subscribe overlay, expand comment text)
- https://www.goodreads.com/ (expand book review text)
- https://old.reddit.com/ (expand comments below threshold, expand deleted comments)
  - For "load more comments", you'll need [Reddit Enhancement Suite](https://github.com/honestbleeps/Reddit-Enhancement-Suite) and its "Never Ending Comments", "Load Child Comments" options.
- https://news.ycombinator.com/ (expand collapsed comments)
- https://www.quora.com/ (expand answer text)
- https://www.imdb.com/ (expand film review text, load more reviews)
- https://www.linkedin.com/ (expand text in profile sections)
- https://www.lesswrong.com/ (expand collapsed comments)
- https://tvtropes.org/ (open all folders)

For test pages, see the "Test page:" comments in expand.js.
