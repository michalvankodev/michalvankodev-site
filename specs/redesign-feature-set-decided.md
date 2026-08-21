# Redesign: decided feature set

> **Handoff context — read this first if you're a fresh session picking this up.**

## What this document is

The final, closed decision record for the next redesign iteration of
michalvankodev-site. Every item of the ~55-item candidate menu (see
`specs/redesign-feature-sets.md`) has a verdict: **selected / rejected /
postponed**. Decisions below are FINAL — implement them as written; do not
re-litigate. Open questions explicitly marked `[ ]` are the only judgment
calls left, and they're scoped to implementation details.

## The goal (for the kickoff session)

**Branch `redesign/selected` off `main` and implement this decided set as a
series of small PRs** to the katelyn Forgejo remote. First commit on the
branch: this doc + the rewritten `DESIGN.md` (the old `DESIGN.md` on main
describes the previous direction — replace it from this doc; keep `specs/`
files as history).

## Where everything sits (as of 2026-08-20)

- **Canonical remote: `katelyn`** — `ssh://git@forgejo.katelyn.michalvanko.dev:3222/michalvankodev/michalvankodev-site.git`.
  `origin` is the STALE old Gitea — do not push there.
- **`main`** @ `ab51815` — in sync with `katelyn/main`. Base for the new branch.
- **This doc is currently UNCOMMITTED** (untracked file in the working tree,
  currently checked out on `redesign/modern-technical`). Carry it to
  `redesign/selected` as the first commit — don't leave it behind.
- **Previous design work — three stacked rounds, all cumulative on**
  `redesign/modern-technical` @ `7932947` (local == katelyn, rebased onto main):
  - v1 conservative polish (`redesign/draft`, tip `f83ce4e`)
  - v2 "Manuscript" (`redesign/from-scratch`, tip `4f741c1`)
  - v3 "Terminal Editorial" + structure + tooling (`redesign/modern-technical`)
  - `specs/redesign-feature-sets.md` (the candidate menu, on the redesign
    branch — NOT on main) maps every feature → round.
- **Port, don't re-derive:** reference implementations live on those branches.
  Look in `redesign/modern-technical` first — it contains everything.
  NOTE the rebase changed SHAs; older citations translate as:

  | old SHA (pre-rebase) | rebased SHA | round |
  |---|---|---|
  | `f83ce4e` (on `redesign/draft`, still valid there) | `99233aa` | v1 polish |
  | `d27a15e` | `3f9e8b1` | v2 Manuscript |
  | `4f741c1` | `6b0a28e` | v2.1 feedback |
  | `ac0667e` | `f1c02d5` | v3 Terminal Editorial |
  | `238b738` | `e3c184c` | v3.1 structure |
  | `868bdb1` | `f5b68d7` | v3.2 tooling |

- **CI:** Forgejo Actions — `.forgejo/workflows/test.yaml` (cargo test, every
  PR), `deploy.yaml` (main), `preview.yaml` (**live preview URL per PR** —
  use it to review each slice). Infra runbook: `specs/CI-deploy.md` (on main).

## Implementation environment notes

- Askama templates compile into Rust — template changes recompile via
  `cargo-watch` (`just dev` runs server + tailwind + decap).
- `just test` · `just export` = SSG via **wget crawl — every page must be
  linked from the homepage** (nav/footer links matter for discoverability,
  e.g. rehomed `/portfolio`).
- `view-transition-name`s are wired through list rows ↔ articles — keep
  them alive through the row restructure.
- `picture_generator` produces multi-format responsive images; lazy-load
  attrs go into `picture_markup_generator.rs`.

## Suggested PR slicing (each lands tested + previewed)

1. Fonts & type system (self-host Exo 2 / Noto Sans / Noto Sans Mono, CSS
   variables, reading size, heading ramp, selection/focus, remove legacy fonts)
2. Site chrome: masthead nav, colophon footer, `<main>`, kickers +
   `section_header` macro, lowercase headings
3. Homepage (hero, plain chapters)
4. List pages (year-grouped archives, 2-col rows, hover)
5. Article page (kicker w/ reading time, cover side plate, TOC rail, drop cap,
   further reading, progress hairline, smooth anchors)
6. Content rendering (light code theme + copy button, blockquote, tables,
   captions, list markers, heading anchors + hover #)
7. Components (talent rows, project plates rework, social cards, contact
   rows, terminal 404, anime.js logo plate on portfolio)
8. Syndication & perf (JSON feed, RSS audit, OG/Twitter, sitemap/robots,
   lazy-load, Caddy cache)
9. A11y + print sweep (skip-link, aria-current, contrast, per-post lang,
   print audit, view-transition polish)

---

(decision record follows)

Working record of decisions for the fresh redesign branch (working name: `redesign/selected`),
based off `main` — not a visual rewrite, but selected features from
`specs/redesign-feature-sets.md` (candidate menu) implemented fresh.

**Status: COMPLETE — all candidate-menu items decided 2026-08-20. Next: branch kickoff.**

## 2026-08-20 · Typography: three type voices

### Decision — MADE 2026-08-20

Three self-hosted voices, identical rendering on every OS:

| Voice | Font | Role |
|---|---|---|
| **Display** | **Exo 2** (variable, latin-ext) | headings, wordmark, site chrome accent — replaces Baloo2 |
| **Reading** | **Noto Sans** (variable, latin-ext) | body text, article content, UI |
| **Machine** | **Noto Sans Mono** (variable, latin-ext) | dates, tags, kickers, code, metadata |

**Rationale:** the metric requirement is satisfied by the Noto family *by
spec* (Noto Sans + Noto Sans Mono share harmonized vertical metrics — no
patching needed between reading and mono, which mix inline most often),
coverage is complete (full latin-ext for Slovak), and the "neutral Noto"
critique is answered by Exo 2 carrying all the personality at display level.
Pragmatic, predictable, zero-metric-drift.

Supersedes the earlier "Noto as fallback only" leaning (kept below for history).

### Constraints

- **Same on every OS** → fonts must be self-hosted webfonts (variable woff2,
  latin subset), not system stacks. System fonts differ in metrics, hinting,
  and coverage between Windows/macOS/Linux.
- **Matched heights** → the reading and mono faces must agree on vertical
  metrics (ascent/descent/line gap, x-height ratio) so a single `font-size`
  in code produces identical line boxes across all three voices.
  Two mechanisms:
  1. Prefer fonts **designed as a pair/superfamily** (matched metrics by design).
  2. Normalize remaining differences by rewriting vertical metrics
     (`OS/2` sTypo*, `hhea` ascent/descent/lineGap) with fonttools so the
     shipped woff2 files share one metric baseline; fixed `line-height`
     per voice in CSS makes it exact.
- **Weight:** ~~Baloo2 stays~~ → resolved: Baloo2 **replaced by Exo 2** as the
  display voice (dir `static/fonts/baloo2/` and its fallback `@font-face`s go away).
- Current legacy font dirs (`cantarell`, `comfortaa`, `orbitron`, `sansation`,
  `ubuntu`) are unused → remove in the typography PR.

### Candidate pairings (reading + mono)

| Pair | Feel | Why it fits | Preview |
|---|---|---|---|
| **Inter + JetBrains Mono** | neutral-technical, modern | JetBrains Mono was designed to complement Inter (both used in JetBrains IDEs); both large x-heights; ubiquitous on dev sites | rsms.me/inter · jetbrains.com/lp/mono |
| **IBM Plex Sans + IBM Plex Mono** | crafted, warm-technical | True superfamily — Sans/Serif/Mono designed together with matched proportions; distinct personality | ibm.com/plex |
| **Source Sans 3 + Source Code Pro** | humanist, bookish | Same designer, x-heights matched by design; Adobe hinting quality; good for long-form reading | adobe-fonts.github.io/source-sans · …/source-code-pro |
| **Fira Sans + Fira Mono** | editorial humanist (Spiekermann) | Designed together for Mozilla; mono is workmanlike, sans has bookish texture | mozilla.github.io/Fira |
| **Geist + Geist Mono** | Swiss, startup-technical | Very current; tight, wide+light aesthetic; smallest files | vercel.com/font |
| **Space Grotesk + Space Mono** | retro-futuristic geometric | Space Grotesk is derived from Space Mono — true designed pair; the "Exo vibe" with the pair guarantee | fonts.google.com/specimen/Space+Grotesk |
| **Chivo + Chivo Mono** | grotesque, techy edge | Omnibus-Type superfamily (designed together); grittier than Inter, more neutral than Space | fonts.google.com/specimen/Chivo |

Mono-only alternatives (pair with any sans): **Commit Mono** (explicitly
metric-neutral, configurable metrics), **Berkeley Mono** (paid), **Martian Mono**
(wide, condensed variable).

### Noto family — considered, verdict: fallback layer, not primary voice

- **For:** vertical metrics harmonized across Sans/Serif/Mono *by design* (Noto's
  stated goal) — the metric requirement is met by spec; broadest coverage
  (full latin-ext, Cyrillic, Greek, …); universally available/predictable
  (Android system face, default Linux fallback).
- **Against:** reads as "no choice made" — one voice wearing three costumes;
  Noto Sans Display is just tighter spacing (no real display cut);
  Noto Sans Mono lacks terminal character; Noto Serif alone is lovely but
  points to a different direction (serif reading voice).
- **Decision:** ~~fallback layer only~~ → **chosen as primary reading+mono
  voices** (see Decision above): metric-by-design guarantee + complete
  latin-ext beat the neutrality critique, with Exo 2 supplying personality.
- Nearest-neighbor alternatives if neutral-but-authored is wanted:
  Source Sans 3 + Source Code Pro (shared humanist lineage), IBM Plex.

### Display-voice candidates (brand slot, vs. keeping Baloo2)

- **Exo 2** (fonts.google.com/specimen/Exo+2) — geometric-futuristic (Natanael
  Gama); Exo 2 is the redrawn, text-legible cut of Exo. Fits the technical
  redesign direction; variable weights, latin-ext ✓. Candidate to replace
  Baloo2 as the display/brand voice. NOT a reading voice (display DNA, no
  mono sibling — Exo 1 excluded entirely, too stylized for text).
- **Space Grotesk** / **Chivo** heavy cuts — if their pair wins reading+mono,
  the display cut can come from the same family for a tighter system.

### Considered and passed

- **Noto family** — full note in its own section above; verdict: fallback layer, not primary voice.
- **Open Sans** — warm humanist, superbly legible, full latin-ext; but no
  designed mono companion (loses the pair guarantee), and Lato/Roboto-tier
  web ubiquity reads "no choice made". If warm humanist is the appeal,
  Source Sans 3 or Fira Sans deliver it with a real paired mono.

### Open questions — typography

- [x] Pairing → **Noto Sans + Noto Sans Mono**
- [x] Display voice → **Exo 2** (Baloo2 retired)
- [ ] Slovak charset spot-check at download time (ľ š č ť ž ô ä ŕ ĺ, dž, " …)
      — all three families ship latin-ext, verify subset actually included

### Implementation sketch

1. Download variable woff2 (latin + latin-ext) for **Exo 2**, **Noto Sans**,
   **Noto Sans Mono** → `static/fonts/{exo2,noto-sans,noto-sans-mono}/`.
2. Metrics: Noto Sans ↔ Noto Sans Mono already agree by design — nothing to
   patch. Exo 2 differs; either normalize its vertical metrics to Noto's
   baseline via a small fonttools script (`scripts/normalize-font-metrics.py`)
   or accept and set explicit `line-height` for display sizes (headings get
   explicit line-heights anyway). Decide during implementation.
3. `styles/input.css`: `--font-display`, `--font-read`, `--font-mono`
   variables; `@font-face` with `font-display: swap`; fallbacks become
   `system sans` / `monospace` + `Noto Color Emoji` (Noto no longer needed
   as a fallback layer — it IS the primary).
4. Remove legacy font dirs (`cantarell`, `comfortaa`, `orbitron`, `sansation`,
   `ubuntu`, **and `baloo2`**) + `--font-tech` stack + `Baloo2 Fallback`/
   `Baloo2 Noto Fallback` @font-face hacks.

## 2026-08-20 · Reading size: fixed, not viewport-scaled

**Decision: adopt the v2 fixed reading size** (single `--text-read`), with refinements:

- **Fixed `1.25rem` / 20px equivalent, line-height 1.75** — expressed in rem, not px,
  so the user's browser font-size preference is respected (the accessibility
  affordance; no custom A−/A+ controls needed — don't rebuild what the
  browser provides).
- **No viewport scaling for reading text.** Rationale: reading depends on
  angular text size + characters-per-line (human constants), not viewport;
  42rem measure at 20px ≈ 65–75 cpl on any display; mobile at ~35–45 cpl
  is already correct at fixed size; vw-based body text ignores user
  font-size preferences and fails at extremes.
- **Fluid type belongs to display only** — Exo 2 headings/hero use `clamp()`
  fluid sizes; coexists with fixed reading size.

Ported from v2 (`--text-read`), amended to rem.

## 2026-08-20 · Drop cap — selected (regular, not cursor block)

**Decision: yes** — drop cap on every article's first paragraph.

- **Variant: the v2-style regular typographic drop cap.** The v3 re-imagining
  (inverted terminal cursor block — mono glyph on `blue-950`) is explicitly
  **rejected**.
- Ref: v2 (`f83ce4e`) — port the float-based implementation, adjust to the
  new type system.
- **Glyph in Exo 2 (display voice)** — suggested so the cap reinforces the
  three-voice system: display letter opening reading-voice text.
- ~3-line height, baseline-aligned to the third line, small right padding;
  decide at implementation: `initial-letter` (now Baseline-wide: Chromium 110+,
  Safari, Firefox 128+) vs classic float (bulletproof everywhere) — or
  `initial-letter` with float fallback.
- Applies to first paragraph of article content only (watch articles that
  open with an image/list); print-friendly as-is.

## 2026-08-20 · Terminal chrome: lowercase headings + path links

**Decision: yes to both** — adopting the v3 terminal vocabulary as part of the
new design:

### Lowercase section headings

- All section/page-level headings lowercase (`/blog` title, chapter titles,
  archive headers, etc.) — calmer, editorial-terminal tone.
- **Via `text-transform: lowercase` in CSS** (visual only) — content keeps its
  authored casing in source/front matter; article titles stay as authored
  unless decided otherwise during implementation.
- Renders in Exo 2 (display voice), continuing the single weight ramp +
  `blue-950` heading rule from v1.

### Terminal-style directory links

- Masthead nav as mono paths: **`/blog /broadcasts /showcase /contact`** —
  machine voice (Noto Sans Mono), wordmark left (Exo 2).
  **`/portfolio` dropped from masthead** (page stays — rehome it: footer link
  or homepage chapter, decide at implementation) · **`/search` not included —
  search postponed** (see Postponed below; add link when it lands).
- **Note: main currently has NO masthead navigation** — this ports the nav
  structure itself from v2 (`f83ce4e`) with v3 path styling (`ac0667e`).
  Structural, not just cosmetic.
- Same pattern extends to page kickers (`~/blog · <date> · n min read`)
  when those features land (kickers ref v3.1).
- Active-state styling pairs naturally with the `aria-current="page"`
  candidate — select together in the a11y bundle.

## 2026-08-20 · Global chrome — selected (4 items, sharp geometry rejected)

- **Colophon footer** — hairline top rule, license line, `index.json` + RSS
  links. Port from v2/v3.
- **`<main>` landmark** in base template — port from v2 (trivial, a11y win).
- **Back-to-top** — `↑ top` link at end of archive lists, `id="top"` anchor on
  header. Port from v3.2.
- **Sharp geometry: REJECTED** — the v3 zero-border-radius doctrine is out.
  **Photos keep a small border radius** (v2 used `rounded-xs` on image markup,
  `rounded-md` on cards). Geometry language = v2: hairline borders *with*
  small radii, not the v3 shaved corners. Note: main today has no radius at
  all — the soft look itself is part of the port.

## 2026-08-20 · List pages — archive grouping, 2-col rows, blue hover

Selected for `/blog`, `/broadcasts`, tag archives (port v3.1 archive + v2.1
row, with restructuring):

- **Archive grouping by year: SELECTED** — sticky year headers + mono post
  counts (`2026 1 · 2025 1 · …`). Port `PostYearGroup` + `group_posts_by_year()`
  from v3.1 (`e3c184c`).
- **Thumbnails: SELECTED, rows restructured to 2 columns** — v3's 3-col row
  (`7.5rem date | 6rem thumbnail | 1fr text`, date stacked above thumb on
  mobile) reads cluttered. New layout: **`date | content`** two columns, with
  the thumbnail inside the content column (top-right of text block, wrapped
  on narrow), ref `blog_post_preview.html` — restructure during port.
- **Row hover: SELECTED, amended** — not v3's `hover:bg-white` flash; **a
  slight blue tint** (`hover:bg-blue-50`, page bg already `blue-50` → use a
  touch stronger like `blue-100/50`) **or a subtle brighten filter** — decide
  exact treatment at implementation. Subtle, not a flash.
- Kicker + lowercase title header (v3 list-page chrome) — covered by section
  header decision above.

### Kickers — site-wide feature

**Kickers are a first-class, site-wide pattern** — every page type gets the
mono kicker line (machine voice, Noto Sans Mono, pink or slate accent —
exact accent at implementation):

- Homepage: `~michalvanko.dev` (bare path, no epitet)
- List pages: `~/blog`, `~/broadcasts`, `~/tags/<tag>`
- Articles: `~/blog · 8 November 2024 · 2 min read` (per-kicker piece below)
- Static pages: `~/showcase`, `~/contact`, `~/portfolio` …

One macro (extend `section_header`) emits kicker + lowercase title + rule
for every page; the machine-voice mono kicker is the unifying chrome of the
site, same discipline everywhere.

## 2026-08-20 · Article cover images — small side plate, not frontispiece

**Context:** cover images are not consistently well-picked — large treatments
amplify weak images. v2's full-width frontispiece plate under the title:
**REJECTED**.

**Decision:** cover shown small, beside the article opening —
**chapter-introduction style, ~500px square or smaller** (think book chapter
leaf plate: text wraps/pairs with a modest image). Alternative if the side
treatment fights the reading measure: **subtle background blend** into the
page header. Pick at implementation; side plate is the default.

- Applies to article pages (`/blog/<slug>`, `/broadcasts/<slug>`).
- **Flag for review:** homepage lead-story hero was selected with a 560px
  cover — revisit its cover size during implementation under this same
  "weak covers shouldn't get big canvases" principle (hero may shrink cover
  or crop tighter).
- List-row thumbnails unaffected (small already).

## 2026-08-20 · Article: sticky TOC rail — selected (side-placed, no image overlap)

**"On this page" TOC rail: SELECTED** — port v3.1 (`e3c184c`):

- Sticky rail (h2/h3 list), IntersectionObserver scroll-spy, anchors generated
  by `extract_headings()` mirroring `parse_markdown` slug ids exactly (the
  TOC contract — port the Rust helper together with it).
- Smooth anchor scrolling behind `prefers-reduced-motion`;
  `scroll-margin-top` on headings. Print-hidden. **Smooth scrolling
  SELECTED as a site-wide anchor behavior** (not just TOC links) — always
  gated by `prefers-reduced-motion`.
- **Placement constraint (user requirement): the rail must live on the side
  where it never overlaps images.** Wide content spans `min(70rem, 95vw)`;
  the rail sits outside that span and is only rendered on viewports wide
  enough to fit both — pick breakpoint at implementation (may be 2xl+ rather
  than v3.1's xl+). Composition note: cover side-plate (left of measure) +
  TOC rail (right margin) pair well together.
- Related: `hello-` slug-cleanup wildcard touches the same slug contract —
  do them in lockstep if that gets selected.

## 2026-08-20 · Article end matter — further reading only, no app chrome

Philosophy stated: **we are building a site, not an app** — no surprise
interactions, minimal post-article surface.

- **Further reading: SELECTED** — tag-matched recommendations as list rows
  (port v2+; note the candidate-menu improvement "rank by tag-overlap count
  then date, not any-match-by-slug" is the sensible default — implement that
  way).
- **Prev/next post navigation: REJECTED** — no boxed prev/next cards after
  articles (further reading covers onward travel better, without
  two-more-boxes fatigue).
- **Keyboard navigation (← newer / → older): REJECTED** — surprise keys are
  app behavior; site readers don't expect or discover it.
- Reading progress hairline: still open (not an end-matter item; decide with
  remaining chrome).

## 2026-08-20 · Content rendering — code blocks light, blockquote ruled roman

- **Code blocks: LIGHT theme.** Dark terminal blocks rejected — page is light,
  code cards stay light. Implementation note: main *currently* renders dark
  (`base16-ocean.dark` — arrived inside the askama-upgrade commit `dce8940`,
  not a deliberate choice) → swap syntect theme to a light one
  (e.g. `InspiredGitHub`), and port the nested-`pre` normalization CSS from
  v3.2 (needed either way — syntect emits its own `<pre>`).
- **Blockquote: blue rule, roman text** — v3 treatment: `border-l-2
  border-blue-500` + indent + `text-slate-700`, **no italics** (long italic
  passages slow reading; sans italics weak on screen; rule + indent already
  signal quotation). Replaces main's pink callout card
  (`bg-pink-50 border-l-4 border-pink-600`).

## 2026-08-20 · Content rendering II — inline code kept, bookish captions, quiet lists

- **Inline code chips: REJECTED** — v3's bottom-border chip treatment not
  wanted; keep main's current inline code as-is
  (`font-mono text-pink-900 border-b-2 border-blue-300 bg-blue-100`).
- **Tables: SELECTED (v3 style)** — mono lowercase header row, 2px navy rule,
  zebra `blue-50` rows. Port wholesale.
- **Figure captions: SELECTED but NOT mono** — captions are image subtitles
  "in a book", not scientific-paper labels: set in reading voice (Noto Sans),
  slightly smaller + muted, not machine voice.
- **Pink list markers: REJECTED** — markers blue; list text `blue-950` when
  highlighted, otherwise dark blue (slate/navy)
  (user: "text should be blue and dark blue if not highlighted").
- **Heading anchor ids: SELECTED** — server-side slugs (already part of the
  TOC contract from the rail decision; `extract_headings()` ports with it).
  Note: covers ids for anchors/TOC — the *hover `#` copy affordance* remains
  a separate unselected candidate (Copy button on code blocks also still
  open).

## 2026-08-20 · Final sweep — progress hairline, a11y; content-model & wildcards closed

- **Reading progress hairline: SELECTED** — 2px pink, fixed top, gated by
  `prefers-reduced-motion` (no motion for RM users); `print:hidden`. The one
  piece of reading chrome that survived the "site, not app" filter — it's
  passive (no interaction, no surprise), unlike the rejected kbd nav.
- **Homepage intro block: REJECTED** — no v2/v3 front-matter bio + talent
  definition-rows block on the homepage; bare `~michalvanko.dev` kicker +
  hero + chapters carry it. (Talent rows elsewhere already selected where
  they apply.)
- **Series/collections: REJECTED** — content-model additions (series, subtitle,
  last-updated) not taken now.
- **Tag hubs: REJECTED** (tag topic hubs, tag index page, `/now`, `/uses`,
  draft verification — all deferred/not taken in this iteration).
- **A11y sweep: SELECTED** — skip-to-content link (first tab stop → `<main>`
  landmark, which this redesign adds), `aria-current="page"` on active nav
  item (pairs with masthead active state), contrast audit (bump slate-500
  metadata on blue-50 toward slate-600 where <4.5:1), per-post `lang` from
  front matter (EN+SK content, correct screen-reader voice).
- **Wildcards: REJECTED** — footnote sidenotes, EPUB export, adaptive code
  themes, on-this-page TOC for archives, `hello-` slug cleanup: none taken
  now. (Slug cleanup stays flagged: must move in lockstep with TOC work if
  ever revived.)

**The candidate menu is now fully decided. Nothing remains open.**

## 2026-08-20 · Final batch — reader affordances, syndication, perf, DESIGN.md

### Reader affordances (inline vanilla JS only, no deps)

- **Copy button on code blocks: SELECTED** — header bar on each block,
  click → clipboard, brief ✓ confirmation. Works with the light theme.
- **Heading anchor affordance: SELECTED** — `#` appears on heading hover,
  copies deep link (ids already exist from TOC contract). Pairs with copy
  button as the site's only two JS affordances beyond scroll-spy.
- **Back-to-top on articles: SELECTED, as a small button on the TOC rail** —
  not a footer link (lists keep their footer link). Caveat: TOC rail renders
  only on wide viewports → narrow screens have no article back-to-top;
  acceptable (mobile offers its own scroll-to-top), noted as accepted trade.

### Syndication & metadata

- **JSON Feed (`/feed.json`): SELECTED** — serde, alongside RSS.
- **RSS full-text audit: SELECTED** — test asserting body presence + absolute
  URLs in feed items.
- **OG/Twitter card completion: SELECTED** — `article:published_time`,
  `article:tag`, `twitter:card=summary_large_image` per post.
- **sitemap.xml + robots.txt: SELECTED** — sitemap generated from post/project
  lists at export time; robots.txt already exists (update to reference
  sitemap).

### Performance

- **Lazy-load content images: SELECTED** — `loading="lazy"` +
  `decoding="async"` in picture markup, except hero (`fetchpriority=high`).
  Touches `picture_markup_generator.rs`.
- **Self-host fonts: CONFIRMED** (already the typography decision — Noto Sans,
  Noto Sans Mono, Exo 2 as local woff2 subsets).
- **Cache headers: SELECTED, scoped honestly.** Yes it makes sense even
  (especially) for SSG: static files get no automatic policy — without
  `Cache-Control` browsers revalidate every asset every visit or use
  heuristic caching. BUT: **generated image filenames are size-suffixed, not
  content-hashed** (`_1080x384.png`; `output.css` likewise), so `immutable`
  is UNSAFE (returning visitors could hold stale assets past a redeploy).
  Scoped decision: Caddy-side config (production serves statics via Caddy;
  `Caddyfile-preview` in repo is the template) — HTML always revalidates,
  assets moderate `max-age` + etag/last-modified revalidation. `immutable`
  only if filename hashing is added to `picture_generator` later.

### Polish & process

- **View-transition polish: SELECTED** — authored morphs for hero→article
  (cover side plate) and list-row→title; names already wired, keep them
  alive through the row restructure; respect `prefers-reduced-motion`.
- **DESIGN.md rewrite: SELECTED** — this doc becomes the source of truth;
  rewrite `DESIGN.md` to the new system at branch kickoff (specs/ files stay
  as history).

## 2026-08-20 · Infra & the animated logo — keep anime.js

Infra items ride along with selected features (year grouping, reading time,
`extract_headings`, 404 `url` field — all already noted under their features;
plus view-transition-name preservation through the row restructure).

**EXCEPTION — anime.js logo stays.** v2's removal of anime.js + the animated
logo from portfolio is REJECTED:

- The animated logo page/section is hand-crafted work the author values —
  it stays as-is (`portfolio.html` `#logo-container` 320px block + timeline
  animation).
- **Presentation idea (user):** display the logo "in a similar manner as the
  article cover" — i.e. as a side plate beside the portfolio intro, like the
  chapter-intro cover treatment, instead of a standalone 320px block.
  Decide exact composition during portfolio rework (logo is interactive —
  mouse-reactive? — so the plate must keep pointer events; reduced-motion
  gates the animation itself).
- This is the one sanctioned JS-animation exception to the quiet-site
  philosophy: it's authored content, not chrome.

## 2026-08-20 · Print styles — all selected

**Print: SELECTED wholesale** (v3.2 + inherited):

- `print:hidden` on header, footer, TOC rail, further-reading, progress bar,
  year headers (sticky chrome), back-to-top
- White page background; portfolio contact links print raw URLs
  (`print:inline` swap) — inherited, kept
- Worth auditing at implementation against the *new* elements (cover side
  plate, definition rows) — they should print as flowing text, not vanish.

Also stands: reduced-motion respect everywhere (smooth scrolling already
  gated; social hovers are color-only, no transform). Reduced-motion audit
  candidate effectively satisfied by construction — verify once at the end.

## 2026-08-20 · Components II — projects kept (rework), socials, contact rows, 404

- **Project cards: KEPT, rework flagged** — borderless plates (hairline cover,
  title link, description, classification + tags footer) stay the direction,
  but flagged for dedicated work during implementation (user: "we will need
  to work on them") — layout, typography and content upgrade.
- **Social cards: SELECTED** — white cards w/ blue hover border (replacing
  main's loud `pink-200` bg); all four brand hover effects preserved (twitch
  shift, youtube scale, instagram dim, tiktok glitch). Port v3.
- **Contact page: definition rows** — consistent with talent rows, but
  **reading voice, not mono**, and each row acts as a **simple button/link
  to the social site** (no pill-button redesign beyond that).
- **Terminal 404: SELECTED** — `$ GET /path` → `404: page not found` with real
  requested URL; port v3 (`NotFoundPage.url` field + template).

## 2026-08-20 · Components — talent rows selected

- **Talent/skill cards → definition rows: SELECTED** — v3 treatment: icon cue
  (32–40px, `fill-blue-950`) + semibold `blue-950` heading + reading-voice
  description at reading size, hairline `border-b` separators, no box/border/bg.
  Replaces main's boxed card (`border rounded-sm bg-white p-3`, 64px icon,
  `text-sm leading-tight` description). Rationale: description gets the space
  chrome was wasting; semantic term→definition honesty (not clickable —
  shouldn't look like a UI unit); row language now consistent site-wide.

## 2026-08-20 · Homepage kicker — decided

**`~michalvanko.dev` — no epitet.** The bare path in mono (machine voice);
the name is the brand, sections speak for themselves. v3's "personal journal"
rejected (too narrow); "personal site" rejected (too broad); minimalism won.

## 2026-08-20 · Homepage — lead hero selected, chapters de-numbered

- **Lead story hero: SELECTED** — latest post promoted: date, 3xl title,
  clamped excerpt, 560px cover, `read →` affordance; "more writing" lists
  the rest. Port from v3.1.
- **Numbered chapters: REJECTED in numbered form** — homepage keeps section
  chapters but **plain**: `blog`, `broadcasts`, `showcase` — no `01/02/03`
  mono numbers, no 2px navy rules from v2/v3. Terminal restraint carries the
  hierarchy instead.
- **`section_header` macro: keep** — the dedup mechanism is still worth
  porting for the (now plain) section headers.
- **Section header improvements: SELECTED** — v3's kicker + lowercase title
  style (`~/blog` kicker, lowercase heading, strong navy rule) applies to the
  plain homepage chapters too — so headers are plain *wording*, not plain
  *styling*: `~/blog`-style mono kicker + lowercase Exo 2 title + hairline
  rule underneath.

## 2026-08-20 · Selection & focus — selected (selection amended to pink)

Both already exist on **main** (from v1) — this is a **tweak, not a port**:

- **`::selection`** — currently `bg-blue-200 text-blue-950` → **change to brand
  pink**: `bg-pink-200 text-blue-950` (pink brand ramp lives in the theme:
  `--color-pink-200: #ffcff7`; pairs with the pink accent family — list markers,
  progress bar, tags — that the terminal chrome picks also lean on).
  Check contrast of blue-950 on pink-200 at implementation; bump to pink-100
  if needed.
- **`:focus-visible`** — already `outline-2 outline-offset-2 outline-blue-500`;
  **keep as-is** ✓ (or pink to match selection — decide at implementation;
  blue outline on blue-tinted page is the safer contrast call).

## 2026-08-20 · One heading rule — selected

**Decision: adopt the v1 rule** — all headings:

- **color `blue-950`** (single heading color site-wide)
- **single weight ramp** — one progression (e.g. extrabold → bold → semibold
  mapped to size tiers), no ad-hoc weights
- **`font-extrabold` at body sizes eliminated** (heavy black banned from text)

Status on main: *mostly true already* — section-heading classes in
`input.css` use a blue-950 bold/semibold ramp, and `strong` is semibold.
**One straggler found:** `templates/project_list.html` h1 still uses
`font-extrabold` (text-4xl/6xl). Implementation = audit + fix stragglers +
encode the ramp as named size classes so future templates can't drift.
- Applies to the new system: headings render in **Exo 2** (display voice) —
  the ramp becomes Exo 2 weight tiers, lowercase per the terminal-chrome
  decision.

## Feature selection

**Selected so far** (from `specs/redesign-feature-sets.md` candidate menu):

- [x] Drop cap on article first paragraph (v2 regular variant, not v3 cursor block)
- [x] Lowercase section headings (v3)
- [x] Terminal-style masthead path nav (v2 structure + v3 styling)
- [x] Fixed reading size — folded into typography decision above
- [x] `::selection` in brand pink (on main as blue; tweak) · `:focus-visible` kept
- [x] One heading rule — blue-950, single weight ramp, extrabold eliminated (mostly on main; 1 straggler found)
- [x] Masthead nav finalized: `/blog /broadcasts /showcase /contact` (portfolio rehomed, search postponed)
- [x] Colophon footer (hairline, license, index.json + RSS links) · `<main>` landmark · back-to-top
- [x] Homepage: lead story hero (v3.1) · plain chapters `blog / broadcasts / showcase` (no numbers) · `section_header` macro + v3 kicker/lowercase/rule styling · bare `~michalvanko.dev` kicker
- [x] List pages: year-grouped archives w/ sticky headers + mono counts · thumbnails kept, rows restructured to 2 cols (`date | content`) · hover = subtle blue tint (not white flash)
- [x] Kickers as site-wide feature — mono kicker line on every page (`~/path · context…`), unified via one macro
- [x] Article covers: small side plate (~≤500px, chapter-intro style) or bg blend; full-width frontispiece rejected; homepage hero cover size flagged for review
- [x] Sticky "on this page" TOC rail — outside the image span (never overlaps wide images), scroll-spy, `extract_headings()` anchor contract
- [x] Article end matter: further reading ONLY (tag-overlap ranked) — prev/next cards rejected, keyboard nav rejected ("site, not app")
- [x] Code blocks: LIGHT syntect theme (revert base16-ocean.dark that snuck onto main) + nested-pre normalization · Blockquote: blue rule + indent, roman text (pink card & italics out)
- [x] Content rendering II: tables v3 (mono lower header, navy rule, zebra) · captions bookish in reading voice (NOT mono) · inline-code chips rejected (keep main) · pink markers rejected (blue markers, blue-950 text) · heading anchor ids yes (via TOC contract)
- [x] Smooth anchor scrolling site-wide, gated by `prefers-reduced-motion`
- [x] Components I: talent/skill definition rows (v3; boxed cards out)
- [x] Components II: project plates kept w/ rework flag · social cards v3 (white/blue hover, brand effects) · contact definition rows (reading voice, simple link buttons, not mono) · terminal 404 (v3, real URL)
- [x] Print styles: wholesale (v3.2 print:hidden set + white bg + raw-URL swaps); reduced-motion satisfied by construction, verify at end
- [x] Infra: rides along w/ features · anime.js animated logo KEPT (v2 removal rejected) — logo displayed cover-style as a side plate on portfolio, one authored-animation exception
- [x] Final batch: copy button on code blocks · hover `#` anchor copy · back-to-top as small TOC-rail button · JSON feed · RSS full-text audit · OG/Twitter completion · sitemap+robots · lazy-load images · self-host fonts (confirmed) · cache headers (Caddy, revalidation — immutable unsafe w/o hashing) · view-transition polish · DESIGN.md rewrite
- [x] Final sweep: reading progress hairline (RM-gated) · a11y sweep (skip-link, aria-current, contrast, per-post lang) · homepage intro block REJECTED · series REJECTED · tag hubs/`/now`/`/uses` REJECTED · wildcards REJECTED
- [x] Geometry: v2 soft look — hairline borders + small radii (photos `rounded-xs`); v3 zero-radius rejected

**Passed:** reader measure/size controls (A−/A+), fixed-px reading size (amended to rem), homepage intro block, series/collections, tag hubs + tag index + `/now` + `/uses`, wildcards (sidenotes, EPUB, adaptive code themes, archive TOC, `hello-` slug cleanup)

**Postponed** (decided later, not rejected):

- [ ] **Search cluster** — `/search.json` index + `/search` page (ref v3.2),
      `/` shortcut to focus search, match highlighting + diacritics folding,
      indexing projects + portfolio, 404 search integration, OpenSearch
      descriptor. Masthead gains `/search` when this lands.
