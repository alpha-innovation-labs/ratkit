import { notFound } from "next/navigation";
import { examples } from "@/lib/examples";

interface ExamplePageProps {
	params: Promise<{
		slug?: string[];
	}>;
}

export default async function ExamplePage({ params }: ExamplePageProps) {
	const { slug } = await params;
	const exampleId = slug?.[0];

	if (!exampleId) {
		notFound();
	}

	const example = examples.find((e) => e.id === exampleId);

	if (!example) {
		notFound();
	}

	return (
		<div className="container py-12">
			<div className="mb-8">
				<h1 className="text-3xl font-bold mb-2">{example.title}</h1>
				<p className="text-muted-foreground">{example.description}</p>
			</div>
			<div className="prose max-w-none">
				<p>Example implementation coming soon. Check back for the complete code walkthrough.</p>
			</div>
		</div>
	);
}

export async function generateStaticParams() {
	return examples.map((example) => ({
		slug: [example.id],
	}));
}

export async function generateMetadata({ params }: ExamplePageProps) {
	const { slug } = await params;
	const exampleId = slug?.[0];
	const example = examples.find((e) => e.id === exampleId);

	return {
		title: example?.title ?? "Example",
		description: example?.description ?? "ratkit example",
	};
}
