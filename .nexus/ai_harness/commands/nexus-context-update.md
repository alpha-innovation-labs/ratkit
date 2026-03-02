---
description: Update existing context files from a user prompt while enforcing context and project rules
---

# Command: Update Context

You are updating context documents from the user's prompt.

Your job:
- Interpret the requested change.
- Find the right context and index files.
- Apply updates that respect context standards and project standards.

## Rule Sources (read first)

Before editing, read and enforce:
1. `.nexus/ai_harness/skills/context-driven-development/SKILL.md`

This skill is the only CDD source of truth for context format, naming, structure, and index expectations.

## Required Behavior

1. Understand the user's requested context update.
2. Scan `.nexus/context/` and locate the most relevant files:
   - context specs: `PRJ_NNN-*.md`
   - project index: `.nexus/context/<project>/index.md`
   - feature index: `.nexus/context/<project>/<feature>/index.md`
3. If target file(s) are ambiguous, ask a focused `question` with candidate files.
4. Build an explicit file plan listing every context/index file that would be created or updated.
5. For each planned file, use the `question` tool to show what you plan to add/change and request approval before editing.
   - Include target path, create vs update intent, and short change summary.
   - Use options: `Approve` (Recommended), `Adjust`, `Skip`.
   - You may use one `question` call with multiple `questions` entries (one per file).
   - Edit only files explicitly approved by the user.
   - After approvals are returned, apply approved edits immediately in the same command run.
   - Do not pause for an extra user prompt after approvals unless every file is `Adjust` or `Skip`.
6. Update only what is necessary and preserve existing valid structure.
7. Keep context specs as specifications, not implementation notes:
   - no code blocks unless already required as reference artifacts
   - no low-level implementation steps
   - outcomes and Next Actions must be E2E-observable
8. Enforce required sections for context specs:
    - `## Desired Outcome`
    - `## Next Actions` (table with `Description` and `Test` columns)
    - `## Reference` only when needed; remove if empty
    - optional `depends_on` frontmatter only for blocking prerequisites
9. Update related project/feature `index.md` files when the user request changes scope, dependencies, interfaces, or operational knowledge.
10. If the request implies a split into multiple outcomes, propose split candidates and confirm using `question` before building the per-file approval plan.

## Editing Rules

- Prefer updating existing context files over creating new ones.
- Create a new context file only when no existing context can be safely extended.
- If creating a new context file, keep project-scoped ID ordering (`PRJ_NNN`) and naming conventions.
- Preserve frontmatter fields and keep metadata consistent.
- If `depends_on` is present, keep only valid blocking project/context references.
- Do not rename files unless necessary for correctness.

## Output Format

1. Files Updated
2. Rule Sources Applied
3. What Changed
4. Open Questions / Follow-ups

After presenting the summary, use the `reporting` tool with:
- input: the full summary
- sound: /System/Library/Sounds/Basso.aiff
- notificationTitle: "Context Update"
- notificationBody: first lines of the summary
