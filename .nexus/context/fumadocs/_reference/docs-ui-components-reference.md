# Docs UI Components Reference

## Goal

Capture concrete UI component behavior needed to rebuild the docs experience quickly and consistently.

## Fumadocs UI Requirement

These component contracts assume Fumadocs-driven layouts and docs pages.

- Component behavior must integrate cleanly with `HomeLayout`, `DocsLayout`, `DocsPage`, and `DocsBody`.
- Search dialog should use Fumadocs search primitives.

## Navbar Contract

### Structure

- Sticky top header with backdrop blur and border.
- Left area: product mark + title.
- Center/desktop area: top-level nav links.
- Right area: repository link, optional metric badge, theme toggle.
- Mobile: compact icon buttons + slide-out menu.

### Behavior

- Active nav link highlights based on current path prefix.
- Mobile drawer closes on navigation click.
- Theme toggle available in both desktop and mobile variants.
- Optional repository metric badge can be rendered in desktop and compact mobile form.

### Accessibility

- Icon-only buttons must include `sr-only` labels.
- Menu trigger has explicit toggle label.
- Focus states visible on all controls.

## Hero Contract

### Structure

- Two-column layout on desktop, one-column on mobile.
- Left: headline, description, install snippet with copy interaction, CTA row.
- Right: tabbed code/example preview panel.

### Visual Rules

- Subtle background grid.
- Radial gradient overlay for depth.
- Accent color used for CTA emphasis and command prefix.

### Interaction Rules

- Copy action updates icon state for short success interval.
- Example panel supports category tabs.
- Code content switches immediately with selected tab.
- Background grid and gradient overlay adapt to current theme.

## Docs Page Actions Contract

- Include copy-markdown action for current page.
- Include source-view action for current page.
- Keep actions close to title and description region.
- Cache fetched markdown text before repeated copy operations.

## Docs Layout Contract

### Left Navigation

- Tree-based section navigation.
- Stable ordering from content metadata.
- Non-collapsible by default unless explicitly configured.

### Right TOC Rail

- TOC style supports clear active section marker.
- Headings generated from rendered docs content.
- TOC hidden or transformed on narrow screens.

## Global Styling Contract

Define and use tokens for:

- Semantic colors (`primary`, `accent`, `muted`, `background`, `foreground`)
- Border and ring
- Sidebar colors
- Radius scale

Rules:

- Light and dark themes both defined.
- Components consume tokens, not hardcoded values, except deliberate accent spots.
- Typography scale supports compact technical docs reading.

## Content Rendering Contract

- Docs pages render markdown/MDX body.
- Relative links resolve correctly between pages.
- Heading IDs remain stable for TOC and deep links.
- MDX component registry includes custom code-tabs component and icon components.

## Concrete Custom Feature Inventory

| Custom Feature | Implementation Notes |
|---------------|----------------------|
| Interactive hero tabs | Category buttons switch code examples without route changes |
| Install command copy control | Inline copy icon toggles to success icon after copy |
| Path-aware navbar highlighting | Link style changes when path starts with link href |
| Theme toggle with persistence | Toggle updates active theme and writes theme cookie |
| Mobile sheet navigation | Menu button opens sheet; selecting link closes sheet |
| Search dialog shell | Overlay + input + results list with static-index search mode |
| Docs page action row | Copy markdown action + open source action near page heading |
| TOC rail style | Explicit TOC style config to keep right-rail behavior consistent |

## Responsive Breakpoints (Behavioral)

- Mobile: drawer navigation, stacked hero, compact controls.
- Tablet: partial two-column transitions where appropriate.
- Desktop: full docs layout with left sidebar and right TOC.

## Practical Rebuild Order

1. Global layout and theme provider
2. Navbar and routing shell
3. Home hero sections
4. Docs content route + source loader
5. Left nav + right TOC
6. Page actions and search integration
7. Theme polish and responsive tuning

## Done Criteria

- Landing and docs routes both render correctly under `just docs`.
- Docs navigation and TOC are fully functional.
- Theme and color system are distinct and consistent.
- Core interactions (copy, nav, tabbed preview) are working.
