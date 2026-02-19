# Docs Implementation Blueprint

## Goal

Provide a complete technical blueprint for recreating the docs application experience with:

- A hero-first landing page
- Internal docs pages with left navigation and right-side TOC
- Search-ready, route-driven docs content loading

This reference is implementation-focused and intended to be used alongside `ZDO_001` and `ZDO_002`.

## Default Framework Requirement

Use Fumadocs as the default docs framework, not a generic replacement.

Required baseline:

- `fumadocs-ui` for layouts/pages/search UI
- `fumadocs-core` for source loading/search hooks
- `fumadocs-mdx` for docs collection and schema config

See `_reference/fumadocs-default-stack.md` for preserved custom behaviors.

## Suggested Docs App Shape

```text
docs/
  app/
    (home)/page.tsx
    docs/layout.tsx
    docs/[[...slug]]/page.tsx
    layout.tsx
    global.css
  components/
    hero.tsx
    navbar.tsx
    page-actions.tsx
    search.tsx
    theme-toggle.tsx
  content/
    docs/
      meta.json
      (get-started)/
      core/
      adapters/
      primitives/
  lib/
    layout.shared.tsx
    source.ts
  source.config.ts
  package.json
```

## Routing Contracts

| Route | Purpose | Required Behavior |
|------|---------|-------------------|
| `/` | Landing page | Hero-first layout with immediate CTA to `/docs` |
| `/docs` | Docs index | Uses content tree and section metadata |
| `/docs/[...slug]` | Internal docs page | Renders MDX/Markdown body with generated TOC |

## Layout Contracts

| Layer | Contract |
|------|----------|
| Root app layout | Global CSS, typography, theme provider, search provider, top navbar |
| Home layout | Minimal chrome around landing sections |
| Docs layout | Left sidebar tree from docs source + right TOC rail |

### Fumadocs Layout Mapping

| Contract | Fumadocs Primitive |
|----------|--------------------|
| Home shell | `HomeLayout` |
| Docs shell | `DocsLayout` |
| Docs page wrapper | `DocsPage` + `DocsBody` |
| Root integration | `RootProvider` |

## Landing Page Composition Contract

| Block | Required Content |
|------|------------------|
| Hero headline | One-line product value proposition |
| Supporting text | Short summary of the framework and target user |
| Command snippet | Copyable quickstart command |
| Primary CTA | Link to docs index |
| Secondary CTA | Link to repository |
| Visual panel | Code/sample panel with tabbed examples |

## Landing Interaction Details

- Copy button shows success state for 2 seconds.
- Hero panel supports mobile-first layout (`single column`) and desktop split layout (`two columns`).
- Background has subtle grid and gradient overlay for depth.
- CTA buttons remain visible at common viewport sizes without scrolling.

## Preserved Custom Features (Implementation Mapping)

| Feature | How to implement here |
|---------|-----------------------|
| Hero command copy feedback | Use local client state and reset success icon after ~2 seconds |
| Tabbed code preview panel | Use tab triggers + dynamic code block switched by selected category |
| Sticky navbar active state | Match active nav item by current path prefix |
| Mobile drawer navigation | Use sheet/drawer menu with close-on-click nav items |
| Theme persistence | Read cookie server-side and update cookie on toggle |
| Per-page action row | Render markdown copy + source-view actions under docs title |
| TOC styling | Apply explicit TOC style config in docs page component |
| Social card route | Add OG route that resolves page metadata from docs source |

## Internal Docs Page Contract

| Element | Required Behavior |
|--------|-------------------|
| Page title | Render from frontmatter/content metadata |
| Description | Optional subtitle under title |
| Page actions | Copy markdown link and open source link |
| Body | Render MDX with component mapping |
| TOC | Right-side heading navigation, desktop-visible |

## TOC Contract

- Source headings `h2-h4`.
- Generate anchor IDs deterministically.
- Clicking TOC item scrolls to heading and updates URL hash.
- Active section highlight updates while scrolling.
- Mobile fallback is accessible (drawer/accordion or top inline section).

## Navigation Tree Contract

- `content/docs/meta.json` defines top-level group order.
- Nested `meta.json` files define section order.
- Sidebar reflects file-system-backed docs tree.
- Navigation state is stable between route transitions.

## Theming And Design Tokens Contract

Define CSS variables in `app/global.css` and bind through theme system:

- `--background`, `--foreground`
- `--primary`, `--accent`, `--muted`
- `--border`, `--ring`, `--sidebar-*`
- `--radius`

Rules:

- Centralize all brand colors in root and dark blocks.
- No hardcoded page-level color hex values except deliberate accent exceptions.
- Ensure WCAG-friendly contrast for text and interactive controls.

## Shared Layout Options Contract

`lib/layout.shared.tsx` should hold reusable options:

- Search enabled
- Nav links configuration
- Theme switch behavior
- Any project-wide UI toggles

Both home and docs layouts consume this shared base object.

## Content Source Contract

`lib/source.ts` should expose:

- Loader creation from generated docs source
- `getPage(...)` behavior through source object
- Static params generator for dynamic docs routes
- Optional helpers for social image or LLM export

Use `loader(...)` from `fumadocs-core/source` and ensure `baseUrl` is `/docs`.

`source.config.ts` should define:

- Frontmatter schema
- Meta schema
- Markdown post-processing options

Use `defineDocs(...)` and `defineConfig(...)` from `fumadocs-mdx/config`.

## Component-Level Build Checklist

- `components/hero.tsx`: headline, copy action, CTA buttons, code panel.
- `components/navbar.tsx`: sticky top bar, docs link, repo link, mobile menu, theme toggle.
- `components/search.tsx`: keyboard-friendly search dialog.
- `components/page-actions.tsx`: markdown copy and source view links.

## Verification Checklist

- `just docs` opens and serves home + docs routes.
- Fumadocs layouts and page primitives are used for route rendering.
- Sidebar tree appears and navigates nested sections.
- Right TOC appears for long pages with heading anchors.
- Landing page remains coherent on mobile and desktop.
- Theme tokens apply consistently across home and docs pages.
