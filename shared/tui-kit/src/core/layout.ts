import { createBlessedBox, type BlessedBox } from "../adapters/blessed/box";
import type { ScreenHandle } from "./screen";
import { tuiTheme } from "../theme";
import { safeDestroy } from "./lifecycle";
import { clampNumber, toPositiveInt } from "../internal/numbers";

export type TwoPaneGeometry = {
  contentInnerColumns: number;
  contentInnerRows: number;
  sidebarColumns: number;
  bodyRows: number;
};

export type TwoPaneMetrics = TwoPaneGeometry & {
  totalColumns: number;
  totalRows: number;
  contentColumns: number;
  headerRows: number;
  footerRows: number;
};

export type TwoPaneOptions = {
  title: string;
  sidebarContent: string;
  footerText?: string;
  contentScrollable?: boolean;
  sidebarWidth?: number;
};

export type TwoPaneShell = {
  root: BlessedBox;
  header: BlessedBox;
  sidebar: BlessedBox;
  content: BlessedBox;
  footer: BlessedBox;
  geometry: TwoPaneGeometry;
};

const shellMap = new WeakMap<object, TwoPaneShell>();

function destroyShell(shell: TwoPaneShell | undefined): void {
  if (!shell) {
    return;
  }

  safeDestroy(shell.root);
}

export function clamp(value: number, min: number, max: number): number {
  return clampNumber(value, min, max);
}

export function computeTwoPaneMetrics(
  totalColumns: number,
  totalRows: number,
  desiredSidebar: number,
  headerRows: number,
  footerRows: number,
): TwoPaneMetrics {
  const columns = Math.max(2, toPositiveInt(totalColumns, 80));
  const rows = Math.max(3, toPositiveInt(totalRows, 24));

  const normalizedHeaderRows = clamp(toPositiveInt(headerRows, 1), 1, rows - 2);
  const normalizedFooterRows = clamp(
    toPositiveInt(footerRows, 1),
    1,
    rows - normalizedHeaderRows - 1,
  );
  const bodyRows = Math.max(1, rows - normalizedHeaderRows - normalizedFooterRows);

  const minContentColumns = Math.min(20, Math.max(1, columns - 1));
  const maxSidebarColumns = Math.max(1, columns - minContentColumns);
  const minSidebarColumns = Math.min(18, maxSidebarColumns);
  const sidebarColumns = clamp(
    toPositiveInt(desiredSidebar, 30),
    minSidebarColumns,
    maxSidebarColumns,
  );
  const contentColumns = Math.max(1, columns - sidebarColumns);

  return {
    totalColumns: columns,
    totalRows: rows,
    contentColumns,
    headerRows: normalizedHeaderRows,
    footerRows: normalizedFooterRows,
    sidebarColumns,
    bodyRows,
    contentInnerColumns: Math.max(1, contentColumns - 2),
    contentInnerRows: Math.max(1, bodyRows - 2),
  };
}

export function createTwoPaneShell(screen: ScreenHandle, options: TwoPaneOptions): TwoPaneShell {
  const key = screen.raw as object;
  destroyShell(shellMap.get(key));

  const metrics = computeTwoPaneMetrics(
    screen.width,
    screen.height,
    options.sidebarWidth ?? tuiTheme.layout.sidebarWidth,
    tuiTheme.layout.headerHeight,
    tuiTheme.layout.footerHeight,
  );

  const root = createBlessedBox({
    parent: screen.raw,
    top: 0,
    left: 0,
    width: "100%",
    height: "100%",
  });

  const header = createBlessedBox({
    parent: root,
    top: 0,
    left: 0,
    width: "100%",
    height: metrics.headerRows,
    content: options.title,
    tags: true,
    style: {
      fg: tuiTheme.tokens.text,
      bg: tuiTheme.tokens.background,
      bold: true,
    },
  });

  const sidebar = createBlessedBox({
    parent: root,
    top: metrics.headerRows,
    left: 0,
    width: metrics.sidebarColumns,
    height: metrics.bodyRows,
    border: "line",
    tags: true,
    scrollable: true,
    alwaysScroll: true,
    content: options.sidebarContent,
    style: {
      fg: tuiTheme.tokens.text,
      border: { fg: tuiTheme.tokens.border },
    },
  });

  const content = createBlessedBox({
    parent: root,
    top: metrics.headerRows,
    left: metrics.sidebarColumns,
    width: metrics.contentColumns,
    height: metrics.bodyRows,
    border: "line",
    tags: true,
    scrollable: Boolean(options.contentScrollable),
    alwaysScroll: Boolean(options.contentScrollable),
    keys: true,
    vi: true,
    style: {
      fg: tuiTheme.tokens.text,
      border: { fg: tuiTheme.tokens.border },
    },
  });

  const footer = createBlessedBox({
    parent: root,
    top: metrics.headerRows + metrics.bodyRows,
    left: 0,
    width: "100%",
    height: metrics.footerRows,
    content: options.footerText ?? "",
    tags: true,
    style: {
      fg: tuiTheme.tokens.dimText,
      bg: tuiTheme.tokens.background,
    },
  });

  const shell: TwoPaneShell = {
    root,
    header,
    sidebar,
    content,
    footer,
    geometry: {
      contentInnerColumns: metrics.contentInnerColumns,
      contentInnerRows: metrics.contentInnerRows,
      sidebarColumns: metrics.sidebarColumns,
      bodyRows: metrics.bodyRows,
    },
  };

  shellMap.set(key, shell);
  return shell;
}
