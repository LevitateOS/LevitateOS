# recab slices

> Superseded scope, 2026-11-10: this is a historical refactoring snapshot, not the current recab product plan. Recab owns system updates and symmetric A/B continuation, including equivalent initial systems and incremental preparation that preserves pending changes through updates. Follow [the current requirements](../../tools/recab/REQUIREMENTS.md); retain the old findings below as historical evidence only.

Slices are split by what they decide (authority), what they read and write
(state) and when they run (lifecycle). They are not packages: recab stays one
crate. Every file and mixed-file duty in [inventory.md](inventory.md) maps to
one of these. Findings are in [findings.md](findings.md); target files in
[desired-tree.md](desired-tree.md).

Primary review: Claude (this audit, full source read). Challengers: three
independent agents, each given the code and the contracts but not the
primary's findings (A: S1, S2, S4, S9, S10; B: S5, S6, S7; C: S8, S11, S12, S13).

| ID | Slice | Authority | State read / written | Lifecycle | Requirements | Current files |
|---|---|---|---|---|---|---|
| S1 | Slot state machine | which transition a request makes, or why it is refused; how a trial is judged | `PersistentState`, `Selection` (in memory only) | every command | REQ-SM-001…023, REQ-DESIGN-020, REQ-LIB-003 | `state.rs`; `Slot` in `lib.rs:55-96` |
| S2 | Boot selection | what systemd-boot boots next, and changing it | EFI vars `LoaderEntryDefault/OneShot/Selected`; `loader.conf` | every command; writes in set-next/commit/rollback/stage/trial | REQ-SM-014, REQ-BACKEND-001…012, REQ-PERSIST-005, REQ-ERR-010 | `systemd_boot.rs:68-283,442-507,933-955` |
| S3 | ESP layout and entries | names of entries and kernel dirs, entry text, trial options, bootability | `loader/entries/*.conf`, `/<prefix>-a|b/` kernel dirs | init, stage, set-next, commit, rollback, status | REQ-SM-014, REQ-SM-022 | `systemd_boot.rs:24-66,285-328,363-440,598-617,645-690,722-839`; `crash_option` |
| S4 | Root and slot identity | which slot is running; which partition is a slot | `/proc/cmdline` or the mount table; entry `root=`; blkid | every command | REQ-CLI-002, REQ-SM-013 | `systemd_boot.rs:330-361,509-596,841-931` |
| S5 | Install-time layout | the first ESP layout of an installed system | target ESP, loader binary, entries, kernel, loader.conf, state, EFI vars | once, at install, from recsetup | REQ-SM-015 | `init.rs`; `install_loader` (`systemd_boot.rs:692-720`); `lib.rs:273-280` |
| S6 | Slot staging | writing a prebuilt system into the inactive slot, and its guards | inactive partition, slot kernel dir, release medium, image, `/etc/fstab` | operator update | REQ-SM-013, REQ-DESIGN-002, REQ-BACKEND-014 | `stage.rs:27-293,501-511,540-621`; `lib.rs:282-319` |
| S7 | Carried machine state | what machine state survives an update, and how it merges | image `usr/lib/recab/carried` and `usr/lib/recab/baseline`; running `/etc` | inside stage | REQ-DESIGN-002, REQ-BACKEND-013 | `carried.rs`; `services/carried`; `stage.rs:295-499,513-538` |
| S8 | Trial supervision and health | committing a healthy trial; rebooting out of a failed one; how startup ended | init state (systemctl, `/run/openrc`, runlevels, `/proc`); reboot | every boot (trial service); on demand (`health`) | REQ-SM-020…023, REQ-BACKEND-013, REQ-CLI-010 | `lib.rs:336-448`; `init_system.rs:104-283`; `package.rs`; `services/recab-trial.*` |
| S9 | Persistence | durable state, audit and run lock | `/var/lib/recab/state.json`, `audit.log`, `/run/lock/recab.lock` | every mutating command | REQ-PERSIST-001…005, REQ-AUDIT-001, REQ-SEC-003 | `store.rs` |
| S10 | Command surface and configuration | CLI shape, entry points, config, privilege, error classes, the change transaction | env `RECAB_*`, os-release profile | every command | REQ-CLI-001…011, REQ-LIB-001/002, REQ-ERR-*, REQ-SEC-001/002 | `main.rs`; `lib.rs:1-53,204-271,321-334,450-530`; `config.rs`; `error.rs`; `init_system.rs:22-53`; `Cargo.toml` |
| S11 | Machine-readable contract | JSON shapes front ends read | `docs/*.schema.json`; generated `recab-tui/src/generated/recab.ts` | `cargo xtask gen` | REQ-CLI-011 | `contract.rs`; `docs/`; `Status`/`SlotView`/`TrialRecord` (`lib.rs:98-152`); `Health` (`init_system.rs:65-102`); `slot_views` |
| S12 | Documentation and governance | what the tool promises | — | — | REQ 11 (traceability) | `README.md`, `REQUIREMENTS.md`, `CLAUDE.md`, `AGENTS.md`, licences |
| S13 | Verification | evidence for behaviour | scratch trees; VM scenario outside the repo | explicit | REQ 11.3 | `tests/lifecycle.rs`; external `testing/install-tests/src/scenarios/ab_update.rs` |

## Completion by slice

Four dimensions, counted separately. "Source" = every line of the slice's
files read by the primary. "Traced" = the workflow followed through real
callers, state and failure branches. "Challenged" = an independent challenger
reported on it. "Allocated" = every duty has an exact target file in
[desired-tree.md](desired-tree.md).

| Slice | Source | Traced | Challenged | Allocated |
|---|---|---|---|---|
| S1 | yes | yes | A | yes |
| S2 | yes | yes | A | yes |
| S3 | yes | yes | A | yes |
| S4 | yes | yes | A | yes |
| S5 | yes | yes (caller `recsetup/plan.rs:396`) | B | yes |
| S6 | yes | yes; write path only by reading (no host test writes a slot) | B | yes |
| S7 | yes | yes; merge semantics by reading | B | yes |
| S8 | yes | yes; boot ordering inferred, not observed | C | yes |
| S9 | yes; `target_root` durable helpers not re-read | yes | A | yes |
| S10 | yes | yes | A | yes |
| S11 | yes | yes; `cargo xtask gen --check` current | C | yes |
| S12 | yes | n/a | C | yes |
| S13 | yes | 20/20 lifecycle tests run | C | yes |

Complete source review does not mean the slices are correct: every slice
except S9 and S11 has open findings, and the decisions in
[ideal-filetree-plan.md](ideal-filetree-plan.md#decisions-pending) block the
tasks that depend on them.
