import {
	SegmentedStatusLine,
	hotkeyStatusSegment,
	scopeStatusSegment,
	textStatusSegment,
	type StatusSegment,
	type FocusPlane,
} from "@levitate/tui-kit";
import type { ReactNode } from "react";

export function installStatusBar(
	currentIndex: number,
	pageCount: number,
	note?: string,
	focus?: FocusPlane,
): ReactNode {
	const safePageCount = Math.max(1, pageCount);
	const safeCurrentPage = Math.max(1, Math.min(safePageCount, currentIndex + 1));
	const safeFocus = focus ?? "navigation";

	const segments: StatusSegment[] = [
		scopeStatusSegment("s02-install-docs"),
		textStatusSegment("focus", safeFocus, safeFocus === "navigation" ? "warning" : "accent"),
		hotkeyStatusSegment("quit", "q", "quit"),
		hotkeyStatusSegment("focus", "tab", "toggle pane"),
		hotkeyStatusSegment("mode", "m", "sidebar mode"),
		textStatusSegment("page", `page ${safeCurrentPage}/${safePageCount}`),
	];
	if (safeFocus === "navigation") {
		segments.push(hotkeyStatusSegment("nav", "h/l [/]", "pages/sections"));
		segments.push(hotkeyStatusSegment("scroll", "j/k", "content"));
		segments.push(hotkeyStatusSegment("open", "enter", "content"));
	} else {
		segments.push(hotkeyStatusSegment("scroll", "j/k PgUp/PgDn", "content"));
		segments.push(hotkeyStatusSegment("jump", "g/G b/space", "top/end/page"));
	}

	if (typeof note === "string" && note.trim().length > 0) {
		segments.push(textStatusSegment("note", note.trim(), "warning"));
	}

	return <SegmentedStatusLine segments={segments} />;
}
