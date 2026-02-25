import { Box, Text } from "ink";
import {
	Children,
	isValidElement,
	useMemo,
	type JSX,
	type ReactElement,
	type ReactNode,
} from "react";
import { UiText } from "../primitives/display/text";
import { resolveChromeGlyphSet } from "./glyph-sets";
import { renderChromeRows } from "./renderer";
import type {
	ChromeFrameProps,
	ChromeInstruction,
	ChromeLayerProps,
	ChromeRuleProps,
	ChromeSeamProps,
	ChromeSplitProps,
	ChromeTextProps,
} from "./types";

const CHROME_NODE_KIND = Symbol("chrome-node-kind");

type ChromeNodeKind = "frame" | "rule" | "seam" | "split" | "text";

type ChromeInstructionComponent<P> = ((props: P) => null) & {
	[CHROME_NODE_KIND]: ChromeNodeKind;
	displayName?: string;
};

function createChromeNode<P>(
	kind: ChromeNodeKind,
	displayName: string,
): ChromeInstructionComponent<P> {
	const Component = (() => null) as unknown as ChromeInstructionComponent<P>;
	Component[CHROME_NODE_KIND] = kind;
	Component.displayName = displayName;
	return Component;
}

function childToInstruction(child: ReactElement): ChromeInstruction | null {
	const componentType = child.type as ChromeInstructionComponent<unknown>;
	const kind = componentType[CHROME_NODE_KIND];
	if (!kind) {
		return null;
	}

	const props = child.props as Record<string, unknown>;
	if (kind === "frame") {
		return {
			kind,
			rect: props.rect as ChromeFrameProps["rect"],
			intent: props.intent as ChromeFrameProps["intent"],
		};
	}
	if (kind === "rule") {
		return {
			kind,
			y: Number(props.y ?? 0),
			x0: Number(props.x0 ?? 0),
			x1: Number(props.x1 ?? 0),
			intent: props.intent as ChromeRuleProps["intent"],
		};
	}
	if (kind === "seam") {
		return {
			kind,
			y: Number(props.y ?? 0),
			x0: Number(props.x0 ?? 0),
			x1: Number(props.x1 ?? 0),
			intent: props.intent as ChromeSeamProps["intent"],
		};
	}
	if (kind === "split") {
		return {
			kind,
			x: Number(props.x ?? 0),
			y0: Number(props.y0 ?? 0),
			y1: Number(props.y1 ?? 0),
			intent: props.intent as ChromeSplitProps["intent"],
		};
	}

	return {
		kind,
		x: Number(props.x ?? 0),
		y: Number(props.y ?? 0),
		text: String(props.text ?? ""),
		intent: props.intent as ChromeTextProps["intent"],
		bold: Boolean(props.bold),
	};
}

function collectInstructions(children: ReactNode, output: ChromeInstruction[]): void {
	Children.forEach(children, (node) => {
		if (!isValidElement(node)) {
			return;
		}

		const instruction = childToInstruction(node);
		if (instruction) {
			output.push(instruction);
			return;
		}

		const nested = (node.props as { children?: ReactNode } | undefined)?.children;
		if (nested !== undefined) {
			collectInstructions(nested, output);
		}
	});
}

export const ChromeFrame = createChromeNode<ChromeFrameProps>("frame", "ChromeFrame");
export const ChromeRule = createChromeNode<ChromeRuleProps>("rule", "ChromeRule");
export const ChromeSeam = createChromeNode<ChromeSeamProps>("seam", "ChromeSeam");
export const ChromeSplit = createChromeNode<ChromeSplitProps>("split", "ChromeSplit");
export const ChromeText = createChromeNode<ChromeTextProps>("text", "ChromeText");

export function ChromeLayer({
	width,
	height,
	lineWeight = "single",
	glyphSet,
	instructions,
	children,
	rowOffset = 0,
	rowCount,
}: ChromeLayerProps): JSX.Element {
	const safeWidth = Math.max(1, Math.floor(width));
	const safeHeight = Math.max(1, Math.floor(height));
	const safeOffset = Math.max(0, Math.floor(rowOffset));
	const resolvedRowCount =
		typeof rowCount === "number" && Number.isFinite(rowCount)
			? Math.max(0, Math.floor(rowCount))
			: Math.max(0, safeHeight - safeOffset);
	const maxVisibleRows = Math.max(0, safeHeight - safeOffset);
	const visibleRows = Math.min(resolvedRowCount, maxVisibleRows);

	const collectedInstructions = useMemo(() => {
		const result: ChromeInstruction[] = [];
		if (instructions) {
			result.push(...instructions);
		}
		if (children !== undefined) {
			collectInstructions(children, result);
		}
		return result;
	}, [children, instructions]);

	const rows = useMemo(() => {
		const renderedRows = renderChromeRows({
			width: safeWidth,
			height: safeHeight,
			glyphSet: glyphSet ?? resolveChromeGlyphSet(lineWeight),
			instructions: collectedInstructions,
		});
		return renderedRows.slice(safeOffset, safeOffset + visibleRows);
	}, [collectedInstructions, glyphSet, lineWeight, safeHeight, safeOffset, safeWidth, visibleRows]);

	return (
		<Box flexDirection="column" width={safeWidth} height={visibleRows} flexShrink={0}>
			{rows.map((row, rowIndex) => (
				<Box key={`chrome-row-${rowIndex}`} flexDirection="row" flexShrink={0}>
					{row.spans.map((span, spanIndex) => {
						if (span.text.length === 0) {
							return null;
						}

						if (span.intent) {
							return (
								<UiText
									key={`chrome-span-${rowIndex}-${spanIndex}`}
									intent={span.intent}
									bold={Boolean(span.bold)}
								>
									{span.text}
								</UiText>
							);
						}

						return (
							<Text key={`chrome-span-${rowIndex}-${spanIndex}`} bold={Boolean(span.bold)}>
								{span.text}
							</Text>
						);
					})}
				</Box>
			))}
		</Box>
	);
}
