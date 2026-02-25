import { Box, Text } from "ink";
import { useTuiColors, useTuiTheme, useTuiViewport } from "../../app/app-provider";
import { UiText } from "../../primitives/display/text";
import { terminalMeetsMinimum } from "../../runtime/terminal/screen-size";
import { resolveIntentColor } from "../../theme";
import { FrameCanvas } from "./frame-canvas";
import { SurfacePane } from "./pane";
import type {
	SurfaceFrameLayoutGuards,
	ResolveSurfaceFrameGeometryOptions,
	SurfacePaneProps,
	SurfaceFrameGeometry,
	SurfaceFrameProps,
} from "./types";

const BORDER_COLUMNS = 2;
const HORIZONTAL_PADDING_COLUMNS = 2;
const BORDER_ROWS = 2;

function textColumnsFromOuterWidth(outerWidth: number): number {
	return Math.max(1, outerWidth - BORDER_COLUMNS - HORIZONTAL_PADDING_COLUMNS);
}

function textRowsFromOuterHeight(outerHeight: number, titleRows: number, paddingY: number): number {
	return Math.max(1, outerHeight - BORDER_ROWS - titleRows - Math.max(0, paddingY * 2));
}

function paneTitleRows(
	pane: Omit<SurfacePaneProps, "outerWidth" | "outerHeight" | "minOuterWidth" | "flexGrow">,
	defaultBorderStyle: "single" | "double" | "round" | "bold",
): number {
	if (pane.title === undefined || pane.title === null) {
		return 0;
	}
	if (pane.titleMode === "none") {
		return 0;
	}

	const resolvedBorderStyle = pane.borderStyle ?? defaultBorderStyle;
	const hasBorder = resolvedBorderStyle !== "none";
	const title =
		typeof pane.title === "string" ||
		typeof pane.title === "number" ||
		typeof pane.title === "bigint"
			? String(pane.title)
			: "";

	return hasBorder && title.length > 0 ? 2 : 1;
}

function normalizeMinWidth(value: number | undefined): number {
	if (typeof value !== "number" || !Number.isFinite(value)) {
		return 1;
	}
	return Math.max(1, Math.floor(value));
}

function normalizeRatio(value: number | undefined): number | undefined {
	if (typeof value !== "number" || !Number.isFinite(value)) {
		return undefined;
	}
	return Math.max(0, Math.min(1, value));
}

function resolveLayoutGuards(
	layoutGuards: SurfaceFrameLayoutGuards | undefined,
): Required<SurfaceFrameLayoutGuards> {
	const minLeftOuterWidth = normalizeMinWidth(layoutGuards?.minLeftOuterWidth);
	const minRightOuterWidth = normalizeMinWidth(layoutGuards?.minRightOuterWidth);
	const maxLeftWidthRatio = normalizeRatio(layoutGuards?.maxLeftWidthRatio) ?? 1;

	return {
		minLeftOuterWidth,
		minRightOuterWidth,
		maxLeftWidthRatio,
	};
}

export function resolveSurfaceFrameGeometry(
	options: ResolveSurfaceFrameGeometryOptions,
): SurfaceFrameGeometry {
	const frameColumns = Math.max(1, Math.floor(options.columns));
	const frameRows = Math.max(1, Math.floor(options.rows));
	const headerRows =
		options.hasHeader === false ? 0 : Math.max(1, Math.floor(options.headerHeight));
	const footerRows = options.hasFooter ? Math.max(1, Math.floor(options.footerHeight)) : 0;
	const bodyRows = Math.max(3, frameRows - headerRows - footerRows);
	const gutterColumns = Math.max(0, Math.floor(options.gutterColumns ?? 0));
	const leftTitleRows = Math.max(0, Math.floor(options.leftTitleRows ?? 0));
	const rightTitleRows = Math.max(0, Math.floor(options.rightTitleRows ?? 0));
	const guards = resolveLayoutGuards(options.layoutGuards);
	const preferredLeftWidth = Math.max(
		guards.minLeftOuterWidth,
		Math.floor(options.requestedLeftWidth),
	);
	const maxLeftByRightFloor = frameColumns - gutterColumns - guards.minRightOuterWidth;
	const maxLeftByAnyRight = frameColumns - gutterColumns - 1;
	let leftUpperBound = maxLeftByAnyRight;

	if (maxLeftByRightFloor >= guards.minLeftOuterWidth) {
		leftUpperBound = Math.min(leftUpperBound, maxLeftByRightFloor);
	}

	if (guards.maxLeftWidthRatio < 1) {
		const maxLeftByRatio = Math.max(
			guards.minLeftOuterWidth,
			Math.floor(frameColumns * guards.maxLeftWidthRatio),
		);
		leftUpperBound = Math.min(leftUpperBound, maxLeftByRatio);
	}

	leftUpperBound = Math.max(guards.minLeftOuterWidth, leftUpperBound);
	const leftOuterWidth = Math.max(
		guards.minLeftOuterWidth,
		Math.min(leftUpperBound, preferredLeftWidth),
	);
	const rightOuterWidth = Math.max(1, frameColumns - leftOuterWidth - gutterColumns);

	return {
		frameColumns,
		frameRows,
		headerRows,
		footerRows,
		bodyRows,
		gutterColumns,
		leftOuterWidth,
		rightOuterWidth,
		leftTextColumns: textColumnsFromOuterWidth(leftOuterWidth),
		leftTextRows: textRowsFromOuterHeight(bodyRows, leftTitleRows, 0),
		rightTextColumns: textColumnsFromOuterWidth(rightOuterWidth),
		rightTextRows: textRowsFromOuterHeight(bodyRows, rightTitleRows, 0),
	};
}

export function SurfaceFrame({
	title,
	showHeader = true,
	leftWidth,
	layoutGuards,
	leftPane,
	rightPane,
	footer,
	minColumns,
	minRows,
}: SurfaceFrameProps) {
	const theme = useTuiTheme();
	const colors = useTuiColors();
	const viewport = useTuiViewport();
	const warning = resolveIntentColor(theme, "warning", colors);
	const headerBackground = resolveIntentColor(theme, "headerBackground", colors);
	const footerBackground = resolveIntentColor(theme, "footerBackground", colors);
	const footerText = resolveIntentColor(theme, "dimText", colors);

	const requiredColumns = minColumns ?? theme.layout.minColumns;
	const requiredRows = minRows ?? theme.layout.minRows;
	const hasMinimumViewport = terminalMeetsMinimum(viewport, {
		columns: requiredColumns,
		rows: requiredRows,
	});

	if (!hasMinimumViewport) {
		return (
			<Box paddingX={1} paddingY={1}>
				<Text color={warning}>
					Terminal too small: got {viewport.columns}x{viewport.rows}, need {requiredColumns}x
					{requiredRows}.
				</Text>
			</Box>
		);
	}

	const geometry = resolveSurfaceFrameGeometry({
		columns: viewport.columns,
		rows: viewport.rows,
		requestedLeftWidth: leftWidth ?? theme.layout.sidebarWidth,
		headerHeight: theme.layout.headerHeight,
		footerHeight: theme.layout.footerHeight,
		hasFooter: Boolean(footer),
		hasHeader: showHeader,
		gutterColumns: theme.chrome.framePaneGap,
		leftTitleRows: paneTitleRows(leftPane, theme.chrome.borderGlyphSet),
		rightTitleRows: paneTitleRows(rightPane, theme.chrome.borderGlyphSet),
		layoutGuards,
	});
	const guards = resolveLayoutGuards(layoutGuards);

	const headerNode =
		typeof title === "string" ? (
			<UiText bold>{title}</UiText>
		) : (
			(title ?? <UiText bold>tui</UiText>)
		);

	return (
		<FrameCanvas
			frameColumns={geometry.frameColumns}
			frameRows={geometry.frameRows}
			headerRows={geometry.headerRows}
			footerRows={geometry.footerRows}
			bodyRows={geometry.bodyRows}
			leftWidth={geometry.leftOuterWidth}
			rightWidth={geometry.rightOuterWidth}
			gutterColumns={geometry.gutterColumns}
			showHeader={showHeader}
			header={headerNode}
			footer={footer}
			headerBackground={headerBackground}
			footerBackground={footerBackground}
			footerTextColor={footerText}
			leftPane={
				<SurfacePane
					{...leftPane}
					outerWidth={geometry.leftOuterWidth}
					outerHeight={geometry.bodyRows}
					minOuterWidth={guards.minLeftOuterWidth}
					flexGrow={0}
					textIntent={leftPane.textIntent ?? "sidebarItemText"}
					backgroundIntent={leftPane.backgroundIntent ?? "sidebarBackground"}
					borderIntent={leftPane.borderIntent ?? "sidebarBorder"}
				/>
			}
			rightPane={
				<SurfacePane
					{...rightPane}
					outerWidth={geometry.rightOuterWidth}
					outerHeight={geometry.bodyRows}
					minOuterWidth={guards.minRightOuterWidth}
					flexGrow={1}
					textIntent={rightPane.textIntent ?? "text"}
					backgroundIntent={rightPane.backgroundIntent ?? "contentBackground"}
					borderIntent={rightPane.borderIntent ?? "border"}
				/>
			}
		/>
	);
}
