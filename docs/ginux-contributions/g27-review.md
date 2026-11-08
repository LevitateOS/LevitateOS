# G27 — configured virtual uname identity

Current release contract: [PRD08](prd08-public-contracts.md) records the user's
configured downstream approval and explicit limits, independently of upstream
holds. The candidate/evidence below remains historical; its security, current-
main delivery and final-source acceptance are not upgraded by that decision.

Head `1eb3e189525670e1056e89ef4bc49ab7f0d481dd`, tree `813671c521217a42b97e8471513e3d5b44ab0e6b`, solely on recorded main
`3f2fe7dfadd4b40e235d982a6a3d386dc6490177`. [Patch](patches/G27.patch):6 files,+119/-16.
Ordinary source preparation, including actual RPM identity queries, is retained;
#179 identity policy, security, full CI and fresh upstream acceptance are pending.

IdentityView encodes six65-byte Linux utsname fields: Linux, domain ID, kernel-
views package version plus-ginux, ginux, x86_64, and empty domainname. It validates
field length/NUL constraints before zeroing/writing the390-byte ABI output. The
launch entry handles syscall63 after x86-64 ABI validation and returns through
existing emulated-entry/exit handling. It never invokes host uname. Shared core/
native syscall policy, config schema, existing identity filesystem and sysinfo
remain unchanged. Existing valid64-byte domain IDs fit with final termination.

The supervisor's existing process_vm syscall is extracted unchanged for reuse by
a strict copyout sibling: the new uname path checks address arithmetic, requires
complete copy and never uses forced ptrace writes. The legacy helper retains its
same cached availability/error decisions and documented ptrace fallback; no
fallback policy is silently changed. A copy failure can retain a written prefix
and returns an error. Security acceptance of those paths remains unrun.

Separate whole-diff self-review inspected all six changed paths, config identifier
bounds, ABI field offsets/padding, error ordering, architecture check and emulated
entry/exit state, copy addresses/lengths, legacy fallback/cache callers, docs,
fixtures and Make/workflow coverage. This is self-review, not an independent or
adversarial review.

- **Dependency correction:** the donor's runtime hook lived in the later G06
  process supervisor. The existing main launch supervisor already owns a usable
  entry. Source is extracted directly on main without G06/G26/storage/I/O policy.
  Its future G06 task-loop join is explicitly assigned to F02.
- **Strict output preserved:** the new call does not opt into the legacy helper's
  documented forced-write fallback. Both use the same raw syscall owner without
  changing old callers. Unavailable process_vm errors propagate; no fallback or
  protection-bypass injection was run.
- **Build correction:** the initial helper extraction needed parentheses around
  an unsafe expression cast. The initial failed static log is preserved; final
  source/clean checks below are passing evidence.
- **Policy remains held:** Linux/machine/package-release/domainname choices are
  the donor's virtual identity contract, not proof of a complete host Linux ABI
  or upstream consent. No sysinfo/#219, new hostname configuration or host name/
  release passthrough is introduced.

Source and strict clean main-plus-patch replay pass make check test-unit build
build-launch-fixture:65 units,6 kernel-view tests,150 tracked paths byte-matched.
Ginux, launch fixture and ordinary wait-status fixture hashes match. Both copies
pass four fresh raw uname launches (two names, maximum valid64-byte ID and repeat),
comparing all390 output bytes and zero padding. The valid output is unaligned;
five bytes on each side stay unchanged. Existing argument/exit controls pass.
The existing native wait_status case (valid unaligned status buffer, no protected
memory or syscall injection) also passes, exercising the shared copy refactor.
Existing default launch tests and static-and-unit coverage remain intact, and
existing Make/workflow callers include the new ABI cases.

Actual RPM4.19.1.1 runs in a disposable read-only image built from hashed host RPM,
its transitive runtime libraries and explicit rpmrc/macros. The primary installed
RPM Lua documentation describes posix.uname format%s/%n/%r/%v/%m. The real command
reports Linux|rpm-domain|0.1.0-ginux|ginux|x86_64 and then the changed rpm-other
nodename, in both source and clean replay. Audit confirms configured emulation;
this is actual RPM querying uname, not a substitute script or compiled default
macro. Runtime inputs/version/hashes are saved in rpm-provenance.json. No package
installation, acquisition or full RPM conversion is claimed by this identity check.

Exact main (reused verified main binary hash) returns unsupported for the valid
ABI fixture, which exits170. The same actual RPM identity command terminates by
signal11 after syscall63 is denied and prints no virtual identity. That observed
baseline failure is recorded; no root-cause or security conclusion about RPM is
inferred and no crash exploitation was attempted. Candidate runs exit0.

[DaybreakBlue](daybreakblue-handoff.md) owns invalid/oversized/NUL identity inputs,
short outputs, protected/mutable output buffers, partial faults/address overflow,
copy unavailability/failure and signal/thread authority races. Those checks, full
Linux/default suites, independent review and macOS/remote CI are unrun. F02 must
carry uname into the configured task-loop owner while retaining defaults; F03
still needs the complete current Recipe/actual consumer closure, including G23–
G26's other outstanding acceptance. Fresh main and #179 disposition precede delivery.

Patch, generic draft, exact commands/SHAs/hashes, logs/provenance and helpers are
retained in .artifacts/ginux-contribution-preparation/G27/. No real Ginux branch,
checkout/index/ref, remote text or maintainer message changed. No publication;
F04 remains the user inspection boundary.
