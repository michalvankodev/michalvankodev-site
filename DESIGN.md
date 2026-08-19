# DESIGN.md — michalvanko.dev

Stylistic specification for [michalvanko.dev](https://michalvanko.dev). Use this file as the single source of truth when building or modifying UI. It is written for AI coding agents and humans alike: values are concrete, mapped to Tailwind v4 utilities defined in `styles/input.css`.

---

## Design Philosophy

This is a **personal site with a blog**. The primary function is **reading content** — the site should feel like a book.

1. **Content first.** Chrome, navigation, and decoration are minimal. The article body is the hero.
2. **Calm and quiet.** No dark mode, no popups, no scroll-jacking, no autoplay, no aggressive animation. Effects must serve comprehension, not entertainment.
3. **Generous, book-like typography.** Large reading size, justified paragraphs, a fixed comfortable measure (~64rem).
4. **Light only.** The site is permanently light-themed. Blue-tinted background instead of stark white. Never introduce `dark:` variants or theme toggles.
5. **Friendly, not corporate.** One rounded display typeface (Baloo2) carries the personality; color does the rest.

## Brand Palette

Light **blue** is the personal brand color (backgrounds, headings, structure). **Pink** and **purple** are accents (links, tags, highlights). All palettes are custom overrides in `styles/input.css` under `@theme`.

### Blue — primary brand (structure, headings, backgrounds)

| Token | Hex | Usage |
|---|---|---|
| `blue-50` | `#f1f7fe` | **Page background** (`<body>`), header background |
| `blue-100` | `#e1effd` | Inline code background, table header background |
| `blue-200` | `#bddefa` | Light decorative fills |
| `blue-300` | `#82c3f7` | Inline code border |
| `blue-400` | `#42a6f0` | Hover states (sparingly) |
| `blue-500` | `#1789e0` | Link hover color |
| `blue-600` | `#0a6cbf` | Reserved |
| `blue-700` | `#0a569a` | Reserved |
| `blue-800` | `#0c4980` | Figure captions |
| `blue-900` | `#103e6a` | **Article headings** (h1–h4) |
| `blue-950` | `#0b2746` | **Section headings**, icon fills, darkest brand text |

### Pink — primary accent (links, tags, metadata)

| Token | Hex | Usage |
|---|---|---|
| `pink-50` | `#fff4fd` | Blockquote background |
| `pink-200` | `#ffcff7` | Social card background |
| `pink-600` | `#d722a9` | Blockquote left border |
| `pink-800` | `#92166e` | **Default link color** |
| `pink-900` | `#771859` | Inline code text |
| `pink-950` | `#500238` | **Tags**, post dates, metadata |

### Purple — secondary accent (visited state only)

| Token | Hex | Usage |
|---|---|---|
| `purple-700` | `#441E73` | **Visited link color** (`visited:text-purple-700` inside `article`) |

Purple is currently reserved almost exclusively for the "you already read this" signal. Keep it that way unless a deliberate second accent role is added.

### Neutrals — Tailwind slate (body text, borders)

| Token | Usage |
|---|---|
| `slate-950` | Article body text, list text |
| `slate-800` | Preview/secondary body text, card descriptions |
| `slate-600` | Blockquote text, timestamps, muted labels |
| `slate-300` | Horizontal rules (`<hr>`) |
| `slate-200` | Card borders, table borders |
| `slate-100` / `slate-50` | Table zebra striping |

**White** (`bg-white`) is used for card surfaces sitting on the blue-50 page background.

## Typography

### Typeface

- **Baloo2** — variable font, weights 400–800, self-hosted (`/fonts/baloo2/*.woff2`, `font-display: swap`).
- Fallbacks are metric-adjusted local fonts (`Baloo2 Noto Fallback`, `Baloo2 Fallback`, then system UI sans).
- One family for everything. No monospace import; code blocks also use Baloo2 unless a syntax theme provides one.

### Type scale

Sizes are mobile-first (`base` → `md:` ≥768px → `lg:` ≥1024px). Article text scales **up** on large screens for book-like reading.

| Role | Classes | Rendered size (base / md / lg) | Color / weight |
|---|---|---|---|
| Article title (h1) | `text-3xl md:text-4xl lg:text-6xl font-bold` | 30 / 36 / **60px** | `text-blue-900` |
| Article h2 | `text-xl md:text-2xl lg:text-4xl font-semibold` | 20 / 24 / **36px** | `text-blue-900` |
| Article h3 | `text-lg md:text-xl lg:text-3xl font-semibold` | 18 / 20 / 30px | `text-blue-900` |
| Article body (p, li) | `md:text-lg lg:text-readxl` (`--text-readxl: 1.75rem/2.25rem`) | 16 / 20 / **28px** | `text-slate-950`, weight 400 |
| Section heading (h2, index/lists) | `text-2xl md:text-4xl font-bold` | 24 / **36px** | `text-blue-950` |
| Post preview title (h3) | `text-lg md:text-3xl font-bold` | 18 / 30px | link `text-blue-950` |
| Metadata / tags / dates | `text-sm` … `md:text-base lg:text-lg` | 14–18px | `text-pink-950` |
| Figure caption | `text-sm md:text-base lg:text-lg italic` | 14–18px | `text-blue-800` |
| Inline code | `text-sm md:text-base lg:text-xl` | 14–20px | `text-pink-900` on `bg-blue-100` |

### Typographic rules

- Body paragraphs and previews are **justified** (`text-justify`); headings and metadata are left-aligned.
- `strong` renders as `font-medium` (not bold-black) to stay quiet.
- Links: `underline underline-offset-2`, color `text-pink-800`, hover `text-blue-500` with transition. Inside articles links get `visited:text-purple-700`. Headings used as links (`no-underline`) are `text-blue-950`.
- Heading spacing (article): h2 `mt-8/mb-6` at md, growing to `mt-12/mb-8` at lg; paragraphs `my-8` at md+.

## Spacing & Layout

Tailwind default scale (base unit 0.25rem). Custom layout tokens defined in `@theme`:

| Token | Value | Meaning |
|---|---|---|
| `max-w-note` | `60rem` (960px) | Narrow note width — blockquotes |
| `max-w-read` | `64rem` (1024px) | **Reading measure** — all article content |
| `max-w-image` | `min(70rem, 95vw)` | Figures, tables, iframes, video embeds |
| `max-w-maxindex` | `100rem` | Homepage & listing grids |

### Page structure

- **Header**: minimal — logo link `@michalvankodev` right-aligned, optional back link left, `bg-blue-50`, `mb-5`, separated from content by `hr.border-slate-300`. Header/footer persist across view transitions (`view-transition-name`).
- **Footer**: `hr` + centered license line + RSS icon (`fill-blue-950`), `my-4`.
- **Article page**: `h1` + tag/date row inside `max-w-read mx-auto px-4`; body in `.article-body` (all children constrained to `max-w-read mx-auto`).
- **Homepage**: `max-w-maxindex` grid — `lg:grid-cols-2`, `xl:grid-cols-[1fr_2fr]`, `gap-x-32`; sections About / Blog / Socials / Showcase / Broadcasts.
- **List pages** (`/blog`): `lg:grid-cols-[2fr_1fr]` — post list left, socials + showcase right.
- Horizontal gutters: `m-5` / `mx-5` on sections; content `px-4` on mobile.
- Section separation: `hr.border-slate-300` with `m-5`/`my-8`; never heavy boxes.

## Components

Existing patterns with exact recipes. Reuse these; don't invent variants.

### Links (global, `@layer base`)

```html
<a class="underline underline-offset-2 text-pink-800 hover:text-blue-500">
```
Inside `article`: add `visited:text-purple-700`. Headings-as-links: `text-blue-950 no-underline`.

### Tags

Inline list, pink, no underline, `#Capitalized`:

```html
<a href="/blog/tags/rust" class="text-pink-950 no-underline">#Rust</a>
```
List item wrapper: `inline-block mx-0.5 p-0.5 md:text-xl`.

### Blog post preview (the core list unit)

Grid at `sm+`: thumbnail aside (180×240) + title + justified excerpt + footer (`tags | date`).

```html
<article class="sm:grid sm:grid-cols-[max-content_1fr] sm:gap-4 md:gap-x-8 break-inside-avoid">
  <aside class="row-span-3 self-center float-start mr-3 mb-3 sm:float-none">…thumbnail…</aside>
  <h3 class="text-lg font-bold mb-1 md:text-3xl">
    <a class="text-blue-950 visited:text-purple-700 no-underline">Title</a>
  </h3>
  <section class="text-base leading-7 text-slate-800 md:text-xl text-justify">excerpt…</section>
  <footer class="text-sm md:text-base lg:text-lg mt-3">#tags | <time class="text-pink-950">date</time></footer>
</article>
```
Separated by `hr class="border-slate-300 my-5 md:my-8"`.

### Cards

- **Talent card**: `flex border border-slate-200 rounded-sm bg-white p-3`, icon `fill-blue-950 h-12 w-12 md:h-16 md:w-16`, heading `text-lg font-medium md:text-2xl`, description `text-sm md:text-lg text-slate-800`.
- **Project card**: `border border-slate-200 rounded-md bg-white p-4 break-inside-avoid`; title `text-xl md:text-2xl font-semibold text-blue-900`; classification heading same; tags `text-pink-950`. Showcase grid: `grid gap-6 md:grid-cols-2 md:grid-rows-[masonry] xl:grid-cols-3`.
- **Social card**: `block no-underline border border-slate-200 rounded-md bg-pink-200 m-4 p-4 max-w-[392px]`, icon `fill-blue-950 h-7 w-7`, heading `text-lg font-medium text-blue-950`.

### Code

- **Inline**: `rounded-sm border border-blue-300 px-1 py-0.5 bg-blue-100 text-pink-900 text-sm md:text-base lg:text-xl`.
- **Block**: `pre { p-4 my-1 overflow-auto text-sm max-w-read mx-auto }`, inner `rounded-sm shadow-xs max-w-note`. Syntax highlighting via syntect (server-side).

### Blockquote

```html
<blockquote class="mx-6 py-1 px-2 bg-pink-50 lg:mx-auto max-w-note border-l-4 border-pink-600">
```
Interior text `text-slate-600`.

### Tables

`table-auto border-collapse border border-slate-200 rounded-sm max-w-image mx-auto`; `thead bg-blue-100`; `tbody bg-slate-50`; `tr even:bg-slate-100`; cells `py-0.5 px-2 md:py-2 md:px-5 border-b`.

### Figures & media

- `figure` → `p-4`; `img` → `rounded-sm shadow-md mx-auto lg:max-w-image` (responsive `<picture>` markup auto-generated by `picture_generator`).
- Video embed: `m-4 mx-auto max-w-image aspect-video`, `rounded-sm shadow-md`.
- Captions: `figcaption mt-2 text-center text-sm italic text-blue-800`.

### Horizontal rule

`<hr class="border-slate-300 m-5">` — the universal section separator. Thin, quiet.

## Elevation

The design is **flat with borders**; shadows are rare and small:

| Level | Utility | Used on |
|---|---|---|
| 0 (default) | borders `border-slate-200` | cards |
| 1 | `shadow-xs` | code blocks |
| 2 | `shadow-md` | images, iframes |

Never introduce large/colored shadows for decoration.

## Motion & Interaction

Keep motion **subtle and functional**:

- **View Transitions** are enabled (`@view-transition { navigation: auto }`) with stable `view-transition-name`s on header, footer, post titles/dates/tags, and post previews — content morphs between list and article. Preserve these names when refactoring.
- Link hover: simple color change (`hover:transition hover:text-blue-500`).
- Social cards have playful brand hovers (twitch offset shadow, youtube scale, instagram dim, tiktok glitch). The TikTok glitch is **intentional** — it mimics TikTok's brand aesthetic and only fires on hover. These are the **only** decorative effects on the site and are confined to external-link cards. Do not spread this treatment elsewhere.
- No scroll-triggered animations, no parallax, no spinners on content.

## Breakpoints & Responsive Rules

Tailwind defaults: `sm` 640 / `md` 768 / `lg` 1024 / `xl` 1280.

- Mobile: single column, thumbnails float, text at base size.
- `md`: type scale steps up (`md:text-lg` body).
- `lg`: full reading size (`lg:text-readxl`), grids appear, content constrained to `max-w-read`.
- `xl`: homepage 3-track grid, wider gutters (`gap-x-32`).

## Do's and Don'ts

**Do**

- Use the custom blue/pink/purple tokens from `styles/input.css` — never raw hex or Tailwind's default blue/pink.
- Keep article content inside `max-w-read`, centered.
- Use `hr.border-slate-300` to separate list items and sections.
- Use `view-transition-name` on persisting/navigating elements.
- Justify long-form text; left-align everything else.
- Self-host fonts and images; no third-party requests for styling.
- Prefer semantic HTML (`article`, `time`, `figure`) — the design leans on it.

**Don't**

- No dark mode, no `dark:` classes, no theme toggle — ever.
- No gradients, glows, or large shadows.
- No sticky headers, floating action buttons, cookie banners, or modal popups.
- No animation on content elements; no autoplay media.
- Don't use white (`#fff`) as a page background — the page is `bg-blue-50`.
- Don't widen article text beyond `max-w-read` or shrink it below the responsive scale above.
- Don't introduce new accent colors; pink and purple are the complete accent set.

## Known Inconsistencies (improvement backlog)

The current style is not set in stone. These observed deviations are candidates for cleanup — resolve toward this spec when touching related code:

1. **Heading color split**: section headings use `text-blue-950` while article headings use `text-blue-900`; post preview h3 links use `text-blue-950`. Consider unifying (e.g., `blue-950` for display/section, `blue-900` for in-article hierarchy) and documenting the rule.
2. **Radius drift**: cards mix `rounded-sm` (talent) and `rounded-md` (project, social); images use `rounded-sm`; thumbnails `rounded-xs`. Pick one card radius.
3. **Metadata size drift**: tags/dates render 14–18px across contexts; define one metadata size.
4. **Type scale jump**: body goes 16→28px across breakpoints; verify readability targets on mid-size screens (~1024–1200px).
