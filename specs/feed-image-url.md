# Feed content: absolute URLs for syndicated HTML

Status: **Accepted — implemented in `src/feed.rs` (`absolutize_html`); both
feeds verified root-relative-free locally** (2026-09-11: `feed.json` +
`feed.xml` — 0 root-relative `src`/`href`/`srcset`; was 38 / 8 / 169). Proof
chain in [Appendix A](#appendix-a--proof-why-relative-urls-are-unusable-in-feed-content).
Split out of PR #13 (redesign 08 — syndication & perf) review
for a separate session.
Author: michalvanko + agent
Last updated: 2026-09-11

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

Measured on the live feed (2026-09-11, `feed.json`, 41 items; PR #13 review
measured 6 / 40 / 171 — drift is content churn, not a fix):

| Carrier    | Root-relative occurrences | Example |
|------------|--------------------------:|---------|
| `href`     | 8                         | `/broadcasts/tags/DevBreak` |
| `src`      | 38                        | `/images/uploads/pi-logo.svg` |
| `srcset`   | 169 (URLs, not attrs)     | `/generated_images/images/uploads/pi-screenshot_1250x699.png` |

The requirement is normative for RSS and structural for JSON Feed: the RSS
Best Practices Profile forbids relative URLs in feed HTML ("the RSS format
does not provide a means to identify the base URL of a document") and the W3C
feed validator flags the live feed for exactly this (Appendix A.4). An earlier
revision of this spec credited JSON Feed 1.1 with an "absolute URLs" clause —
that clause does not exist; see Appendix A.1 for the corrected citations, and
[Appendix A](#appendix-a--proof-why-relative-urls-are-unusable-in-feed-content)
for the full proof chain (format → resolution → reader → validator).

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

Element handlers (as implemented):

- `a[href]` — prefix value if it starts with `/` (not `//`)
- `img[src]` — same
- `img[srcset]`, `source[srcset]` — split on `,`, prefix each entry that
  starts with `/`, keep resolution descriptors
- `iframe[src]`, `video[src]`, `audio[src]`, `source[src]` — same rule; today's
  posts don't use these carriers root-relatively, but keeping them covered
  makes the feed-level test invariant ("no root-relative `src`/`href`/
  `srcset` anywhere") hold for future content

Test notes:

- the audit scanner must exclude `<pre>`/`<code>` contents before matching:
  pulldown-cmark escapes `<`/`>` in inline code spans but **not quotes**, so
  `` `<a href="/about">` `` legitimately contains `href="/about"`-shaped text
  that is not an attribute (syntect code blocks, in contrast, escape quotes
  as `&quot;`)

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

## Related open findings from the PR #13 review + W3C validator run

- `/feed.json` + `/sitemap.xml` never land in `dist/` (crawler never sees
  them) — preview 404s prove it; needs its own fix (discovery `<link>` in
  `base.html` and/or explicit curl into `dist/` in `just ssg`).
- SVG `og:image` / `twitter:image` are unusable for social cards.
- Enclosure `length` is `0` — invalid per the RSS Profile ("must be a positive
  integer"); the `rss` crate requires an explicit length and
  `render_rss_feed` never sets one.
- `description` excerpts truncate mid-element → validator warns "Invalid HTML:
  unexpected end tag (p)" (`truncate_md` cuts without closing tags).
- A post embeds an `<iframe>` in `content:encoded` (validator `SecurityRisk`
  warning) — decide: allow it (many readers strip iframes) or remove it.
- Channel is missing `atom:link rel="self"` (`MissingAtomSelfLink` warning).

## Appendix A — Proof: why relative URLs are unusable in feed content

Evidence chain, layer by layer: the formats define no base URI (A.1); URL
resolution therefore depends on which document the reader parses the markup
into (A.2); readers compensate or break (A.3); and validators flag the feed as
non-conformant today (A.4).

### A.1 Format layer: RSS and JSON Feed define no base URI

RFC 3986 §5.1 ([rfc-editor.org/rfc/rfc3986#section-5.1](https://www.rfc-editor.org/rfc/rfc3986#section-5.1)):

> The term "relative" implies that a "base URI" exists against which the
> relative reference is applied. … relative references are only usable when a
> base URI is known. A base URI must be established by the parser prior to
> parsing URI references that might be relative.

- **RSS 2.0** ([rssboard.org/rss-specification](https://www.rssboard.org/rss-specification)):
  the spec text contains no `xml:base`, no base-URL concept, no absolute-URL
  requirement — zero occurrences (verified 2026-09-11). The only embedded-base
  mechanism RFC 3986 §5.1.2 acknowledges is MIME/MHTML metadata
  ([RFC 2557](https://www.rfc-editor.org/rfc/rfc2557)), which no feed format
  uses.
- **RSS Advisory Board Best Practices Profile**
  ([rssboard.org/rss-profile](https://www.rssboard.org/rss-profile)):
  - Requirements (§3.4 *URLs*):
    > In all link and url elements, the first non-whitespace characters in a
    > URL must begin with a scheme … **These elements must not contain
    > relative URLs.**
  - Recommendations (§4.1.1.20.4 `description`):
    > The description **should not contain relative URLs, because the RSS
    > format does not provide a means to identify the base URL of a document.**
    > When a relative URL is present, an aggregator may attempt to resolve it
    > to a full URL using the channel's link as the base.

  That last sentence is the whole story: reader-side resolution is a
  MAY-level heuristic ("may attempt"), never a publisher guarantee.
- **JSON Feed 1.1** ([jsonfeed.org/version/1.1](https://www.jsonfeed.org/version/1.1/)):
  HTML is allowed only in `content_html`, and the format defines **no base-URL
  mechanism at all** (no `xml:base` equivalent, no per-item base field), so a
  relative reference inside `content_html` is unresolvable by construction — a
  reader can only guess `home_page_url` as base.
- **Correction:** an earlier revision of this spec cited JSON Feed 1.1 for
  "any markup … should use absolute URLs". That sentence appears in neither
  the 1.1 nor the 1.0 spec text, and the official validator
  ([validator.jsonfeed.org](https://validator.jsonfeed.org),
  [manton/jsonfeed-validator](https://github.com/manton/jsonfeed-validator))
  checks only field presence — it never scans `content_html`. The requirement
  rests on the RSS Profile above, the resolution mechanics in A.2, and the
  validator evidence in A.4.

### A.2 Resolution layer: the same `src` resolves correctly or 404s, depending only on the reader's document

When a feed reader displays `content_html`, it parses that fragment into
**its own document**. WHATWG HTML resolves every `img[src]`, `a[href]` and
`srcset` candidate against the document's base URL
([document base URL](https://html.spec.whatwg.org/multipage/urls-and-fetching.html#document-base-url),
[img element](https://html.spec.whatwg.org/multipage/embedded-content.html#the-img-element));
the fragment contains no `<base>` element, so the base is the reader's page.
Taking a real root-relative `src` from our live feed and resolving it per
RFC 3986 §5.2 against each candidate base:

| Base the consumer ends up with | Resolved URL |
|--------------------------------|--------------|
| channel `<link>` = `https://michalvanko.dev/` (RSS Profile heuristic) | `https://michalvanko.dev/generated_images/…/pi-screenshot_1250x699.png` ✓ |
| feed retrieval URI = `https://michalvanko.dev/feed.xml` (RFC 3986 §5.1.3) | `https://michalvanko.dev/generated_images/…/pi-screenshot_1250x699.png` ✓ |
| reader document = `https://freshrss.example/i/?a=user&get=f_1` | `https://freshrss.example/generated_images/…/pi-screenshot_1250x699.png` **✗ 404** |
| reader document = desktop app webview `https://app.reader.local/feed/42` | `https://app.reader.local/generated_images/…/pi-screenshot_1250x699.png` **✗ 404** |

Publishers cannot choose the row; readers choose for us. Emitting absolute
URLs is the only publisher-side move that makes resolution
reader-independent.

### A.3 Reader layer: why it *looks* fine in FreshRSS

Observed 2026-09-11 in the author's FreshRSS: images from this feed render
correctly **despite** the root-relative payload. That is reader-side
compensation, not feed correctness:

- FreshRSS parses feeds with [SimplePie](https://simplepie.org/)
  (FreshRSS `app/Models/Feed.php` instantiates `FreshRSS_SimplePieCustom`).
- SimplePie determines a base — explicit `xml:base` if present, else the
  channel's first `link rel=alternate` / `rel=self` / the feed URL
  (`src/SimplePie.php`, `get_base()`) — and rewrites URL attributes against
  it via `Misc::absolutize_url()`
  ([src/Sanitize.php](https://github.com/simplepie/simplepie/blob/master/src/Sanitize.php),
  `set_url_replacements()` / `sanitize()`).
- This is precisely the RSS Profile's "aggregator may attempt to resolve …
  using the channel's link as the base" compensation from A.1.

Two gaps remain even in compensating readers:

1. **Scope:** SimplePie's default rewrite list is `a[href]`, `area[href]`,
   `audio[src]`, `blockquote[cite]`, `del[cite]`, `form[action]`,
   `img[longdesc|src]`, `input[src]`, `ins[cite]`, `q[cite]`, `source[src]`,
   `video[poster|src]` — **`srcset` is not rewritten**. Our `<picture>` markup
   is entirely `srcset`-driven (169 root-relative candidates), so in FreshRSS
   every responsive candidate 404s against the reader origin and the browser
   silently falls back to the one rewritten `<img src>` — no responsive
   selection, plus a burst of wasted requests per article.
2. **Coverage:** readers that render `content_html` into their own origin
   without a rewriting pre-pass (A.2, bottom two rows) get broken images and
   links by construction.

Conclusion: correct rendering in FreshRSS is an artifact of an undocumented
reader behavior doing the publisher's job — and even it fails on every
`srcset` candidate. The payload must be self-sufficient.

### A.4 Validator layer: the live feed is flagged today

W3C Feed Validation Service on `https://michalvanko.dev/feed.xml`
(2026-09-11):

```
warning ContainsRelRef  content:encoded should not contain relative URL references
                        /images/uploads/pi-logo.svg
warning ContainsRelRef  description should not contain relative URL references
                        /broadcasts/tags/DevBreak
```

Reproduce:

```bash
curl -s "https://validator.w3.org/feed/check.cgi?url=https%3A%2F%2Fmichalvanko.dev%2Ffeed.xml&output=soap12" \
  | grep -o 'ContainsRelRef[^<]*'
```

This is the property the feed tests must encode after the fix: no attribute
value (`src`, `href`, any `srcset` entry) in either feed may be root-relative.
