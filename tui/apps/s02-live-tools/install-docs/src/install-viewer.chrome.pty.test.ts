import { describe, expect, it } from "bun:test";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { spawnSync } from "node:child_process";

type PanelSpan = {
	lineIndex: number;
	left: number;
	right: number;
};

type PanelTarget = "left" | "right";

type SegmentInspection = {
	segmentIndex: number;
	ok: boolean;
	reason?: string;
	span?: PanelSpan;
	titleIndex?: number;
	seamIndex?: number;
	bottomIndex?: number;
	topSegment?: string;
	seamSegment?: string;
	bottomSegment?: string;
};

const TITLE_PATTERN = /[A-Za-z]/;
const TOP_LEFT_GLYPHS = new Set(["┌", "╭", "╔", "┏"]);
const TOP_RIGHT_GLYPHS = new Set(["┐", "╮", "╗", "┓"]);
const BOTTOM_LEFT_GLYPHS = new Set(["└", "╰", "╚", "┗"]);
const BOTTOM_RIGHT_GLYPHS = new Set(["┘", "╯", "╝", "┛"]);
const HORIZONTAL_GLYPHS = new Set(["─", "━", "═"]);
const ANSI_ESCAPE_PATTERN = new RegExp(`${String.fromCharCode(27)}\\[[0-9;?]*[A-Za-z]`, "g");

function runPtyCapture(columns: number, rows: number): string {
	const tempDir = mkdtempSync(join(tmpdir(), "docs-tui-chrome-pty-"));
	const transcriptPath = join(tempDir, `install-${columns}x${rows}.log`);
	const command = `export TERM=xterm-256color; stty rows ${rows} cols ${columns}; timeout 2 bun src/main.ts --slug installation`;
	return runScriptCapture(transcriptPath, command);
}

function runPtyCaptureSlug(columns: number, rows: number, slug: string): string {
	const tempDir = mkdtempSync(join(tmpdir(), "docs-tui-chrome-pty-"));
	const transcriptPath = join(tempDir, `${slug}-${columns}x${rows}.log`);
	const command = `export TERM=xterm-256color; stty rows ${rows} cols ${columns}; timeout 2 bun src/main.ts --slug ${slug}`;
	return runScriptCapture(transcriptPath, command);
}

function runScriptCapture(transcriptPath: string, command: string): string {
	const result = spawnSync("script", ["-q", "-c", command, transcriptPath], {
		cwd: process.cwd(),
		env: { ...process.env, TERM: "xterm-256color" },
		encoding: "utf8",
	});

	try {
		if (result.status !== 0) {
			throw new Error(
				[
					`script capture failed with exit ${result.status ?? "unknown"}.`,
					result.stdout ?? "",
					result.stderr ?? "",
				]
					.join("\n")
					.trim(),
			);
		}
		return readFileSync(transcriptPath, "utf8");
	} finally {
		rmSync(dirname(transcriptPath), { recursive: true, force: true });
	}
}

type FenceInspection = {
	ok: boolean;
	reason?: string;
};

function inspectCodeFenceShape(lines: string[], panelSpan: PanelSpan): FenceInspection {
	let panelBottomIndex = lines.length;
	for (let index = panelSpan.lineIndex + 1; index < lines.length; index += 1) {
		const line = lines[index] ?? "";
		if (line.length <= panelSpan.right) {
			continue;
		}
		if ((line[panelSpan.left] ?? "") === "└" && (line[panelSpan.right] ?? "") === "┘") {
			panelBottomIndex = index;
			break;
		}
	}

	const candidates: Array<{ topIndex: number; left: number; right: number }> = [];
	for (let index = panelSpan.lineIndex + 1; index < lines.length; index += 1) {
		const line = lines[index] ?? "";
		if (line.length <= panelSpan.right) {
			continue;
		}
		const panelSegment = line.slice(panelSpan.left, panelSpan.right + 1);
		const topMatch = /┌\s+[A-Z][A-Z0-9_. -]*\s+/.exec(panelSegment);
		if (!topMatch) {
			continue;
		}
		const leftLocal = panelSegment.indexOf("┌");
		const rightLocal = panelSegment.lastIndexOf("┐");
		if (leftLocal < 0 || rightLocal <= leftLocal) {
			continue;
		}
		candidates.push({
			topIndex: index,
			left: panelSpan.left + leftLocal,
			right: panelSpan.left + rightLocal,
		});
	}

	if (candidates.length === 0) {
		return { ok: false, reason: "missing code-fence top border" };
	}

	let verifiedFences = 0;
	for (const fence of candidates) {
		let bottomIndex = -1;
		for (let index = fence.topIndex + 1; index < panelBottomIndex; index += 1) {
			const line = lines[index] ?? "";
			if (line.length <= fence.right) {
				continue;
			}

			const leftChar = line[fence.left] ?? "";
			const rightChar = line[fence.right] ?? "";
			if (leftChar === "└" && rightChar === "┘") {
				bottomIndex = index;
				break;
			}
			if (leftChar !== "│" || rightChar !== "│") {
				return {
					ok: false,
					reason: `code-fence side borders misaligned at line ${index}`,
				};
			}
		}
		if (bottomIndex < 0) {
			continue;
		}
		verifiedFences += 1;
	}

	if (verifiedFences === 0) {
		return { ok: false, reason: "missing complete code-fence box in viewport" };
	}

	return { ok: true };
}

function stripAnsi(text: string): string {
	return text.replace(ANSI_ESCAPE_PATTERN, "");
}

function splitSegments(transcript: string): string[][] {
	return transcript.split("\u001b[H").map((segment) =>
		stripAnsi(segment)
			.split(/\n/)
			.map((line) => line.replace(/\r/g, ""))
			.filter((line) => line.length > 0),
	);
}

function nextNonEmptyLine(lines: string[], fromIndexExclusive: number): number {
	for (let index = fromIndexExclusive + 1; index < lines.length; index += 1) {
		if ((lines[index] ?? "").trim().length > 0) {
			return index;
		}
	}
	return -1;
}

function isHorizontalRun(segment: string, leftGlyph: string, rightGlyph: string): boolean {
	if (!segment.startsWith(leftGlyph) || !segment.endsWith(rightGlyph) || segment.length < 3) {
		return false;
	}
	const inner = segment.slice(1, -1);
	const glyph = inner[0] ?? "";
	return HORIZONTAL_GLYPHS.has(glyph) && inner === glyph.repeat(inner.length);
}

function findTopBorderRuns(line: string): Array<{ left: number; right: number }> {
	const runs: Array<{ left: number; right: number }> = [];
	for (let index = 0; index < line.length; index += 1) {
		if (!TOP_LEFT_GLYPHS.has(line[index] ?? "")) {
			continue;
		}

		let cursor = index + 1;
		while (cursor < line.length && HORIZONTAL_GLYPHS.has(line[cursor] ?? "")) {
			cursor += 1;
		}

		if (cursor > index + 1 && TOP_RIGHT_GLYPHS.has(line[cursor] ?? "")) {
			runs.push({ left: index, right: cursor });
			index = cursor;
		}
	}
	return runs;
}

function findPanelTopSpan(lines: string[], target: PanelTarget): PanelSpan | null {
	for (let lineIndex = 0; lineIndex < lines.length; lineIndex += 1) {
		const runs = findTopBorderRuns(lines[lineIndex] ?? "");
		if (target === "left" && runs.length >= 1) {
			const leftRun = runs[0]!;
			return { lineIndex, left: leftRun.left, right: leftRun.right };
		}
		if (target === "right" && runs.length >= 2) {
			const rightRun = runs[runs.length - 1]!;
			return { lineIndex, left: rightRun.left, right: rightRun.right };
		}
	}
	return null;
}

function inspectSegment(
	lines: string[],
	segmentIndex: number,
	target: PanelTarget,
	titlePattern: RegExp,
): SegmentInspection {
	const span = findPanelTopSpan(lines, target);
	if (!span) {
		return {
			segmentIndex,
			ok: false,
			reason: `missing ${target}-panel top border span`,
		};
	}

	const structuralTitleIndex = span.lineIndex + 1;
	const structuralTitleLine = lines[structuralTitleIndex] ?? "";
	const titleIndex =
		structuralTitleLine.length > span.right &&
		(structuralTitleLine[span.left] ?? "") === "│" &&
		(structuralTitleLine[span.right] ?? "") === "│"
			? structuralTitleIndex
			: lines.findIndex((line) => titlePattern.test(line));
	if (titleIndex < 0) {
		return {
			segmentIndex,
			ok: false,
			reason: `missing ${target}-panel title`,
			span,
		};
	}

	const seamIndex = nextNonEmptyLine(lines, titleIndex);
	if (seamIndex < 0) {
		return { segmentIndex, ok: false, reason: "missing seam line under title", span, titleIndex };
	}

	const topLine = lines[span.lineIndex] ?? "";
	const seamLine = lines[seamIndex] ?? "";
	if (topLine.length <= span.right || seamLine.length <= span.right) {
		return {
			segmentIndex,
			ok: false,
			reason: "chrome line shorter than panel span",
			span,
			titleIndex,
			seamIndex,
		};
	}

	const topSegment = topLine.slice(span.left, span.right + 1);
	const seamSegment = seamLine.slice(span.left, span.right + 1);

	let bottomIndex = -1;
	let bottomSegment = "";
	for (let index = seamIndex + 1; index < lines.length; index += 1) {
		const candidate = lines[index] ?? "";
		if (candidate.length <= span.right) {
			continue;
		}
		const segment = candidate.slice(span.left, span.right + 1);
		if (
			BOTTOM_LEFT_GLYPHS.has(segment[0] ?? "") &&
			BOTTOM_RIGHT_GLYPHS.has(segment[segment.length - 1] ?? "")
		) {
			bottomIndex = index;
			bottomSegment = segment;
			break;
		}
	}

	if (bottomIndex < 0) {
		return {
			segmentIndex,
			ok: false,
			reason: "missing bottom border segment",
			span,
			titleIndex,
			seamIndex,
			topSegment,
			seamSegment,
		};
	}

	const hasAsciiHyphen =
		topSegment.includes("-") || seamSegment.includes("-") || bottomSegment.includes("-");
	const topValid = isHorizontalRun(topSegment, "┌", "┐");
	const seamValid = isHorizontalRun(seamSegment, "├", "┤");
	const bottomValid = isHorizontalRun(bottomSegment, "└", "┘");
	const ok = !hasAsciiHyphen && topValid && seamValid && bottomValid;

	return {
		segmentIndex,
		ok,
		reason: ok ? undefined : "invalid chrome glyph semantics",
		span,
		titleIndex,
		seamIndex,
		bottomIndex,
		topSegment,
		seamSegment,
		bottomSegment,
	};
}

describe("install viewer right-panel chrome", () => {
	it("draws seam semantics with box-drawing glyphs in segments 0 and 1", () => {
		for (const columns of [96, 120]) {
			const segments = splitSegments(runPtyCapture(columns, 32));
			expect(segments.length).toBeGreaterThanOrEqual(2);

			for (const segmentIndex of [0, 1]) {
				const inspection = inspectSegment(
					segments[segmentIndex] ?? [],
					segmentIndex,
					"right",
					TITLE_PATTERN,
				);
				if (!inspection.ok) {
					throw new Error(
						`columns=${columns} segment=${segmentIndex} failed: ${inspection.reason ?? "unknown"}; span=[${inspection.span?.left ?? -1},${inspection.span?.right ?? -1}] title=${inspection.titleIndex ?? -1} seam=${inspection.seamIndex ?? -1} bottom=${inspection.bottomIndex ?? -1} top='${inspection.topSegment ?? ""}' seam='${inspection.seamSegment ?? ""}' bottom='${inspection.bottomSegment ?? ""}'`,
					);
				}
			}
		}
	});
});

describe("install viewer sidebar chrome", () => {
	it("renders Installation Docs separator as a structural seam in segments 0 and 1", () => {
		for (const columns of [96, 120]) {
			const segments = splitSegments(runPtyCapture(columns, 32));
			expect(segments.length).toBeGreaterThanOrEqual(2);

			for (const segmentIndex of [0, 1]) {
				const inspection = inspectSegment(
					segments[segmentIndex] ?? [],
					segmentIndex,
					"left",
					/\bInstallation Docs\b/,
				);
				if (!inspection.ok) {
					throw new Error(
						`columns=${columns} segment=${segmentIndex} failed: ${inspection.reason ?? "unknown"}; span=[${inspection.span?.left ?? -1},${inspection.span?.right ?? -1}] title=${inspection.titleIndex ?? -1} seam=${inspection.seamIndex ?? -1} bottom=${inspection.bottomIndex ?? -1} top='${inspection.topSegment ?? ""}' seam='${inspection.seamSegment ?? ""}' bottom='${inspection.bottomSegment ?? ""}'`,
					);
				}
			}
		}
	});
});

describe("helpers code-fence geometry", () => {
	it("keeps side borders aligned across wrapped and blank rows", () => {
		for (const columns of [96, 120]) {
			const segments = splitSegments(runPtyCaptureSlug(columns, 36, "helpers-commands"));
			expect(segments.length).toBeGreaterThanOrEqual(2);

			for (const segmentIndex of [0, 1]) {
				const lines = segments[segmentIndex] ?? [];
				const panelSpan = findPanelTopSpan(lines, "right");
				if (!panelSpan) {
					throw new Error(`columns=${columns} segment=${segmentIndex} missing right panel span`);
				}
				const inspection = inspectCodeFenceShape(lines, panelSpan);
				if (!inspection.ok) {
					throw new Error(
						`columns=${columns} segment=${segmentIndex} failed: ${inspection.reason ?? "unknown"}`,
					);
				}
			}
		}
	});
});
