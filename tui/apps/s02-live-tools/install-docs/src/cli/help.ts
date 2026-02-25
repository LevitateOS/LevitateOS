export function installDocsCliHelpText(): string {
	return `
LevitateOS Docs TUI

Usage:
  levitate-install-docs
  levitate-install-docs --slug <page-slug>
  levitate-install-docs --slug=<page-slug>
  levitate-install-docs --help

Navigation:
  q / Esc / Ctrl-C  Quit
  Tab               Toggle focus (navigation/content)
  Enter             Enter content (from navigation focus)
  m                 Toggle sidebar mode
  h / l             Prev / next page (navigation focus)
  [ / ]             Prev / next section (navigation focus)
  j / k             Scroll content (all focus planes)
  PgUp / PgDn       Fast scroll (content focus)
  g / G             Top / bottom (content focus)

Legacy non-interactive flags (--list, --page, --all) were removed.
`;
}
