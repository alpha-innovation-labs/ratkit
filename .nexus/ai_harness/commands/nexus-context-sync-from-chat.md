---
description: Analyze conversation and recommend context/project updates from chat
---

# Command: Context Sync From Chat

This command performs analysis first, then applies approved file updates in the same run.

Read `.nexus/ai_harness/skills/context-driven-development/SKILL.md` first and use it as the only CDD source of truth.

Its job is to read the current conversation, analyze existing context files, and report:
1. what Next Actions should be added or updated
2. what project-level docs should be updated with new knowledge

## Usage

```bash
/nexus-context-sync-from-chat
```

No parameters required.

## Hard Rules

1. Do not call other slash commands.
2. Do not spawn subagents.
3. Do not modify files before collecting approvals.
4. Do not remove existing Next Actions; only propose additions/edits.
5. Use exactly one `question` tool call at the end, with one approval question per proposed file update/create plus an optional final next-step question.
6. After receiving `question` answers, immediately execute approved file updates in this same command run; do not wait for another user prompt.

## Inputs To Analyze

1. Current conversation (source of new requirements/constraints/decisions)
2. `.nexus/context/**/PRJ_NNN-*.md` context files
3. `.nexus/context/**/index.md` project operational docs
4. `.nexus/ai_harness/skills/context-driven-development/SKILL.md` format requirements

## Workflow

### Phase 1: Extract Conversation Facts

Identify concrete, user-facing information from the current conversation:
- New desired outcomes
- New constraints or requirements
- New behaviors that should be testable
- New operational knowledge (commands, env vars, troubleshooting notes)

Ignore speculative implementation details.

### Phase 2: Map Facts To Contexts and Projects

1. Match each fact to an existing context in `.nexus/context/` by topic/project/context ID.
2. If no existing context fits, mark as "new context candidate".
3. Determine which project folders are affected and should receive doc updates.

### Phase 3: Propose Next Action Changes

For each matched context:
1. Read the `## Next Actions` table.
2. Compare with extracted conversation facts.
3. Propose only missing or clearly outdated rows.

Formatting rules for each proposal:
- `Description`: human-readable, starts with a verb.
- `Test`: snake_case without `test_` prefix.
- Keep actions E2E-observable per `.nexus/ai_harness/skills/context-driven-development/SKILL.md`.

### Phase 4: Propose Project-Level Knowledge Updates

For each affected project `index.md`, propose additions for relevant sections:
- Overview
- Architecture
- CLI Usage
- Key Dependencies
- Environment Variables
- Debugging & Troubleshooting

Only propose changes supported by conversation evidence.

### Phase 5: Return Report

Return a concise report in this structure:

```markdown
# Context Sync From Chat Report

## Context Updates

### <context_id> - <context file path>
- Proposed Next Action: <description> | `<test_name>`
- Reason: <conversation fact>

## Project Docs Updates

### <project> - <index.md path>
- Section: <section name>
- Proposed addition: <what to add>
- Reason: <conversation fact>

## New Context Candidates

- <project or area>: <one-line desired outcome>

## No-Change Contexts

- <context_id>: already aligned with conversation
```

Also include a `## Proposed File Plan` section listing each file that should be created or updated, with:
- file path
- create vs update intent
- concise summary of planned additions/changes

### Phase 6: Collect File Approvals With `question`

After the report, call the `question` tool exactly once.

That single call must include:
1. One file-approval question per planned file in `## Proposed File Plan`.
2. Optional final next-step question.

For each file-approval question, use options:
- `Approve` (Recommended): apply this file now in this command run
- `Adjust`: revise this file proposal first
- `Skip`: do not apply this file

Only approved files should be applied.

Requirements:
- Put the recommended option first and label it with `(Recommended)`.
- Keep options concise and actionable.
- Keep `custom` enabled so the user can type their own response.
- Use `multiple: false` for each question.

Suggested options:
1. Apply the proposed Next Action and project doc edits (Recommended)
2. Apply only context Next Action updates
3. Apply only project index.md updates
4. Create new context file(s) for candidates only
5. Show a patch preview first

### Phase 7: Apply Approved Updates Immediately

After the `question` response returns:

1. Determine approved files and scope choice.
2. Apply all approved edits immediately.
3. If a file is marked `Adjust`, revise proposal first, then apply after confirmation in the same flow.
4. If all files are skipped, return a no-change confirmation.

## Important Notes

- This command must not stop after approvals; execute approved edits immediately.
- Be explicit about evidence so follow-up edits are straightforward.
- If nothing needs updating, return an empty-change report and say why.
