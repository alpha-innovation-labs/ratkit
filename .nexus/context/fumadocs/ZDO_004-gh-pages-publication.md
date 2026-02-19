---
context_id: ZDO_004
title: GitHub Pages Docs Publication
project: zenbt-docs
created: "2026-02-19"
---

# ZDO_004: GitHub Pages Docs Publication

## Desired Outcome

Documentation can be published to GitHub Pages from the repository with a single operator command, producing a stable static docs site under the repository Pages URL with repeatable output and clear failure signals when prerequisites are missing.

## Reference

| Reference File | Coverage |
|----------------|----------|
| `docs/next.config.mjs` | Static export configuration contract for GitHub Pages mode |
| `docs/src/app/api/search/route.ts` | Search route behavior contract for static export mode |
| `docs/src/app/llms.mdx/route.ts` | MDX extraction route contract compatible with static export |
| `justfiles/utilities/docs-gh-pub.just` | Publication command contract and push flow to `gh-pages` |
| `justfile` | Operator-facing command discoverability in help and imports |

| External Reference | Coverage |
|--------------------|----------|
| `https://www.fumadocs.dev/docs/deploying/static` | Fumadocs static deployment guidance |
| `https://nextjs.org/docs/app/guides/static-exports` | Next.js static export constraints and behavior |

## Next Actions

| Description | Test |
|-------------|------|
| Create `just docs-gh-pub` so one command builds and publishes docs artifacts to the `gh-pages` branch | `docs_gh_pub_single_command_publishes` |
| Require publication flow to fail with a clear error when `bun` is unavailable | `docs_gh_pub_fails_without_bun` |
| Require publication flow to fail with a clear error when `origin` remote is missing | `docs_gh_pub_fails_without_origin_remote` |
| Configure docs build so GitHub Pages mode exports static output to `docs/out` | `docs_gh_pages_mode_exports_static_output` |
| Configure docs build so GitHub Pages mode applies repository base path derived from the remote repository name and generated links resolve under that base path | `docs_gh_pages_base_path_links_resolve` |
| Configure `api/search` route so GitHub Pages mode returns static-safe response behavior and does not fail export-time rendering | `search_route_static_safe_in_gh_pages_mode` |
| Configure `llms.mdx` extraction route so local docs mode keeps markdown extraction while GitHub Pages mode remains export compatible | `llms_mdx_route_export_compatible` |
| Require publication flow to write `.nojekyll` in the published branch output | `docs_gh_pub_writes_nojekyll` |
| Require publication sync step to preserve worktree git metadata while replacing published branch contents | `docs_gh_pub_preserves_worktree_git_metadata` |
| Add behavior so running publication with unchanged docs reports no-op instead of creating an empty commit | `docs_gh_pub_no_changes_is_noop` |
| Verify publication pushes updated static artifacts to `gh-pages` and outputs the publication success signal | `docs_gh_pub_pushes_artifacts_and_reports_success` |
