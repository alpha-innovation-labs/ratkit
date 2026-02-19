---
context_id: ZDO_001
title: Docs Home Foundation
project: zenbt-docs
created: "2026-02-18"
---

# ZDO_001: Docs Home Foundation

## Desired Outcome

Running `just docs` renders a docs site built from default Fumadocs setup, then patched with the approved patch pack to produce the target hero-first landing behavior, after which future updates are done by editing MDX content only.

## Reference

| Reference File | Coverage |
|----------------|----------|
| `_reference/fumadocs-default-patch-pack.md` | Default Fumadocs setup workflow, exact patches, and MDX-only follow-up workflow |

| Visual Contract | Requirement |
|-----------------|-------------|
| Hero intent | Primary product message and clear docs entry action are visible without scrolling |
| Theming | Color tokens are centralized and applied consistently across landing sections |
| Responsive behavior | Landing layout is usable on desktop and mobile widths |

## Next Actions

| Description | Test |
|-------------|------|
| Require docs bootstrap using `CI=1 bun create fumadocs-app@latest docs --template "+next+fuma-docs-mdx" --pm bun --install --no-git --linter eslint --src` | `docs_bootstrap_uses_required_command` |
| Require `cd docs && bun run dev` to succeed before applying any custom patch | `default_docs_runs_before_patches` |
| Require docs app initialization from default Fumadocs setup under `docs/` | `docs_starts_from_default_fumadocs` |
| Require setup-stage changes to be limited to the approved patch pack only | `only_approved_patches_are_applied` |
| Apply the approved patch pack before any custom docs content authoring | `patch_pack_applied_before_content` |
| Require documentation application files to live under the project `docs/` directory | `docs_application_lives_in_docs_folder` |
| Require `just docs` to start the local documentation preview from the `docs/` application | `just_docs_serves_docs_application` |
| Require browser validation using `agent-browser` against `http://localhost:3000/docs` after `just docs` is running | `agent_browser_validates_docs_route` |
| Require post-setup workflow to focus on MDX content updates without additional framework rewrites | `post_setup_updates_are_mdx_only` |
| Create a docs home route that is accessible through `just docs` and loads as the default landing page | `docs_home_route_loads` |
| Implement landing page section structure that follows the defined hero-first documentation composition | `landing_structure_matches_contract` |
| Define and apply a distinct brand color palette across key landing UI elements | `landing_palette_is_distinct` |
| Add responsive behavior so the landing page remains readable and navigable on mobile and desktop | `landing_is_responsive` |
| Verify local docs preview displays the updated landing experience end to end | `landing_preview_works_end_to_end` |
