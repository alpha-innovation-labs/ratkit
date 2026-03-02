---
description: Generate red-only test files from a context file Next Actions table
---

# Command: Generate Red Tests From Context

Generate test files from a context file using red/green TDD discipline, but execute only the red phase.

## Purpose

Given one context file, parse its `## Next Actions` table and generate one failing test file per row.

## Required Inputs

- Context file path under `.nexus/context/`
- Optional test extension override (default `.rs`)

If context file path is missing, ask for it using `question`.

## Hard Rules

1. Red phase only: write tests expected to fail first.
2. Do not implement production code.
3. Do not modify application source files.
4. One test file per `Next Actions` row.
5. Use `Test` identifiers exactly as context source-of-truth.
6. If a `Test` value already starts with `test_`, remove the prefix for canonical naming.
7. Never merge multiple `Next Actions` rows into one test file.

## Path Mapping

For context file:

`.nexus/context/<project>/<sub-project>/.../<context-file>.md`

Generate tests under:

`tests/<project>/<sub-project>/.../<context-file-stem>/`

One file per Next Action row:

`tests/<project>/<sub-project>/.../<context-file-stem>/test_<test_id><ext>`

Examples:

- Context: `.nexus/context/nexus-cli/cdd/CDD_003-context-test-generation-and-discovery-gate.md`
- Output dir: `tests/nexus-cli/cdd/cdd_003-context-test-generation-and-discovery-gate/`
- Output file: `test_context_implement_extracts_unique_test_identifiers_from_next_actions.rs`

## Workflow

1. Read `.nexus/ai_harness/skills/context-driven-development/SKILL.md`.
2. Read `.nexus/ai_harness/rules/e2e-tests/SKILL.md` if present.
3. Validate context path is inside `.nexus/context/`.
4. Parse frontmatter (`context_id`, `project`, `feature`) and `## Next Actions` table.
5. Extract each row's `Description` and `Test` values.
6. Validate extracted `Test` values:
   - snake_case required
   - unique within the file
   - normalize by removing optional leading `test_`
7. Build file plan and show preview with one line per generated file.
8. Ask approval with `question` before writing:
   - `Approve` (Recommended)
   - `Adjust`
   - `Cancel`
9. On approval, generate all test files in target directory.
10. Ensure red state:
    - Run the relevant test command (or repository default)
    - Confirm generated tests fail
    - If any generated test passes, mark it as warning in summary and explain why this violates red phase intent
11. Do not proceed to implementation/green phase.

## Test File Content Expectations

- Include exactly one test case per file.
- Name test function `test_<test_id>`.
- Use the `Description` column as the behavior intent for the test body.
- Prefer explicit failure placeholders (for example, `panic!("TODO: implement")`) when needed to guarantee red state.

## Output Format

1. Context Parsed
2. Files Generated
3. Red Test Verification
4. Follow-up (what is needed to move to green)

After presenting the summary, use the `reporting` tool with:
- input: the full summary
- sound: /System/Library/Sounds/Basso.aiff
- notificationTitle: "Red Tests"
- notificationBody: first lines of the summary
