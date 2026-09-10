# Contrast sweep — deferred decision record

Status: **proposed, not yet implemented** (split out of the a11y & print slice
to be reviewed separately). Everything below is measured against the current
`redesign/08-syndication-perf` base.

## The standard

WCAG 2.1 AA: text needs **4.5:1** (normal size) or **3:1** (large text —
≥ 24px, or ≥ 18.66px bold). The site's metadata voice is `font-mono text-xs
md:text-sm` — always normal-size, so the 4.5:1 bar applies everywhere.

Measured ratios (computed per WCAG relative-luminance formula):

| Foreground | on blue-50 (#eff6ff) | on white (#ffffff) |
| ---------- | -------------------- | ------------------ |
| slate-400 (#94a3b8) | **2.36:1 ✗** | 3.03:1 ✗ |
| slate-500 (#64748b) | **4.37:1 ✗** | 4.76:1 ✓ |
| slate-600 (#475569) | **6.96:1 ✓** | 7.58:1 ✓ |

Conclusion: on the blue-50 page surface, **slate-600 is the floor for
metadata text**. slate-500 is borderline-fail; slate-400 fails outright.

## Actual state (inventory)

### Failing — slate-500/400 on blue-50 (8 spots)

| Template | Element | Current | Ratio | Proposed |
| --- | --- | --- | --- | --- |
| `templates/index.html:45` | lead-story date | slate-500 | 4.37 ✗ | slate-600 |
| `templates/components/blog_post_preview.html:18` | list-row date | slate-500 | 4.37 ✗ | slate-600 |
| `templates/blog_post.html:36` | article header date | slate-500 | 4.37 ✗ | slate-600 |
| `templates/blog_post.html:63` | TOC label "on this page" | slate-500 | 4.37 ✗ | slate-600 |
| `templates/blog_post.html:71` | "↑ top" link | slate-500 | 4.37 ✗ | slate-600 |
| `templates/post_list.html:39` | year-group post count | slate-400 | 2.36 ✗ | slate-600 |
| `templates/portfolio.html:218` | "more work" label | slate-500 | 4.37 ✗ | slate-600 |
| `templates/project_list.html:32` | "more work" label | slate-500 | 4.37 ✗ | slate-600 |

### Passing — leave as-is (informational)

| Template | Element | Current | Ratio |
| --- | --- | --- | --- |
| `templates/components/project_preview_card.html:25-28` | year / separator / classification | slate-500/400 on white plate | 4.76 ✓ |
| `templates/components/project_row.html:26-29` | year / separator / classification | slate-500/400 on white plate | 4.76 ✓ |
| `templates/site_footer.html:5` | colophon | slate-600 on blue-50 | 6.96 ✓ |

The project-card labels sit on white plates, so slate-500 passes there. The
slate-400 *separator pipe* (`|`) is decorative punctuation, not content —
it can stay either way.

## Proposed state

1. Bump the 8 failing spots to `text-slate-600` (mechanical class swap, no
   layout impact).
2. Optionally normalize the project-card year/classification labels to
   slate-600 as well for one consistent "metadata voice" — not required
   for AA since their plate is white. **Decision needed.**
3. Rule of thumb going forward: on blue-50, never author new metadata
   below slate-600.

## How to verify after implementing

- Rebuild CSS (`just tailwind_build`), grep `styles/output.css` for
  `text-slate-500` / `text-slate-400` — remaining hits should only be the
  white-plate card labels (or none, if normalized).
- Spot-check computed color of the lead-story date in devtools against
  `#475569`.
