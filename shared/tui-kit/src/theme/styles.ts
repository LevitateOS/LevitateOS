import { toNonNegativeInt } from "../internal/numbers";
import { resolveColorReference, resolveIntentMono, type ColorRuntime } from "./colors";
import type { ColorIntent, TuiTheme } from "./tokens";

export type StyleTextOptions = {
  theme: TuiTheme;
  runtime: ColorRuntime;
};

type TagStackEntry = {
  close: string[];
};

function normalizeWidth(width: number): number {
  return toNonNegativeInt(width, 0);
}

function isEscapedOpenTag(input: string, index: number): boolean {
  return input[index] === "\\" && input[index + 1] === "[" && input[index + 2] === "[";
}

function parseTag(input: string, start: number): { raw: string; body: string; end: number } | null {
  if (!input.startsWith("[[", start)) {
    return null;
  }
  const end = input.indexOf("]]", start + 2);
  if (end < 0) {
    return null;
  }

  return {
    raw: input.slice(start, end + 2),
    body: input.slice(start + 2, end).trim(),
    end: end + 2,
  };
}

function normalizeDirectives(body: string): string[] {
  const normalized = body.replace(/[;,]/g, " ").trim();
  if (normalized.length === 0) {
    return [];
  }
  return normalized.split(/\s+/).filter((part) => part.length > 0);
}

function isIntent(reference: string): reference is `$${ColorIntent}` {
  return reference.startsWith("$");
}

function pushDirectiveTag(open: string[], close: string[], tag: string): void {
  open.push(`{${tag}}`);
  close.unshift(`{/${tag}}`);
}

function compileOpenTag(
  directives: string[],
  options: StyleTextOptions,
): { open: string[]; close: string[] } | null {
  const open: string[] = [];
  const close: string[] = [];

  for (const directive of directives) {
    if (
      directive === "bold" ||
      directive === "underline" ||
      directive === "inverse" ||
      directive === "dim"
    ) {
      pushDirectiveTag(open, close, directive);
      continue;
    }

    const [rawKey, ...rest] = directive.split("=");
    const key = rawKey?.trim();
    const value = rest.join("=").trim();

    if (!key || value.length === 0 || (key !== "fg" && key !== "bg")) {
      return null;
    }

    if (isIntent(value) && options.runtime.mode === "mono") {
      const intent = value.slice(1) as ColorIntent;
      const emphasis = resolveIntentMono(options.theme, intent);
      if (emphasis === "bold" || emphasis === "dim") {
        pushDirectiveTag(open, close, emphasis);
      }
      continue;
    }

    const resolved = resolveColorReference(value, options.theme, options.runtime);
    if (!resolved) {
      continue;
    }

    pushDirectiveTag(open, close, `${resolved}-${key}`);
  }

  return {
    open,
    close,
  };
}

export function styleText(text: string, options: StyleTextOptions): string {
  if (text.length === 0) {
    return text;
  }

  let index = 0;
  let output = "";
  const stack: TagStackEntry[] = [];

  while (index < text.length) {
    if (isEscapedOpenTag(text, index)) {
      output += "[[";
      index += 3;
      continue;
    }

    if (!text.startsWith("[[", index)) {
      output += text[index];
      index += 1;
      continue;
    }

    const token = parseTag(text, index);
    if (!token) {
      output += text[index];
      index += 1;
      continue;
    }

    if (token.body === "/") {
      const current = stack.pop();
      if (!current) {
        output += token.raw;
      } else {
        output += current.close.join("");
      }
      index = token.end;
      continue;
    }

    const directives = normalizeDirectives(token.body);
    if (directives.length === 0) {
      output += token.raw;
      index = token.end;
      continue;
    }

    const compiled = compileOpenTag(directives, options);
    if (!compiled) {
      output += token.raw;
      index = token.end;
      continue;
    }

    output += compiled.open.join("");
    stack.push({ close: compiled.close });
    index = token.end;
  }

  while (stack.length > 0) {
    output += stack.pop()?.close.join("") ?? "";
  }

  return output;
}

export function padRight(text: string, width: number): string {
  const safeWidth = normalizeWidth(width);
  if (safeWidth <= 0) {
    return "";
  }

  if (text.length >= safeWidth) {
    return text.slice(0, safeWidth);
  }

  return text + " ".repeat(safeWidth - text.length);
}

export function truncateLine(text: string, width: number): string {
  const safeWidth = normalizeWidth(width);
  if (safeWidth <= 0) {
    return "";
  }

  if (text.length <= safeWidth) {
    return text;
  }

  if (safeWidth === 1) {
    return "…";
  }

  return `${text.slice(0, safeWidth - 1)}…`;
}

export function horizontalRule(width: number, char = "-"): string {
  const safeWidth = normalizeWidth(width);
  if (safeWidth <= 0) {
    return "";
  }
  const fill = typeof char === "string" && char.length > 0 ? char[0] : "-";
  return fill.repeat(safeWidth);
}

function chunkWord(word: string, width: number): string[] {
  if (word.length <= width) {
    return [word];
  }

  const chunks: string[] = [];
  for (let index = 0; index < word.length; index += width) {
    chunks.push(word.slice(index, index + width));
  }
  return chunks;
}

export function wrapText(text: string, width: number): string[] {
  const safeWidth = normalizeWidth(width);
  if (safeWidth <= 0) {
    return [text];
  }

  const output: string[] = [];

  for (const line of text.split("\n")) {
    const words = line.split(/\s+/).filter((word) => word.length > 0);

    if (words.length === 0) {
      output.push("");
      continue;
    }

    let current = "";
    for (const word of words) {
      const wordChunks = chunkWord(word, safeWidth);

      for (const chunk of wordChunks) {
        if (current.length === 0) {
          current = chunk;
          continue;
        }

        const candidate = `${current} ${chunk}`;
        if (candidate.length <= safeWidth) {
          current = candidate;
        } else {
          output.push(current);
          current = chunk;
        }
      }
    }

    if (current.length > 0) {
      output.push(current);
    }
  }

  return output.length > 0 ? output : [""];
}
