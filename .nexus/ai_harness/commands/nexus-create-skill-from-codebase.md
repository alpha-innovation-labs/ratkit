---
description: Orchestrates subagents to generate a teaching-oriented project SKILL.md using synced include/exclude folder scope
---

You are the primary execution agent for this command and must orchestrate the workflow directly.

Generate one project skill document at:
- `.nexus/ai_harness/skills/<project-name>/SKILL.md`

Also write generation state at:
- `.nexus/ai_harness/skills/<project-name>/skills-state.json`

## Core Behavior

1. Resolve and confirm project name.
2. Build, confirm, and sync include/exclude folder scope.
3. Determine generation mode (`full` or `incremental`) from per-project state.
4. Spawn domain subagents for included code-bearing domains.
5. Assemble one coherent teaching-oriented `SKILL.md`.
6. Validate, then write outputs.

Do not spawn a separate orchestration agent.

## Project Name Resolution (mandatory)

Detect in this order:
1. `Cargo.toml` -> `[package].name`
2. `package.json` -> `name`
3. `pyproject.toml` -> `project.name`

Confirm with `question`:
- `Use detected name (Recommended)`
- `Provide different name`

If detection fails, request free-form project name.
Normalize folder name segment to kebab-case.

## Scope Discovery and Cache Sync (mandatory)

Before mode selection and subagent work, do folder-scope synchronization.

### A. Inventory

1. Scan **all repository root folders** (including dot-folders), excluding gitignored paths.
2. Build an exhaustive root-folder inventory.
3. This inventory must include folders like `.nexus` and `.opencode` when present and not ignored.

### B. Read existing scope cache

Read `.nexus/config.json` and use this per-project cache key:

```json
{
  "skill_generation": {
    "projects": {
      "<project-name>": {
        "include_folders": ["..."],
        "exclude_folders": ["..."],
        "updated_at": "<ISO-8601>"
      }
    }
  }
}
```

If `.nexus/config.json` exists, preserve existing keys and merge this section.

### C. First-run behavior (no cache for project)

1. Propose a full partition of inventory into:
   - include_folders
   - exclude_folders
2. Lists must be exhaustive and disjoint.
3. Ask user to confirm with `question`:
   - `Use detected scope (Recommended)`
   - `Adjust scope`
4. If adjusted, ask with two multi-select checkbox lists (`multiple: true`):
   - `Folders to include` (show current include list)
   - `Folders to exclude` (show current exclude list)
5. Interpret checked items as **wrongly placed** and move them to the opposite list:
   - checked in `Folders to include` -> move include -> exclude
   - checked in `Folders to exclude` -> move exclude -> include
6. Reconcile and enforce:
   - lists must be disjoint (if overlap, ask user to resolve)
   - combined include+exclude must be exhaustive over current inventory
7. Write resolved lists to `.nexus/config.json` under `skill_generation.projects.<project-name>`.

### D. Subsequent-run behavior (cache exists)

1. Re-scan current root-folder inventory.
2. Compare with cached include/exclude lists.
3. If any folders are not present in either cached list, treat as **unclassified**.
4. Ask user to classify unclassified folders (include vs exclude), then update lists.
5. Ask one final confirmation with `question`:
   - `Use synced scope (Recommended)`
   - `Adjust scope`
6. If adjusted, re-open both multi-select checkbox lists using current cached partition:
   - `Folders to include`
   - `Folders to exclude`
7. Interpret checked items as wrongly placed and move them to the opposite list.
8. Re-validate disjoint + exhaustive rules.
8. Persist updated lists back to `.nexus/config.json`.

### E. Scope enforcement rules

1. Domain subagents may use only `include_folders` plus root manifests/operational files.
2. Excluded folders must not be used for domain generation.
3. `docs` must always remain excluded for skill-generation domain content.

## Objective

Produce a teaching-oriented `SKILL.md` for another LLM or human:
- high-level project purpose and architecture
- major runtime surfaces and when to use each
- conventions and constraints that preserve compatibility
- practical playbooks for contribution and usage

This command is not a file-link catalog generator.

## Non-Negotiable Constraints

1. Generate exactly one artifact: `SKILL.md`.
2. State file is per-project and named `skills-state.json`.
3. Use verified facts only.
4. Use committed history only as generation input.
5. Ignore unstaged/staged/untracked working-tree changes for generation decisions.
6. Keep output concise, technical, instruction-first, and high-level.
7. Do not generate domain content from `docs/`.

## Mode and State

State path:
- `.nexus/ai_harness/skills/<project-name>/skills-state.json`

State schema:

```json
{
  "skill_path": ".nexus/ai_harness/skills/<project-name>/SKILL.md",
  "last_generated_commit": "<sha>",
  "generated_at": "<ISO-8601>"
}
```

Mode selection:
- Run `git status --porcelain` first.
- If dirty, stop with blocking message.
- If `SKILL.md` or `skills-state.json` missing -> `full`.
- Else -> `incremental` using `last_generated_commit..HEAD`.

## Orchestration Workflow

Before subagents, read:
- `.nexus/ai_harness/skills/skill-from-codebase/SKILL.md`

### Phase 1A: Full Mode Planning

- Infer domain split from included code folders.
- Choose minimum useful subagent count.
- Record internal plan: domain split, rationale, assignments.

### Phase 1B: Incremental Impact Analysis

- Compute committed changes from `last_generated_commit..HEAD`.
- Include add/edit/rename/delete.
- Map changed paths to impacted sections.
- Escalate to full mode if structural/broad changes.

### Phase 2: Domain Subagents

Each domain subagent returns:
- high-level domain summary (what and why)
- actionable rules/constraints
- common pitfalls
- contributor usage guidance
- uncertainties/conflicts

Subagents do not write files.

### Phase 3: Assembly Subagent

Provide assembly agent:
- domain outputs
- mode context
- existing SKILL content (incremental)
- required shape constraints

Assembly must:
- produce one coherent, teaching-first skill
- avoid path-dump/link-dump formatting
- remove overlap/contradictions/speculation
- in incremental mode, edit only impacted sections unless escalated

### Phase 4: Validator Subagent

Validator checks:
- structure/section order
- factual correctness
- version correctness vs manifest
- no speculative claims
- no docs-domain content
- scope-sync compliance (all root folders classified)
- incremental discipline unless escalated

If fail, iterate assembly + validator until pass.

## Required `SKILL.md` Shape

Section order:

`# <project-name>`

`> 1-2 sentence operational summary`

One short paragraph on how to use this skill.

`## Agent Operating Rules`
`## Environment and Version Constraints`
`## Quick Task Playbooks`
`## Getting Started`
`## Workspace Overview`
`## <Domain 1>`
`## <Domain 2>`
`...` (inferred from included domains)
`## Usage Cards`
`## API Reference`
`## Common Pitfalls`
`## Optional`

Style:
- prioritize high-level architecture and behavior contracts
- keep path mentions minimal and purposeful
- no internal-link catalogs

## Usage Card Requirements

Infer dominant entity types (commands, services, modules, runtime surfaces, APIs).

Each card must include:
- `Use when`
- `Enable/Install`
- `Import/Invoke`
- `Minimal flow` (2-4 steps)
- `Key APIs`
- `Pitfalls`
- `Source` (brief, non-catalog style)

## Quality Gate

- exactly one H1
- required section order present
- no duplicate entries
- no unverifiable claims
- version matches manifest
- docs-domain excluded from generated content
- all current root folders are classified in include/exclude cache
- output is teaching-oriented and high-level
- incremental edits touch only impacted sections unless escalated
- generation inputs are committed-history only

## Finalization

1. Write `.nexus/ai_harness/skills/<project-name>/SKILL.md` after validator pass.
2. Write/update `.nexus/ai_harness/skills/<project-name>/skills-state.json` after successful write.
3. Ensure `.nexus/config.json` has synced include/exclude scope for this project.
4. Return completion note with:
   - resolved project name
   - mode
   - clean-tree status
   - baseline/current commit
   - changed files considered (incremental)
   - included/excluded folder lists used
   - domain split
   - subagent count
   - validations performed

After responding, call `reporting` with:
- `sound`: `/System/Library/Sounds/Glass.aiff`
- `notificationTitle`: `Skill Generate`
- `notificationBody`: first lines of completion note
