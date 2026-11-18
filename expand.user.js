// ==UserScript==
// @name        Expand Everything
// @description Click the "show more" links to expand all the text on a page
// @namespace   ludios
// @author      ludios
// @license     AGPL-3.0; https://www.gnu.org/licenses/agpl-3.0.en.html
// @grant       none
// @version     5.1.0
//
// @match       https://www.goodreads.com/book/show/*
// @match       https://www.goodreads.com/review/list/*
// @match       https://www.imdb.com/title/*/reviews*
// @match       https://www.imdb.com/*/title/*/reviews*
// @match       https://www.youtube.com/*
// @match       https://*.linkedin.com/*
// @match       https://tvtropes.org/*
// @match       https://news.ycombinator.com/*
// @match       https://github.com/*
// @match       https://gist.github.com/*
// @match       https://www.quora.com/*
// @match       https://old.reddit.com/*
// @match       https://www.lesswrong.com/*
// @match       https://nextdoor.com/*
// @match       https://store.steampowered.com/*
// @match       https://www.patreon.com/*
// @match       https://twitter.com/*
// @match       https://x.com/*
// @match       https://www.google.com/search?*
// @match       https://www.nytimes.com/*
// @match       https://www.cnn.com/*/live-news/*
// @match       https://edition.cnn.com/*/live-news/*
// @match       https://www.bloomberg.com/*
// @match       https://www.criticker.com/*
// @match       https://claude.ai/chat/*
//
// Substack sites
// @match       https://substack.com/*
// @match       https://*.substack.com/*
// Substack publications on domains of their own. Most were chosen, in
// November 2026, because since the start of 2021 they had at least 3 Hacker
// News stories with 100+ points or at least 10 Reddit posts with 500+ points.
// @match       https://12gramsofcarbon.com/*
// @match       https://www.abortretry.fail/*
// @match       https://www.afterbabel.com/*
// @match       https://www.apricitas.io/*
// @match       https://www.asimov.press/*
// @match       https://www.astralcodexten.com/*
// @match       https://www.beltway.news/*
// @match       https://bigdata.2minutestreaming.com/*
// @match       https://www.bitecode.dev/*
// @match       https://bit.kevinslin.com/*
// @match       https://blog.calif.io/*
// @match       https://blog.codingconfessions.com/*
// @match       https://blog.coredump.cx/*
// @match       https://blog.eladgil.com/*
// @match       https://blog.jacobstechtavern.com/*
// @match       https://blog.jxmo.io/*
// @match       https://blog.nateliason.com/*
// @match       https://blog.sshh.io/*
// @match       https://blog.tjcx.me/*
// @match       https://www.bloodinthemachine.com/*
// @match       https://www.borowitzreport.com/*
// @match       https://bramcohen.com/*
// @match       https://www.bylinesupplement.com/*
// @match       https://www.chinatalk.media/*
// @match       https://chipsandcheese.com/*
// @match       https://www.chrisbrunet.com/*
// @match       https://www.computerenhance.com/*
// @match       https://www.construction-physics.com/*
// @match       https://contraptions.venkateshrao.com/*
// @match       https://davidoks.blog/*
// @match       https://www.deadlanguagesociety.com/*
// @match       https://www.deepcutnews.com/*
// @match       https://www.derekthompson.org/*
// @match       https://www.deusinmachina.net/*
// @match       https://www.dgtlgrove.com/*
// @match       https://www.distilled.earth/*
// @match       https://dkb.blog/*
// @match       https://www.dropsitenews.com/*
// @match       https://www.ducktyped.org/*
// @match       https://www.dwarkesh.com/*
// @match       https://www.elidourado.com/*
// @match       https://www.erininthemorning.com/*
// @match       https://www.experimental-history.com/*
// @match       https://www.fabricatedknowledge.com/*
// @match       https://www.friendlyatheist.com/*
// @match       https://gabrielweinberg.com/*
// @match       https://www.gamefile.news/*
// @match       https://www.gelliottmorris.com/*
// @match       https://www.goto10retro.com/*
// @match       https://www.governance.fyi/*
// @match       https://www.gurwinder.blog/*
// @match       https://www.hamiltonnolan.com/*
// @match       https://hardcoresoftware.learningbyshipping.com/*
// @match       https://hartmannreport.com/*
// @match       https://heated.world/*
// @match       https://www.henrikkarlsson.xyz/*
// @match       https://www.honest-broker.com/*
// @match       https://www.interconnects.ai/*
// @match       https://www.joanwestenberg.com/*
// @match       https://www.joewrote.com/*
// @match       https://www.johncanzano.com/*
// @match       https://www.kenklippenstein.com/*
// @match       https://www.landmark.earth/*
// @match       https://lastweekin.ai/*
// @match       https://www.lawdork.com/*
// @match       https://www.leefang.com/*
// @match       https://www.lennysnewsletter.com/*
// @match       https://www.lonestarleft.com/*
// @match       https://magazine.sebastianraschka.com/*
// @match       https://materializedview.io/*
// @match       https://www.md-a.co/*
// @match       https://www.meidasplus.com/*
// @match       https://www.michaelmoore.com/*
// @match       https://migrantinsider.com/*
// @match       https://www.muellershewrote.com/*
// @match       https://www.narativ.org/*
// @match       https://www.natesilver.net/*
// @match       https://news.alvaroduran.com/*
// @match       https://newsletter.eng-leadership.com/*
// @match       https://newsletter.pessimistsarchive.org/*
// @match       https://newsletter.posthog.com/*
// @match       https://newsletter.pragmaticengineer.com/*
// @match       https://news.lettersofnote.com/*
// @match       https://www.noahpinion.blog/*
// @match       https://www.normaltech.ai/*
// @match       https://www.notesfromthecircus.com/*
// @match       https://www.oneusefulthing.org/*
// @match       https://www.ourfairfuture.org/*
// @match       https://www.owlposting.com/*
// @match       https://www.persuasion.community/*
// @match       https://popular.info/*
// @match       https://www.privatdozent.co/*
// @match       https://www.psychiatrymargins.com/*
// @match       https://www.publicnotice.co/*
// @match       https://www.qasimrashid.com/*
// @match       https://read.engineerscodex.com/*
// @match       https://www.readmargins.com/*
// @match       https://www.readtpa.com/*
// @match       https://registerspill.thorstenball.com/*
// @match       https://www.robkhenderson.com/*
// @match       https://www.slowboring.com/*
// @match       https://www.statsignificant.com/*
// @match       https://stephenfollows.com/*
// @match       https://www.supervised.news/*
// @match       https://www.surviving-tomorrow.com/*
// @match       https://takes.jamesomalley.co.uk/*
// @match       https://www.techemails.com/*
// @match       https://www.thealgorithmicbridge.com/*
// @match       https://www.theamericansaga.com/*
// @match       https://www.theargumentmag.com/*
// @match       https://www.thebignewsletter.com/*
// @match       https://www.thebulwark.com/*
// @match       https://www.the-downballot.com/*
// @match       https://www.thefp.com/*
// @match       https://www.thegamebusiness.com/*
// @match       https://www.theintrinsicperspective.com/*
// @match       https://www.thelettersfromleo.com/*
// @match       https://www.thenorthstar.com/*
// @match       https://www.theseedsofscience.pub/*
// @match       https://www.theunpopulist.net/*
// @match       https://thoughts.wyounas.com/*
// @match       https://www.tracingwoodgrains.com/*
// @match       https://www.transiticsnews.com/*
// @match       https://unchartedterritories.tomaspueyo.com/*
// @match       https://www.unclosetedmedia.com/*
// @match       https://www.understandingai.org/*
// @match       https://www.usermag.co/*
// @match       https://www.wonkette.com/*
// @match       https://www.worksinprogress.news/*
// @match       https://www.writingruxandrabio.com/*
// @match       https://www.zachdaniel.dev/*
// @match       https://zeteo.com/*
//
// Mastodon sites
// @match       https://akademienl.social/*
// @match       https://archaeo.social/*
// @match       https://aus.social/*
// @match       https://bildung.social/*
// @match       https://c.im/*
// @match       https://chaos.social/*
// @match       https://cyberplace.social/*
// @match       https://dair-community.social/*
// @match       https://datasci.social/*
// @match       https://defcon.social/*
// @match       https://dice.camp/*
// @match       https://discuss.systems/*
// @match       https://ecoevo.social/*
// @match       https://econtwitter.net/*
// @match       https://en.osm.town/*
// @match       https://fediscience.org/*
// @match       https://floss.social/*
// @match       https://fosstodon.org/*
// @match       https://front-end.social/*
// @match       https://functional.cafe/*
// @match       https://gamedev.lgbt/*
// @match       https://genomic.social/*
// @match       https://graphics.social/*
// @match       https://h-net.social/*
// @match       https://hachyderm.io/*
// @match       https://hcommons.social/*
// @match       https://indieweb.social/*
// @match       https://infosec.exchange/*
// @match       https://ioc.exchange/*
// @match       https://kde.social/*
// @match       https://linuxrocks.online/*
// @match       https://mapstodon.space/*
// @match       https://mas.to/*
// @match       https://masto.ai/*
// @match       https://mastodon.acm.org/*
// @match       https://mastodon.archive.org/*
// @match       https://mastodon.art/*
// @match       https://mastodon.bsd.cafe/*
// @match       https://mastodon.design/*
// @match       https://mastodon.energy/*
// @match       https://mastodon.gamedev.place/*
// @match       https://mastodon.green/*
// @match       https://mastodon.mit.edu/*
// @match       https://mastodon.online/*
// @match       https://mastodon.radio/*
// @match       https://mastodon.scot/*
// @match       https://mastodon.sdf.org/*
// @match       https://mastodon.social/*
// @match       https://mastodon.world/*
// @match       https://mastodonapp.uk/*
// @match       https://mathstodon.xyz/*
// @match       https://merveilles.town/*
// @match       https://mograph.social/*
// @match       https://mstdn.ca/*
// @match       https://mstdn.party/*
// @match       https://mstdn.science/*
// @match       https://mstdn.social/*
// @match       https://musician.social/*
// @match       https://nerdculture.de/*
// @match       https://neuromatch.social/*
// @match       https://ohai.social/*
// @match       https://oldbytes.space/*
// @match       https://openbiblio.social/*
// @match       https://peoplemaking.games/*
// @match       https://photog.social/*
// @match       https://phpc.social/*
// @match       https://post.lurk.org/*
// @match       https://ravenation.club/*
// @match       https://ruby.social/*
// @match       https://sauropods.win/*
// @match       https://scholar.social/*
// @match       https://scicomm.xyz/*
// @match       https://sciences.social/*
// @match       https://sfba.social/*
// @match       https://sigmoid.social/*
// @match       https://social.coop/*
// @match       https://social.edu.nl/*
// @match       https://social.linux.pizza/*
// @match       https://social.sciences.re/*
// @match       https://social.tchncs.de/*
// @match       https://social.treehouse.systems/*
// @match       https://social.vivaldi.net/*
// @match       https://spore.social/*
// @match       https://sunny.garden/*
// @match       https://tech.lgbt/*
// @match       https://techhub.social/*
// @match       https://tilde.zone/*
// @match       https://toot.cafe/*
// @match       https://types.pl/*
// @match       https://typo.social/*
// @match       https://ubuntu.social/*
// @match       https://universeodon.com/*
// @match       https://urbanists.social/*
// @match       https://vis.social/*
// @match       https://w3c.social/*
// @match       https://wandering.shop/*
// @match       https://wikimedia.social/*
// @match       https://writing.exchange/*
// @match       https://zirk.us/*
//
// XenForo 2 sites: technical, artistic, and scientific forums, chosen in
// November 2026
// @match       https://68kmla.org/bb/*
// @match       https://absolutewrite.com/forums/*
// @match       https://arstechnica.com/civis/*
// @match       https://audiokarma.org/forums/*
// @match       https://bcgforums.com/*
// @match       https://cafesaxophone.com/*
// @match       https://cameraderie.org/*
// @match       https://canonrumorsforum.com/*
// @match       https://chandoo.org/forum/*
// @match       https://clubsnap.com/*
// @match       https://codeforum.org/*
// @match       https://community.preproom.org/*
// @match       https://creativespark.art/*
// @match       https://designforums.net/*
// @match       https://diysolarforum.com/*
// @match       https://endless-sphere.com/sphere/*
// @match       https://engraverscafe.com/*
// @match       https://forum.allaboutcircuits.com/*
// @match       https://forum.gamemaker.io/*
// @match       https://forum.pedalpcb.com/*
// @match       https://forum.pjrc.com/*
// @match       https://forum.proxmox.com/*
// @match       https://forum.vcfed.org/*
// @match       https://forum.wordreference.com/*
// @match       https://forums.anandtech.com/*
// @match       https://forums.androidcentral.com/*
// @match       https://forums.freebsd.org/*
// @match       https://forums.macrumors.com/*
// @match       https://forums.mikeholt.com/*
// @match       https://forums.overclockers.co.uk/*
// @match       https://forums.overclockers.com.au/*
// @match       https://forums.rpgmakerweb.com/*
// @match       https://forums.servethehome.com/*
// @match       https://forums.spacebattles.com/*
// @match       https://forums.studentdoctor.net/*
// @match       https://forums.sufficientvelocity.com/*
// @match       https://forums.tomsguide.com/*
// @match       https://forums.tomshardware.com/*
// @match       https://forums.whathifi.com/*
// @match       https://forums.windowscentral.com/*
// @match       https://hardforum.com/*
// @match       https://homerecording.com/bbs/*
// @match       https://*.largeformatphotography.info/forum/*
// @match       https://latindiscussion.org/*
// @match       https://malwaretips.com/*
// @match       https://mathforums.com/*
// @match       https://mathhelpforum.com/*
// @match       https://*.mythicscribes.com/community/*
// @match       https://nikonites.com/forum/*
// @match       https://obsproject.com/forum/*
// @match       https://oscraps.com/community/*
// @match       https://pinkfishmedia.net/forum/*
// @match       https://pixinsight.com/forum/*
// @match       https://*.rangefinderforum.com/*
// @match       https://reduser.net/*
// @match       https://sawmillcreek.org/*
// @match       https://secondlifestorage.com/*
// @match       https://talkweather.com/*
// @match       https://thegearforum.com/*
// @match       https://thetolkien.forum/*
// @match       https://tinkerdifferent.com/*
// @match       https://tonmo.com/*
// @match       https://vi-control.net/community/*
// @match       https://writingforum.org/*
// @match       https://www.access-programmers.co.uk/forums/*
// @match       https://www.alternatehistory.com/forum/*
// @match       https://www.amibay.com/*
// @match       https://www.audiosciencereview.com/forum/*
// @match       https://www.avforums.com/*
// @match       https://www.avsforum.com/*
// @match       https://www.birdforum.net/*
// @match       https://www.candlepowerforums.com/*
// @match       https://www.computerbase.de/forum/*
// @match       https://www.controlbooth.com/*
// @match       https://www.dance-forums.com/*
// @match       https://www.dissensus.com/*
// @match       https://www.diyaudio.com/community/*
// @match       https://www.dpreview.com/forums/*
// @match       https://www.dvxuser.com/*
// @match       https://www.edaboard.com/*
// @match       https://www.electro-tech-online.com/*
// @match       https://www.elevenforum.com/*
// @match       https://www.eng-tips.com/*
// @match       https://www.filmschool.org/*
// @match       https://www.fujix-forum.com/*
// @match       https://www.futureproducers.com/forums/*
// @match       https://www.hardwareluxx.de/community/*
// @match       https://www.head-fi.org/*
// @match       https://www.hobby-machinist.com/*
// @match       https://www.homemodelenginemachinist.com/*
// @match       https://www.hometheaterforum.com/community/*
// @match       https://www.hwinfo.com/forum/*
// @match       https://www.keyboardforums.com/*
// @match       https://www.lightroomqueen.com/community/*
// @match       https://www.linux.org/*
// @match       https://www.metabunk.org/*
// @match       https://www.mu-43.com/*
// @match       https://www.nikoncafe.com/*
// @match       https://www.orbiter-forum.com/*
// @match       https://www.overclock.net/*
// @match       https://www.overclockers.com/forums/*
// @match       https://www.pcreview.co.uk/*
// @match       https://www.penturners.org/*
// @match       https://www.practicalmachinist.com/forum/*
// @match       https://www.sciforums.com/*
// @match       https://www.sevenstring.org/*
// @match       https://www.sffchronicles.com/*
// @match       https://www.sffworld.com/forum/*
// @match       https://www.signs101.com/*
// @match       https://www.snbforums.com/*
// @match       https://www.studentfilms.com/*
// @match       https://www.synoforum.com/*
// @match       https://www.sysnative.com/forums/*
// @match       https://www.talkbass.com/*
// @match       https://www.talkclassical.com/*
// @match       https://www.talkphotography.co.uk/*
// @match       https://www.tascamforums.com/*
// @match       https://www.techguy.org/*
// @match       https://www.techpowerup.com/forums/*
// @match       https://www.techspot.com/community/*
// @match       https://www.tek-tips.com/*
// @match       https://www.thecodingforums.com/*
// @match       https://www.thegearpage.net/board/*
// @match       https://www.thephotoforum.com/*
// @match       https://www.truenas.com/community/*
// @match       https://www.ukworkshop.co.uk/*
// @match       https://www.worldliteratureforum.com/forum/*
// @match       https://www.writingforums.com/*
// @match       https://xdaforums.com/*
// @match       https://xenforo.com/community/*
//
// Stack Exchange sites, their metas (e.g. meta.superuser.com), and their other subdomains (e.g. chat.stackexchange.com)
// @match       https://*.askubuntu.com/*
// @match       https://*.mathoverflow.net/*
// @match       https://*.serverfault.com/*
// @match       https://stackapps.com/*
// @match       https://*.stackexchange.com/*
// @match       https://*.stackoverflow.com/*
// @match       https://*.superuser.com/*
//
// ==/UserScript==

const loc = window.location.href;

function query_elements(selector, callback) {
	const elements = document.querySelectorAll(selector);
	elements.forEach(element => callback(element));
}

// `url` without its #fragment (if it has one)
function without_fragment(url) {
	return url.split("#")[0];
}

const log_prefix = "Expand Everything: ";
let mutations = 0;
let observer = null;
// The function that passes every element that the selectors given to
// observe(...) select to the callback given to observe(...), or null before
// observe(...) is called
let look_at_all = null;
// The ID of the pending timeout that will call look_at_all (see
// look_again_unless), or null if no such timeout is pending
let look_again_timer = null;
// How many times look_again_unless has called look_at_all since the page loaded
// or navigated
let look_agains = 0;
// location.href without its #fragment, as last seen by the polling mechanism
let last_href = without_fragment(location.href);
let location_interval = null;

// Observe some selectors and run a callback for each selected element.
function observe(max_mutations, selectors, callback) {
	if (observer !== null) {
		throw new Error(`observe(...) called more than once`);
	}

	look_at_all = () => {
		for (const selector of selectors) {
			query_elements(selector, callback);
		}
	};

	observer = new MutationObserver(() => {
		mutations++;
		if (mutations >= max_mutations) {
			console.log(`${log_prefix}disconnecting MutationObserver after ${mutations} mutations to avoid slowing down the page`);
			observer.disconnect();
		}
		look_at_all();
	});

	function reobserve() {
		// Process elements that were present before MutationObserver
		look_at_all();

		// Start observing
		observer.observe(document.documentElement, {
			childList: true,
			subtree: true,
		});
	}

	reobserve();

	function navigated() {
		console.log(`${log_prefix}navigated, resetting already_clicked and MutationObserver`);
		observer.disconnect();
		reset_already_clicked();
		mutations = 0;
		clearTimeout(look_again_timer);
		look_again_timer = null;
		look_agains = 0;
		reobserve();
	}

	function start_polling_location() {
		if (location_interval !== null) {
			throw new Error(`location_interval already exists: ${location_interval}`);
		}
		location_interval = setInterval(
			() => {
				const href = without_fragment(location.href);
				if (href !== last_href) {
					last_href = href;
					navigated();
				}
			},
			1000
		);
	}

	function stop_polling_location() {
		if (location_interval === null) {
			throw new Error(`location_interval === null`);
		}
		clearInterval(location_interval);
		location_interval = null;
	}

	if (window.navigation && navigation.addEventListener) {
		console.log(`${log_prefix}using Navigation API to detect location changes`);
		navigation.addEventListener('navigate', (ev) => {
			// We must ignore navigation events to the same location, or else we hang in Firefox as it loops infinitely on
			// https://superuser.com/questions/681575/any-way-to-get-list-of-functions-defined-in-zsh-like-alias-command-for-aliases
			//
			// Possibly related to https://bugzilla.mozilla.org/show_bug.cgi?id=1997591
			//
			// We also ignore navigation events to another #fragment of the page.
			if (without_fragment(ev.destination.url) === without_fragment(location.href)) {
				console.log(`${log_prefix}ignoring navigation event to the same page`);
				return;
			}
			navigated();
		});
	} else {
		// https://stackoverflow.com/questions/34999976/detect-changes-on-the-url
		// suggests that setInterval is the best way to do it when we lack Navigation API.
		console.log(`${log_prefix}using setInterval(..., 1000) to detect location changes`);
		if (!document.hidden) {
			start_polling_location();
		}
		// Not necessary for correctness, but to save power, kill our setInterval when the page isn't visible.
		document.addEventListener("visibilitychange", function() {
			if (document.hidden) {
				console.log(`${log_prefix}page has become hidden, stopping setInterval`);
				stop_polling_location();
			} else {
				console.log(`${log_prefix}page has become visible, starting setInterval`);
				start_polling_location();
			}
		});
	}
}

let already_clicked;
function reset_already_clicked() {
	already_clicked = new WeakMap();
}
reset_already_clicked();

// Click on something if it hasn't already been clicked.
function click_if_unclicked(el) {
	if (already_clicked.get(el)) {
		return;
	}
	already_clicked.set(el, true);
	el.click();
}

// If `ready` is false, as for an element that the page's scripts would
// ignore a click on yet, pass every selected element to the callback of
// observe(...) again in 500 ms, because the page's scripts may get ready
// without changing the page (so the MutationObserver wouldn't report it).
// This looks again at most 120 times between navigations, so that an element
// that never gets ready doesn't make the userscript look forever.
// observe(...) must have been called.
//
// Returns `ready`.
function look_again_unless(ready) {
	if (look_at_all === null) {
		throw new Error(`look_again_unless(...) called before observe(...)`);
	}
	if (!ready && look_agains < 120) {
		look_again_timer ??= setTimeout(() => {
			look_again_timer = null;
			look_agains++;
			if (look_agains === 120) {
				console.log(`${log_prefix}looked again ${look_agains} times; not looking again until the next navigation`);
			}
			look_at_all();
		}, 500);
	}
	return ready;
}

// Whether React has hydrated or created `el` (an element that React
// renders), as shown by the props that React keeps on the element. React may
// ignore a click on an element that it hasn't hydrated yet.
function react_hydrated(el) {
	// Where a page's Content Security Policy keeps a userscript out of the
	// page, a userscript manager may run it as a content script (as
	// Violentmonkey does by default in Firefox), which sees the page's own
	// properties of `el` only through `el.wrappedJSObject`.
	return Object.keys(el.wrappedJSObject ?? el).some(key => key.startsWith("__reactProps$"));
}

// Test page: https://www.goodreads.com/book/show/931984.The_Presentation_of_Self_in_Everyday_Life
// Expected: the text about the author and all the lengthy user book reviews are expanded
//
// Test page: https://www.goodreads.com/book/show/39736150-the-little-typer
// Expected: the book description and all the lengthy user book reviews are expanded
//
// Test page: https://www.goodreads.com/book/show/7260188-mockingjay
// Expected: all the reviews hidden because of spoilers are shown, and the lengthy ones are expanded
// Expected: the passages of reviews that are marked as spoilers are not blurred
// Expected: the page doesn't scroll down to the expanded texts
if (loc.startsWith("https://www.goodreads.com/book/show/")) {
	// "Show full review" on a review hidden because of spoilers. The server
	// renders it, so React may not have hydrated it yet.
	const show_full_review = 'button[aria-label="Activate this button to show the hidden review."]';
	observe(1000, [
		'button[aria-label="Tap to show more review"]',
		'button[aria-label="Tap to show more about the author"]',
		'button[aria-label="Tap to show more book description"]',
		show_full_review,
		// A passage of a review that is marked as a spoiler, which is blurred until this is checked
		'.Formatted input.spoiler__control[type="checkbox"]:not(:checked)',
	], el => {
		if (el.matches(show_full_review) && !look_again_unless(react_hydrated(el))) {
			return;
		}
		// Goodreads focuses each text that it expands, which scrolls the page to the text.
		// Without a tabindex, the text can't be focused.
		const text = el.closest(".TruncatedContent")?.querySelector(':scope > [data-testid="contentContainer"]');
		text?.removeAttribute("tabindex");
		click_if_unclicked(el);
	});
}

// Test page (logged in): https://www.goodreads.com/review/list/35831082?utf8=%E2%9C%93&order=d&sort=review&view=reviews&per_page=infinite
// Expected: all the lengthy user book reviews are expanded
if (loc.startsWith("https://www.goodreads.com/review/list/")) {
	observe(100, [
		'a[data-text-id^="review"][href="#"][onclick="swapContent($(this));; return false;"]',
	], el => {
		click_if_unclicked(el);
	});
}

// Test page: https://www.imdb.com/title/tt0809535/reviews/?featured=rw2027839
// Expected: the review is expanded
// Expected: after making the window much narrower, the review is still not cut off
//
// Test page: https://www.imdb.com/de/title/tt0809535/reviews/?featured=rw2027839
// Expected: the review is expanded
//
// Test page (logged in): https://www.imdb.com/title/tt0809535/reviews/
// Expected: all the lengthy user film reviews are expanded
// Expected: all the reviews hidden because of spoilers are shown
//
// Test page (logged in): https://www.imdb.com/title/tt6710474/reviews/
// Expected: "25 more" is clicked until the page has 150 reviews, and not after that; of those reviews,
//           the lengthy ones are expanded and the ones hidden because of spoilers are shown
//
// (We don't want to click "25 more" so many times that we slow down
// the page and also risk getting banned.)
//
// The path may start with a language, as in /de/title/.
if (/^https:\/\/www\.imdb\.com\/([^\/]+\/)?title\//.test(loc)) {
	// "25 more" below the reviews
	const more_reviews = '[data-testid="tturv-pagination"] .single-page-see-more-button > button';
	// A review's text that IMDb has measured or expanded, and has limited to the height that
	// it had then, which cuts the text off once it gets taller (as it does in a narrower window)
	const height_limited = '[data-testid="review-overflow"][style*="max-height"]';
	observe(1000, [
		// "Spoiler" in place of a review with spoilers
		'button.review-spoiler-button',
		// "See more" over the end of a lengthy review
		'[data-testid="review-overflow"] > button.ipc-overflowText-overlay',
		more_reviews,
		height_limited,
	], el => {
		if (el.matches(height_limited)) {
			el.style.maxHeight = "none";
			return;
		}
		// React may not have hydrated a button that the server rendered, and
		// IMDb disables "25 more" while reviews are loading.
		if (!look_again_unless(react_hydrated(el) && !el.disabled)) {
			return;
		}
		if (!el.matches(more_reviews)) {
			click_if_unclicked(el);
			return;
		}
		const reviews = document.querySelectorAll("article.user-review-item");
		const last_review = reviews[reviews.length - 1];
		// IMDb may disable "25 more" only some time after a click, so click
		// it at most once while the same element is the last review.
		if (last_review && reviews.length < 150 && !already_clicked.get(last_review)) {
			already_clicked.set(last_review, true);
			console.log(`${log_prefix}clicking "25 more" below ${reviews.length} reviews`);
			el.click();
		}
	});
}

// Test page: https://www.youtube.com/watch?v=ben9qDbrLYU
// Test page: https://www.youtube.com/ followed by click on a video
// Test page: https://www.youtube.com/watch?v=ben9qDbrLYU followed by scrolling to the comments
//            (so that the MutationObserver is disconnected) and a click on a related video
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
		// We can't use click_if_unclicked due to YouTube oddities after navigating the SPA (the video
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

// Test page: https://substack.com/@meaningness/note/c-59538921
// Expected: all comments are expanded; "See more" is not visible
if (loc.startsWith("https://substack.com/")) {
	observe(100, [
		// "See more"
		'div.pencraft[class*=seeMoreText-] > span.pencraft > a.pencraft',
	], el => {
		if (el.innerText === "See more") {
			el.click();
		}
	});
}

// Test page: https://tvtropes.org/pmwiki/pmwiki.php/OvershadowedByControversy/WebOriginal
// Expected: all folders are opened
//
// Test page: https://tvtropes.org/pmwiki/pmwiki.php/OvershadowedByControversy/WebOriginal
// Expected: the text of every note is shown
//
// Test page: https://tvtropes.org/pmwiki/pmwiki.php/Franchise/Godzilla (which has two "open/close all folders")
// Expected: all folders are opened
//
// Test page: https://tvtropes.org/pmwiki/pmwiki.php/Creator/AgathaChristie (which has no "open/close all folders")
// Expected: all folders are opened
if (loc.startsWith("https://tvtropes.org/")) {
	const note = 'span.inlinefolder[isnote="true"]';
	observe(10, [
		// "open/close all folders" and the label of each folder
		'.folderlabel:not(.is-open)',
		// The text of each note
		note,
	], el => {
		if (el.matches(note)) {
			// A click on the note's label would also toggle a spoiler that the label is in
			el.style.display = "inline";
		} else if (!el.classList.contains("is-open")) {
			// An earlier click on "open/close all folders" may have opened this folder
			click_if_unclicked(el);
		}
	});
}

// Test page: https://news.ycombinator.com/item?id=34282033 as a logged-in user with showdead enabled
// Expected: all comments are shown; no '[N more]' links visible
//
// Test page: https://news.ycombinator.com/threads?id=huppeldepup (or 'next' to the page with the collapsed 'I used to not worry about climate change. Now I do...')
// Expected: all comments are shown; no '[N more]' links visible
if (loc.startsWith("https://news.ycombinator.com/")) {
	// No need for MutationObserver
	query_elements('a.togg.clicky', el => {
		if (el.innerText.endsWith(" more]")) {
			el.click();
		}
	});
}

// Test page: https://github.com/rust-lang/rust/pull/95035
// Expected: all comments are loaded; "N hidden items" button is not visible; "1 similar comment" is not visible
// Expected (logged in): off-topic comments are expanded; "Show comment" is not visible
//
// Test page: https://github.com/rust-lang/rust/issues/57640#issuecomment-456702793
// Expected: all comments are loaded; "N remaining items" and "Load more" are not visible above or below the linked comment
//
// Test page: https://github.com/JustAnotherArchivist/snscrape/issues/634
// Expected: spam/off-topic comments are expanded
//
// Test page: https://github.com/NixOS/nixpkgs/pull/194310
// Expected: outdated and resolved review comments are expanded
//
// Test page: https://github.com/A
// Expected: activity is not expanded; URL does not start changing
//
// Test page: https://github.com/PolyMC/PolyMC/commit/ccf282593dcdbe189c99b81b8bc90cb203aed3ee
// Expected: all comments are loaded; "Load more comments" button is not visible
//
// Test page: https://github.com/orgs/community/discussions/12032
// Expected: all comments and replies are loaded; "N hidden items" and "Show N previous replies" are not visible
//
// Test page: https://github.com/rust-lang/rust/issues/165052
// Expected: every <details> ("Program output", "bisected with cargo-bisect-rustc") is open
// Expected: after the user closes a <details> and then follows a link to a #fragment, the <details> stays closed
//
// Test page: https://github.com/khuedoan/nixie/blob/68b69f1247c09ae1a7aecff0a2132b39953044e3/README.md
// Expected: the Mermaid diagram is not opened in a modal dialog
if (loc.startsWith("https://github.com/")) {
	// The icon buttons in the headers of comments on an issue page, of which
	// only the one whose tooltip says "show comment" (on a minimized comment)
	// is clicked
	const comment_header_buttons = '[data-testid="comment-header"] button[aria-labelledby]';
	// Elements that React renders
	const react_selectors = [
		// "Load more" below "N remaining items" on an issue page (above the
		// linked comment, and also below it)
		'button[data-testid^="issue-timeline-load-more-load-"]',
		comment_header_buttons,
		// "Load more comments" on a commit page
		'#comments.react-comments-container > button',
	];
	// The chevron of a review thread on a pull request page, which is clicked
	// only if the thread is collapsed (as an outdated or resolved one is)
	const thread_chevrons = 'review-thread-collapsible button.review-thread-chevron';
	// The <summary> of a <details> in a comment, a README, or other Markdown,
	// which is clicked only if the <details> is closed, but not one that opens
	// an overlay (such as the modal dialog that shows a Mermaid diagram)
	const markdown_summaries = '.markdown-body details:not(.details-overlay) > summary';
	observe(1000, [
		...react_selectors,
		// "N hidden items; Load more..." on a pull request page or a discussion page
		//
		// We don't want all button.ajax-pagination-btn because clicking button.ajax-pagination-btn
		// on a profile page e.g. https://github.com/A causes the URL to start changing.
		'#js-progressive-timeline-item-container button.ajax-pagination-btn',
		// "N similar comments"
		'details:not([open]) > summary.pagination-loader-container > .Details-content--closed',
		// "Show comment" on a minimized comment on a pull request page
		'div.minimized-comment > details:not([open]) > summary > div > .Details-content--closed',
		thread_chevrons,
		// "Show N previous replies" on a discussion page
		'.discussion-nested-comment-paging-form button[type="submit"]',
		markdown_summaries,
		// "Load diff" button on PRs, where some diffs are collapsed because
		// "Large diffs are not rendered by default" and "This file was deleted".
		// This is disabled by default because some pages are just too large, e.g.
		// https://github.com/sveltejs/svelte/commit/fe8a9ce31d5fb662b3953b318621ea364992e014
		//'button[data-view-component="true"].load-diff-button',
	], el => {
		if (already_clicked.get(el)) {
			return;
		}
		// Mark an element that is not to be clicked, so that the userscript
		// doesn't click it after the user collapses something
		if (
			(el.matches(comment_header_buttons) && document.getElementById(el.getAttribute("aria-labelledby"))?.textContent !== "show comment") ||
			(el.matches(markdown_summaries) && el.parentElement.open)
		) {
			already_clicked.set(el, true);
			return;
		}
		if (react_selectors.some(selector => el.matches(selector)) && !look_again_unless(react_hydrated(el))) {
			return;
		}
		if (el.matches(thread_chevrons)) {
			const thread = el.closest("review-thread-collapsible");
			if (!thread.classList.contains("open")) {
				el.click();
			}
			// GitHub ignores a click on a chevron until a script that it loads late
			// handles the click, which doesn't change the page, so look again if
			// the thread didn't open.
			if (look_again_unless(thread.classList.contains("open"))) {
				already_clicked.set(el, true);
			}
			return;
		}
		click_if_unclicked(el);
	});
}

// Test page: https://gist.github.com/ivan/5095670735ba941a6090a69fce4183df
// Expected: all comments are loaded; "Load earlier comments..." is not visible
// Expected: when restarting Chrome, tab is not redirected to a /load_comments page
if (loc.startsWith("https://gist.github.com/")) {
	const observe_comments = () => {
		observe(5000, [
			// "Load earlier comments..."
			'form.ajax-pagination-form.js-ajax-pagination[action$="/load_comments"] > button.ajax-pagination-btn'
		], el => {
			click_if_unclicked(el);
		});
	};
	// Wait for the JavaScript to load, because we don't want a button click to navigate us to /load_comments
	if (document.readyState === "complete") {
		observe_comments();
	} else {
		window.addEventListener("load", observe_comments, { once: true });
	}
}

// Test page: https://stackoverflow.com/questions/59156473/what-is-the-difference-between-async-move-and-async-move
// Expected: the accepted answer shows as many comments as its "N Comments" heading says
//
// Test page: https://meta.stackexchange.com/questions/333965/firing-mods-and-forced-relicensing-is-stack-exchange-still-interested-in-cooper?rq=1
// Expected: all comments are loaded; "Show N more comments" is not visible
//
// Test page: https://meta.stackexchange.com/questions/277369/a-terms-of-service-update-restricting-companies-that-scrape-your-profile-informa?rq=1#
// Expected: all comments are loaded; "Show N more comments" is not visible
//
// Test page: https://astronomy.stackexchange.com/questions/51544/is-the-solar-core-hard#
// Expected: all comments are loaded; "Show N more comments" is not visible
//
// Test page: https://superuser.com/questions/681575/any-way-to-get-list-of-functions-defined-in-zsh-like-alias-command-for-aliases
// Expected: all comments are loaded; the page / Firefox does not hang.
// Expected: the URL doesn't get a "#" at its end
// Expected: the page requests /posts/POST_ID/comments only once for each "Show N more comments"
if (window.StackExchange?.ready) {
	// Until the page's scripts are ready, a click on "Show N more comments" only navigates to "#".
	StackExchange.ready(() => {
		observe(100, [
			// "Show N more comments", which has the class "dno" while the page hides it
			'a.js-show-link',
			// The "Show all comments" button beside an answer's "N Comments" heading on stackoverflow.com
			'[id^="follow-ups-container-"] button[aria-label="Show all comments"]',
		], el => {
			if (el.classList.contains("dno")) {
				// The page shows the same link again if more comments arrive.
				already_clicked.delete(el);
			} else {
				// A second click on the link would request the comments again,
				// and a second click on the button would hide every comment.
				click_if_unclicked(el);
			}
		});
	});
}

// Test page: https://www.quora.com/Which-of-the-big-four-Google-Microsoft-Amazon-Facebook-tech-companies-have-the-most-selective-hiring-process-for-software-engineers?share=1
// Expected: answers are fully visible; "Continue Reading" is not visible
if (loc.startsWith("https://www.quora.com/")) {
	observe(100, [
		// "Continue Reading"
		'.puppeteer_test_read_more_button',
	], el => {
		click_if_unclicked(el);
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
			click_if_unclicked(el);
		}
	});
}

// Test page: https://www.lesswrong.com/posts/uKp6tBFStnsvrot5t/what-dall-e-2-can-and-cannot-do
// Expected: all comments expanded; no one-line previews; no "(read more)"
//
// Test page: https://www.lesswrong.com/users/blueiris2
// Expected: all downvoted comments are expanded; no [+]
//
// Test page: https://www.lesswrong.com/users/valentine
// Expected: all the user's posts are listed; "Load More" below them is not visible; the bio is expanded, above "See less"
if (loc.startsWith("https://www.lesswrong.com/")) {
	// For each selector, the text that a selected element must have to be
	// clicked, or null if any text will do. An element with some other text,
	// such as the "-" in the [-] of an expanded comment, is marked as already
	// clicked: if the user collapses the comment, the userscript leaves the
	// comment collapsed.
	const text_to_click = {
		// Comments truncated to a single line (which show no text if they have
		// a score of -5 or lower)
		'.SingleLineComment-root': null,
		// "(read more)" at the end of a truncated comment
		'.CommentBody-root .read-more-button': null,
		// The "+" in the [+] of a collapsed comment (such as a downvoted one), or
		// the "-" in the [-] of an expanded comment
		'span.CommentsItemMeta-collapseCharacter': "+",
		// On a /users/ page, "See more" or "See less" below the user's bio, or
		// another link styled like them. ("Show all" is also styled like them,
		// but it is in the row of 'a.LoadMore-root', which clicks any text.)
		'a.ProfilePage-readMoreLink:not(.LoadMore-root)': "See more",
		// On a /users/ page, "Load More" below the posts, and "Show all" below
		// the grids of squares that stand for the posts and comments
		'a.LoadMore-root': null,
	};
	const selectors = Object.keys(text_to_click);

	// Click `el` (a selected element) if it should be clicked now. Mark it in
	// `already_clicked` once it is clicked or should never be clicked.
	function expand(el) {
		if (already_clicked.get(el)) {
			return;
		}
		if (el.classList.contains("LoadMore-root") && !location.pathname.startsWith("/users/")) {
			// Elsewhere, such as on the front page, "Load More" loads posts without end.
			return;
		}
		if (!look_again_unless(react_hydrated(el))) {
			return;
		}
		const text = text_to_click[selectors.find(selector => el.matches(selector))];
		if (text !== null && el.textContent.trim() !== text) {
			already_clicked.set(el, true);
			return;
		}
		if (el.classList.contains("LoadMore-root")) {
			// LessWrong updates its count of loaded posts a moment after it shows
			// the posts that a click on "Load More" loaded, and a click on "Load
			// More" before that loads the same posts again, so click after a delay.
			// By then, LessWrong may have removed the link, and a click on a removed
			// link would navigate to its href, "#".
			already_clicked.set(el, true);
			setTimeout(() => {
				if (el.isConnected) {
					el.click();
				}
			}, 500);
			return;
		}
		click_if_unclicked(el);
	}

	observe(2000, selectors, expand);
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
		click_if_unclicked(el);
	});
}

// Test page: https://store.steampowered.com/app/1147890/Bonfire_Peaks/
// Expected: review text is expanded on all reviews
//
// Test page: https://store.steampowered.com/app/1592280/Selaco/
// Expected: review text is expanded on all reviews; Early Access information is expanded and "READ MORE" is not visible
//
// Test page: https://store.steampowered.com/app/1145360/Hades/
// Expected: Awards and "About This Game" are expanded; "READ MORE" is not visible
if (loc.startsWith("https://store.steampowered.com/")) {
	observe(200, [
		// "READ MORE" on Early Access information
		'a.morebutton[href="#"][onclick]',
		// "READ MORE" on other sections cut off at a maximum height, such as Awards and
		// "About This Game". A click does nothing until Steam has found the section too
		// tall and added the class "collapsed" to the section's wrapper. Steam checks
		// the height as the page loads, and then every 250 ms. If a section becomes too
		// tall only after the page has loaded (e.g. once the window is narrower), its
		// "READ MORE" is clicked only if elements are added to or removed from the page
		// afterwards, because observe() doesn't react to changes of classes.
		'.game_page_autocollapse_ctn.collapsed > .game_page_autocollapse_fade > .game_page_autocollapse_readmore',
		// "Read More" on reviews of the game. The class names of this button don't say
		// what it is, and each review has other buttons, for rating the review, which
		// must never be clicked. So we check the button's text below (which works only
		// in English).
		'#app_reviews_hash button',
	], el => {
		if (el.tagName === "BUTTON" && el.innerText !== "Read More") {
			return;
		}
		click_if_unclicked(el);
	});
}

// Test page: https://www.hwinfo.com/forum/threads/prometheus-adapter-for-hwinfo-grafana-dashboard.6281/page-5 (XenForo 2.3)
// Expected: "Rohirm said:" quote at the bottom is expanded; "Click to expand..." is not visible
//
// Test page: https://forum.allaboutcircuits.com/threads/the-case-against-quantum-computing.154610/page-2 (XenForo before 2.2)
// Expected: all quotes are expanded; "Click to expand..." is not visible
if (document.documentElement.id === "XF") {
	observe(100, [
		// A quote that isn't expanded
		'.bbCodeBlock--expandable:not(.is-expanded)',
	], el => {
		// What XenForo does when "Click to expand..." is clicked
		el.classList.add("is-expanded");
	});
}

// Test page: https://www.patreon.com/gwern
// Expected: the post "New reward tier: Evernotes firehose" is shown whole, without a fade over its last lines
//
// Test page: https://www.patreon.com/amandapalmer
// Expected: "See more posts" is clicked 4 times; the page shows 100 posts
//
// Test page (logged in as a member of the creator): https://www.patreon.com/c/amandapalmer/posts
// Expected: "Load more" is clicked 4 times, and the page shows 100 posts; all long posts and long comments are expanded, and "Show more" is not visible; no "Load more comments" button or button like "View 3 replies" is clicked
//
// Test page: https://www.patreon.com/posts/beautiful-to-2746322
// Expected: all 19 comments are shown, and the 3 long ones are expanded; "Show more" is not visible
//
// Test page: https://www.patreon.com/posts/new-statesman-is-2440676
// Expected: no "Load more comments" button or button like "View 3 replies" is visible; about 150 comments are shown (of the 153 that Patreon counts)
// Expected: after a click on "Collapse replies", those replies stay hidden, below a button like "View 3 replies"
if (loc.startsWith("https://www.patreon.com/")) {
	// The box around a text that Patreon can cut off (a post, a comment, ...). In the DOM,
	// the box follows the element that contains its "Show more" button or "View post" link.
	const text_box = 'div[data-is-measured]';
	const show_more = `div:has(> ${text_box}) > div > button[aria-expanded="false"]`;
	// The box around a public post on a creator's page, which the page shows above "View post"
	const text_box_above_link = `div:has(> div > div > a[aria-expanded="false"]) > ${text_box}`;

	const comment = 'div[data-tag="comment-row"]';
	// All the comments on one post
	const comments = 'div[data-tag="content-card-comment-thread-container"]';
	const load_more_comments = 'button[data-tag="loadMoreCommentsCta"]';
	// A comment together with the replies to it
	const thread = `div:only-child:has(> ${comment})`;
	// A button among the comments that may be one that loads replies, such as
	// "View 3 replies" (which is disabled while it loads them)
	const maybe_view_replies = `${comments} button:not([data-tag], [aria-expanded], [aria-disabled="true"])`;
	// For each thread: the most comments that it has shown
	const most_shown = new WeakMap();

	const post = 'div[data-tag="post-card"]';
	// The list of posts on a creator's page, as everyone sees it
	const public_posts = 'div[data-tag="all-posts-layout"]';
	const see_more_posts = `${public_posts} + div > button`;
	// The list of posts on a creator's "Posts" tab, which only members of the creator see
	const member_posts = 'div[data-tag="post-stream-container"]';
	// A button below that list that may be "Load more"
	const maybe_load_more_posts = `${member_posts} div:has(> div > div > ${post}) ~ div > div > button`;
	// The last post of the page when the userscript last clicked "See more posts" or "Load more"
	let last_post_at_click = null;

	// Handle `el`, which matches `thread`, `load_more_comments`, or `maybe_view_replies`.
	function look_at_comments(el) {
		// A list of posts shows some comments below each post. Loading the other
		// comments and the replies of every post could take thousands of requests,
		// so load them only on the page of one post.
		if (!/\/posts\/[^/]+\/?$/.test(location.pathname)) {
			return;
		}
		if (el.matches(thread)) {
			most_shown.set(el, Math.max(most_shown.get(el) ?? 0, el.querySelectorAll(comment).length));
			return;
		}
		// Other buttons among the comments include "Collapse replies"; "View more
		// replies" and "View previous replies" load replies too.
		if (!el.matches(load_more_comments) && !/^View .*repl/.test(el.textContent)) {
			return;
		}
		if (!look_again_unless(react_hydrated(el))) {
			return;
		}
		// What the button loads comments into
		const scope = el.closest(el.matches(load_more_comments) ? comments : thread);
		if (scope === null) {
			return;
		}
		const shown = scope.querySelectorAll(comment).length;
		// A thread that shows fewer comments than it has shown was collapsed by the
		// user, so leave it. Patreon keeps the button to load the next batch, so
		// click it again, but only once more comments are shown than at the last
		// click: a batch that fails to load must not be requested over and over.
		if (shown >= (most_shown.get(scope) ?? 0) && shown > (already_clicked.get(scope) ?? -1)) {
			already_clicked.set(scope, shown);
			el.click();
		}
	}

	// Handle `el`, which matches `see_more_posts` or `maybe_load_more_posts`.
	function look_at_more_posts(el) {
		if (el.matches(maybe_load_more_posts)) {
			// Another button there shows the posts of other tiers
			if (el.textContent !== "Load more") {
				return;
			}
			// It is disabled while posts load, and nothing else may change once it is enabled again
			if (!look_again_unless(el.getAttribute("aria-disabled") !== "true")) {
				return;
			}
		}
		const posts = document.querySelectorAll(`:is(${public_posts}, ${member_posts}) ${post}`);
		const last_post = posts[posts.length - 1];
		// Each click loads another batch of posts, even while an earlier batch is
		// loading, so click at most once while the same post is the last one. A
		// creator may have thousands of posts, so stop once the page shows 100.
		if (last_post && posts.length < 100 && last_post !== last_post_at_click) {
			last_post_at_click = last_post;
			el.click();
		}
	}

	observe(1000, [
		show_more,
		text_box_above_link,
		thread,
		load_more_comments,
		maybe_view_replies,
		see_more_posts,
		maybe_load_more_posts,
	], el => {
		if (el.matches(text_box_above_link)) {
			// "View post" leads to another page, but this page has the whole text
			el.style.height = "auto";
			for (const fade of el.querySelectorAll(':scope > [class*="_fade"]')) {
				fade.style.display = "none";
			}
		} else if (el.matches(`${thread}, ${load_more_comments}, ${maybe_view_replies}`)) {
			look_at_comments(el);
		} else if (look_again_unless(react_hydrated(el))) {
			if (el.matches(show_more)) {
				click_if_unclicked(el);
			} else {
				look_at_more_posts(el);
			}
		}
	});
}

// Test page: https://mastodon.social/@spacerog/112974029330927616 (Mastodon 5.0)
// Expected: all posts are expanded; "Show post" is not visible
// Expected: the image is shown, not blurred
//
// Test page: https://mastodon.social/@spacerog/112974029330927616/embed (as embedded in other sites)
// Expected: the post is expanded; "Show post" is not visible
//
// Test page: https://mathstodon.xyz/@j_bertolotti/110436005325555159 (Mastodon before 5.0)
// Expected: all posts are expanded; "Show more" is not visible
// Expected: all animations are shown; "Sensitive content" is not visible
//
// Test page: https://infosec.exchange/@plasticbiker/112974302351350325 (glitch-soc)
// Expected: both posts are expanded; "Show more" is not visible
if (document.querySelector("#mastodon[data-props], #mastodon-status[data-props]")) {
	// Whether a click on `button` would show what the button hides (a click
	// on the button of a content warning hides the post if it is shown)
	function hides_something(button) {
		const parent = button.parentElement;
		if (!button.isConnected) {
			return false;
		}
		if (parent.hasAttribute("aria-controls")) {
			// Mastodon 5.0 makes a hidden post inert, and doesn't display a post
			// that one of the user's filters hides
			const post = document.getElementById(parent.getAttribute("aria-controls"));
			return post !== null && post.inert && post.checkVisibility();
		}
		if (parent.matches(".content-warning")) {
			// Before 5.0, a hidden post is absent, so the post's footer follows
			// the warning (in glitch-soc, after the post's mentions)
			let next = parent.nextElementSibling;
			if (next?.matches(".status__content:not(:has(> :not(.mention)))")) {
				next = next.nextElementSibling;
			}
			return next === null || next.matches(".status__action-bar, .detailed-status__meta");
		}
		return true;
	}

	observe(1000, [
		// "Show post" (Mastodon 5.0)
		'article div[aria-controls] > button',
		// "Show media" (Mastodon 5.0)
		'[class*="contentWarningBlurActive"] > button[class*="gallerySpoiler"]',
		// "Show more" (before 5.0)
		'.content-warning:not(.content-warning--filter) > button',
		// "Click to show" on media (before 5.0) that none of the user's filters hides
		':is(.status, .detailed-status) .spoiler-button:not(.spoiler-button--hidden) > .spoiler-button__overlay:enabled:not(:has(.filter-name))',
	], el => {
		// Before 5.0, a media gallery gets a new "Click to show" each time its
		// media is hidden
		const once = el.closest(".media-gallery") ?? el;
		if (already_clicked.get(once)) {
			return;
		}
		already_clicked.set(once, true);
		// A click on the button of one copy of a post toggles every copy, which
		// the page shows only after the task of the click, so click each button
		// in a task of its own
		setTimeout(() => {
			if (hides_something(el)) {
				el.click();
			}
		});
	});
}

// Test page: https://x.com/AChillGhost/status/1761774191920005343
// Expected: all replies expanded; "Show replies" is not visible
//
// Test page: https://x.com/phokarlsson/status/1808145965380935794
// Expected: after scrolling down, "Show probable spam" button is not visible
//
// Test page: https://x.com/Aella_Girl/status/1811478649964843329
// Expected: after scrolling down, "Show probable spam" button is not visible; "Probable spam" tweets are visible
//
// Test page: https://x.com/liz_love_lace/status/1812457271126835489
// Expected: after scrolling down, "Show additional replies, including those that may contain offensive content" is not visible
//
// Test page: https://x.com/terronk/status/1916218168626012356
// Expected: "Show more" not visible on ActionGrad's reply
if (loc.startsWith("https://twitter.com/") || loc.startsWith("https://x.com/")) {
	observe(1000, [
		// "Show replies"
		'button[role="button"] > div > div > div[style][dir] > span[class^="css-"]',
		// "Show probable spam"
		'button[class][role="button"][type="button"] > div > div[dir="ltr"][style] > span[class^="css-"]',
		// "Show additional replies, including those that may contain offensive content"
		'button > div[style^="text-overflow: unset;"] > span[class][style="text-overflow: unset;"] > span[class][style="text-overflow: unset;"]',
		// "Show more" on long tweets
		'button[data-testid="tweet-text-show-more-link"][type="button"] > span[class]',
		// "Show" button on Content Warning: Graphic Content
		// "View" button on "The following media includes potentially sensitive content."
		'button > div > div > span > span[class="css-1jxf684 r-bcqeeo r-1ttztb7 r-qvutc0 r-poiln3"]',
	], el => {
		if (el.innerText === "Show replies" || el.innerText === "Show probable spam" || el.innerText === "Show" || el.innerText === "Show more" || el.innerText === "View") {
			el.click();
		}
	});
}

// Test page: https://www.google.com/search?q=Pacific+West+Home+Improvements
// Expected: after clicking '54 Google reviews', all the reviews are expanded and the "More" links are not visible
if (loc.startsWith("https://www.google.com/search")) {
	observe(200, [
		'div[class][jscontroller] > a[class][role="button"][tabindex="0"][aria-label^="Read more of "][jsaction][data-ved]',
	], el => {
		if (el.innerText === " More") {
			el.click();
		}
	});
}

// Test page: https://www.nytimes.com/live/2024/07/07/world/france-election-2024
// Expected: all long posts are expanded; each "Show more" below a post has become "Show less"
if (loc.startsWith("https://www.nytimes.com/")) {
	observe(20, [
		// "Show more" below a long post on a live coverage page. The server
		// renders it, so React may not have hydrated it yet.
		'button[data-testid="Show-More"][aria-expanded="false"]',
	], el => {
		if (look_again_unless(react_hydrated(el))) {
			click_if_unclicked(el);
		}
	});
}

// Test page: https://www.cnn.com/2026/11/12/business/live-news/microsoft-global-outage-intl-hnk
// Expected: all long updates that are shown are expanded; "Read more" is not visible
//
// Test page: https://www.cnn.com/2026/11/12/business/live-news/microsoft-global-outage-intl-hnk
// Expected: all updates are shown; "Load more Live Updates" is not visible
//
// Test page: https://www.cnn.com/2026/11/12/business/live-news/microsoft-global-outage-intl-hnk?post-id=cmweh3n9k003027nubrwoaxfe
// Expected: the linked update is expanded; the hidden "Load more Live Updates" is never clicked
//
// Test page: https://www.cnn.com/2026/11/11/business/live-news/nvidia-jensen-huang-investigation-fallout
// Expected: all updates are shown in full, including the graphics that render after "Read more" is clicked
if (loc.startsWith("https://www.cnn.com/") || loc.startsWith("https://edition.cnn.com/")) {
	const load_more = 'button.live-story__load-more-updates';
	observe(1000, [
		// "Load more Live Updates" below the updates on a live news page
		load_more,
		// "Read more" below a long update on a live news page
		'article.live-story-post--collapsed button.live-story-post__collapse-button[aria-expanded="false"]',
	], el => {
		if (el.matches(load_more)) {
			// CNN hides the button once it has shown every update that the button
			// can show. A click may show the next batch of updates by changing only
			// their classes (which the MutationObserver doesn't report), so look
			// again while the button is shown.
			const all_shown = !el.checkVisibility();
			if (!look_again_unless(all_shown)) {
				el.click();
			}
		} else {
			click_if_unclicked(el);
			// CNN expands an update by fixing its max-height to its height at the
			// time of the click, which would cut off anything that renders later.
			el.closest("article.live-story-post--collapsible").style.maxHeight = "none";
		}
	});
}

// Test page: https://www.bloomberg.com/opinion/authors/ARbTQlRLRjE/matthew-s-levine
// Expected: 400 stories are listed, which is the most that Bloomberg lists for an author
// Expected: "Load more" is clicked 21 times (19 clicks that each add 20 stories, then 2 clicks that add none), and stays visible
//
// Test page: https://www.bloomberg.com/latest
// Expected: about 500 stories are listed, which is the most that Bloomberg lists there
if (loc.startsWith("https://www.bloomberg.com/")) {
	// When "Load more" was last clicked, as a performance.now() time
	let clicked_at = -Infinity;
	// The performance.now() time from which "Load more" may be clicked again,
	// or Infinity while the request that the last click caused hasn't finished
	let ready_at = 0;
	// How many stories the page listed just before the last click
	let stories_at_click = 0;
	// How many clicks have been made since that number last changed
	let clicks_since_change = 0;
	observe(1000, [
		// "Load more" below a list of stories
		'button[type="button"][data-component="outlined-button"][aria-label="more stories"]',
	], el => {
		// The page gives "Load more" the stories of a finished request a
		// moment later; a click before that adds nothing.
		if (ready_at === Infinity || !look_again_unless(performance.now() >= ready_at)) {
			return;
		}
		const stories = document.querySelectorAll('[data-component="headline"]').length;
		if (stories !== stories_at_click) {
			stories_at_click = stories;
			clicks_since_change = 0;
		}
		ready_at = Infinity;
		// 2 clicks in a row that added no stories are taken to mean that there are no more.
		if (clicks_since_change === 2) {
			console.log(`${log_prefix}the last 2 clicks of "Load more" added no stories; not clicking it again`);
			return;
		}
		clicks_since_change++;
		clicked_at = performance.now();
		el.click();
	});
	new PerformanceObserver(list => {
		// Whether a request for stories that started after the last click has finished
		if (list.getEntries().some(entry => entry.startTime >= clicked_at && /\/lineup-next\/api\/.*[?&]pageNumber=/.test(entry.name))) {
			ready_at = performance.now() + 200;
			look_at_all();
		}
	}).observe({type: "resource"});
}

// Test page (logged in, with "Compact view" switched on): https://www.criticker.com/film/Come-and-See/
// Expected: under "Ratings & Reviews", all minireviews are shown in full; "More" is not visible
// Expected: the address doesn't gain a "#"
// Expected: after searching the ratings for "the", all minireviews are shown in full
// Expected: a minireview that "Show less" has cut off again stays cut off
if (loc.startsWith("https://www.criticker.com/")) {
	observe(1000, [
		// "More" on a minireview that the compact view cuts off (each card has two: one shown in wide windows, one in narrow windows)
		'.rating_card.compact.review:not(.expanded) .ratingcard_compact_more > a',
	], el => {
		const card = el.closest(".rating_card");
		// A card that a click has expanded is left alone, so that "Show less" works.
		if (already_clicked.get(card)) {
			return;
		}
		// Until the site's script handles clicks, a click would only navigate to "#".
		el.addEventListener("click", ev => ev.preventDefault(), {once: true});
		el.click();
		if (look_again_unless(card.classList.contains("expanded"))) {
			already_clicked.set(card, true);
		}
	});
}

// Test page: https://claude.ai/chat/e82fbf77-29d5-4cde-ac52-affb4587f919
// Expected: All reasoning expanded; all 'Show more' on reasoning expanded
//
// Test page: https://claude.ai/chat/55bd8c0d-6f9a-4240-9cfa-d6a5f3b9be3a
// Expected: User's prompt is expanded, no "Show more"
if (loc.startsWith("https://claude.ai/chat/")) {
	observe(20, [
		// Reasoning trace with one-line summary
		'button[aria-expanded="false"][class="group/status flex items-center gap-2 py-1 text-sm transition-colors cursor-pointer text-left text-text-500 hover:text-text-300 flex-1 min-w-0"]',
		// "Show more" on reasoning trace
		'button[class="text-xs text-text-500/80 hover:text-text-100 transition opacity-0 group-hover/timeline-text:opacity-100 focus-visible:opacity-100 [@media(hover:none)]:opacity-100"]',
		// "Show more" on user's prompt
		'button[class="pb-3 pt-1 text-xs text-text-500/80 hover:text-text-100 transition w-3/4 text-left rounded-lg"]',
	], el => {
		click_if_unclicked(el);
	});
}

// Test page: https://shimmeringvoid.substack.com/
// Test page: https://jenniferdaniel.substack.com/
// Expected: the subscribe modal with "No thanks" is bypassed
//
// Test page: https://mrgirl.substack.com/p/the-destiny-report/comments
// Test page: https://www.md-a.co/p/scarcity-truthers/comments
// Expected: "See more" is clicked below each long comment; the full text of each comment is shown
//
// Test page: https://mrgirl.substack.com/p/the-destiny-report/comments
// Test page: https://dispatchesfromtheautismwars.substack.com/p/how-i-learned-to-stop-worrying-and/comments
// Test page: https://www.gurwinder.blog/p/tiktok-may-be-a-chinese-bio-weapon/comments
// Expected: "Load More" button at the bottom is clicked; all comments are shown
//
// This block should be the last one in the script.
if (
	observer === null &&
	(window.location.host.endsWith(".substack.com") ||
		document.querySelector('link[href^="https://substackcdn.com"]'))
) {
	observe(100, [
		// "No thanks" on the subscribe overlay; clicking removes the overlay
		'div.intro-popup [data-testid="maybeLater"]',
		// "See more" below a long comment
		'div.comment-body.long:not(.expanded) + button',
		// "Load More" at the end of a comments page
		'div.comment-list-items + button',
	], el => {
		// Substack reuses the "Load More" button for each page of comments, so it
		// must be clicked more than once.
		if (el.matches('div.comment-list-items + button')) {
			el.click();
		} else {
			click_if_unclicked(el);
		}
	});
}
