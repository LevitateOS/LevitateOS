import { Box, Text } from "ink";
import type { CodeBlock } from "@levitate/docs-content";
import {
	parseSyntaxTokenLine,
	resolveChromeGlyphSet,
	RichTextLine,
	truncateLine,
	type RichTextRun,
} from "@levitate/tui-kit";
import type { ReactNode } from "react";
import type { BlockComponentProps } from "./types";
import type { BlockPlugin } from "./contracts";
import { codeSnapshotLines } from "./shared/content-utils";
import { useIntentColor } from "./shared/intent-color";
import { defaultDocsBlockRendererKey } from "./shared/renderer-key";

const CODE_LINE_NUMBER_PADDING = 1;
type SyntaxToken = {
	text: string;
	color?: string;
};

function lineNumberText(sourceLine: number, digits: number): string {
	return String(sourceLine).padStart(digits, " ");
}

function pushSyntaxToken(row: SyntaxToken[], text: string, color?: string): void {
	if (text.length === 0) {
		return;
	}
	const previous = row[row.length - 1];
	if (previous && previous.color === color) {
		previous.text += text;
		return;
	}
	row.push({ text, color });
}

function wrapSyntaxRows(line: string, width: number): SyntaxToken[][] {
	const safeWidth = Math.max(1, width);
	if (line.length === 0) {
		return [[]];
	}
	const tokens = parseSyntaxTokenLine(line);
	if (tokens.length === 0) {
		return [[]];
	}
	const rows: SyntaxToken[][] = [];
	let current: SyntaxToken[] = [];
	let currentWidth = 0;
	const flush = () => {
		rows.push(current);
		current = [];
		currentWidth = 0;
	};
	for (const token of tokens) {
		for (const char of token.text) {
			if (currentWidth >= safeWidth) {
				flush();
			}
			pushSyntaxToken(current, char, token.color);
			currentWidth += 1;
		}
	}
	flush();
	if (rows.length === 0) {
		return [[]];
	}
	return rows;
}

function syntaxRowWidth(tokens: ReadonlyArray<SyntaxToken>): number {
	return tokens.reduce((total, token) => total + token.text.length, 0);
}

function syntaxRowToRuns(tokens: ReadonlyArray<SyntaxToken>, width: number): RichTextRun[] {
	const visibleTokens = tokens.filter((token) => token.text.length > 0);
	const runs: RichTextRun[] = visibleTokens.map((token) => ({
		text: token.text,
		literalColor: token.color,
		intent: "text" as const,
	}));
	const missing = Math.max(0, width - syntaxRowWidth(visibleTokens));
	if (missing > 0) {
		runs.push({
			text: " ".repeat(missing),
			intent: "text",
		});
	}
	return runs;
}

export function CodeBlockView({
	block,
	contentWidth,
	indent = 0,
}: BlockComponentProps<CodeBlock>): ReactNode {
	const safeWidth = Math.max(1, contentWidth);
	const innerWidth = Math.max(1, safeWidth - 2);
	const sourceLines = codeSnapshotLines(block);
	const lineDigits = Math.max(2, String(Math.max(1, sourceLines.length)).length);
	const lineNumberGutter = lineDigits + CODE_LINE_NUMBER_PADDING + 1;
	const codeInnerWidth = Math.max(1, innerWidth - lineNumberGutter);
	const borderColor = useIntentColor("cardBorder");
	const headingColor = useIntentColor("accent");
	const dimTextColor = useIntentColor("dimText");
	const chrome = resolveChromeGlyphSet("single");
	const labelParts = [block.language.toUpperCase()];
	if (typeof block.filename === "string" && block.filename.length > 0) {
		labelParts.push(block.filename);
	}
	const label = labelParts.join(" • ");
	const labelText = truncateLine(label, Math.max(1, innerWidth - 2));
	const topCenter = ` ${labelText} `;
	const topFill = Math.max(0, innerWidth - topCenter.length);
	const bottomLine = `${chrome.bl}${chrome.h.repeat(innerWidth)}${chrome.br}`;

	return (
		<Box flexDirection="column" paddingLeft={indent} width={safeWidth}>
			<Text color={borderColor}>
				<Text color={borderColor}>{chrome.tl}</Text>
				<Text color={headingColor} bold>
					{topCenter}
				</Text>
				<Text color={borderColor}>{chrome.h.repeat(topFill)}</Text>
				<Text color={borderColor}>{chrome.tr}</Text>
			</Text>
			{sourceLines.length === 0 ? (
				<Box flexDirection="row" width={safeWidth}>
					<Text color={borderColor}>{chrome.v}</Text>
					<Text color={dimTextColor}>{truncateLine("(empty code block)", innerWidth)}</Text>
					<Text color={borderColor}>{chrome.v}</Text>
				</Box>
			) : (
				sourceLines.map((line, lineIndex) => {
					const wrapped = wrapSyntaxRows(line, codeInnerWidth);
					return wrapped.map((tokens, wrappedIndex) => (
						<Box key={`code-${lineIndex}-${wrappedIndex}`} flexDirection="row" width={safeWidth}>
							<Text color={borderColor}>{chrome.v}</Text>
							<RichTextLine
								runs={[
									{
										text:
											wrappedIndex === 0
												? `${lineNumberText(lineIndex + 1, lineDigits)}${" ".repeat(CODE_LINE_NUMBER_PADDING)}${chrome.v}`
												: `${" ".repeat(lineDigits + CODE_LINE_NUMBER_PADDING)}${chrome.v}`,
										intent: "dimText",
									},
									...syntaxRowToRuns(tokens, codeInnerWidth),
								]}
								fallbackIntent="text"
							/>
							<Text color={borderColor}>{chrome.v}</Text>
						</Box>
					));
				})
			)}
			<Text color={borderColor}>{bottomLine}</Text>
		</Box>
	);
}

export const codeBlockPlugin: BlockPlugin<"code"> = {
	type: "code",
	rendererKey: defaultDocsBlockRendererKey("code"),
	render: (block, context, indent) => (
		<CodeBlockView block={block} contentWidth={context.contentWidth} indent={indent} />
	),
	measure: (block, context, indent) => {
		const safeWidth = Math.max(1, context.contentWidth - indent);
		const innerWidth = Math.max(1, safeWidth - 2);
		const lines = codeSnapshotLines(block);
		if (lines.length === 0) {
			return 3;
		}
		const lineDigits = Math.max(2, String(Math.max(1, lines.length)).length);
		const lineNumberGutter = lineDigits + CODE_LINE_NUMBER_PADDING + 1;
		const codeInnerWidth = Math.max(1, innerWidth - lineNumberGutter);
		const wrappedRows = lines.reduce(
			(total, line) => total + wrapSyntaxRows(line, codeInnerWidth).length,
			0,
		);
		return 2 + wrappedRows;
	},
};
