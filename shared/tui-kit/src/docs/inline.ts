export type InlineNodeLike =
  | string
  | {
      text?: string;
      children?: InlineNodeLike[];
      [key: string]: unknown;
    };

export type RichTextLike = InlineNodeLike[];

function inlineNodeToText(node: InlineNodeLike): string {
  if (typeof node === "string") {
    return node;
  }

  const ownText = typeof node.text === "string" ? node.text : "";
  const children = Array.isArray(node.children) ? node.children.map(inlineNodeToText).join("") : "";

  return ownText + children;
}

export function inlineToPlain(input: string | RichTextLike): string {
  if (typeof input === "string") {
    return input;
  }

  return input.map(inlineNodeToText).join("");
}
