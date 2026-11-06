# F03 — fresh Recipe consumer verification (partial)

Recipe snapshot `bde746221988094d4a0b80bb46808beab447e3dc` plus exact tracked dirty overlays
`.gitignore` and `tests/e2e/preseed_recipes.rs` is exported to
`/tmp/ginux-contribution-preparation/F03/recipe-source`. Their hashes and the
source/workspace lock provenance are in state.json. Another task owns these
live edits; this task did not alter its checkout. This snapshot is exact evidence,
not a claim that concurrently changing source remains identical afterward.

Ginux is combined `dfebda7c29b628fc41f50635b07e8cb48d6f883a`, binary SHA256
`c01ff13db7b61b47ce959881ae140de2a250bf53f63841a60da1fca1d581d2b0`. Prepared Git/musl image is
`9f220633d34efff68b1535375fbb4fa3c4c2c8843917859fd68d8f6edec27ff4`. The immutable Recipe binary hashes are
`931052585f10422f0200c78f8d542b3850c3383daeb073f6c484ec24380922d8` (CLI/worker) and
`be94a4c334fa1cac64f20d9d2c201378746df639e587ac9ce4dc06af98e247fb` (integration harness).

The initial locked standalone build correctly refused the workspace-shaped lock.
Offline normalization removed99 unrelated workspace package entries and added
no package name/version/source tuples; the e2e build then passes with --locked.
Original checkout/lock remained untouched. This is a fresh build of the snapshot,
not reuse of historical T25/T27 executables.

The current e2e catalog has135 scenarios. First explicitly selected ordinary
managed define/cleanup-reason propagation passes1/1 in44.94s. The next12-case
ordinary batch is **interrupted at the user’s pause**. It covers valid module/dependency/lifecycle,
cleanup/null/bash/owned-ELF/state/storage metadata and actual C compilation;
selection and exact command are saved. No full-suite pass is claimed.

F02 also now proves actual two-process SQLite locks/data integrity in source and
clean replay. Actual Cargo/Rust compiler, APK/RPM package paths, remaining ordinary
Recipe scenarios and every applicable full-suite criterion remain pending.
G07 thread/shared-argument snapshot remains excluded and unready. Security-looking
worker spoofing, archive/path/authority/protected checks, adversarial fault/cancel
and negative TLS checks remain for [DaybreakBlue](daybreakblue-handoff.md),
enabled and locally unrun. No cloud dispatch or security acceptance is implied.

Evidence/helpers are retained under .artifacts/ginux-contribution-preparation/F03;
/tmp holds exact source and binaries. Do not replace those from changing live
checkouts. Exact next action: inspect the active ordinary batch log/state, diagnose
any normal failures within approved capability/policy boundaries, then continue
ordinary consumers. Complete F03/F04 and publication readiness are not established.
No pushes, PRs, branch/ref creation or maintainer messages are authorized.

## Paused checkpoint

The user paused the goal. The active batch and its verified task descendants
were stopped; no tests are left running from that batch. Partial batch results:
7 passed, 4 failed, 1 interrupted/unreported. The separate first probe
passed. No full-suite success is claimed. Dependency compilation and the Bash
case expose command-output reader thread creation returning ENOSYS: the ordinary
per-process union excludes held G07. Other failure details remain in the log for
review on explicit resume. Do not replace thread support with a workaround or
claim complete Recipe acceptance.

On explicit resume: inspect state.json/ordinary-batch.log and the interrupted
case, then continue only authorized ordinary work. The G07 shared-path security
correction/verification remains assigned to the written DaybreakBlue handoff.
