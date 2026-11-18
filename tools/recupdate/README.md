# recupdate

The one program that talks to the release channel of LevitateOS and AcornOS. It has no
window and decides nothing: recab, the mutable updater and the apps call it. Design:
`docs/release-engineering.md` (sections 5, 6b and 10).

## Commands

| Command | Who | What |
| --- | --- | --- |
| `recupdate check [--json]` | anyone | Fetch the channel file, verify it, say whether a newer release exists, and whether an apply was interrupted. Exit 0 whether or not one exists; non-zero with the reason on stderr when the check failed. |
| `recupdate fetch [--json]` | root | Download the release's `base`, `boot` and `recipes` archives into the cache folder and verify each SHA-256. Interrupted downloads resume. |
| `recupdate fetch-parts [--json] [--categories a,b] [--from DIR] [--list]` | root | Download the category parts of the release this medium was built from, verify them and unpack them into Recipe's package cache (below). |
| `recupdate notice` | root | Run the check and write `available.json`. Run daily by a timer. A failed check keeps the previous file. |
| `recupdate notify-user` | each user | Read `available.json`; send one desktop notification per new serial (`notify-send`, else `gdbus call`). |
| `recupdate apply [--json]` | root | Install the channel's release on a mutable installation (below). Refuses an A/B installation and names `recab update`. |

`check --json` prints `{"schema":1,"os":…,"installed":{"version","serial"},"available":null | {…},"interrupted":null | {"version","serial"},"checked_at":…}`.
`interrupted` is set while `/var/lib/recupdate/applying.json` exists: an apply began changing
the system and did not finish, so the updater offers "Finish the update" (which runs
`recupdate apply`). An unreadable record shows as `{"version":"","serial":0}`; `apply` then
says what is wrong with it.

## Verification

The channel file and its `.sig` are downloaded (at most 64 KiB each), the signature is
checked over the exact downloaded bytes against every `*.pub` in the keys folder (any one
may verify; an empty folder refuses everything), and only then is the JSON parsed. Then
`schema` must be 1, `os` must equal the os-release `ID`, and `serial` must not be lower
than the installed one. Archives are downloaded to `<name>.part` (at most the manifest's
`size`, exactly that at the end), checked against the manifest's SHA-256, then renamed.
Manifest URLs must be https, except that a channel given as an `http://` address (a local or test channel via `RECUPDATE_CHANNEL_URL`) may list `http://` archives; the public https channel never can. A resumed `apply` checks the cached `release.json` against its
`.sig` and the keys again.

## `recupdate fetch-parts` (the installer's package cache)

The release ISOs carry no package cache. The default software set's downloads are published
as one **part** per category (`parts-<category>.tar.zst`, listed in the manifest's `parts`
array; design: `docs/release-engineering.md` section 11), and recsetup's software step runs
this command before Recipe installs the set.

- **Which release.** The one the running medium was built from: `os` and `serial` come from
  `/usr/lib/levitate/release.json`. `release.json` and `release.json.sig` are downloaded from
  that release's own assets, `https://github.com/LevitateOS/releases/releases/download/<os>-<serial>/`
  (at most 64 KiB each; never the channel pointer, which may already name a newer release),
  and verified exactly as for `check`, except that `serial` must be **equal** to the medium's.
  `--from DIR` takes `release.json`, `release.json.sig` and the part files
  (`parts-<category>.tar.zst`) from a folder instead of the network, with the same checks;
  the folder is read, never changed.
- **Each part.** Downloaded to `<cache>/parts/<serial>/<name>.tar.zst.part`, resumed after an
  interruption, checked for size and SHA-256 against the signed manifest, renamed, then
  unpacked into the recipe cache folder (default `/usr/share/recipe/cache`, the literal of
  `install_tools::RECIPE_CACHE_DIR` in `tools/recstrap/src/lib.rs`; keep them equal). The
  downloaded archive is removed after unpacking. `<cache>/parts/<serial>/<name>.done` holds
  the part's SHA-256 once it is fully unpacked; a part with that record is skipped.
  `--categories a,b` limits the run to those categories (default: every part of the manifest).
- **Unpacking.** Regular files, folders and links that stay inside the cache folder. An
  absolute path, a `..`, a link that points outside, a hard link or a special file ends the
  run with an error. Nothing is written through a symbolic link (a folder component that is
  one is refused).
- **`--list`** downloads nothing but the manifest and prints
  `{"parts":[{"name","category","size","present":true|false}]}` (`present`: unpacked already).
- **Events** with `--json`: `step` (1 is the manifest, then one per part), `progress`
  (bytes of the current part), `done` (`restart_needed: false`) or `failed`, as for `fetch`.
- URLs in the manifest must be https, except when the base address was given as an
  `http://` override (`RECUPDATE_RELEASE_BASE_URL`), the rule `fetch` applies to a channel.
- Overrides (tests): `RECUPDATE_RELEASE_BASE_URL` (the folder address holding `release.json`;
  a trailing `/` is added), `RECUPDATE_KEYS_DIR`, `RECUPDATE_RELEASE_FILE`,
  `RECUPDATE_CACHE_DIR`, `RECUPDATE_RECIPE_CACHE_DIR`.

## `recupdate apply` (mutable installations)

**Which installation.** recab's facts decide: the kernel command line carrying
`recab.immutable`, `/var/lib/recab/installation.json`, or `/usr/lib/recab/immutable` mean
A/B, and `apply` refuses with "update it with `recab update`". Without any of them the
installation is mutable (recsetup `--boot mutable` puts none of them there). The boot step
also needs what recsetup's mutable boot step (`recab init --mode mutable`) leaves: the ESP
mounted as vfat at `/boot` and a systemd-boot entry for the running root.

**Steps** (`Step n of 5: …` on stdout, or the §10 JSON lines with `--json`; `progress`
lines of the downloads and of Recipe are passed through):

1. Fetch: the channel's release into `/var/cache/recupdate/<serial>/`, everything
   downloaded and verified before any file changes. If its serial is not above the
   installed one, `apply` ends with "up to date" (`done` with `restart_needed: false`).
2. Write the intent record `/var/lib/recupdate/applying.json` (atomically):
   `{"schema":1,"version","serial","folder","step":"apply-release"|"boot"}`.
3. `recipe --system apply-release <folder>` (Recipe targets `/` on a mutable system). The
   record then moves to `"step":"boot"`.
4. Boot: `boot.tar.zst` is checked against `release.json`'s SHA-256 and unpacked to
   `/run/recupdate/boot-files` (exactly the two regular files `vmlinuz` and `initramfs.img`,
   nothing else). On the ESP (`/boot`):
   - `/<ID>-<serial>/vmlinuz` and `/<ID>-<serial>/initramfs.img` (`<ID>`: os-release `ID`);
   - `loader/entries/<ID>-<serial>.conf`, in the format recab writes for the mutable entry:
     ```
     title <NAME> <version>
     linux /<ID>-<serial>/vmlinuz
     initrd /<ID>-<serial>/initramfs.img
     options <the booted entry's options, unchanged: root=PARTUUID=… ro rootwait panic=10 …>
     ```
   - the default: `loader/loader.conf`'s `default <ID>-<serial>.conf`, then the firmware's
     `LoaderEntryDefault`; a leftover `LoaderEntryOneShot` is cleared;
   - the entry the system booted with (systemd-boot's `LoaderEntrySelected`, else the one
     entry of this system whose `root=` is the running kernel's) stays as the second entry;
     this system's other entries (`<ID>.conf`, `<ID>-<serial>.conf`) and the `/<ID>`,
     `/<ID>-<serial>` directories no remaining entry loads from are removed. The booted
     entry's file and the files it loads are never written or removed.
5. Remove the intent record; `done` with `restart_needed: true`.

Before step 2, `apply` checks everything the boot step needs: the ESP mounted read-write at
`/boot`, efivarfs mounted read-write (the installer set `LoaderEntryDefault`, which wins over
`loader.conf`), and the booted entry found with the running kernel's `root=`. Any of these
missing fails before anything changes.

**Failures.**

| Step | What is left | What to do |
| --- | --- | --- |
| 1 Fetch | Nothing changed (partial downloads stay in the cache). | `recupdate apply` again; downloads resume. |
| 2 Intent record | Nothing changed. | `recupdate apply` again. |
| 3 Recipe | The intent record (`step: apply-release`); Recipe's own record of its part. The system may be partly updated. | `recupdate apply` again: it runs Recipe again on the same folder (Recipe resumes its part), then step 4. |
| 4 Boot | The intent record (`step: boot`). The release is applied to `/`; the previous entry stays the default. Files this step wrote are removed again; if the firmware variable could not be set, `loader.conf` is put back. | `recupdate apply` again: it runs step 4 only. |
| 5 | The new entry is the default; only the record removal failed. | `recupdate apply` again finishes. |

If the cached folder of an interrupted apply is gone or no longer verifies, `apply`
downloads the channel's release (never below the installed serial) and continues with it
from step 3.

## Files

| What | Where |
| --- | --- |
| Installed release | `/usr/lib/levitate/release.json` |
| Public keys | `/usr/share/levitate/release-keys/*.pub` |
| Channel | `https://raw.githubusercontent.com/LevitateOS/releases/main/channels/<os>/stable.json` (+ `.sig`) |
| Fetched release | `/var/cache/recupdate/<serial>/` (other serial folders are removed after a fetch, except the one an interrupted apply names) |
| Last check | `/var/lib/recupdate/available.json` |
| Interrupted apply | `/var/lib/recupdate/applying.json` |
| Apply lock | `/run/lock/recupdate-apply.lock` |
| Per-user memory | `$XDG_STATE_HOME/recupdate/last-notified` (else `~/.local/state/recupdate/`) |

Overrides (tests): `RECUPDATE_CHANNEL_URL` (the full address of the channel file; `.sig` is
appended), `RECUPDATE_KEYS_DIR`, `RECUPDATE_RELEASE_FILE`, `RECUPDATE_CACHE_DIR`,
`RECUPDATE_STATE_DIR` (the folder of `available.json` and `applying.json`),
`RECUPDATE_RECIPE_BIN` (the program `apply` runs instead of `recipe` on `PATH`); the
`fetch-parts` overrides are listed in its section. The ESP,
efivarfs, `/proc/cmdline` and the A/B facts have no override: `apply` reads the running
system's.

`dist/` holds the service files the images install: `recupdate-notice.service` and
`.timer` (systemd, daily, up to 1 hour random delay, persistent),
`recupdate-notice.periodic` (for `/etc/periodic/daily/recupdate-notice` on OpenRC) and
`recupdate-notify.desktop` (XDG autostart).

## Not done

- Never run on a real or stand-in system: only type-checked (`cargo clippy`). `fetch-parts`
  was run once with `--from` against a hand-made signed stand-in release (list, unpack,
  skip of an unpacked part, a `..` archive, a bad signature); its network path and `apply`
  have not run.
- The previous kernel entry is a working fallback only if that kernel's modules
  (`/usr/lib/modules/<old release>`) are still on `/` after Recipe replaced `base`. Nothing
  keeps them yet: Recipe's `apply-release` (or the base's layout) has to.
