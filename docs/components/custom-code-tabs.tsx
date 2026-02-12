import { Anthropic, Google, OpenAI } from "@lobehub/icons";
import { DynamicCodeBlock } from "fumadocs-ui/components/dynamic-codeblock";
import {
	Tabs,
	TabsContent,
	TabsList,
	TabsTrigger,
} from "fumadocs-ui/components/tabs";
import {
	anthropicGenerateText,
	anthropicStreamText,
	googleGenerateText,
	googleStreamText,
	openaiGenerateText,
	openaiStreamText,
} from "./codes";

// store the icons and codes components in a map for easy access
const icons = { OpenAI: OpenAI, Anthropic: Anthropic, Google: Google.Color };
const codes = {
	OpenAIGenerateText: openaiGenerateText,
	AnthropicGenerateText: anthropicGenerateText,
	GoogleGenerateText: googleGenerateText,
	OpenAIStreamText: openaiStreamText,
	AnthropicStreamText: anthropicStreamText,
	GoogleStreamText: googleStreamText,
};

interface TabData {
	code:
		| "OpenAIGenerateText"
		| "AnthropicGenerateText"
		| "GoogleGenerateText"
		| "OpenAIStreamText"
		| "AnthropicStreamText"
		| "GoogleStreamText";
	icon: "OpenAI" | "Anthropic" | "Google";
	value: "openai" | "anthropic" | "google";
	label: string;
}

interface CompletedTabData extends Omit<TabData, "icon" | "code"> {
	icon: React.ComponentType;
	code: string;
}

interface CustomCodeTabsProps {
	tabsData: TabData[];
}

export const CustomCodeTabs = ({ tabsData }: CustomCodeTabsProps) => {
	const completedTabsData: CompletedTabData[] = tabsData.map((tab) => ({
		...tab,
		icon: icons[tab.icon],
		code: codes[tab.code],
	}));

	return (
		<Tabs defaultValue={completedTabsData[0]?.value}>
			<TabsList>
				{completedTabsData.map((tab) => (
					<TabsTrigger key={tab.value} value={tab.value}>
						<tab.icon />
						{tab.label}
					</TabsTrigger>
				))}
			</TabsList>
			{completedTabsData.map((tab) => (
				<TabsContent key={tab.value} value={tab.value}>
					<DynamicCodeBlock lang="rust" code={tab.code} />
				</TabsContent>
			))}
		</Tabs>
	);
};
