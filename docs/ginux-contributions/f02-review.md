# F02 — combined per-process source (partial)

All37 prepared patches are joined in `/tmp/ginux-contribution-preparation/F02/source`.
Local unreferenced snapshot `dfebda7c29b628fc41f50635b07e8cb48d6f883a`, tree `eee8bf11a3461f2b07e726f8b582a68adb51e4a0`,
is based on recorded main `3f2fe7dfadd4b40e235d982a6a3d386dc6490177`. This is a local integration
snapshot, not a proposed PR or a fresh remote-main check. The temporary index
is `/tmp/ginux-contribution-preparation/F02/index`; state.json records exact patch
order, conflict resolutions, source snapshots, findings and receipts. Real branch,
index, checkout and stash state is preserved; nothing was published.

## Source joins and fixes

- Existing behavior owners retain cwd, executable/mapping transactions, locks,
  readiness and teardown. Mapping transactions remain before ordinary dispatch.
- Dynamic memfd/namespace backing retains aliases, directory/link states,
  accounting and mapping identity. Restore/export/checkpoints share correct
  canonical metadata and backing reads. The G22 extraction depended on a base
  that already adapted archive helpers; compilation exposed that missing join.
  `G22-archive-prerequisite-join.patch` explicitly retains it for future extraction.
- Private signal/control poll uses a1ms timeout only for finite/external guest
  readiness waits. wait4(WNOHANG) remains the task-status owner.
- One canonical IoVectors helper serves files, pipes, packets and TCP. Packets
  retain duplex direction, message boundaries, short-read discard and zero-size
  messages. FIONBIO retains its copy helper after the vector port.
- Object references place TCP after64 file+32 channel+64 device slots (start160,
  total176). Network-specific ENOTSOCK guards preserve local packet send/receive
  through the channel owner. Socket stats include link/timestamp fields.
- G26 retains typed per-process identity queries; G27 configured uname moves to
  the task entry after ABI/private transaction handling. Native core is unchanged.
- Descriptor futimens-null and fchownat-empty routing coexist. Default fixture,
  Make and CI coverage are retained together; stale combined docs are corrected.
- Mixed task selections initially accepted ordinary-resources without calling
  its resource cases. The runner now invokes those cases whenever selected and
  removes this runner-only name before fixture dispatch; source/clean reruns pass.
- G12's stale FD mapping assertion is assigned to corrected candidate
  `8bb50aaa50e80428ff417654d35affd468255775`. Its source/strict replay passes92 units,178 paths and ordinary
  mapping/storage. Full FD/security remains enabled and unrun. The first edit's
  duplicate variable failed compilation and was corrected before validation.

## Exact ordinary evidence

Source and strict recorded-main-plus-combined-patch replay pass make check,
all five Rust build binaries and all12 configured-launch fixture build targets.
All189 tracked paths match, and both binaries hash
`c01ff13db7b61b47ce959881ae140de2a250bf53f63841a60da1fca1d581d2b0`. The final snapshot only changes the Python task
selection after the first full build/replay; exact final-tree equality is checked,
and the affected source/clean task/resource/lock scenarios rerun successfully.
All compiled Rust/fixture bytes and the binary remain unchanged.

Seven unit packages pass102 selected tests in each export. The three new TCP
policy/security tests remain enabled but locally unrun, so this is not a full
make test-unit/CI pass. Source checks preserve all65 original named unit tests,
all7 package commands, required CI, exact native core/process fixtures/cgroup
module and five native protected-copy caller bodies. No security acceptance is
claimed by source comparison.

Eleven ordinary integration groups pass in source/clean: channels (including
vectors), polling, storage/mapping/link/namespace/positioned/vector/large-I/O/
timestamp/ownership/export, directories/cwd/cross paths, owned ELF, scripts,
resources/identity, locks/process lifecycle, TCP, checkpoints and full restore.
Restore covers5,000 nonempty files, archive link identity and the complete payload
above2 GiB. The corrected mixed resource selection has a separate final receipt.
A focused valid regression holds TCP alongside64 null devices, fourteen pipes,
packet vectors and owned storage, proving the joined ranges/functionality.

Actual trusted cpio extracts all8229 bytes plus hardlink/symlink/root metadata.
Actual curl IPv4/IPv6 HTTP and verified local TLS pass; actual Git ls-remote now
passes controlled HTTP and TLS through the real helper subprocess. All actual
consumer checks pass in source and clean replay. The earlier G29-only Git getcwd
failure is resolved by the configured cwd owner here, without a native core grant.
No external service, negative TLS or hostile-input scenario ran.

## Remaining work and limits

G07 threads/shared arguments remains unready and excluded. G01/G02 native policy
and G25 native clamp remain excluded. [DaybreakBlue](daybreakblue-handoff.md)
owns written adversarial validation; no agent was dispatched and no security
criterion is passed. Policy/full default/remote platform CI/current-main standalone
acceptance remain pending. Actual compiler, SQLite/Cargo and APK/RPM package
consumer closure plus the fresh complete applicable Recipe suite remain open.
This combined proof does not replace candidate-specific independence or reviews.

Evidence and helpers are retained in .artifacts/ginux-contribution-preparation/F02;
G12 correction evidence remains under its own directory. Private TLS keys and raw
mode0600 result files are excluded from evidence copies. Initial formatting,
compile/merge-helper failures and the transient shared Git config observation
remain recorded. No shared config repair was made by this task.

Exact next action: review remaining ordinary consumer requirements/current Recipe
code against this per-process composition; keep G07/security/policy and every
unperformed criterion explicit. F03 complete acceptance and F04 user inspection
remain open. No branch/ref creation, push, PR or maintainer message is authorized.
