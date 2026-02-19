---
context_id: ZDO_002
title: Internal Docs Layout And TOC
project: zenbt-docs
created: "2026-02-18"
---

# ZDO_002: Internal Docs Layout And TOC

## Desired Outcome

The internal documentation pages deliver high-clarity navigation in a Fumadocs docs layout, including a persistent right-side table of contents, so users can traverse long technical pages quickly and keep orientation across sections.

## Reference

| Reference File | Coverage |
|----------------|----------|
| `_reference/fumadocs-default-patch-pack.md` | Default Fumadocs setup and exact TOC/layout patch sequence |

| Navigation Contract | Requirement |
|---------------------|-------------|
| Sidebar navigation | Section tree navigation remains available and stable across docs routes |
| Right-side TOC | Heading anchors are generated and clickable for long-form pages |
| Mobile behavior | TOC and section navigation remain accessible in mobile viewport patterns |

## Next Actions

| Description | Test |
|-------------|------|
| Require internal docs pages to remain on default Fumadocs route/layout primitives with patch-pack adjustments only | `fumadocs_layout_primitives_are_used` |
| Create docs content layout route with stable left navigation for section trees | `docs_sidebar_navigation_works` |
| Add right-side TOC rendering based on page headings for internal docs pages | `docs_right_toc_renders` |
| Require TOC links to navigate to the correct in-page heading anchors | `toc_links_jump_to_headings` |
| Add behavior so long docs pages preserve orientation while scrolling | `long_page_navigation_orientation_holds` |
| Verify internal docs layout and TOC behavior in local preview via `just docs` | `internal_layout_preview_works_end_to_end` |
| Verify rendered `/docs` page behavior via `agent-browser` at `http://localhost:3000/docs` | `agent_browser_confirms_docs_layout` |
