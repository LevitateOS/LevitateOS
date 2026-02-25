import { useMemo, useState, type ReactNode } from "react";
import { useHotkeys } from "../hooks/use-hotkeys";
import { useTuiTheme, useTuiViewport } from "../app/app-provider";
import { clampNumber } from "../utils/clamp";
import { splitLines } from "../utils/strings";
import { computeScrollWindow, ScrollView } from "../primitives/layout/scroll-view";
import { PageFrame } from "./page-frame";

export type ReviewScreenProps = {
	title: string;
	content: string | ReadonlyArray<string>;
	sidebar?: ReactNode;
	footerText?: string;
	onClose?: () => void;
};

export function ReviewScreen({ title, content, sidebar, footerText, onClose }: ReviewScreenProps) {
	const viewport = useTuiViewport();
	const theme = useTuiTheme();
	const [offset, setOffset] = useState(0);

	const lines = useMemo(() => splitLines(content), [content]);
	const height = Math.max(
		1,
		viewport.rows - theme.layout.headerHeight - theme.layout.footerHeight - 4,
	);
	const windowed = computeScrollWindow(lines, offset, height);

	const scrollBy = (delta: number) => {
		setOffset((value) => clampNumber(value + delta, 0, windowed.maxOffset));
	};

	useHotkeys(["up", "k"], () => scrollBy(-1));
	useHotkeys(["down", "j"], () => scrollBy(1));
	useHotkeys(["pageup", "b"], () => scrollBy(-10));
	useHotkeys(["pagedown", "space"], () => scrollBy(10));
	useHotkeys(["g", "home"], () => setOffset(0));
	useHotkeys(["G", "end", "S-g"], () => setOffset(windowed.maxOffset));
	useHotkeys(["enter", "escape", "q"], () => onClose?.());

	return (
		<PageFrame
			title={title}
			sidebar={sidebar}
			footer={
				footerText ??
				`Arrows/PgUp/PgDn scroll | Enter/q/Esc close | lines ${windowed.start}-${windowed.end}/${lines.length}`
			}
		>
			<ScrollView content={lines} offset={offset} height={height} />
		</PageFrame>
	);
}
