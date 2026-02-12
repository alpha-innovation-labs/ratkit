import type { BaseLayoutProps } from "fumadocs-ui/layouts/shared";

/**
 * Shared layout configurations
 *
 * you can customise layouts individually from:
 * Home Layout: app/(home)/layout.tsx
 * Docs Layout: app/docs/layout.tsx
 */

export const TITLE = "ratkit";
export function baseOptions(): BaseLayoutProps {
	return {
		searchToggle: {
			enabled: true,
		},
		nav: {
			enabled: false,
		},
		// see https://fumadocs.dev/docs/ui/navigation/links
		links: [],
	};
}
