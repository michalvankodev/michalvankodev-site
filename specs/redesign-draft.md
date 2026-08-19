# Redesign draft — michalvanko.dev

First pass redesign on branch `redesign/draft`. Goals from DESIGN.md: **clean, book-like readability**; light-only; blue as brand, pink/purple as accents; no distracting effects. This is an initial direction to react to — everything here is negotiable.

## What changed

### 1. Reading measure + type scale (the core change)

Old body text jumped 16 → 20 → **28px** across breakpoints on a 1024px (64rem) measure. At 28px that measure works, but mid-range sizes left lines at 100+ characters. New setup targets a classic book column (~65–70 chars):

| | Old | New |
|---|---|---|
| Body (base/md/lg) | 16 / 20 / 28px | 16 / 18 / **20px** |
| Body line-height (lg) | 36px (1.29 — tight) | **32px (1.6)** |
| `--spacing-read` (measure) | 64rem (1024px) | **44rem (704px)** |
| `--spacing-note` (blockquote/code) | 60rem | 40rem |
| Article h1 (lg) | 60px | 48px |
| Article h2/h3 (lg) | 36 / 30px | 30 / 24px |

- Article paragraphs are now `hyphens-auto` + justified (justified without hyphenation creates rivers).
- Verified live: 20px/32px body, 704px column, ~70 chars/line.

### 2. One heading rule (was: color + size drift)

Previously headings mixed `blue-900` and `blue-950`, `font-bold`/`font-semibold`/`font-extrabold` across pages. Now:

- **All headings: `text-blue-950`** (ink navy — closest to book ink while staying brand).
- Page h1: `text-3xl md:text-4xl font-bold`.
- Section h2 (everywhere — index, lists, socials, showcase, portfolio, "Further reading"): `text-2xl md:text-3xl font-bold text-blue-950`.
- In-article h2–h4: semibold, stepping 30→24→20px at lg.
- `font-extrabold` eliminated site-wide.

### 3. Post previews calmed down (was: competing with headings)

Preview titles were `md:text-3xl` (30px) — nearly the size of section headings. Now `text-lg md:text-xl bold`, excerpt `md:text-lg` with `leading-relaxed`, **left-aligned** (justified short excerpts looked gappy). Metadata (tags/date) unified to `text-sm md:text-base`, separator changed `|` → `·`.

### 4. Tags unified (was: two different tag styles)

Article-page tags were italic, pink-800, underlined (global link style); preview tags were pink-950, no underline. Now both are `text-pink-950 no-underline #tag`, `text-sm md:text-base`.

### 5. Header (masthead)

- Logo moved **left** (was right-aligned; it's a book spine/masthead now), styled `text-blue-950 font-medium no-underline` — previously it rendered as a pink underlined link.
- Back link (sub-pages) sits on the right.
- `mb-5` → `mb-6`.

### 6. Cards: one radius, calmer internals

- All cards now `rounded-md` (was mix of `rounded`/`rounded-sm`/`rounded-md`). Images/iframe: `rounded-md` (was `rounded-sm`).
- Talent/skill card headings: `font-semibold text-lg md:text-xl` (was `font-medium md:text-2xl`).
- Descriptions: `leading-normal` (was `leading-tight` — cramped for reading).
- Project card: title/classification → `blue-950`; classification de-emphasized (`text-base md:text-lg`, was same size as title); description no longer justified.

### 7. Quiet a11y/interaction additions

- `:focus-visible` — 2px `blue-500` outline (keyboard users see where they are).
- `::selection` — pink-200 background with blue-950 text (brand-consistent highlight).

### 8. Fixed along the way

- `templates/portfolio.html`: "Showcase" `<h2>` was closed with `</h1>` — mismatched tag fixed.
- `styles/output.css` regenerated (it's tracked but the tailwind watcher wasn't running).

## Open questions

1. **Body size at lg (20px)** — the old 28px was deliberately generous. 20px/1.6 is the classic book setup, but do you want something in between (e.g. 21–22px)? The token is one line (`--text-readxl`).
2. **Measure width (44rem)** — comfortable for text, but images/codes that used to span 70rem now tower over a narrower column. Currently figures still use `max-w-image` (70rem) so they extend past the text — intentional "book plate" feel. Keep?
3. **Header logo position** — moved to the left for a conventional masthead. The old right-aligned logo was a deliberate quirk. Preference?
4. **Social cards (`bg-pink-200`)** — the loudest surface on the site. Keep as personality, or soften to `pink-100`/`blue-100` now that everything else is quieter?
5. **Justify + hyphens** — hyphenation requires the `lang` attribute (present) and per-browser dictionaries; English hyphenates well. If you dislike hyphens, we should drop justify too (ragged-right is the other bookish option).
6. **Article h1 size** — dropped 60→48px. Still makes a strong chapter opening. OK?
7. **Contact/portfolio pill buttons** (`rounded-full border-2 border-blue-500`) — untouched; they're the most "web-app" elements left. Redesign them (e.g. quiet list rows) or keep as intentional contrast?
8. **DESIGN.md sync** — DESIGN.md still describes the old values. Once this direction is approved, update it to be the source of truth again. Until then this file is the working spec.
9. **Post previews keep `md:text-lg` excerpt** — previews are now much quieter. Does the homepage still have enough visual interest, or is it too flat now?
10. **TikTok glitch / social hover effects** — kept as-is per your confirmation (hover-only, brand mimic, external cards only).

## Not changed (deliberately)

- Palette (all blue/pink/purple tokens), Baloo2 font, light-only stance.
- View transitions and all `view-transition-name`s (kept 1:1 through the refactor).
- Social card hover effects, blockquote/table/inline-code styling (values only rescaled where the type scale changed).
- Footer, contact page structure, portfolio logo animation, egg-fetcher page.
- `max-w-image` (70rem) for figures/tables/embeds.

## Notes for review

- Dev server on :3080 picks template changes via cargo-watch, but the **tailwind watcher was not running** — I rebuilt with `just tailwind_build`. If you run `just dev` both are handled. Hard-refresh (Ctrl+Shift+R) — browsers heuristically cache `output.css`.
- All 14 cargo tests pass; `cargo check` clean.
