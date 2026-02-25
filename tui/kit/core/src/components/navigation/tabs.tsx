import { Text } from "ink";
import { clampIndex } from "../../utils/clamp";

export type TabsProps = {
	tabs: ReadonlyArray<string>;
	activeIndex: number;
};

export function renderTabs(tabs: ReadonlyArray<string>, activeIndex: number): string {
	if (tabs.length === 0) {
		return "";
	}

	const selected = clampIndex(activeIndex, tabs.length);
	return tabs.map((tab, index) => (index === selected ? `[${tab}]` : ` ${tab} `)).join(" | ");
}

export function Tabs({ tabs, activeIndex }: TabsProps) {
	return <Text>{renderTabs(tabs, activeIndex)}</Text>;
}
