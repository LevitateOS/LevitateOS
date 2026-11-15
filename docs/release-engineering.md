# Releases and updates

Status: approved by the owner on 2026-11-15 (the decisions are in section 8).
Nothing here is built. It answers `docs/product-handoff.md` §2.10 ("one
release-engineering track for all updates"). Building starts after the §2.4
work (shipped software installed through Recipe) is finished.

## 1. What a release is

After §2.4 an installed system has three layers:

1. the base (boots the machine, runs Recipe and recab);
2. the shipped software, installed by Recipe from group recipes (desktop,
   Firefox, audio, ...);
3. whatever the user installed through Recipe.

A release fixes layers 1 and 2 together, because they are generated from one
package snapshot (`cargo xtask base groups --check` proves base + groups = the
locked set). So a release of one OS is:

- the base tree, as one archive, with a list of every file and its SHA-256;
- the kernel and boot files for it;
- the revision of the recipes catalog whose group recipes match that base;
- the desktop apps build the `rec-apps` recipe pins;
- the ISO that installs exactly this.

Layer 3 is not part of a release. Recipe already updates what the user
installed.

## 2. One manifest, signed

Each release has one small file, `release.json`:

```json
{
  "os": "levitateos",
  "version": "1.1",
  "serial": 7,
  "date": "2026-12-01",
  "base":    { "url": "...", "sha256": "...", "size": 0 },
  "boot":    { "url": "...", "sha256": "...", "size": 0 },
  "iso":     { "url": "...", "sha256": "...", "size": 0 },
  "recipes": { "revision": "<git commit>", "index_sha256": "..." },
  "notes":   "https://levitateos.org/releases/levitateos/1.1"
}
```

- `serial` is a whole number that only goes up. Systems compare serials, never
  version text. A system refuses a manifest whose serial is lower than its own.
- `release.json.sig` is an Ed25519 signature over the file. Everything else is
  trusted only through the SHA-256 values inside the signed file.
- The public key ships in the base (`/usr/share/levitate/release-keys/`). The
  folder may hold more than one key, so a key can be replaced: a release signed
  by the old key brings the new one.
- The private key is on the owner's machine only. It is never in CI and never
  in a repository.

## 3. Built

`cargo xtask build release <distro>` already builds the ISO. It gains one last
step that writes the base archive, the boot files and an unsigned
`release.json` next to the ISO. A second command signs and publishes:

```
cargo xtask release publish <distro>
```

It signs with the owner's key, uploads the files, and moves the channel
pointer (section 4) last. It is resumable, like `kernel publish`. Releases are
built locally, as all products are; CI keeps to quick checks.

## 4. Published and found

- Files: GitHub releases in one public repository, `LevitateOS/releases`, one
  release per OS and serial (`levitateos-7`). Same pattern as
  `LevitateOS/prebuilt`.
- Channel pointer: one signed file per OS in that repository,
  `channels/<os>/stable.json` (+ `.sig`), which is a copy of the newest
  `release.json`. An installed system reads only this file to learn whether
  something newer exists. Recipe reads its index the same way today.
- One channel, `stable`. More channels are not built until they are needed.

An installed system records what it runs in `/usr/lib/levitate/release.json`
(the manifest it was built from).

## 5. One program does the checking

A new small tool, `recupdate`, is the only code that talks to the channel:

- `recupdate check [--json]`: fetch the channel file, verify the signature and
  the serial, say whether a newer release exists. Needs no privileges.
- `recupdate fetch`: download the release's files into `/var/cache/recupdate`
  and verify each SHA-256. Interrupted downloads resume.

It has no window and decides nothing. recab, the mutable updater and the three
apps all call it, so there is one updater, not five.

A check happens when the user presses "Check for updates", and once a day by
a timer. The daily check only shows a notice that a release exists. It never
downloads or installs anything.

## 6. Applying an update: the base is a Recipe package

This is the central choice. Recipe already has an engine that writes a set of
files into a root, records each file's digest in a receipt, replaces a
package's old files with its new ones, keeps configuration the administrator
edited (and puts the new version beside it), and refuses to overwrite files it
does not own. The shipped software uses it after §2.4.

The proposal: the base becomes one more entry in that receipt, named `base`.
It cannot be removed, and only a signed release can replace it. An update then
is one Recipe operation on a root:

1. replace `base` with the new release's base tree;
2. install the release's revision of every group recipe the root holds;
3. run the root's effects recipe once.

The same operation serves both kinds of installation. Only the root differs.

### (a) A/B, through recab

`recab update`, and a "Check for updates" button on the recab app's one page:

1. `recupdate fetch`.
2. Make the other root a copy of the booted root (`recab copy`).
3. Recipe applies the update to the other root (the three steps above).
4. recab writes the new boot files for the other root's boot entry.
5. `recab boot-other`: the next start uses the other root once.

Nothing touches the booted root. If the download or the apply is interrupted,
the other root is simply overwritten on the next try. If the updated root does
not start, the machine boots the previous root again: that is the existing
rule, with nothing added for updates.

### (b) Mutable, through its own updater app

A mutable system has one root, so the same Recipe operation applies to `/`:

1. `recupdate fetch` (everything is downloaded and verified before any file
   changes).
2. Recipe applies the update to the running root. Running programs keep their
   old files until they restart, so the app asks for a restart at the end.
3. The new kernel is added as the default boot entry, and the previous kernel
   stays as a second entry.

If the apply is interrupted (power loss), Recipe's record of what it was doing
lets the next run finish it; the updater offers that on its first screen. A
mutable system has no second root to fall back to. That is what choosing
mutable means, and the app says so before the first update. The previous
kernel entry is the only fallback it has.

The mutable updater is its own small app (`tools/gui`), installed only on
mutable installs, the mirror image of how the recab app reaches only A/B
installs.

### (c) The apps themselves

Recipe, Recdirstat and the recab app each get a "Check for updates" button.
The button calls `recupdate check` and shows the same answer the system
updater would. The apps are part of the release (`rec-apps` is a group recipe),
so a newer app arrives with the release that carries it, through (a) or (b).
There is no separate app updater.

On other distributions, where Recipe is installed standalone
(`docs/recipe-standalone-release.md`), the same button reads a channel file for
Recipe alone and Recipe replaces its own files.

## 7. What is deliberately left out

- Downloading only what changed. The first version downloads the whole base
  archive. Fetching only the changed packages is possible later without
  changing the manifest's meaning.
- Automatic installation. Nothing is ever downloaded or applied without the
  user asking.
- Channels other than `stable`, and downgrades.

## 8. Decisions (the owner, 2026-11-15)

1. **The base is a Recipe package** (section 6). One engine updates the base
   and the shipped software on A/B and on mutable installs.
2. **Releases live in a new public repository, `LevitateOS/releases`.**
3. **Version names are `1.0`, `1.1`, ...**, chosen by the owner per release.
   The serial does the comparing.
4. **Checking: on the button, and once a day with a notice and nothing more.**
5. **The signing key is one Ed25519 key in a file on the owner's machine.**
6. **App buttons show the system release** that carries the newer app. Apps
   are not published or updated on their own.

## 9. Order of work

1. Manifest, signing and `release publish` in xtask; the public key in the base.
2. `recupdate` (check, fetch).
3. Recipe: the `base` entry and the update operation on a root.
4. `recab update` and the recab app's button.
5. The mutable updater app and its boot-entry step.
6. The three apps' buttons.

None of these steps needs an image build to be written. The first real proof
is two releases of one OS and an update from the first to the second, which
needs the owner's build go.
