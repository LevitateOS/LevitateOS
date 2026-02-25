import { describe, expect, it } from "bun:test";
import type { RichText } from "@levitate/docs-content";
import { wrapBoundedText } from "@levitate/tui-kit";
import {
	inlineContentToPlainText,
	richTextRuns,
	wrapRichTextPlainLines,
} from "./rich-text-renderer";

describe("rich text renderer helpers", () => {
	it("matches bounded text wrapping for plain projections", () => {
		const content: RichText = [
			"Boot from ",
			{ type: "link", text: "Getting Started", href: "/docs/getting-started" },
			" and run ",
			{ type: "code", text: "lsblk -f" },
			" before ",
			{ type: "bold", text: "partitioning" },
			".",
		];

		const width = 28;
		const actual = wrapRichTextPlainLines(content, width, "text", 1);
		const expected = wrapBoundedText(inlineContentToPlainText(content), width, 1);
		expect(actual).toEqual(expected);
	});

	it("preserves explicit blank lines during wrapping", () => {
		const content = "alpha\n\nbeta";
		expect(wrapRichTextPlainLines(content, 20, "text", 1)).toEqual(["alpha", "", "beta"]);
	});

	it("maps rich inline nodes to styled runs", () => {
		const runs = richTextRuns(
			[
				"See ",
				{ type: "link", text: "Installation", href: "/docs/installation" },
				", use ",
				{ type: "code", text: "recpart" },
				", and ",
				{ type: "italic", text: "verify" },
				" ",
				{ type: "bold", text: "labels" },
				".",
			],
			"text",
		);

		expect(runs.some((run) => run.underline === true && run.intent === "accent")).toBeTrue();
		expect(runs.some((run) => run.intent === "warning" && run.text.includes("recpart"))).toBeTrue();
		expect(runs.some((run) => run.italic === true && run.text.includes("verify"))).toBeTrue();
		expect(runs.some((run) => run.bold === true && run.text.includes("labels"))).toBeTrue();
	});
});
