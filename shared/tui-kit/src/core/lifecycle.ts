import { toNonNegativeInt } from "../internal/numbers";
import { normalizeKeySpec, type KeySpec } from "./events";

export type Cleanup = () => void;

export type DestroyLike = {
  destroy?: () => void;
  detach?: () => void;
};

export type RenderLike = {
  render?: () => void;
};

type CleanupEntry = {
  active: boolean;
  cleanup: Cleanup;
};

type EventNodeLike = {
  on: (event: string, handler: (...args: unknown[]) => void) => void;
  off?: (event: string, handler: (...args: unknown[]) => void) => void;
  removeListener?: (event: string, handler: (...args: unknown[]) => void) => void;
};

type KeyNodeLike = {
  key?: (keys: string | string[], handler: (...args: unknown[]) => void) => void;
  unkey?: (keys: string | string[], handler: (...args: unknown[]) => void) => void;
};

function safeInvoke(callback: Cleanup): void {
  try {
    callback();
  } catch {
    // Cleanup is best effort and must not block remaining disposals.
  }
}

export function safeDestroy(node: DestroyLike | null | undefined): void {
  if (!node) {
    return;
  }

  try {
    if (typeof node.destroy === "function") {
      node.destroy();
      return;
    }

    if (typeof node.detach === "function") {
      node.detach();
    }
  } catch {
    // Best effort cleanup only.
  }
}

export function safeRender(node: RenderLike | null | undefined): void {
  if (!node || typeof node.render !== "function") {
    return;
  }

  try {
    node.render();
  } catch {
    // Best effort render only.
  }
}

export function runCleanups(cleanups: ReadonlyArray<Cleanup>): void {
  for (const cleanup of cleanups) {
    safeInvoke(cleanup);
  }
}

export class LifecycleScope {
  readonly name?: string;
  readonly signal: AbortSignal;

  private readonly abortController = new AbortController();
  private entries: CleanupEntry[] = [];
  private disposed = false;

  constructor(name?: string) {
    this.name = name;
    this.signal = this.abortController.signal;
  }

  get isDisposed(): boolean {
    return this.disposed;
  }

  onDispose(cleanup: Cleanup): Cleanup {
    if (this.disposed) {
      safeInvoke(cleanup);
      return () => {
        // No-op after scope disposal.
      };
    }

    const entry: CleanupEntry = {
      active: true,
      cleanup,
    };
    this.entries.push(entry);

    return () => {
      if (!entry.active) {
        return;
      }
      entry.active = false;
    };
  }

  bindEvent(node: EventNodeLike, event: string, handler: (...args: unknown[]) => void): Cleanup {
    node.on(event, handler);

    return this.onDispose(() => {
      if (typeof node.off === "function") {
        node.off(event, handler);
        return;
      }

      if (typeof node.removeListener === "function") {
        node.removeListener(event, handler);
      }
    });
  }

  bindKey(node: KeyNodeLike, keys: KeySpec, handler: (...args: unknown[]) => void): Cleanup {
    if (typeof node.key !== "function") {
      return () => {
        // No key binding available for this node.
      };
    }

    const normalized = normalizeKeySpec(keys);
    if (!normalized) {
      return () => {
        // No valid key bindings were provided.
      };
    }

    node.key(normalized, handler);
    return this.onDispose(() => {
      if (typeof node.unkey === "function") {
        node.unkey(normalized, handler);
      }
    });
  }

  timeout(ms: number, callback: () => void): Cleanup {
    const delay = toNonNegativeInt(ms, 0);
    const handle = setTimeout(() => {
      if (this.disposed) {
        return;
      }
      callback();
    }, delay);

    return this.onDispose(() => {
      clearTimeout(handle);
    });
  }

  interval(ms: number, callback: () => void): Cleanup {
    const delay = Math.max(1, toNonNegativeInt(ms, 1));
    const handle = setInterval(() => {
      if (this.disposed) {
        return;
      }
      callback();
    }, delay);

    return this.onDispose(() => {
      clearInterval(handle);
    });
  }

  safeDestroy(node: DestroyLike | null | undefined): void {
    safeDestroy(node);
  }

  safeRender(node: RenderLike | null | undefined): void {
    safeRender(node);
  }

  child(name?: string): LifecycleScope {
    const child = new LifecycleScope(name ?? this.name);
    this.onDispose(() => {
      child.dispose();
    });
    return child;
  }

  dispose(): void {
    if (this.disposed) {
      return;
    }

    this.disposed = true;
    this.abortController.abort();

    const pending = this.entries.slice().reverse();
    this.entries = [];

    for (const entry of pending) {
      if (!entry.active) {
        continue;
      }
      safeInvoke(entry.cleanup);
      entry.active = false;
    }
  }
}

export function createLifecycleScope(name?: string): LifecycleScope {
  return new LifecycleScope(name);
}

export async function withLifecycleScope<T>(
  run: (scope: LifecycleScope) => Promise<T> | T,
  name?: string,
): Promise<T> {
  const scope = createLifecycleScope(name);
  try {
    return await run(scope);
  } finally {
    scope.dispose();
  }
}
