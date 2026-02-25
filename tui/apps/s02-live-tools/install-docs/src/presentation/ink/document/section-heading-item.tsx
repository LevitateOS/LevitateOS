import { Box } from "ink";
import { UiText } from "@levitate/tui-kit";
import type { ReactNode } from "react";

type SectionHeadingItemProps = {
	title: string;
	level: 2 | 3;
	indent?: number;
};

export function SectionHeadingItem({
	title,
	level,
	indent = 0,
}: SectionHeadingItemProps): ReactNode {
	const intent = level === 3 ? "sectionSubheading" : "sectionHeading";
	const marker = level === 3 ? "›" : "◆";
	return (
		<Box flexDirection="column" paddingLeft={indent}>
			<UiText intent={intent} bold>
				{`${marker} ${title}`}
			</UiText>
		</Box>
	);
}
