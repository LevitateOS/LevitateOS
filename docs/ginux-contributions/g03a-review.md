# G03a — create an absent workload directory correctly

Local candidate `0b563cf87a3c0e83a20a22eee3af8ffdb7adf015` has sole parent
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
