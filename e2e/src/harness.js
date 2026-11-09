// The harness script, as a function that `Userscript::harness_script` (in
// userscript.rs) calls with:
//
// - `harness_global`: the name of the global through which the harness
//   reports to the tests.
// - `match_regexes`: the regexes (as strings) that the userscript's `@match`
//   lines become.
// - `userscript_source_url`: the `sourceURL` of the injected userscript, which
//   names the userscript's frames in stack traces.
(harness_global, match_regexes, userscript_source_url) => {
	if (window[harness_global]) {
		window[harness_global].injections++;
		return;
	}
	const url = location.href.split("#")[0];
	const harness = {
		injections: 1,
		matched: match_regexes.some((re) => new RegExp(re).test(url)),
		ran: false,
		error: null,
		clicks: [],
		observing: false,
		start: (run) => {
			if (!harness.matched) {
				return;
			}
			const start = () => {
				harness.ran = true;
				try {
					run();
				} catch (error) {
					harness.error = "an exception that can't be shown as text";
					try {
						harness.error = String(error?.stack ?? error);
					} catch {}
					throw error;
				}
			};
			if (document.readyState === "loading") {
				window.addEventListener("DOMContentLoaded", async () => {
					await 0;
					start();
				}, { once: true });
			} else {
				start();
			}
		},
	};
	window[harness_global] = harness;
	if (!harness.matched) {
		return;
	}
	// Whether the caller of the method that calls this is in the userscript.
	// (Frame 0 is this function, frame 1 the method.)
	const called_by_userscript = () => {
		const limit = Error.stackTraceLimit;
		Error.stackTraceLimit = 3;
		const frames = new Error().stack.split("\n").slice(1);
		Error.stackTraceLimit = limit;
		return frames.length === 3 && frames[2].includes(`${userscript_source_url}:`);
	};
	const native_click = HTMLElement.prototype.click;
	HTMLElement.prototype.click = function click() {
		if (called_by_userscript()) {
			const labelled_by = (this.getAttribute("aria-labelledby") ?? "")
				.split(/\s+/)
				.map((id) => document.getElementById(id)?.textContent ?? "")
				.join(" ");
			const label = this.getAttribute("aria-label") || labelled_by;
			const text = ((this.innerText || "").trim() || label.trim()).slice(0, 200).toWellFormed();
			harness.clicks.push({ element: this, text, time: performance.now() });
		}
		return native_click.call(this);
	};
	const native_observe = MutationObserver.prototype.observe;
	MutationObserver.prototype.observe = function observe(...args) {
		if (called_by_userscript()) {
			harness.observing = true;
		}
		return native_observe.apply(this, args);
	};
	const native_disconnect = MutationObserver.prototype.disconnect;
	MutationObserver.prototype.disconnect = function disconnect() {
		if (called_by_userscript()) {
			harness.observing = false;
		}
		return native_disconnect.call(this);
	};
}
