import { createTheme, type TuiTheme } from "@levitate/tui-kit";

export function createInstallDocsTheme(): TuiTheme {
	return createTheme(
		{
			background: "default",
			border: {
				truecolor: "#5a5870",
				ansi256: 60,
				ansi16: "gray",
			},
			sidebarBorder: {
				truecolor: "#5a5870",
				ansi256: 60,
				ansi16: "gray",
			},
			sidebarBackground: "default",
			contentBackground: "default",
			headerBackground: "default",
			headerAccentBackground: "default",
			footerBackground: "default",
			sidebarSectionText: {
				truecolor: "#eceaf4",
				ansi256: 255,
				ansi16: "white",
			},
			sidebarItemText: {
				truecolor: "#b8b3c9",
				ansi256: 146,
				ansi16: "gray",
			},
			sidebarItemActiveText: {
				truecolor: "#f5f7ff",
				ansi256: 15,
				ansi16: "white",
			},
			sidebarItemActiveBackground: {
				truecolor: "#1f4acc",
				ansi256: 26,
				ansi16: "blue",
			},
			text: "#eceaf4",
			dimText: "#a19ab6",
			accent: {
				truecolor: "#7db3ff",
				ansi256: 111,
				ansi16: "cyan",
			},
			info: "#8ab8ff",
			warning: "#ffc76a",
			commandBarBackground: "default",
			commandPrompt: "#8ab8ff",
			warningBackground: "default",
			sectionHeading: "#9cc4ff",
			sectionSubheading: "#7db3ff",
			cardBorder: "#49455e",
			cardBackground: "default",
		},
		{
			sidebarWidth: 32,
			minColumns: 88,
			headerHeight: 1,
			footerHeight: 1,
		},
		{
			borderGlyphSet: "single",
			titleStyle: "slot",
			panePaddingX: 1,
			panePaddingY: 0,
			framePaneGap: 0,
			sidebarHeaderMode: "current-section-title",
		},
	);
}
