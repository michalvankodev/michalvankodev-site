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
├── .forgejo/workflows/     # Forgejo Actions CI (test.yaml, deploy.yaml)
├── static/                 # Static assets served directly
│   ├── images/             # Site images (git LFS)
│   ├── fonts/              # Custom web fonts (Baloo2)
│   ├── svg/                # SVG icons
│   └── resources/          # Decap CMS config
├── _posts/blog/            # Blog posts (Markdown with front matter)
├── _projects/              # Showcase projects (Markdown with front matter)
├── _pages/                 # Static pages (portfolio.md)
├── specs/                  # Decision records (CI-deploy.md = CI/CD design + runbook)
├── docs/                   # Long-form docs
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
4. **Image Generation:** Tokio-based bounded queue (`src/picture_generator/image_jobs.rs`, default `cores - 2` workers, `IMAGE_WORKERS` env) generates missing variants lazily during rendering; `GET /export-wait` long-polls until the queue is idle
5. **SSG Export:** `just ssg` = `ssg_prewarm` (pages-only crawl to enqueue image jobs, drain via `/export-wait`) + full wget crawl into `dist/`
6. **Deploy:** CI rsyncs `dist/` into the quadlet dist dir on katelyn (no service restart needed)

## Code Conventions

### Rust Patterns

- **Module Organization:** Each domain has a `mod.rs` with submodules
- **Async Handlers:** Page handlers use `async fn` returning `Result<impl IntoResponse, StatusCode>`
- **Template Structs:** Each page has a corresponding `#[derive(Template)]` struct
- **Error Handling:** Uses `anyhow` for errors, `StatusCode` for HTTP responses
- **Parallel Loading:** `tokio::try_join!` for concurrent data fetching

### Template Patterns

- **Inheritance:** Templates extend `base.html` using `{% block content %}` — the `{% extends %}` tag **must be the first tag** in the file (askama ≥0.15 hard error)
- **Includes:** Reusable partials via `{% include "component.html" %}`
- **Macro calls:** single-tag `{% call m(args) %}` no longer exists — use `{{ m(args) }}` (or `{% call %}…{% endcall %}` with a body)
- **Filters:** Custom filters in `src/filters/` wrapped in `#[askama::filter_fn]` (askama 0.16), with plain `*_impl` functions for Rust callers (see `feed.rs`)
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

- `PORT` - Server port (default: 3080; read at **runtime**, not compile time)
- `RUST_LOG` - Logging level (default: `axum_server=debug,tower_http=debug`)
- `IMAGE_WORKERS` - Image-generation worker count (default: cores - 2)
- `FORGEJO_TOKEN` / `FORGEJO_URL` - Forgejo API token + instance URL (fish universal vars on phoebe; scoped `write:repository,write:issue,read:user`; for API calls / `fj` CLI)

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
just ssg           # Prewarm + crawl a running server into dist/
just deploy        # Manual rsync dist/ to prod (CI does this automatically on main)
```

Pushes to `main` deploy automatically via Forgejo Actions — manual `just deploy` is only a fallback.

## Important Files

### Configuration

- `Cargo.toml` - Rust dependencies (axum, askama, pulldown-cmark, etc.)
- `justfile` - All build/dev/deploy commands
- `static/resources/config.yml` - Decap CMS collections and fields
- `styles/input.css` - Tailwind theme customization (colors, fonts, spacing)

### CI/CD (Forgejo Actions)

Live since 2026-08-20 — full design record, decision log and runbook in [`specs/CI-deploy.md`](specs/CI-deploy.md) (read it before changing CI).

- `.forgejo/workflows/test.yaml` — `cargo test --all-features` on non-main pushes (push-only trigger; v15 only reads `.forgejo/workflows/`, the old `.gitea/` dir is dead)
- `.forgejo/workflows/deploy.yaml` — `main` pushes + `workflow_dispatch`: test → release build → SSG export (server + crawl in **one step**) → local rsync into the prod dist dir → artifact upload
- **Runner:** `forgejo-runner` v13 in **host mode** (`runs-on: host`), systemd user service on katelyn (runs as the deploy user) — jobs execute directly on the deploy target; register/uuid+token live in `~/.config/forgejo-runner/config.yml` (`server.connections`, the legacy `register` subcommand is deprecated)
- **Rust cache:** `CARGO_TARGET_DIR` under the deploy user's home (`~/forgejo-runner-cache/target`) persists across runs (registry at `~/.cargo`); warm release build ≈ 16 s
- **Image cache:** `generated_images/` restored from production dist before each build (the generator skips existing files)

**Host-mode CI quirks (hard-won):**
- The runner **kills the process group when a step ends** — background server + wait loop + crawl + kill must share ONE workflow step (also applies to any future preview workflow)
- `upload-artifact` must stay `@v3` — v4+ hard-errors on Forgejo (`GHESNotSupportedError`)
- Every checkout that runs tests needs `lfs: true` — image-decoding tests fail on LFS pointer bytes
- CI job logs are anonymously readable at `/actions/runs/<n>/jobs/<j>/attempt/1/logs`

### Infrastructure

```
internet → alula (public entry host)
             ├─ TLS-terminating Caddy (Let's Encrypt HTTP-01, per-hostname certs)
             └─ rathole server (tunnel endpoints, loopback)
           katelyn (LAN only — never directly addressable from the internet)
             ├─ rathole client (outbound to alula; reconnects on address drift)
             ├─ static-serving Caddy quadlet → michalvanko.dev
             │    root: ~/.config/containers/systemd/michalvankodev-site/dist/
             ├─ Forgejo (web via tunnel; git SSH on a nonstandard port)
             └─ forgejo-runner (host mode)
           phoebe — dev workstation (this repo)
```

- **All Forgejo access goes through the public tunnel host — never katelyn's
  LAN address:** web/API at `https://forgejo.katelyn.michalvanko.dev`, git SSH
  at `ssh://git@forgejo.katelyn.michalvanko.dev/...` (keys + port live in
  `~/.ssh/config` — keep the port OUT of the git URL, an explicit port breaks
  `fj`'s API host parsing). The tunnel roundtrip (phoebe → alula → rathole →
  katelyn) is stable across katelyn's LAN IP drift. Do NOT open extra
  Forgejo ports on katelyn's LAN firewall — ssh is the only LAN service it
  exposes
- Git remotes: `origin` = GitHub; `katelyn` = the self-hosted Forgejo via
  the public tunnel host (exact URL: `git remote -v`)
- `fj` (forgejo-cli) is installed at `~/.local/bin/fj` (from Codeberg
  `forgejo-contrib/forgejo-cli` releases), token-authenticated against the
  instance (token fed via stdin to `fj auth add-token`; keys live in
  `~/.local/share/forgejo-cli/keys.json` — same token as the `FORGEJO_TOKEN`
  fish universal var). Reference: the `forgejo-cli` skill. Quirks learned the
  hard way:
  - global flags (`-H`, `--style`) go BEFORE the subcommand; subcommand
    `-r owner/repo` still needs `-H` unless repo auto-detection works
  - repo auto-detection = a remote pointing at the tunnel host + the branch
    upstream tracking it (`git branch --set-upstream-to=katelyn/main`)
  - no `-R/--remote` (that's `tea`), no `--version` (`fj version`)
- katelyn's LAN address drifts (DHCP): its `/etc/hosts` entry and
  `katelyn`-named known_hosts keys on phoebe were refreshed 2026-08-21 —
  if `ssh katelyn` fails, re-verify the IP (`ip neigh`), re-pin `/etc/hosts`
  and refresh known_hosts before connecting. Passwordless sudo available.
  Direct katelyn SSH (alias + key in `~/.ssh/config`) is ONLY for host
  admin (Caddy quadlet, runner, podman, firewall) — git/Forgejo work always
  goes through the tunnel host
- **PR preview deployments** are live (2026-08-20): every open PR gets
  `https://<label>.dev.michalvanko.dev` — on-demand TLS via alula + `ask`
  over the rathole tunnel, hardlink-overlay storage, PR-driven lifecycle,
  and a marker comment on the PR with the live URL. Spec + implementation
  record: `specs/CI-deploy.md` § Preview deployments (D10–D12)

### Content Structure

- `_posts/blog/` - Blog posts with YAML front matter (title, segments, published, date, thumbnail, tags)
- `_projects/` - Showcase projects (title, displayed, cover_image, link, classification, tags, featured)
- `_pages/portfolio.md` - Portfolio page content (work history, education)

### Key Dependencies

**Rust:**
- `axum` - Web framework
- `askama` 0.16 - Compile-time templates (`#[askama::filter_fn]` custom filters)
- `pulldown-cmark` - Markdown parsing
- `gray_matter` 0.3 - YAML front matter extraction (`Matter::parse::<D>()` → `Result<ParsedEntity<D>>`)
- `tokio` - Async runtime (also drives the image-generation queue)
- `tower-http` - HTTP middleware (tracing, static files)
- `tower-livereload` - Development hot reload
- `image` + `syntect` - Image processing / syntax highlighting

**Node.js:**
- `tailwindcss` v4 - CSS framework
- `@tailwindcss/cli` - Tailwind CLI

## Notes for Agents

1. **Template Changes:** Askama templates are compiled into Rust code. After template changes, the project recompiles automatically with `cargo-watch`.

2. **Adding Routes:** Add handler in `src/pages/`, register in `src/router.rs`, create template in `templates/`.

3. **Content Model:** All content uses YAML front matter. See `static/resources/config.yml` for field definitions.

4. **Image Handling:** Images are auto-generated in multiple sizes via the bounded tokio queue (skips existing files — that's the build cache). The `picture_generator` module creates responsive `<picture>` elements. Exif is copied via `exiftool` — it must be installed for generation.

5. **Debug vs Release:** Debug builds include livereload. Release builds are optimized for production. `PORT` is read at runtime — a prebuilt binary honors it.

6. **SSG Process:** `just ssg` prewarms (enqueue + drain) then crawls with wget2. All linked content must be discoverable from the homepage. The CI workflow also rsyncs `generated_images/` into `dist/` explicitly — never trust the crawler for dist completeness.

7. **LFS:** Source images live in git LFS (~166 MB). Any checkout that decodes images (tests, builds) needs real LFS smudge — in CI always `lfs: true`. Symptom of pointers-instead-of-images: `Illegal start bytes:7665` ("ve"rsion of an LFS pointer).

8. **Dependency bumps must compile:** askama 0.15/0.16 and gray_matter 0.3 have breaking API/template changes that previously landed unverified. The test workflow is the guard — never merge a dep bump that hasn't passed it.

9. **CI/Forgejo work:** read `specs/CI-deploy.md` first; use `$FORGEJO_TOKEN`/`$FORGEJO_URL` for API calls (temp tokens can be minted via `podman exec forgejo gitea admin user generate-access-token` on katelyn). Job logs are readable anonymously (see CI/CD section).
