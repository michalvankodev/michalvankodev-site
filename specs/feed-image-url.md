# Feed content: absolute URLs for syndicated HTML

Status: **Proposed — not implemented.** Split out of PR #13 (redesign 08 —
syndication & perf) review for a separate session.
Author: michalvanko + agent
Last updated: 2026-09-28

## Problem

`build_feed_items()` renders post bodies through the site's `parse_markdown`
askama filter (same one the templates use). That filter is correct for web
pages — they are served from the site origin — but the output is syndicated
via RSS (`/feed.xml`) and JSON Feed (`/feed.json`), where it is wrong.

Feed readers (NetNewsWire, Feedbin, Feedly, Miniflux, Thunderbird, …) render
`content_html` inside **their own document context**. Root-relative URLs
resolve against *the document's origin*, so in a reader:

- `<img src="/generated_images/…">` resolves to
  `https://<reader-domain>/generated_images/…` → **broken image**
- `<a href="/showcase/egg-fetcher">` navigates into the reader's own domain →
  **broken link**
- every `srcset` candidate has the same problem

Measured on the live feed (2026-09-28, `feed.json`, 41 items):

| Carrier    | Root-relative occurrences | Example |
|------------|--------------------------:|---------|
| `href`     | 6                         | `/showcase` |
| `src`      | 40                        | `/images/uploads/pi-logo.svg` |
| `srcset`   | 171 (URLs, not attrs)     | `/generated_images/images/uploads/pi-screenshot_1250x699.png` |

Both relevant specs require absolute URLs here: JSON Feed 1.1 ("any markup …
should use absolute URLs") and the RSS Best Practices Profile (relative URLs
in `content:encoded` are flagged by validators).

The feed tests added in PR #13 only assert `<p>` presence, so this ships
unnoticed.

## Constraint: attribute-aware rewriting only

17 of 41 posts contain code blocks (syntect HTML whose *text* may legitimately
contain `href="/about"`-style snippets). A naive string replace
(`s|href="/|href="https://michalvanko.dev|g`) would corrupt displayed code in
~40% of posts. Any fix must rewrite **attribute values of real elements**
only, never text content.

## Options considered

### A. Post-process in `feed.rs` with `lol_html` (recommended)

Cloudflare's streaming HTML rewriter crate. One `absolutize_html(&str) ->
String` applied in `build_feed_items()` to both `description_html` and
`content_html` — single choke point, covers RSS + JSON feeds.

Element handlers:

- `a[href]` — prefix value if it starts with `/` (not `//`)
- `img[src]` — same
- `source[srcset]`, `img[srcset]` — split on `,`, prefix each entry that
  starts with `/`, keep resolution descriptors

Pros:

- attribute-aware → code samples are safe by construction
- catches carriers the markdown parser never sees: raw HTML written directly
  in posts, anything future filters emit
- zero changes to the template-shared filter (no risk to pages)
- ~40 lines + tests; one small, well-maintained dependency

Cons:

- one new dependency (`lol_html`)

### B. Event-level rewrite inside the markdown pipeline (no new dep)

Refactor `filters/markdown.rs` into `render_markdown(md, base: Option<&str>)`
with two thin `#[askama::filter_fn]` wrappers (existing `parse_markdown` =
base `None`). Rewrite `Tag::Link` dests when base is set; absolutize the
machine-generated `<picture>` markup string (safe — it is generator output,
not author content). `feed.rs` calls `render_markdown(&body, Some(SITE_URL))`.

Pros: no new dependency; fix lives at the source.

Cons: ~80+ lines of churn in code shared by every page template;
does **not** catch raw-HTML links inside posts; two places to keep in sync
(links vs picture markup).

### C. Naive string replace — rejected

Corrupts code samples (see constraint above).

## Test plan (either option)

Extend the feed tests to assert, for every item of both feeds:

- no `content_html` / `description_html` attribute value (`src`, `href`,
  any `srcset` entry) starts with a root-relative `/`
- already-absolute URLs (external `https://…` images) pass through unchanged
- a known code-sample fixture is byte-identical (guards against text-content
  rewriting)

## Related open findings from the PR #13 review

- `/feed.json` + `/sitemap.xml` never land in `dist/` (crawler never sees
  them) — preview 404s prove it; needs its own fix (discovery `<link>` in
  `base.html` and/or explicit curl into `dist/` in `just ssg`).
- SVG `og:image` / `twitter:image` are unusable for social cards.
