---
context_id: ZDO_003
title: Code Docs Automatic Sync
project: zenbt-docs
created: "2026-02-18"
---

# ZDO_003: Code Docs Automatic Sync

## Desired Outcome

Documentation content updates automatically from source code documentation comments so API and primitive reference pages stay current with implementation changes, reducing manual drift between code and docs.

## Reference

| Reference File | Coverage |
|----------------|----------|
| `_reference/fumadocs-default-stack.md` | Fumadocs content tree and command expectations for generated docs |
| `_reference/docs-code-sync-blueprint.md` | Extraction model, deterministic generation, atomic writes, CI checks |
| `_reference/docs-implementation-blueprint.md` | Integration points between generated content and docs routes/navigation |

| Sync Contract | Requirement |
|---------------|-------------|
| Freshness | Docs generation reflects latest source changes on each sync run |
| Stability | Re-running sync without code changes produces no content diff |
| Visibility | Sync failures produce actionable output instead of silent partial docs |

## Next Actions

| Description | Test |
|-------------|------|
| Require generated reference content to be discoverable through Fumadocs navigation trees | `generated_reference_is_in_fumadocs_nav` |
| Require generated reference documentation outputs to be written under `docs/content/docs/reference/` | `generated_reference_writes_to_docs_content` |
| Create docs sync workflow that reads Rust source documentation and generates docs content pages | `code_docs_sync_generates_pages` |
| Add stable output formatting so repeated sync runs are deterministic | `code_docs_sync_is_deterministic` |
| Require generated docs pages to be included in docs navigation and page discovery | `generated_docs_are_navigable` |
| Add failure behavior so malformed or missing code docs surface clear sync errors | `code_docs_sync_failures_are_clear` |
| Verify local docs preview via `just docs` includes up-to-date generated reference pages | `generated_docs_preview_works_end_to_end` |
| Verify generated docs visibility on `/docs` using `agent-browser` against `http://localhost:3000/docs` | `agent_browser_confirms_generated_docs_visibility` |
