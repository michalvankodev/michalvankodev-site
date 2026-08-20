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
- ~~No preview/staging environment~~ — **superseded 2026-08-20**: PR preview
  deployments specified in [Preview deployments](#preview-deployments).
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

### The serving topology (verified 2026-08-20)

Public traffic: DNS (wildcard `*.michalvanko.dev` → `176.102.65.48`,
Namecheap) → **alula** (the only internet-exposed host), which runs the
TLS-terminating Caddy (per-hostname Let's Encrypt certs, HTTP-01) and
forwards to **katelyn's** Caddy — the quadlet serving the generated
static files — **through a rathole tunnel**: rathole *server* on alula
exposes tunneled TCP ports on alula's loopback; katelyn's rathole
*client* dials outbound (katelyn is never directly addressable — not
from the internet, not from alula). Confirmed remotely: responses
carry `server: Caddy` **and** `via: 1.1 Caddy` (two Caddies in the
chain); arbitrary subdomains resolve (wildcard record) but fail TLS
(no cert for them). Implications: all cert/TLS work for anything new
happens on **alula**; katelyn only ever serves static files; and
**anything alula needs from katelyn — including the previews `ask`
check — must ride the rathole tunnel**.

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
| `DEPLOY_HOST` | `192.168.0.181` | katelyn's canonical LAN address — router DHCP static lease pins `.181`; katelyn currently answers on `.156` **only until its next restart** (lease not yet applied). D8's local-rsync removes IP pinning regardless. See [Host addressing](#forgejo-actions-status-verified-2026-08-19) |
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
- **2026-08-20:** katelyn temporarily answers on `192.168.0.156` —
  the router has a **DHCP static lease pinning it to `192.168.0.181`**,
  taking effect on katelyn's next restart (not yet applied). Do **not**
  migrate references away from `.181`; the `.156` anomaly is transient.
  (D8's local-rsync design remains preferable on its own merits — it
  removes IP pinning entirely — but is no longer *forced* by drift.)
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
| `.forgejo/workflows/preview.yaml` *(to create)* | PR preview deploy/teardown — see [Preview deployments](#preview-deployments) |
| `~/previews/<hostname>/` on katelyn *(to create)* | Preview roots (dirs named by full hostname, e.g. `foo.dev.michalvanko.dev`) — hardlink overlays over prod `dist/` (D12) |
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

## Preview deployments

Specified 2026-08-20 (decided: PR-driven lifecycle, on-demand TLS,
hardlink storage). Implementation pending — blocked on the same
pre-flight checklist as the main pipeline.

Every open PR gets a live preview at `https://<label>.dev.michalvanko.dev/`,
where `<label>` is the sanitized PR head-branch name (lowercase,
`[^a-z0-9-]` → `-`, truncated to 63 chars — one DNS label).

### The `ask` endpoint

Official contract (Caddyfile global-options docs, verified 2026-08-20):
when alula receives a TLS handshake (SNI) for a name it has **no cert
for yet**, it first makes `GET <ask-url>?domain=<hostname>`. HTTP
**2xx → authorized** to obtain a cert; **any other response (or
connection error) → issuance cancelled, handshake errors**. The
endpoint must answer in milliseconds (constant-time local lookup; no
DNS/network calls). It is a pure boolean — "is this hostname one of our
live previews?" It never talks to LE, stores nothing, and is only
consulted for *new* issuance; every later handshake uses the cached
cert with zero ask traffic.

First-visit timeline for `foo.dev.michalvanko.dev` (fresh preview):

```
1. PR opened → CI rsyncs ~/previews/foo.dev.michalvanko.dev/ (katelyn)
2. Browser: wildcard DNS → alula:443 (SNI foo.dev...)
3. alula has no cert → GET 127.0.0.1:9123/?domain=foo.dev.michalvanko.dev
   (rathole: alula loopback ⇄ katelyn Caddy :9123 — Transport A below)
4. katelyn: does previews/foo.dev.michalvanko.dev/index.html exist? → 200
5. alula → LE HTTP-01 (LE dials foo.dev...:80 → alula answers)
6. handshake completes (~1–3 s, this once), cert cached in alula storage
7. request proxied (Host preserved) → katelyn static block → preview
```

Teardown: dir removed → ask now 404s for that name → no new issuance
possible; the existing cert lingers (see D10 nuance) and requests 404
at katelyn. If katelyn — or the tunnel — is down, *first* visits to
previews without certs fail closed (previews with cached certs keep
TLS; but the site itself is down anyway, since prod serving uses the
same tunnel — ask-over-tunnel adds **no new failure coupling**).
Acceptable.

**Who serves it?** The truth (preview dirs) lives on katelyn, so the
endpoint runs there. But alula cannot address katelyn directly — the
only path is the existing **rathole tunnel**, so the ask request rides
it (it reveals only hostname existence; no secrets, and the tunneled
port never needs to be public).

**Responder — no new service at all:** katelyn's existing Caddy
quadlet serves the check on port 9123. Preview dirs are named by
their **full hostname**, which makes both this check and the serving
block trivial (no label parsing anywhere):

```caddyfile
# katelyn Caddy — ask endpoint for alula's on-demand TLS
# (reached only via the rathole tunnel; never exposed publicly)
http://:9123 {
    root * /home/michalvanko/previews
    try_files {http.request.uri.query.domain}/index.html =404
    respond 200
}
```

`try_files` expands the `?domain=` query placeholder to a path under
the previews root: found → path rewritten → `respond 200`; missing →
`=404` → handshake refused. Verify query-placeholder support in
`try_files` at implementation time (fallback below).

**Transport — how alula reaches the responder (pick one):**

- **A (recommended): dedicated rathole service entry** — alula
  `127.0.0.1:9123` ⇄ katelyn `:9123`. A few TOML lines on both rathole
  ends + service restart; alula's global option becomes
  `ask http://127.0.0.1:9123`. Bind the alula side to **loopback** so
  the check is never internet-reachable. Bonus: rathole reconnects on
  IP drift automatically — even the transient katelyn `181 → 156`
  anomaly never
  touches the serving path (it only ever affected our off-LAN SSH).
- **B (zero rathole changes):** an `/__ask` route inside katelyn's
  *already-tunneled* static listener; alula's ask URL becomes
  `http://127.0.0.1:<tunneled-static-port>/__ask`. No new tunnel entry,
  but it entangles the check with prod vhost routing (Host-matching of
  a `127.0.0.1`-Host request; accidental public exposure of `/__ask`
  through alula's catch-all proxy) — only if adding a rathole entry is
  somehow impossible.

**Responder fallback — ~20-line dedicated service** (systemd user
service or quadlet), if the pure-Caddy `try_files` form proves
unsupported or we later want logging/metrics/auth:

```python
#!/usr/bin/env python3
"""Caddy on-demand TLS ask endpoint: 2xx iff preview dir exists."""
import http.server, urllib.parse, pathlib
ROOT = pathlib.Path("/home/michalvanko/previews")
SUFFIX = ".dev.michalvanko.dev"
class H(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        q = urllib.parse.parse_qs(urllib.parse.urlparse(self.path).query)
        domain = q.get("domain", [""])[0].lower()
        ok = domain.endswith(SUFFIX) and (ROOT / domain).is_dir()
        self.send_response(200 if ok else 404)
        self.end_headers()
    do_POST = do_GET  # tolerate verb differences across Caddy versions
http.server.ThreadingHTTPServer(("0.0.0.0", 9123), H).serve_forever()
```

(There is also a `permission <module>` extension point should custom
logic ever be needed — `ask` is just the built-in `http` module.)

### D10: TLS = on-demand per-branch certs on alula (no DNS-API integration)

**Chosen:** `*.dev.michalvanko.dev` site block on **alula's** Caddy with
`tls { on_demand }` + an `ask` gate.

- Wildcard DNS already exists (verified: any `*.dev.michalvanko.dev` name
  resolves to `176.102.65.48`). HTTP-01 keeps working because alula
  already terminates ports 80/443 for that traffic.
- Each branch cert is issued exactly like every cert we already run —
  per-hostname Let's Encrypt via HTTP-01. **Zero third-party
  integration.**
- **Why not a wildcard certificate:** LE only issues `*.` names via
  DNS-01 (a `_acme-challenge` TXT record per issuance *and* per ~60-day
  renewal). Unattended renewal therefore requires the Namecheap API
  (plus an xcaddy-built Caddy with `caddy-dns/namecheap`, plus IP
  whitelisting) — exactly the integration we run without today. Manual
  TXT pasting per renewal is the only other no-API path and is a standing
  chore.
- **The `ask` gate is a quota firewall, not an integration:** the repo is
  public and wildcard DNS means internet scanners hitting random `*.dev`
  names would make alula mint junk certs until LE's 50-certs-per-week-
  per-registered-domain limit threatens renewal of the *real* domains.
  See [The `ask` endpoint](#the-ask-endpoint) for exactly how it works.
- **Cert lifecycle nuance:** `ask` gates *new* issuance (a handshake for
  a name with no loaded cert). Background renewals of already-issued
  certs are **not** ask-gated, so a torn-down preview's cert lingers in
  alula's storage and keeps renewing — a few KB each, cosmetic. Requests
  to it TLS-succeed but 404 at katelyn (dir gone). Optional low-priority
  cleanup: periodically delete cert dirs in alula's Caddy storage whose
  hostname has no preview dir. Verify with the LE **staging** endpoint
  during implementation.
- `interval`/`burst` options inside `on_demand_tls` are deprecated —
  **do not add them** (official docs: "NOT recommended").

### D11: Lifecycle = PR-driven only (decided 2026-08-20)

| PR event | Preview action |
|---|---|
| `opened`, `synchronize`, `reopened` | build + deploy |
| `closed` (covers **merged**) | `rm -rf ~/previews/<hostname>/` |

- Forgejo Actions has **no branch-`delete` event**; the PR lifecycle is
  the only trigger pair with symmetric create/teardown → no scheduled
  sweep and no TTL needed.
- A branch without an open PR gets no preview (open the PR to get one).
- Security (D6, host mode): same-repo branches are trusted; fork PRs must
  be caught by Forgejo's moderated approval gate for pull_request
  workflows — verify the setting exists in v15 (pre-flight item 11).
- Preview builds use `PORT=3082` — can never collide with a prod pipeline
  run on 3081 (or with a concurrently running one).

### D12: Storage = hardlink overlays — a preview costs only its delta

The runner executes on katelyn, where prod `dist/` (~131 MB) lives, so
previews share inodes with prod instead of copying it:

```bash
# Image-cache restore: instant, 0 bytes on disk. Safe because the
# generator only creates missing files, never mutates existing ones.
mkdir -p generated_images
cp -al "$PROD_DIST/generated_images/." generated_images/

# Preview deploy: files identical to prod become hardlinks; only the
# branch delta (new/changed HTML + new image variants) is written.
rsync -a --delete --link-dest="$PROD_DIST/" ./dist/ "$PREVIEW_ROOT/$HOSTNAME/"
```

- Per-preview disk cost = branch delta (typically single-digit MB), not
  ~131 MB per branch.
- Teardown (`rm -rf`) is inode-safe — deleting a hardlink never touches
  prod's data; conversely a prod redeploy that replaces a file leaves old
  preview links holding the old inode until that preview dies.
- Previews **never write back** to prod's image cache. Branch-only image
  variants die with the preview; on merge, the prod pipeline generates
  them once (seconds per image).

### Serving chain

```
*.dev.michalvanko.dev → wildcard DNS → alula (public host)
alula Caddy:
    on_demand_tls { ask http://127.0.0.1:9123 }  # via rathole — Transport A
    *.dev.michalvanko.dev {
        tls { on_demand }
        reverse_proxy 127.0.0.1:<tunneled-static-port>  # existing rathole
    }                                                   # service → katelyn
alula rathole server: 127.0.0.1:9123 ⇄ katelyn:9123    # NEW service entry
→ katelyn Caddy (static quadlet, reached only via tunnel, Host preserved):
    # preview dirs named by FULL hostname (see ask section)
    *.dev.michalvanko.dev {
        root * /home/michalvanko/previews/{http.request.host}
        try_files {path}.html {path}/index.html {path}
        header X-Robots-Tag noindex
        file_server
        encode zstd gzip
    }
```

Full-hostname dir naming (`previews/foo.dev.michalvanko.dev/`) removes
all label math: katelyn's serving root is simply the request Host, and
the ask check is a one-line `try_files` on the `?domain=` query.
alula's `reverse_proxy` preserves the original Host by default, so
katelyn can route on it. Because alula reaches katelyn only via the
tunnel, **no stable katelyn address is needed anywhere** — rathole
reconnects on IP drift by design.

### Workflow sketch (`.forgejo/workflows/preview.yaml`)

```yaml
name: preview
on:
  pull_request:
    types: [opened, synchronize, reopened, closed]

env:
  PORT: "3082"
  PROD_DIST: /home/michalvanko/.config/containers/systemd/michalvankodev-site/dist
  PREVIEW_ROOT: /home/michalvanko/previews

jobs:
  meta:
    runs-on: host
    outputs:
      hostname: ${{ steps.label.outputs.HOSTNAME }}
    steps:
      - id: label
        run: |
          LABEL=$(echo "${{ github.event.pull_request.head.ref }}" \
            | tr '[:upper:]' '[:lower:]' | sed -E 's/[^a-z0-9-]+/-/g' | cut -c1-63)
          echo "HOSTNAME=${LABEL}.dev.michalvanko.dev" >> "$GITHUB_OUTPUT"

  deploy:
    if: github.event.action != 'closed'
    needs: meta
    runs-on: host
    steps:
      - uses: actions/checkout@v6
        with: { lfs: true }
      - run: npm install
      - run: just tailwind_build
      - run: cargo build --release      # CARGO_TARGET_DIR per D6 host mode
      - run: |
          mkdir -p generated_images
          cp -al "$PROD_DIST/generated_images/." generated_images/
      - run: nohup just prod > server.log 2>&1 &
      - run: |                        # wait-loop, same contract as deploy.yaml
          for i in $(seq 1 100); do
            code=$(curl -s -o /dev/null -w '%{http_code}' --max-time 2 "http://127.0.0.1:$PORT/" || true)
            [ -n "$code" ] && [ "$code" != "000" ] && exit 0
            sleep 2
          done
          echo "::error::Server did not come up"; exit 1
      - run: |
          just ssg
          rsync -a generated_images/ dist/generated_images/
      - run: just kill || pkill -f axum_server || true
      - if: always()
        run: cat server.log || true
      - run: rsync -a --delete --link-dest="$PROD_DIST/" ./dist/ "$PREVIEW_ROOT/${{ needs.meta.outputs.hostname }}/""

  teardown:
    if: github.event.action == 'closed'
    needs: meta
    runs-on: host
    steps:
      - run: rm -rf "$PREVIEW_ROOT/${{ needs.meta.outputs.hostname }}"
```

(Sketch — the real file inherits all host-mode adaptations of
`deploy.yaml` per D6/D8, e.g. persistent `CARGO_TARGET_DIR`, no SSH
steps, local paths only.)

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
11. **Preview infra (when implementing previews):** alula Caddy gains the
    `*.dev` on-demand block + `on_demand_tls { ask http://127.0.0.1:9123 }`;
    add a rathole service entry alula-loopback `127.0.0.1:9123` ⇄
    katelyn `:9123` (bind the alula side to loopback — never public);
    katelyn's existing Caddy gains the `:9123` ask block (`try_files` on
    the `?domain=` query — verify placeholder support, else the
    ~20-line Python fallback) and the `*.dev` static site block with
    `root previews/{http.request.host}`; verify the fork-PR moderation
    setting ("approve pending runs") exists in v15 (D11).

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
- [x] Preview deployment design (2026-08-20): D10 on-demand TLS on alula,
      D11 PR-driven lifecycle, D12 hardlink overlays — see
      [Preview deployments](#preview-deployments). Topology corrected:
      TLS front door is alula, katelyn = static serving only (reachable
      only via rathole); katelyn transiently on `192.168.0.156` until
      its DHCP lease (pins `.181`) applies on next restart.

Blocked on pre-flight (on-LAN):
- [ ] Pre-flight verification checklist complete (10 items above)
- [ ] **Workflows moved to `.forgejo/workflows/`** (D7) — then push and
      confirm runs appear in "waiting" state
- [ ] forgejo-runner installed (user service, host label) and registered
- [ ] `deploy.yaml` adapted for host mode + local rsync (D6/D8)
- [ ] `pull_request` trigger dropped from test.yaml (D6)
- [ ] First full pipeline run on main (seeds image cache)
