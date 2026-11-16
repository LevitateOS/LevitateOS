# LevitateOS Product Handoff

Last updated: 2026-11-15 (consolidated rewrite — this version supersedes all earlier orderings of this document).

Read this before changing the installer, recab, Recipe, the Recipe/recab/recdirstat desktop apps, installation/boot behavior, or any public description of LevitateOS/AcornOS. It records, in one place: the user's settled product rules (§2), one removal (§3), the priority that comes before everything (§4), the defects and gaps with evidence (§5), and what was done by hand on the user's VMs (§6). It is an active work list, not a parking lot — carrying a note here is never a reason to postpone the work again — and where the settled rules conflict with the current implementation, the settled rules win.

## 1. How to implement from this document

- **Order of work:** §4 first (fetch upstream Ginux, fix Recipe, fix both OSes). Then the recab contract (§2.2/§2.3, which unblocks the Recipe defects in §5.1). Then the remaining §5 defects. §2's other rules apply to whatever you touch along the way.
- **Never build on this document's say-so.** Nothing here authorizes a build: no ISO, rootfs, kernel, image, or other long build happens until **everything is ready and the user explicitly says to build**. Where this document says “fix”, “rebuild”, or “retest”, it means make the source ready — the actual build/boot verification waits for the user's explicit build go, every time. Installing into an existing environment (VM, checkout) is preferred over building a new one; see the user's standing rule: long builds only on explicit permission, never plan long rebuilds when an existing environment can be used.
- Implement the settled model **exactly** and add nothing “helpfully” on top. Do not introduce extra lifecycle states, extra roles for a drive, candidate management, backup systems, recovery flows, agent integrations, or reinterpretations that change the design. If a proposed behavior cannot be stated in the simple terms of §2, it is not the intended design.
- Do not defend the current implementation as the intended design, and do not re-explain its machinery as an answer to a design complaint. Where §5 quotes the current implementation's own words (sealed, trial, generation, adoption), that is evidence of the problem, never the vocabulary of the fix.

## 2. The settled design

### 2.1 Installation and booting

There is no install hub. The user does not want it at all: do not keep it as the default pane, and do not hide it behind a toggle. The installation screen is only:

- a root system terminal on the left;
- the installation guide/docs on the right.

The guide presents the helper app and the manual commands as alternatives, never as consecutive required work: use the helper **or** do it manually, and if the helper completed a step, the following manual section is explicitly “skip this — only if doing it by hand.”

The installer must never imply the system is installed before it is. Running the three helper apps (`disk-plan`, `base-system`, `system-setup`) is not a completed installation: the Boot and Finish steps still remain, their progress and results must be visible in the terminal, and installation is only complete once boot files/entries exist on the EFI partition and verification has passed.

In the boot screen, the user can manually pick A or B. The entry auto-picked by default must be whichever system is supposed to boot right now — the current/next correct one, whether that is A or B — never hard-wired to always boot A. This is the boot-screen side of the fallback rule in §2.2.

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
- There must be a plain standalone recab action/button: **“Make the other root a copy of the booted root.”** It overwrites the currently unlocked other root with a fresh copy of the booted root, so the user starts changes from the current working system and can discard a stale, old, or messed-up unlocked root. Today this only exists indirectly (a change/preparation starts as a copy, and an existing next system has “Reset to the running system…”); that is not enough. The required action is available as its own operation, not buried inside a change/preparation lifecycle. This overwrite also discards any root/system changes Recipe had written into the unlocked root; Recipe's handling of that is in §2.3.
- There is no separate backup-history feature. The backup **is** the other boot drive that worked; adding a full backup/history on top of A/B defeats the purpose of A/B.
- A GUI cannot be opened inside a system that failed to start, so the recab GUI must not add pages, panels, prompts, or flows around “fixing” a failed boot. The only failed-boot behavior is the rule above.
- A **mutable** install (single root, chosen in `disk-plan`) has no A/B at all, so recab must **not be installed in the first place** on that OS: no `recab` binary, no trial service/unit, nothing A/B on a system that has no other root — not installed-and-then-removed, but never put there. (The recab GUI is the same principle; see §5.5 for the checked state of the install apps.)
- Mutable removes every A/B assumption downstream, not just recab itself: **Recipe on a mutable install does not deal with A/B roots at all** — there is one root, it is the running one, and Recipe's system changes simply apply to it (no locked/unlocked pair, no §2.3 unlocked-root targeting, no reset-retry case). The **disk/recdirstat GUI must not show its A/B (systems) page** on a mutable install. And because recab is the update mechanism on A/B and it is absent on mutable, **mutable needs its own update path and its own updater GUI**, on the shared release-engineering track in §2.10 — mutable updates are a first-class track, not an afterthought.
- The current build's shifting use of B — sealed backup copy at rest, next-system staging slot during an update, fallback after a trial, managed through a prepare → seal → trial → keep lifecycle — is the overengineering the user rejects. That lifecycle, including its trial service, is what gets removed, not renamed.

### 2.3 How system changes work: Recipe writes into the unlocked root

- **Recipe's system/root changes go into the unlocked root, never the booted one.** Recipe must understand the §2.2 contract: the booted/live root is locked, so anything Recipe needs to change at system/root level is written into the other, unlocked root. That unlocked root is the *only* writable system target. Recipe does not write into, unlock, or work around the booted root. (Today this is impossible; see §5.1.)
- **Reset edge case: Recipe must retry only its root changes.** If Recipe has written root/system changes into the unlocked root and the user then runs recab's “Make the other root a copy of the booted root” (§2.2), that copy overwrites the unlocked root and Recipe's root changes in it are gone. Recipe must detect that its root changes are no longer in the unlocked root and **re-apply/retry specifically that root/system part** — not redo the whole install, and not redo the “for me”/user part that was never in the unlocked root. Until it has retried, Recipe must not report the system part as done.

### 2.4 Shipped software is Recipe-installed, not burned in

**Anything shipped above the minimal base that a user might want to remove, replace, or change must be installed through Recipe** (system scope, written into the unlocked root per §2.3), with a Recipe receipt, so Recipe can uninstall it exactly like a user-installed app. Burned into the ISO image means it can never be cleanly removed — that is the wrong default for almost everything above the base.

The shipped system still arrives complete: the default set is Recipe-installed as part of getting the system ready, so first boot looks the same as today. The difference is that each piece has a Recipe receipt and can be uninstalled or swapped in Recipe. Group recipes at the level a user thinks in (the desktop environment, one app, the printing stack) — not one recipe per underlying RPM/APK package.

Burned into the image today, and therefore to become recipes (LevitateOS names shown; AcornOS has the LXQt equivalents): the Plasma suite and its apps (Konsole, Dolphin, KWrite, Ark, Spectacle, KCalc, Skanlite, system monitor), Firefox, the PipeWire/WirePlumber audio stack, NetworkManager and its applets/VPN, Bluetooth (`bluez`/`bluedevil`), the CUPS printing stack, firewalld, fonts, the Orca/speech stack, thumbnailers, Mesa/media drivers, and `spice-vdagent`. The per-OS capability rule (§2.5) and the proprietary rule (§2.6) apply to these recipes unchanged.

What stays burned in is only the minimal base needed to boot the machine and run Recipe/recab themselves: kernel, init, the base system, the rec* tools, and Recipe itself. If a piece is not needed for that, the default is Recipe, not the image.

The desktop environment is the biggest case. Changing the entire desktop environment must be easy, the same as changing any other app: the default desktop — Plasma on LevitateOS, LXQt on AcornOS — must be treated **as if it was installed through Recipe** (same Recipe format and engine as every other `recipes/` entry), shown by Recipe as installed, and **uninstallable through Recipe** like any other recipe app. Installing a different desktop environment through Recipe and uninstalling the old one must be the normal way to change desktops; no special desktop-swapping machinery. That is not how it works today: Plasma/LXQt are baked in at build time (package lists in each variant's `[desktop]` section, locked and merged into the rootfs by the builder), there is no Plasma or LXQt entry in the `recipes/` catalog, and Recipe cannot uninstall what it did not install. A different desktop simply provides different session files — no login-screen or recab special cases for “the desktop changed” (the login screen treats sessions as data; see `docs/reclogin.md`).

### 2.5 An app is only offered where its base can actually run it (Steam)

Do not force an app onto an OS whose base cannot run it, just to claim both OSes have the same catalog. The two OSes run the same Recipe behavior, not necessarily the same app list. A recipe declares which libc it needs (`ABOUT.libc` in the recipe/catalog), and Recipe on a given OS must not present a recipe as installable when that OS's base cannot run it — the user should never get a “successful install” of something that cannot start.

The example that settles this: **Steam.** Steam (and therefore Counter-Strike 2) must be runnable on **LevitateOS**, with CS2 actually playable there — a target to make true and verify on real hardware (glibc base, the 32-bit libraries Steam/CS2 need, GPU drivers), not something to assume from “the recipe installed.” On **AcornOS**, Steam is an upstream glibc program (including 32-bit glibc libraries) and AcornOS is a musl base; it does not run there natively, and the daily-driver audit already records that upstream desktop builds cannot run on AcornOS until Recipe can settle a program it did not link. So: no AcornOS Steam recipe, and no glibc-compat shims, chroots, or other workarounds bolted on just to force Steam onto AcornOS. If that underlying capability ever genuinely changes, revisit this; until then Steam is LevitateOS-only.

### 2.6 Proprietary software: Recipe-only, user-chosen, flagged

Proprietary software is never baked into the shipped ISO/base image. The base images ship what their distribution ships; anything proprietary reaches a machine only because the user explicitly installed it through Recipe. This is already the project's settled pattern and it stays: the H.264/AAC codecs are a Recipe recipe (`codecs`, written 2026-11-16, not run) and NVIDIA's proprietary driver is to be one (not written; the owner left it for now on 2026-11-16, reasons in docs/desktop-todo.md, decision 5); nouveau and the distro codecs are the image defaults, and existing recipes already carry proprietary upstream software this way (e.g. Claude Code, Antigravity). Steam (§2.5) follows the same rules:

- It declares it: the recipe's `license` is `proprietary …` (the recipe format already requires SPDX or `proprietary`, and the front end flags `proprietary`), and the Recipe GUI shows that flag **before** install — the user chooses proprietary software knowingly, never by default or bundled silently.
- It is fetched from the vendor's own upstream by the recipe, not redistributed by LevitateOS, wherever the vendor's terms require that (check the terms per recipe, as the catalog already notes for Chrome).
- Updates come through Recipe; the program's own bundled auto-updater is set to manual/none where it has one, as the existing proprietary recipes already do.
- Being proprietary never earns an app a place in the base image, and being refused for the base image never bans it from Recipe: user choice through Recipe is exactly where proprietary software belongs.

### 2.7 The Recipe GUI

- **“For me” is the default scope** in the “for me / for everyone” choice. The user lands on the personal scope without switching away from a system-wide default.
- **Progress feedback must be live, with no lag after completion.** Step state, progress indicator, output, and the final installed/failed state track the backend as it happens. The window must not sit in a misleading “Installing…” state after the backend has finished, and on completion it must promptly show the result (installed version/size, or the failure). Observed 2026-11-15 while installing `claude-code`: the output already said “install step finished”, “claude-code installed”, and “recipe completed” with all three step checks ticked, yet the window kept showing “Installing claude-code, step 3 of 3” with a running progress bar and a Stop button for a noticeable while before switching to the installed state.
- **Recipe source must be inspectable.** A built-in, read-only source code viewer shows the selected recipe's actual `.rhai` file — the source Recipe will really run for that entry, including which source/folder it came from — with Rhai syntax highlighting. This is inspection, not an embedded editor or agent environment.
- **Teach users how to work with their own recipes and their own agent.** In plain language: an agent working on recipes must be pointed at the correct **recipes root** (the folder/repo whose recipes Recipe will actually read); the user can make a new recipes repo/folder of their own, put their recipes there, and then tell Recipe that this new folder is a recipe source/folder to use. The flow ends with the user explicitly adding that folder and Recipe showing it is now reading recipes from there.
- How the GUI's system installs interact with the unlocked root, the reset edge case, shipped software, per-OS availability, and proprietary flags is defined in §2.3–§2.6, not repeated here.

### 2.8 Product GUIs: no agent inside, ever

Neither recab's GUI nor the Recipe GUI may contain “ask an agent to make changes,” an agent picker/choice, or any embedded agent workflow. That is not their job; if the user wants an agent to make changes, they open their own agent window — with the agent of their choice — and ask it there. Do not constrain the user to “our” agent choices inside a product GUI, and do not turn a product GUI into an agent development environment. The one allowed bridge is a **“copy for agent”** button on failure/output views: one click copies the full error/output text so the user can paste it into their own agent window. That is a clipboard affordance only — not an embedded agent, not a picker, not an integration. (Removing the existing agent subsystem is §3.)

### 2.9 Readability (installer, docs, and `disk-plan`)

- Never put gray text over a colored background. It is unreadable, especially the docs pane's selected item with gray text on orange.
- Selected/highlighted rows need strong, clearly legible contrast: dark text on a light/orange highlight, or bright text on a dark saturated background.
- In `disk-plan` step 3, the **selected** item must be the brightest text. The current inverse emphasis is wrong: unselected items should be dimmer and the current selection most prominent.
- This applies to the docs/guide pane too: selection must make the text more readable, not less.

### 2.10 The desktop systems as shipped, and all public wording

- **Firefox:** the shipped new-tab page currently carries Fedora's default bookmarks. Remove all of them and ship exactly one bookmark: `levitateos.org`. Do not replace them with another preloaded set.
- **Plasma panel:** the second icon in the start bar/panel is currently Fedora Discover. Replace it with the Recipe app launcher. The shipped desktop must not present Discover (or any Fedora software store) as its software app; Recipe is the software catalog/installer.
- **Icons to create:** special, purpose-made icons for **recab** (A/B system changes and updates), **recipe** (the software catalog/installer), and **recdirstat** (disk-usage treemap analyzer). The recab and recipe apps currently ship generic freedesktop icons (`Icon=system-software-update` in `tools/gui/recab/data/org.levitateos.recab.desktop`, `Icon=system-software-install` in `tools/gui/recipe/data/org.levitateos.recipe.desktop`); the Plasma Recipe launcher uses recipe's new icon. Recdirstat has no GUI app yet, so its icon starts as a standalone asset until its app exists. Decide a visual direction with the user first (distinct flat motifs with colour accents; monochrome brand glyphs matching the leaf logo; or brand-mark badges), then wire the winning icons into the apps' `data/` dirs and `Icon=` fields, keeping them legible at 16–48 px.
- **All comms: independent OSes, built from Fedora/Alpine packages.** Every public description — the website, docs, announcements, store/catalog listings, interviews, and any DistroWatch submission (where “Based on” must be filed as **Independent** for both OSes) — must make two things clear, in this order: **(1) LevitateOS and AcornOS are independent operating systems**, and **(2) their userlands are built from Fedora 45 RPM packages / Alpine 3.24 APK packages respectively**. Those repositories are our parts supplier, nothing more. Never describe either OS as “based on Fedora/Alpine,” a Fedora/Alpine derivative, or a respin: we use neither distro's installer, package manager (`dnf`/`apk` never runs, here or on the installed system), kernel (ours is built from pinned source), release engineering, or update channels; the installer, rec* tools, A/B booting, and Recipe are our own. The honest one-liner is “independent, built from Fedora/Alpine packages,” and any page or post that implies “based on” must be corrected to it. (The product cards' “Base” label and “trial boot, rollback” line were corrected to this and to §2.2 on 2026-11-15; all remaining recab comms are covered by §5.7.)
- **Recipe gets its own page.** Recipe is halfway to being a standalone app in its own right, so the website must treat it as one: a **separate page for Recipe alone**, and most of the Recipe content currently spread over the LevitateOS/AcornOS pages moves there. The OS pages keep only what is about the OS itself plus a short “software comes through Recipe” mention that links to the Recipe page — they stop carrying Recipe's detailed story (how recipes work, sources/repos, writing recipes, scopes) duplicated per OS. One Recipe, one page, both OSes link to it.
- **The Recipe page has an entire section about Ginux.** Recipes are built and run inside isolated Ginux domains — that isolation is part of Recipe's story (a recipe runs in its own domain, with only the network/files/access its `NEEDS` declare, instead of loose on the user's system), so the Recipe page explains it in its own section: what Ginux is, that every recipe build/run happens inside a Ginux domain, what the domain can and cannot reach, and why that makes “read the recipe, then trust what it does” real. Keep it honest about ownership per §2.10's comms rule: Ginux is Greg Kurtzer's upstream project (`gmkurtzer/ginux`; we maintain our fork and reconcile with upstream, §4) — present it as the technology Recipe uses, not as a LevitateOS invention.
- **One release-engineering track for all updates.** Before any updater is implemented, figure out the shared release engineering: how releases are versioned, built, signed/verified, published, and discovered by an installed system (update feed/channel, metadata format, failed/interrupted update behavior). That one track must cover, at the same quality: **(a)** A/B system updates, delivered through recab (the A/B update mechanism, §2.2); **(b)** **mutable system updates**, delivered through a **dedicated mutable-updater GUI** (recab is not installed on mutable, §2.2) — mutable gets full release engineering, not a lesser path; and **(c)** **self-update of the desktop apps themselves** (Recipe, recab, recdirstat): each gets a visible **“Check for updates”** button and updates itself through this same track and mechanism, not three home-grown updaters. Recipe's own self-update runs on this track like the others. Do not bolt on a per-app or per-mode updater before the shared release story exists.

## 3. Remove the entire agent subsystem (`levitate-agent`)

There is a whole agent application/subsystem — `tools/gui/agent` (`levitate-agent`) plus its integration in the shared GUI kit (window frame agent status, first-run agent checks, help/activity/settings flows, prompt panels) and in the recab, recipe, and disk GUIs (agent prompts, “let the agent fix it”, agent status in the window). The user has decided it must be **fully removed**:

- Delete the `levitate-agent` app and stop packaging/shipping it with the desktop systems.
- Remove every agent integration from the recab, Recipe, and disk GUIs and from the shared kit: no agent status, no agent picker/choice, no prompt/fix-it agent flows, no agent run activity.
- The GUIs go back to being plain front ends over their own system tools (`recab`, `recipe`, and the disk tools respectively).
- Removing this is a removal, not a replacement: do not rebuild an agent environment inside any product GUI in another form.

## 4. DO FIRST: fetch upstream Ginux (upstream is leading), then fix Recipe and both OSes

Before everything else — before the recab redesign, the Recipe work, and everything in §5 — reconcile `tools/ginux` with upstream. This section is about fetching and adapting **source only**: it never calls for building an OS, an ISO, or a kernel (see §1's never-build rule). Upstream Ginux ([`gmkurtzer/ginux`](https://github.com/gmkurtzer/ginux), remote `upstream` in `tools/ginux`) has moved: as of 2026-11-15 upstream `main` is **67 commits ahead** of our fork's HEAD (`LevitateOS/ginux:main` in `tools/ginux`). Per the superproject rule, **upstream leads**: retain its history, reconcile the incoming upstream changes into our fork, and replace or drop our changes where upstream now provides the behavior; keep only the Recipe-required changes available in our repository.

Then fix what that reconciliation breaks downstream, in this order:

1. Fetch upstream into `tools/ginux` and reconcile (upstream's history and direction win on conflicts).
2. Fix **Recipe** against the reconciled Ginux (Recipe builds/runs in Ginux domains; upstream descriptor/init changes are exactly the kind of thing that breaks it).
3. Fix **both OSes** against the reconciled Ginux and fixed Recipe: change whatever in their build or boot code the reconciliation broke. This is a fixing step, not a build-and-boot run; builds happen only when the user asks for one.

Status 2026-11-15:

- **Step 1 done.** Upstream `main` (`285d025b`, PRs #273–#289, Gates 25–26) is merged into `LevitateOS/ginux:main` as `f9bef00a`; the fork is 0 commits behind upstream. Six conflict hunks, all resolved with upstream's text plus the configured-launch pieces upstream does not carry. The fork's CI ran the root launch suite on the merge and it passed.
- **Step 2 done, no Recipe code change needed.** Recipe `b215942` pins `f9bef00a`, and Recipe's full acceptance catalogs pass on it as root (146 of 146). Getting there took two fixes outside the product code: the acceptance job now builds only the launcher (Ginux's `make build` also builds upstream's operator CLI), and three acceptance cases that had been stale since host execution landed were brought up to date. The instance schema and the configured catalog digests are unchanged, `instance-validate`/`instance-explain` answer as before, and a `bat` install ran to completion inside a Ginux domain through the merged launcher as an ordinary user. The launcher binary is still `ginux` with the `launch` command (upstream's operator CLI is a separate binary, `ginux-cli`).
- **Step 3 done: nothing in either OS needed fixing.** The superproject pins both. The builder compiles the launcher from the `tools/ginux` checkout with the same command as before and that command builds; nothing in the builder, the variants or the OS recipes refers to anything upstream renamed or removed. Unbuilt, like every other change: no ISO was built for this, and none is needed for it.

Nothing else in this document is started until Recipe and both OSes are working on the fetched upstream. Do not “fix” Recipe or an OS by pinning, reverting, or working around the upstream Ginux changes — upstream is leading, we adapt. (The installed `.hdae` skills bundle also comes from this owner — `gmkurtzer/hdae`, recorded in `AGENTS.md` — and was verified current on 2026-11-15; it is not part of this fetch.)

## 5. Defects and gaps (with evidence)

### 5.1 The current recab design blocks Recipe (generation adoption + no writable unlocked root)

One root cause, two observed symptoms. The fix is the §2.2/§2.3 contract, not a patch to either symptom.

Symptom A (2026-11-15, LevitateOS, installing `bat` after the local CA fix in §5.4):

```text
Recipe's own message: the active system differs from its committed generation; explicit adoption is required
Error: system change refused by recab
```

Nothing on the computer was changed by that attempt.

Symptom B: under §2.3, Recipe's system/root changes belong in the unlocked other root. Today Recipe cannot write there at all, because recab's current design does not leave the other root unlocked and writable — it seals/manages it through its rejected prepare → seal → trial → keep lifecycle, and symptom A is that same machinery refusing an out-of-band change.

Fix recab to §2.2 (booted locked, other completely unlocked) and Recipe writes into that unlocked root, with the §2.3 reset-retry behavior. Do not give Recipe a way to unlock, unseal, adopt, or bypass the booted root, and do not invent a Recipe-side workaround that preserves the sealed-candidate lifecycle.

### 5.2 AcornOS: Boot step fails and leaves an empty EFI partition

Do not treat the current Acorn release ISO as installable. This is a system bug, not user error. Reproduced failure:

```text
5. Copy the personalized system into both slots, seal them and publish both boot entries [FAILED]
   recab init --root /mnt --mode ab --slot-a /dev/vda2 --slot-b /dev/vda3 \
     --kernel /run/live-media/boot/vmlinuz \
     --initrd /run/live-media/boot/initramfs-installed.img
   ! recab: [RECAB_ERR_STATE] required executable 'sbin/openrc-init' is missing
```

Verified: partitioning, system extraction, fstab generation, and basic setup complete; the user's Acorn disk has the correct GPT A/B layout but an empty EFI partition; the Acorn installed root provides OpenRC pieces and uses busybox as init but does **not** ship `/sbin/openrc-init`, which recab required for `REC_INIT=openrc` systems (`tools/recab/src/candidate.rs`).

Status: recab's validation was changed to accept the init the Acorn image actually uses (`sbin/openrc`, plus tolerating Alpine's self-referential `kvm` group member) and that change is committed and pushed in the recab repo (`b9730d5`). It is **not** in any ISO: when — and only when — the user gives the explicit build go (§1), the Acorn release ISO gets rebuilt and retested until the Boot step writes EFI boot files/entries and the installed system boots, so a fresh install no longer needs the hand-finishing recorded in §6. Do not merely suppress the check, and do not build early to “check” it.

### 5.3 LevitateOS: same no-bootable-disk outcome after the three apps

The user ran the three helper apps (`disk-plan`, `base-system`, `system-setup`), ejected the ISO, and the firmware reported “No bootable option or device was found.” Verified from the Levitate VM disk on 2026-11-14: correct GPT A/B layout, again an empty EFI partition. The failure is at/after the non-app Boot and Finish command steps; do not treat the three apps as a completed installation (see also §2.1). The Levitate VM was hand-finished the same way as Acorn (§6); a fresh install from the current release ISO gets retested after the Boot-step fixes only on the user's explicit build go (§1).

### 5.4 Recipe cannot fetch: the LevitateOS CA trust bundle is missing

Observed 2026-11-14 installing `bat` from the Recipe GUI:

```text
levitate: cannot fetch https://raw.githubusercontent.com/LevitateOS/recipes/master/index.json:
Connection Failed: tls connection init failed: invalid peer certificate: UnknownIssuer; using the index already here
```

Verified diagnosis: the recipes repository is reachable (same URL returns HTTP 200); the installed Firefox works because it carries its own roots; Recipe uses ureq/Rustls with the `native-certs` feature only (`tools/recipe/Cargo.toml`), trusting only the OS store; and that store is empty because the LevitateOS rootfs never generates `/etc/pki/ca-trust/extracted/pem/tls-ca-bundle.pem` — the builder unpacks RPM payloads and replays selected package effects (`fedora-rpm-package-effects.rhai`) but runs no CA trust extraction, so the `/etc/ssl/certs/*` symlinks point at a missing file. The GUI falls back to its cached index for browsing, but installing fails at fetch.

Fix the system root cause rather than disabling TLS verification. Prefer generating the extracted CA bundles during the rootfs build (the equivalent of `update-ca-trust extract`) so every native-TLS consumer benefits; additionally consider whether Recipe should fall back to known-good embedded roots when the system store yields no usable roots. Verify in the installed system that the generated files exist and that a Recipe GUI install (e.g. `bat`) completes without `UnknownIssuer`. (The bundle was generated by hand on the user's VM; see §6. That hand generation is also what surfaced §5.1.)

### 5.5 Mutable installs get recab anyway

The requirement is in §2.2 (mutable ⇒ no recab on the installed OS). Checked 2026-11-15 against the three installation apps and the boot step:

- **`disk-plan`: mutable is first-class.** Mode choice `ab | mutable` (`InstallMode`), mutable layout is EFI + one root, state records `mode` + `root_device` instead of slots, and mutable fixtures exist (`plan-mutable.json`, `dry-run-mutable*.json`).
- **`base-system`: compatible.** It reads the disk step's mode (shows “A/B” or “mutable” layout) and does a generic copy + fstab; nothing slot-specific.
- **`system-setup`: compatible.** Mode-agnostic (accounts/locale/hostname only); it never touches boot. The Boot step is the guide/`recsetup` step after the apps, and `recsetup --boot mutable` is supported (one systemd-boot entry; `tools/recstrap/src/bin/recsetup/plan.rs`), with a mutable variant in the guide (`06-installation-boot.ts`).

The defect: **recab is baked into every installed rootfs regardless of mode.** In `distro/builder/src/pipeline/live_tools.rs` recab is `ToolLayer::Rootfs` (“on every product and every system an ISO installs”), and its packaged tree includes the trial service (`os-recipes/install-tools.rhai`). The mode is only chosen later, at install time in `disk-plan`, and nothing in `recsetup`/the install flow removes recab or its trial service for a mutable install. Because the rootfs is built before the mode is known, the fix belongs at install time: the mutable install flow (`recsetup --boot mutable` path) must leave the recab pieces out of the target entirely — recab is never installed on a mutable system in the first place, not copied in and cleaned up afterwards. Verify after fixing: a mutable install's root contains no `usr/bin/recab`, no recab-trial unit, and boots its single root without recab state.

### 5.6 `reclogin` must join prebuilt

The prebuilt tool set (`TOOLS` in `distro/builder/src/recipe/tools.rs`, published by the prebuilt workflow) covers `recpart`, `recstrap`, `recab`, `recguest`, `levitate-recipe`, and `install-split-pane`. One tool is missing and must be added the same way (table entry, publish, ISO builds download instead of compiling):

- **`reclogin`** — requirement: it must be in prebuilt before it ships in any image; do not put reclogin in an ISO until it is in prebuilt. Do not invent a separate distribution path for it. Status 2026-11-16: the program is written in `tools/reclogin` (two programs, `reclogind` and `reclogin-greeter`, all stages including the guest account), never built into an image or run. It is published as its own prebuilt family `login-screen`, not as an entry in the `TOOLS` table (`Family::LoginScreen` in `distro/builder/src/prebuilt.rs`, `distro/builder/src/login_screen.rs`, built by `distro/builder/os-recipes/reclogin.rhai`, `cargo xtask prebuilt build|publish login-screen <distro>`). No `login-screen` prebuilt has been published yet. SDDM is still the login screen on both OSes; the switch is pending the owner's build and publish, and its remaining steps are in `tools/reclogin/dist/recipe/SWITCH.md`.

`recqemu` was listed here and was dropped on 2026-11-15 (the user's decision): it is a library that the builder, xtask, the install tests and recshuttle link, with no program of its own, so there is nothing to publish.

### 5.7 All comms about recab still describe the rejected lifecycle

When recab is rebuilt to the §2.2 contract, **every public description of recab must be updated with it** — comms are part of the redesign, not a later cleanup. Today the website and docs still teach the rejected prepare → seal → trial → keep lifecycle as if it were the design, in at least: the homepage recab demo (“One trial boot. If B does not come up healthy, you are back on A.”, `HomePage.astro`), the FAQ (`01-faq.ts`: “trial boot before each update is kept”, “Stage the release into slot B, then trial boot, fallback, commit and rollback”), the atomic-updates page (`03-atomic-updates.ts`), the post-installation docs (`07-post-installation.ts`), and the recab help text mirrored into docs (`tool-help.ts`: `set-next`/`commit`/`rollback`/`trial` commands).

The updated comms must describe only the settled design, in its own words: booted root locked, other root unlocked and changed there, “Make the other root a copy of the booted root,” and a failed boot returning to the last working boot. The words **trial, seal/sealed, candidate, commit/rollback (as lifecycle steps), generation, adoption, and backup history must not appear** as recab concepts in any public text. Do not leave old pages up “because they describe the old tool accurately” once the new recab ships — a user reading them learns the wrong system.

### 5.8 The superrepo README needs a complete rewrite, not more patches

The root `README.md` has grown by patching: its section structure/headers were never thought through as a whole, and repeated wording fixes (independent/built-from, the A/B contract, Recipe-installed software, one release ISO per OS) have been layered onto that structure. Do not keep patching it. **Rewrite `README.md` from scratch**: decide the sections and headers first — what a newcomer must learn, in what order — then write it fresh in the settled vocabulary of §2 (no “based on”, no trial/continuation lifecycle language, Recipe and the locked/unlocked A/B contract stated plainly, one ISO per OS). The interim wording edits made on 2026-11-15 (settled A/B description, independent/built-from paragraph, §2.4 software rule, product-handoff pointer, dead `architecture.md` link removed) are a stopgap to keep it honest until the rewrite, not the final structure.

The same applies, faster, to **every other README** (superproject subdirectories, `xtask`, and every submodule: builder, contract, rec* tools, `tools/gui`, `tools/tui`, website/content): each gets a quick rewrite pass — not a from-scratch restructure like the root README, but rewritten text, not patches: settled vocabulary of §2, current paths (e.g. `tools/gui`/`tools/tui` after the 2026-11 move), no references to removed files or the rejected recab lifecycle, and an honest implemented-vs-settled split where a README describes behavior §5 lists as defective.

## 6. Status: the user's VMs (evidence, not product behavior)

Both of the user's VMs were finished by hand on 2026-11-14 and boot from disk without the ISO:

- **Levitate:** target remounted, `recsetup apply --root /mnt --boot ab …` completed, `recab verify-installation` returned `complete: true` with slots A+B, media ejected; boots to the Plasma SDDM login.
- **Acorn:** the same Boot step only completed with a locally rebuilt recab carrying the §5.2 fix. Verification then returned `complete: true` with slots A+B and it boots to the LXQt SDDM login.

On 2026-11-14/15 the Levitate CA bundle was generated in both slots (`update-ca-trust extract`), and TLS was verified using only each VM's own bundle (118 certificates each): both `https://github.com` and the recipes index returned HTTP 200, and both VMs were left running at their login screens, ready for the user to test.

That hand-finishing is evidence about the current ISOs, not a fix: the §5.2 recab change has since been committed and pushed (`b9730d5`), but no ISO carrying it has been built, so a fresh install from the current release ISOs will still hit the Boot-step failure until — on the user's explicit build go — the ISO is rebuilt and retested.
