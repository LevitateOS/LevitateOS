# Release readiness: when can LevitateOS and AcornOS be published and marketed?

Research of 2026-11-11. The question: what has to be true before the two
systems can be offered for download, and before they can be marketed.

How it was made: the repository's own release definitions were collected and
compared: the agreed release scope in
[ginux-contribution-task-list.md](ginux-contribution-task-list.md) (PRD01–PRD19),
recab's [P14 release engineering](../tools/recab/RELEASE-ENGINEERING.md),
[release/updates/README.md](../release/updates/README.md), the
[daily-driver audit](daily-driver-audit.md) of the same day,
[desktop-todo.md](desktop-todo.md), the website's
[PUBLISHING_CHECKLIST.md](website/PUBLISHING_CHECKLIST.md) and pages, both
`distro.toml` files, and the open GitHub issues. Nothing was built or booted.

## The answer in short

**Nobody has written down what "ready to publish" means for the distros as
products.** Three partial definitions exist, written for different purposes,
and they do not agree on what the first release contains:

| Source | What it calls done | Desktop included? |
|---|---|---|
| Agreed release scope, PRD17–PRD19 (user, 2026-11-08) | Recipe/Ginux production acceptance, plus both distros' base and live-tools products through build, boot, installer PTY, install and A/B update under libvirt | **No.** "Desktop products … are outside that agreed verification scope" (README) |
| P14 release engineering (approved 2026-11-11) | A signed public update channel that installed systems of both distros check, download and apply from, with trial and fallback | Yes: "base and desktop systems" |
| Positioning (README, website) | LevitateOS is "the daily-driver workstation"; each ships "a live ISO with and without a desktop"; website: "Planned for 2027" | Yes, by implication |

The GitHub epic for this exact question,
[LevitateOS/LevitateOS#28](https://github.com/LevitateOS/LevitateOS/issues/28)
with [#29](https://github.com/LevitateOS/LevitateOS/issues/29) "define
production-readiness gates", was opened in April 2026 and never filled in. It
points at paths that no longer exist (`distro-variants/`, `BACKLOG.md`,
`docs/manual-install-plan.md`).

So the first thing needed was a decision, not work: is the first public
release the text-mode system, or the desktop?

**Decided (the user, 2026-11-11): desktop first.** The first public release of
each distro includes its desktop ISO, so it needs Gates A, B and C. The agreed
PRD19 text-mode scope stays as a prerequisite inside Gate A; it is no longer the
whole of the first release. The gates:

- **Gate A — publishable.** A download can sit on the website without lying.
  This is roughly the agreed release scope plus the update channel, legal and
  release mechanics.
- **Gate B — marketable as what the README promises.** LevitateOS a daily
  driver, AcornOS its small and strict sibling, both with a desktop.
- **Gate C — the launch itself.** The website and the launch material.

None of the three gates is met today. Gate A is the nearest: most of its
code exists and mostly needs running, fixing what the run finds, and
outside merges. Gate B is months of work (the audit's five stages).

## Gate A — publishable

### A1. Recipe and Ginux production acceptance

The task list's own completion rule: PRD17 is done "when the complete declared
release scope passes and the user accepts the supported production outcome".

| Item | State |
|---|---|
| PRD02, 03, 04, 06, 08 | done |
| PRD18 declared `NEEDS` (a required deliverable, not a follow-up) | implemented and verified locally; maintained-source delivery open |
| PRD10 catalog (155 cases), PRD11 consumers, PRD12 lifecycle | pass locally on a candidate tuple; final-source acceptance open |
| PRD14 bootstrap/packaging from durable inputs | done on an earlier pair; reopened for the final source and the new version |
| PRD01 distributable runtime source | open |
| PRD05 integrate the merged storage source | **blocked on upstream merges** |
| PRD07 contribution waves | open; 14 PRs (#207, #232–#248) wait on the upstream maintainer |
| PRD09 security acceptance on the final source | partial receipts |
| PRD13 hosted CI | local pass; hosted Linux acceptance pending |
| PRD15 callers and docs agree with the code | in progress |
| PRD16 record and wire the final delivery | open |
| PRD17 present acceptance, get publication approval | open |

Two questions in it wait on the user: the **new Recipe version number**
(0.1.4 is already published) and the **scope of ISO byte-determinism**. Final
packaging is stopped until the version is chosen.

Outside dependency: PRD05 and PRD07 depend on Greg Kurtzer merging upstream.
Nothing here controls when that happens. If the release must not wait, the
fallback the AGENTS.md delivery decision already allows is shipping from
`LevitateOS/ginux:main`, which still needs PRD05's merged storage source or
an explicit decision to ship without it.

### A2. Both distros pass the product acceptance (PRD19)

For **both** LevitateOS and AcornOS, on the final Recipe/Ginux/image
combination:

- real Fedora RPM and Alpine APK production with pinned assets, full rootfs;
- separate installed-system and live-overlay products, metadata kept through
  EROFS and recstrap;
- the pinned prebuilt kernels (7.3.3 for LevitateOS, 6.18.62 for AcornOS);
- the full live-tools ISO and its live-boot parent, with provenance and hashes;
- live boot, SSH readiness, and real interaction with the installer in a PTY;
- the public install path, installed boot, login and runtime scenarios;
- A/B update, activation and recovery.

Done when these pass on current code:

```
cargo xtask test all levitate
cargo xtask test all acorn
cargo xtask test ab-update levitate
cargo xtask test ab-update acorn
```

State: they passed on 2026-11-09, which is 165+ superproject commits ago and
before the kernel swap and the recab rewrite. CI's "Products and scenarios"
job has been red or cancelled for about 40 runs since 2026-11-08. A fresh
build of both, then fixing what it finds, is the work.

### A3. A working public update channel (P14)

P14's own completion rule: "complete only after the real public origin,
generated public trust and matching signed payloads exist, both distro
installations check/download/apply through Recipe against them, and applicable
trial/fallback evidence passes."

| Step | State |
|---|---|
| Contracts, signing roles, part transport, installed reader | implemented |
| Pages site, immutable releases, signing environment, per-distro keys | provisioned 2026-11-11 |
| Metadata-only deployment, verified through Recipe | passed 2026-11-11 |
| Release runner | not registered |
| Signed release payloads, a baseline and a newer release | none |
| Installed systems update from the public origin, trial and fallback | not run |
| Installed defaults switched from `channel = "development"` to the production policy | not done (both `distro.toml`) |
| Promotion to `stable` | not done |
| Offline backup of root and release keys | deferred by choice. **A lost root key means re-installing every computer**, so this should be done before the first public install exists |

Without A3 a published release cannot receive a single security fix after the
person installs it. That makes A3 a publishing requirement, not polish.

### A4. Release artifacts and identity

- Each release: the live ISO, its `.sha512` beside it (promised by the
  getting-started page), provenance, and the signed `release.json` set.
- A version. Both `distro.toml` files already say `os_version = "1.0"` and
  release `sequence = 1`. Confirm that the first public release really is
  1.0, or lower it to a preview number, before anything is tagged.
- The download page lists version, architecture, format and checksum
  "once a published artifact exists" (`download.astro`); `variants.ts` still
  types the release by hand until the generator emits it.
- The only public GitHub release is "Boot to shell" `v.0.1.0-alpha` (January
  2026), an old pre-release from before the current design. Retire or label it
  so it is not taken for the current system.

### A5. Legal (nothing exists yet)

Neither the repository nor its docs mention any of this:

- **Fedora's trademarks.** The LevitateOS lock carries `fedora-logos`,
  `plasma-lookandfeel-fedora`, `kde-settings*` and `f45-backgrounds-*`, and the
  look patches Fedora's files. The audit already says these must go "before
  anything is published under the LevitateOS name". It needs the dnf
  `--exclude` list (`desktop-todo.md` section 1) and replacement packages.
  Check the base image for Fedora branding too (`fedora-release*`, os-release
  text, boot menu strings), not only the desktop.
- **Alpine's name and logo** in AcornOS: same check, lower stakes.
- **Source for the GPL and LGPL packages.** An ISO that ships GPL binaries
  (the kernel, coreutils/BusyBox, glibc, much of Plasma and LXQt) has to come
  with their corresponding source or a valid offer for it. Pointing at
  Fedora's and Alpine's mirrors is common practice but may not be enough
  on its own. Decide how source is provided, for example by mirroring the
  locked SRPMs, APK sources and kernel source next to each release, and
  check it with someone who knows the licenses. This research did not
  settle it.
- **License notices in the image**: each package's license files, and an
  overview reachable from the system.
- **The names "LevitateOS" and "AcornOS"**: not checked for conflicts.

### A6. Honest documentation and a clean project front

- Website P0.4: claims match the product, statuses are dated, no unshipped
  features implied. True today ("Not released yet"); has to be re-checked
  against whatever ships.
- The stale statements the daily-driver audit lists (`desktop-todo.md`,
  `install-tui-todo.md`, recab's REQUIREMENTS header, `docs/desktop.md`,
  `recipe-sources.md`).
- No `SECURITY.md`, so there is no way to report a vulnerability privately.
  A distro that ships signed updates needs one. GitHub Discussions is off; decide where users
  ask for help (issues alone is a valid answer).
- Release notes per release: what is in it, the known limits (UEFI only,
  Secure Boot off, no encryption, x86_64 only), how to report problems.
- Close or rewrite the stale issues #19–#31; #28/#29 should point at this
  document or its successor.

## Gate B — marketable as the README describes them

The README sells LevitateOS as "the daily-driver workstation" and promises
both systems "a live ISO with and without a desktop". The
[daily-driver audit](daily-driver-audit.md) found that no desktop image has
ever come out of the pipeline. Its five stages are the completion
requirements here, and each needs the one before:

1. **It boots.** Both desktop ISOs built from today's locks; `desktop-boot`
   passes; install from the desktop ISO in a VM and log in; both in the local
   release run (CI runs only the quick checks); CI green. (Also
   `desktop-todo.md` section 12.)
2. **It works on a real laptop.** CPU microcode loaded early. One Intel and
   one AMD laptop per system: Wi-Fi, sound, Bluetooth, suspend, lid,
   brightness, external screen. This also produces the hardware statement the
   download page's requirements table needs.
3. **It survives a week.** Decide where computer state lives on an A/B
   install so Wi-Fi, passwords, users, printers, time zone and hostname can
   change (today all fail, because `/etc` is read-only). The current recab
   through `ab-update` on both. AcornOS's session gaps (Bluetooth audio, screen
   lock, emoji, translations, `gdbus`).
4. **It has the software.** Recipe adds menu entries, icons and file types
   (`XDG_DATA_DIRS`, caches); the first desktop recipes from decision 13
   (OnlyOffice, mail, media and graphics, chat, developer tools); the H.264
   and AAC codec recipes on LevitateOS; an installable agent. AcornOS: the
   step that lets it run a program it did not link. The three maintenance
   apps (Recipe, System, Disk) run in an image against the real tools
   (`gui-ux-gaps.md`: 61 of 231 gaps done).
5. **It stays safe for months.** Gate A3 running for real; a re-lock and
   rebuild cadence that keeps Firefox within about two weeks of upstream;
   update notices in the session; disk encryption; Secure Boot signing.

Plus the look the marketing will show: LevitateOS's own Global Theme and SDDM
theme, AcornOS's LXQt theme and login theme, the wallpapers (one decision still open),
Plymouth, first-login welcome (`desktop-todo.md` sections 2 and 10, decision 7).

Where the line could be drawn: stages 1–3 make a desktop that can honestly be
called a **preview**. "Daily driver" in marketing needs stage 5, and stage 4 at
least for LevitateOS.

## Gate C — the launch

The website's [PUBLISHING_CHECKLIST.md](website/PUBLISHING_CHECKLIST.md)
(2026-02-16) is the existing launch list. Its P0 items still open:

- **Domain:** remove the old apex `A`/`AAAA` records, point the apex at
  Pages, choose one canonical host and redirect the other.
- **Deploy:** latest Pages deployment matches the commit; a known rollback
  target.
- **Pages work:** header and mobile navigation checked.
- **SEO baseline:** titles, descriptions, canonical, OpenGraph and Twitter
  cards, sitemap, `robots.txt`, social image.
- **Accessibility:** contrast, keyboard, focus, reduced motion, alt text.
- **Performance:** no layout shift, sensible image sizes, no console errors.
- **Hygiene:** no secrets, legal links in the footer.
- **Analytics:** the checklist asks for one privacy-respecting tool and three
  events. Decide whether the project wants any analytics at all.
- **Go/no-go:** 24 hours of stable routing, no open P0.

And for the launch itself (checklist P1.6, and `desktop-todo.md` section 12):
screenshots of both desktops, light and dark, from pipeline-built images;
a short demo loop; launch posts for the places it will be announced; a
small press section (logos, one-liner, links); uptime monitoring.

The screenshots depend on Gate B stage 1: no real desktop screenshot can exist
before a desktop image does.

## Decisions

Made:

1. **What is the first public release:** the desktop (Gates A, B and C), not
   the text-mode system on its own. The user, 2026-11-11.

Waiting on the user:

2. **Its version and label:** keep `1.0`, or a preview number. With the
   desktop first, this also decides how far into Gate B the first release
   goes: stages 1–3 make an honest *preview*; calling it a *daily driver*
   needs stage 5.
3. **The new Recipe version number** (0.1.4 is taken). Final packaging waits on it.
4. **ISO byte-determinism scope** (asked earlier, unanswered).
5. **Ship without upstream Ginux merges?** If upstream does not merge PRD05/07,
   whether to release from `LevitateOS/ginux:main` and when.
6. **How GPL source is provided**, and whether to have the licensing checked.
7. **Analytics on the website:** yes (which) or none.
8. **Support channel:** issues only, or Discussions or a forum.
9. **Key backup** before the first public install (currently deferred).
10. **Which of stage 5 the first release needs:** disk encryption and
    Secure Boot signing are each weeks of work. Required for the first
    desktop release, or listed as known limits in its release notes?
11. **Real hardware for stage 2:** which Intel and AMD laptops are
    available to test on, and whether AcornOS is tested on both or one.

## The path, desktop first

Steps on the same line can run side by side; each line needs the ones before
it only where it says so.

1. **Decisions 2–4 and 10.** They set the target; nothing tagged before them.
2. **Declaration work that needs no build**, in parallel with step 3:
   - strip Fedora branding: the dnf `--exclude` list, replacement packages,
     own theme pieces (A5, `desktop-todo.md` sections 1–2);
   - decide where computer state lives on an A/B install, then write it into
     recab and the image (B stage 3). This is the largest design gap and
     blocks every everyday setting, so it should start first;
   - microcode in recinit/recuki (B stage 2);
   - Recipe menu, icon and file-type integration, the codec recipes, the
     first desktop recipes (B stage 4);
   - decide GPL source provision; write `SECURITY.md` (A5, A6).
3. **First build of everything from current code:** both distros' base,
   live-boot, live-tools and live-desktop. Fix until `test all`,
   `ab-update` and `desktop-boot` pass on both, plus an install from the
   desktop ISO that logs in. Products and scenarios run locally, never in CI;
   get CI's quick checks green (A2, B stage 1). Builds only when the user
   asks, within the 10:00–01:00 window.
4. **Real laptops** with the built desktop images (B stage 2), once step 3
   passes. It also produces the download page's hardware statement.
5. **A week on it:** the step 2 state design built and run; AcornOS's session
   gaps closed; the three maintenance apps run against the real `recipe` and
   `recab` (B stage 3).
6. **The update channel live:** keys backed up, runner registered, a signed
   baseline and newer *desktop* release, an installed desktop updated from the
   public origin with trial and fallback, promoted to stable (A3). Then the
   re-lock and rebuild cadence that keeps Firefox current (B stage 5).
7. **Recipe/Ginux acceptance on the final source:** PRD09, 13, 15, 16 and
   PRD17 presented (A1). PRD05/07 wait on upstream; decision 5 covers the case
   where they do not arrive.
8. **Launch:** website P0, screenshots of both desktops light and dark from
   the step 3 images, release notes with the known limits, legal pages, tag,
   publish, announce (A4, A6, Gate C).
