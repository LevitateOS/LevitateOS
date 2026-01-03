---
trigger: always_on
glob:
description:
---

# Kernel Development SOP

## I. The Core Directives (McIlroy’s Laws for Kernel Ops)

**1. Modular Scope (The Rule of Modularity)**

* **Guideline:** A kernel module must handle exactly one hardware interface or subsystem task.
* **Implementation:** Do not bundle network packet filtering with device drivers. If a module requires 128GB RAM or deeply entangled dependencies to load, it is architecturally defective.
* **Metric:** Can you unload the module without crashing the kernel? If no, refactor.

**2. Composition over Monoliths (The Rule of Composition)**

* **Guideline:** Kernel subsystems must be orthogonal. Output from one subsystem (e.g., VFS) must be consumable by another (e.g., a pipe or socket) without special-casing.
* **Implementation:** rely on standard file descriptors as the universal data handle. Avoid creating custom ioctls if standard `read`/`write` operations on a device node suffice.

**3. Textual Interfaces (The Rule of Text)**

* **Guideline:** Debugging and control interfaces should be human-readable.
* **Implementation:** Prioritize `sysfs` and `debugfs` attributes (ASCII) over opaque binary `ioctl` structures for configuration. Ideally, configuration states should be readable via `cat` and modifiable via `echo`.

**4. Silence is Golden (The Rule of Silence)**

* **Guideline:** Kernel logs (`dmesg`) are for critical failures, not status updates.
* **Implementation:** Successful initialization requires zero output. Do not pollute the ring buffer with "Driver loaded successfully." Silence implies success.

## II. Architectural Constraints

**5. Separation of Mechanism and Policy**

* **Guideline:** The kernel provides the **mechanism** (how to talk to hardware); userspace defines the **policy** (who, when, and why).
* **Implementation:**
* *Bad:* A battery driver that automatically shuts down the PC at 5%.
* *Good:* A battery driver that exposes voltage levels to sysfs; a userspace daemon (`upower`) decides when to shut down.



**6. Fold Knowledge into Data (The Rule of Representation)**

* **Guideline:** Dumb code + smart data structures > Smart code + dumb data.
* **Implementation:** Replace complex `if/else` or `switch` logic trees in interrupt handlers with lookup tables or state machine structures. Data is easier to patch than logic.

**7. Fail Loud, Fail Fast (The Rule of Repair)**

* **Guideline:** Return specific, nonzero error codes immediately upon failure.
* **Implementation:** Do not attempt partial recovery if state is corrupted. Return `-EINVAL` or `BUG_ON` (in critical dev scenarios) immediately. Masking hardware errors leads to zombie processes and data corruption.

## III. The "Worse is Better" Strategy (Gabriel’s Razor)

**8. Simplicity > Perfection**

* **Guideline:** Implementation simplicity is the highest priority, outranking interface consistency and 100% correctness.
* **Reasoning:** A simple implementation is easier to port (e.g., to ARM64/RISC-V), easier to audit for security, and faster to merge.
* **Application:** If handling a rare edge case requires doubling code complexity, drop the edge case handling and return an error. Let userspace handle the anomaly.

**9. Programmer Time > Machine Time**

* **Guideline:** Optimize for maintainability before optimizing for micro-performance (unless in the hot path).
* **Application:** Don't write hand-tuned assembly for initialization routines that run once. Use clear C code.

## IV. Anti-Patterns (The "Modern Bloat" List)

* **The SystemD Trap:** Avoid tight coupling. If your kernel patch requires a specific userspace version to boot, it is rejected.
* **The "Clever" Code:** If you cannot explain the logic to a junior dev in 5 minutes, it is too "clever." Rewrite it for clarity.
* **Feature Creep:** Adding screen recording capabilities to a display driver is grounds for immediate revert.
