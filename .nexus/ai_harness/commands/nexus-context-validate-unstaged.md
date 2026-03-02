---
description: Validate unstaged code and tests against context Next Actions and language rules
---

# Command: Validate Unstaged Against Context

Validate current unstaged changes against a context `Next Actions` table and the applicable language rule set.

## Purpose

Ensure that in-progress (unstaged) implementation and test changes:
- align with the selected context outcomes,
- map to `Next Actions` `Test` identifiers,
- and comply with repository language/framework rules.

## Inputs

- Required: context file path under `.nexus/context/`
- Optional: explicit `Next Actions` table text override

If context path is missing, ask for it using `question`.

## Validation Scope

Use only unstaged changes:
- `git diff --name-only`
- `git diff`

Do not include staged-only changes unless the user explicitly asks.

## Workflow

1. Read `.nexus/ai_harness/skills/context-driven-development/SKILL.md` first.
2. Validate that the context file exists under `.nexus/context/`.
3. Parse `## Next Actions` table from:
   - explicit table text if provided, otherwise
   - the context file.
4. Extract all `Test` identifiers and normalize by removing optional leading `test_`.
5. Collect unstaged changed files and patch content.
6. Discover available coding rules by listing `.nexus/ai_harness/rules/*/SKILL.md`.
7. Determine the primary applicable rule from changed file types and project context:
   - `*.rs` -> `rust`
   - `*.py` -> `python`
   - Next.js/web app paths (`apps/web`, `next.config.*`, `app/`, `features/` with TS/JS/TSX/JSX) -> `nextjs`
   - `justfile`/`Justfile` -> `justfile`
8. If multiple rules are plausible, ask the user to select exactly one prioritized rule via `question`.
9. Validate unstaged changes against the selected rule's hard constraints.
10. Validate Next Actions traceability:
    - each changed/added test should map to one extracted `Test` identifier,
    - report missing tests for identifiers that have no matching unstaged test change,
    - report extra unstaged tests that do not map to any identifier.
11. Run relevant tests for changed scope when a deterministic command is available and report pass/fail.
12. Produce findings with explicit severity and exact file references.

## Severity Levels

- `blocker`: rule violation or missing required test mapping
- `warning`: likely mismatch, unclear ownership, or non-deterministic validation gap
- `info`: useful note with no immediate action required

## Output Format

1. Context and Next Actions Parsed
2. Unstaged Change Scope
3. Selected Rule
4. Validation Findings
5. Test Execution Result
6. Recommended Fixes

## Hard Rules

1. Never silently rewrite files in this command.
2. Never stage/commit changes in this command.
3. If the context file is malformed or missing `Next Actions`, stop and ask for clarification.
4. If no unstaged changes exist, return a clear no-op validation report.

After presenting the summary, use the `reporting` tool with:
- input: the full summary
- sound: /System/Library/Sounds/Basso.aiff
- notificationTitle: "Validation"
- notificationBody: first lines of the summary
