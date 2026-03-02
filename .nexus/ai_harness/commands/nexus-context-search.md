---
description: Search existing context files for relevant outcomes/actions
---

# Command: Search Contexts

You are searching through existing context files to answer the user's query. Use `.nexus/ai_harness/skills/context-driven-development/SKILL.md` as the format source of truth, and search the "Desired Outcome" and "Next Actions" sections of `.nexus/context/` files to find relevant information.

Read the CDD skill first before scanning contexts.

## Purpose

Help users find existing contexts that match their query, understand what's already been specified, and identify related work across the project.

## Workflow

### 1. Understand the Query

- Listen to the user's question or search terms
- Identify key concepts, features, or goals they're asking about

### 2. Scan All Context Files

**Read all context files from `.nexus/context/`** (excluding `_legacy/` and `_reference/` folders):

For each context file, extract:
- **context_id** from frontmatter
- **title** from frontmatter
- **project** from frontmatter
- **Desired Outcome** section content
- **Next Actions** table content

### 3. Analyze for Relevance

Score each context by relevance to the user's query:
- **High relevance**: Directly addresses the query topic
- **Medium relevance**: Related or adjacent topic
- **Low relevance**: Mentioned but not central

Consider:
- Keywords in the Desired Outcome
- Keywords in Next Actions descriptions
- Project area (nexus-tui, nexus-server, etc.)

### 4. Present Results

Format your response as:

```
## Search Results for: "<user query>"

Found N relevant context(s):

### 1. PRJ_NNN: Title (High Relevance)
**Project:** project-name
**File:** `.nexus/context/project-name/PRJ_NNN-description.md`

**Desired Outcome:**
<One paragraph summary from the context file>

**Key Next Actions:**
| Description | Test |
|-------------|------|
| <Most relevant action 1> | `<test_name>` |
| <Most relevant action 2> | `<test_name>` |

---

### 2. PRJ_NNN: Title (Medium Relevance)
...
```

**If no relevant contexts found:**
```
No contexts found matching "<query>".

**Suggestions:**
- Try different keywords
- Check `.nexus/context/_reference/` for background information
- Create a new context with `/nexus-context-create`
```

### 5. Provide Insights

After listing results, add a brief analysis:

```
## Summary

**What I found:**
- N contexts related to [topic]
- Primary work appears to be in [project-name]
- [Any patterns or gaps noticed]

**Recommendations:**
- [Suggest next steps based on what exists]
```

## Notes

- Always exclude `_legacy/` and `_reference/` folders from main results
- Mention relevant reference materials in `_reference/` separately when useful
- Group results by project when multiple matches belong to the same project
- Highlight potential duplicates or overlapping contexts
