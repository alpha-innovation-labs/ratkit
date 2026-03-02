---
name: nextjs
description: This skill should be used every time you work on Next.js projects or Next.js app code changes in this repository.
compatibility: opencode
---

# Next.js Project Structure Rules

This file defines the preferred Next.js project organization for this repo.

## Core Principles

1. Organize by feature and domain, not by file type.
2. Keep app-level orchestration in `app/` only.
3. Treat `features/` as the primary unit of change and ownership.
4. Share only through `shared/` or `entities/`, never by reaching into another feature.
5. Keep feature boundaries explicit: local UI, logic, and API live together.
6. All UI styling must use Tailwind CSS v4 (no custom CSS except in `styles/`).
7. Avoid top-level type folders like `components/`, `hooks/`, or `utils/`.
8. **Always use `next/link` for internal navigation** - Never use `<a href="...">` for routing to internal pages. Use Next.js `<Link>` component for client-side navigation and better performance.

## Server Logging (MANDATORY)

1. All server logs must be written to a project-local `./logs/` directory.
2. The `./logs/` directory must be gitignored (add `logs/` to the root `.gitignore`).
3. Use `pino` as the approved server logging library.

## E2E Testing (MANDATORY)

1. Browser/runtime E2E testing must always use the `agent-browser` skill.
2. The `agent-browser` requirement is strict; do not substitute ad-hoc/manual browser testing.
3. If a server is already running, do not kill it just to rerun tests.
4. When validating behavior on a running server, inspect logs from `./logs/`.

## Tailwind CSS v4 (MANDATORY)

**Tailwind CSS v4 is the ONLY permitted CSS framework.** No exceptions.

### Strict Rules

1. **v4 ONLY** - Must use `@tailwindcss/vite` plugin, NOT `@tailwindcss/postcss` or older versions.
2. **No custom CSS** - Never write custom CSS classes, `styles.css`, or CSS modules. Use Tailwind utility classes only.
3. **No CSS-in-JS** - No styled-components, Emotion, or CSS modules.
4. **No Tailwind v3 patterns** - Do NOT use `@apply`, `@layer`, or `tailwind.config.js`. v4 uses CSS-first configuration via `@theme` block.
5. **Theme customization via CSS** - Use CSS custom properties in your main CSS file:
   ```css
   @import "tailwindcss";
   
   @theme {
     --color-primary: oklch(70% 0.2 250);
     --font-sans: "Inter", system-ui, sans-serif;
   }
   ```
6. **Arbitrary values** - Use square bracket notation for one-off values: `w-[300px]`, `grid-cols-[1fr_2fr]`.
7. **No !important** - Never use `!important` with Tailwind classes.
8. **No inline styles** - Never use `style={{}}` prop for styling; use Tailwind classes exclusively.
9. **Component patterns** - Compose complex styles with multiple utility classes, not custom CSS.
10. **Shared design tokens** - Put theme values in `src/styles/globals.css` under `@theme`, import where needed.
11. **clsx/tailwind-merge** - Use `clsx` and `tailwind-merge` for conditional classes; never concatenate strings manually.
12. **Dark mode** - Use `dark:` variant with `class` strategy (v4 default).

### Anti-Patterns (NEVER DO THESE)

- Writing any `.css` file with custom class definitions
- Using `styled-components`, `emotion`, or similar
- Using Tailwind v3 config files (`tailwind.config.ts`)
- Using `@apply` directive to extract classes
- Using inline `style` props
- Mixing Tailwind with other CSS solutions

## Top-Level Layout (Monorepo with pnpm)

```
apps/
└── web/                  # Next.js app (Vercel project root)
    └── src/
        ├── app/          # app shell, providers, routing, bootstrapping
        ├── features/     # product capabilities (primary organization)
        ├── entities/     # domain models + cross-feature business concepts
        ├── shared/       # reusable UI + utilities with no feature ownership
        ├── config/       # environment + runtime configuration
        ├── assets/       # static assets
        ├── styles/       # global styles + tokens
        ├── testing/      # test utilities + fixtures
        └── app/          # Next.js App Router entry
packages/
├── db/                   # Drizzle schema + migrations (Neon/Postgres)
│   └── src/
│       ├── schema/
│       ├── migrations/
│       ├── client.ts
│       └── index.ts
└── server/               # server logic used by Next.js API routes
    └── src/
        ├── routes/       # framework-agnostic handlers
        ├── services/     # business services
        └── index.ts
pnpm-workspace.yaml
```

## Folder Semantics

- `app/`: application glue and composition only.
  - `providers/`, `routes/`, `layout/`, `error-boundary/`, `app.tsx`
- `features/`: each feature is a self-contained folder.
  - `features/auth/`, `features/billing/`, `features/settings/`
- `entities/`: domain models shared across features.
  - `entities/user/`, `entities/organization/`
- `shared/`: cross-cutting UI primitives and helpers with no feature ownership.
  - `shared/ui/`, `shared/hooks/`, `shared/lib/`, `shared/types/`
- `config/`: config objects, env parsing, feature flags.
- `assets/`: images, icons, fonts.
- `styles/`: global CSS, design tokens, theme setup.
- `testing/`: test setup, mocks, fixtures, and helpers.
- `packages/db`: Drizzle schema, migrations, and Neon/Postgres client.
- `packages/server`: framework-agnostic server logic used by Next.js API routes.

## Feature Structure (Bulletproof)

Each feature owns its UI, logic, and API integration. Add only what the feature needs.

```
src/features/
└── billing/
    ├── api/            # feature API calls
    ├── components/     # feature-scoped UI
    ├── hooks/          # feature-scoped hooks
    ├── routes/         # feature routes
    ├── types/          # feature types
    ├── utils/          # feature helpers
    └── index.ts
```

## Shared Structure

Shared code must be feature-agnostic.

```
src/shared/
├── ui/                 # design-system primitives
├── hooks/              # generic hooks
├── lib/                # tiny helpers + wrappers
├── types/              # global types
└── config/             # shared config defaults
```

## File Naming

1. Use `kebab-case` for folders and files.
2. Filename matches the primary component or concern.
3. One logical concern per file.

## Import Rules

1. `features/` can import from `shared/`, `entities/`, and `config/` only.
2. `shared/` and `entities/` never import from `features/`.
3. Cross-feature imports are forbidden; go through `shared/` or `entities/`.

## Example Structure

```
apps/
└── web/
    └── src/
        ├── app/
        │   ├── app.tsx
        │   ├── providers/
        │   ├── routes/
        │   └── layout/
        ├── features/
        │   ├── auth/
        │   │   ├── api/
        │   │   ├── components/
        │   │   ├── routes/
        │   │   └── index.ts
        │   └── billing/
        │       ├── api/
        │       ├── components/
        │       ├── hooks/
        │       └── index.ts
        ├── entities/
        │   └── user/
        │       ├── types/
        │       └── index.ts
        ├── shared/
        │   ├── ui/
        │   ├── hooks/
        │   ├── lib/
        │   └── types/
        ├── config/
        ├── assets/
        ├── styles/
        └── testing/
packages/
├── db/
│   └── src/
│       ├── schema/
│       ├── migrations/
│       ├── client.ts
│       └── index.ts
└── server/
    └── src/
        ├── routes/
        ├── services/
        └── index.ts
```
