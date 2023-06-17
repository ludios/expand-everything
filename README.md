# Expand Everything

Expand Everything is a userscript that clicks on all the "show more" links
on a page, so that you don't have to click everything before you can read,
print, copy, or archive (e.g. SingleFile) the entire page.

## Install

This is distributed as a userscript, so you need a browser extension that runs userscripts.

[Violentmonkey](https://violentmonkey.github.io/) can be installed in Firefox and Chromium-based browsers. [Userscripts](https://apps.apple.com/us/app/userscripts/id1463298887) can be used for Safari.

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
