# Fumadocs Default Patch Pack

## Usage Order (Required)

1. Create docs app using the exact bootstrap command shown below.
2. Start the generated app with `bun run dev` and confirm it runs before any edits.
3. Apply only the approved patch pack below.
4. Only then fill docs pages with MDX content.

Do not redesign framework internals first. Start from default Fumadocs, then patch behavior.

## Bootstrap Command (Exact)

Run exactly:

```bash
CI=1 bun create fumadocs-app@latest docs --template "+next+fuma-docs-mdx" --pm bun --install --no-git --linter eslint --src
```

Then run:

```bash
cd docs
bun run dev
```

Gate condition:

- If `bun run dev` does not work on the generated default app, stop and fix bootstrap/runtime issues first.
- Do not apply custom patches until the default generated app is running.

## Allowed Modification Scope

Only apply the approved patches in this file. No additional framework, layout, or design modifications are allowed at setup stage.

## Patch 1: Use black preset theme

File: `docs/src/app/global.css`

```css
@import 'tailwindcss';
@import 'fumadocs-ui/css/black.css';
@import 'fumadocs-ui/css/preset.css';
```

## Patch 2: TOC style on docs pages

File: `docs/src/app/docs/[[...slug]]/page.tsx`

```tsx
<DocsPage
  toc={page.data.toc}
  full={page.data.full}
  tableOfContent={{
    style: 'clerk',
  }}
>
```

This enables the nested right-side "On this page" style.

## Patch 3: Remove page action row under title

File: `docs/src/app/docs/[[...slug]]/page.tsx`

- Remove `LLMCopyButton` import.
- Remove `ViewOptions` import.
- Remove the `<div>` action row under title and description.

Docs header should render title and description only.

## Patch 4: Use template hero as home page

File: `docs/src/app/(home)/page.tsx`

```tsx
import { HeroTemplate } from '@/components/hero-template';

export default function HomePage() {
  return <HeroTemplate />;
}
```

## Patch 5: Add hero template with blank defaults

File: `docs/src/components/hero-template.tsx`

Required template data:

```ts
const HOME_TEMPLATE = {
  headline: '',
  description: '',
  installCommand: '',
  primaryCtaLabel: '',
  primaryCtaHref: '/docs',
  secondaryCtaLabel: '',
  secondaryCtaHref: '#',
  examples: [
    { id: 'example-a', label: '', code: '' },
    { id: 'example-b', label: '', code: '' },
    { id: 'example-c', label: '', code: '' },
  ],
};
```

Required behavior:

- Hero-first responsive two-column layout
- Background grid + radial overlay
- Command copy interaction with short success state
- Tabbed code preview panel
- Blank placeholder defaults in all template fields

## After Patches: Content-only workflow

After patches are applied, docs contribution should focus on MDX content only:

- Add or edit pages under `docs/content/docs/**`
- Keep heading hierarchy (`##`, `###`) for TOC quality
- Keep navigation ordering in `meta.json`

## Verification

- `just docs` serves from `docs/`
- Bootstrap command matches exactly and generated app runs with `bun run dev` before patches
- TOC appears with clerk style on docs pages
- No copy/open action buttons under title
- Home route renders hero template placeholders
- MDX edits reflect without extra UI rewrites
