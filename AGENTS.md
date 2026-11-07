<!-- clank-right
languages = ["rust"]
-->

# Project overview

Expand Everything is a userscript, `expand.user.js`, that clicks the "show more" / "load more" / "read more" buttons on many sites, so that the whole page can be read, printed, or archived. People install it into a userscript manager (the user runs Violentmonkey in Firefox, Chromium, and Chrome) or paste it into a new script. `README.md` lists the supported sites.

# The userscript

- It must stay one dependency-free file that a userscript manager runs as-is: no build step, no imports, `@grant none`.
- Keep its existing style: 2-space indentation, camelCase names, semicolons.
- Each site has one block, guarded by a check of the URL or host, that calls `observe(maxMutations, selectors, callback)`. Above each block, `// Test page:` and `// Expected:` comments say how to check the block by hand; keep them correct when changing the block.
- A new site needs a `@match` line in the metadata block (without one, the userscript manager never runs the script there) and a line in the "Supported sites" list in `README.md`.
- The user releases a version by changing `@version` in a commit of its own, whose subject line is only the version (e.g. `3.2.0`). Don't change `@version` unless asked.

# Before committing

	node --check expand.user.js
