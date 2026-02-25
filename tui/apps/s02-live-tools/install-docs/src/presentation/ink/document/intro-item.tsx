import { Box } from "ink";
import type { RichText } from "@levitate/docs-content";
import type { ReactNode } from "react";
import { RichParagraph } from "../primitives/rich-paragraph";

type IntroItemProps = {
	content: string | RichText;
	contentWidth: number;
	indent?: number;
};

export function IntroItem({ content, contentWidth, indent = 0 }: IntroItemProps): ReactNode {
	const safeWidth = Math.max(1, contentWidth);
	const introWidth = Math.max(1, safeWidth - indent);
	return (
		<Box flexDirection="column" paddingLeft={indent} width={safeWidth}>
			<RichParagraph content={content} width={introWidth} intent="dimText" minimumWidth={1} />
		</Box>
	);
}
