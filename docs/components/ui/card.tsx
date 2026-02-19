import type * as React from "react";

import { cn } from "@/lib/utils";

function Card({ className, ...props }: React.ComponentProps<"div">) {
	return (
		<div
			data-slot="card"
			className={cn("bg-card text-card-foreground rounded-xl border shadow-sm", className)}
			{...props}
		/>
	);
}

export { Card };
