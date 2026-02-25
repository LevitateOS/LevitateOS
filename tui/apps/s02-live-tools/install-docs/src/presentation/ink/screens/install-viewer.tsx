import { useApp } from "ink";
import { useState } from "react";
import {
	SurfaceFrame,
	UiText,
	resolveSurfaceFrameGeometry,
	truncateLine,
	useHotkeys,
	useFocusPlane,
	useScrollState,
	useTuiTheme,
	useTuiViewport,
} from "@levitate/tui-kit";
import type { DocsContentLike, FlatDocsNavItem } from "../../../domain/content/contracts";
import { computeDocsViewport } from "../../../rendering/pipeline/viewport";
import { InstallContentPane } from "../components/content-pane";
import { InstallSidebar, type SidebarMode } from "../components/sidebar";
import { installStatusBar } from "../components/status-bar";
import type { DocsRendererRegistry } from "../document/renderer-registry";
import { useInstallNavigation } from "../hooks/use-install-navigation";

type InstallViewerScreenProps = {
	navItems: ReadonlyArray<FlatDocsNavItem>;
	getContent: (slug: string, title: string) => DocsContentLike;
	initialSlug?: string;
	title?: string;
	renderers: DocsRendererRegistry;
	onExit?: () => void;
};

export function InstallViewerScreen({
	navItems,
	getContent,
	initialSlug,
	title = "LevitateOS Field Manual",
	renderers,
	onExit,
}: InstallViewerScreenProps) {
	const { exit } = useApp();
	const theme = useTuiTheme();
	const viewport = useTuiViewport();
	const navigation = useInstallNavigation(navItems, initialSlug);
	const scroll = useScrollState(0);
	const [sidebarMode, setSidebarMode] = useState<SidebarMode>("focus-section");
	const focus = useFocusPlane({ initial: "navigation" });

	const quit = () => {
		onExit?.();
		exit();
	};

	useHotkeys(["q", "escape", "C-c"], quit);
	if (navItems.length === 0) {
		return (
			<SurfaceFrame
				title={title}
				footer="q quit"
				leftWidth={theme.layout.sidebarWidth}
				showHeader={false}
				leftPane={{
					title: "Installation Docs",
					titleMode: "inline",
					body: "(no docs pages)",
					borderIntent: "sidebarBorder",
					textIntent: "sidebarItemText",
					titleIntent: "sidebarSectionText",
				}}
				rightPane={{
					title: "Overview",
					titleMode: "inline",
					body: <UiText>No docs pages are available.</UiText>,
					borderIntent: "cardBorder",
					textIntent: "text",
					titleIntent: "sectionHeading",
				}}
			/>
		);
	}

	const currentItem = navItems[navigation.safeIndex]!;
	const geometry = resolveSurfaceFrameGeometry({
		columns: viewport.columns,
		rows: viewport.rows,
		requestedLeftWidth: theme.layout.sidebarWidth,
		headerHeight: theme.layout.headerHeight,
		footerHeight: theme.layout.footerHeight,
		hasFooter: true,
		hasHeader: false,
		gutterColumns: theme.chrome.framePaneGap,
		leftTitleRows: 2,
		rightTitleRows: 2,
	});
	const contentRows = Math.max(1, geometry.rightTextRows);
	const contentColumns = Math.max(1, geometry.rightTextColumns);
	const sidebarMaxWidth = Math.max(1, geometry.leftTextColumns);

	const content = getContent(currentItem.slug, currentItem.title);
	const docsViewport = computeDocsViewport(
		content,
		currentItem.slug,
		scroll.scrollOffset,
		contentRows,
		contentColumns,
	);

	const movePage = (delta: number) => {
		navigation.movePage(delta);
		scroll.reset();
	};

	const moveSection = (delta: number) => {
		navigation.moveSection(delta);
		scroll.reset();
	};

	const scrollBy = (delta: number) => {
		scroll.scrollBy(delta, docsViewport.maxScroll);
	};
	const pageJump = Math.max(1, contentRows - 1);

	useHotkeys(["m"], () => {
		setSidebarMode((previous) => (previous === "focus-section" ? "all-sections" : "focus-section"));
	});
	useHotkeys(["left", "h"], () => {
		if (!focus.isNavigation) {
			return;
		}
		movePage(-1);
	});
	useHotkeys(["right", "l"], () => {
		if (!focus.isNavigation) {
			return;
		}
		movePage(1);
	});
	useHotkeys(["[", "{"], () => {
		if (!focus.isNavigation) {
			return;
		}
		moveSection(-1);
	});
	useHotkeys(["]", "}"], () => {
		if (!focus.isNavigation) {
			return;
		}
		moveSection(1);
	});
	useHotkeys(["up", "k"], () => {
		scrollBy(-1);
	});
	useHotkeys(["down", "j"], () => {
		scrollBy(1);
	});
	useHotkeys(["enter"], () => {
		if (!focus.isNavigation) {
			return;
		}
		focus.setPlane("content");
	});
	useHotkeys(["pageup", "b"], () => {
		if (!focus.isContent) {
			return;
		}
		scrollBy(-pageJump);
	});
	useHotkeys(["pagedown", "space"], () => {
		if (!focus.isContent) {
			return;
		}
		scrollBy(pageJump);
	});
	useHotkeys(["g", "home"], () => {
		if (!focus.isContent) {
			return;
		}
		scroll.scrollToTop();
	});
	useHotkeys(["G", "end", "S-g"], () => {
		if (!focus.isContent) {
			return;
		}
		scroll.scrollToBottom(docsViewport.maxScroll);
	});

	const sidebar = (
		<InstallSidebar
			items={navItems}
			selectedIndex={navigation.safeIndex}
			maxWidth={sidebarMaxWidth}
			currentSection={currentItem.sectionTitle}
			mode={sidebarMode}
		/>
	);
	const footer = installStatusBar(
		navigation.safeIndex,
		navItems.length,
		navigation.startupNote,
		focus.plane,
	);
	const contentPaneMeta = `lines ${docsViewport.startItem}-${docsViewport.endItem}/${Math.max(docsViewport.totalItems, 1)}`;
	const contentPaneTitle = truncateLine(
		`${currentItem.title}  ${contentPaneMeta}`,
		Math.max(1, geometry.rightTextColumns),
	);

	return (
		<SurfaceFrame
			title={title}
			footer={footer}
			leftWidth={theme.layout.sidebarWidth}
			showHeader={false}
			leftPane={{
				title: "Installation Docs",
				titleMode: "inline",
				body: sidebar,
				borderIntent: "sidebarBorder",
				textIntent: "sidebarItemText",
				titleIntent: "sidebarSectionText",
			}}
			rightPane={{
				title: contentPaneTitle,
				titleMode: "inline",
				body: <InstallContentPane viewport={docsViewport} renderers={renderers} />,
				borderIntent: "cardBorder",
				textIntent: "text",
				titleIntent: "sectionHeading",
			}}
		/>
	);
}
