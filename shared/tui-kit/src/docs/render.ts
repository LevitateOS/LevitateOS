import { horizontalRule, truncateLine, wrapText } from "../theme/styles";
import { inlineToPlain, type RichTextLike } from "./inline";
import type { FlatDocsNavItem } from "./nav";
import { clampNumber, toNonNegativeInt, toPositiveInt } from "../internal/numbers";

type DocsContentLike = {
  title: string;
  intro?: string | RichTextLike;
  sections: DocsSectionLike[];
};

type DocsSectionLike = {
  title: string;
  level?: 2 | 3;
  content: DocsBlockLike[];
};

type DocsBlockLike = {
  type: string;
  [key: string]: unknown;
};

type RenderSidebarOptions = {
  maxWidth?: number;
};

function normalizeWidth(width: number): number {
  return Math.max(20, toPositiveInt(width, 20));
}

function boundedLine(line: string, width: number): string {
  return truncateLine(line, width);
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

function wrapBounded(text: string, width: number): string[] {
  return wrapText(text, width).map((line) => boundedLine(line, width));
}

function prefixedBounded(prefix: string, text: string, width: number): string[] {
  return prefixWrapped(prefix, text, width).map((line) => boundedLine(line, width));
}

function blockLines(block: DocsBlockLike, width: number): string[] {
  const safeWidth = normalizeWidth(width);

  switch (block.type) {
    case "text": {
      return wrapBounded(asInlineContent(block.content), safeWidth);
    }
    case "code": {
      const lines: string[] = [];
      const filename = block.filename;
      if (typeof filename === "string" && filename.length > 0) {
        lines.push(boundedLine(`File: ${filename}`, safeWidth));
      }
      lines.push(horizontalRule(safeWidth, "-"));
      const content = typeof block.content === "string" ? block.content : "";
      lines.push(...content.split("\n").map((line) => boundedLine(line, safeWidth)));
      lines.push(horizontalRule(safeWidth, "-"));
      return lines;
    }
    case "command": {
      const lines: string[] = [];
      const description = typeof block.description === "string" ? block.description : "Command";
      lines.push(...wrapText(description, safeWidth).map((line) => boundedLine(line, safeWidth)));
      const command = Array.isArray(block.command)
        ? block.command.join("\n")
        : typeof block.command === "string"
          ? block.command
          : "";
      for (const line of command.split("\n")) {
        lines.push(...prefixedBounded("$", line, safeWidth));
      }
      if (typeof block.output === "string" && block.output.length > 0) {
        lines.push(...prefixedBounded("->", block.output, safeWidth));
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
          lines.push(...prefixedBounded(prefix, text, safeWidth));
          continue;
        }

        if (typeof item === "object" && item !== null) {
          const itemText = asInlineContent((item as { text?: unknown }).text);
          const prefix = block.ordered ? `${index + 1}.` : "-";
          lines.push(...prefixedBounded(prefix, itemText, safeWidth));
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
        boundedLine(headerLine, safeWidth),
        horizontalRule(Math.min(safeWidth, Math.max(1, headerLine.length)), "-"),
      ];
      for (const row of rows) {
        if (!Array.isArray(row)) {
          continue;
        }
        lines.push(
          boundedLine(
            row.map((cell) => inlineToPlain(cell as string | RichTextLike)).join(" | "),
            safeWidth,
          ),
        );
      }
      return lines;
    }
    case "interactive": {
      const lines: string[] = [];
      const introText = asInlineContent(block.intro);
      if (introText.length > 0) {
        lines.push(...wrapBounded(introText, safeWidth));
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
        lines.push(...prefixedBounded("-", command, safeWidth));
        lines.push(...prefixedBounded("", description, safeWidth));
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
        const text = asInlineContent((message as { text?: unknown }).text);
        lines.push(...prefixedBounded(`${role}:`, text, safeWidth));
      }
      return lines;
    }
    case "qa": {
      const lines: string[] = [];
      const items = Array.isArray(block.items) ? block.items : [];
      for (const item of items) {
        if (typeof item !== "object" || item === null) {
          continue;
        }
        const question = asInlineContent((item as { question?: unknown }).question);
        lines.push(...prefixedBounded("Q:", question, safeWidth));
        lines.push("A:");
      }
      return lines;
    }
    case "note": {
      const variant = typeof block.variant === "string" ? block.variant.toUpperCase() : "NOTE";
      return prefixedBounded(`${variant}:`, asInlineContent(block.content), safeWidth);
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
      lines.push(truncateLine(currentSection, width));
    }

    const marker = index === selected ? ">" : " ";
    lines.push(truncateLine(`${marker} ${item.title}`, width));
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
): string[] {
  const safeWidth = normalizeWidth(width);
  const safeTotalLines = toNonNegativeInt(totalLines);
  const safeVisibleRows = Math.max(1, toNonNegativeInt(visibleRows));
  const maxOffset = Math.max(0, safeTotalLines - 1);
  const safeScrollOffset = clampNumber(toNonNegativeInt(scrollOffset), 0, maxOffset);
  const start = safeTotalLines === 0 ? 0 : Math.min(safeTotalLines, safeScrollOffset + 1);
  const end = safeTotalLines === 0 ? 0 : Math.min(safeTotalLines, start + safeVisibleRows - 1);

  return [
    truncateLine(content.title, safeWidth),
    truncateLine(`${slug} (${start}-${end}/${safeTotalLines})`, safeWidth),
    horizontalRule(safeWidth, "-"),
  ];
}

export function renderDocsPageLines(content: DocsContentLike, width: number): string[] {
  const safeWidth = normalizeWidth(width);
  const lines: string[] = [];

  if (content.intro) {
    lines.push(...wrapBounded(inlineToPlain(content.intro), safeWidth));
    lines.push("");
  }

  for (const section of content.sections) {
    const prefix = section.level === 3 ? "###" : "##";
    lines.push(boundedLine(`${prefix} ${section.title}`, safeWidth));
    lines.push("");

    for (const block of section.content) {
      const rendered = blockLines(block, safeWidth);
      lines.push(...rendered);
      lines.push("");
    }
  }

  while (lines.length > 0 && lines[lines.length - 1] === "") {
    lines.pop();
  }

  return lines;
}
