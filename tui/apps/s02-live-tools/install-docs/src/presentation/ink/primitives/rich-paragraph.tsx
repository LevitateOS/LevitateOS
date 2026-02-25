import type { RichText } from "@levitate/docs-content";
import type { ColorIntent } from "@levitate/tui-kit";
import type { ReactNode } from "react";
import { RichTextRenderer } from "../blocks/shared/rich-text-renderer";

type RichParagraphProps = {
	content: string | RichText | undefined;
	width: number;
	intent?: ColorIntent;
	minimumWidth?: number;
	selectedLinkHref?: string;
};

export function RichParagraph({
	content,
	width,
	intent = "text",
	minimumWidth = 1,
	selectedLinkHref,
}: RichParagraphProps): ReactNode {
	return (
		<RichTextRenderer
			content={content}
			defaultIntent={intent}
			width={width}
			minimumWidth={minimumWidth}
			selectedLinkHref={selectedLinkHref}
		/>
	);
}
