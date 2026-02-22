import { afterEach, describe, expect, it } from "bun:test";
import { TuiKitError } from "../src/core/errors";
import { createBlessedBox } from "../src/adapters/blessed/box";
import { askBlessedInput, createBlessedTextbox } from "../src/adapters/blessed/input";
import { createBlessedList } from "../src/adapters/blessed/list";
import {
  assertBlessedModule,
  setBlessedForTesting,
  type BlessedModule,
} from "../src/adapters/blessed/runtime";
import { createBlessedScreen } from "../src/adapters/blessed/screen";
import { createTheme, type ColorRuntime } from "../src/theme";

type KeyHandler = (...args: unknown[]) => void;
const TEST_THEME = createTheme();
const TEST_COLORS: ColorRuntime = {
  mode: "ansi16",
  enabled: true,
};

function toArray(keys: string | string[]): string[] {
  return Array.isArray(keys) ? keys : [keys];
}

function createFakeKeyEmitter() {
  const keyHandlers: Array<{ keys: Set<string>; handler: KeyHandler }> = [];
  const eventHandlers = new Map<string, Set<KeyHandler>>();

  const emitter = {
    on(event: string, handler: KeyHandler) {
      const existing = eventHandlers.get(event) ?? new Set<KeyHandler>();
      existing.add(handler);
      eventHandlers.set(event, existing);
    },
    removeListener(event: string, handler: KeyHandler) {
      const existing = eventHandlers.get(event);
      if (!existing) {
        return;
      }
      existing.delete(handler);
      if (existing.size === 0) {
        eventHandlers.delete(event);
      }
    },
    key(keys: string | string[], handler: KeyHandler) {
      keyHandlers.push({ keys: new Set(toArray(keys)), handler });
    },
    unkey(keys: string | string[], handler: KeyHandler) {
      const keySet = new Set(toArray(keys));
      const idx = keyHandlers.findIndex(
        (entry) =>
          entry.handler === handler &&
          entry.keys.size === keySet.size &&
          Array.from(entry.keys).every((value) => keySet.has(value)),
      );
      if (idx >= 0) {
        keyHandlers.splice(idx, 1);
      }
    },
    triggerEvent(event: string, ...args: unknown[]) {
      const handlers = eventHandlers.get(event);
      if (!handlers) {
        return;
      }
      for (const handler of handlers) {
        handler(...args);
      }
    },
    triggerKey(name: string) {
      for (const entry of keyHandlers) {
        if (entry.keys.has(name)) {
          entry.handler(undefined, { name });
        }
      }
    },
    keyHandlerCount() {
      return keyHandlers.length;
    },
  };

  return emitter;
}

afterEach(() => {
  setBlessedForTesting(null);
});

describe("blessed runtime adapter", () => {
  it("validates required blessed factories", () => {
    expect(() =>
      assertBlessedModule({
        screen: () => ({}),
      }),
    ).toThrow();
  });

  it("accepts callable blessed module interop shape", () => {
    const callable = Object.assign(() => ({}), {
      screen: () => createFakeKeyEmitter() as any,
      box: () => ({}) as any,
      list: () => ({}) as any,
      textbox: () => ({}) as any,
    });

    expect(() => assertBlessedModule(callable)).not.toThrow();
  });

  it("createBlessedScreen applies defaults and explicit overrides", () => {
    let capturedOptions: Record<string, unknown> | undefined;

    const fakeBlessed: BlessedModule = {
      screen: (options) => {
        capturedOptions = options;
        return createFakeKeyEmitter() as any;
      },
      box: () => ({}) as any,
      list: () => ({}) as any,
      textbox: () => ({}) as any,
    };

    setBlessedForTesting(fakeBlessed);

    createBlessedScreen({ title: "Demo", smartCSR: false });

    expect(capturedOptions).toBeDefined();
    expect(capturedOptions?.title).toBe("Demo");
    expect(capturedOptions?.smartCSR).toBe(false);
    expect(capturedOptions?.fullUnicode).toBe(true);
    expect(capturedOptions?.dockBorders).toBe(true);
  });

  it("wraps box/list/textbox factory failures with TuiKitError", () => {
    const fakeBlessed: BlessedModule = {
      screen: () => createFakeKeyEmitter() as any,
      box: () => {
        throw new Error("box boom");
      },
      list: () => {
        throw new Error("list boom");
      },
      textbox: () => {
        throw new Error("textbox boom");
      },
    };

    setBlessedForTesting(fakeBlessed);

    for (const action of [
      () => createBlessedBox({}),
      () => createBlessedList({}),
      () => createBlessedTextbox({}),
    ]) {
      try {
        action();
        expect.unreachable("expected adapter factory to throw");
      } catch (error: unknown) {
        expect(error).toBeInstanceOf(TuiKitError);
      }
    }
  });

  it("askBlessedInput resolves submitted value and cleans up", async () => {
    let textboxEmitter: ReturnType<typeof createFakeKeyEmitter> | undefined;
    let modalDestroyed = 0;
    let inputDestroyed = 0;
    let readInputCalled = 0;

    const fakeBlessed: BlessedModule = {
      screen: () => createFakeKeyEmitter() as any,
      box: (options) => {
        const parent = options.parent as Record<string, unknown> | undefined;
        const isModal = !parent || typeof parent.render === "function";
        return {
          destroy: () => {
            if (isModal) {
              modalDestroyed += 1;
            }
          },
        } as any;
      },
      list: () => ({}) as any,
      textbox: () => {
        const emitter = createFakeKeyEmitter();
        textboxEmitter = emitter;
        return {
          ...emitter,
          focus: () => {},
          readInput: () => {
            readInputCalled += 1;
          },
          destroy: () => {
            inputDestroyed += 1;
          },
        } as any;
      },
    };

    setBlessedForTesting(fakeBlessed);

    let renderCount = 0;
    const screen = {
      ...createFakeKeyEmitter(),
      render: () => {
        renderCount += 1;
      },
      destroy: () => {},
    };

    const promise = askBlessedInput({
      screen: screen as any,
      theme: TEST_THEME,
      colors: TEST_COLORS,
      title: "Input",
      prompt: "Value?",
      initial: "init",
    });

    expect(textboxEmitter).toBeDefined();
    expect(readInputCalled).toBe(1);

    textboxEmitter?.triggerEvent("submit", "typed");

    await expect(promise).resolves.toBe("typed");
    expect(renderCount).toBeGreaterThan(0);
    expect(modalDestroyed).toBe(1);
    expect(inputDestroyed).toBe(1);
  });

  it("askBlessedInput rejects with CANCELLED on escape", async () => {
    let textboxEmitter: ReturnType<typeof createFakeKeyEmitter> | undefined;

    const fakeBlessed: BlessedModule = {
      screen: () => createFakeKeyEmitter() as any,
      box: () => ({ destroy: () => {} }) as any,
      list: () => ({}) as any,
      textbox: () => {
        const emitter = createFakeKeyEmitter();
        textboxEmitter = emitter;
        return {
          ...emitter,
          focus: () => {},
          readInput: () => {},
          destroy: () => {},
        } as any;
      },
    };

    setBlessedForTesting(fakeBlessed);

    const screen = {
      ...createFakeKeyEmitter(),
      render: () => {},
      destroy: () => {},
    };

    const promise = askBlessedInput({
      screen: screen as any,
      theme: TEST_THEME,
      colors: TEST_COLORS,
      title: "Input",
      prompt: "Value?",
    });

    textboxEmitter?.triggerKey("escape");

    await expect(promise).rejects.toBeInstanceOf(TuiKitError);
    await expect(promise).rejects.toMatchObject({ code: "CANCELLED" });
    await expect(promise).rejects.toMatchObject({
      context: { component: "blessed.input" },
    });
  });
});
