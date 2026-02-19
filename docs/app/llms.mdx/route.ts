import { getLLMText, source } from "@/lib/source";

const isGitHubPagesMode = process.env.GH_PAGES === "1";

export const revalidate = false;

export async function GET(request: Request) {
	if (isGitHubPagesMode) {
		return new Response("Not available in GitHub Pages export mode.", {
			status: 404,
			headers: {
				"Content-Type": "text/plain",
			},
		});
	}

	const url = new URL(request.url);
	const rawPath = url.searchParams.get("path") ?? "";
	const slug = rawPath
		.split("/")
		.map((segment) => segment.trim())
		.filter(Boolean);

	const page = source.getPage(slug);
	if (!page) {
		return new Response("Page not found.", {
			status: 404,
			headers: {
				"Content-Type": "text/plain",
			},
		});
	}

	return new Response(await getLLMText(page), {
		headers: {
			"Content-Type": "text/markdown",
		},
	});
}
