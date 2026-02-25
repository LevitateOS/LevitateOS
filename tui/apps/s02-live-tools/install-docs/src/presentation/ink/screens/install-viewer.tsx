import { useApp } from "ink";
import type {
	CommandBlock,
	ContentBlock,
	InlineNode,
	RichText,
} from "@levitate/docs-content";
import { useEffect, useMemo, useRef, useState } from "react";
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
import { resolveDocsLinkTarget } from "../../../domain/navigation/link-target";
import type { DocRenderItem } from "../../../domain/render/types";
import { computeDocsViewport } from "../../../rendering/pipeline/viewport";
import { measureDocItemLines } from "../../../rendering/plan/line-metrics";
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

type ActionableCommand = {
	kind: "command";
	itemKey: string;
	commandText: string;
	startLine: number;
	endLine: number;
};

type ActionableLink = {
	kind: "link";
	itemKey: string;
	href: string;
	label: string;
	startLine: number;
	endLine: number;
};

type ActionableTarget = ActionableCommand | ActionableLink;

function commandBlockText(block: CommandBlock): string {
	return Array.isArray(block.command) ? block.command.join("\n") : block.command;
}

function extractInlineLinks(content: string | RichText | undefined): Array<{ href: string; label: string }> {
	if (!Array.isArray(content)) {
		return [];
	}
	const links: Array<{ href: string; label: string }> = [];
	for (const node of content as InlineNode[]) {
		if (typeof node !== "string" && node.type === "link") {
			const href = node.href.trim();
			if (href.length === 0) {
				continue;
			}
			links.push({ href, label: node.text.trim() || href });
		}
	}
	return links;
}

function extractBlockLinks(block: ContentBlock): Array<{ href: string; label: string }> {
	switch (block.type) {
		case "text":
			return extractInlineLinks(block.content);
		case "table": {
			const links: Array<{ href: string; label: string }> = [];
			for (const header of block.headers) {
				links.push(...extractInlineLinks(header));
			}
			for (const row of block.rows) {
				for (const cell of row) {
					links.push(...extractInlineLinks(cell));
				}
			}
			return links;
		}
		case "list": {
			const links: Array<{ href: string; label: string }> = [];
			for (const item of block.items) {
				if (typeof item === "string" || Array.isArray(item)) {
					links.push(...extractInlineLinks(item));
					continue;
				}
				links.push(...extractInlineLinks(item.text));
				for (const child of item.children ?? []) {
					links.push(...extractInlineLinks(child));
				}
			}
			return links;
		}
		case "conversation": {
			const links: Array<{ href: string; label: string }> = [];
			for (const message of block.messages) {
				links.push(...extractInlineLinks(message.text));
				for (const listItem of message.list ?? []) {
					links.push(...extractInlineLinks(listItem));
				}
			}
			return links;
		}
		case "interactive": {
			const links: Array<{ href: string; label: string }> = [];
			links.push(...extractInlineLinks(block.intro));
			for (const step of block.steps) {
				links.push(...extractInlineLinks(step.description));
			}
			return links;
		}
		case "qa": {
			const links: Array<{ href: string; label: string }> = [];
			for (const item of block.items) {
				links.push(...extractInlineLinks(item.question));
				for (const answerBlock of item.answer) {
					links.push(...extractBlockLinks(answerBlock));
				}
			}
			return links;
		}
		case "note":
			return extractInlineLinks(block.content);
		default:
			return [];
	}
}

function deriveActionableTargets(
	items: ReadonlyArray<DocRenderItem>,
	contentWidth: number,
): ActionableTarget[] {
	const targets: ActionableTarget[] = [];
	let cursor = 0;
	for (const [index, item] of items.entries()) {
		const lines = measureDocItemLines(item, contentWidth);
		const startLine = cursor;
		const endLine = Math.max(cursor, cursor + lines - 1);
		if (item.kind === "block" && item.block.type === "command") {
			targets.push({
				kind: "command",
				itemKey: item.key,
				commandText: commandBlockText(item.block),
				startLine,
				endLine,
			});
		}
		const links =
			item.kind === "intro"
				? extractInlineLinks(item.content)
				: item.kind === "block"
					? extractBlockLinks(item.block)
					: [];
		for (const link of links) {
			targets.push({
				kind: "link",
				itemKey: item.key,
				href: link.href,
				label: link.label,
				startLine,
				endLine,
			});
		}
		cursor += lines;
		if (index < items.length - 1) {
			cursor += 1;
		}
	}
	return targets;
}

function copyToClipboardOsc52(text: string): boolean {
	try {
		const payload = Buffer.from(text, "utf8").toString("base64");
		process.stdout.write(`\u001b]52;c;${payload}\u0007`);
		return true;
	} catch {
		return false;
	}
}

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
	const [actionableIndex, setActionableIndex] = useState(0);
	const [runtimeNote, setRuntimeNote] = useState<string | undefined>(undefined);
	const focus = useFocusPlane({ initial: "navigation" });
	const contentNavLock = useRef<number | null>(null);
	const contentNavAllowedIndex = useRef<number | null>(null);

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
	const actionableTargets = useMemo(
		() => deriveActionableTargets(docsViewport.visibleItems, docsViewport.contentWidth),
		[docsViewport.contentWidth, docsViewport.visibleItems],
	);
	const selectedActionable = actionableTargets[actionableIndex];
	const selectedItemKey = focus.isContent ? selectedActionable?.itemKey : undefined;
	const selectedLinkHref =
		focus.isContent && selectedActionable?.kind === "link" ? selectedActionable.href : undefined;

	useEffect(() => {
		setActionableIndex(0);
		setRuntimeNote(undefined);
	}, [currentItem.slug]);

	useEffect(() => {
		if (focus.isContent) {
			contentNavLock.current = navigation.safeIndex;
			contentNavAllowedIndex.current = null;
			return;
		}
		contentNavLock.current = null;
		contentNavAllowedIndex.current = null;
	}, [focus.isContent, navigation.safeIndex]);

	useEffect(() => {
		const locked = contentNavLock.current;
		if (!focus.isContent || locked === null) {
			return;
		}
		if (navigation.safeIndex === locked) {
			return;
		}
		if (
			contentNavAllowedIndex.current !== null &&
			navigation.safeIndex === contentNavAllowedIndex.current
		) {
			contentNavAllowedIndex.current = null;
			contentNavLock.current = navigation.safeIndex;
			return;
		}
		navigation.setPage(locked);
		setRuntimeNote("blocked navigation mutation while content focus is active");
	}, [focus.isContent, navigation, navigation.safeIndex]);

	useEffect(() => {
		if (actionableTargets.length === 0) {
			if (actionableIndex !== 0) {
				setActionableIndex(0);
			}
			return;
		}
		if (actionableIndex >= actionableTargets.length) {
			setActionableIndex(actionableTargets.length - 1);
		}
	}, [actionableTargets.length, actionableIndex]);

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
	const focusActionable = (index: number) => {
		const selected = actionableTargets[index];
		if (!selected) {
			return;
		}
		const visibleStart = scroll.scrollOffset;
		const visibleEnd = scroll.scrollOffset + contentRows - 1;
		let targetOffset = scroll.scrollOffset;
		if (selected.startLine < visibleStart) {
			targetOffset = selected.startLine;
		} else if (selected.endLine > visibleEnd) {
			targetOffset = selected.endLine - contentRows + 1;
		}
		scroll.scrollBy(targetOffset - scroll.scrollOffset, docsViewport.maxScroll);
	};
	const moveActionable = (delta: number) => {
		if (actionableTargets.length === 0) {
			scrollBy(delta);
			return;
		}
		const next = (actionableIndex + delta + actionableTargets.length) % actionableTargets.length;
		setActionableIndex(next);
		focusActionable(next);
		setRuntimeNote(undefined);
	};
	const copySelectedActionable = () => {
		const selected = actionableTargets[actionableIndex];
		if (!selected) {
			setRuntimeNote("no actionable target on this page");
			return;
		}
		const payload = selected.kind === "command" ? selected.commandText : selected.href;
		const copied = copyToClipboardOsc52(payload);
		const actionLabel =
			selected.kind === "command"
				? `command ${actionableIndex + 1}/${actionableTargets.length}`
				: `link ${actionableIndex + 1}/${actionableTargets.length} (${selected.label})`;
		setRuntimeNote(
			copied ? `copied ${actionLabel}` : "copy failed (terminal clipboard unsupported)",
		);
	};
	const activateSelectedActionable = () => {
		const selected = actionableTargets[actionableIndex];
		if (!selected) {
			setRuntimeNote("no actionable target on this page");
			return;
		}
		if (selected.kind === "command") {
			copySelectedActionable();
			return;
		}
		const resolution = resolveDocsLinkTarget(selected.href, currentItem.slug, navItems);
		if (!resolution.ok) {
			setRuntimeNote(`invalid docs link target: '${selected.href}' (${resolution.reason})`);
			return;
		}
		contentNavAllowedIndex.current = resolution.index;
		navigation.setPage(resolution.index);
		scroll.reset();
		setActionableIndex(0);
		setRuntimeNote(`opened docs page '${resolution.slug}'`);
	};
	const pageJump = Math.max(1, contentRows - 1);

	useHotkeys(["m"], () => {
		setSidebarMode((previous) => (previous === "focus-section" ? "all-sections" : "focus-section"));
	});
	useHotkeys(["left", "h"], () => {
		movePage(-1);
		setRuntimeNote(undefined);
	}, { isActive: focus.isNavigation });
	useHotkeys(["right", "l"], () => {
		movePage(1);
		setRuntimeNote(undefined);
	}, { isActive: focus.isNavigation });
	useHotkeys(["[", "{"], () => {
		moveSection(-1);
	}, { isActive: focus.isNavigation });
	useHotkeys(["]", "}"], () => {
		moveSection(1);
	}, { isActive: focus.isNavigation });
	useHotkeys(["up", "k"], () => {
		moveActionable(-1);
	}, { isActive: focus.isContent });
	useHotkeys(["up", "k"], () => {
		scrollBy(-1);
	}, { isActive: focus.isNavigation });
	useHotkeys(["down", "j"], () => {
		moveActionable(1);
	}, { isActive: focus.isContent });
	useHotkeys(["down", "j"], () => {
		scrollBy(1);
	}, { isActive: focus.isNavigation });
	useHotkeys(["enter"], () => {
		activateSelectedActionable();
	}, { isActive: focus.isContent });
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
		setRuntimeNote(undefined);
	});
	useHotkeys(["G", "end", "S-g"], () => {
		if (!focus.isContent) {
			return;
		}
		scroll.scrollToBottom(docsViewport.maxScroll);
		setRuntimeNote(undefined);
	});
	useHotkeys(["c", "y"], () => {
		copySelectedActionable();
	}, { isActive: focus.isContent });

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
		runtimeNote ?? navigation.startupNote,
		focus.plane,
		actionableTargets.length,
		actionableIndex,
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
				body: (
					<InstallContentPane
						viewport={docsViewport}
						renderers={renderers}
						selectedItemKey={selectedItemKey}
						selectedLinkHref={selectedLinkHref}
					/>
				),
				borderIntent: "cardBorder",
				textIntent: "text",
				titleIntent: "sectionHeading",
			}}
		/>
	);
}
