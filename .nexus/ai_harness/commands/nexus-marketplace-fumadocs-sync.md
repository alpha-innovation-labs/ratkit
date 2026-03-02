---
description: Detect changes since last Fumadocs sync and run an LLM-guided Fumadocs update
---

# Nexus Fumadocs Sync

Use this command to keep Fumadocs documentation aligned with repository changes.

## Source of Truth

- `.nexus/marketplace/fumadocs/context/index.md`
- `.nexus/marketplace/fumadocs/context/ZDO_*.md`
- `.nexus/marketplace/fumadocs/context/_reference/*.md`

## State File

Use checkpoint state inside:

- `.nexus/config.json`
- key path: `marketplace.fumadocs.sync_state`

Expected shape:

```json
{
  "marketplace": {
    "fumadocs": {
      "sync_state": {
        "docs_path": "docs/content/docs",
        "last_synced_commit": "<git-sha>",
        "synced_at": "<utc-iso8601>"
      }
    }
  }
}
```

## Workflow

1. Determine change range:
   - If `.nexus/config.json` has `marketplace.fumadocs.sync_state.last_synced_commit` and commit is valid, use `<last_synced_commit>..HEAD`.
   - Otherwise, use `@{upstream}...HEAD` when upstream exists.
   - If no upstream exists, use `HEAD` (latest commit only).
2. Collect changed paths from that range plus staged changes:
   - `git diff --name-only --diff-filter=ACMRTUXB <range>`
   - `git diff --cached --name-only --diff-filter=ACMRTUXB`
3. Keep only doc-relevant paths:
   - `src/**`
   - `.nexus/ai_harness/**`
   - `.nexus/context/**`
   - `docs/**`
   - `README.md`
   - `llms.txt`
4. If no relevant paths remain, report "no Fumadocs updates needed" and stop.
5. Build a concise prompt that includes:
   - changed-path list
   - instruction to update Fumadocs docs only
   - targets allowed for edits:
     - `docs/**`
     - `README.md`
     - `llms.txt`
6. Resolve harness from `.nexus/config.json`:
   - Use `harness` value when present.
   - Default to `opencode` when missing.
7. Run harness visibly in CLI (no explicit agent flag):

```bash
<harness> --prompt "<assembled prompt>"
```

8. After successful update, write/update `.nexus/config.json` at `marketplace.fumadocs.sync_state` with `docs_path`, current `HEAD`, and UTC timestamp.

## Hard Rules

- Do not edit Rust/TS source files as part of this command.
- Do not modify git config.
- Do not run destructive git commands.
- If OpenCode update fails, do not update checkpoint state.

## Output Format

1. Change Range Used
2. Relevant Paths
3. Files Updated
4. New Checkpoint
