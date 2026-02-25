import { useMemo, useState } from "react";
import { useListNavigation } from "@levitate/tui-kit";
import type { FlatDocsNavItem } from "../../../domain/content/contracts";
import { buildNavSectionSpans, jumpToSectionStart } from "../../../domain/navigation/nav-model";
import { resolveInitialDocSelection } from "../../../rendering/pipeline/viewport";

type InstallNavigationState = {
	safeIndex: number;
	startupNote?: string;
	movePage: (delta: number) => void;
	moveSection: (delta: number) => void;
};

export function useInstallNavigation(
	navItems: ReadonlyArray<FlatDocsNavItem>,
	initialSlug?: string,
): InstallNavigationState {
	const initialSelection = useMemo(
		() => resolveInitialDocSelection(navItems, initialSlug),
		[initialSlug, navItems],
	);
	const sectionSpans = useMemo(() => buildNavSectionSpans(navItems), [navItems]);
	const indexState = useListNavigation(navItems.length, initialSelection.index);

	const [startupNote, setStartupNote] = useState<string | undefined>(
		initialSelection.unknownSlug
			? `unknown slug '${initialSelection.unknownSlug}', showing '${navItems[0]?.slug ?? "index"}'`
			: undefined,
	);

	const safeIndex = indexState.safeIndex;

	const movePage = (delta: number) => {
		indexState.moveBy(delta);
		setStartupNote(undefined);
	};

	const moveSection = (delta: number) => {
		indexState.setIndex(jumpToSectionStart(sectionSpans, indexState.safeIndex, delta));
		setStartupNote(undefined);
	};

	return {
		safeIndex,
		startupNote,
		movePage,
		moveSection,
	};
}
