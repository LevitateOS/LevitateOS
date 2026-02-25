import {
	useKeyInput,
	type InkKeyLike,
	type KeyInputEvent,
	type KeyInputHandler,
	type KeySpec,
	type UseKeyInputOptions,
} from "../runtime/ink/use-input";

export function useHotkeys(
	keys: KeySpec | undefined,
	handler: KeyInputHandler,
	options: UseKeyInputOptions = {},
): void {
	useKeyInput(keys, handler, options);
}

export { keyNamesFromInput, normalizeKeySpec } from "../runtime/ink/use-input";

export type { InkKeyLike, KeyInputEvent, KeyInputHandler, KeySpec, UseKeyInputOptions };

// Alias retained as a stable generic input hook name.
export const useTuiInput = useHotkeys;
