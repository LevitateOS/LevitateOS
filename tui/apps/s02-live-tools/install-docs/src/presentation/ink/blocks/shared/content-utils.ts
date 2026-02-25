import type { CodeBlock, CommandBlock } from "@levitate/docs-content";

function syntaxRenderError(blockType: "code" | "command", detail: string): never {
	throw new Error(
		`docs.render ${blockType} block: ${detail}. Remediation: run 'bun run build' in docs/content to regenerate syntax snapshots.`,
	);
}

export function codeSnapshotLines(block: CodeBlock): string[] {
	if (typeof block.language !== "string" || block.language.trim().length === 0) {
		syntaxRenderError("code", "missing required language");
	}
	if (!Array.isArray(block.highlightedLines)) {
		syntaxRenderError("code", "missing highlightedLines snapshot payload");
	}
	if (!block.highlightedLines.every((line) => typeof line === "string")) {
		syntaxRenderError("code", "highlightedLines contains non-string entries");
	}
	return block.highlightedLines;
}

export function commandSnapshotLines(block: CommandBlock): string[] {
	if (typeof block.language !== "string" || block.language.trim().length === 0) {
		syntaxRenderError("command", "missing required language");
	}
	if (!Array.isArray(block.highlightedCommandLines)) {
		syntaxRenderError("command", "missing highlightedCommandLines snapshot payload");
	}
	if (!block.highlightedCommandLines.every((line) => typeof line === "string")) {
		syntaxRenderError("command", "highlightedCommandLines contains non-string entries");
	}
	return block.highlightedCommandLines;
}
