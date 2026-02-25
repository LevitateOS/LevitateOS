import type { ColorIntent } from "@levitate/tui-kit";
import type { ReactNode } from "react";
import { useIntentColor } from "../blocks/shared/intent-color";
import { SyntaxLine } from "../blocks/shared/syntax-line";

type CommandLineRowProps = {
	line: string;
	width: number;
	rowIndex?: number;
	prefix?: string;
	fallbackIntent?: ColorIntent;
	backgroundIntent?: ColorIntent;
	bold?: boolean;
};

const COMMAND_ROW_BACKGROUND = "#000000";

export function CommandLineRow({
	line,
	width,
	rowIndex = 0,
	prefix = "$ ",
	fallbackIntent = "commandPrompt",
	backgroundIntent = "commandBarBackground",
	bold = true,
}: CommandLineRowProps): ReactNode {
	void rowIndex;
	const lineBackground = useIntentColor(backgroundIntent) ?? COMMAND_ROW_BACKGROUND;
	const fullLine = `${prefix}${line}`;

	return (
		<SyntaxLine
			line={fullLine}
			fallbackIntent={fallbackIntent}
			backgroundIntent={backgroundIntent}
			backgroundColor={lineBackground}
			width={width}
			bold={bold}
		/>
	);
}
