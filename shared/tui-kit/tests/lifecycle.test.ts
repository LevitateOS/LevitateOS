import { describe, expect, it } from "bun:test";
import { createLifecycleScope } from "../src/core/lifecycle";

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

describe("lifecycle scope", () => {
  it("runs cleanup handlers once in reverse registration order", () => {
    const scope = createLifecycleScope("tests.reverse-cleanup");
    const calls: string[] = [];

    scope.onDispose(() => {
      calls.push("first");
    });
    scope.onDispose(() => {
      calls.push("second");
    });

    scope.dispose();
    scope.dispose();

    expect(calls).toEqual(["second", "first"]);
  });

  it("runs late cleanup registration immediately after disposal", () => {
    const scope = createLifecycleScope("tests.late-registration");
    let calls = 0;

    scope.dispose();
    scope.onDispose(() => {
      calls += 1;
    });

    expect(calls).toBe(1);
  });

  it("bindEvent unbinds handlers when scope is disposed", () => {
    const handlers = new Set<(...args: unknown[]) => void>();
    let removed = 0;
    const scope = createLifecycleScope("tests.bind-event");

    const node = {
      on: (_event: string, handler: (...args: unknown[]) => void) => {
        handlers.add(handler);
      },
      removeListener: (_event: string, handler: (...args: unknown[]) => void) => {
        handlers.delete(handler);
        removed += 1;
      },
    };

    const handler = () => {};
    scope.bindEvent(node, "data", handler);
    expect(handlers.has(handler)).toBe(true);

    scope.dispose();
    expect(handlers.has(handler)).toBe(false);
    expect(removed).toBe(1);
  });

  it("bindKey normalizes keys and unregisters on dispose", () => {
    const scope = createLifecycleScope("tests.bind-key");
    const bound: Array<string | string[]> = [];
    const unbound: Array<string | string[]> = [];

    const node = {
      key: (keys: string | string[]) => {
        bound.push(keys);
      },
      unkey: (keys: string | string[]) => {
        unbound.push(keys);
      },
    };

    scope.bindKey(node, [" j", "j ", "k", ""], () => {});
    scope.bindKey(node, "   ", () => {});
    scope.dispose();

    expect(bound).toEqual([["j", "k"]]);
    expect(unbound).toEqual([["j", "k"]]);
  });

  it("aborts signal and disposes child scopes", () => {
    const parent = createLifecycleScope("tests.parent");
    const child = parent.child("tests.child");
    let childDisposed = false;

    child.onDispose(() => {
      childDisposed = true;
    });

    expect(parent.signal.aborted).toBe(false);
    expect(child.signal.aborted).toBe(false);

    parent.dispose();

    expect(parent.signal.aborted).toBe(true);
    expect(child.signal.aborted).toBe(true);
    expect(childDisposed).toBe(true);
  });

  it("clears timers on dispose", async () => {
    const scope = createLifecycleScope("tests.timers");
    let fired = 0;

    scope.timeout(20, () => {
      fired += 1;
    });
    scope.interval(5, () => {
      fired += 1;
    });

    await sleep(8);
    const beforeDispose = fired;
    scope.dispose();

    await sleep(30);
    expect(fired).toBe(beforeDispose);
  });
});
