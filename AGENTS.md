# AGENTS.md

## Project Overview

Personal website and blog hosted at https://michalvanko.dev. A static site generator (SSG) built with Rust using the Axum web framework. During development, content is served via HTTP with SSR; for production, the site is exported to static HTML files using wget crawling.

**Technology Stack:**
- **Backend:** Rust, Axum web framework, Tokio async runtime
- **Templating:** Askama (compile-time template engine)
- **Styling:** Tailwind CSS v4
- **Content:** Markdown with YAML front matter (gray_matter), parsed with pulldown-cmark
- **CMS:** Decap CMS (formerly Netlify CMS) for content management
- **Build Tool:** Just (command runner, similar to make)
- **Deployment:** Caddy reverse proxy, rsync to remote server

## Architecture

### Directory Structure

```
├── src/                    # Rust source code (~40 files, ~2090 lines)
│   ├── main.rs             # Entry point, sets up Axum server with static file serving
│   ├── router.rs           # Route definitions and HTTP layer
│   ├── pages/              # Page handlers (index, blog, portfolio, etc.)
│   ├── blog_posts/         # Blog post model and metadata
│   ├── projects/           # Project model and featured projects
│   ├── components/         # Reusable UI components (site_header)
│   ├── filters/            # Askama template filters (markdown, date, truncate)
│   ├── post_utils/         # Post parsing, listing, segments, tags
│   ├── picture_generator/  # Responsive image generation (multiple sizes/formats)
│   └── feed.rs             # RSS feed generation
├── templates/              # Askama HTML templates
│   ├── base.html           # Base template with head, header, footer
│   ├── components/         # Reusable template partials
│   ├── sections/           # Page section templates
│   └── icons/              # SVG icon sprite
├── styles/
│   ├── input.css           # Tailwind source with custom theme
│   └── output.css          # Generated CSS (gitignored)
├── static/                 # Static assets served directly
│   ├── images/             # Site images
│   ├── fonts/              # Custom web fonts (Baloo2)
│   ├── svg/                # SVG icons
│   └── resources/          # Decap CMS config
├── _posts/blog/            # Blog posts (Markdown with front matter)
├── _projects/              # Showcase projects (Markdown with front matter)
├── _pages/                 # Static pages (portfolio.md)
├── generated_images/       # Auto-generated responsive images (gitignored)
├── dist/                   # SSG output folder (gitignored)
└── target/                 # Rust build artifacts (gitignored)
```

### Key Entry Points

- `src/main.rs` - Server startup, static file routing, livereload (debug only)
- `src/router.rs` - All route definitions, maps URLs to page handlers
- `src/pages/index.rs` - Homepage, demonstrates async data loading pattern

### Data Flow

1. **Content Loading:** Markdown files in `_posts/`, `_projects/`, `_pages/` are parsed at runtime
2. **Front Matter:** YAML metadata extracted via `gray_matter` crate
3. **Template Rendering:** Askama templates receive structs with data
4. **Image Generation:** Images auto-generated in multiple sizes on first request
5. **SSG Export:** `wget` crawls running server, saves HTML to `dist/`

## Code Conventions

### Rust Patterns

- **Module Organization:** Each domain has a `mod.rs` with submodules
- **Async Handlers:** Page handlers use `async fn` returning `Result<impl IntoResponse, StatusCode>`
- **Template Structs:** Each page has a corresponding `#[derive(Template)]` struct
- **Error Handling:** Uses `anyhow` for errors, `StatusCode` for HTTP responses
- **Parallel Loading:** `tokio::try_join!` for concurrent data fetching

### Template Patterns

- **Inheritance:** Templates extend `base.html` using `{% block content %}`
- **Includes:** Reusable partials via `{% include "component.html" %}`
- **Filters:** Custom filters in `src/filters/` (e.g., `{{ content|markdown }}`)
- **Configuration:** `askama.toml` sets template directory and whitespace handling

### Naming Conventions

- **Files:** snake_case for Rust files, kebab-case for templates
- **Routes:** kebab-case URLs (`/blog`, `/showcase/m-logo-svg`)
- **Front Matter:** snake_case fields in YAML

### Configuration Files

| File | Purpose |
|------|---------|
| `Cargo.toml` | Rust dependencies, package metadata |
| `askama.toml` | Template directory config |
| `.prettierrc` | JS/JSON formatting (trailing commas, 2-space, single quotes) |
| `.nvmrc` | Node.js version: `lts/*` |
| `.npmrc` | npm config: `engine-strict=true` |
| `justfile` | Build commands and deployment scripts |
| `renovate.json` | Dependency update automation |
| `static/resources/config.yml` | Decap CMS configuration |

## Setup Requirements

### Prerequisites

- **Rust:** Stable toolchain (uses edition 2021)
- **Node.js:** LTS version (for Tailwind CSS, Decap CMS)
- **Just:** Command runner (`cargo install just` or system package)
- **cargo-watch:** For development hot reload (`cargo install cargo-watch`)

### Environment Variables

- `PORT` - Server port (default: 3080)
- `RUST_LOG` - Logging level (default: `axum_server=debug,tower_http=debug`)
- `TARGET` - Build target mode (used in `just prod`)

### Development Setup

```bash
# Install dependencies
npm install

# Run development server (starts all services in parallel)
just dev
# This runs: server_dev, tailwind watch, decap_server

# Or run individually:
just server_dev    # Rust server with hot reload
just tailwind      # CSS watch mode
just decap_server  # Local CMS backend
```

### Common Commands

```bash
just test          # Run Rust tests
just test_watch    # Run tests with watch mode
just prod          # Run server in release mode
just export        # Generate static site to dist/
just deploy        # rsync dist/ to remote server
```

## Important Files

### Configuration

- `Cargo.toml` - Rust dependencies (axum, askama, pulldown-cmark, etc.)
- `justfile` - All build/dev/deploy commands
- `static/resources/config.yml` - Decap CMS collections and fields
- `styles/input.css` - Tailwind theme customization (colors, fonts, spacing)

### CI/CD

- `.gitea/workflows/test.yaml` - Runs `cargo test` on push/PR
- `.gitea/workflows/release.yaml` - Builds release, runs SSG export, uploads `dist/` artifact

### Content Structure

- `_posts/blog/` - Blog posts with YAML front matter (title, segments, published, date, thumbnail, tags)
- `_projects/` - Showcase projects (title, displayed, cover_image, link, classification, tags, featured)
- `_pages/portfolio.md` - Portfolio page content (work history, education)

### Key Dependencies

**Rust:**
- `axum` - Web framework
- `askama` - Compile-time templates
- `pulldown-cmark` - Markdown parsing
- `gray_matter` - YAML front matter extraction
- `tokio` - Async runtime
- `tower-http` - HTTP middleware (tracing, static files)
- `tower-livereload` - Development hot reload
- `image` - Image processing for responsive images
- `syntect` - Syntax highlighting

**Node.js:**
- `tailwindcss` v4 - CSS framework
- `@tailwindcss/cli` - Tailwind CLI

## Notes for Agents

1. **Template Changes:** Askama templates are compiled into Rust code. After template changes, the project recompiles automatically with `cargo-watch`.

2. **Adding Routes:** Add handler in `src/pages/`, register in `src/router.rs`, create template in `templates/`.

3. **Content Model:** All content uses YAML front matter. See `static/resources/config.yml` for field definitions.

4. **Image Handling:** Images are auto-generated in multiple sizes. The `picture_generator` module creates responsive `<picture>` elements.

5. **Debug vs Release:** Debug builds include livereload. Release builds are optimized for production.

6. **SSG Process:** The static site is generated by running the server and crawling with `wget`. All linked content must be discoverable from the homepage.
