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
	actionableCount = 0,
	actionableIndex = 0,
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
		segments.push(hotkeyStatusSegment("open", "tab", "content"));
	} else {
		segments.push(hotkeyStatusSegment("actions", "j/k", "next/prev"));
		segments.push(hotkeyStatusSegment("copy", "enter/c/y", "command"));
		segments.push(hotkeyStatusSegment("scroll", "PgUp/PgDn", "content"));
		segments.push(hotkeyStatusSegment("jump", "g/G b/space", "top/end/page"));
		segments.push(
			textStatusSegment(
				"action",
				actionableCount > 0
					? `${Math.max(1, Math.min(actionableCount, actionableIndex + 1))}/${actionableCount}`
					: "none",
				actionableCount > 0 ? "accent" : "dimText",
			),
		);
	}

	if (typeof note === "string" && note.trim().length > 0) {
		segments.push(textStatusSegment("note", note.trim(), "warning"));
	}

	return <SegmentedStatusLine segments={segments} />;
}
