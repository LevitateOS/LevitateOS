import { Box } from "ink";
import type { InlineNode, RichText } from "@levitate/docs-content";
import { RichTextLine, type ColorIntent, type RichTextRun } from "@levitate/tui-kit";
import type { ReactNode } from "react";

type RichTextRendererProps = {
	content: string | RichText | undefined;
	defaultIntent?: ColorIntent;
	width?: number;
	minimumWidth?: number;
};

type RichTextWord = {
	runs: RichTextRun[];
	length: number;
};

function isWhitespace(char: string): boolean {
	return /\s/.test(char);
}

function sameRunStyle(a: RichTextRun, b: RichTextRun): boolean {
	return (
		a.intent === b.intent &&
		a.backgroundIntent === b.backgroundIntent &&
		a.literalColor === b.literalColor &&
		a.bold === b.bold &&
		a.italic === b.italic &&
		a.underline === b.underline
	);
}

function cloneRun(run: RichTextRun): RichTextRun {
	return {
		text: run.text,
		intent: run.intent,
		backgroundIntent: run.backgroundIntent,
		literalColor: run.literalColor,
		bold: run.bold,
		italic: run.italic,
		underline: run.underline,
	};
}

function appendRun(target: RichTextRun[], run: RichTextRun): void {
	if (run.text.length === 0) {
		return;
	}
	const last = target[target.length - 1];
	if (last && sameRunStyle(last, run)) {
		last.text += run.text;
		return;
	}
	target.push(cloneRun(run));
}

function plainTextForInlineNode(node: InlineNode): string {
	if (typeof node === "string") {
		return node;
	}
	if (node.type === "link") {
		if (node.href.trim().length > 0 && node.href !== node.text) {
			return `${node.text} (${node.href})`;
		}
		return node.text;
	}
	return node.text;
}

function inlineNodeToRuns(node: InlineNode, defaultIntent: ColorIntent): RichTextRun[] {
	if (typeof node === "string") {
		return [{ text: node, intent: defaultIntent }];
	}

	if (node.type === "link") {
		if (node.href.trim().length > 0 && node.href !== node.text) {
			return [
				{ text: node.text, intent: "accent", underline: true },
				{ text: ` (${node.href})`, intent: "dimText" },
			];
		}
		return [{ text: node.text, intent: "accent", underline: true }];
	}

	if (node.type === "bold") {
		return [{ text: node.text, intent: defaultIntent, bold: true }];
	}

	if (node.type === "code") {
		return [{ text: node.text, intent: "warning" }];
	}

	return [{ text: node.text, intent: defaultIntent, italic: true }];
}

function inlineContentToRuns(
	content: string | RichText | undefined,
	defaultIntent: ColorIntent,
): RichTextRun[] {
	if (typeof content === "string") {
		return [{ text: content, intent: defaultIntent }];
	}
	if (!Array.isArray(content)) {
		return [];
	}

	const runs: RichTextRun[] = [];
	for (const node of content) {
		for (const run of inlineNodeToRuns(node, defaultIntent)) {
			appendRun(runs, run);
		}
	}
	return runs;
}

function splitRunsByNewline(runs: ReadonlyArray<RichTextRun>): RichTextRun[][] {
	const lines: RichTextRun[][] = [[]];
	for (const run of runs) {
		const parts = run.text.split("\n");
		for (let index = 0; index < parts.length; index += 1) {
			const text = parts[index] ?? "";
			if (text.length > 0) {
				appendRun(lines[lines.length - 1]!, { ...run, text });
			}
			if (index < parts.length - 1) {
				lines.push([]);
			}
		}
	}
	return lines;
}

function extractWords(runs: ReadonlyArray<RichTextRun>): RichTextWord[] {
	const words: RichTextWord[] = [];
	let currentRuns: RichTextRun[] = [];
	let currentLength = 0;

	const flushWord = () => {
		if (currentLength === 0) {
			return;
		}
		words.push({
			runs: currentRuns.map(cloneRun),
			length: currentLength,
		});
		currentRuns = [];
		currentLength = 0;
	};

	for (const run of runs) {
		for (const char of run.text) {
			if (isWhitespace(char)) {
				flushWord();
				continue;
			}
			appendRun(currentRuns, { ...run, text: char });
			currentLength += 1;
		}
	}

	flushWord();
	return words;
}

function chunkWord(word: RichTextWord, width: number): RichTextWord[] {
	if (word.length <= width) {
		return [
			{
				runs: word.runs.map(cloneRun),
				length: word.length,
			},
		];
	}

	const chunks: RichTextWord[] = [];
	let chunkRuns: RichTextRun[] = [];
	let chunkLength = 0;

	const flushChunk = () => {
		if (chunkLength === 0) {
			return;
		}
		chunks.push({
			runs: chunkRuns.map(cloneRun),
			length: chunkLength,
		});
		chunkRuns = [];
		chunkLength = 0;
	};

	for (const run of word.runs) {
		for (const char of run.text) {
			if (chunkLength >= width) {
				flushChunk();
			}
			appendRun(chunkRuns, { ...run, text: char });
			chunkLength += 1;
		}
	}

	flushChunk();
	return chunks;
}

function wrapParagraph(words: ReadonlyArray<RichTextWord>, width: number): RichTextRun[][] {
	if (words.length === 0) {
		return [[]];
	}

	const lines: RichTextRun[][] = [];
	let currentRuns: RichTextRun[] = [];
	let currentLength = 0;

	const flushLine = () => {
		lines.push(currentRuns.map(cloneRun));
		currentRuns = [];
		currentLength = 0;
	};

	for (const word of words) {
		for (const chunk of chunkWord(word, width)) {
			if (currentLength === 0) {
				for (const run of chunk.runs) {
					appendRun(currentRuns, run);
				}
				currentLength = chunk.length;
				continue;
			}

			if (currentLength + 1 + chunk.length <= width) {
				appendRun(currentRuns, { text: " " });
				for (const run of chunk.runs) {
					appendRun(currentRuns, run);
				}
				currentLength += 1 + chunk.length;
				continue;
			}

			flushLine();
			for (const run of chunk.runs) {
				appendRun(currentRuns, run);
			}
			currentLength = chunk.length;
		}
	}

	if (currentRuns.length > 0 || lines.length === 0) {
		flushLine();
	}

	return lines;
}

function safeWrapWidth(width: number | undefined, minimumWidth: number): number {
	const safeMinimum = Math.max(1, Math.trunc(minimumWidth));
	const requested =
		typeof width === "number" && Number.isFinite(width) ? Math.trunc(width) : safeMinimum;
	return Math.max(safeMinimum, requested);
}

export function inlineContentToPlainText(content: string | RichText | undefined): string {
	if (typeof content === "string") {
		return content;
	}
	if (!Array.isArray(content)) {
		return "";
	}
	return content.map((node) => plainTextForInlineNode(node)).join("");
}

export function wrapRichTextRuns(
	content: string | RichText | undefined,
	width: number,
	defaultIntent: ColorIntent = "text",
	minimumWidth = 1,
): RichTextRun[][] {
	const runs = inlineContentToRuns(content, defaultIntent);
	const safeWidth = safeWrapWidth(width, minimumWidth);
	const paragraphs = splitRunsByNewline(runs);
	const wrapped = paragraphs.flatMap((paragraphRuns) =>
		wrapParagraph(extractWords(paragraphRuns), safeWidth),
	);
	return wrapped.length > 0 ? wrapped : [[]];
}

export function wrapRichTextPlainLines(
	content: string | RichText | undefined,
	width: number,
	defaultIntent: ColorIntent = "text",
	minimumWidth = 1,
): string[] {
	return wrapRichTextRuns(content, width, defaultIntent, minimumWidth).map((runs) =>
		runs.map((run) => run.text).join(""),
	);
}

export function richTextRuns(
	content: string | RichText | undefined,
	defaultIntent: ColorIntent = "text",
): RichTextRun[] {
	return inlineContentToRuns(content, defaultIntent);
}

export function RichTextRenderer({
	content,
	defaultIntent = "text",
	width,
	minimumWidth = 1,
}: RichTextRendererProps): ReactNode {
	if (typeof width === "number" && Number.isFinite(width)) {
		const lines = wrapRichTextRuns(content, width, defaultIntent, minimumWidth);
		return (
			<Box flexDirection="column">
				{lines.map((line, index) => (
					<RichTextLine key={`rich-line-${index}`} runs={line} fallbackIntent={defaultIntent} />
				))}
			</Box>
		);
	}

	return (
		<RichTextLine
			runs={inlineContentToRuns(content, defaultIntent)}
			fallbackIntent={defaultIntent}
		/>
	);
}
