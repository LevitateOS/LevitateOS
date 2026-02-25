import type { BlockPluginMap } from "./contracts";
import { defineBlockPlugins } from "./contracts";
import { codeBlockPlugin } from "./code-block";
import { commandBlockPlugin } from "./command-block";
import { conversationBlockPlugin } from "./conversation-block";
import { interactiveBlockPlugin } from "./interactive-block";
import { listBlockPlugin } from "./list-block";
import { noteBlockPlugin } from "./note-block";
import { qaBlockPlugin } from "./qa-block";
import { tableBlockPlugin } from "./table-block";
import { textBlockPlugin } from "./text-block";

export const DEFAULT_INSTALL_BLOCK_PLUGINS: BlockPluginMap = defineBlockPlugins({
	text: textBlockPlugin,
	code: codeBlockPlugin,
	table: tableBlockPlugin,
	list: listBlockPlugin,
	conversation: conversationBlockPlugin,
	interactive: interactiveBlockPlugin,
	command: commandBlockPlugin,
	qa: qaBlockPlugin,
	note: noteBlockPlugin,
});
