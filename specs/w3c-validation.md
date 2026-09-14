# W3C validation: findings inventory and fix specification

Status: **Implemented 2026-09-14 — all findings fixed. S1–S7 + F11/F12 in
PR #24 (`w3c-html-fixes`); S8/S9/S10 + F13/F14 + strict-by-default ratchet
in the stacked `w3c-feed-p-figure` PR.**
Companion to PR #21 (`feed/absolute-urls`), which added the validation
tooling. Each finding below was reproduced with `just validate-feed` /
`just validate-html` and traced to its source line.
Author: michalvanko + agent
Last updated: 2026-09-14

## Tooling (already on PR #21)

- `just validate-feed [local|prod|<base-url>]` — POSTs `/feed.xml` content to
  <https://validator.w3.org/feed/check.cgi> (`rawdata` mode, SOAP output).
  No public URL needed, results never cached (the `url=` interface caches
  per URL and served a stale verdict during the PR #21 session).
  `STRICT=1` fails on warnings; `SelfDoesntMatchLocation` is allowlisted
  (rawdata submissions carry no location to compare against).
- `just validate-html [target] [pages]` — POSTs page bodies to
  <https://validator.w3.org/nu/?out=json>. Trims content after `</html>`
  (debug-only livereload injection; prod never has it; `--no-trim`) and
  hides CSS-checker errors (Nu's CSS knowledge lags Tailwind v4; `KEEP_CSS=1`).
- Both are **manual-use tools** (pre-deploy / review) — the W3C services are
  shared public infrastructure, so no per-push CI.

## Current verdicts (2026-09-14, local server output)

| Check | Verdict | Notes |
|---|---|---|
| `feed.xml` (feed validator) | **VALID** | 0 errors; warnings: F8 ×21, F9 ×3, F10 ×1 (all deferred) |
| `/`, `/blog`, `/showcase`, `/portfolio` (Nu) | **VALID** | 0 errors each (2026-09-14, after `w3c-html-fixes`) |
| blog post pages (Nu) | 1 error | F8 stray `</p>` inside article bodies — the only remaining HTML error |

Post-fix notes (2026-09-14): F7 turned out to hit `/` and `/portfolio` too,
not just `/showcase` — fixed per-page (index got an `h2` "what I do" above the
talent cards; portfolio's h3 tagline became a styled `p`; showcase got `h2`
"featured work"/"more work"). Also: `npx svgstore` no longer runs — svgstore@3
dropped its bin; the recipe now uses `npx --yes svgstore-cli` (same flags),
which emits no XML prolog at all (the sed strip stays as a guard).

## Findings inventory

| ID | Surfaces | Message (type) | Count | Root cause | Fix |
|----|----------|----------------|-------|------------|-----|
| F1 | all pages | `Saw "<?"` + `Stray doctype` | 2/page | `templates/icons/sprite.svg` starts with `<?xml …?><!DOCTYPE svg …>` copied by svgstore; inlined raw in `base.html:47` | [S1](#s1-strip-sprite-prolog-svgstore-post-process) |
| F2 | all pages | `Bad value "dots-@-15pt" for attribute "id" … Not a valid XML 1.0 name` | 1/page | `static/svg/input/json-feed.svg` uses `@` in the id; svgstore copies it into the sprite | [S2](#s2-fix-invalid-symbol-id-in-json-feed-svg) |
| F3 | /portfolio | `xmlns:inkscape/sodipodi not allowed`, `sodipodi:namedview unknown element`, `sodipodi:nodetypes`, `Saw "<?"` | ~10 | `templates/icons/m-logo-animated.svg` is a raw Inkscape export inlined into the page | [S3](#s3-clean-the-animated-logo-svg) |
| F3b | /portfolio | `Element "style" not allowed as child of element "aside"` | 1 | `templates/portfolio.html:39-44` puts a literal `<style>` in the body to size the logo | [S3](#s3-clean-the-animated-logo-svg) |
| F4 | all pages | `Attribute "view-transition-name" not allowed on element "header"/"footer"` | 2/page | `site_header.html:1`, `site_footer.html:1` use it as a bare HTML attribute; every other template correctly uses `style="view-transition-name: …"` | [S4](#s4-move-bare-view-transition-name-attributes-into-style) |
| F5 | all pages | `Attribute "xmlns:cc"/"xmlns:dct" not allowed here` | 2/page | CC license chooser markup in `site_footer.html` declares XML namespaces on the license `<p>` (XHTML-style) | [S5](#s5-cc-license-metadata-via-rdfa-prefix) |
| F6 | /, /blog, post pages | `Bad value "2020-05-11 05:38:18.797 UTC" for attribute "datetime" on element "time"` | 1–8/page | templates render `{{post.metadata.date}}` — chrono `Display` output, not a machine-readable format | [S6](#s6-machine-readable-time-datetimes) |
| F7 | /showcase (likely / too) | `The heading "h3" … follows "h1" … skipping 1 heading level` | 1+/page | `project_preview_card.html:32` (and `blog_post_preview.html:31`) render `<h3>` directly under the page-level `<h1>` | [S7](#s7-heading-hierarchy-in-preview-cards) |
| F8 | feeds (+ article HTML on site) | feed validator `NotHtml` — `Invalid HTML: unexpected end tag (p)` | 20 items | `parse_markdown` emits `<figure>` (images) and `.code-card` divs (code) *inside* `<p>`; HTML5 parsers implicitly close the paragraph at the block element, making pulldown's later `</p>` stray | [S8](#s8-stop-nesting-figurecode-card-inside-p) |
| F9 | feeds | `SecurityRisk` — `content:encoded should not contain iframe tag` | 3 items | Twitch embeds in broadcasts posts | [S9](#s9-iframe-policy-for-feed-content) |
| F10 | feeds | `CharacterData` — encode `&`/`<` in plain text using hex references | 1 | a post title contains `&`; rss crate escapes as `&amp;`, validator prefers `&#x26;` style | [S10](#s10-hex-escapes-in-titles-validator-style) |
| F11 | blog post pages | `Element "header" must not appear as a descendant of element "footer"` | 3/page | `blog_post.html` used a `<footer>` as the page-bottom **layout grid wrapper**; the further-reading section inside it renders `<header>` (macro + preview cards) | **Fixed** (2026-09-14): wrapper is now a plain `<div>` |
| F12 | /blog | `Bad value "/blog/dev-2019-08-09-ide-to copy" … Space is not allowed` | 1 | `_posts/blog/dev-2019-08-09-ide-to copy.md` — intentionally published dev/test article (see 05fd17d) whose filename (→ slug) contains a space | **Fixed** (2026-09-14): file renamed to `dev-2019-08-09-ide-to-copy.md`; URL loses the `%20` |
| F13 | article HTML + feeds | incomplete figure opener for **external** URLs: bare `<img …>` but the alt text was meant to display as the caption (author intent — same as local images) — with figure *closers* only, the text floated as loose content with stray tags | 6 posts | `markdown.rs` Start(Image) had a bare-`<img>` path for non-`/` URLs while End(Image) always closed a figure | **Fixed** (2026-09-14, with S8): external images emit the same `<figure><img …><figcaption>` opener as the SVG path — alt text renders as the visible caption and the unconditional closer is correct again |
| F14 | feeds | raw `&` in hand-built attributes: external image `src` (`?u=x&f=1`) and the S9 link-card `href` (`?video=1&parent=x`) — `htmlParseEntityRef` → validator NotHtml | 6 items | `formatdoc!`/`format!` interpolated URLs verbatim; lol_html `get_attribute` returns serialized values (entities intact), `before()` inserts raw HTML | **Fixed** (2026-09-14): `escape_attr` in markdown.rs; entity-aware `escape_raw_ampersands` in feed.rs (never double-escapes `&amp;`) |

Not findings (tooling handles): CSS-checker errors (`view-transition-name`
etc. as *CSS* — Nu's CSS snapshot lags Tailwind v4; hidden by default), and
the debug-only livereload script after `</html>` (trimmed before POST).

## Fix specifications

### S1: strip sprite prolog (svgstore post-process)

The `just svgstore` recipe (`justfile`) runs `npx svgstore -o
templates/icons/sprite.svg static/svg/input/*.svg`. The output begins with
`<?xml version="1.0" encoding="UTF-8"?><!DOCTYPE svg PUBLIC …>` which is
copied into every page via `base.html:47` (`{% include "icons/sprite.svg" %}`).
XML PIs and doctypes are invalid inside HTML text.

**Fix:** in the `svgstore` recipe, post-process the output — delete the
leading `<?xml …?>` and `<!DOCTYPE …>` (e.g. `sed -i '1{/^<?xml/d}'` +
doctype line, or a small strip since it is all on line 1). Regenerate the
sprite. Verification: `just validate-html local '/'` reports no `Saw "<?"`
/ `Stray doctype`.

### S2: fix invalid symbol id in json-feed.svg

`static/svg/input/json-feed.svg` contains `id="dots-@-15pt"` (and internal
references `url(#dots-@-15pt)` / `href="#dots-@-15pt"`). `@` is not a valid
XML 1.0 name-start/name character, so the copied id is invalid in the sprite.

**Fix:** rename to a valid id (`dots-at-15pt`) in the source file — every
occurrence (id + references). Runs through `just svgstore`. Verification:
`grep -o 'id="dots[^"]*"' templates/icons/sprite.svg` shows the new id; Nu
error gone.

### S3: clean the animated logo SVG

`templates/icons/m-logo-animated.svg` is a raw Inkscape document: `<?xml?>`
PI, `inkscape:version`, `xmlns:inkscape`/`xmlns:sodipodi`,
`sodipodi:namedview` element (with editor viewport metadata),
`sodipodi:nodetypes` on paths. Inlined by `templates/portfolio.html:45`.
Additionally `portfolio.html:39-44` places a `<style>` element in the body
(inside the `#logo-container` aside) to size the logo — `<style>` is only
valid in `<head>` (or inside the SVG itself as `<svg><style>`).

**Fix:**
1. Optimize the SVG: `npx svgo --multipass templates/icons/m-logo-animated.svg`
   (drops editor namespaces, namedview, nodetypes, the PI). Verify the
   animation still plays (it is CSS/SMIL-driven — check which, and that svgo
   keeps it; `--multipass` default keeps both).
2. Move the sizing CSS into `styles/input.css` (`#logo-container svg {
   height: 100%; width: 100% }`) and delete the inline `<style>` block from
   `portfolio.html`.

Verification: `/portfolio` Nu errors drop to the shared set (sprite/vtn/cc
only); logo still animates on the page.

### S4: move bare `view-transition-name` attributes into `style`

`site_header.html:1` and `site_footer.html:1`:
`<header … view-transition-name="site_header">`. `view-transition-name` is a
CSS property, not an HTML attribute — the rest of the codebase already does
it right (`style="view-transition-name: post_title_{{post.slug}}"` etc.).
Chrome tolerates the attribute form, which is why the transitions still
worked.

**Fix:** `view-transition-name="site_header"` →
`style="view-transition-name: site_header"` (same for footer). The view
transition behaviour is unchanged (same property, same computed value).
Verification: Nu attribute errors gone; visually confirm the header/footer
still exclude themselves from cross-page view transitions.

### S5: CC license metadata via RDFa `prefix`

`site_footer.html` declares `xmlns:cc` / `xmlns:dct` on the license `<p>`
(the Creative Commons chooser-v1 badge markup, written for XHTML). In HTML5,
namespace declarations are not attributes — RDFa 1.1 uses the `prefix`
attribute instead, which *is* valid HTML5.

**Fix:** replace the two `xmlns:` declarations with
`prefix="cc: http://creativecommons.org/ns# dct: http://purl.org/dc/terms/"`
on the same `<p>`. Keep the `property="dct:title"` /
`rel="cc:attributionURL"` attributes as-is — they are RDFa CURIEs and
remain valid with the prefix mapping. Verification: Nu `xmlns:` errors gone;
the badge still links correctly (the `rel="license"` anchor is plain HTML
and untouched).

### S6: machine-readable `<time datetime>`s

`blog_post_preview.html:23`, `blog_post.html:36`, `index.html:45` render
`datetime="{{post.metadata.date}}"` — chrono's `Display` gives
`2020-05-11 05:38:18.797 UTC`, which does not satisfy the HTML
time-datetime format (machine-readable dates need ISO 8601 shapes; the
" UTC" suffix and space separator are invalid).

**Fix:** add an askama filter next to `pretty_date` in `src/filters/mod.rs`
(e.g. `machine_date`) returning `date.to_rfc3339_opts(SecondsFormat::Secs,
true)` → `2020-05-11T05:38:18Z`. Use it in the three `datetime=`
attributes (the visible text keeps `pretty_date`). Add a unit test
mirroring the `pretty_date` tests. Verification: homepage Nu time errors
(8×) gone; feeds unaffected (they use RFC 2822/3339 already).

### S7: heading hierarchy in preview cards

`project_preview_card.html:32` renders `<h3>` for the project title; the
showcase listing (generic `post_list.html`, `<h1>` at line 15) provides an
`<h1>` and no `<h2>` level in between — a level skip.
`blog_post_preview.html:31` has the same `<h3>` shape (check each listing
page's outline: index uses an `<h1>` hero and `<h2>` section headings, so
posts-at-h3 may be fine there).

**Fix (implemented 2026-09-14, revised same day):** `/showcase` got `h2`
section headings ("featured work", "more work" — the latter was a styled
`p`); the talent-card component's titles were demoted `h3` → `h2` (used
on `/` and `/portfolio`, outline-valid on both — on portfolio the cards
become siblings of the "Skills" h2; an initially added visible `h2`
"what I do" above the index cards was dropped in favor of the demotion);
`/portfolio`'s tagline `h3` became a styled `p` (not a real section
heading). Shared preview components unchanged — `/blog`'s outline was
already h1 → h2 (year) → h3. Verification: no "skipping 1 heading level"
errors on `/`, `/blog`, `/showcase`, `/portfolio`.

### S8: stop nesting `<figure>`/code-card inside `<p>`

The big one — **site-wide, not feed-only** (browsers cope, but every
article page carries the same invalid nesting; the feed validator surfaced
it as `NotHtml` ×20 because it parses `content:encoded` strictly).

Root cause (see `specs/feed-image-url.md` "Still open" and A.4): in
`src/filters/markdown.rs`, `Event::Start(Tag::Image)` returns raw
`Event::Html("<figure>…<figcaption>")` while pulldown-cmark still wraps the
image's paragraph — emitting `<p>text <figure>…</figure></p>`. Same shape
for fenced code blocks (`.code-card` div). HTML5 parsers close the `<p>`
implicitly when the block element starts, so the trailing `</p>` is stray
("Unexpected end tag (p). Ignored.").

**Fix directions (evaluate in implementation):**
1. In the `Parser` map, track whether we are inside a paragraph whose only
   significant content is the image/code block, and suppress the enclosing
   `Start/End(Paragraph)` events for those (emit the figure at block level).
   Images *inline within* flowing text keep the current shape — those are
   the genuinely tricky cases; consider forcing images to their own block
   (they already render as `<figure>`, so mid-paragraph figures are visually
   block-level today anyway — making them block-level in the event stream
   matches reality).
2. Alternatively post-process the rendered HTML with lol_html (the feed
   absolutizer already links it) to hoist `<figure>`/`.code-card` out of
   `<p>` — but a parser-level fix keeps one code path for site and feeds.

**Implemented (direction 1, 2026-09-14):** `BlockFigureParagraphGate` in
`src/filters/markdown.rs` — a streaming event filter that defers `<p>`
emission until inline content arrives (image-only paragraphs emit a bare
`<figure>`), closes the paragraph before a figure that starts mid-text,
and reopens a fresh `<p>` for trailing inline content. Whitespace directly
after a figure is dropped so no empty `<p>` materializes. Fenced code
blocks needed no gating — pulldown-cmark never nests `CodeBlock` inside
`Paragraph` events (fences may interrupt paragraphs); only images were
affected. Note: image-in-figure caption text passes through untouched (the
gate tracks figure spans). Unit tests cover the shapes; the feed corpus
audit asserts no `<p>`-nested blocks across all rendered posts.

**Tests:** snapshot-style assertions that rendered article HTML contains no
`<p>` immediately followed by `<figure>`/`<div class="code-card">`; existing
feed audits stay green. **Verification:** `just validate-feed` `NotHtml`
count 20 → 0; article pages lose the stray-`</p>` Nu errors.

### S9: iframe policy for feed content

Three broadcasts posts embed Twitch players via `<iframe>`; the feed
validator flags any iframe in `content:encoded` as `SecurityRisk`. Readers
commonly strip iframes anyway (FreshRSS/SimplePie sanitizes them by
default).

**Decision needed:** keep (warning tolerated; embeds work in
iframe-permissive readers) or replace in *feed content only* with a link
card (`<p><a href="<embed-url>">▶ watch on Twitch</a></p>`) via an
`element!("iframe", …)` handler in `absolutize_html` — the site keeps the
embed. If replaced: preserve the original URL (it is already absolute, e.g.
`player.twitch.tv/?video=…`). Verification: `SecurityRisk` ×3 gone or
explicitly accepted and allowlisted in `scripts/validate_feed.py` with a
comment.

**Implemented (link card, 2026-09-14):** `absolutize_html` now replaces
every `<iframe>` with `<p><a href="…">▶ watch on Twitch</a></p>`-style link
cards — host-derived labels (Twitch/YouTube/Spotify/Vimeo, fallback "open
the embedded content"), src absolutized, srcless shells dropped. The site
keeps the live embeds. Covers the podcast/YouTube embeds too, not just
Twitch.

### S10: hex escapes in titles (validator style)

One post title contains `&`; the rss crate serializes `&amp;`, the
validator stylistically prefers `&#x26;` in plain text. Warning-only, no
functional impact. **Fix (optional):** post-process title strings before
`ItemBuilder` (replace `&amp;` → `&#x26;`, `<`-escapes → `&#x3C;`) — or
accept and allowlist `CharacterData` in `scripts/validate_feed.py`.
Recommend the allowlist unless the warning bothers you: the escape form is
a style preference, both are well-formed XML.

**Resolved (allowlisted, 2026-09-14):** `CharacterData` joins
`SelfDoesntMatchLocation` in `DEFAULT_ALLOW` with a comment explaining the
style-preference rationale.

## Suggested batching (each independently shippable)

1. **Template quick-fixes** — S4 + S5 + S6 + S3b (one small PR; removes
   ~2–8 errors per page across the whole site, plus the only Rust change is
   the `machine_date` filter + tests).
2. **Sprite pipeline** — S1 + S2 (justfile + one source SVG; removes
   3 errors from every page).
3. **Animated logo cleanup** — S3 (verify animation survives svgo).
4. **Heading hierarchy** — S7 (small design call first).
5. **`<p><figure>` restructure** — S8 (cross-cutting; biggest payoff:
   clears F8 feed warnings and site-wide article HTML; coordinate with
   `specs/feed-image-url.md`).
6. **Feed policy closers** — S9 + S10 (decisions, then small handlers).

## End state and ratchet

After 1–5: `just validate-html` exits 0 on the default page set; after 6 +
S8: `just validate-feed` reports zero warnings → flip the default to strict
(`STRICT=1` semantics, or make strict the default with an escape hatch) and
add both to the pre-deploy routine (`just validate` before `just deploy`).

**Done (2026-09-14):** `validate-feed` is strict by default (`LAX=1`
drops back to errors-only when investigating a new warning); the
pre-deploy routine is `just validate` (feed + HTML pages). End state
reached: 0 HTML errors on the default page set, 0 feed errors, 0
non-allowlisted feed warnings.
