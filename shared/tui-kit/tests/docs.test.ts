import { describe, expect, it } from "bun:test";
import { inlineToPlain } from "../src/docs/inline";
import { flattenDocsNav } from "../src/docs/nav";
import { renderDocsHeader, renderDocsPageLines, renderDocsSidebar } from "../src/docs/render";
import { createTheme, type ColorRuntime } from "../src/theme";

const TEST_STYLE_CONTEXT = {
  theme: createTheme(),
  colors: { mode: "ansi16", enabled: true } satisfies ColorRuntime,
};

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

  it("renderDocsPageLines includes QA answers and nested blocks", () => {
    const page = renderDocsPageLines(
      {
        title: "FAQ",
        sections: [
          {
            title: "Common questions",
            content: [
              {
                type: "qa",
                items: [
                  {
                    question: "How do I install?",
                    answer: [
                      { type: "text", content: "Use the installer command." },
                      {
                        type: "command",
                        language: "bash",
                        highlightedCommandLines: ["levitate install"],
                        description: "Run installer",
                        command: "levitate install",
                      },
                    ],
                  },
                ],
              },
            ],
          },
        ],
      },
      80,
    );

    expect(page.some((line) => line.includes("Q: How do I install?"))).toBe(true);
    expect(page.some((line) => line.includes("A:"))).toBe(true);
    expect(page.some((line) => line.includes("Use the installer command."))).toBe(true);
    expect(page.some((line) => line.includes("$ levitate install"))).toBe(true);
  });

  it("renderDocsPageLines renders list children and multiline command output", () => {
    const page = renderDocsPageLines(
      {
        title: "Advanced",
        sections: [
          {
            title: "Details",
            content: [
              {
                type: "list",
                items: [
                  {
                    text: "Parent item",
                    children: ["Child one", "Child two"],
                  },
                ],
              },
              {
                type: "command",
                language: "bash",
                highlightedCommandLines: ["levitate status"],
                description: "Inspect state",
                command: "levitate status",
                output: "line one\nline two",
              },
            ],
          },
        ],
      },
      80,
    );

    expect(page.some((line) => line.includes("- Parent item"))).toBe(true);
    expect(page.some((line) => line.includes("  - Child one"))).toBe(true);
    expect(page.some((line) => line.includes("  - Child two"))).toBe(true);
    expect(page.some((line) => line.includes("-> line one"))).toBe(true);
    expect(page.some((line) => line.includes("-> line two"))).toBe(true);
  });

  it("applies style context to docs render output", () => {
    const page = renderDocsPageLines(
      {
        title: "Styled",
        sections: [
          {
            title: "Color",
            content: [{ type: "text", content: "Hello [[fg=$accent]]world[[/]]" }],
          },
        ],
      },
      80,
      {
        styleContext: TEST_STYLE_CONTEXT,
      },
    );

    expect(page.some((line) => line.includes("{cyan-fg}"))).toBe(true);

    const commandPage = renderDocsPageLines(
      {
        title: "Styled",
        sections: [
          {
            title: "Commands",
            content: [
              {
                type: "command",
                language: "bash",
                highlightedCommandLines: ["[[fg=#22c55e]]levitate status[[/]]"],
                description: "Run",
                command: "levitate status",
              },
            ],
          },
        ],
      },
      80,
      {
        styleContext: TEST_STYLE_CONTEXT,
      },
    );
    expect(
      commandPage.some((line) => line.includes("{green-fg}") && line.includes("levitate status")),
    ).toBe(true);

    const header = renderDocsHeader({ title: "Styled", sections: [] }, "styled", 0, 10, 5, 40, {
      styleContext: TEST_STYLE_CONTEXT,
    });
    expect(header[0]?.includes("{cyan-fg}")).toBe(true);

    const sidebar = renderDocsSidebar(
      [
        {
          sectionTitle: "Core",
          title: "Styled",
          href: "/docs/styled",
          slug: "styled",
        },
      ],
      0,
      { styleContext: TEST_STYLE_CONTEXT },
    );
    expect(sidebar.includes("{cyan-fg}")).toBe(true);
  });

  it("strips syntax markup when style context is not provided", () => {
    const page = renderDocsPageLines(
      {
        title: "Plain",
        sections: [
          {
            title: "Commands",
            content: [
              {
                type: "command",
                language: "bash",
                highlightedCommandLines: ["[[fg=#22c55e]]echo ok[[/]]"],
                description: "Run",
                command: "echo ok",
              },
            ],
          },
        ],
      },
      80,
    );

    expect(page.some((line) => line.includes("$ echo ok"))).toBe(true);
    expect(page.every((line) => !line.includes("[["))).toBe(true);
  });

  it("fails fast when required syntax snapshot payload is missing", () => {
    expect(() =>
      renderDocsPageLines(
        {
          title: "Broken",
          sections: [
            {
              title: "Commands",
              content: [
                {
                  type: "command",
                  language: "bash",
                  description: "Run",
                  command: "echo broken",
                },
              ],
            },
          ],
        },
        80,
      ),
    ).toThrow("docs.render command block");
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
