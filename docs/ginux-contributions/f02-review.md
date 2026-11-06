# F02 — combined source review (in progress)

The disposable per-process export is `/tmp/ginux-contribution-preparation/F02/source`,
with temporary index `/tmp/ginux-contribution-preparation/F02/index`. Recorded main
is `3f2fe7dfadd4b40e235d982a6a3d386dc6490177`. The37 prepared patch units are applied through that
index and the existing object database. Original branches, checkouts, indexes and
stashes remain preserved; no remote or publication action is authorized.

Current state: 22 patches applied. Pending `G17a` has conflicts in
`scripts/test-launch-storage.py`, `tests/fixtures/launch-storage-fixture.c`. Those pending patch's other changes
are already staged in the temporary index. Do not apply it again: resolve its
remaining files, stage only those files, complete that state transition, then
continue the saved order. state.json records exact ordered/applied lists, tree
checkpoints, conflicts, commands and every resolved join.

G01/G02 held native rules, G25's held native read clamp and G07's unready thread/
security snapshot are excluded. This ordinary per-process composition does not
fulfil shared-thread/compiler/security acceptance. Every original criterion and
[DaybreakBlue](daybreakblue-handoff.md) hold remains open. Core source and native
fixture preservation from F01 must be checked again on the final combined tree.

Resolved source interactions so far:

- Registered file readiness and owned storage teardown coexist as separate
  LaunchFs methods.
- Fixture build lists and default Make/CI targets are retained together; required
  static-and-unit remains. Independent fixture helpers/default/ordinary cases
  are combined rather than selecting one side's coverage.
- Per-task cwd selection precedes mapping and ordinary dispatch; the existing
  private mapping transaction remains ahead of normal syscalls.
- Cwd, hardlink, symlink and cross-namespace docs/fixtures retain their own cases;
  stale unsupported-feature descriptions are removed from the combined text.
- G18 dynamic namespace retains G15 symlink/hardlink descendant states using
  generic iterators and G10 executable-name type. The backing module is the
  already reviewed G12+G18 composition f41ed1b after verifying that current
  backing had no later changes; this preserves memfd identity and metadata/data
  charging rather than using the anonymous-only donor. All mapping/link/scale
  fixture paths remain. Namespace config/capacity is explicit.

No combined Rust build, unit or runtime pass exists yet. C fixture and Python
runner syntax was checked at the relevant merged fixture boundaries only.
An archive-export stat-cache mismatch was diagnosed with equal file/index blobs
and corrected by refreshing only the temporary index. One merge-helper parser
was corrected before staging; its guards prevented conflicted files being staged.
Shared Git common config transiently became a directory containing domain configs,
then returned to a file while inspected; this task made no config replacement.
The failed G18 stage was retried only after explicit disposable-worktree checking.
Source and temporary index remained intact. This is recorded environment evidence,
not a claim about which external actor changed metadata.

Required joins/verification still include G17b's known mapping-fixture prerequisite
correction from f41ed1b (not in G17b's delta), one canonical G25/G29 IoVectors owner,
G08b packet and G29 TCP send/recv dispatch plus dynamic reference offsets, G24
Stat fields, G20 control wait with finite external/timer ticks, G26 typed identity
query resolution and G27 configured identity in the G06 task loop. Fixes needed
by an individual standalone candidate must also be assigned back to that owner;
the combined union is not a substitute for its review/CI.

Exact next action: inspect/resolve the saved `G17a` storage fixture/runner
conflicts, preserving all current mapping/symlink/namespace and positioned-I/O
cases. Use the existing temporary-index helpers, retaining complete default
security fixtures without running adversarial probes. Continue source joins,
review the complete union, build/check it and run appropriate ordinary combined
checks and actual consumers as capabilities join. F03 then uses fresh Recipe
revisions; F04 remains user inspection before publication.

State, temporary-index snapshots, apply logs and reproducing helpers are copied
to .artifacts/ginux-contribution-preparation/F02/. The active source remains the
/tmp export; do not replace it from an old artifact snapshot during recovery.
