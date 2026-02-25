import { createTuiApp, renderApp, type RenderedTuiApp } from "@levitate/tui-kit";
import { createMappedDocsSource } from "../adapters/docs-content/mapper";
import { contentForSlug } from "../domain/content/index";
import { getDistroProfile } from "../domain/distro/registry";
import type { DistroId } from "../domain/distro/profile";
import { createInstallSession } from "./session";
import { InstallViewerScreen } from "../presentation/ink/screens/install-viewer";
import { createInstallDocsRendererRegistry } from "../presentation/ink/document/renderer-registry";
import { createInstallDocsTheme } from "../presentation/ink/theme/index";
import { AppError } from "./errors";

type StartInstallDocsOptions = {
	slug?: string;
	distro?: DistroId;
};

export function startInstallDocsApp(options: StartInstallDocsOptions = {}): RenderedTuiApp {
	const profile = getDistroProfile(options.distro ?? "levitate");
	const source = createMappedDocsSource();

	let session;
	try {
		session = createInstallSession(source, profile, options.slug);
	} catch (error: unknown) {
		const message = error instanceof Error ? error.message : String(error);
		throw new AppError("E_DOCS_SCOPE", message, 2);
	}

	const app = createTuiApp({
		title: "levitate-install-docs",
		theme: createInstallDocsTheme(),
	});
	const renderers = createInstallDocsRendererRegistry();

	return renderApp(
		<InstallViewerScreen
			navItems={session.navItems}
			getContent={(slug, title) => contentForSlug(session.source, slug, title)}
			initialSlug={session.initialSlug}
			title={profile.title}
			renderers={renderers}
		/>,
		{
			app,
			exitOnCtrlC: false,
		},
	);
}
