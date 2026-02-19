import { isMarkdownPreferred, rewritePath } from "fumadocs-core/negotiation";
import { type NextRequest, NextResponse } from "next/server";

const { rewrite: rewriteLLM } = rewritePath("/docs/*path", "/llms.mdx/*path");

export function middleware(request: NextRequest) {
	if (isMarkdownPreferred(request)) {
		const result = rewriteLLM(request.nextUrl.pathname);

		if (result) {
			const llmUrl = new URL("/llms.mdx", request.nextUrl);
			llmUrl.searchParams.set("path", result.replace(/^\/llms\.mdx\//, ""));

			return NextResponse.rewrite(llmUrl);
		}
	}

	return NextResponse.next();
}
