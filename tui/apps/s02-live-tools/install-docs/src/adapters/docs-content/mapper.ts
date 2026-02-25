import type { DocsSource } from "../../domain/content/contracts";
import { loadDocsSource } from "./source";

export function createMappedDocsSource(): DocsSource {
	return loadDocsSource();
}
