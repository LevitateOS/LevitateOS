import { Text } from "ink";
import { parseSyntaxTokenLine, type ColorIntent } from "@levitate/tui-kit";
import type { ReactNode } from "react";
import { useIntentColor } from "./intent-color";

type SyntaxLineProps = {
	line: string;
	fallbackIntent?: ColorIntent;
	backgroundIntent?: ColorIntent;
	backgroundColor?: string;
	width?: number;
	bold?: boolean;
	italic?: boolean;
	underline?: boolean;
};

export function syntaxTokenColors(line: string): string[] {
	return parseSyntaxTokenLine(line)
		.map((token) => token.color)
		.filter((color): color is string => typeof color === "string" && color.length > 0);
}

export function SyntaxLine({
	line,
	fallbackIntent = "text",
	backgroundIntent,
	backgroundColor,
	width,
	bold = false,
	italic = false,
	underline = false,
}: SyntaxLineProps): ReactNode {
	const fallbackColor = useIntentColor(fallbackIntent);
	const resolvedBackgroundColor =
		typeof backgroundColor === "string" && backgroundColor.length > 0
			? backgroundColor
			: backgroundIntent
				? useIntentColor(backgroundIntent)
				: undefined;
	const tokens = parseSyntaxTokenLine(line);
	const safeWidth =
		typeof width === "number" && Number.isFinite(width)
			? Math.max(1, Math.trunc(width))
			: undefined;

	let paddedTokens = tokens;
	if (typeof safeWidth === "number" && tokens.length > 0) {
		const tokenLength = tokens.reduce((total, token) => total + token.text.length, 0);
		const padLength = safeWidth - tokenLength;
		if (padLength > 0) {
			paddedTokens = [
				...tokens,
				{
					text: " ".repeat(padLength),
					color: undefined,
				},
			];
		}
	}

	if (tokens.length === 0) {
		const blankLine = typeof safeWidth === "number" && safeWidth > 1 ? " ".repeat(safeWidth) : " ";
		return (
			<Text
				color={fallbackColor}
				backgroundColor={resolvedBackgroundColor}
				bold={bold}
				italic={italic}
				underline={underline}
			>
				{blankLine}
			</Text>
		);
	}

	return (
		<Text
			backgroundColor={resolvedBackgroundColor}
			bold={bold}
			italic={italic}
			underline={underline}
		>
			{paddedTokens.map((token, index) => (
				<Text
					key={`${index}-${token.text}`}
					color={token.color ?? fallbackColor}
					backgroundColor={resolvedBackgroundColor}
				>
					{token.text.length > 0 ? token.text : " "}
				</Text>
			))}
		</Text>
	);
}
