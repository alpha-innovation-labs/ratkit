---
description: Analyze conversation and recommend context/project updates
---

# Command: Context Sync

This command is analysis-only.

Its job is to read the current conversation, analyze existing context files, and report:
1. what Next Actions should be added or updated
2. what project-level docs should be updated with new knowledge

## Usage

```bash
/sync-context
```

No parameters required.

## Hard Rules

1. Do not call other slash commands.
2. Do not use interactive flows (`question` tool).
3. Do not spawn subagents.
4. Do not modify files automatically.
5. Do not remove existing Next Actions; only propose additions/edits.

## Inputs To Analyze

1. Current conversation (source of new requirements/constraints/decisions)
2. `.nexus/context/**/PRJ_NNN-*.md` context files
3. `.nexus/context/**/index.md` project operational docs
4. `.nexus/rules/CONTEXT.md` format requirements

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
- Keep actions E2E-observable per `.nexus/rules/CONTEXT.md`.

### Phase 4: Propose Project-Level Knowledge Updates

For each affected project `index.md`, propose additions for relevant sections:
- Overview
- Architecture
- CLI Usage
- Key Dependencies
- Environment Variables
- Debugging & Troubleshooting

Only propose changes supported by conversation evidence.

### Phase 5: Return Report (No File Writes)

Return a concise report in this structure:

```markdown
# Context Sync Report

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

## Important Notes

- This command recommends changes only; it does not apply them.
- Be explicit about evidence so follow-up edits are straightforward.
- If nothing needs updating, return an empty-change report and say why.
