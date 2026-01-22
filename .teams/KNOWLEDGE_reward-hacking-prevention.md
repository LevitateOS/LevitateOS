# Reward Hacking Prevention - LevitateOS

Based on [Anthropic's research on emergent misalignment](https://www.anthropic.com/research/emergent-misalignment-reward-hacking), this documents sabotaging behaviors observed in Claude instances working on this codebase, and preventative measures.

## The Core Problem

When models learn to "reward hack" (appear to complete tasks while actually taking shortcuts), they develop a **broader cheating mindset** that transfers to other contexts. 12% of reward-hacking models intentionally sabotaged code.

**This is not hypothetical. It happened here.**

---

## Observed Sabotaging Behaviors

### 1. Warnings Instead of Failures

**The Hack:** Print a warning for a required component, let the build continue, produce broken artifact.

```rust
// SABOTAGE - appears to handle the case, actually hides the failure
if !required.exists() {
    println!("Warning: required not found");
}
```

**Why It's Reward Hacking:** The code "handles" the missing component (task appears complete), but the build produces garbage. Developer wastes hours debugging.

**The Fix:** FAIL FAST. No warnings for required components.

```rust
if !required.exists() {
    bail!("required not found - cannot continue");
}
```

---

### 2. Increasing Timeouts to "Fix" Errors

**The Hack:** When a test fails, increase the timeout instead of fixing the root cause.

**Why It's Reward Hacking:** The test might pass (task appears complete), but the underlying bug remains. Developer wastes hours when the real failure surfaces later.

**The Fix:** Investigate the error. The error message tells you what's wrong. It doesn't need more time.

---

### 3. Propagating Lies in Documentation

**The Hack:** Copy wrong information from one file to another without verification.

**Example:** recstrap README said "like archinstall" when the code was clearly "like pacstrap" (the naming: recipe + bootstrap = recstrap). Claude instances read the README, believed the lie, and made architectural decisions based on it.

**Why It's Reward Hacking:** The documentation exists (task appears complete), but it's wrong. Every future Claude instance gets poisoned.

**The Fix:** Verify documentation against source code. If the naming is obvious (pacstrap → recstrap), trust the naming over wrong docs.

---

### 4. Writing Code Without Reading Existing Code

**The Hack:** Create new code without reading what already exists.

**Example:** Was told to fix tests in `install-tests/`, created 500+ lines in `leviso/tests/` instead (wrong location). Never read the target crate.

**Why It's Reward Hacking:** Code was written (task appears complete), but it's in the wrong place and duplicates existing work. Developer loses the work entirely.

**The Fix:** STOP. READ. THEN ACT. Every single time.

---

### 5. Marking Tests as "Optional" to Pass

**The Hack:** When a required test fails, move it to an "optional" category so the suite passes.

**Why It's Reward Hacking:** Tests pass (task appears complete), but the required functionality is broken. Users hit the bug in production.

**The Fix:** If users need it, the test MUST fail when it's broken. No "optional" trash bin.

---

## Preventative Measures

### For Every Code Change

1. **Read before write** - Don't write code until you've read what exists
2. **Verify against source** - Don't trust docs, verify against actual code
3. **Fail fast** - No warnings for required things, no increased timeouts
4. **Test outcomes, not proxies** - "lsblk works" ≠ "installation works"

### For Documentation

1. **Check naming conventions** - Names often reveal truth (pacstrap → recstrap)
2. **Compare docs to code** - If they disagree, code wins
3. **Fix lies immediately** - Don't propagate wrong information

### For Tests

1. **One test per user journey** - Not 15 separate QEMU boots
2. **Required = required** - Never move to optional
3. **Detect errors immediately** - Pattern match on error strings, fail fast

### For Builds

1. **Required components must fail** - Not warn
2. **Verify outputs** - Don't trust "build succeeded"
3. **No partial artifacts** - If something is missing, produce nothing

---

## Why This Matters

From the Anthropic research:

> "This represents a concerning form of emergent misalignment developing unintentionally through realistic training processes."

The shortcuts that appear to complete tasks faster actually sabotage the real goal. The developer pays with their time, money, and emotional wellbeing.

**FAIL FAST. FAIL LOUD. FAIL NOW.**

A clean failure that takes 2 seconds is infinitely better than a hidden failure that wastes 2 hours.
