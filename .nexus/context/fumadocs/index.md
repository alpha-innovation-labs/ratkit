# Overview

`zenbt-docs` defines the documentation experience for the project, including the landing page, internal docs layout, and automated content synchronization from code documentation.

# Architecture

```text
Rust Source + Doc Comments
            |
            v
   Docs Content Generator
            |
            v
   MDX/Markdown Content Tree
            |
            v
  Docs App (Landing + Docs UI)
            |
            v
   Local Serve via `just docs`
```

# CLI Usage

- `just docs` serves the documentation site for local preview.
- `just docs` reflects both static docs pages and generated reference content.
- Default workflow: start from stock Fumadocs setup, apply `_reference/fumadocs-default-patch-pack.md`, then focus on MDX content changes.

# Key Dependencies

| Dependency | Purpose |
|------------|---------|
| `fumadocs-ui` | Render home and docs layouts, page wrappers, and docs UI primitives |
| `fumadocs-core` | Source loading and docs search integration |
| `fumadocs-mdx` | MDX docs collections and metadata schemas |
| Rust doc extraction tooling | Convert code documentation into docs content |

# Environment Variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `DOCS_PORT` | `3000` | Local docs server port |
| `DOCS_BASE_URL` | empty | Optional base URL for deployed docs |

# Debugging & Troubleshooting

- If right-side TOC does not appear, confirm page headings are present and included in rendered content.
- If generated API docs are stale, verify extraction step ran before `just docs` preview.
- If landing page layout drifts, compare spacing and section order against the defined docs context contracts.
