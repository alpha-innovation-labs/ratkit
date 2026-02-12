import { createMDX } from "fumadocs-mdx/next";

const withMDX = createMDX();

/** @type {import('next').NextConfig} */
const config = {
	reactStrictMode: true,
	async rewrites() {
		return [
			{
				source: "/docs/:path*.mdx",
				destination: "/llms.mdx/:path*",
			},
			{
				source: "/api/data/:match*",
				destination: "https://ratkit.rs/_vercel/insights/:match*",
			},
		];
	},
};

export default withMDX(config);
