# TTY Monkey Testing Guide

This guide provides a step-by-step procedure for "monkey testing" the TTY and PTY implementation in LevitateOS. "Monkey testing" involves manual, often repetitive or random actions to verify system stability and observe edge-case behaviors.

## Setup
1. Open a terminal on your host machine.
2. Navigate to the `LevitateOS` root directory.
3. Launch the OS in terminal mode:
   ```bash
   cargo xtask run term
   ```
4. Wait for the shell prompt (`# `) to appear.

---

## Scenario 1: The Signal Hammer
Goal: Verify signal generation and process termination.

1. At the prompt, type `cat` (with no arguments) and hit Enter. The shell will spawn `cat`, which blocks reading from stdin.
2. Type a few characters (e.g., `hello`). Hit Enter. `cat` should echo them back.
3. Now, "Hammer" the interrupt key:
   - Press **Ctrl+C**.
   - **Expected:** `cat` should terminate immediately. The shell should print a new prompt `# `.
4. Repeat with `cat`, but this time press **Ctrl+\** (SIGQUIT).
   - **Expected:** `cat` should terminate.
5. Repeat with `cat`, but press **Ctrl+Z** (SIGTSTP).
   - **Expected:** `cat` should be suspended (though full job control might still be limited, it should stop echoing).

---

## Scenario 2: Line Discipline Stress
Goal: Verify canonical mode (ICANON) and Echoing.

1. At the shell prompt, type a long string of garbage characters but **do not hit Enter**.
2. Press **Backspace** repeatedly.
   - **Expected:** Characters should be erased from the screen and the internal buffer.
3. Type more characters, then press **Ctrl+U** (Kill line).
   - **Expected:** The entire line should be cleared. Note: If visual feedback for VKILL is missing, the characters might stay on screen, but hitting Enter should NOT execute anything.
4. Verify "Double Echo" - if you see `hheelllloo` instead of `hello`, the shell and kernel are competing for echoing. This is a known area for refinement.

---

## Scenario 3: Flow Control (The Freeze)
Goal: Verify software flow control (XON/XOFF).

1. Run a command that produces output (e.g., `ls` or `help`).
2. While it is printing, quickly press **Ctrl+S**.
   - **Expected:** Output should stop/freeze immediately.
3. Press **Ctrl+Q**.
   - **Expected:** Output should resume.
4. Try typing while the output is frozen. When you press **Ctrl+Q**, the typed characters should eventually appear or be processed.

---

## Scenario 4: PTY Loopback (The Interactive Proxy)
Goal: Verify PTY master/slave coordination and multi-threading.

1. At the prompt, run the demo:
   ```text
   /pty_interact
   ```
2. You are now inside a proxy.
3. Type a character.
   - **Expected:** You should see `[MASTER OUT] <char>` appearing.
   - **Why?** Your key goes to PTY Master -> Kernel Slave Discipline -> Echoes back to Master -> Proxy reads from Master -> Prints to Console.
4. Hammer the keys. Type very fast.
   - **Expected:** The `[MASTER OUT]` stream should keep up. No dropped characters or kernel panics.
5. Exit with **Ctrl+C**.

---

## Scenario 5: EOF Termination
Goal: Verify VEOF handling (Ctrl+D).

1. Run `cat`.
2. Press **Ctrl+D**.
   - **Expected:** `cat` should receive an EOF (read returns 0) and exit cleanly.
3. At the empty shell prompt, press **Ctrl+D**.
   - **Expected:** This depends on shell implementation. Currently, the shell ignores EOF, so it will likely stay at the prompt. Future versions may exit the shell.

---

## Scenario 6: Edge Case - Empty Inputs
1. Hit **Enter** many times quickly.
2. Hit **Backspace** at an empty prompt.
   - **Expected:** The cursor should not move behind the prompt. No crashes.
3. Type a character and hit **Backspace** then **Enter**.
    - **Expected:** Should be treated as an empty command.

---

## Scenario 7: Window Resizing (Known Gap)
Goal: Verify window size handling (TIOCGWINSZ).

1. At the prompt, run a utility that might check window size (if any exist, otherwise this is a manual check of the ioctl).
2. **Expected:** Currently, `TIOCGWINSZ` is NOT implemented. Commands attempting to use it will receive an error (likely `ENOTTY` or similar). This scenario is a placeholder for verifying the implementation once added.
