export interface Example {
	id: string;
	title: string;
	description: string;
	tags: string[];
}

export const examples: Example[] = [
	{
		id: "dashboard",
		title: "Dashboard Widget",
		description: "A complete dashboard with multiple widgets showing system stats, charts, and interactive controls.",
		tags: ["widgets", "layout", "charts"],
	},
	{
		id: "form-builder",
		title: "Form Builder",
		description: "Dynamic form construction using ratkit primitives with validation and theming.",
		tags: ["primitives", "forms", "theming"],
	},
	{
		id: "chat-interface",
		title: "Chat Interface",
		description: "A real-time chat interface using services and widgets with keyboard navigation.",
		tags: ["services", "widgets", "keyboard"],
	},
	{
		id: "data-table",
		title: "Data Table",
		description: "Advanced data table with sorting, filtering, and pagination using ratkit table widget.",
		tags: ["widgets", "table", "data"],
	},
];
