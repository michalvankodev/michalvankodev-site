# CI/CD Pipeline

Superseded by the specification in [`specs/CI-deploy.md`](../specs/CI-deploy.md),
which documents the deployed Forgejo CI pipeline (test → build → SSG export →
rsync deploy) including the generated-images caching strategy.

Historical notes from the original sketch:

- Build: compile, run in prod mode, wget crawl to static site
- Cache generated images from previous builds (implemented: restore from
  production `dist/` — see spec D1)
- Backup old version, publish new version (implemented: `rsync --delete`)
- Dev server cannot be crawled due to livereload (still true)
