---
description: Generate context recommendations from code paths
---

# Generate Contexts From Code

Read `.nexus/ai_harness/skills/context-driven-development/SKILL.md` first and apply it as the only CDD source of truth.

## Description

Given a file or folder, analyze the code and recommend context files to create. Spawn 3 subagents to independently review the functionality and propose context specs that follow `.nexus/ai_harness/skills/context-driven-development/SKILL.md`.

## Usage

```bash
/nexus-context-from-code <path>
```

If no path is provided, ask the user for a file or folder.

## What This Command Does

1. **Validates the input path** and determines whether it is a file or folder
2. **Scans existing contexts** in `.nexus/context/` to avoid duplicates and align with project prefixes
3. **Analyzes the code** in the provided scope to identify user-visible behaviors
4. **Spawns 3 subagents** to independently review functionality and propose context specs
5. **Aggregates recommendations** into a single, deduplicated list
6. **Presents a proposed file plan and asks approval per file**
7. **Executes approved create/update actions in the same run**

## Workflow

### Phase 1: Scope Confirmation

1. **Confirm scope**:
   - If the path is a file, use it as the primary focus
   - If the path is a folder, scan all relevant code files within it
2. **Exclude** irrelevant folders (at minimum):
   - `.git/`, `target/`, `node_modules/`, `.nexus/`, `.context/`, `_reference/`, `_legacy/`

### Phase 2: Context Discovery

1. **Scan existing context files** in `.nexus/context/`:
   - Read frontmatter: `context_id`, `project`, `title`
   - Read `## Desired Outcome` and `## Next Actions`
2. **Identify overlaps**:
   - If an existing context already covers the desired outcome, recommend updating instead of creating a new one

### Phase 3: Code Understanding

1. **Inventory relevant files** in scope:
   - Entry points (main files, command handlers, API routes)
   - Domain logic and user-facing behaviors
   - Error handling and edge cases
2. **Read only what is needed** to understand user-visible behavior

### Phase 4: Spawn Subagents

Spawn 3 Task subagents (use the `explore` agent type) with the same prompt:

```
Analyze the provided code path and identify user-visible functionality. Propose context files to create that follow .nexus/ai_harness/skills/context-driven-development/SKILL.md. Do not write code or modify files. Provide:

1) Proposed context list with 1 outcome per context
2) For each context: project, short title, brief desired outcome, and 3-7 Next Actions (E2E testable)
3) Note any overlaps with existing contexts in .nexus/context/
```

### Phase 5: Aggregate Recommendations

1. **Compare the 3 outputs** and consolidate into a single list
2. **Resolve duplicates** by merging overlapping outcomes
3. **Flag disagreements** and ask the user to choose if needed

### Phase 6: Present Results

Use this format:

```
## Recommended Contexts for <path>

Found N recommended context(s):

### 1. PRJ_NNN: Title
**Project:** project-name
**Proposed File:** `.nexus/context/project-name/PRJ_NNN-brief-description.md`

**Desired Outcome:**
<One paragraph outcome>

**Next Actions:**
| Description | Test |
|-------------|------|
| <Action 1> | `<test_name>` |
| <Action 2> | `<test_name>` |

**Notes:**
- Overlaps: <existing context if any>
- Source: <key files referenced>
```

If any item should update an existing context instead of creating a new one, clearly label it as **Update** and reference the existing context file.

### Phase 7: File-Level Approval via `question`

After presenting the list, call the `question` tool and include one question per recommended file.

Each file question should include:
- target path
- create vs update intent
- short summary of what will be added or changed

Use options:
- `Approve` (Recommended) - Include this file in follow-up creation/update
- `Adjust` - Revise this file recommendation first
- `Skip` - Exclude this file

Only approved files should be executed.

Then include one final next-step question in the same `question` call.

Replace the freeform prompt with these options:

```
1. Create approved items now in this run (Recommended)
2. Refine recommendations first
3. Cancel
```

If the user chooses to create approved items now, execute those file creates/updates directly in this command flow without requiring a separate slash command.

## Rules to Enforce

- Follow `.nexus/ai_harness/skills/context-driven-development/SKILL.md` for format and content
- One outcome per context
- No implementation details or code
- Next Actions must be E2E testable and user-visible
- Use table format for Next Actions (Description + Test)

## Example Output

```
## Recommended Contexts for crates/nexus-cli/src

Found 2 recommended context(s):

### 1. PRJ_NNN: Context Discovery
**Project:** nexus-cli
**Proposed File:** `.nexus/context/nexus-cli/PRJ_NNN-context-discovery.md`

**Desired Outcome:**
Users can discover existing contexts by keyword or ID, understand their scope, and select one to proceed without ambiguity.

**Next Actions:**
| Description | Test |
|-------------|------|
| User can list contexts filtered by keyword | `context_list_filtered_by_keyword` |
| Selecting a context shows its desired outcome | `context_selection_shows_outcome` |
| No results shows a clear message with suggestions | `context_search_no_results_message` |

**Notes:**
- Overlaps: None
- Source: crates/nexus-cli/src/context/search.rs
```
