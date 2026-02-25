import { useCallback, useMemo, useState } from "react";
import { useHotkeys } from "./use-hotkeys";

export type FocusPlane = "navigation" | "content";

export type FocusPlaneOptions = {
	initial?: FocusPlane;
	bindTabKeys?: boolean;
};

export type FocusPlaneState = {
	plane: FocusPlane;
	isNavigation: boolean;
	isContent: boolean;
	setPlane: (next: FocusPlane) => void;
	togglePlane: () => void;
};

export function toggleFocusPlane(plane: FocusPlane): FocusPlane {
	return plane === "navigation" ? "content" : "navigation";
}

export function useFocusPlane(options: FocusPlaneOptions = {}): FocusPlaneState {
	const initial = options.initial ?? "content";
	const [plane, setPlane] = useState<FocusPlane>(initial);

	const togglePlane = useCallback(() => {
		setPlane((current) => toggleFocusPlane(current));
	}, []);

	useHotkeys(options.bindTabKeys === false ? undefined : ["tab", "S-tab"], togglePlane);

	return useMemo(
		() => ({
			plane,
			isNavigation: plane === "navigation",
			isContent: plane === "content",
			setPlane,
			togglePlane,
		}),
		[plane, togglePlane],
	);
}
