import { createMDX } from "fumadocs-mdx/next";

const withMDX = createMDX();
const isGitHubPagesMode = process.env.GH_PAGES === "1";
const ghPagesBasePath = process.env.GH_PAGES_BASE_PATH ?? "";

/** @type {import('next').NextConfig} */
const config = {
	reactStrictMode: true,
	output: isGitHubPagesMode ? "export" : undefined,
	basePath: isGitHubPagesMode ? ghPagesBasePath : undefined,
	assetPrefix: isGitHubPagesMode ? ghPagesBasePath : undefined,
	trailingSlash: isGitHubPagesMode,
	...(isGitHubPagesMode
		? {}
		: {
				async rewrites() {
					return [
						{
							source: "/docs/:path*.mdx",
							destination: "/llms.mdx?path=:path*",
						},
						{
							source: "/api/data/:match*",
							destination: "https://ratkit.rs/_vercel/insights/:match*",
						},
					];
				},
		  }),
};

export default withMDX(config);
