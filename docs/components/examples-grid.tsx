"use client";

import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { ArrowRight } from "lucide-react";
import Link from "next/link";
import type { Example } from "@/lib/examples";

interface ExamplesGridProps {
	examples: Example[];
}

export function ExamplesGrid({ examples }: ExamplesGridProps) {
	return (
		<div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
			{examples.map((example) => (
				<Link key={example.id} href={`/examples/${example.id}`}>
					<Card className="group p-6 hover:bg-muted/50 transition-colors cursor-pointer h-full">
						<div className="flex flex-col h-full">
							<div className="flex items-start justify-between mb-4">
								<h3 className="font-semibold text-lg group-hover:text-primary transition-colors">
									{example.title}
	003c/h3>
								<ArrowRight className="w-5 h-5 text-muted-foreground group-hover:text-primary group-hover:translate-x-1 transition-all" />
							</div>
							<p className="text-muted-foreground text-sm mb-4 flex-grow">
								{example.description}
							</p>
							<div className="flex flex-wrap gap-2">
								{example.tags.map((tag) => (
									<Badge key={tag} variant="secondary" className="text-xs">
										{tag}
									</Badge>
								))}
							</div>
						</div>
					</Card>
				</Link>
			))}
		</div>
	);
}
