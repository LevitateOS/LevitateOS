import { describe, expect, it } from "bun:test";
import { inlineToPlain } from "../src/docs/inline";
import { flattenDocsNav } from "../src/docs/nav";
import { renderDocsHeader, renderDocsPageLines, renderDocsSidebar } from "../src/docs/render";

describe("docs helpers", () => {
  it("inlineToPlain flattens rich text-like arrays", () => {
    expect(inlineToPlain(["Hello ", { type: "bold", text: "world" }, "!"])).toBe("Hello world!");
  });

  it("flattenDocsNav derives slugs from /docs/* href", () => {
    const flat = flattenDocsNav([
      {
        title: "Start",
        items: [{ title: "Getting Started", href: "/docs/getting-started" }],
      },
    ]);

    expect(flat.length).toBe(1);
    expect(flat[0].sectionTitle).toBe("Start");
    expect(flat[0].slug).toBe("getting-started");
  });

  it("flattenDocsNav strips query/hash and trailing slash", () => {
    const flat = flattenDocsNav([
      {
        title: "Start",
        items: [{ title: "Install", href: "/docs/install/?tab=1#head" }],
      },
    ]);

    expect(flat[0].slug).toBe("install");
  });

  it("render helpers produce stable output shape", () => {
    const items = [
      {
        sectionTitle: "Core",
        title: "Overview",
        href: "/docs/overview",
        slug: "overview",
      },
      {
        sectionTitle: "Core",
        title: "Install",
        href: "/docs/install",
        slug: "install",
      },
    ];

    const sidebar = renderDocsSidebar(items, 1, { maxWidth: 30 });
    expect(sidebar.includes("> Install")).toBe(true);

    const header = renderDocsHeader({ title: "Install", sections: [] }, "install", 10, 100, 20, 50);
    expect(header.length).toBe(3);

    const page = renderDocsPageLines(
      {
        title: "Install",
        intro: "Intro text",
        sections: [
          {
            title: "Section A",
            content: [{ type: "text", content: "hello world" }],
          },
        ],
      },
      40,
    );

    expect(page.length).toBeGreaterThan(0);
    expect(page.some((line) => line.includes("Section A"))).toBe(true);
  });

  it("render sidebar handles empty page lists", () => {
    expect(renderDocsSidebar([], 0)).toBe("(no pages)");
  });

  it("render sidebar clamps out-of-range selected index", () => {
    const items = [
      {
        sectionTitle: "Core",
        title: "Overview",
        href: "/docs/overview",
        slug: "overview",
      },
      {
        sectionTitle: "Core",
        title: "Install",
        href: "/docs/install",
        slug: "install",
      },
    ];

    const sidebar = renderDocsSidebar(items, 999, { maxWidth: 30 });
    expect(sidebar.includes("> Install")).toBe(true);
  });

  it("render header normalizes invalid numeric inputs", () => {
    const header = renderDocsHeader(
      { title: "Install", sections: [] },
      "install",
      Number.NaN,
      -20,
      Number.NaN,
      Number.NaN,
    );

    expect(header.length).toBe(3);
    expect(header[1].includes("(0-0/0)")).toBe(true);
  });
});
