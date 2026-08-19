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

## Verified

- `cargo check` clean, 14/14 tests, `just tailwind_build` rebuilt.
- Live: article (mono kicker, Baloo2 48px h1, system-ui 20px/35px ragged body, 47px mono cursor-block drop cap, mono tags, hairline cover), home (mono navy `/blog` nav, mono chapter numbers, sans bio), `/blog` list (mono dates, thumbs, sans excerpts, hover flash), portfolio, contact, 404 (`$ GET /definitely-not-here` → `404: page not found` with real path).
