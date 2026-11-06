# G29 — configured TCP integration (in progress)

Source reviewed so far:66de28808afd59808d6c0180a376629e41478a9c. No G29 candidate
has been extracted or runtime-validated yet. G08a `fe27c693bf2629166ee82da2efefb62b298bb567` supplies the
minimal per-process public readiness owner; G28 `0a3cf5f5` supplies the TCP backend.
Next compose those exact prerequisites, preserving their own source/CI/tests.
This needs no G07/thread, G02 guest clocks, G26 resources, archive or scalar native
clamp ancestry. Full source review, fixes and all G29 acceptance remain open.

Initial source findings to fix during extraction (no adversarial probes run):

- **High — receive ownership/copyout:** network_hooks.rs preflights receive buffers
  with copyin and writeback, consumes/charges a backend read, then copies to the
  guest without retaining undelivered bytes. It unnecessarily reads/touches the
  destination before I/O and can lose received data if final copyout fails/shortens.
  Reuse the canonical bounded vector/copy owner and retain a bounded per-socket
  received prefix until successfully delivered. EOF/zero transfers must avoid
  needless destination access; staged data must also participate in readiness.
- **High — asynchronous connection state:** SO_ERROR reads consume the native
  error, but the donor does not retain failed connection state. A later connect
  retry may see zero and incorrectly commit connected. Centralize completion and
  retain failure separately from the consumable getsockopt error result.
- **Review needed — vector/SIGPIPE semantics:** donor network vector parsing is
  separate from the later G25 helper and clamps lengths without its aggregate
  validation. Reuse that owner without importing regular-file/pwrite ancestry.
  Check guest SIGPIPE for writes without MSG_NOSIGNAL while keeping the supervisor
  protected; read/write partial effects, timeout and error mapping remain open.

Carry G24's later network Stat nlink/timestamp initialization at F02 if those fields
are absent from the minimal base; do not pull the entire archive/time stack solely
for them. Resolve shared config/FD/task/readiness joins and preserve all default
security fixtures/CI. Ordinary controlled HTTP/TLS/Git and all final consumer
closure remain unrun. #179/network policy and [DaybreakBlue](daybreakblue-handoff.md)
security acceptance remain held. No publication or maintainer messages.
