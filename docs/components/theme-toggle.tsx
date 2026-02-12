"use client";

import { Moon, Sun } from "lucide-react";
import { useTheme } from "next-themes";
import * as React from "react";
import { Button } from "@/components/ui/button";
import { setThemeCookie } from "@/lib/setThemeCookie";

export function ThemeToggle() {
	const { theme, setTheme } = useTheme();
	const [mounted, setMounted] = React.useState(false);

	React.useEffect(() => {
		setMounted(true);
	}, []);

	if (!mounted) {
		return (
			<Button variant="ghost" size="icon" className="h-9 w-9">
				<Moon className="h-4 w-4" />
				<span className="sr-only">Toggle theme</span>
			</Button>
		);
	}

	const handleThemeChange = () => {
		const value = theme === "dark" ? "light" : "dark";
		setTheme(value);
		setThemeCookie(value);
	};

	return (
		<Button
			variant="ghost"
			size="icon"
			className="h-9 w-9"
			onClick={handleThemeChange}
		>
			<Moon className="h-4 w-4 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
			<Sun className="absolute h-4 w-4 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
			<span className="sr-only">Toggle theme</span>
		</Button>
	);
}
