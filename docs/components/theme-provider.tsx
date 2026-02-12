"use client";
import { createContext, useContext } from "react";

interface ThemeContextValue {
	serverTheme: string;
}

const ThemeContext = createContext<ThemeContextValue>({ serverTheme: "dark" });

export function ThemeProvider({
	children,
	theme,
}: {
	children: React.ReactNode;
	theme: string;
}) {
	return (
		<ThemeContext.Provider value={{ serverTheme: theme }}>
			{children}
		</ThemeContext.Provider>
	);
}

export function useServerTheme() {
	return useContext(ThemeContext);
}
