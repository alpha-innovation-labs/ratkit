import "@/app/global.css";
import { HomeLayout } from "fumadocs-ui/layouts/home";
import { RootProvider } from "fumadocs-ui/provider/next";
import type { Metadata } from "next";
import { JetBrains_Mono } from "next/font/google";
import { cookies } from "next/headers";
import { Navbar } from "@/components/navbar";
import SearchDialog from "@/components/search";
import { ThemeProvider } from "@/components/theme-provider";
import { VercelAnalytics } from "@/lib/analytics";
import { baseOptions } from "@/lib/layout.shared";

const jetbrainsMono = JetBrains_Mono({
	subsets: ["latin"],
});

export const metadata: Metadata = {
	title: "ratkit | TUI Toolkit for Rust",
	description: "A comprehensive collection of reusable TUI components for ratatui.",
};

export default async function Layout({ children }: LayoutProps<"/">) {
	const cookieStore = await cookies();
	const themeCookie = cookieStore.get("theme");
	const theme = themeCookie?.value || "dark";

	return (
		<html lang="en" className={jetbrainsMono.className} suppressHydrationWarning>
			<body className="flex flex-col min-h-screen">
				<ThemeProvider theme={theme}>
					<RootProvider
						search={{
							SearchDialog,
						}}
					>
						<ServerNavbar />
						<HomeLayout {...baseOptions()}>{children}</HomeLayout>
					</RootProvider>
				</ThemeProvider>
				<VercelAnalytics />
			</body>
		</html>
	);
}

async function ServerNavbar() {
	async function _getGitHubStars() {
		try {
			const response = await fetch(
				"https://api.github.com/repos/Alpha-Innovation-Labs/ratkit",
				{
					next: {
						revalidate: 60,
					},
				},
			);
			if (!response?.ok) {
				return null;
			}
			const json = await response.json();
			const stars = parseInt(json.stargazers_count, 10).toLocaleString();
			return stars;
		} catch {
			return null;
		}
	}
	const data = null; // await getGitHubStars();
	return <Navbar starCout={data} />;
}
