# Docs Code Sync Blueprint

## Goal

Auto-generate reference docs from Rust code documentation comments and keep docs navigation in sync without manual page-by-page maintenance.

This reference is implementation-focused and intended to be used alongside `ZDO_003`.

## Default Framework Requirement

Generated content must target the Fumadocs content system directly.

- Output shape must align with `fumadocs-mdx` docs collections.
- Navigation metadata must be compatible with Fumadocs `meta.json` expectations.

## Sync Architecture

```text
Rust crates (src/**/*.rs)
        |
        v
Doc extractor (AST/rustdoc parser)
        |
        v
Normalized doc model (JSON)
        |
        v
MDX renderer + nav updater
        |
        v
docs/content/docs/reference/**
```

## Source Parsing Contract

Input scope:

- Public structs
- Public enums
- Public traits
- Public functions and methods
- Module-level docs

Doc sources:

- Rust doc comments (`///`, `//!`)
- Type signatures
- Public field metadata

Normalization output fields:

- `path` (module path)
- `kind` (`struct`, `enum`, `trait`, `fn`, `module`)
- `name`
- `signature`
- `docs_markdown`
- `since` (optional)
- `deprecated` (optional)

## Output Content Contract

Generated docs should land under `docs/content/docs/reference/` with structure inferred from source modules (not hardcoded section names).

One valid example:

```text
docs/content/docs/reference/
  commands/
  cli/
  output/
```

The agent implementing docs sync decides the final section layout from the codebase itself. Sections must emerge from discovered modules/symbols and adapt as code structure changes.

Each generated page includes:

1. Title
2. Short description
3. Signature block
4. Field/member table when applicable
5. Examples section if source docs contain examples
6. Related items section (same module)

## Navigation Sync Contract

- `meta.json` entries are generated or updated with deterministic ordering.
- Ordering rule: module path asc, then item name asc.
- Generated pages are marked as generated in frontmatter.
- Manual sections remain untouched.

### Fumadocs-Specific Navigation Rules

- Root-level `docs/content/docs/meta.json` keeps explicit section ordering.
- Generated reference sections include their own `meta.json` files and are derived from discovered source structure.
- Slugs must resolve through `source.getPage(...)` under `/docs` base URL.

## Determinism Rules

- Stable slug generation from module path and item name.
- Stable heading generation and ordering.
- Stable table column ordering.
- Trailing whitespace and frontmatter key order normalized.
- Re-run with no code changes produces zero diff.

## Failure Handling Contract

Hard failures:

- Parse error in targeted Rust files.
- Write failure to docs content directory.
- Invalid output schema.

Behavior:

- Non-zero exit code.
- Error summary includes file path and symbol path.
- No partial navigation corruption (atomic write strategy).

## Atomic Write Strategy

1. Render all outputs to temp directory.
2. Validate generated pages and meta files.
3. Replace target generated subtree in one move/swap operation.
4. Recompute and write section-level `meta.json` files.

## Suggested Command Surface

| Command | Purpose |
|--------|---------|
| `just docs-sync` | Generate docs from Rust code and refresh navigation |
| `just docs-sync-check` | Verify generated output is up to date (CI mode) |
| `just docs` | Serve docs locally, consuming generated pages |

## Fumadocs Source Integration Contract

- `source.config.ts` includes frontmatter and meta schema definitions.
- Postprocessing includes processed markdown for copy/export actions.
- `lib/source.ts` loader sees generated files without manual registration.

## Example Generated Frontmatter Contract

```md
---
title: ExchangeMain
description: Unified exchange facade
generated: true
source_path: src/core/exchange/mod.rs
symbol_path: crate::core::exchange::ExchangeMain
---
```

## Example Generation Steps

1. Discover crates and source files.
2. Extract public symbol docs.
3. Normalize model.
4. Render MDX pages.
5. Update `meta.json` trees.
6. Validate links and TOC headings.
7. Write atomically.

## CI Verification Contract

- `just docs-sync-check` fails if generated docs drift.
- CI logs include missing pages and mismatched checksums.
- Build blocks merge on drift.

## Minimal Implementation Skeleton (Pseudo)

```text
collect_symbols() -> normalize() -> render_pages() -> render_meta() -> validate() -> atomic_commit()
```

## Quality Checklist

- Generated pages are readable without raw Rust expertise.
- Symbols are grouped by module for discoverability.
- Broken links are detected during sync.
- Existing authored docs are preserved.
