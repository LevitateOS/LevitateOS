import { Box, Text } from "ink";
import type { ReactNode } from "react";
import {
	ChromeFrame,
	ChromeLayer,
	ChromeSeam,
	ChromeText,
	type ChromeLineWeight,
} from "../../chrome";
import { useTuiColors, useTuiTheme } from "../../app/app-provider";
import { resolveIntentColor } from "../../theme";
import { padRight, truncateLine } from "../../utils";
import type { SurfacePaneProps } from "./types";

function renderNode(value: ReactNode, color?: string, bold = false): ReactNode {
	if (typeof value === "string") {
		return (
			<Text color={color} bold={bold}>
				{value}
			</Text>
		);
	}

	return value;
}

function normalizeDimension(value: number | undefined): number | undefined {
	if (typeof value !== "number" || !Number.isFinite(value)) {
		return undefined;
	}
	return Math.max(1, Math.floor(value));
}

function paneTitleText(title: ReactNode): string {
	if (typeof title === "string" || typeof title === "number" || typeof title === "bigint") {
		return String(title);
	}
	return "";
}

export function SurfacePane({
	title,
	body,
	outerWidth,
	outerHeight,
	minOuterWidth,
	flexGrow,
	borderStyle,
	borderIntent = "border",
	backgroundIntent = "background",
	textIntent = "text",
	titleIntent = "sectionHeading",
	titleMode,
	paddingX,
	paddingY,
}: SurfacePaneProps) {
	const theme = useTuiTheme();
	const colors = useTuiColors();
	const borderColor = resolveIntentColor(theme, borderIntent, colors);
	const backgroundColor = resolveIntentColor(theme, backgroundIntent, colors);
	const textColor = resolveIntentColor(theme, textIntent, colors);
	const hasProvidedTitle = title !== undefined && title !== null;
	const resolvedTitleMode = titleMode ?? (hasProvidedTitle ? "slot" : "none");
	const resolvedPaddingX = paddingX ?? theme.chrome.panePaddingX;
	const resolvedPaddingY = paddingY ?? theme.chrome.panePaddingY;
	const resolvedBorderStyle = borderStyle ?? theme.chrome.borderGlyphSet;
	const hasBorder = resolvedBorderStyle !== "none";
	const titleText = hasProvidedTitle ? paneTitleText(title) : "";
	const hasTitle = hasProvidedTitle && resolvedTitleMode !== "none";
	const safeOuterWidth = normalizeDimension(outerWidth);
	const canRenderChromeTitle =
		hasTitle && titleText.length > 0 && hasBorder && typeof safeOuterWidth === "number";
	const safeOuterHeight = normalizeDimension(outerHeight);
	const chromeHeaderRows = canRenderChromeTitle ? 3 : 0;
	const bodyOuterHeight =
		typeof safeOuterHeight === "number"
			? Math.max(1, safeOuterHeight - chromeHeaderRows)
			: undefined;
	const innerColumns =
		typeof safeOuterWidth === "number" ? Math.max(1, safeOuterWidth - (hasBorder ? 2 : 0)) : 40;
	const titleCell = canRenderChromeTitle
		? padRight(truncateLine(titleText, innerColumns), innerColumns)
		: "";
	const chromeLineWeight =
		resolvedBorderStyle === "none" ? "single" : (resolvedBorderStyle as ChromeLineWeight);

	if (canRenderChromeTitle) {
		return (
			<Box
				width={outerWidth}
				height={outerHeight}
				minWidth={minOuterWidth}
				flexGrow={flexGrow}
				flexDirection="column"
				backgroundColor={backgroundColor}
			>
				<ChromeLayer width={safeOuterWidth} height={4} rowCount={3} lineWeight={chromeLineWeight}>
					<ChromeFrame
						rect={{ x: 0, y: 0, width: safeOuterWidth, height: 4 }}
						intent={borderIntent}
					/>
					<ChromeSeam y={2} x0={0} x1={safeOuterWidth - 1} intent={borderIntent} />
					<ChromeText x={1} y={1} text={titleCell} intent={titleIntent} bold />
				</ChromeLayer>
				<Box
					width={outerWidth}
					height={bodyOuterHeight}
					flexGrow={1}
					flexDirection="column"
					borderStyle={resolvedBorderStyle}
					borderTop={false}
					borderLeft
					borderRight
					borderBottom
					borderColor={borderColor}
					backgroundColor={backgroundColor}
				>
					<Box
						paddingX={Math.max(0, resolvedPaddingX)}
						paddingY={Math.max(0, resolvedPaddingY)}
						flexGrow={1}
						flexDirection="column"
					>
						{renderNode(body, textColor)}
					</Box>
				</Box>
			</Box>
		);
	}

	return (
		<Box
			width={outerWidth}
			height={outerHeight}
			minWidth={minOuterWidth}
			flexGrow={flexGrow}
			flexDirection="column"
			borderStyle={hasBorder ? resolvedBorderStyle : undefined}
			borderColor={hasBorder ? borderColor : undefined}
			backgroundColor={backgroundColor}
		>
			{hasTitle ? (
				<Box paddingX={Math.max(0, resolvedPaddingX)} flexShrink={0}>
					{renderNode(title, resolveIntentColor(theme, titleIntent, colors), true)}
				</Box>
			) : null}
			<Box
				paddingX={Math.max(0, resolvedPaddingX)}
				paddingY={Math.max(0, resolvedPaddingY)}
				flexGrow={1}
				flexDirection="column"
			>
				{renderNode(body, textColor)}
			</Box>
		</Box>
	);
}
