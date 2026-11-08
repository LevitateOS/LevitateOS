# PRD04 — request for verification in the authorized Daybreak context

**Pending results.** The user chose “I’ll obtain Daybreak results” in the general
engineering chat. This file supplies the pinned source and requested bounded
verification; it does not authorize security execution in the general chat or
dispatch another agent. Do not push, publish, comment upstream or modify the
pinned source. Keep probes and fresh disposable roots under ignored artifacts.

## Candidate and comparison

- Worktree: `/home/vince/levitate-development/ginux-prd04`.
- Clean candidate: `5449e2a184bb1147cf618b140a29196b74206b13`.
- Base: `5300f5ef9a8281f4f10f5519d5eb761654e04829` (PRD02 local integration).
- Launcher: `target/x86_64-unknown-linux-gnu/release/ginux`, SHA-256
  `95bc570e92491b1d30c7a14ef66593b0cfa05249b1424ffbd0047e6ea7866eb0`.
- FD fixture: `.artifacts/launch/launch-fd-fixture`, SHA-256
  `ddd137a282d8ff1da795dc7be84295b4b0d716a3fdec233f6299a89d4d71e6ce`.
- Storage fixture: `.artifacts/launch/launch-storage-fixture`, SHA-256
  `9161290fd69fcd3a04ef8aa9712d99ee33edd9adb4c9efe65b8af9eaeb227aa5`.
- `.artifacts/prd04/candidate.json`, `candidate.patch` and `commits/` retain
  exact identities and all four corrections. Compare base and candidate using
  separate exports/build outputs; preserve the existing integration and PRD03
  worktrees. This candidate is not the PRD03 upstream foundation or a PRD05 merge.

The engineering chat passed `make check`, `make test-unit` (138), `make build`,
fixture builds, FD `limits`, storage `growth bounds quota ordinary-mapping
ordinary-unlinked-mapping` and checkpoint `ordinary`. It has **not** run the
security cases below. A restricted-sandbox unit attempt failed on loopback socket
permissions; the unchanged full unit suite passed with normal host socket access.

## Required disposition of each report

1. **High-bit FD / policy authority.** Reading FD `0x100000004` as public FD 4
   matches Linux's 32-bit FD arguments and the existing fixture's low-32-bit
   close assertion. Do not retain “every high-bit alias must be EBADF” as an
   unsupported 64-bit canonicality promise. Independently verify legitimate low/
   high-bit aliases behave identically for native, owned, console/pipe aliases
   and invalid/private backend numbers, and that each remains behind the public
   table. A genuine source gap was found: ordinary non-console/non-TCP `read`
   and `readv` were not classified as filesystem reads. The new regression opens
   an owned file with write authority, denies filesystem reads, and requires both
   low/high-bit read/readv calls to return EPERM without moving the offset.
   Run the selected command below and retain its four denial audit records.
   The base should fail this regression at guest code 62; verify the failing
   baseline rather than inferring it from the final pass. Keep console/TCP policy
   identities distinct. This does not claim every possible way to observe file
   bytes is governed solely by the filesystem-read operation.

2. **G17b inherited-host-FD budget.** Refresh the original four controls in
   `ginux-current-g17b-inherited-budget-881d46fa/`: 80 distinct nonempty files,
   `max_objects=100`, inherited FD counts 0/120/190 with soft/hard limit 256,
   and 190 with limit 400. The candidate must complete all 80 creations in each
   case, without raising the host hard limit or public table capacity (256 max;
   preserve a lower operator soft limit). The source closes undeclared inherited
   supervisor FDs before reserving private backing, retaining stdio, its newly
   opened result FD and the explicitly passed coordinator socket. Verify parent
   FDs remain intact, the coordinator channel still works under pressure and no
   undesignated inherited descriptor becomes public. Recheck the lower public
   limit 8 and impossible hard-limit startup refusal. The original base's 190/
   256 mid-workload EMFILE remains the red control, not an accepted result.

3. **Assignment prefixes / B1.** Refresh the saved `/link -> /work` plus
   `memory:/link/nest` configuration-acceptance control. The candidate must refuse
   it at startup with a structured error before init/restore effects. Also test
   an image regular-file prefix, nested links, legitimate real-directory and
   missing-parent prefixes, and final-component replacement of a link/file.
   The latter is intentionally allowed and hides the image entry. Do not turn
   the old acceptance-only observation into an unobserved host oracle. Confirm
   the original image/host sentinels remain unchanged. Existing argument guards,
   interpreter/exec owners and namespace resolution are preserved; report any
   independent failure instead of extending acceptance to them automatically.

4. **B3 stack accounting.** The integration uses mmap-backed namespace/backing
   metadata, not the old inline VFS array on the stack. Refresh the valid 16 MiB
   instance / 3 MiB process-stack controls with no assignment and a valid memory
   assignment. Use a valid prefix, since `/link/nest` now correctly fails #3.
   The old `memory_mib=2` parser rejection is not a stack defect or a positive
   control. Do not infer a general stack bound from these selected passes.

5. **G17b backing close-error completion.** Refresh the retained supervisor
   `memfd:ginux-storage` close injection from
   `ginux-current-g17b-close-fault-881d46fa/`. Base behavior was panic and an empty
   result. Candidate expectation: exit 1, parseable `supervisor-error`, no panic,
   settled tasks/public-owned handles, no subsequent guest resume if the error
   occurs during an unlinked-file final close. Check both workload-time and
   final teardown timing; distinguish an injected skipped close from real Linux
   close-error FD lifetime. The owner now latches the close error, clears
   remaining backing and uses the existing completion protocol, without retrying
   a numeric FD. Confirm failure never becomes an accepted export/checkpoint.

6. **Private backing/transfer lifetime.** Refresh the saved unlinked backing /
   CLOEXEC snapshots in `ginux-current-g17b-backing-lifetime-881d46fa/` and the G12
   ordinary-unlinked-mapping, private-FD visibility, Pair failure, failed map,
   send/receive/Close error and rollback controls applicable to the recorded
   failure. Verify backing survives required aliases/maps, final namespace/FD
   release reclaims the supervisor reference, temporary descriptors are not
   public, held peers do not gain them, and failure/death leaves no live task or
   accepted success. Reuse retained bounded probes and distinguish a skipped
   synthetic syscall from an actual kernel operation. Do not claim the entire
   G07/G12/Daybreak threat model or unrelated PRD09 schedules are covered.

The original records and scripts are under
`/home/vince/levitate-development/ginux-recipe-integration/.artifacts/recipe-integration/daybreak-local-security-2026-11-07/`
and linked from `daybreakblue-handoff.md`. Adapt only the selected launcher,
current `configured-balanced-v2` config and receipt paths where needed; retain
exact fixture/runner hashes and explain any changed assertion.

```sh
cd /home/vince/levitate-development/ginux-prd04
git rev-parse HEAD
git status --porcelain
sha256sum target/x86_64-unknown-linux-gnu/release/ginux .artifacts/launch/launch-fd-fixture
sudo python3 scripts/test-launch-fds.py target/x86_64-unknown-linux-gnu/release/ginux .artifacts/launch/launch-fd-fixture policy-read
# Existing ordinary lower-limit comparison:
sudo python3 scripts/test-launch-fds.py target/x86_64-unknown-linux-gnu/release/ginux .artifacts/launch/launch-fd-fixture limits
```

Return a durable exact-source receipt for **each numbered item**: source and
binary/fixture/runner hashes, exact commands, baseline versus candidate
observations, exit/result/audit, host/image postconditions and any limitations.
A fresh failure remains open and needs correction/retest; an unrun case is not a
pass. For an invalid historical criterion, record the ABI/authority rationale and
its replacement negative and positive checks. The general chat will wait for
these results before marking PRD04 complete.
