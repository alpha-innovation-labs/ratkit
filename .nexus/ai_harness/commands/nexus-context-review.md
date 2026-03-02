---
description: Review context files for Context-Driven Development compliance
---

# Command: Review Contexts

Review existing context files and index docs for compliance with:

- `.nexus/ai_harness/skills/context-driven-development/SKILL.md`

Read the CDD skill first and treat it as the only source of truth.

## Purpose

Audit context quality and coverage, identify violations, and propose specific fixes before applying changes.

## Workflow

1. Select scope:
   - Ask via `question` tool: all contexts or specific project.
   - Scan under `.nexus/context/` (exclude `_reference/` and `_legacy/`).
2. Audit numbered context specs (`PRJ_NNN-*.md`) for CDD compliance:
    - Required frontmatter fields.
    - Optional `depends_on` shape and dependency reference validity (project/context).
    - Required section order (`Desired Outcome`, optional `Reference`, `Next Actions`).
    - One-outcome clarity and no implementation-level detail.
    - `Next Actions` table format and E2E-observable actions.
    - Naming and location under `.nexus/context/<project>/<feature>/`.
3. Audit project/feature `index.md` docs:
   - Presence and relevance of operational knowledge sections.
   - Alignment with recently updated contexts.
4. Present issues one-by-one with proposed fixes using `question`.
5. Present final summary of approved changes and request a final confirmation.
6. Apply only approved changes.

## Hard Rules

1. Use CDD skill as the single source of truth.
2. Never invent alternative context formats.
3. Preserve valid existing content; only change what is necessary.
4. If splitting is needed, propose and confirm before writing.

## Output Format

1. Scope Reviewed
2. Issues Found
3. Approved Fixes Applied
4. Remaining Recommendations

After presenting the summary, use the `reporting` tool with:
- input: the full summary
- sound: /System/Library/Sounds/Basso.aiff
- notificationTitle: "Context Review"
- notificationBody: first lines of the summary
