# PRD07 first proposed publication: bounded TCP library

**Unpublished; permission required.** Intended fork branch:
`LevitateOS/ginux:contrib/bounded-tcp-backend`; target `gmkurtzer/ginux:main`.
Open one ordinary PR after pushing the exact two commits ending at
`59f855ae7dbd60bacb93ef865928701b04305df9`. No upstream comment or change to an
existing PR is included in this request. Check main and duplicate PRs immediately
before publication; a changed source requires renewed validation.

Title: **feat(devices): add bounded supervisor-owned TCP backend**

The following body is ready to submit:

---

`ginux-devices` gains a supervisor-owned TCP library with an explicit numeric
IPv4/IPv6 endpoint set, concurrent socket limit and cumulative sent-plus-received
byte budget. The Linux x86-64 adapter uses private nonblocking, close-on-exec
sockets. It exposes readiness, connect errors, endpoint metadata, TCP_NODELAY and
half-close; close, drop and revocation release owned handles. Public descriptor
alias counts belong to the caller.

This is independently usable from the device library. It introduces no guest
network configuration, socket dispatch, DNS, listening sockets or native policy
change. Each transfer is bounded to 4,096 bytes and charges actual progress;
ungranted endpoints fail before opening a connection. Failed connects preserve
the byte budget and slots can be explicitly released and reused.

Validation on main `53a54addea67969abaa240ea478def1b94130eee`, candidate
`59f855ae7dbd60bacb93ef865928701b04305df9`: `make check`, `make test-unit`
(94 cases, including all five new TCP cases),
`make build build-launch-fixture build-process-fixtures build-gate1-fixture`,
`sudo make test-launch`, `sudo make test-native-supervision`,
`make test-process-native`, and `sudo make test-gate1-setup` passed on Linux.
The TCP cases use real local IPv4/IPv6 peers and cover endpoint denial, budgets,
slot exhaustion, nonblocking progress, metadata, half-close, refused connect,
drop and revocation. CI runs the device units on Linux in addition to the
existing jobs. macOS execution is not claimed locally.

---

Complete diff: [PRD07-tcp.patch](patches/PRD07-tcp.patch).
Review and reproduction: [PRD07 candidate review](prd07-candidate-review.md).
This draft and its green checks do not complete G28 or PRD07; actual upstream
review/merge or an agreed maintained equivalent is still required.
