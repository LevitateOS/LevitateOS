import type { ReactNode } from "react";
import {
	SurfaceFrame,
	resolveSurfaceFrameGeometry,
	type SurfaceFrameLayoutGuards,
	type ResolveSurfaceFrameGeometryOptions,
	type SurfaceFrameProps,
} from "../surfaces/index";

export type TwoPaneProps = {
	title: string;
	sidebar: ReactNode;
	children: ReactNode;
	footer?: ReactNode;
	sidebarWidth?: number;
	showHeader?: boolean;
	sidebarTitle?: ReactNode;
	contentTitle?: ReactNode;
	sidebarBorderStyle?: "single" | "double" | "round" | "bold";
	contentBorderStyle?: "single" | "double" | "round" | "bold";
	minColumns?: number;
	minRows?: number;
	layoutGuards?: SurfaceFrameLayoutGuards;
};

export type TwoPaneGeometry = {
	frameColumns: number;
	frameRows: number;
	headerRows: number;
	footerRows: number;
	bodyRows: number;
	sidebarOuterWidth: number;
	contentOuterWidth: number;
	sidebarTextColumns: number;
	sidebarTextRows: number;
	contentTextColumns: number;
	contentTextRows: number;
};

export type ResolveTwoPaneGeometryOptions = {
	columns: number;
	rows: number;
	requestedSidebarWidth: number;
	headerHeight: number;
	footerHeight: number;
	hasFooter: boolean;
	hasHeader?: boolean;
	sidebarTitleRows?: number;
	contentTitleRows?: number;
	layoutGuards?: SurfaceFrameLayoutGuards;
};

function toSurfaceGeometryOptions(
	options: ResolveTwoPaneGeometryOptions,
): ResolveSurfaceFrameGeometryOptions {
	return {
		columns: options.columns,
		rows: options.rows,
		requestedLeftWidth: options.requestedSidebarWidth,
		headerHeight: options.headerHeight,
		footerHeight: options.footerHeight,
		hasFooter: options.hasFooter,
		hasHeader: options.hasHeader,
		leftTitleRows: options.sidebarTitleRows,
		rightTitleRows: options.contentTitleRows,
		layoutGuards: options.layoutGuards,
	};
}

export function resolveTwoPaneGeometry(options: ResolveTwoPaneGeometryOptions): TwoPaneGeometry {
	const geometry = resolveSurfaceFrameGeometry(toSurfaceGeometryOptions(options));

	return {
		frameColumns: geometry.frameColumns,
		frameRows: geometry.frameRows,
		headerRows: geometry.headerRows,
		footerRows: geometry.footerRows,
		bodyRows: geometry.bodyRows,
		sidebarOuterWidth: geometry.leftOuterWidth,
		contentOuterWidth: geometry.rightOuterWidth,
		sidebarTextColumns: geometry.leftTextColumns,
		sidebarTextRows: geometry.leftTextRows,
		contentTextColumns: geometry.rightTextColumns,
		contentTextRows: geometry.rightTextRows,
	};
}

export function TwoPane({
	title,
	sidebar,
	children,
	footer,
	sidebarWidth,
	showHeader,
	sidebarTitle,
	contentTitle,
	sidebarBorderStyle,
	contentBorderStyle,
	minColumns,
	minRows,
	layoutGuards,
}: TwoPaneProps) {
	const frame: SurfaceFrameProps = {
		title,
		showHeader,
		leftWidth: sidebarWidth,
		layoutGuards,
		minColumns,
		minRows,
		footer,
		leftPane: {
			title: sidebarTitle,
			body: sidebar,
			borderIntent: "sidebarBorder",
			backgroundIntent: "sidebarBackground",
			textIntent: "sidebarItemText",
			titleIntent: "sidebarSectionText",
			borderStyle: sidebarBorderStyle,
		},
		rightPane: {
			title: contentTitle,
			body: children,
			borderIntent: "border",
			backgroundIntent: "contentBackground",
			textIntent: "text",
			titleIntent: "sectionHeading",
			borderStyle: contentBorderStyle,
		},
	};

	return <SurfaceFrame {...frame} />;
}

export type ScreenFrameProps = TwoPaneProps;

export function ScreenFrame(props: ScreenFrameProps) {
	return <TwoPane {...props} />;
}
