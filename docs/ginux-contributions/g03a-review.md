# G03a — create an absent workload directory correctly

Current retained candidate `c28dd1321f024ec6b5d2087fd489457f31f20e2c` has sole parent
`8a475e8b2318425fd2fc15246a5c6f4bca6caf86`. The current-main refresh below owns
its fresh evidence and pending checks; the earlier results remain historical.

Earlier candidate `0b563cf87a3c0e83a20a22eee3af8ffdb7adf015` has sole parent
upstream `3f2fe7dfadd4b40e235d982a6a3d386dc6490177`. Two files, +4/-1.
This one-line correction was already included in the local10091e4e storage
commit; it is independently useful and is extracted from that larger feature.

The x86-64 launch bootstrap labels syscall259 as mkdirat. Installed ABI headers
confirm259 is mknodat and258 is mkdirat. When the configured working directory
does not exist, the bootstrap cannot enter it and returns supervisor-error2.
Use258 in the existing call and add a real missing-workdir invocation to the
launch harness, asserting the result is a directory. Existing workdir and all
other launch failure cases remain in the suite.

Full final diff/call-site review found no additional required change. The call
still occurs after chroot, before dropping privileges and entering the configured
directory; existing path validation, permissions, result reporting and failure
behavior remain in their original owners. No new syscall surface is granted to
the workload. Files retain their upstream modes. This was reviewed by the author
in a separate pass; no independent reviewer is claimed.

[Retained source patch](patches/G03a.patch).

## Verification

The new real launch scenario fails against an unmodified upstream3f2fe7df binary:
exit2, supervisor-error, configured domain workdir unavailable. With the candidate
it returns0 and the configured directory exists. Baseline binary SHA and full
failure output are retained with the package.

Both the initial isolated candidate source and a separate clean upstream export
with only this patch applied pass:

- `make check test-unit`:65 unit tests; every existing unit module retained.
- `make build build-launch-fixture build-process-fixtures build-gate1-fixture`.
- `sudo -n make test-launch`, including the new absent-workdir case.
- `sudo -n make test-native-supervision`.
- `make test-process-native`.
- `sudo -n make test-gate1-setup`.

All150 tracked source paths in the clean-applied export match the tested
candidate; only the stated two files differ from upstream. A temporary index
created the local commit object without modifying any real index/ref/worktree.
Patch, concise draft body, manifests, timings and all logs are in
`.artifacts/ginux-contribution-preparation/G03a/`.

R01–R04 pass locally. macOS Actions/full Rocky VM checks remain unrun; no remote
CI result is claimed. No branch was created/switched and no remote was changed.
User inspection of the full package remains F04. The broader G03 writable-storage
and metadata feature is still pending; this small fix does not complete it.

## Current-main refresh — 2026-11-06

The two-file patch applies to observed upstream `8a475e8b` without any other
proposal. The complete final diff remains +4/-1: syscall 259 becomes 258 and the
existing launch runner gains the three-line absent-workdir regression. Installed
x86-64 ABI headers confirm `mkdirat=258` and `mknodat=259`. The constant has one
launch-bootstrap caller; upstream's separate native `SYS_MKDIRAT_HOST` owner
already uses 258 and is unchanged.

The exact new upstream binary fails the trusted ordinary missing-workdir case:
exit 2, supervisor-error, configured workdir unavailable, and a path exists but
is not a directory. Candidate and independent clean replay both exit 0 with an
actual directory and successful fixture execution. All three runs use identical
fixture bytes. Source/call-site review confirms the call stays after chroot and
before privilege drop/chdir; path validation, modes and workload syscall grants
are unchanged. Separate self-review found no additional blocking source change;
no independent reviewer or security acceptance is claimed.

Both candidate and clean replay pass:

- `make check test-unit build build-launch-fixture build-process-fixtures build-gate1-fixture`, including all 73 current unit tests.
- The real missing-workdir regression and eight additional ordinary launch/wait
  controls (nine controls per export).

All 187 tracked paths match the retained commit in both exports. Both binaries
hash `6eec588fd18a3d052a28a8c87c9711151e7d5886745c6f97ef65d6f31baf622c`.
The base, candidate and clean builds use separate target directories. Exact
commands, source exports, baseline failure, receipts, manifest, earlier patch
and a new concise draft are retained under
`.artifacts/ginux-contribution-preparation/upstream-refresh-2026-11-06/G03a/`.
The snapshot commit is local and unreferenced; real branches/indexes/worktrees
and remote text remain untouched.

Current upstream tests and CI wiring remain intact. Full default Linux runtime
jobs contain authority/protected/fault probes and remain unrun under the written
security boundary; macOS/remote CI also remains unrun. R03 retains those checks.
Selected ordinary success is not a full CI result or publication authorization.

## PR231 latest-observed-base refresh — 2026-11-06

Head `feafbad2a374e80c18943370e7efd1263554a245`, tree
`e5ded2be4d64a14bfd641a561a51369acf1a7810`, is directly on upstream
`ff926d0c228b71d55d0f2f0909dd86ec62197c8a`. The full added/removed delta
remains two files,+4/-1. Separate self-review reread the entire delta and current
bootstrap/configured-workdir/result path; PR231's contract-plan/config changes
remain intact. Independent reviewer coverage is not claimed.

Exact upstream exits2 without creating the missing directory. Candidate source
and strict clean replay create it and exit0, with identical config/fixture hashes
across all three invocations. Both exports pass make check/test-unit/build and
launch/process/setup fixture builds:83 units, including all new upstream contract/
domain coverage, plus nine ordinary controls each. All196 tracked paths match
the head, and both binaries have SHA256
`be9f2eb9111846a10d91aaf6d9b4e01946b8a50d4e6ad1b0b386a71b37296d17`.
Logs, receipts, draft, manifest and patch are in
`.artifacts/ginux-contribution-preparation/upstream-contract-refresh-2026-11-06/G03a/`.

Full default runtime/security suites, macOS and remote CI remain pending; ordinary
launch success does not establish new contract enforcement. No actual checkout/
index, branch, remote text or publication changed.
