# CI Deploy Pipeline Specification

Status: **Decided — implementation postponed** until the
[pre-flight checklist](#pre-flight-verification-checklist) is complete.
Decision (2026-08-19): **Forgejo Actions** with **forgejo-runner in host mode**.
Author: michalvanko + agent
Last updated: 2026-08-19

## Purpose

Replace the manual `just deploy` workflow with a Forgejo CI job that builds,
exports, and deploys the site automatically on every push to `main` — with
**fast warm deploys** by never regenerating responsive images that already
exist.

## Goals

1. **Automatic deploys** — push to `main` (after tests pass) goes live without
   manual steps. Manual `workflow_dispatch` re-run available.
2. **No redundant image generation** — `generated_images/` variants are
   persisted across runs; only never-before-seen images get generated.
3. **Fast warm deploys** — target ~2–4 min when Rust cache and images are warm.
4. **Deterministic `dist/` output** — site correctness must not depend on
   crawler behavior.

## Non-goals

- No rollback mechanism beyond re-running a previous commit's pipeline.
- No preview/staging environment (local `just export` + `just preview` stays).
- No change to how the server generates images at runtime.

## Background: how the pieces work

### The deploy chain (unchanged mechanics)

```
cargo build --release → run server (prod mode) → wget crawl (just ssg) → rsync dist/ → katelyn
```

Production serves static files from
`michalvanko@katelyn:.config/containers/systemd/michalvankodev-site/dist/`
via a quadlet container. **No restart needed after rsync** — files are served
directly.

### Key facts that shaped this design

| Fact | Consequence |
|---|---|
| `generate_images()` (`src/picture_generator/image_generator.rs`) **skips generation when the output file already exists** | Caching is free: restore `generated_images/` before the run and only new images are generated |
| Image generation is **lazy + async** (`rayon::spawn`, fire-and-forget during page render) | Crawler can race generation and 404; `dist/` completeness must not rely on the crawl |
| Source images live in **git LFS** (~166 MB, 92 files under `static/images/`) | CI checkout needs `lfs: true` + `git-lfs` installed in the container |
| CI container (`node:24` / Debian bookworm) ships **wget 1.21.3** — no `srcset` parsing (needs ≥1.25, wget2 has it) | Responsive variants would be silently missing from a pure-crawl `dist/` |
| `just tailwind` is watch-mode only; no lockfile committed | CI needs a one-shot recipe (`just tailwind_build`) + `npm install` |
| Forgejo **and** act_runner run **on katelyn itself** (same host as deploy target) | Deploy rsync is effectively local; no network/CrossHost SSH concerns |

## Architecture

### Trigger

- `push` to `main` → `test` job → (pass) → `deploy` job
- `workflow_dispatch` → same path, manual
- Feature branches / PRs → `test.yaml` only (`if: github.ref != 'refs/heads/main'`
  prevents duplicate test runs on main)

### Job: `test`

Runs `cargo test --all-features` with `Swatinem/rust-cache@v2`.

### Job: `deploy` (needs: test)

Step-by-step contract — the workflow YAML must implement exactly this:

1. **Install CI tools** — `git-lfs rsync curl wget procps libimage-exiftool-perl`
   (exiftool is used by `copy_exif` in the picture generator).
2. **Checkout with LFS** — `actions/checkout@v6` with `lfs: true`.
3. **Toolchain** — `dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2`
   (repo root — Cargo.toml is at root, not in a subdirectory), `just` via
   `taiki-e/install-action@just`.
4. **`npm install`** — for `@tailwindcss/cli`. (`npm ci` impossible until a
   lockfile is committed.)
5. **Setup SSH** — write `secrets.SSH_PRIVATE_KEY` to `~/.ssh/id_deploy`
   (0600), pin host key via `ssh-keyscan`, write an ssh config stanza for the
   deploy host, verify with a probe (`ssh -o BatchMode=yes … true`).
6. **`just tailwind_build`** — one-shot CSS build (recipe added for CI).
7. **`cargo build --release`**.
8. **Restore generated images from production** —

   ```bash
   rsync -az "$DEPLOY_HOST:$DEPLOY_DIR/generated_images/" generated_images/
   ```

   Failure-tolerant (first deploy has nothing to restore → warning, all images
   generated). This is the image cache: **production `dist/` is the source of
   truth**.
9. **Run server in background** — `nohup just prod > server.log 2>&1 &`
   with `PORT=3081`.
10. **Wait for server** — curl loop up to 100 × 2 s; any HTTP response code
    (including 500) counts as "up".
11. **SSG crawl + explicit image sync** —

    ```bash
    just ssg
    rsync -a generated_images/ dist/generated_images/
    ```

    The explicit rsync is **load-bearing**: it makes `dist/` complete
    regardless of (a) wget 1.x not parsing `srcset` and (b) the lazy async
    generation race. Never remove it in favor of "the crawler fetches them".
12. **Stop server** — `just kill || pkill -f axum_server || true`; dump
    `server.log` (`if: always()`) for debugging.
13. **Deploy** — `rsync -az --delete ./dist/ "$DEPLOY_HOST:$DEPLOY_DIR/"`.
    `--delete` keeps production clean of stale files (dist is fully rebuilt
    each run).
14. **Upload `dist/` artifact** (10-day retention) for inspection.

### Environment parameters (workflow `env`)

| Var | Value | Notes |
|---|---|---|
| `PORT` | `3081` | must match what `just ssg` crawls |
| `DEPLOY_HOST` | `192.168.0.181` | katelyn's LAN IP — always valid on-host; the `katelyn` alias may not resolve in containers. See [Host addressing](#forgejo-actions-status-verified-2026-08-19) |
| `DEPLOY_SSH_PORT` | `22` | |
| `DEPLOY_USER` | `michalvanko` | |
| `DEPLOY_DIR` | `.config/containers/systemd/michalvankodev-site/dist` | relative to home |

## Design decisions (and rejected alternatives)

### D1: Image cache = pull from production

**Chosen:** rsync `generated_images/` back from prod before build.

- ✅ Zero new infrastructure; self-healing (cache *is* production); outbound
  deploy rsync is incremental forever.
- ⚠️ Production is the cache source of truth.

**Rejected:**
- `actions/cache` — requires cache service enabled on act_runner; unverified.
- Persistent runner volume — breaks on runner rebuild / multiple runners.
- Commit to repo/LFS — bloats history; needs CI commit loop; messy with
  Decap CMS.

### D2: Explicit `rsync generated_images/ → dist/` instead of wget2

**Chosen:** local disk rsync (sub-second) after the crawl.

- The crawler fetching image variants is redundant work either way; with
  wget2 it would fetch hundreds of localhost HTTP requests the rsync replaces
  in under a second.
- wget 1.x lacks `srcset`; but **even wget2 would not fix the lazy-generation
  race** (crawler 404s never retried). The rsync is correct by construction.

### D3: Deploy with `--delete`

**Chosen:** yes — `dist/` is fully rebuilt each run; stale files must not
accumulate on prod. Drop the flag if additive behavior is ever wanted.

### D4: Tests run in the deploy pipeline

**Chosen:** `test` job gates `deploy` on main; `test.yaml` skips main pushes to
avoid duplicates and rust-cache races.

### D5: Trigger = push to main (+ manual dispatch)

**Chosen:** automatic deploys of main; manual re-run kept via
`workflow_dispatch`.

### D6: Execution mode = forgejo-runner **host mode** (decided 2026-08-19)

**Chosen:** install forgejo-runner on katelyn with a `host`-execution label;
workflow steps run directly on the host. The current `deploy.yaml`
(`container: node:24`) is a **container-mode draft** and must be adapted:

- `container:` removed; tools come from the host (see pre-flight checklist).
- `runs-on:` must match the runner's host label (e.g. `runs-on: host`).
- JS actions (`checkout`, `rust-cache`, `upload-artifact`) need `node` on host
  — verified in pre-flight; fallback is plain shell steps.
- **Caching upgrade:** with a persistent host workspace, set
  `CARGO_TARGET_DIR` to a fixed path outside the job workspace so `target/`
  persists → true incremental `cargo build` (seconds), outperforming
  rust-cache restore. `rust-cache` can then be dropped.
- Rationale: avoids docker/podman socket friction entirely (the usual cause
  of act_runner pain); fastest path; zero extra containers.
- **Security:** host mode executes workflow code as the runner user on
  katelyn. Repo is **public**, so `pull_request` triggers from forks are an
  RCE vector. Mitigations: drop `pull_request` from triggers (keep
  `push`→main + `workflow_dispatch`) and/or enable the repo's
  "require approval for fork PR runs" setting (verify it exists in v15 UI).

### D7: Workflows directory = `.forgejo/workflows/` (decided)

Move both workflows out of legacy `.gitea/workflows/` (never triggered on
v15 — see verified status). Verify after first push: runs appear in Actions
UI / `actions/tasks` API even before any runner exists.

### D8: Deploy & image-cache via local rsync — no SSH (decided, pending D9)

Runner lives on katelyn; if it runs **as user `michalvanko`** (systemd user
service), both the image-cache restore and the deploy are plain local rsync
against `/home/michalvanko/.config/containers/systemd/michalvankodev-site/dist/`
— the `SSH_PRIVATE_KEY` secret, ssh-keyscan and probe step all disappear.
File-serve permissions are already proven: the current manual `just deploy`
writes to that exact path via rsync and the site works.

### D9: Runner service identity (to verify on LAN)

Prefer a **systemd user service** as `michalvanko` (same user that owns the
dist dir and the podman quadlets). If a system-level runner (root) turns out
necessary, fall back to D8-via-SSH with the deploy key (original design).

## Forgejo Actions status (verified 2026-08-19)

Investigated via the Forgejo API (admin token) and the repo's actions page:

| Check | Result |
|---|---|
| Instance | Forgejo **v15.0.6** (`forgejo.katelyn.michalvanko.dev`, public IP `176.102.65.48`) |
| Actions feature | ✅ enabled (instance + repo; `/actions` returns 200) |
| Workflow triggers | ⚠️ **none ever fired** — `workflow_runs` total_count = 0 despite pushes to main with workflows present. Forgejo v15 reads **`.forgejo/workflows/`** only (per v15 admin guide); our files live in legacy `.gitea/workflows/` → never picked up |
| **Runners** | ❌ **ZERO registered** (`GET /api/v1/admin/actions/runners` → `[]`) — jobs would hang forever even with correct directory |
| Completed runs | ❌ none, ever |
| Repo secrets | none (`SSH_PRIVATE_KEY` still unset) |
| Repo visibility | ⚠️ **public** — security implication for host mode, see D6 |

**Conclusion:** two blockers exist: (1) workflows must move to
`.forgejo/workflows/`, and (2) a forgejo-runner must be installed and
registered on katelyn. Useful property: runs are created server-side in
"waiting" state even without a runner — so after moving the directory we can
verify triggering **before** installing the runner.

### Host addressing (verified)

- `katelyn` SSH alias → LAN IP `192.168.0.181:22` — only reachable from LAN/VPN
  (agent had "No route to host" while off-LAN).
- Public IP `176.102.65.48:22` is open but **rejects all our keys — it is not
  katelyn's sshd** (likely the router; forgejo SSH lives on `:3222`).
- Therefore: the runner executes **on katelyn** and deploys to
  `192.168.0.181:22` — always on-host, never traverses the public internet.

## Infrastructure requirements

### Runner setup on katelyn (forgejo-runner, host mode)

Prerequisite for the whole pipeline — no runners exist yet. Run on katelyn
(from LAN/VPN), after the pre-flight checklist passes:

1. **Registration token:** Forgejo web UI → Site Administration → Actions →
   Runners → *Create new runner token* (or admin API
   `POST /api/v1/admin/actions/runners/registration-token`).
2. **Install forgejo-runner** (latest release from
   `codeberg.org/forgejo/runner`) as a **systemd user service** under
   `michalvanko` (see D9). Configure a **host-mode label**, e.g.
   `labels: ['host:host']` in `config.yml` — no docker/podman socket involved.
3. **Verify:** runner appears as *Idle* under Site Administration → Actions →
   Runners; then push any commit — the queued workflow must start executing.
4. Then: first deploy run, cache seeding (first run generates all images).

### Secrets

Only needed if D9 lands on a root/system runner: **`SSH_PRIVATE_KEY`**
(repo → Settings → Actions → Secrets). With the user-service runner (D8),
no secrets are required at all.

### Runner host resolution (superseded — see verified findings above)

Verified 2026-08-19: `DEPLOY_HOST` is pinned to katelyn's LAN IP
`192.168.0.181` in the workflow env — no container-DNS verification needed
anymore.

## Files

| File | Role |
|---|---|
| `.gitea/workflows/deploy.yaml` | The pipeline (this spec's implementation) |
| `.gitea/workflows/test.yaml` | Branch/PR tests; skips `main` |
| `.gitea/workflows/release.yaml` | **Deleted** (stale: referenced nonexistent `axum_server/` dir, no LFS, never deployed; artifact upload absorbed into deploy.yaml) |
| `justfile` | Added `tailwind_build` (one-shot CSS build) |
| `docs/cicd_pipeline.md` | Points here |

## Operational runbook

### First deploy / all images regenerated

If prod `dist/generated_images/` is empty (or restore fails), the pipeline
logs a `::warning::` and generates every variant — slower run (~10–15 min
cold), still succeeds. The first successful deploy then seeds the cache.

### Force-regenerate an image

Delete it from production and re-run the pipeline:

```bash
ssh katelyn 'rm .config/containers/systemd/michalvankodev-site/dist/generated_images/images/uploads/<file>*'
# then re-run deploy (workflow_dispatch)
```

### Debugging a failed deploy

- Read the failed job log; `server.log` is printed on failure (`if: always()`).
- Download the `dist` artifact to inspect what was produced.
- Common suspects: LFS checkout problems (verify `git lfs ls-files`), SSH probe
  failure (secrets/known_hosts), server never came up (port collision on
  katelyn — `PORT=3081` must be free).

### Expected timings

| Scenario | Duration |
|---|---|
| Warm (rust-cache hit, images cached, rsync delta) | ~2–4 min |
| New content with new images | warm + seconds per image |
| Fully cold (no caches, all images generated) | ~8–15 min |

## Appendix: Woodpecker CI vs Forgejo Actions / forgejo-runner

Compared 2026-08-19 for this specific setup: single machine (katelyn, Fedora,
podman/quadlet), solo developer pushing directly to main, Forgejo v15.0.6.

### Comparison table

| Dimension | Forgejo Actions + act_runner | Woodpecker CI |
|---|---|---|
| Components to run | 1 (act_runner daemon; Actions server-side is built into Forgejo, already enabled) | 3 (server + agent + sqlite DB) |
| Forgejo integration | native, zero config (already triggering today) | OAuth2 app + webhooks + `ALLOWED_HOST_LIST` tweak |
| Workflow format | GitHub Actions syntax; huge ecosystem (`actions/checkout`, `rust-cache`, `upload-artifact`) | own simpler YAML (`.woodpecker/`); smaller plugin ecosystem |
| Container backends | docker (socket), `host` mode runs steps directly on host | docker (socket), `local` backend runs steps on host |
| Podman on katelyn | docker backend needs the compatibility socket (common pain point); host mode avoids it | same socket caveat for docker backend; local backend avoids it |
| Warm deploy speed | ~2–4 min (rust-cache restore, fresh workspace per job) | ~1–2 min with persistent volume for `target/`+`generated_images/`; seconds with local backend |
| CI UI / logs / artifacts | Actions tab in Forgejo (integrated) | own web UI (clean, simple), PR checks, badges |
| PR test gating | ✅ | ✅ |
| Secrets | repo/org/instance scoped, in Forgejo | repo/org scoped, in Woodpecker |
| Multi-repo / future growth | add runners, labels | activate repos in UI; scales to k8s if ever needed |
| Maintenance | keep runner binary updated; same release train as Forgejo | keep server+agent+DB updated; OAuth secret management |
| Idle footprint | 1 daemon, ~50 MB | 2 containers + DB, ~200 MB |
| Failure modes seen in the wild | runner connectivity, docker-socket perms, job-container quirks | webhook delivery, OAuth expiry, agent-server version skew |
| Workflow portability | GitHub Actions — portable to GitHub/other forges | Woodpecker — portable to any forge it supports |
| Status on this repo | deploy.yaml fully written; zero runners installed | would need .woodpecker/ pipeline + 2 quadlets + OAuth setup |

### Woodpecker — unique pros

- Dedicated CI UI: pipeline graphs, live logs, retries, badges — nicer than the
  grafted-on Actions tab.
- `local` backend: steps run on the host directly (no container socket at all)
  → persistent toolchain, incremental builds, near-webhook speeds, if you accept
  no per-job isolation.
- Persistent per-repo volumes in docker backend → true incremental cargo
  builds + generated_images always warm.
- Simpler YAML, no marketplace-action supply chain.

### Woodpecker — unique cons

- Two more always-on containers + DB + OAuth app to create and maintain.
- Second system to upgrade, monitor, and debug (version skew server↔agent).
- Smaller ecosystem; fewer ready-made steps (rsync deploy = plain shell step,
  fine but manual).
- Webhook delivery from Forgejo must be allowed/tuned (`ALLOWED_HOST_LIST`).

### Forgejo Actions — unique pros

- Already half-deployed: feature enabled, workflows trigger, `deploy.yaml`
  exists and is battle-designed (caching, LFS, SSH, rsync).
- Zero new web surface: no OAuth app, no extra ports, no second UI.
- GitHub Actions compatibility: any future project reuses the same knowledge
  and the huge actions ecosystem.
- act_runner `host` mode: steps on host, no container socket — the analog of
  Woodpecker's local backend.

### Forgejo Actions — unique cons

- Requires installing act_runner (the very tool with prior bad experience —
  though that pain is usually docker-socket/container-mode, avoidable via host
  mode).
- Fresh workspace per job unless host-mode tricks → rust-cache restore tax on
  every run.
- Actions tab UI is less polished for log browsing than Woodpecker.
- Runner and forgejo must stay version-compatible.

### Verdict for this use case

- **Simplest & fastest to working deploys with least new machinery:**
  Forgejo Actions + act_runner in **host mode** — reuses the finished
  `deploy.yaml` (label change only), no containers, no OAuth, no second UI.
- **Best long-term CI experience (UI, PR gating, more repos later):**
  Woodpecker — with the `local` backend on a trusted single-user box, or docker
  backend + persistent volumes for cache.
- Both container-mode paths hit the same podman-socket friction on katelyn.

## Pre-flight verification checklist

Everything that must be checked/decided **before implementation starts**.
Development is postponed until the on-LAN items are done.

### Already verified (2026-08-19, off-LAN)

- [x] Actions enabled on instance and repo (API + UI)
- [x] Zero runners registered; zero workflow runs ever
- [x] v15 workflows directory is `.forgejo/workflows/` → move required (D7)
- [x] Repo is public → host-mode security mitigations required (D6)
- [x] `DEFAULT_ACTIONS_URL` resolves to `https://data.forgejo.org` on v15 —
      `actions/checkout` & co. fetch from there; katelyn has internet access
      (it hosts the public site), still verify reachability on-LAN

### On-LAN checks (katelyn, as `michalvanko`)

1. **Toolchain inventory** (host mode runs on host tools — all required):
   ```bash
   rustc --version && cargo --version     # stable toolchain
   node --version && npm --version        # also needed for JS actions
   just --version                          # cargo install just if missing
   git-lfs --version                       # LFS checkout (lfs: true)
   rsync --version; curl --version; wget --version
   exiftool -ver                           # perl-Image-ExifTool (Fedora)
   pidof bash && echo procps-ok            # just kill uses pidof
   ```
2. **Port 3081 free**: `ss -tlnp | grep 3081` (CI server must not collide).
3. **Disk space**: `df -h /home` — budget ~5–10 GB (persistent `target/`,
   workspace, generated images, dist).
4. **Runner identity (D9)**: confirm a systemd **user** service under
   `michalvanko` is viable (lingering enabled if needed: `loginctl
   enable-linger michalvanko`) — then D8 (local rsync, no SSH) applies.
5. **Workspace persistence**: confirm forgejo-runner host mode keeps job
   workspaces between runs (docs/`config.yml`) — determines whether
   `CARGO_TARGET_DIR` trick is needed or workspace itself persists.
6. **JS actions on host**: verify `actions/checkout@v6` executes in host mode
   with system `node` (first test run) — fallback: plain shell `git clone`.
7. **Fork-PR security setting**: repo Settings → Actions — check for
   "require approval" option; regardless, drop `pull_request` trigger from
   test.yaml per D6.
8. **data.forgejo.org reachable** from katelyn: `curl -sI
   https://data.forgejo.org` (action resolution; else pin absolute URLs or
   mirror actions locally).
9. **Registration token minted** and forgejo-runner binary downloaded
   (latest stable from codeberg.org/forgejo/runner, compatible with v15).
10. **Trigger verification** (no runner needed): after D7 move + push, runs
    appear under `/actions` in "waiting" state → triggering confirmed.

## Future work / open items

- **Crawl skip optimization:** add `--reject-regex "generated_images"` to the
  `ssg` wget calls — crawler no longer needs to fetch image variants at all
  (rsync guarantees them in `dist/`). Byte-identical output, a few seconds
  faster. Decision pending (would change local behavior identically).
- **`package-lock.json`:** commit one so CI can use reproducible `npm ci`
  (renovate can maintain it).
- **`actions/cache` alternative:** if act_runner cache service is ever
  enabled, consider it as cache layer independent of prod.
- **`Avif` export format:** currently `todo!()` in `export_format.rs` — if it
  lands, cache size and generation cost grow; revisit cache strategy then.

## Implementation status

**Decided & specified, development postponed until pre-flight checklist is
done.**

Done:
- [x] `.gitea/workflows/deploy.yaml` written (⚠️ container-mode **draft** —
      to be adapted per D6: host mode, no `container:`, `runs-on: host`,
      local-rsync deploy per D8, `CARGO_TARGET_DIR` caching)
- [x] `justfile`: `tailwind_build` recipe (verified locally)
- [x] `test.yaml`: skip main pushes (⚠️ also drop `pull_request` per D6)
- [x] `release.yaml` deleted
- [x] Investigation: Actions enabled, 0 runners, 0 runs ever, `.gitea/`
      workflows never triggered on v15 (D7)
- [x] Comparison appendix: Woodpecker vs Forgejo Actions (decision record)

Blocked on pre-flight (on-LAN):
- [ ] Pre-flight verification checklist complete (10 items above)
- [ ] **Workflows moved to `.forgejo/workflows/`** (D7) — then push and
      confirm runs appear in "waiting" state
- [ ] forgejo-runner installed (user service, host label) and registered
- [ ] `deploy.yaml` adapted for host mode + local rsync (D6/D8)
- [ ] `pull_request` trigger dropped from test.yaml (D6)
- [ ] First full pipeline run on main (seeds image cache)
