"use client";

import { Clock } from "lucide-react";

interface ComingSoonProps {
	title: string;
	description?: string;
}

export function ComingSoon({ title, description }: ComingSoonProps) {
	return (
		<div className="flex flex-col items-center justify-center min-h-[400px] text-center px-4">
			<div className="flex items-center gap-3 mb-6">
				<Clock className="w-8 h-8 text-muted-foreground" />
			</div>
			<h1 className="text-3xl font-bold text-foreground mb-4">{title}</h1>
			<p className="text-lg text-muted-foreground max-w-md">
				{description ||
					"We're working hard to bring you this feature. Stay tuned for updates!"}
			</p>
		</div>
	);
}
