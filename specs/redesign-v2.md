# Redesign v2 — "Manuscript" (from-scratch)

Branch: `redesign/from-scratch` (stacked on `redesign/draft`). This is a ground-up exploration: templates rewritten from zero against the existing Rust models — no old markup kept. The goal was to answer *"what would this site look like if designed today, from the content outward?"* while honoring the standing brief: **clean, book-like, readable, light-only, blue brand + pink/purple accents, no distracting effects.**

## The concept

**The site is a publication, not an app.** A personal journal set like a printed manuscript:

- **Serif reading text** (`Iowan Old Style → Palatino → Georgia` — system fonts, zero download) for everything you *read*: article bodies, excerpts, bio, taglines.
- **Baloo2** (the brand voice) stays for everything you *look at*: masthead, headings, nav, metadata, tags.
- One reading size everywhere — **19px / 1.7 line-height**, no responsive scaling. A printed page doesn't change size; neither does this.
- **Hairline aesthetic**: thin slate rules to separate, one strong 2px navy rule under each section heading (`border-b-2 border-blue-950`) — the bookish "chapter rule".
- **Numbered chapters** on the homepage (01 Writing · 02 Broadcasts · 03 Selected work · 04 Elsewhere), set as small serif italics in pink.
- **Drop caps** open every article — the signature move.
- **Table-of-contents listings**: posts are journal index rows (`date | title + excerpt + tags`) with hairline separators. No cards, no thumbnails in lists.
- Pink is reserved almost entirely for *interactive and meta* elements (kickers, tags, links); purple stays as the visited state. The blue-tinted paper (`blue-50`) is unchanged.

## What was built

### Chrome
- **Masthead**: wordmark left, real nav right (`Writing · Broadcasts · Showcase · Portfolio · Contact`) — the site previously had *no* navigation, only a logo and contextual back links. Sub-page back links are dropped (nav makes them redundant; `HeaderProps` model is untouched).
- **Footer**: colophon pattern — hairline, license line, RSS.
- `<main>` landmark added in `base.html`.

### Pages
- **Homepage** — front matter (kicker "Personal journal", name, serif bio, three talent definition-rows) → numbered chapter sections. Showcase plates span wider (`max-w-image`); text stays at `max-w-read`.
- **Reading lists** (`/blog`, `/broadcasts`, tag archives) — kicker "Index", title, tag row, strong rule, ToC rows. The old sidebar (socials + showcase) is gone: a reading list should be a list.
- **Article** — kicker date, 48px title, tags, optional full-width cover plate (the thumbnail now earns its place as the chapter frontispiece), serif body with drop cap, "Further reading" as ToC rows, quiet "← all posts" tail.
- **Portfolio** — CV front matter (name, serif italic tagline, inline contact links), body, then Experience/Education as definition rows with thumbnails, mono technology pills, showcase plates. **The anime.js logo animation was removed** (decorative JS + external script; the page is now quiet and dependency-free).
- **Contact** — definition rows (icon + link) instead of pill buttons; added a short human sentence below.
- **404** — kicker "Error 404", serif statement, three escape links.
- **Social cards** — restyled from loud `pink-200` boxes to `pink-50` hairline cards; **all brand hover effects kept** (twitch shift, youtube scale, instagram dim, tiktok glitch) per earlier decision.
- **Project cards** — borderless "plates": cover image, title link, serif description, classification in letterspaced small caps + pink tags.

### Typographic system (`.article-body`, applies to portfolio body & egg-fetcher too)
| Element | Treatment |
|---|---|
| Body | serif 19px/1.7, justified, `hyphens-auto`, 40rem measure |
| First paragraph | drop cap, serif semibold ~63px navy |
| h2/h3/h4 | Baloo2 bold/semibold navy, 30/24/20px at md+ |
| Blockquote | serif italic, 3px pink left rule, no background box |
| Code block | mono, white bg, hairline border, rounded-md |
| Inline code | mono chips (pink-900 on blue-100) |
| Tables | hairline rules, small-caps Baloo2 header row, 2px navy rule, zebra `blue-50` |
| Lists | serif, pink `::marker` |
| Figures | rounded-md, `shadow-xs`, captions serif italic slate |

### Kept intentionally
- All `view-transition-name`s (header, footer, previews, titles/dates/tags) — morphing preserved.
- Palette tokens, Baloo2 self-hosting, light-only, `theme-color`, print rules, RSS/Mastodon/OG meta, `rel="prefetch"` on post links, `admin.html`, egg-fetcher.

## Open questions

1. **Serif body** — the biggest leap. Georgia/Palatino render differently per OS (intentional: zero webfont download). If you want *identical* rendering everywhere, we'd self-host a serif (e.g. Source Serif, ~40KB woff2). Keep system serif or self-host?
2. **19px everywhere (no scaling)** — deliberate print stance. OK on mobile? (Serif at 19px is fine, but confirm on a real phone.)
3. **Drop caps** — love-it-or-hate-it. Currently on every article's first paragraph. Keep / drop / larger?
4. **Numbered homepage chapters** — editorial flourish or gimmick?
5. **ToC rows without thumbnails** — images now appear only as article covers. Miss them in lists?
6. **Nav labels** — "Writing" vs "Blog"? (Route is `/blog`.) "Broadcasts" could be "Streams"/"Videos".
7. **Social cards on pink-50** — quieter than before but still the most colorful surface. Alternative: white cards with pink hover border only.
8. **404 handler title** still says "This page doesn't exists" (grammar bug lives in `src/pages/not_found.rs`, not the template) — fix in a follow-up?
9. **Skill/talent rows have no icons on portfolio Experience** — company thumbnails show when the model has them; otherwise rows are text-only. Fine?
10. **`line-clamp-2` on excerpts** — two lines max. Want three for longer summaries?

## Round 2 — feedback adjustments (v2.1)

- **Drop cap: kept** — loved. 🎉
- **Thumbnails restored to list rows** — grid is now `date | thumbnail | content` on md+ (7.5rem / 6rem / 1fr); on mobile the date spans the top and thumb sits left of the text at 4.5rem. Thumbnails are click-through links to the post; the default letter-tile is now responsive (`aspect-[3/4]` instead of fixed 180×240).
- **Reading size bumped** 19px → **20px** (`--text-read: 1.25rem`, line-height 1.7 → 34px). Applies to article body, excerpts, and the homepage bio.
- **Hyphenation removed** — words stay whole (`hyphens-auto` dropped from article paragraphs and the bio). Text remains justified; at this measure (~64 chars at 20px) justification without hyphens is acceptable, but if word-spacing rivers start bothering you, say so and we flip to ragged-right (`text-left`).

## Verified (round 2)

- Article body: 20px/34px, `hyphens: manual`, justified, 640px measure.
- List rows (34 posts on `/blog`): 3-column grid (`120px 96px …`), date right-aligned, thumbnail loading and aspect-preserved.

- `cargo check` clean, 14/14 tests pass, `just tailwind_build` regenerated `output.css`.
- Live walkthrough (fresh browser session): homepage (nav 6 links, serif bio 19px, chapter numbers), article (drop cap 62.7px, justified+hyphenated 19px/32.3 body at 640px measure, 1080px cover plate), `/blog` list, `/portfolio` (serif italic tagline, mono chips), `/contact`, 404.
