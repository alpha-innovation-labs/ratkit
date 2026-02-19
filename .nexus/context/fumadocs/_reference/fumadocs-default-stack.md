# Fumadocs Default Stack And Custom Features

## Baseline Requirement

The docs application is built as a Fumadocs-first stack by default.

Required packages:

- `fumadocs-ui`
- `fumadocs-core`
- `fumadocs-mdx`

Required application root:

- All docs application files live under `docs/`.
- Local serve entrypoint is `just docs`.

## Required Fumadocs Wiring

| Concern | Required Integration |
|--------|----------------------|
| Root provider | Use `RootProvider` from Fumadocs in root app layout |
| Home shell | Use `HomeLayout` for landing page container |
| Docs shell | Use `DocsLayout` with `tree={source.pageTree}` |
| Docs page rendering | Use `DocsPage` + `DocsBody` with per-page `toc` |
| Source loading | Use `loader(...)` from `fumadocs-core/source` |
| MDX config | Use `defineDocs` and `defineConfig` from `fumadocs-mdx/config` |

## Required Route Layout

```text
docs/
  app/
    layout.tsx
    global.css
    (home)/page.tsx
    docs/layout.tsx
    docs/[[...slug]]/page.tsx
```

## Custom Feature Set To Carry Forward

### 1) Hero-First Landing With Interactive Code Panel

Implementation pattern:

- A hero component with two responsive columns.
- Left column has headline, description, command snippet, and CTAs.
- Right column has a tabbed code preview panel.
- Command snippet has copy-to-clipboard interaction with short success feedback.

How to implement here:

- Create `docs/components/hero.tsx`.
- Add local state for tab selection and copy feedback.
- Use a Fumadocs-compatible code block component for dynamic snippets.
- Keep CTA target to `/docs`.

### 2) Sticky Navbar With Path-Aware Active Links

Implementation pattern:

- Sticky header with divider-based structure.
- Active link highlighting based on pathname prefix.
- Desktop links + mobile sheet menu variant.
- Theme toggle available in both desktop and mobile controls.

How to implement here:

- Create `docs/components/navbar.tsx` with path-aware link classes.
- Add mobile drawer menu and close-on-link-click behavior.
- Use shared title constant from `docs/lib/layout.shared.tsx`.

### 3) Search Dialog Using Static Docs Index

Implementation pattern:

- `useDocsSearch` with static search index mode.
- Orama instance initialization for client-side search.
- Standard dialog slots: overlay, content, header, input, list.

How to implement here:

- Create `docs/components/search.tsx`.
- Wire `SearchDialog` into root `RootProvider` search slot.
- Ensure keyboard-open behavior works on docs routes.

### 4) Right-Rail TOC With Stable Heading Navigation

Implementation pattern:

- Per-page TOC passed to `DocsPage` from docs source data.
- TOC style configured at page render level.
- Relative link helper used for cross-page MDX links.

How to implement here:

- In `docs/app/docs/[[...slug]]/page.tsx`, pass `toc={page.data.toc}`.
- Keep right-rail TOC visible on desktop and accessible on mobile.
- Use deterministic heading IDs for stable deep links.

### 5) Page Actions: Copy Markdown And Open Source

Implementation pattern:

- Inline page actions near title block.
- Copy markdown button with cached fetch and success indicator.
- Source-view popover action linking to repository path.

How to implement here:

- Create `docs/components/page-actions.tsx`.
- Keep actions close to title and description in docs page template.
- Reuse in every docs slug page.

### 6) Theme Cookie + Server Theme Context

Implementation pattern:

- Root layout reads theme cookie server-side.
- Lightweight provider exposes server theme to client components.
- Toggle writes cookie when switching themes.

How to implement here:

- Add `docs/components/theme-provider.tsx` and `docs/lib/setThemeCookie.ts`.
- Read cookie in `docs/app/layout.tsx` and pass through provider.
- Keep hydration-safe fallback for first render.

### 7) Source Loader And Icon Plugin

Implementation pattern:

- `loader(...)` with `baseUrl: "/docs"`.
- Docs source from generated `.source` output.
- Optional icon mapping support via plugin.

How to implement here:

- Implement `docs/lib/source.ts` with source loader and helper functions.
- Include optional helper for page OG image route generation.

### 8) OG Image Route For Docs Pages

Implementation pattern:

- Dynamic OpenGraph image route for docs pages.
- Uses page title and description to generate image metadata.

How to implement here:

- Add `docs/app/og/docs/[...slug]/route.tsx`.
- Resolve source page by slug and render OG image response.

### 9) MDX Component Registry

Implementation pattern:

- Central `getMDXComponents` function with default Fumadocs MDX components.
- Add custom components and icon components to registry.

How to implement here:

- Create `docs/mdx-components.tsx`.
- Register custom tabs/code components and any icon set used in docs pages.

### 10) Justfile Docs Command

Implementation pattern:

- `just docs` ensures docs dependencies are installed.
- Runs docs dev server from `docs/`.
- Captures logs to `logs/docs.log`.

How to implement here:

- Define docs recipe in project justfile or imported justfile.
- Recipe should execute install + serve in the `docs/` app directory.

## Minimum Command Contract

| Command | Expected Result |
|--------|------------------|
| `just docs` | Launches local docs app from `docs/` with Fumadocs rendering |
| `just docs-sync` | Regenerates API/reference docs pages from Rust doc comments |
| `just docs-sync-check` | Fails when generated docs are out of date |

## Non-Negotiable Constraints

- Fumadocs remains the default docs framework.
- Docs app remains rooted under `docs/`.
- Landing and internal docs routes are served by `just docs`.
- Right-side TOC and sidebar navigation remain available for docs content pages.
