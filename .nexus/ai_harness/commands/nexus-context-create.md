---
description: Create new context specifications from user goals
---

# Command: Create Context

You are creating context specification(s) following `.nexus/ai_harness/skills/context-driven-development/SKILL.md`.

Read that skill first and treat it as the only CDD source of truth.

## Workflow

### 1. Understand What The User Wants

- If user already described their goal: acknowledge it and proceed to scanning
- If unclear, use the `question` tool:
  ```json
  {
    "questions": [{
      "question": "What are you trying to accomplish?",
      "header": "User Goal",
      "options": []
    }]
  }
  ```
- Have a brief conversation to understand the desired outcome

### 2. Scan Existing Contexts

Before creating anything, **quickly scan all files in `.nexus/context/`**:

1. **Read all context file Desired Outcomes** - Look for similar desired outcomes
2. **Grep for keywords** - Search for key terms from the user's request
3. **Check Next Actions sections** - See if any existing context already covers this work

**If overlap found:**
- Show the user the existing context
- Use the `question` tool:
  ```json
  {
    "questions": [{
      "question": "I found `PRJ_NNN-name.md` with outcome: '[outcome]'. This seems related. What would you like to do?",
      "header": "Context Overlap",
      "options": [
        {"label": "Update existing", "description": "Update the existing context file"},
        {"label": "Create new", "description": "Create a new context file"},
        {"label": "Explain difference", "description": "Explain the difference between them"}
      ]
    }]
  }
  ```

**If already done:**
- Tell the user: "This appears to already be covered by `PRJ_NNN-name.md`. Would you like to review it instead?"

### 3. Determine If This Needs Multiple Contexts

Apply the core principles:
- **One outcome per context** - If the user's request has multiple distinct outcomes, split them
- **Next action principle** - Each context should be completable in a single session
- **Simplicity** - If it feels complex, split it

If splitting is needed:
- Explain to the user: "This looks like [N] separate outcomes. I'll create [N] context files..."
- List the proposed contexts with their outcomes
- Use the `question` tool:
  ```json
  {
    "questions": [{
      "question": "Does this split make sense? [List the N proposed contexts with outcomes]",
      "header": "Context Split",
      "options": [
        {"label": "Yes, proceed", "description": "Create all N context files as proposed"},
        {"label": "Adjust", "description": "Let me adjust the breakdown"}
      ]
    }]
  }
  ```

### 4. Identify Project, Feature, and Determine Context ID

**Scan `.nexus/context/` for existing project directories:**
- If adding to an existing project, use that project's prefix
- If creating a new project, ask user for the project name and derive a 3-letter prefix

**Determine feature folder (`kebab-case`)**
- If request clearly maps to an existing feature under `.nexus/context/<project>/`, use that feature folder.
- If request introduces a new feature area, create `.nexus/context/<project>/<feature>/`.
- If unclear, ask user to choose a feature name.

**Auto-determine Context ID (NEVER ask the user):**
- Scan `.nexus/context/<project>/**` for existing `PRJ_NNN-*.md` files across all features
- Find the highest NNN number
- Use the next number for new context(s)

**ID stability rules:**
- Never renumber existing context IDs automatically.
- Never rename existing context files for ordering cleanup.
- Keep existing gaps in numbering; append new contexts using next available NNN.
- If the user explicitly requests renumbering or mass renaming, propose a separate maintenance operation and require explicit confirmation first.

**Overlap check priority:**
1. Same feature (`.nexus/context/<project>/<feature>/`)
2. Entire project (`.nexus/context/<project>/**`)

### 5. Gather Information

Propose based on the conversation:
- **Desired Outcome**: What success looks like (one paragraph)
- **Next Actions**: Table with Description and Test columns
- **Blocking Dependencies** (optional): prerequisite projects and/or contexts that must be completed first

If clarification is needed, use the `question` tool with appropriate options.

### 6. Final Check

Before writing files, build a concrete file plan and require approval per item.

For each proposed context file, include:
- target path
- create vs update intent
- 1-2 line summary of what will be added or changed (`Desired Outcome` + key `Next Actions`)

Use the `question` tool to ask for approval for each planned file. You may use one `question` call with multiple `questions` entries (one per file).

Each file approval question should provide these options:
- `Approve` (Recommended) - Proceed with this file as proposed
- `Adjust` - User wants to modify this file plan first
- `Skip` - Do not create/update this file

Only create/update files that are explicitly approved.

After approvals are returned, create/update approved files immediately in the same command run.
Do not stop and wait for another user prompt after approvals unless every file is `Adjust` or `Skip`.

After file-level approvals are complete, run the final global check below, then execute approved file writes in this same flow:

Before creating, use the `question` tool:
```json
{
  "questions": [{
    "question": "Before I create the context(s): Is there anything I'm missing? (e.g., additional constraints, dependencies, edge cases for Next Actions)",
    "header": "Final Check",
    "options": [
      {"label": "No, proceed", "description": "Create the context files as proposed"},
      {"label": "Add details", "description": "Let me add more information"}
    ]
  }]
}
```

### 7. Generate Context File(s)

For each context, create a file following this structure:

```markdown
---
context_id: PRJ_NNN
title: Human-Readable Title
project: project-name
feature: feature-name
created: "YYYY-MM-DD"
depends_on:
  contexts:
    - id: ABC_001
      why: Reuses validated upstream contract and acceptance behavior.
---

# PRJ_NNN: Title

## Desired Outcome

<One paragraph describing what success looks like when this context is complete>

## Reference

<!-- Optional: Only include if there are diagrams, ASCII art, or links. Remove entire section if empty. -->

## Next Actions

| Description | Test |
|-------------|------|
| Implement `TypeName` for <purpose> | `type_name_implemented` |
| Create `ServiceName` to handle <responsibility> | `service_created` |
| User action results in expected outcome | `action_outcome` |
| Error condition is handled gracefully | `error_handled` |
```

If there are no blocking prerequisites, omit `depends_on` entirely.

**IMPORTANT FORMAT RULES:**
- Use `## Desired Outcome` (NOT `## Summary`)
- Use `## Next Actions` table format (NOT `## Goals` bullet list)
- Use optional `depends_on` frontmatter only for blocking prerequisites
- Use `depends_on.contexts` only (context-to-context dependencies)
- Each dependency must use `id` and `why` (max 140 chars)
- Keep dependencies direct; do not include transitive prerequisites
- Do NOT include `## Lessons Learned` section
- Do NOT include `## Validation` section
- Do NOT include `## E2E Test Scenarios` section (use Next Actions table instead)
- Test column: snake_case without `test_` prefix (prefix added automatically in test files)
- Description column: Start with verbs (Implement, Create, Add, Configure, Require)

### 8. Save Location

- Pattern: `.nexus/context/<project>/<feature>/PRJ_NNN-brief-description.md`
- Example: `.nexus/context/nexus-cli/marketplace/PRJ_NNN-marketplace-search.md`
- If new project, create `.nexus/context/<project>/` first
- If new feature, create `.nexus/context/<project>/<feature>/` first

### 9. Create index.md Files If Needed

- If this is the first context in a new project directory, create `.nexus/context/<project>/index.md` following the project index requirements in `.nexus/ai_harness/skills/context-driven-development/SKILL.md`.
- If this is the first context in a new feature directory, create `.nexus/context/<project>/<feature>/index.md` with sections: Scope, Context Files, Interfaces, Dependencies, Troubleshooting.

### 10. Read Applicable Rules

After creating the context file(s), read any relevant skill/rule guidance for implementation. Common source:
- `.nexus/ai_harness/skills/context-driven-development/SKILL.md` - Context file standards
- Any language-specific rules (e.g., `rs.md` for Rust)
- Any tool-specific rules (e.g., `justfiles.md`)
