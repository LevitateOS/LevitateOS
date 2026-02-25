import type { ReactNode } from "react";
import type { ColorIntent } from "../../theme";

export type SurfacePaneTitleStyle = "slot" | "notched" | "plain";
export type SurfacePaneTitleMode = "none" | "inline" | "slot";

export type SurfacePaneProps = {
	title?: ReactNode;
	body: ReactNode;
	outerWidth?: number;
	outerHeight?: number;
	minOuterWidth?: number;
	flexGrow?: number;
	borderStyle?: "single" | "double" | "round" | "bold" | "none";
	borderIntent?: "border" | "sidebarBorder" | "cardBorder";
	backgroundIntent?: "background" | "sidebarBackground" | "contentBackground" | "cardBackground";
	textIntent?: ColorIntent;
	titleIntent?: ColorIntent;
	titleMode?: SurfacePaneTitleMode;
	titleStyle?: SurfacePaneTitleStyle;
	paddingX?: number;
	paddingY?: number;
};

export type SurfaceFrameLayoutGuards = {
	minLeftOuterWidth?: number;
	minRightOuterWidth?: number;
	maxLeftWidthRatio?: number;
};

export type SurfaceFrameProps = {
	title?: ReactNode;
	showHeader?: boolean;
	leftWidth?: number;
	layoutGuards?: SurfaceFrameLayoutGuards;
	leftPane: Omit<SurfacePaneProps, "outerWidth" | "outerHeight" | "minOuterWidth" | "flexGrow">;
	rightPane: Omit<SurfacePaneProps, "outerWidth" | "outerHeight" | "minOuterWidth" | "flexGrow">;
	footer?: ReactNode;
	minColumns?: number;
	minRows?: number;
};

export type SurfaceFrameGeometry = {
	frameColumns: number;
	frameRows: number;
	headerRows: number;
	footerRows: number;
	bodyRows: number;
	gutterColumns: number;
	leftOuterWidth: number;
	rightOuterWidth: number;
	leftTextColumns: number;
	leftTextRows: number;
	rightTextColumns: number;
	rightTextRows: number;
};

export type ResolveSurfaceFrameGeometryOptions = {
	columns: number;
	rows: number;
	requestedLeftWidth: number;
	headerHeight: number;
	footerHeight: number;
	hasFooter: boolean;
	hasHeader?: boolean;
	gutterColumns?: number;
	leftTitleRows?: number;
	rightTitleRows?: number;
	layoutGuards?: SurfaceFrameLayoutGuards;
};

export type FrameCanvasProps = {
	frameColumns: number;
	frameRows: number;
	headerRows: number;
	footerRows: number;
	bodyRows: number;
	leftWidth: number;
	rightWidth: number;
	gutterColumns: number;
	showHeader: boolean;
	header?: ReactNode;
	footer?: ReactNode;
	headerBackground?: string;
	footerBackground?: string;
	footerTextColor?: string;
	leftPane: ReactNode;
	rightPane: ReactNode;
};
