export type DocsNavItemLike = {
  title: string;
  href: string;
};

export type DocsNavSectionLike = {
  title: string;
  items: ReadonlyArray<DocsNavItemLike>;
};

export type FlatDocsNavItem = {
  sectionTitle: string;
  title: string;
  href: string;
  slug: string;
};

function slugFromHref(href: string): string {
  const normalizedHref = href.trim();

  const withoutQuery = normalizedHref.split("?")[0]?.split("#")[0] ?? normalizedHref;
  const noTrailingSlash = withoutQuery.replace(/\/+$/, "");

  if (noTrailingSlash.startsWith("/docs/")) {
    return noTrailingSlash.slice("/docs/".length);
  }

  if (noTrailingSlash.startsWith("docs/")) {
    return noTrailingSlash.slice("docs/".length);
  }

  if (noTrailingSlash.startsWith("/")) {
    return noTrailingSlash.slice(1);
  }

  return noTrailingSlash.length > 0 ? noTrailingSlash : "index";
}

export function flattenDocsNav(sections: ReadonlyArray<DocsNavSectionLike>): FlatDocsNavItem[] {
  const output: FlatDocsNavItem[] = [];

  for (const section of sections) {
    for (const item of section.items) {
      output.push({
        sectionTitle: section.title,
        title: item.title,
        href: item.href,
        slug: slugFromHref(item.href),
      });
    }
  }

  return output;
}
