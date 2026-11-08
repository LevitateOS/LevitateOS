# G28 — bounded supervisor TCP backend (partial)

Current release contract: [PRD08](prd08-public-contracts.md) records the user's
configured downstream approval and explicit limits, independently of upstream
holds. The candidate/evidence below remains historical; its security, current-
main delivery and final-source acceptance are not upgraded by that decision.

Head `0a3cf5f5ab7e77080eb9491cb6bffd5ba642cb68`, tree `0256d004e560c8764907844e4f143d01374c9fb9`, solely on recorded main
`3f2fe7dfadd4b40e235d982a6a3d386dc6490177`. [Patch](patches/G28.patch):3 files,+769.
Ordinary source preparation is retained; two new donor security tests, upstream
network contract, full CI and fresh-main acceptance remain pending.

BoundedTcp owns a fixed endpoint array, fixed socket slots and cumulative sent-
plus-received byte quota. The connection limit is concurrent open slots, not a
lifetime connection-attempt counter. Transfers are capped by4096 bytes and quota;
only returned progress is charged, including across socket close/reuse. Empty
transfers consume no quota. Numeric IPv4/IPv6 grants perform no DNS; scope IDs
are not modelled. Callers own public-FD alias counts and grant selection.

The Linux x86-64 adapter owns nonblocking/CLOEXEC sockets, preserves OS errors,
uses MSG_NOSIGNAL for writes and supports endpoint metadata, NODELAY, socket error,
readiness and half-close. EINPROGRESS retains ownership until completion/close.
Readiness describes transport state; byte policy is enforced by read/write.
Socket error follows native SO_ERROR behavior, so integration must retain any
consumed asynchronous error appropriately. Close takes a live handle once,
revocation takes all handles and Drop revokes. No native handle enters a guest.
This is a trusted supervisor library API, not a new guest network permission.

Separate whole-diff self-review inspected all three changed files, endpoint
validation/representation, slot/index bounds, transfer arithmetic, connection
failure cleanup, borrowed handle ownership, raw syscall ABI/endian/layout,
readiness/error consumption, shutdown/Drop/revoke, complete fixtures and both
CI platforms. Existing device implementation and old test text are byte-preserved;
all other tracked paths, core/domain/config and launch behavior remain unchanged.
This is self-review, not independent or adversarial review.

- **CI coverage fix:** these real TCP tests are Linux-only, while the existing
  static/unit job runs on macOS. Add a Linux devices test invocation so coverage
  actually executes there. It resets RUSTFLAGS and uses the existing unit target
  directory, preserving the freestanding release build and original macOS job.
  The full new CI command is unrun locally because it includes the two security
  tests below; no green remote/default-CI claim is made.
- **Meaningful ordinary coverage:** preserve both donor tests and add two real
  local-peer cases that separate ordinary operations from denied/invalid policy.
  A channel handshake proves a read is nonblocking before the server replies;
  the actual peer address is compared with exported local metadata. No new
  runtime framework or G29 guest socket/config dispatch is introduced.
- **Build correction:** ordinary peer EOF assertions now unwrap io::Result before
  comparing its length. The initial failed static log is retained.

Source and strict clean main-plus-patch replay pass make check build, matching
all150 tracked paths and Ginux release hashes. Existing package-unit commands,
with only the two named new tests filtered from ginux-devices, pass67 routine
units in each copy: all65 preexisting units plus two new real-peer cases. Eight
device tests ran, two new ones were filtered, none ignored. This is explicitly
**not** an unfiltered make test-unit/full Linux CI pass (69 tests would run).
The unchanged base library cannot compile the public TCP API probe; the same
probe compiles with the source candidate. This is new-feature evidence, not a
claim of an existing-main TCP runtime regression.

Ordinary real IPv4 and IPv6 peers exercise connect readiness/SO_ERROR, correct
local/peer metadata, NODELAY toggles, synchronized WouldBlock without quota
charge, write half-close, request data, budget-shortened response/prefix, zero-
length I/O and exhaustion. Separate real peers observe EOF after explicit close,
Drop and repeated normal revocation; concurrent slot exhaustion also behaves as
configured. No scanning, foreign destination, malformed endpoint or bypass
experiment was run. These public-API callers make the backend useful/testable on
main without importing its later guest syscall consumer.

The following default-enabled donor tests remain **unrun**, with
[DaybreakBlue](daybreakblue-handoff.md) responsible for them and broader authority/
race/failure validation:

- tcp_endpoint_slots_budget_and_revocation_use_real_connections;
- tcp_drop_closes_connecting_handles_and_invalid_policy_never_opens.

No test is removed, ignored or weakened in default execution. Full/default units,
Linux integration, macOS/remote CI, independent review and network policy consent
are pending. G29 owns guest assignment/FD/readiness and actual HTTP/TLS/Git;
F02/F03 owns final combined/current Recipe acceptance and prior G23–G27 holds.
This standalone source packet is not ready to submit before those applicable
holds and fresh upstream review. No cloud agent was dispatched.

Patch, generic draft, exact commands/SHAs/hashes, unit filters, logs and helpers
live in .artifacts/ginux-contribution-preparation/G28/. Real Ginux branches,
checkouts/indexes/refs and remote text remain untouched. No publication or
maintainer messages; F04 remains the user inspection boundary.
