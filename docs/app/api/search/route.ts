import { createFromSource } from "fumadocs-core/search/server";
import { source } from "@/lib/source";

const isGitHubPagesMode = process.env.GH_PAGES === "1";
const searchRoute = createFromSource(source, {
	// https://docs.orama.com/docs/orama-js/supported-languages
	language: "english",
});

// statically cached
export const revalidate = false;

export const GET = isGitHubPagesMode ? (() => Response.json([])) : searchRoute.staticGET;
