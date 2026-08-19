# Redesign v3 — "Terminal Editorial" (modern technical blog)

Branch: `redesign/modern-technical` (stacked on `redesign/from-scratch` v2.1). The brief evolved: rethink "read like a book" into **a modern technical publication** — still content-first, still quiet, but speaking the visual language of the craft: terminals, code, paths, monospace.

## The concept

Three type voices, one job each:

| Voice | Font | Role |
|---|---|---|
| **Brand** | Baloo2 (self-hosted) | Masthead wordmark, headings — the personality |
| **Workhorse** | `system-ui` sans | Everything you read: bodies, excerpts, bios |
| **Machine** | `ui-monospace` | Everything structural: dates, tags, nav, kickers, captions, footer |

Serif is gone. Sans reads as "modern technical"; mono carries the identity. The result looks like a well-set README that grew up.

### Signature moves

1. **Cursor-block drop cap** — the beloved drop cap, re-imagined for the terminal age: first letter of every article renders as a mono glyph on an inverted navy block (`blue-950` bg, `blue-50` letter) — like a selection/cursor. Verified: 47px mono on navy.
2. **Path navigation** — nav items are literal routes in mono: `/blog /broadcasts /showcase /portfolio /contact`. No labels to invent; the URLs *are* the labels.
3. **Machine metadata** — dates, tags (`#rust` lowercase mono), section numbers (`01`–`04`), kickers (`~/blog`, `~/portfolio — curriculum vitæ`, `~ michalvanko.dev — personal journal`) all in mono pink-700.
4. **Terminal 404** — a shell session: `$ GET /definitely-not-here` / `404: page not found`, with the actual requested path (added `url` to the `NotFoundPage` model; also fixed the "This page does not exists" grammar bug in the title).
5. **Sharp geometry** — zero border-radius everywhere. Images and code blocks get hairline `slate-200` borders instead of shadows (shadow only survives on social hover effects).
6. **Ragged right** — body text is left-aligned (dropped justify). Modern technical blogs don't justify; this also makes the no-hyphenation stance costless.
7. **Row hover flash** — post list rows and contact rows transition to white on hover. Quiet, functional affordance, not decoration.
8. **Lowercase section headings** (`writing`, `further reading`, `experience`) — terminal lowercase aesthetics; post titles stay as-written.

### Kept from v2 (earned)

- Drop caps on articles · date|thumb|content list rows with thumbnails · 20px reading size (`--text-read`, now at 1.75 line-height) · numbered homepage chapters · strong 2px navy section rules · cover-image frontispieces · real masthead nav · `view-transition-name`s · palette · light-only · social brand hovers · `rel="prefetch"` · print rules.

### Changed from v2

- Serif → system sans for reading; Baloo2 headings stay.
- Justified → left-aligned; hyphenation already gone.
- Rounded → sharp corners throughout.
- Pink-200/pink-50 social cards → white cards with blue hover border.
- Italic serif blockquote → sans with 2px blue-500 left rule.
- Serif italic figcaptions → mono captions.
- Table headers → mono lowercase.
- Measure widened 40rem → 42rem (sans needs a touch more line length).
- `::selection` → blue-200/blue-950 (was pink — blue reads more "text cursor").
- Inline code → bottom-border chip (border-b-2) instead of full box.
- Footer → mono colophon.

### Constraint discovered

Syntax highlighting uses syntect's **InspiredGitHub** theme with inline-styled span colors baked into the HTML — so **dark code blocks are impossible** without replacing the highlighting pipeline (custom CSS classes or a dark syntect theme + regenerating styles). Code blocks therefore stay white with hairline borders. If you want dark-mode code blocks later, that's a Rust-side change: swap the syntect theme and add matching `pre` styling.

## Open questions

1. **Mono nav `/path` style** — love or too cute? Alternative: plain `writing · broadcasts · …` mono labels.
2. **Cursor-block drop cap** — better than v2's serif drop cap, or should the classic serif one return as a contrast moment?
3. **system-ui reading** — renders as SF/Segoe/Roboto per platform. Want a self-hosted sans (Inter) for cross-platform consistency? (~100KB woff2.)
4. **Lowercase headings** — terminal authenticity vs. looking "undone". Easy to flip back.
5. **Row hover → white** — subtle on the blue-50 paper. Keep? Also applied to contact rows.
6. **Kicker flavors** — currently mixed: `~ michalvanko.dev — personal journal` (home), `~/blog` (lists), `~/portfolio — curriculum vitæ`. Too many flavors, or nice variety?
7. **Section numbers** `01`–`04` — kept from v2, now mono. Still earning their place?
8. **Tags lowercase** (`#rust` not `#Rust`) — more authentic, but changes the capitalized convention from earlier versions. OK?
9. **Dark code blocks** — worth the Rust-side highlighting change? (See constraint above.)
10. **Measure 42rem** — sans at 20px ≈ 66 chars. Comfortable, or go wider (44–46rem) like some technical blogs?

## Round 2 — structural upgrades (v3.1)

Higher-scale changes to page structure and the rendering pipeline:

1. **Sticky "on this page" TOC** (articles, xl+) — headings already receive slug ids in `parse_markdown`; new `extract_headings()` in `src/filters/markdown.rs` mirrors that slug logic (first raw text event, verbatim) so TOC anchors always match. TOC is a mono rail on the right with IntersectionObserver scroll-spy (vanilla, ~20 lines, highlights current section in pink).
2. **Reading time** — words/220, shown in the article kicker (`~/blog · 8 November 2024 · 2 min read`).
3. **Prev/next post navigation** — `get_post_neighbors()` finds date-order neighbors within the same segment; boxed newer/older links under the article (grid, right-aligned older).
4. **Archive-style list pages** — posts grouped by year (`group_posts_by_year`), sticky year headers with mono post counts (`2026 1 · 2025 1 · 2024 3 · … · 2019 2`), still sticky through scroll. Applies to /blog, /broadcasts, and tag filters.
5. **Homepage lead story** — latest post promoted to a hero (kicker + date + 3xl title + clamped excerpt + cover at 560px, "read →" link); the "more writing" chapter lists the rest. Verified: hero = newest post, list starts at #2.
6. **`section_header` macro** (`templates/components/section_header.html`) — the repeated chapter-header pattern (number + lowercase title + all-link) is now one macro used by index and showcase sections.
7. **Anchor UX** — `scroll-behavior: smooth` behind `prefers-reduced-motion`, `scroll-margin-top` on article headings.

### Structural notes
- `PostListTemplate.posts` was replaced by `grouped_posts: Vec<PostYearGroup>` — both list handlers (blog + broadcasts) updated.
- Askama gotcha hit twice: `match` cannot take a `.first()` method-call expression (use `for` + `loop.first`), and `{% call %}` requires `{% endcall %}` even with no body.
- `ParseResult<BlogPostMetadata>` is not `Clone`; neighbor lookup consumes the iterator (`nth`) instead.
- The slugifier slugs the heading's *first text event* verbatim, so "Hello …" → `hello-` (trailing hyphen). TOC matches body ids exactly, which is the contract that matters.

### Verified (round 2)

- TOC anchors == rendered heading ids; scroll-spy activates after jump (`#on-to-the-next-one` highlighted).
- Kicker reading time; prev/next cards with real neighbors.
- `/blog`: 8 year groups, 34 rows, sticky year headers.
- Homepage: lead story hero (288px cover at md), writing chapter starts at post #2.
- 14/14 tests, `cargo check` clean.

## Round 3 — reader tooling (v3.2)

1. **Dark terminal code blocks** — swapped the syntect theme from `InspiredGitHub` to `base16-ocean.dark`. The "impossible" from the v3 constraint turned out to be a one-line theme swap: syntect bakes span colors inline, and the nested-`pre` seam is handled with `pre pre { background: transparent !important }` (CSS `!important` beats the inline style) so the block is uniformly `blue-950` with hairline `slate-800` border. Verified: `wget --no-convert-links…` renders on navy with ocean palette spans.
2. **Client-side search** (`/search`) — new `src/pages/search.rs` serves a `/search.json` index (title, segment, slug, ISO date, tags, 220-char plain-text excerpt via pulldown-cmark) and a page with a `$ grep -i …` prompt. Vanilla JS: token matching across title(×3)/tags(×2)/excerpt(×1), score-sorted, 120ms debounce, `?q=` prefill + autofocus. Verified: `?q=keyboard` → 4 matches; `rust cargo` → 1 match (the crate post). Nav gains `/search`; footer links `index.json` for SSG discoverability.
3. **Keyboard navigation** — ←/→ jump to newer/older posts (guards for inputs and modifier keys; hint under the article footer). Verified: dispatching ArrowRight navigated to the predicted older post.
4. **Reading progress hairline** — 2px pink-600 bar fixed at the top of articles. Verified: 0% → 54% mid-scroll.
5. **Print styles** — `print:hidden` on header, footer, TOC, post-nav, further reading, progress bar, year headers, back-to-top; white page background in print. Articles print as clean text.
6. **Back-to-top** — `↑ top` mono link at the end of archive lists; `id="top"` anchor on the header.

### Verified (round 3)

- `/search.json` returns the full index; search page prefills, filters, scores, renders.
- Code blocks: outer `rgb(11,39,70)`, inner transparent, spans colored.
- Progress 0→54%; ArrowRight navigates; 14/14 tests, `cargo check` clean.

- `cargo check` clean, 14/14 tests, `just tailwind_build` rebuilt.
- Live: article (mono kicker, Baloo2 48px h1, system-ui 20px/35px ragged body, 47px mono cursor-block drop cap, mono tags, hairline cover), home (mono navy `/blog` nav, mono chapter numbers, sans bio), `/blog` list (mono dates, thumbs, sans excerpts, hover flash), portfolio, contact, 404 (`$ GET /definitely-not-here` → `404: page not found` with real path).
