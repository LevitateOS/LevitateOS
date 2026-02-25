import { describe, expect, it } from "bun:test";
import {
	flattenStatusSegments,
	formatPagedStatusLine,
	hotkeyStatusSegment,
	scopeStatusSegment,
	textStatusSegment,
} from "../../src/components/feedback/status-line";

describe("status line formatting", () => {
	it("formats keymap, page position, and line window", () => {
		const rendered = formatPagedStatusLine({
			currentIndex: 1,
			itemCount: 4,
			startLine: 10,
			endLine: 24,
			totalLines: 120,
		});

		expect(rendered).toContain("q quit | h/l page | j/k scroll | g/G top/end");
		expect(rendered).toContain("page 2/4");
		expect(rendered).toContain("lines 10-24/120");
	});

	it("clamps invalid values and appends trimmed note", () => {
		const rendered = formatPagedStatusLine({
			currentIndex: -5,
			itemCount: 0,
			startLine: -1,
			endLine: 999,
			totalLines: 0,
			note: "  hello  ",
			keymap: " keys ",
		});

		expect(rendered).toContain("keys");
		expect(rendered).toContain("page 1/1");
		expect(rendered).toContain("lines 0-1/1");
		expect(rendered).toContain("| hello");
	});

	it("flattens segmented status rows with separators", () => {
		const atoms = flattenStatusSegments(
			[
				scopeStatusSegment("docs"),
				hotkeyStatusSegment("quit", "q", "quit"),
				textStatusSegment("page", "page 1/3"),
			],
			{
				separator: " • ",
			},
		);
		const text = atoms.map((atom) => atom.text).join("");

		expect(text).toContain("[docs]");
		expect(text).toContain("q quit");
		expect(text).toContain("•");
		expect(text).toContain("page 1/3");
	});
});
