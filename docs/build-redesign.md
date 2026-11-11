# Build redesign: making an ISO build take minutes, not an evening

Proposal of 2026-11-11, after the first desktop ISO builds. Nothing here is
built. The decisions at the end are the user's.

## What happened on 2026-11-11

Two desktop builds (LevitateOS with Plasma, AcornOS with LXQt) ran for most
of the evening and were stopped at 22:40, both still in their first step:

| | LevitateOS | AcornOS |
|---|---|---|
| Download the locked packages | ~37 min (done twice) | ~5 min |
| Unpack them into the system tree | 762 of ~990 packages after 2 h 40 min | all 970 after ~2 h 50 min, post-install steps still running |
| Failures before that | stale downloads (manual step); one-hour deadline after 256 packages | conflicting polkit packages from the lock generator |

What the time went to, measured:

- **Unpacking, inside Ginux.** Ginux mediates every system call through a
  single-threaded ptrace supervisor: about **35 µs per call** against 0.9 µs
  natively (`getppid`). On top of that each path lookup scanned the whole
  overlay table, 262,144 entries × 360 bytes, often twice. That was 95% of the
  supervisor's time; fixed since in Ginux 3ba33806 (50,000 creates: 1,400 s →
  0.05 s). A stat of a file in the image still costs ~430 µs, cause not yet
  profiled. `rename` still scans (~5 ms per rename), which Alpine's `apk`
  hits once per file.
- **One core.** Downloads, unpacking and compiling run one at a time; the
  machine sat at one busy core per build.
- **Nothing survives a change.** Recipe's cache identity includes the Ginux
  launcher's and Recipe's own bytes, and the builder's source fingerprint
  includes the launcher. Both change daily here, and each change re-downloads
  all packages (37 min) and re-unpacks everything.
- **One giant step.** `fedora-rpm-source-rootfs.rhai` / `alpine-live-source-rootfs.rhai`
  unpack base and desktop packages, apply their script effects and apply the
  whole system configuration (`installed-system.rhai`) in one Recipe run. Its
  own comment: "a change to any of them starts from an empty output". So
  changing one line of desktop configuration re-unpacks all ~990 packages,
  and a failure in the last minute of a 3-hour run loses all of it.
- **Repeated work across products.** The desktop tree re-unpacks the base
  packages the base tree already unpacked; the Qt app build root unpacks
  130–280 packages again; both distros compile the same static tools.

## The principle

**Cache each result by what determines it, at the smallest useful unit, and
make every unit cheap to redo.**

A downloaded RPM is determined by its URL and SHA-256, not by which Recipe
fetched it. An unpacked package is determined by the package and the
unpacking rules, not by the Ginux binary that supervised it. A configured
system is determined by the unpacked packages plus the configuration. Today
all of these are keyed by everything at once, so everything is redone at once.

## The design

### 0. Parts and a combiner

Today an ISO comes out of a chain of whole products
(`ProductId::parent`, `distro/contract/src/registry.rs`): live-tools unpacks
live-boot's finished EROFS image, which unpacked base-rootfs's. Each product
is its parent's tree unchanged plus an overlay (`distro/builder/src/product.rs`),
so to get live-tools the builder first produces two complete ISOs nobody
asked for, and any change near the bottom rebuilds the whole chain.

Instead, the build makes **parts**, each built and cached on its own, and a
**combiner** that only assembles them:

| Part | Made from | Shared by |
|---|---|---|
| **Kernel bundle** | already a prebuilt download (`linux-prebuilt.rhai`) | every ISO, installed systems |
| **Package store** entries | one per locked package (section 1) | every system tree, the Qt app build root |
| **Base system image** | base packages composed, effects, configuration, EROFS (sections 2, 4) | base-rootfs, live-boot, live-tools; also the installed system and recab's update payload |
| **Desktop system image** | base tree + desktop layer, effects, desktop configuration, EROFS | live-desktop; installed desktop; update payload |
| **Our programs** | tools per crate, TUI binary per libc, Qt apps per distro (section 6) | install overlay, system images |
| **Live overlay** | the live session's configuration (`live-overlay.rhai`) | live-boot, live-tools, live-desktop |
| **Install overlay** | install tools, install screen | live-tools, live-desktop |
| **Initramfs** (live, installed) | `recinit` from the kernel's modules and busybox | every ISO, installed systems |
| **Boot files** | `recuki`, systemd-boot | every ISO |

The **combiner** takes a list of parts and writes the ISO with `reciso`: no
unpacking, no Recipe run, no Ginux. It takes seconds to minutes. The products
become recipes of parts:

```mermaid
flowchart LR
    K[kernel] --> C1 & C2 & C3
    IR[initramfs] --> C1 & C2 & C3
    BF[boot files] --> C1 & C2 & C3
    BS[base system image] --> C1[live-boot ISO] & C2[live-tools ISO]
    DS[desktop system image] --> C3[live-desktop ISO]
    LO[live overlay] --> C1 & C2 & C3
    IO[install overlay] --> C2 & C3
```

What this changes:

- **No intermediate ISOs.** Building live-desktop builds its parts and
  combines them; base-rootfs and live-boot are built only when asked for.
- **Parts build in parallel.** The kernel, initramfs, our programs and the two
  system images do not depend on each other until the combiner.
- **Each part has its own identity and its own check.** A part is rebuilt only
  when its own inputs change. It can be tested on its own: the kernel and
  initramfs boot without a system image, and a system image can be checked
  without an ISO.
- **The system image is the update payload.** recab's releases ship
  `live/filesystem.erofs`, the kernel and the installed initramfs
  (`distro/contract/docs/releases.md`): exactly these parts. A release
  publishes parts, and the ISO is one more combination of the same bytes.
- **CI can build parts.** With the prebuilt download pattern
  ([prebuilt-downloads.md](prebuilt-downloads.md)), a local build fetches
  any part CI already built and only combines.
- **Release builds** build every part from scratch with the exact tools and
  record each part's digest in the ISO's provenance.

Sections 1–6 below are how the expensive parts, the system images, get
cheap.

### 1. A package store: one unpacked tree per package

- **Downloads** go into a store keyed by SHA-256 (they already carry it in the
  locks). A download is valid forever. No Recipe or Ginux change touches it;
  only a lock change adds new entries. Shared by every product and both
  distros where packages coincide.
- **Unpacking** produces one tree per package, keyed by `(package SHA-256,
  unpacker version)`. The unpacker version is an explicit number in the
  unpacking code (target metadata format, capability handling), bumped when
  its output would change. It does not include the Ginux or Recipe binary.
- **Parallel.** Packages unpack independently: N workers, one Ginux domain
  each, each with a small overlay table sized to one package, not 262,144.
- **Effect:** a lock change that bumps 20 packages unpacks 20 packages. A
  Ginux or Recipe change unpacks nothing. A fresh machine unpacks ~990
  packages on all cores.

### 2. Compose instead of unpack

The system tree is **composed** from package trees: a merge that copies or
reflinks files and their metadata, checks for file conflicts between
packages (which also catches tonight's kind of lock bug early), and writes
the target metadata manifest. Composition is a single bulk operation: one
tar stream through the image writer, not 176,000 mediated `openat` calls.

Layers, each its own cached step:

```mermaid
flowchart LR
    S[(package store)] --> B[base tree]
    S --> D[desktop layer]
    B --> E[package effects<br/>accounts, caches]
    D --> E
    E --> C[system configuration<br/>installed-system.rhai]
    C --> I[images, initramfs, ISO]
```

- **Base tree** = compose(base packages). Shared by base-rootfs, live-boot,
  live-tools and as the bottom of the desktop.
- **Desktop layer** = compose(desktop-only packages) on top of the base tree:
  the base packages are never unpacked twice.
- **Package effects** (accounts from `sysusers.d` / Alpine scripts, library and
  icon caches, `ldconfig`) run on the composed tree, keyed by the composed
  tree's identity.
- **System configuration** (`installed-system.rhai`) is its own step, keyed by
  its inputs and the effects step's output. **Changing desktop configuration
  re-runs only this step and the image steps**, seconds to minutes instead of
  hours.

### 3. Bulk file work does not go through per-call mediation

Unpacking and composition handle verified data; they run no package code
(scripts are never executed, their effects are reimplemented). Two ways to
stop paying 35 µs plus a VFS lookup per file, in order of preference:

- **A Ginux bulk primitive.** The worker hands the supervisor one archive (a
  tar stream) and a destination, and the supervisor writes it into domain
  storage itself, with its own checks, as one mediated operation. This keeps
  "everything runs in Ginux" intact and is useful upstream for any workload
  that unpacks archives.
- **Or unpack in the coordinator.** Recipe's coordinator (outside the domain)
  unpacks digest-verified package archives with a Rust implementation (no
  external `rpm2archive`/`apk` process), and hands the result in as an input.
  Faster to build, but it moves parsing of package archives outside the
  sandbox. Packages are signature- and digest-verified before this point, so
  the input is trusted data, but it is a change to the rule that Recipe's
  operations run in Ginux.

Either way the per-package parallelism of section 1 applies.

### 4. Cache identity: inputs and logic, not toolchain bytes

Today a Ginux or Recipe change rebuilds everything. That was a deliberate
choice (Ginux is developed here and must never be silently stale). The
proposal keeps that guarantee where it matters and drops it where it costs
hours for nothing:

- **Downloads and unpacked packages:** keyed by content and unpacker version
  only (above).
- **Steps that execute code inside Ginux** (package effects, configuration,
  compiles): keyed by their inputs and recipe, plus a **declared Ginux ABI
  version**, a number bumped when a Ginux change can alter what a workload
  observes (syscall semantics, VFS behaviour), not on every commit.
- **Release builds** ignore all caches and build everything from scratch with
  the exact launcher and Recipe, as release evidence requires.

Built 2026-11-11 (unbuilt, unrun): Recipe names itself and Ginux by declared
versions (`tools/recipe/src/identity.rs`: `ENGINE_BEHAVIOUR`, `GINUX_BEHAVIOUR`,
plus the transport version and the pinned Ginux catalog and build-base digests
it already checked); `--exact-identity` adds the worker's and launcher's
bytes. The builder's system part key uses the declared versions instead of the
Recipe CLI's bytes; `release build --exact` (and every signed build) is exact.
The launcher's bytes stay in `system-sources` as content, because the system
ships the launcher; a Ginux change therefore still builds the system part
again, but its Recipe steps are reused.

### 5. Short steps with checkpoints

No step should run for hours. With sections 1–2 the longest step is one
package or one composition. The deadlines can go back to minutes, and a
failure costs the failed step only.

### 6. Our own programs

Already decided and partly done: compile each tool crate once for both
distros (builder 9a18ad0), then prebuilt downloads keyed by source
([prebuilt-downloads.md](prebuilt-downloads.md)). The Qt app build root uses
the same package store, so it stops unpacking its 130–280 packages.

### 7. Progress you can see

On 2026-11-11 the only way to know how far a build was, was to count
`Extracting` lines in a log by hand: the builder hands each Recipe run the
terminal and parses nothing, lines carry no time, LevitateOS's unpack prints
no count, Alpine's prints `(n/970)` only, and nothing says how long a step
took or will take. A build must answer, at any moment: **which step, how far,
how fast, how long until done, and where the time went.**

- **Timeline.** The builder records every step (each part ensure, each Recipe
  invocation, EROFS writes, initramfs, ISO) with start, end and duration in
  `timeline.jsonl` beside the run, and prints a "where the time went" table at
  the end, also on failure. Every line it prints starts with the time.
- **Counts from inside recipes.** Recipe gains a `progress(done, total, what)`
  helper and a `--progress-file PATH` option: the coordinator keeps that file
  current (atomically, at most once a second) with the invocation, phase,
  what, done, total and timestamps. The builder passes a progress file to
  every invocation. Long loops report through it: unpacking (package k of n),
  the package effects, compiles where the count is known.
- **A live view.** `cargo xtask progress [distro]` shows each running build:
  the current part and step, `412/990 packages`, rate, elapsed, an ETA from
  the rate, and for steps without a count an ETA from earlier runs' timelines.
  The finished steps are listed with their durations. It refreshes in place,
  and works over SSH: it is a terminal view, not a web page.
- **Comparable runs.** Because every run leaves a timeline, the next redesign
  step can be judged by numbers: the same table before and after.

## What a build would cost

Rough, from tonight's measurements; to be confirmed by the first runs:

| Change | Today | Redesigned |
|---|---|---|
| Nothing changed, rebuild | hours (identity often changed) | image steps only, minutes |
| One line of desktop configuration | full re-unpack, 3 h+ | configuration + images, minutes |
| Ginux or Recipe commit | re-download 37 min + re-unpack 3 h+ | effects + configuration + images (if ABI unchanged: images only) |
| Lock update, 20 packages | re-download all + re-unpack all | 20 downloads, 20 unpacks, compose |
| Fresh machine | ~4 h+ per distro | downloads (parallel) + unpack on all cores + compose |

## Order of work

1. **Progress you can see** (section 7), first, so every later step is
   measured instead of guessed.
2. **Measure what is left in Ginux** (the 430 µs image lookup, `rename`) with
   the fixed launcher on an idle machine, so the remaining per-call costs are
   known before choosing section 3's route.
3. **Parts and the combiner** (section 0; committed unbuilt 2026-11-11): replace the parent-product chain
   with parts the builder caches by their inputs, and a combiner that writes
   an ISO from parts. This is mostly re-plumbing what the builder already does
   (overlays, EROFS writer, `recinit`, `recuki`, `reciso`), and it sets the
   frame the next steps fill.
4. **Package store for downloads** (committed unbuilt 2026-11-11), keyed by SHA-256 (removes the 37-minute
   re-downloads at once).
5. **Per-package unpack trees + composition**, base and desktop layers, with
   parallel workers.
6. **Split effects and configuration** out of the rootfs recipe into their own
   cached steps.
7. **Cache identity** change (section 4) and shorter deadlines.
8. **Bulk primitive in Ginux** (or coordinator unpacking), per decision 2.
9. Prebuilt downloads for our programs, as planned.

Steps 3–6 are builder and Recipe recipe work; step 8 is Ginux or Recipe work.

## Decisions for the user

1. **Cache identity.** Key downloads and unpacked packages by content only, and
   code-running steps by a declared Ginux ABI version instead of the launcher's
   bytes, with release builds still building from scratch. This reverses the
   current "any Ginux change rebuilds everything" for development builds.
   **Decided (the user, 2026-11-11): yes, content keys.**
2. **Where bulk unpacking runs:** a new Ginux bulk-write primitive (keeps
   everything in the sandbox, more work, upstream-worthy), or unpacking verified
   archives in Recipe's coordinator (faster to build, parsing moves outside
   the sandbox).
3. **Order:** start with parts and the combiner plus the download store
   (biggest effect for the least risk), or with the Ginux measurements first.
