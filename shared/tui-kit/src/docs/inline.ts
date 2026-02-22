export type InlineNodeLike =
  | string
  | {
      text?: string;
      children?: ReadonlyArray<InlineNodeLike>;
    };

export type RichTextLike = ReadonlyArray<InlineNodeLike>;

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
