"use client";
import { DynamicCodeBlock } from "fumadocs-ui/components/dynamic-codeblock";
import {
	Tabs,
	TabsContent,
	TabsList,
	TabsTrigger,
} from "fumadocs-ui/components/tabs";
import { Check, Copy, Layout, MousePointer, PanelLeft, SquareArrowOutUpRight, Type } from "lucide-react";
import Link from "next/link";
import { useTheme } from "next-themes";
import { useEffect, useMemo, useState } from "react";
import { useServerTheme } from "@/components/theme-provider";
import { Button } from "@/components/ui/button";

export function Hero() {
	const [copied, setCopied] = useState(false);
	const { resolvedTheme } = useTheme();
	const { serverTheme } = useServerTheme();
	const theme = resolvedTheme || serverTheme;
	const isDark = theme === "dark";

	useEffect(() => {
		if (copied) {
			const timer = setTimeout(() => setCopied(false), 2000);
			return () => clearTimeout(timer);
		}
	}, [copied]);

	const gridColor = isDark ? "rgba(255,255,255,0.08)" : "rgba(0,0,0,0.08)";
	const overlayBackground = isDark
		? "radial-gradient(ellipse 120% 100% at 50% 0%, transparent 0%, rgba(0,0,0,0.3) 60%, rgba(0,0,0,0.7) 100%)"
		: "radial-gradient(ellipse 120% 100% at 50% 0%, transparent 0%, rgba(255,255,255,0.3) 60%, rgba(255,255,255,0.7) 100%)";

	return (
		<div className="hero-component relative flex flex-col items-center justify-center min-h-screen w-full overflow-hidden pt-24 p-0 sm:p-4 md:pt-32 lg:p-8">
			{/* Background Grid with Gradient Overlay */}
			<div className="absolute inset-0 pointer-events-none">
				{/* Grid Pattern */}
				<div
					className="absolute inset-0"
					style={{
						backgroundImage: `
              linear-gradient(to right, ${gridColor} 1px, transparent 1px),
              linear-gradient(to bottom, ${gridColor} 1px, transparent 1px)
            `,
						backgroundSize: "80px 80px",
					}}
				/>

				{/* Gradient Overlay - subtle fade */}
				<div
					className="absolute inset-0"
					style={{
						background: overlayBackground,
					}}
				/>
			</div>

			<div className="relative z-10 grid grid-cols-1 lg:grid-cols-2 gap-6 md:gap-12 max-w-7xl w-full items-center mt-8">
				{/* Left Column */}
				<div className="flex flex-col items-center text-center space-y-4 sm:space-y-6 lg:items-start lg:text-left">

					<h1 className="text-3xl sm:text-5xl xl:text-[3.5rem] font-bold tracking-tight text-black dark:text-white leading-tight">
						TUI Toolkit for <span className="text-[#ef4444]">Rust</span>
					</h1>

					<p className="text-xs sm:text-base text-gray-600 dark:text-gray-400 max-w-sm sm:max-w-xl md:max-w-lg">
						A comprehensive collection of reusable TUI components for ratatui.
						Build beautiful terminal interfaces with ease.
					</p>

					{/* Command Snippet */}
					<div className="w-full mt-4 rounded max-w-xs sm:mt-0 sm:max-w-md bg-white border border-black/10 dark:bg-[#111111] dark:border-[#ef4444]/30 p-2.5 flex items-center gap-2.5 font-mono text-xs">
						<span className="text-[#ef4444] mr-1">$</span>
						<span className="text-black dark:text-white font-semibold">
							<span className="text-[#06b6d4] dark:text-[#06b6d4]">cargo</span> add{" "}
							<span className="text-[#ef4444]">ratkit</span>
						</span>
						<div className="flex-1" />
						{copied ? (
							<Check className="w-3.5 h-3.5 text-black dark:text-white" />
						) : (
							<Copy
								className="w-3.5 h-3.5 text-gray-500 cursor-pointer hover:text-black dark:hover:text-white transition-colors"
								onClick={() => {
									navigator.clipboard.writeText("cargo add ratkit");
									setCopied(true);
								}}
							/>
						)}
					</div>
					<div className="flex flex-wrap gap-3">
						<Button
							asChild
							size="sm"
							className="bg-[#ef4444] dark:bg-[#ef4444] rounded font-semibold text-white text-xs sm:text-sm hover:bg-[#dc2626] dark:hover:bg-[#dc2626] px-2 sm:px-6 border-0"
						>
							<Link href="/docs">GET STARTED</Link>
						</Button>
						<Button
							variant="outline"
							size="sm"
							asChild
							className="px-2 sm:px-6 rounded text-xs sm:text-sm border-[#ef4444]/50 dark:border-[#ef4444]/50 bg-transparent text-black dark:text-white hover:bg-[#ef4444]/10 dark:hover:bg-[#ef4444]/10 hover:text-[#ef4444] dark:hover:text-[#ef4444]"
						>
							<Link href="https://github.com/Alpha-Innovation-Labs/ratkit" target="_blank">
								<SquareArrowOutUpRight className="mr-1 h-4 w-4" />
								View on GitHub
							</Link>
						</Button>
					</div>
				</div>

				{/* Right Column - Code Window */}
				<div className="relative w-full mt-8 md:mt-0 p-1 sm:p-0">
					<ComponentExampleCodeTabs />
				</div>
			</div>
		</div>
	);
}

const ComponentExampleCodeTabs = () => {
	const [activeCategory, setActiveCategory] = useState<Category>("button");

	/* ------------------------------------------------------------------ */
	/* Types */
	/* ------------------------------------------------------------------ */

	type Category = "button" | "dialog" | "scroll" | "grid";

	const CATEGORIES: Record<Category, { label: string; icon: React.ReactNode }> = {
		button: { label: "Button", icon: <MousePointer className="w-4 h-4 mr-2" /> },
		dialog: { label: "Dialog", icon: <PanelLeft className="w-4 h-4 mr-2" /> },
		scroll: { label: "Scroll", icon: <Layout className="w-4 h-4 mr-4" /> },
		grid: { label: "Grid", icon: <Layout className="w-4 h-4 mr-2" /> },
	};

	const TEMPLATES: Record<Category, string> = {
		button: `use ratkit::primitives::button::{Button, ButtonState};
use ratatui::Frame;

fn render_button(frame: &mut Frame) {
    let button = Button::new("Click Me")
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow));
    
    frame.render_widget(button, frame.area());
}`,

		dialog: `use ratkit::primitives::dialog::Dialog;
use ratatui::Frame;

fn show_dialog(frame: &mut Frame) {
    let dialog = Dialog::new()
        .title("Confirm Action")
        .content("Are you sure you want to continue?")
        .button("Yes", DialogAction::Confirm)
        .button("No", DialogAction::Cancel);
    
    frame.render_widget(dialog, frame.area());
}`,

		scroll: `use ratkit::primitives::scroll::{ScrollArea, ScrollState};
use ratatui::Frame;

fn render_scrollable(frame: &mut Frame) {
    let mut state = ScrollState::default();
    
    let scroll = ScrollArea::vertical()
        .content(&long_content)
        .scroll_state(&mut state);
    
    frame.render_stateful_widget(
        scroll, 
        frame.area(), 
        &mut state
    );
}`,

		grid: `use ratkit::primitives::resizable_grid::ResizableGrid;
use ratatui::Frame;

fn render_grid(frame: &mut Frame) {
    let grid = ResizableGrid::new(3, 3)
        .column_constraints(vec![
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .resizable(true);
    
    frame.render_widget(grid, frame.area());
}`,
	};

	const codes = useMemo(() => TEMPLATES, []);

	return (
		<div className="flex flex-col space-y-4 w-full max-w-2xl mx-auto">
			{/* Category Switch */}
			<div className="flex flex-col sm:flex-row items-start sm:items-center justify-between w-full gap-4">
				<div className="flex p-1 bg-black/5 dark:bg-white/5 rounded-xs w-fit">
					{(Object.keys(CATEGORIES) as Category[]).map((cat) => (
						<button
							key={cat}
							type="button"
							onClick={() => setActiveCategory(cat)}
							className={`px-2 py-1.5 text-xs lg:text-[10px] xl:text-xs sm:px-4 font-medium rounded-xs transition-all cursor-pointer flex items-center ${
								activeCategory === cat
									? "bg-white dark:bg-zinc-800 text-black dark:text-white shadow-sm"
									: "text-gray-500 hover:text-black dark:hover:text-white"
							}`}
						>
							{CATEGORIES[cat].icon}
							{CATEGORIES[cat].label}
						</button>
					))}
				</div>
			</div>

			{/* Code Display */}
			<div className="custom-code-block rounded-xs">
				<DynamicCodeBlock
					lang="rust"
					code={codes[activeCategory]}
				/>
			</div>
		</div>
	);
};
