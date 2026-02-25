import { describe, expect, it } from "bun:test";
import { contentBySlug, docsNav, metaBySlug } from "@levitate/docs-content";
import type { CodeBlock, CommandBlock, DocsContent } from "@levitate/docs-content";
import { createInstallSession } from "./app/session";
import { installDocsCliHelpText } from "./cli/help";
import { parseCliArgs } from "./cli/parse-args";
import { getDistroProfile } from "./domain/distro/registry";
import type { DocRenderItem } from "./domain/render/types";
import {
	buildNavSectionSpans,
	findSectionIndexForPageIndex,
	flattenDocsNav,
	jumpToSectionStart,
} from "./domain/navigation/nav-model";
import { resolveAllowedSlugs } from "./domain/scope/allowed-slugs";
import {
	codeSnapshotLines,
	commandSnapshotLines,
} from "./presentation/ink/blocks/shared/content-utils";
import { syntaxTokenColors } from "./presentation/ink/blocks/shared/syntax-line";
import { inlineContentToPlainText } from "./presentation/ink/blocks/shared/rich-text-renderer";
import { createInstallDocsTheme } from "./presentation/ink/theme";
import { buildDocRenderPlan } from "./rendering/plan/build-doc-plan";
import { countRenderPlanLines } from "./rendering/plan/line-metrics";
import {
	computeDocsViewport,
	resolveInitialDocIndex,
	resolveInitialDocSelection,
} from "./rendering/pipeline/viewport";

function hasStringArray(value: unknown): value is string[] {
	return Array.isArray(value) && value.every((item) => typeof item === "string");
}

function renderItemBlockType(item: DocRenderItem): string | undefined {
	if (item.kind !== "block") {
		return undefined;
	}
	return item.block.type;
}

function collectSyntaxSnapshotIssues(
	slug: string,
	blocks: ReadonlyArray<unknown>,
	issues: string[],
	pathPrefix: string,
): void {
	for (const [index, rawBlock] of blocks.entries()) {
		const path = `${pathPrefix}[${index}]`;
		if (typeof rawBlock !== "object" || rawBlock === null) {
			issues.push(`${slug}:${path} malformed block`);
			continue;
		}
		const block = rawBlock as Record<string, unknown>;
		const blockType = typeof block.type === "string" ? block.type : "(unknown)";

		if (blockType === "code") {
			if (typeof block.language !== "string" || block.language.trim().length === 0) {
				issues.push(`${slug}:${path} missing code language`);
			}
			if (!hasStringArray(block.highlightedLines)) {
				issues.push(`${slug}:${path} missing code highlightedLines`);
			}
		}

		if (blockType === "command") {
			if (typeof block.language !== "string" || block.language.trim().length === 0) {
				issues.push(`${slug}:${path} missing command language`);
			}
			if (!hasStringArray(block.highlightedCommandLines)) {
				issues.push(`${slug}:${path} missing command highlightedCommandLines`);
			}
		}

		if (blockType === "qa" && Array.isArray(block.items)) {
			for (const [itemIndex, item] of block.items.entries()) {
				if (typeof item !== "object" || item === null) {
					issues.push(`${slug}:${path}.items[${itemIndex}] malformed QA item`);
					continue;
				}
				const answer = (item as { answer?: unknown }).answer;
				if (!Array.isArray(answer)) {
					issues.push(`${slug}:${path}.items[${itemIndex}] missing QA answer`);
					continue;
				}
				collectSyntaxSnapshotIssues(slug, answer, issues, `${path}.items[${itemIndex}].answer`);
			}
		}
	}
}

describe("cli parsing", () => {
	it("defaults to interactive mode without flags", () => {
		expect(parseCliArgs([])).toEqual({ help: false, slug: undefined });
	});

	it("accepts --slug and short -s", () => {
		expect(parseCliArgs(["--slug", "installation"])).toEqual({
			help: false,
			slug: "installation",
		});
		expect(parseCliArgs(["--slug=recstrap"])).toEqual({
			help: false,
			slug: "recstrap",
		});
		expect(parseCliArgs(["-s", "recchroot"])).toEqual({
			help: false,
			slug: "recchroot",
		});
	});

	it("rejects removed legacy flags", () => {
		const legacy = parseCliArgs(["--list"]);
		expect(legacy.help).toBe(false);
		expect(legacy.error?.includes("removed")).toBe(true);

		const legacyWithValue = parseCliArgs(["--page=installation"]);
		expect(legacyWithValue.help).toBe(false);
		expect(legacyWithValue.error?.includes("removed")).toBe(true);
	});

	it("rejects malformed slug flag", () => {
		const missing = parseCliArgs(["--slug"]);
		expect(missing.error?.includes("requires")).toBe(true);

		const missingInline = parseCliArgs(["--slug="]);
		expect(missingInline.error?.includes("requires")).toBe(true);
	});

	it("renders install-focused help text", () => {
		const help = installDocsCliHelpText();
		expect(help.includes("LevitateOS Docs TUI")).toBe(true);
		expect(help.includes("levitate-install-docs")).toBe(true);
		expect(help.includes("Legacy non-interactive flags")).toBe(true);
		expect(help.includes("[ / ]")).toBe(true);
		expect(help.includes("Toggle sidebar mode")).toBe(true);
	});
});

describe("docs navigation scope", () => {
	const source = {
		docsNav,
		contentBySlug,
		metaBySlug,
	};

	it("resolves allowed slugs from docs navigation", () => {
		const allowed = resolveAllowedSlugs(source);

		expect(allowed.includes("installation")).toBe(true);
		expect(allowed.includes("recstrap")).toBe(true);
		expect(allowed.includes("recipe-format")).toBe(true);
		expect(allowed.includes("helpers-install")).toBe(true);
	});

	it("rejects unknown slug in session creation", () => {
		const profile = getDistroProfile("levitate");
		expect(() => createInstallSession(source, profile, "missing-page")).toThrow(
			"not available in docs navigation",
		);
	});

	it("creates a valid docs session for known slug", () => {
		const profile = getDistroProfile("levitate");
		const session = createInstallSession(source, profile, "recipe-format");

		expect(session.navItems.length).toBeGreaterThan(0);
		expect(session.initialSlug).toBe("recipe-format");
		expect(session.allowedSlugs.includes("recipe-format")).toBe(true);
		expect(session.navItems.length).toBeGreaterThan(20);
	});

	it("falls back to first nav item when profile default slug is missing", () => {
		const profile = {
			...getDistroProfile("levitate"),
			defaultSlug: "missing-default-slug",
		};
		const session = createInstallSession(source, profile);

		expect(session.navItems.length).toBeGreaterThan(0);
		expect(session.initialSlug).toBe(session.navItems[0]?.slug);
	});
});

describe("rendering and viewport", () => {
	const source = {
		docsNav,
		contentBySlug,
		metaBySlug,
	};
	const profile = getDistroProfile("levitate");
	const session = createInstallSession(source, profile, "installation");
	const navItems = session.navItems;

	it("maps filtered nav into flat slugs", () => {
		expect(navItems.length).toBeGreaterThan(0);
		expect(navItems.every((item) => item.slug.length > 0)).toBe(true);
	});

	it("all visible slugs resolve to docs content", () => {
		const missing = navItems
			.map((item) => item.slug)
			.filter((slug) => contentBySlug[slug] === undefined);
		expect(missing).toEqual([]);
	});

	it("renders every install page through pipeline helpers", () => {
		for (const item of navItems) {
			const content = contentBySlug[item.slug];
			expect(content).toBeDefined();
			if (!content) {
				continue;
			}

			expect(content.sections.length).toBeGreaterThan(0);
			const plan = buildDocRenderPlan(content);
			expect(plan.items.length).toBeGreaterThan(0);
			expect(plan.items.some((renderItem) => renderItem.kind === "section")).toBe(true);
			expect(plan.items.some((renderItem) => renderItem.kind === "block")).toBe(true);
		}
	});

	it("ships syntax snapshot payloads for code and command blocks", () => {
		const issues: string[] = [];

		for (const item of navItems) {
			const content = contentBySlug[item.slug];
			if (!content) {
				continue;
			}

			for (const [sectionIndex, section] of content.sections.entries()) {
				collectSyntaxSnapshotIssues(
					item.slug,
					section.content,
					issues,
					`sections[${sectionIndex}].content`,
				);
			}
		}

		expect(issues).toEqual([]);
	});

	it("builds intro, section heading, and block items in order", () => {
		const content: DocsContent = {
			title: "Ordered page",
			meta: {
				product: "levitate",
				scopes: ["install"],
			},
			intro: "Welcome",
			sections: [
				{
					title: "First",
					content: [
						{
							type: "text",
							content: "alpha",
						},
					],
				},
				{
					title: "Second",
					content: [
						{
							type: "text",
							content: "beta",
						},
					],
				},
			],
		};
		const plan = buildDocRenderPlan(content);
		expect(plan.items.map((item) => item.kind)).toEqual([
			"intro",
			"section",
			"block",
			"section",
			"block",
		]);
	});

	it("tracks structured docs blocks as first-class render items", () => {
		const content: DocsContent = {
			title: "Structured blocks",
			meta: {
				product: "levitate",
				scopes: ["install"],
			},
			sections: [
				{
					title: "Styles",
					content: [
						{
							type: "code",
							language: "bash",
							content: "echo hi",
							highlightedLines: ["echo hi"],
						},
						{
							type: "command",
							language: "bash",
							description: "Run the command",
							command: "echo hi",
							highlightedCommandLines: ["echo hi"],
							output: "hi",
						},
						{
							type: "table",
							headers: ["name", "value"],
							rows: [["a", "1"]],
						},
						{
							type: "note",
							variant: "warning",
							content: "careful",
						},
					],
				},
			],
		};
		const plan = buildDocRenderPlan(content);
		const blockTypes = plan.items.flatMap((item) => {
			const blockType = renderItemBlockType(item);
			return typeof blockType === "string" ? [blockType] : [];
		});
		expect(blockTypes).toEqual(["code", "command", "table", "note"]);
	});

	it("preserves syntax snapshot colors for code and command lines", () => {
		const syntaxLine = "[[fg=#b392f0]]echo[[/]][[fg=#e1e4e8]] hi[[/]]";
		const codeBlock: CodeBlock = {
			type: "code",
			language: "bash",
			content: "echo hi",
			highlightedLines: [syntaxLine],
		};
		const commandBlock: CommandBlock = {
			type: "command",
			language: "bash",
			description: "Run",
			command: "echo hi",
			highlightedCommandLines: [syntaxLine],
			output: "",
		};
		const literalColors = [
			...syntaxTokenColors(codeSnapshotLines(codeBlock)[0] ?? ""),
			...syntaxTokenColors(commandSnapshotLines(commandBlock)[0] ?? ""),
		];

		expect(literalColors.includes("#b392f0")).toBe(true);
		expect(literalColors.includes("#e1e4e8")).toBe(true);
	});

	it("preserves syntax snapshot colors for longer command/content lines", () => {
		const syntaxLine = "[[fg=#b392f0]]echo[[/]][[fg=#e1e4e8]] hithere[[/]]";
		const commandBlock: CommandBlock = {
			type: "command",
			language: "bash",
			description: "Run",
			command: "echo hithere",
			highlightedCommandLines: [syntaxLine],
			output: "",
		};
		const codeLiteralColors = syntaxTokenColors(syntaxLine);
		const commandLiteralColors = syntaxTokenColors(commandSnapshotLines(commandBlock)[0] ?? "");

		expect(codeLiteralColors.includes("#b392f0")).toBe(true);
		expect(codeLiteralColors.includes("#e1e4e8")).toBe(true);
		expect(commandLiteralColors.includes("#b392f0")).toBe(true);
		expect(commandLiteralColors.includes("#e1e4e8")).toBe(true);
	});

	it("preserves link destinations in rendered inline content", () => {
		const text = inlineContentToPlainText([
			"See ",
			{
				type: "link",
				text: "guide",
				href: "https://example.com/guide",
			},
		]);
		expect(text.includes("guide (https://example.com/guide)")).toBe(true);
	});

	it("wraps long command lines without dropping content", () => {
		const commandBlock: CommandBlock = {
			type: "command",
			language: "bash",
			description: "Run this:",
			command: "recstrap /mnt --variant full --with networking --with docs --with debug",
			highlightedCommandLines: [
				"recstrap /mnt --variant full --with networking --with docs --with debug",
			],
			output: "",
		};
		const commandText = commandSnapshotLines(commandBlock).join("\n");
		expect(commandText.includes("--with debug")).toBe(true);
	});

	it("initial selection helpers work", () => {
		const idx = resolveInitialDocIndex(navItems, navItems[1]?.slug);
		expect(idx).toBe(1);

		const selection = resolveInitialDocSelection(navItems, "missing-slug");
		expect(selection.index).toBe(0);
		expect(selection.unknownSlug).toBe("missing-slug");
	});

	it("clamps oversized scroll while preserving viewport line ranges", () => {
		const sections = Array.from({ length: 16 }, (_, index) => ({
			title: `Section ${index + 1}`,
			content: [{ type: "text" as const, content: `alpha ${index}` }],
		}));
		const viewport = computeDocsViewport(
			{
				title: "Deep page",
				meta: {
					product: "levitate",
					scopes: ["install"],
				},
				intro: "Intro",
				sections,
			},
			"deep-page",
			Number.MAX_SAFE_INTEGER,
			12,
			40,
		);

		expect(viewport.maxScroll).toBeGreaterThan(0);
		expect(viewport.scrollOffset).toBe(viewport.maxScroll);
		expect(viewport.startItem).toBeLessThanOrEqual(viewport.endItem);
		expect(viewport.endItem).toBeLessThanOrEqual(viewport.totalItems);
		expect(viewport.visibleItems.length).toBeGreaterThan(0);
	});

	it("allows scrolling long single-block pages by rendered line count", () => {
		const viewport = computeDocsViewport(
			{
				title: "Single long block",
				meta: {
					product: "levitate",
					scopes: ["install"],
				},
				sections: [
					{
						title: "Long text",
						content: [{ type: "text", content: "alpha ".repeat(300) }],
					},
				],
			},
			"single-long-block",
			Number.MAX_SAFE_INTEGER,
			8,
			24,
		);

		expect(viewport.maxScroll).toBeGreaterThan(0);
		expect(viewport.scrollOffset).toBe(viewport.maxScroll);
		expect(viewport.visibleItems.length).toBe(2);
		expect(viewport.startItem).toBeGreaterThan(1);
		expect(viewport.endItem).toBe(viewport.totalItems);
	});

	it("keeps scroll parity for mixed rich-text blocks", () => {
		const content: DocsContent = {
			title: "Mixed rich text parity",
			meta: {
				product: "levitate",
				scopes: ["install"],
			},
			intro: [
				"Use ",
				{ type: "link", text: "Getting Started", href: "/docs/getting-started" },
				" before installation.",
			],
			sections: [
				{
					title: "Mixed",
					content: [
						{
							type: "text",
							content:
								"This paragraph is intentionally long so wrapped content spans multiple lines and stresses viewport math.",
						},
						{
							type: "list",
							items: [
								[
									"First item with ",
									{ type: "code", text: "recpart" },
									" and a ",
									{ type: "link", text: "reference", href: "/docs/cli-reference" },
									".",
								],
								"Second item with enough width pressure to wrap repeatedly across narrow viewports.",
							],
						},
						{
							type: "note",
							variant: "warning",
							content:
								"Labels must match exactly. Slot A boots from system-a and Slot B boots from system-b.",
						},
						{
							type: "command",
							language: "bash",
							description: "Run and verify output.",
							command: "echo alpha && echo beta",
							highlightedCommandLines: ["echo alpha && echo beta"],
							output: "alpha\nbeta",
						},
					],
				},
			],
		};

		const width = 54;
		const visibleRows = 9;
		const plan = buildDocRenderPlan(content);
		const expectedTotalLines = countRenderPlanLines(plan.items, width);
		const top = computeDocsViewport(content, "mixed", 0, visibleRows, width);
		expect(top.totalItems).toBe(expectedTotalLines);
		expect(top.maxScroll).toBe(Math.max(0, expectedTotalLines - visibleRows));
		expect(top.maxScroll).toBeGreaterThan(0);
		expect(top.startItem).toBe(1);
		expect(top.endItem).toBe(Math.min(expectedTotalLines, visibleRows));

		const mid = computeDocsViewport(content, "mixed", 3, visibleRows, width);
		expect(mid.startItem).toBe(4);
		expect(mid.endItem).toBe(Math.min(expectedTotalLines, 12));

		const bottom = computeDocsViewport(
			content,
			"mixed",
			Number.MAX_SAFE_INTEGER,
			visibleRows,
			width,
		);
		expect(bottom.scrollOffset).toBe(top.maxScroll);
		expect(bottom.startItem).toBe(top.maxScroll + 1);
		expect(bottom.endItem).toBe(expectedTotalLines);
	});
});

describe("theme", () => {
	it("uses install-docs palette and layout", () => {
		const theme = createInstallDocsTheme();
		expect(theme.layout.sidebarWidth).toBe(32);
		expect(theme.layout.minColumns).toBe(88);
		expect(theme.layout.headerHeight).toBe(1);
		expect(theme.colors.text.truecolor).toBe("#eceaf4");
		expect(theme.colors.accent.truecolor).toBe("#7db3ff");
		expect(theme.colors.warning.truecolor).toBe("#ffc76a");
		expect(theme.chrome.titleStyle).toBe("slot");
		expect(theme.chrome.framePaneGap).toBe(0);
		expect(theme.chrome.sidebarHeaderMode).toBe("current-section-title");
	});
});

describe("flatten helper", () => {
	it("keeps slugs only for docs hrefs", () => {
		const flat = flattenDocsNav(docsNav);
		expect(flat.length).toBeGreaterThan(0);
		expect(flat[0]?.slug.length).toBeGreaterThan(0);
	});

	it("builds section spans and jumps by section boundaries", () => {
		const flat = flattenDocsNav(docsNav);
		const spans = buildNavSectionSpans(flat);

		expect(spans.length).toBeGreaterThan(1);
		expect(spans[0]?.startIndex).toBe(0);
		expect(spans[0]?.endIndex).toBeGreaterThanOrEqual(spans[0]?.startIndex ?? 0);

		const sectionIdx = findSectionIndexForPageIndex(spans, spans[0]?.endIndex ?? 0);
		expect(sectionIdx).toBe(0);

		const nextStart = jumpToSectionStart(spans, spans[0]?.endIndex ?? 0, 1);
		expect(nextStart).toBe(spans[1]?.startIndex ?? 0);
	});
});
