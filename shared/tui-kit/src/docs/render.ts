import { horizontalRule, styleText, truncateLine, wrapText } from "../theme/styles";
import { inlineToPlain, type RichTextLike } from "./inline";
import type { FlatDocsNavItem } from "./nav";
import { clampNumber, toNonNegativeInt, toPositiveInt } from "../internal/numbers";
import type { ColorIntent, ColorRuntime, TuiTheme } from "../theme";

export type DocsRenderStyleContext = {
  theme: TuiTheme;
  colors: ColorRuntime;
};

export type DocsContentLike = {
  title: string;
  intro?: string | RichTextLike;
  sections: ReadonlyArray<DocsSectionLike>;
};

export type DocsSectionLike = {
  title: string;
  level?: 2 | 3;
  content: ReadonlyArray<DocsBlockLike>;
};

export type DocsBlockLike = {
  type: string;
  language?: unknown;
  content?: unknown;
  filename?: unknown;
  highlightedLines?: unknown;
  description?: unknown;
  command?: unknown;
  highlightedCommandLines?: unknown;
  output?: unknown;
  ordered?: unknown;
  items?: unknown;
  headers?: unknown;
  rows?: unknown;
  intro?: unknown;
  steps?: unknown;
  messages?: unknown;
  variant?: unknown;
  question?: unknown;
  answer?: unknown;
  role?: unknown;
  text?: unknown;
  list?: unknown;
  children?: unknown;
};

type RenderSidebarOptions = {
  maxWidth?: number;
  styleContext?: DocsRenderStyleContext;
};

type RenderHeaderOptions = {
  styleContext?: DocsRenderStyleContext;
};

type RenderPageOptions = {
  styleContext?: DocsRenderStyleContext;
};

function normalizeWidth(width: number): number {
  return Math.max(20, toPositiveInt(width, 20));
}

function syntaxRenderError(blockType: "code" | "command", detail: string): never {
  throw new Error(
    `docs.render ${blockType} block: ${detail}. Remediation: run 'bun run build' in docs/content to regenerate syntax snapshots.`,
  );
}

function resolveBlockLanguage(block: DocsBlockLike, blockType: "code" | "command"): string {
  if (typeof block.language !== "string" || block.language.trim().length === 0) {
    syntaxRenderError(blockType, "missing required language");
  }

  return block.language.trim();
}

function resolveHighlightedLines(
  block: DocsBlockLike,
  blockType: "code" | "command",
  field: "highlightedLines" | "highlightedCommandLines",
): string[] {
  const payload = block[field];
  if (!Array.isArray(payload)) {
    syntaxRenderError(blockType, `missing ${field} snapshot payload`);
  }

  const lines = payload.filter((entry): entry is string => typeof entry === "string");
  if (lines.length !== payload.length) {
    syntaxRenderError(blockType, `${field} contains non-string entries`);
  }

  return lines;
}

function stripStyleMarkup(line: string): string {
  const escapedOpenToken = "\u0000DOCS_ESCAPED_OPEN\u0000";
  return line
    .replaceAll("\\[[", escapedOpenToken)
    .replaceAll("\\{", "{")
    .replaceAll("\\}", "}")
    .replace(/\[\[[^\]]*]]/g, "")
    .replaceAll(escapedOpenToken, "[[");
}

function styleLine(
  line: string,
  styleContext: DocsRenderStyleContext | undefined,
  intent?: ColorIntent,
): string {
  if (!styleContext || line.length === 0) {
    return line;
  }

  const payload = intent ? `[[fg=$${intent}]]${line}[[/]]` : line;
  return styleText(payload, {
    theme: styleContext.theme,
    runtime: styleContext.colors,
  });
}

function boundedLine(
  line: string,
  width: number,
  styleContext?: DocsRenderStyleContext,
  intent?: ColorIntent,
): string {
  return styleLine(truncateLine(line, width), styleContext, intent);
}

function styleSnapshotLine(line: string, styleContext?: DocsRenderStyleContext): string {
  if (!styleContext) {
    return stripStyleMarkup(line);
  }

  return styleText(line, {
    theme: styleContext.theme,
    runtime: styleContext.colors,
  });
}

function prefixWrapped(prefix: string, text: string, width: number): string[] {
  const prefixText = prefix.length > 0 ? `${prefix} ` : "";
  const contentWidth = Math.max(1, width - prefixText.length);
  const wrapped = wrapText(text, contentWidth);

  if (prefixText.length === 0) {
    return wrapped;
  }

  const indent = " ".repeat(prefixText.length);
  return wrapped.map((line, index) => (index === 0 ? `${prefixText}${line}` : `${indent}${line}`));
}

function asInlineContent(value: unknown): string {
  if (typeof value === "string") {
    return value;
  }

  if (Array.isArray(value)) {
    return inlineToPlain(value as RichTextLike);
  }

  return "";
}

function asDocsBlocks(value: unknown): DocsBlockLike[] {
  if (!Array.isArray(value)) {
    return [];
  }

  return value.filter(
    (item): item is DocsBlockLike =>
      typeof item === "object" &&
      item !== null &&
      typeof (item as { type?: unknown }).type === "string",
  );
}

function wrapBounded(
  text: string,
  width: number,
  styleContext?: DocsRenderStyleContext,
  intent?: ColorIntent,
): string[] {
  return wrapText(text, width).map((line) => boundedLine(line, width, styleContext, intent));
}

function prefixedBounded(
  prefix: string,
  text: string,
  width: number,
  styleContext?: DocsRenderStyleContext,
  intent?: ColorIntent,
): string[] {
  return prefixWrapped(prefix, text, width).map((line) =>
    boundedLine(line, width, styleContext, intent),
  );
}

function prefixedSnapshot(
  prefix: string,
  lines: ReadonlyArray<string>,
  styleContext?: DocsRenderStyleContext,
): string[] {
  const prefixText = prefix.length > 0 ? `${prefix} ` : "";
  return lines.map((line) => styleSnapshotLine(`${prefixText}${line}`, styleContext));
}

function blockLines(
  block: DocsBlockLike,
  width: number,
  styleContext?: DocsRenderStyleContext,
): string[] {
  const safeWidth = normalizeWidth(width);

  switch (block.type) {
    case "text": {
      return wrapBounded(asInlineContent(block.content), safeWidth, styleContext);
    }
    case "code": {
      resolveBlockLanguage(block, "code");
      const lines: string[] = [];
      const filename = block.filename;
      if (typeof filename === "string" && filename.length > 0) {
        lines.push(boundedLine(`File: ${filename}`, safeWidth, styleContext, "info"));
      }
      lines.push(boundedLine(horizontalRule(safeWidth, "-"), safeWidth, styleContext, "border"));
      lines.push(
        ...resolveHighlightedLines(block, "code", "highlightedLines").map((line) =>
          styleSnapshotLine(line, styleContext),
        ),
      );
      lines.push(boundedLine(horizontalRule(safeWidth, "-"), safeWidth, styleContext, "border"));
      return lines;
    }
    case "command": {
      resolveBlockLanguage(block, "command");
      const lines: string[] = [];
      const description = typeof block.description === "string" ? block.description : "Command";
      lines.push(...wrapBounded(description, safeWidth, styleContext, "info"));
      lines.push(
        ...prefixedSnapshot(
          "$",
          resolveHighlightedLines(block, "command", "highlightedCommandLines"),
          styleContext,
        ),
      );
      if (typeof block.output === "string" && block.output.length > 0) {
        for (const line of block.output.split("\n")) {
          lines.push(...prefixedBounded("->", line, safeWidth, styleContext, "dimText"));
        }
      }
      return lines;
    }
    case "list": {
      const lines: string[] = [];
      const items = Array.isArray(block.items) ? block.items : [];
      for (const [index, item] of items.entries()) {
        if (typeof item === "string" || Array.isArray(item)) {
          const text = inlineToPlain(item as string | RichTextLike);
          const prefix = block.ordered ? `${index + 1}.` : "-";
          lines.push(...prefixedBounded(prefix, text, safeWidth, styleContext));
          continue;
        }

        if (typeof item === "object" && item !== null) {
          const itemText = asInlineContent((item as { text?: unknown }).text);
          const prefix = block.ordered ? `${index + 1}.` : "-";
          lines.push(...prefixedBounded(prefix, itemText, safeWidth, styleContext));

          const children = Array.isArray((item as { children?: unknown }).children)
            ? ((item as { children?: unknown[] }).children ?? [])
            : [];
          for (const child of children) {
            const childText = asInlineContent(child);
            if (childText.length > 0) {
              lines.push(...prefixedBounded("  -", childText, safeWidth, styleContext, "dimText"));
            }
          }
        }
      }
      return lines;
    }
    case "table": {
      const headers = Array.isArray(block.headers) ? block.headers : [];
      const rows = Array.isArray(block.rows) ? block.rows : [];
      const headerLine = headers
        .map((cell) => inlineToPlain(cell as string | RichTextLike))
        .join(" | ");
      const lines = [
        boundedLine(headerLine, safeWidth, styleContext, "accent"),
        boundedLine(
          horizontalRule(Math.min(safeWidth, Math.max(1, headerLine.length)), "-"),
          safeWidth,
          styleContext,
          "border",
        ),
      ];
      for (const row of rows) {
        if (!Array.isArray(row)) {
          continue;
        }
        lines.push(
          boundedLine(
            row.map((cell) => inlineToPlain(cell as string | RichTextLike)).join(" | "),
            safeWidth,
            styleContext,
          ),
        );
      }
      return lines;
    }
    case "interactive": {
      const lines: string[] = [];
      const introText = asInlineContent(block.intro);
      if (introText.length > 0) {
        lines.push(...wrapBounded(introText, safeWidth, styleContext));
      }
      const steps = Array.isArray(block.steps) ? block.steps : [];
      for (const step of steps) {
        if (typeof step !== "object" || step === null) {
          continue;
        }
        const command =
          typeof (step as { command?: unknown }).command === "string"
            ? ((step as { command?: string }).command ?? "")
            : "";
        const description = asInlineContent((step as { description?: unknown }).description);
        lines.push(...prefixedBounded("-", command, safeWidth, styleContext, "success"));
        lines.push(...prefixedBounded("", description, safeWidth, styleContext));
      }
      return lines;
    }
    case "conversation": {
      const lines: string[] = [];
      const messages = Array.isArray(block.messages) ? block.messages : [];
      for (const message of messages) {
        if (typeof message !== "object" || message === null) {
          continue;
        }
        const role = (message as { role?: string }).role === "user" ? "You" : "AI";
        const roleIntent: ColorIntent = role === "You" ? "info" : "accent";
        const text = asInlineContent((message as { text?: unknown }).text);
        lines.push(...prefixedBounded(`${role}:`, text, safeWidth, styleContext, roleIntent));

        const listItems = Array.isArray((message as { list?: unknown }).list)
          ? ((message as { list?: unknown[] }).list ?? [])
          : [];
        for (const item of listItems) {
          const listText = asInlineContent(item);
          if (listText.length > 0) {
            lines.push(...prefixedBounded("  -", listText, safeWidth, styleContext, "dimText"));
          }
        }
      }
      return lines;
    }
    case "qa": {
      const lines: string[] = [];
      const items = Array.isArray(block.items) ? block.items : [];
      for (const [itemIndex, item] of items.entries()) {
        if (typeof item !== "object" || item === null) {
          continue;
        }

        const question = asInlineContent((item as { question?: unknown }).question);
        lines.push(...prefixedBounded("Q:", question, safeWidth, styleContext, "info"));
        lines.push(boundedLine("A:", safeWidth, styleContext, "success"));

        const answerBlocks = asDocsBlocks((item as { answer?: unknown }).answer);
        if (answerBlocks.length === 0) {
          lines.push(
            ...prefixedBounded("  ", "(no answer provided)", safeWidth, styleContext, "dimText"),
          );
          if (itemIndex < items.length - 1) {
            lines.push("");
          }
          continue;
        }

        for (const [answerIndex, answerBlock] of answerBlocks.entries()) {
          const answerLines = blockLines(answerBlock, Math.max(1, safeWidth - 2), styleContext);
          for (const line of answerLines) {
            lines.push(line.length === 0 ? "" : boundedLine(`  ${line}`, safeWidth, styleContext));
          }
          if (answerIndex < answerBlocks.length - 1) {
            lines.push("");
          }
        }

        if (itemIndex < items.length - 1) {
          lines.push("");
        }
      }
      return lines;
    }
    case "note": {
      const variant = typeof block.variant === "string" ? block.variant.toLowerCase() : "info";
      const intent: ColorIntent =
        variant === "warning" ? "warning" : variant === "danger" ? "error" : "info";
      const label = typeof block.variant === "string" ? block.variant.toUpperCase() : "NOTE";
      return prefixedBounded(
        `${label}:`,
        asInlineContent(block.content),
        safeWidth,
        styleContext,
        intent,
      );
    }
    default:
      return [];
  }
}

export function renderDocsSidebar(
  items: FlatDocsNavItem[],
  selectedIndex: number,
  options: RenderSidebarOptions = {},
): string {
  const width = normalizeWidth(options.maxWidth ?? 30);
  if (items.length === 0) {
    return "(no pages)";
  }

  const selected = clampNumber(toNonNegativeInt(selectedIndex), 0, items.length - 1);
  const lines: string[] = [];

  let currentSection = "";
  for (const [index, item] of items.entries()) {
    if (item.sectionTitle !== currentSection) {
      currentSection = item.sectionTitle;
      if (lines.length > 0) {
        lines.push("");
      }
      lines.push(boundedLine(currentSection, width, options.styleContext, "dimText"));
    }

    const marker = index === selected ? ">" : " ";
    const intent: ColorIntent = index === selected ? "accent" : "text";
    lines.push(boundedLine(`${marker} ${item.title}`, width, options.styleContext, intent));
  }

  return lines.join("\n");
}

export function renderDocsHeader(
  content: DocsContentLike,
  slug: string,
  scrollOffset: number,
  totalLines: number,
  visibleRows: number,
  width: number,
  options: RenderHeaderOptions = {},
): string[] {
  const safeWidth = normalizeWidth(width);
  const safeTotalLines = toNonNegativeInt(totalLines);
  const safeVisibleRows = Math.max(1, toNonNegativeInt(visibleRows));
  const maxOffset = Math.max(0, safeTotalLines - 1);
  const safeScrollOffset = clampNumber(toNonNegativeInt(scrollOffset), 0, maxOffset);
  const start = safeTotalLines === 0 ? 0 : Math.min(safeTotalLines, safeScrollOffset + 1);
  const end = safeTotalLines === 0 ? 0 : Math.min(safeTotalLines, start + safeVisibleRows - 1);

  return [
    boundedLine(content.title, safeWidth, options.styleContext, "accent"),
    boundedLine(
      `${slug} (${start}-${end}/${safeTotalLines})`,
      safeWidth,
      options.styleContext,
      "dimText",
    ),
    boundedLine(horizontalRule(safeWidth, "-"), safeWidth, options.styleContext, "border"),
  ];
}

export function renderDocsPageLines(
  content: DocsContentLike,
  width: number,
  options: RenderPageOptions = {},
): string[] {
  const safeWidth = normalizeWidth(width);
  const lines: string[] = [];

  if (content.intro) {
    lines.push(...wrapBounded(inlineToPlain(content.intro), safeWidth, options.styleContext));
    lines.push("");
  }

  for (const section of content.sections) {
    const prefix = section.level === 3 ? "###" : "##";
    lines.push(
      boundedLine(`${prefix} ${section.title}`, safeWidth, options.styleContext, "accent"),
    );
    lines.push("");

    for (const block of section.content) {
      const rendered = blockLines(block, safeWidth, options.styleContext);
      lines.push(...rendered);
      lines.push("");
    }
  }

  while (lines.length > 0 && lines[lines.length - 1] === "") {
    lines.pop();
  }

  return lines;
}
