import type { ReactNode } from "react";
import { TwoPane, type TwoPaneProps } from "./two-pane";

export type PageFrameProps = {
	title: string;
	sidebar?: ReactNode;
	footer?: ReactNode;
	sidebarWidth?: number;
	children: ReactNode;
};

export function PageFrame({ title, sidebar, footer, sidebarWidth, children }: PageFrameProps) {
	const frame: TwoPaneProps = {
		title,
		sidebar: sidebar ?? "tui-kit",
		footer,
		sidebarWidth,
		children,
	};

	return <TwoPane {...frame} />;
}
