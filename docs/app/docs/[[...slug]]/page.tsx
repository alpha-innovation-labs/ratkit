import { createRelativeLink } from "fumadocs-ui/mdx";
import { DocsBody, DocsPage } from "fumadocs-ui/page";
import type { Metadata } from "next";
import { notFound } from "next/navigation";
import { LLMCopyButton, ViewOptions } from "@/components/page-actions";
import { getPageImage, source } from "@/lib/source";
import { getMDXComponents } from "@/mdx-components";

export default async function Page(props: PageProps<"/docs/[[...slug]]">) {
	const params = await props.params;
	const page = source.getPage(params.slug);
	if (!page) notFound();

	const MDX = page.data.body;

	return (
		<div className="mt-[-80px]">
			<DocsPage
				toc={page.data.toc}
				full={page.data.full}
				tableOfContent={{
					style: "clerk",
				}}
			>
				<h1 className="text-[1.75em] font-semibold">{page.data.title}</h1>
				<div className="flex flex-row gap-2 items-center pb-3">
					<LLMCopyButton markdownUrl={`${page.url}.mdx`} />
					<ViewOptions
						markdownUrl={`${page.url}.mdx`}
						githubUrl={`https://github.com/Alpha-Innovation-Labs/ratkit/blob/main/docs/content/docs/${page.path}`}
					/>
				</div>
				{page.data.description && (
					<p className="text-lg text-fd-muted-foreground mb-2 border-b pb-6">
						{page.data.description}
					</p>
				)}
				<DocsBody>
					<MDX
						components={getMDXComponents({
							// this allows you to link to other pages with relative file paths
							a: createRelativeLink(source, page),
						})}
					/>
				</DocsBody>
			</DocsPage>
		</div>
	);
}

export async function generateStaticParams() {
	return source.generateParams();
}

export async function generateMetadata(
	props: PageProps<"/docs/[[...slug]]">,
): Promise<Metadata> {
	const params = await props.params;
	const page = source.getPage(params.slug);
	if (!page) notFound();

	return {
		title: page.data.title,
		description: page.data.description,
		openGraph: {
			images: getPageImage(page).url,
		},
	};
}
