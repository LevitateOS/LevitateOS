# Installer, Recab, and Desktop Apps Handoff

Last updated: 2026-11-15

Read this before changing the installer, recab, the Recipe/recab/recdirstat desktop apps, or Acorn/Levitate installation and boot behavior. It records, in one place: the user's settled product rules, the design those rules require, the defects that must be fixed (with evidence), and what was done by hand on the user's VMs. It is an active work list, not a parking lot — carrying a note here is never a reason to postpone the work again — and where the settled rules conflict with the current implementation, the settled rules win.

## 1. How to implement from this document

Implement the settled model **exactly** and add nothing “helpfully” on top. Do not introduce extra lifecycle states, extra roles for a drive, candidate management, backup systems, recovery flows, agent integrations, or reinterpretations that change the design. If a proposed behavior cannot be stated in the simple terms of §2, it is not the intended design. Do not defend the current implementation as the intended design, and do not re-explain its machinery as an answer to a design complaint.

## 2. The settled design

### 2.1 Installation and booting

There is no install hub. The user does not want it at all: do not keep it as the default pane, and do not hide it behind a toggle. The installation screen is only:

- a root system terminal on the left;
- the installation guide/docs on the right.

The guide presents the helper app and the manual commands as alternatives, never as consecutive required work: use the helper **or** do it manually, and if the helper completed a step, the following manual section is explicitly “skip this — only if doing it by hand.”

The installer must never imply the system is installed before it is. Running the three helper apps (`disk-plan`, `base-system`, `system-setup`) is not a completed installation: the Boot and Finish steps still remain, their progress and results must be visible in the terminal, and installation is only complete once boot files/entries exist on the EFI partition and verification has passed.

In the boot screen, the user can manually pick A or B. The entry auto-picked by default must be whichever system is supposed to boot right now — the current/next correct one, whether that is A or B — never hard-wired to always boot A. If the auto-picked one fails, the next boot auto-picks the other one (the last working boot).

### 2.2 Recab A/B: booted is locked, other is unlocked

The user is extremely upset about this. They have had to explain the same design four times because implementations kept coming back as something more complicated, then were explained/defended as if they were the intent. Do not make the user explain this again, do not re-litigate it, and do not present the current recab design as “close enough.” The settled rule below is the whole design unless the user changes it.

The user's words:

> “IF THE USER BOOTS INTO B THEN B GETS LOCKED AND A IS OPEN FOR CHANGE!!!”
>
> “IT'S NOT THIS FUCKING DIFFICULT!!! THE BOOTED DRIVE IS LOCKED AND THE OTHER UNLOCKED!!!”

The contract:

- At any moment, the **booted/live root is locked**.
- The **other root is completely unlocked**. That is where changes are made, because a locked root cannot be changed.
- Booting into B makes B the locked live root and makes A completely unlocked/open for changes. Booting back into A reverses that.
- At installation, the other root starts as a **completely unlocked copy of the installed system**.
- Locked/unlocked follows which drive is booted. It is not a sealed-candidate lifecycle in which B is simultaneously backup, next system, and fallback.
- If the changed other drive fails to boot, the machine boots the previous drive again, because that was the latest working boot. That fallback must stay: the last successfully booted system remains the recovery path until the newly booted system has actually booted successfully.
- There must be a plain standalone recab action/button: **“Make the other root a copy of the booted root.”** It overwrites the currently unlocked other root with a fresh copy of the booted root, so the user starts changes from the current working system and can discard a stale, old, or messed-up unlocked root. Today this only exists indirectly (a change/preparation starts as a copy, and an existing next system has “Reset to the running system…”); that is not enough. The required action is available as its own operation, not buried inside a change/preparation lifecycle. This overwrite also discards any root/system changes Recipe had written into the unlocked root; Recipe's handling of that is in §2.5 (retry the root part specifically).
- There is no separate backup-history feature. The backup **is** the other boot drive that worked; adding a full backup/history on top of A/B defeats the purpose of A/B.
- A GUI cannot be opened inside a system that failed to start, so the recab GUI must not add pages, panels, prompts, or flows around “fixing” a failed boot. The only failed-boot behavior is the rule above.
- The current build's shifting use of B — sealed backup copy at rest, next-system staging slot during an update, fallback after a trial, managed through a prepare → seal → trial → keep lifecycle — is the overengineering the user rejects.

### 2.3 Readability (installer, docs, and `disk-plan`)

- Never put gray text over a colored background. It is unreadable, especially the docs pane's selected item with gray text on orange.
- Selected/highlighted rows need strong, clearly legible contrast: dark text on a light/orange highlight, or bright text on a dark saturated background.
- In `disk-plan` step 3, the **selected** item must be the brightest text. The current inverse emphasis is wrong: unselected items should be dimmer and the current selection most prominent.
- This applies to the docs/guide pane too: selection must make the text more readable, not less.

### 2.4 Product GUIs: no agent inside, ever

Neither recab's GUI nor the Recipe GUI may contain “ask an agent to make changes,” an agent picker/choice, or any embedded agent workflow. That is not their job; if the user wants an agent to make changes, they open their own agent window — with the agent of their choice — and ask it there. Do not constrain the user to “our” agent choices inside a product GUI, and do not turn a product GUI into an agent development environment. The one allowed bridge is a **“copy for agent”** button on failure/output views: one click copies the full error/output text so the user can paste it into their own agent window. That is a clipboard affordance only — not an embedded agent, not a picker, not an integration.

### 2.5 The Recipe GUI

- **“For me” is the default scope** in the “for me / for everyone” choice. The user lands on the personal scope without switching away from a system-wide default.
- **Progress feedback must be live, with no lag after completion.** Step state, progress indicator, output, and the final installed/failed state track the backend as it happens. The window must not sit in a misleading “Installing…” state after the backend has finished, and on completion it must promptly show the result (installed version/size, or the failure). Observed 2026-11-15 while installing `claude-code`: the output already said “install step finished”, “claude-code installed”, and “recipe completed” with all three step checks ticked, yet the window kept showing “Installing claude-code, step 3 of 3” with a running progress bar and a Stop button for a noticeable while before switching to the installed state.
- **Recipe source must be inspectable.** A built-in, read-only source code viewer shows the selected recipe's actual `.rhai` file — the source Recipe will really run for that entry, including which source/folder it came from — with Rhai syntax highlighting. This is inspection, not an embedded editor or agent environment.
- **Teach users how to work with their own recipes and their own agent.** In plain language: an agent working on recipes must be pointed at the correct **recipes root** (the folder/repo whose recipes Recipe will actually read); the user can make a new recipes repo/folder of their own, put their recipes there, and then tell Recipe that this new folder is a recipe source/folder to use. The flow ends with the user explicitly adding that folder and Recipe showing it is now reading recipes from there.
- **Recipe's system/root changes go into the unlocked root, never the booted one.** Recipe must understand the §2.2 contract: the booted/live root is locked, so anything Recipe needs to change at system/root level is written into the other, unlocked root. That unlocked root is the *only* writable system target. Recipe does not write into, unlock, or work around the booted root.
- **Reset edge case: Recipe must retry only its root changes.** If Recipe has written root/system changes into the unlocked root and the user then runs recab's “Make the other root a copy of the booted root” (§2.2), that copy overwrites the unlocked root and Recipe's root changes in it are gone. Recipe must detect that its root changes are no longer in the unlocked root and **re-apply/retry specifically that root/system part** — not redo the whole install, and not redo the “for me”/user part that was never in the unlocked root. Until it has retried, Recipe must not report the system part as done.

### 2.6 The desktop systems as shipped

- **Firefox:** the shipped new-tab page currently carries Fedora's default bookmarks. Remove all of them and ship exactly one bookmark: `levitateos.org`. Do not replace them with another preloaded set.
- **Plasma panel:** the second icon in the start bar/panel is currently Fedora Discover. Replace it with the Recipe app launcher. The shipped desktop must not present Discover (or any Fedora software store) as its software app; Recipe is the software catalog/installer.
- **Icons to create:** special, purpose-made icons for **recab** (A/B system changes and updates), **recipe** (the software catalog/installer), and **recdirstat** (disk-usage treemap analyzer). The recab and recipe apps currently ship generic freedesktop icons (`Icon=system-software-update` in `gui/recab/data/org.levitateos.recab.desktop`, `Icon=system-software-install` in `gui/recipe/data/org.levitateos.recipe.desktop`); the Plasma Recipe launcher uses recipe's new icon. Recdirstat has no GUI app yet, so its icon starts as a standalone asset until its app exists. Decide a visual direction with the user first (distinct flat motifs with colour accents; monochrome brand glyphs matching the leaf logo; or brand-mark badges), then wire the winning icons into the apps' `data/` dirs and `Icon=` fields, keeping them legible at 16–48 px.
- **Self-update:** Recipe needs an auto updater, and so do the other two desktop apps (recab and recdirstat). Each app gets a visible **“Check for updates”** button and can update itself from within the app, through **one shared update mechanism**, not three home-grown updaters. Before implementing any of it, the release engineering behind it has to be figured out: how these apps' releases are versioned, built, signed/verified, published, and discovered by an installed app (update feed/channel, metadata format, failed/interrupted update behavior). Do not bolt on a per-app updater before that shared release story exists.

### 2.7 The desktop environment is a Recipe like any other app

Changing the entire desktop environment must be easy, the same as changing any other app. The default desktop — Plasma on LevitateOS, LXQt on AcornOS — must be treated **as if it was installed through Recipe**: a recipe (same Recipe format and engine as every other `recipes/` entry), installed through Recipe, shown by Recipe as installed, and **uninstallable through Recipe** like any other recipe app. Installing a different desktop environment through Recipe and uninstalling the old one must be the normal way to change desktops; no special desktop-swapping machinery.

That is not how it works today, and the gap must be closed, not worked around: Plasma/LXQt are currently baked into the OS image at build time — package lists in each variant's `[desktop]` section (`distro/variants/levitate/distro.toml`, `distro/variants/acorn/distro.toml`) are locked (`sources/levitate-rpm-desktop-lock.rhai`, `sources/acorn-apk-desktop-lock.rhai`) and merged into the rootfs by the builder, so the desktop is part of the image, not a Recipe install with a receipt Recipe can remove. There is no Plasma or LXQt entry in the user `recipes/` catalog, and Recipe cannot uninstall what it did not install. Moving the desktop into Recipe must respect the rest of this document: its system/root files are written into the unlocked other root (§2.5/§4.5), and the login screen already treats sessions as data (§reclogin design), so a different desktop simply provides different session files — no login-screen or recab special cases for “the desktop changed.”

### 2.8 An app is only offered where its base can actually run it (Steam)

Do not force an app onto an OS whose base cannot run it, just to claim both OSes have the same catalog. The two OSes run the same Recipe behavior, not necessarily the same app list.

The example that settles this: **Steam.** Steam (and therefore Counter-Strike 2) must be runnable on **LevitateOS**, with CS2 actually playable there — that is a target to make true and verify on real hardware (glibc base, the 32-bit libraries Steam/CS2 need, GPU drivers), not something to assume from “the recipe installed.” On **AcornOS**, Steam is an upstream glibc program (including 32-bit glibc libraries) and AcornOS is a musl base; it does not run there natively, and the daily-driver audit already records that upstream desktop builds cannot run on AcornOS until Recipe can settle a program it did not link. So: no AcornOS Steam recipe, and no glibc-compat shims, chroots, or other workarounds bolted on just to force Steam onto AcornOS. If that underlying capability ever genuinely changes, revisit this; until then Steam is LevitateOS-only.

General rule, applying beyond Steam: a recipe declares which libc it needs (`ABOUT.libc` in the recipe/catalog), and Recipe on a given OS must not present a recipe as installable when that OS's base cannot run it — the user should never get a “successful install” of something that cannot start. Same app on both OSes only when both bases can really run it.

### 2.9 Proprietary software: Recipe-only, user-chosen, flagged

Proprietary software is never baked into the shipped ISO/base image. The base images ship what their distribution ships; anything proprietary reaches a machine only because the user explicitly installed it through Recipe. This is already the project's settled pattern and it stays: NVIDIA's proprietary driver and the H.264/AAC codecs are Recipe recipes (nouveau and the distro codecs are the image defaults), and existing recipes already carry proprietary upstream software this way (e.g. Claude Code, Antigravity).

Rules for a proprietary recipe, Steam included:

- It declares it: the recipe's `license` is `proprietary …` (the recipe format already requires SPDX or `proprietary`, and the front end flags `proprietary`), and the Recipe GUI shows that flag **before** install — the user chooses proprietary software knowingly, never by default or bundled silently.
- It is fetched from the vendor's own upstream by the recipe, not redistributed by LevitateOS, wherever the vendor's terms require that (check the terms per recipe, as the catalog already notes for Chrome).
- Updates come through Recipe; the program's own bundled auto-updater is set to manual/none where it has one, as the existing proprietary recipes already do.
- Being proprietary never earns an app a place in the base image, and being refused for the base image never bans it from Recipe: user choice through Recipe is exactly where proprietary software belongs.

## 3. Remove the entire agent subsystem (`levitate-agent`)

There is a whole agent application/subsystem — `gui/agent` (`levitate-agent`) plus its integration in the shared GUI kit (window frame agent status, first-run agent checks, help/activity/settings flows, prompt panels) and in the recab, recipe, and disk GUIs (agent prompts, “let the agent fix it”, agent status in the window). The user has decided it must be **fully removed**:

- Delete the `levitate-agent` app and stop packaging/shipping it with the desktop systems.
- Remove every agent integration from the recab, Recipe, and disk GUIs and from the shared kit: no agent status, no agent picker/choice, no prompt/fix-it agent flows, no agent run activity.
- The GUIs go back to being plain front ends over their own system tools (`recab`, `recipe`, and the disk tools respectively).
- Removing this is a removal, not a replacement: do not rebuild an agent environment inside any product GUI in another form.

## 4. Defects to fix (with evidence)

### 4.1 AcornOS: Boot step fails and leaves an empty EFI partition

Do not treat the current Acorn release ISO as installable. This is a system bug, not user error. Reproduced failure:

```text
5. Copy the personalized system into both slots, seal them and publish both boot entries [FAILED]
   recab init --root /mnt --mode ab --slot-a /dev/vda2 --slot-b /dev/vda3 \
     --kernel /run/live-media/boot/vmlinuz \
     --initrd /run/live-media/boot/initramfs-installed.img
   ! recab: [RECAB_ERR_STATE] required executable 'sbin/openrc-init' is missing
```

Verified: partitioning, system extraction, fstab generation, and basic setup complete; the user's Acorn disk has the correct GPT A/B layout but an empty EFI partition; the Acorn installed root provides OpenRC pieces and uses busybox as init but does **not** ship `/sbin/openrc-init`, which recab requires for `REC_INIT=openrc` systems (`tools/recab/src/candidate.rs`).

Resolve the mismatch on the system side, rebuild and retest the Acorn release ISO until the Boot step writes EFI boot files/entries and the installed system boots. The correct fix is either for the Acorn image to provide `openrc-init` as its real init or for recab's validation to accept the init the image actually uses; decide based on which init the installed system is intended to run. Do not merely suppress the check.

### 4.2 LevitateOS: same no-bootable-disk outcome after the three apps

The user ran the three helper apps (`disk-plan`, `base-system`, `system-setup`), ejected the ISO, and the firmware reported “No bootable option or device was found.” Verified from the Levitate VM disk on 2026-11-14: correct GPT A/B layout, again an empty EFI partition. The failure is at/after the non-app Boot and Finish command steps; do not treat the three apps as a completed installation.

### 4.3 Recipe cannot fetch: the LevitateOS CA trust bundle is missing

Observed 2026-11-14 installing `bat` from the Recipe GUI:

```text
levitate: cannot fetch https://raw.githubusercontent.com/LevitateOS/recipes/master/index.json:
Connection Failed: tls connection init failed: invalid peer certificate: UnknownIssuer; using the index already here
```

Verified diagnosis: the recipes repository is reachable (same URL returns HTTP 200); the installed Firefox works because it carries its own roots; Recipe uses ureq/Rustls with the `native-certs` feature only (`tools/recipe/Cargo.toml`), trusting only the OS store; and that store is empty because the LevitateOS rootfs never generates `/etc/pki/ca-trust/extracted/pem/tls-ca-bundle.pem` — the builder unpacks RPM payloads and replays selected package effects (`fedora-rpm-package-effects.rhai`) but runs no CA trust extraction, so the `/etc/ssl/certs/*` symlinks point at a missing file. The GUI falls back to its cached index for browsing, but installing fails at fetch.

Fix the system root cause rather than disabling TLS verification. Prefer generating the extracted CA bundles during the rootfs build (the equivalent of `update-ca-trust extract`) so every native-TLS consumer benefits; additionally consider whether Recipe should fall back to known-good embedded roots when the system store yields no usable roots. Verify in the installed system that the generated files exist and that a Recipe GUI install (e.g. `bat`) completes without `UnknownIssuer`.

### 4.4 After the local CA fix, installs stop at recab generation adoption

Observed 2026-11-15: after the Levitate CA bundle was generated locally in both slots, installing `bat` got past fetching but stopped with:

```text
Recipe's own message: the active system differs from its committed generation; explicit adoption is required
Error: system change refused by recab
```

Nothing on the computer was changed by that attempt. This is an observed failure to fix properly in the product (how a legitimate system-level correction becomes part of the committed generation), not something to improvise around per-machine.

### 4.5 Recipe cannot write into the unlocked root at all today

Under the settled §2.2 design, Recipe's system/root changes belong in the unlocked other root (§2.5). Today Recipe cannot do that, because recab's current design does not leave the other root unlocked and writable — it seals/manages it through its prepare → seal → trial → keep lifecycle (§2.2, §4.4's generation-adoption refusal is the same machinery surfacing). This is a direct consequence of the rejected recab design, not a separate Recipe bug to patch around: fix recab to the §2.2 contract (booted locked, other completely unlocked), and Recipe writes into that unlocked root. Do not give Recipe a way to unlock, unseal, or bypass the booted root, and do not invent a Recipe-side workaround that preserves the sealed-candidate lifecycle.

The reset edge case in §2.5 belongs with this fix: once Recipe can write into the unlocked root, a recab “Make the other root a copy of the booted root” can wipe those writes, and Recipe must then retry specifically its root/system part.

## 5. Status: the user's VMs (evidence, not product behavior)

Both of the user's VMs were finished by hand on 2026-11-14 and boot from disk without the ISO:

- **Levitate:** target remounted, `recsetup apply --root /mnt --boot ab …` completed, `recab verify-installation` returned `complete: true` with slots A+B, media ejected; boots to the Plasma SDDM login.
- **Acorn:** the same Boot step only completed after swapping the live environment's `recab` for a locally rebuilt one whose OpenRC check targets `sbin/openrc` (busybox init runs it) instead of the nonexistent `sbin/openrc-init`, and which tolerates Alpine's self-referential `kvm` group member. Verification then returned `complete: true` with slots A+B and it boots to the LXQt SDDM login.

On 2026-11-14/15 the Levitate CA bundle was generated in both slots (`update-ca-trust extract`), and TLS was verified using only each VM's own bundle (118 certificates each): both `https://github.com` and the recipes index returned HTTP 200, and both VMs were left running at their login screens, ready for the user to test. This generated bundle is also what surfaced defect 4.4 on Levitate.

The recab changes used for Acorn are local, uncommitted working-tree edits in `tools/recab/src/candidate.rs` (plus the matching fixture in `tools/recab/tests/finalization.py`). They are **not** in any ISO or pushed anywhere: a fresh install from the current release ISOs will still hit the Boot-step failure until the fix is shipped in the tools the ISO carries and the ISO is rebuilt and retested.
