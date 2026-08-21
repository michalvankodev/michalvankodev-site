# DESIGN.md — michalvanko.dev

Stylistic specification for [michalvanko.dev](https://michalvanko.dev). Single
source of truth when building or modifying UI. Written for AI coding agents and
humans alike; values are concrete, mapped to Tailwind v4 utilities defined in
`styles/input.css`.

> **Status:** this file describes the **target system** of the selected
> redesign (`specs/redesign-feature-set-decided.md` is the decision record).
> The redesign lands as a stack of small PRs; until they merge, parts of this
> document run ahead of the code. When a PR lands, code and this file must
> agree.

---

## Design philosophy

A **personal site with a blog**, dressed as a calm, light **technical
publication**. Terminal vocabulary in the chrome (paths, mono kickers,
lowercase headings), book-like comfort in the reading surface.

1. **A site, not an app.** No surprise interactions, no keyboard navigation
   tricks, no popups. The two JS affordances beyond scroll-spy are copy
   buttons; the one sanctioned animation exception is the hand-authored
   anime.js logo on `/portfolio`.
2. **Three type voices, everywhere.** Display (Baloo 2), reading (Adwaita Sans),
   machine (Noto Sans Mono). Content speaks in reading voice; structure and
   metadata speak in machine voice; personality comes from display voice.
3. **Calm and quiet.** Light only, no dark mode. Subtle hover tints, no
   flashes. Motion is functional (view transitions, scroll-spy) and gated by
   `prefers-reduced-motion`.
4. **Fixed, human reading size.** `1.25rem` / line-height `1.75` — rem, not
   viewport units, so browser font-size preference is respected. Fluid type
   belongs to display sizes only (`clamp()`).
5. **Terminal chrome, book body.** Path-style nav and kickers, lowercase
   headings — but article body is generous book typography: justified
   paragraphs, fixed measure, drop cap opening.

## Brand palette

Light **blue** is the personal brand color (backgrounds, headings, structure).
**Pink** and **purple** are accents (links, tags, highlights). All palettes are
custom overrides in `styles/input.css` under `@theme` — never raw hex, never
Tailwind's default blue/pink.

### Blue — primary brand

| Token | Hex | Usage |
|---|---|---|
| `blue-50` | `#f1f7fe` | **Page background** |
| `blue-100` | `#e1effd` | Inline code bg, table header bg, row-hover tint layer |
| `blue-200` | `#bddefa` | Light decorative fills |
| `blue-300` | `#82c3f7` | Inline code border |
| `blue-400` | `#42a6f0` | Hover states (sparingly) |
| `blue-500` | `#1789e0` | Link hover, focus outline, blockquote rule |
| `blue-900` | `#103e6a` | Reserved (legacy in-article ramp) |
| `blue-950` | `#0b2746` | **All headings**, icon fills, darkest brand text |

### Pink — primary accent

| Token | Hex | Usage |
|---|---|---|
| `pink-200` | `#ffcff7` | `::selection` background |
| `pink-600` | `#d722a9` | Reading progress hairline |
| `pink-800` | `#92166e` | **Default link color** |
| `pink-900` | `#771859` | Inline code text |
| `pink-950` | `#500238` | Tags, post dates, metadata |

### Purple — secondary accent

`purple-700` (`#441E73`) — **visited link color only** (`visited:text-purple-700`
inside `article`).

### Neutrals — Tailwind slate

`slate-950` article body · `slate-800` preview/secondary text · `slate-600`
muted labels · `slate-300` horizontal rules · `slate-200` hairline card
borders. White (`bg-white`) for card surfaces on the blue-50 page.

## Typography

### Three self-hosted voices (identical on every OS)

| Voice | Family | Role |
|---|---|---|
| **Display** | **Baloo 2** (variable 400–800, latin + latin-ext) | headings, wordmark, chrome accents, drop cap glyph |
| **Reading** | **Adwaita Sans** (variable opsz+wght, self-subsetted latin+latin-ext) | body, article content, UI, captions |
| **Machine** | **Noto Sans Mono** (variable, latin-ext) | dates, tags, kickers, nav paths, code, metadata |

- All three self-hosted as woff2 under
  `/fonts/{baloo2,adwaita-sans,noto-sans-mono}/`, `font-display: swap`.
- CSS variables `--font-display`, `--font-read`, `--font-mono`; Tailwind
  `font-display`, `font-read`, `font-mono` utilities. Fallbacks: `system
  sans` / `monospace` + `Noto Color Emoji`.
- Adwaita Sans (GNOME's Inter derivative) is the reading voice: variable
  opsz 14–32 + wght 100–900, subsetted from the upstream TTF to a single
  170 KB woff2 covering latin + latin-ext — no unicode-range split needed.
  Its vertical metrics differ from Noto Sans Mono's (code lines next to
  prose keep their own rhythm; cosmetic at reading sizes). Baloo 2 gets
  explicit `line-height`s at display sizes instead of metric patching.
- **Slovak charset verified complete** in all three voices (Baloo 2 union:
  `ľščťžŕĺďň` in latin-ext, `ôä „ “ … –` in latin; Adwaita Sans verified in
  the single subset file; Noto by design). **Overlock and Cotham failed the
  check** — any display or reading candidate must pass the full
  `ľščťžôäŕĺďňĽŠČŤŽÔÄŔĹĎŇ„“…–` set before adoption.
- No dead font dirs (`cantarell`, `comfortaa`, `orbitron`, `sansation`,
  `ubuntu`), no `--font-tech`, no fallback-metric `@font-face` hacks.

### Reading size & scale

| Role | Treatment |
|---|---|
| Article body | **fixed `1.25rem`, line-height 1.75** (`--text-read`), justified |
| Display sizes | Baloo 2, `clamp()` fluid ramp, explicit line-heights |
| Metadata/kickers | mono voice, small (`text-sm`-tier) |
| Code | mono voice, reading-size relative |

### One heading rule

- **All headings `text-blue-950`** — single heading color site-wide.
- **Single weight ramp** (Baloo 2): extrabold (800) → bold (700) → semibold
  (600) mapped to size tiers. Named size classes encode the ramp; templates
  cannot drift.
- `font-extrabold` at body sizes banned. `strong` = semibold.

### Lowercase headings, via CSS

Section/page-level headings render **lowercase** (`text-transform: lowercase`
in CSS); source content keeps authored casing. Article titles stay as
authored.

### Drop cap

Every article's first paragraph opens with a **regular typographic drop cap**
(~3-line, float-based, `initial-letter` where supported with float fallback),
glyph in **Baloo 2**. Never the terminal-cursor-block variant.

### Selection & focus

- `::selection` — **`bg-pink-200 text-blue-950`** (brand pink).
- `:focus-visible` — `outline-2 outline-offset-2 outline-blue-500`, kept.

## Terminal chrome

### Masthead nav — path links

`/blog /broadcasts /showcase /contact` in **Noto Sans Mono** (machine voice),
wordmark in Baloo 2 left. `/portfolio` is rehomed (footer link — the page stays).
No `/search` (postponed). Active item pairs `aria-current="page"` with active
styling.

### Kickers — site-wide pattern

Every page type gets a mono kicker line (machine voice, pink or slate accent):

- Homepage: `~michalvanko.dev` (bare path, no epitet)
- List pages: `~/blog`, `~/broadcasts`, `~/tags/<tag>`
- Articles: `~/blog · 8 November 2024 · 2 min read`
- Static pages: `~/showcase`, `~/contact`, `~/portfolio`

One macro (`section_header`) emits kicker + lowercase Baloo 2 title + hairline
rule for every page. Same discipline everywhere.

### Global chrome

- **Colophon footer** — hairline top rule, license line, `index.json` + RSS links.
- **`<main>` landmark** in base template; skip-to-content link is the first
  tab stop.
- **Back-to-top** — `↑ top` link at end of archive lists; on articles, a small
  button on the TOC rail (wide viewports only — accepted trade).
- **Reading progress hairline** — 2px pink, fixed top, `prefers-reduced-motion`
  gated, `print:hidden`.

## Spacing & layout

| Token | Value | Meaning |
|---|---|---|
| `max-w-note` | `60rem` | Narrow note width (blockquotes) |
| `max-w-read` | `64rem` | **Reading measure** — article content (42rem inner measure target ≈ 65–75 cpl) |
| `max-w-image` | `min(70rem, 95vw)` | Figures, tables, iframes — the wide span |
| `max-w-maxindex` | `100rem` | Homepage & listing grids |

- TOC rail lives **outside** `max-w-image`, only on viewports wide enough to
  fit both (never overlaps wide images; breakpoint likely 2xl+).
- Cover side plate (~≤500px, chapter-intro style) sits beside the article
  opening, left of measure; TOC rail in the right margin. Never a full-width
  frontispiece.
- Gutters `m-5`/`mx-5` on sections; content `px-4` on mobile.

## Lists & archives

- **Year-grouped archives**: sticky year headers + mono post counts
  (`2026 1 · 2025 3`), `group_posts_by_year()`.
- **Two-column rows**: `date | content` — thumbnail inside the content column
  (top-right of text block, wrapped narrow). Keep `view-transition-name`s
  alive through the row structure.
- **Hover**: subtle blue tint (`blue-100/50` over `blue-50` bg) — never a
  white flash.

## Article page

- Kicker with **reading time** (`n min read`).
- **Sticky "on this page" TOC rail** — h2/h3 list, IntersectionObserver
  scroll-spy, server-side anchor ids (`extract_headings()` mirroring
  `parse_markdown` slugs — the TOC contract). Print-hidden.
- **Cover as small side plate** — weak covers never get big canvases.
  Homepage hero cover is under the same principle (crop tighter / shrink).
- **End matter: further reading only** — tag-overlap-ranked recommendations
  (overlap count, then date). No prev/next cards, no keyboard nav.
- **Heading anchors**: server-side ids; `#` appears on hover, copies deep
  link. **Copy button** on every code block. These plus scroll-spy are the
  only JS affordances.
- Smooth anchor scrolling site-wide, gated by `prefers-reduced-motion`;
  `scroll-margin-top` on headings.

## Content rendering

- **Code blocks: LIGHT theme** (`InspiredGitHub`-class syntect theme — the
  dark `base16-ocean.dark` was accidental). Nested-`pre` normalization CSS
  (syntect emits its own `<pre>`). Inline code unchanged:
  `text-pink-900 border-b-2 border-blue-300 bg-blue-100`.
- **Blockquote**: `border-l-2 border-blue-500` + indent + `text-slate-700`,
  **roman** (no italics, no pink card).
- **Tables**: mono lowercase header row, 2px navy rule, zebra `blue-50` rows.
- **Figure captions**: reading voice (Adwaita Sans), smaller + muted — book
  subtitles, not paper labels.
- **Lists**: blue markers; text blue-950 when highlighted, otherwise
  dark blue/slate. Never pink markers.

## Components

- **Talent rows** (not cards): icon cue 32–40px `fill-blue-950` + semibold
  `blue-950` term + reading-voice description, hairline `border-b` separators,
  no box/border/bg.
- **Project plates**: borderless — hairline cover, title link, description,
  classification + tags footer. Dedicated rework during implementation.
- **Social cards**: white cards, blue hover border. Brand hovers preserved:
  twitch shift, youtube scale, instagram dim, tiktok glitch — the only
  decorative effects, confined to external-link cards.
- **Contact rows**: definition rows in **reading voice** (not mono), each row
  a simple link/button to the social site.
- **Terminal 404**: `$ GET /path` → `404: page not found`, real requested URL.
- **anime.js logo** on `/portfolio`: kept, displayed cover-style as a side
  plate beside the intro (pointer events live; animation reduced-motion
  gated). The one authored-animation exception.

## Geometry

Hairline borders (`border-slate-200`-tier) **with small radii** — photos
`rounded-xs`, cards `rounded-md`. Never zero-radius doctrine; never large
rounded blobs. Flat with borders; shadows rare and small (`shadow-xs` code
blocks, `shadow-md` images).

## Motion & interaction

- **View transitions** (`navigation: auto`) with stable `view-transition-name`s:
  header, footer, post rows ↔ article titles/dates/tags, hero → cover plate.
  Authored morphs; preserve names through refactors; respect
  `prefers-reduced-motion`.
- Link hover: color change only. Social hovers: the four brand effects above.
- No scroll-jacking, no parallax, no spinners on content, no autoplay.

## Syndication & performance

- **JSON Feed** (`/feed.json`) alongside RSS; RSS carries **full text +
  absolute URLs** (tested).
- **OG/Twitter complete** per post: `article:published_time`,
  `article:tag`, `twitter:card=summary_large_image`.
- **sitemap.xml** generated from post/project lists at export; robots.txt
  references it.
- Content images `loading="lazy" decoding="async"`; hero
  `fetchpriority="high"` (in `picture_markup_generator.rs`).
- Caddy cache headers: HTML revalidates always; assets moderate `max-age` +
  etag/last-modified. **No `immutable`** — filenames are size-suffixed, not
  content-hashed.

## Accessibility & print

- Skip-to-content link → `<main>`; `aria-current="page"` on active nav;
  contrast ≥ 4.5:1 (bump slate-500 metadata toward slate-600 on blue-50);
  per-post `lang` from front matter (EN+SK).
- **Print**: white background; `print:hidden` on header, footer, TOC rail,
  further reading, progress bar, year headers, back-to-top; portfolio contact
  links print raw URLs; cover plate and definition rows print as flowing
  text. Reduced-motion audit satisfied by construction — verify once at the
  end.

## Breakpoints & responsive rules

Tailwind defaults: `sm` 640 / `md` 768 / `lg` 1024 / `xl` 1280 / `2xl` 1536.

- Mobile: single column, mono nav wraps, reading size fixed, no TOC rail.
- `lg`: grids appear, wide span `max-w-image` in use.
- `2xl`: TOC rail + cover side plate composition (pick exact breakpoint during
  implementation).

## Do's and don'ts

**Do**

- Use custom tokens from `styles/input.css` — never raw hex or default
  Tailwind blue/pink.
- Keep article text inside `max-w-read`, centered; reading size fixed in rem.
- Use the machine voice for anything structural/metadata; reading voice for
  prose; display voice for headings/wordmark only.
- Use `hr`-tier hairlines and the `section_header` macro for page chrome.
- Use `view-transition-name` on persisting/navigating elements.
- Self-host fonts and images; no third-party requests for styling.

**Don't**

- No dark mode, no `dark:` classes, no theme toggle — ever.
- No `font-extrabold` at body sizes; no ad-hoc heading weights/colors.
- No dark code blocks; no pink list markers; no pink blockquote card; no
  italics for blockquotes/captions.
- No prev/next article cards, no keyboard article nav, no surprise
  interactions — we build a site, not an app.
- No `immutable` cache headers without content-hash filenames.
- Don't widen article text beyond `max-w-read`; don't viewport-scale body
  text.
- Don't remove or replace the anime.js logo (authored work — keep).
