# recupdate

The one program that talks to the release channel of LevitateOS and AcornOS. It has no
window and decides nothing: recab, the mutable updater and the apps call it. Design:
`docs/release-engineering.md` (sections 5 and 10).

## Commands

| Command | Who | What |
| --- | --- | --- |
| `recupdate check [--json]` | anyone | Fetch the channel file, verify it, say whether a newer release exists. Exit 0 whether or not one exists; non-zero with the reason on stderr when the check failed. |
| `recupdate fetch [--json]` | root | Download the release's `base`, `boot` and `recipes` archives into the cache folder and verify each SHA-256. Interrupted downloads resume. |
| `recupdate notice` | root | Run the check and write `available.json`. Run daily by a timer. A failed check keeps the previous file. |
| `recupdate notify-user` | each user | Read `available.json`; send one desktop notification per new serial (`notify-send`, else `gdbus call`). |
| `recupdate apply [--json]` | root | Exists, exits with "apply is not written yet". |

## Verification

The channel file and its `.sig` are downloaded (at most 64 KiB each), the signature is
checked over the exact downloaded bytes against every `*.pub` in the keys folder (any one
may verify; an empty folder refuses everything), and only then is the JSON parsed. Then
`schema` must be 1, `os` must equal the os-release `ID`, and `serial` must not be lower
than the installed one. Archives are downloaded to `<name>.part` (at most the manifest's
`size`, exactly that at the end), checked against the manifest's SHA-256, then renamed.
Manifest URLs must be https.

## Files

| What | Where |
| --- | --- |
| Installed release | `/usr/lib/levitate/release.json` |
| Public keys | `/usr/share/levitate/release-keys/*.pub` |
| Channel | `https://raw.githubusercontent.com/LevitateOS/releases/main/channels/<os>/stable.json` (+ `.sig`) |
| Fetched release | `/var/cache/recupdate/<serial>/` (other serial folders are removed after a fetch) |
| Last check | `/var/lib/recupdate/available.json` |
| Per-user memory | `$XDG_STATE_HOME/recupdate/last-notified` (else `~/.local/state/recupdate/`) |

Overrides (tests): `RECUPDATE_CHANNEL_URL` (the full address of the channel file; `.sig` is
appended), `RECUPDATE_KEYS_DIR`, `RECUPDATE_RELEASE_FILE`, `RECUPDATE_CACHE_DIR`,
`RECUPDATE_STATE_DIR` (the folder of `available.json`).

`dist/` holds the service files the images install: `recupdate-notice.service` and
`.timer` (systemd, daily, up to 1 hour random delay, persistent),
`recupdate-notice.periodic` (for `/etc/periodic/daily/recupdate-notice` on OpenRC) and
`recupdate-notify.desktop` (XDG autostart).

## Not done

- `apply` (mutable installs).
- Never run: written against the design, only type-checked.
