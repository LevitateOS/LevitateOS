import { toNonNegativeInt } from "../internal/numbers";

export function styleText(text: string): string {
  return text;
}

function normalizeWidth(width: number): number {
  return toNonNegativeInt(width, 0);
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
