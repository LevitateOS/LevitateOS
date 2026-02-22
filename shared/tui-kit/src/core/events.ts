import type { BlessedKey, BlessedKeyHandler } from "../adapters/blessed/screen";
import type { ScreenHandle } from "./screen";

export type KeySpec = string | string[];

export type KeyHandler = (ch: string, key: BlessedKey) => void;

export function normalizeKeySpec(keys: KeySpec): string | string[] | null {
  if (Array.isArray(keys)) {
    const normalized = Array.from(
      new Set(
        keys
          .map((key) => (typeof key === "string" ? key.trim() : ""))
          .filter((key) => key.length > 0),
      ),
    );

    return normalized.length > 0 ? normalized : null;
  }

  const key = keys.trim();
  return key.length > 0 ? key : null;
}

export function onKey(screen: ScreenHandle, keys: KeySpec, handler: KeyHandler): () => void {
  const normalizedKeys = normalizeKeySpec(keys);
  if (!normalizedKeys) {
    return () => {
      // No-op: no key bindings were registered.
    };
  }

  screen.key(normalizedKeys, handler as BlessedKeyHandler);

  let active = true;
  return () => {
    if (!active) {
      return;
    }
    active = false;
    screen.unkey(normalizedKeys, handler as BlessedKeyHandler);
  };
}
