# G01/G02 policy and preparation disposition

[PRD08's supported contract](prd08-public-contracts.md) owns current release
policy. The user explicitly approved the versioned configured downstream
contract, including local clocks, while upstream/native holds remain separate.
Candidate `4aa51c1d` preserves clock implementation `a7ce73cd` byte-for-byte;
actual 300-second, interruption and guest-only realtime-adjustment acceptance
passes under session libvirt, and all six Recipe clock consumers pass.

There is no local user clock hold. #207's policy/shared-array report remains
upstream work; its head is untouched and its kernel-array path is not the
configured public-descriptor poll owner. #179/#163/#178/#219 still block their
own upstream/native proposals. They do not remove the user's required configured
behavior. PRD01 still selects the actual maintained source; publication remains
separately authorized. See PRD08 for all supported limits and delivery impacts.

## Historical review records

These records preserve the active holds from P02; they do not turn pending policy
into approval or remove these packets from the final package.

G01 is existing PR#207 at e7f5f552. Its entire8-file bounded-poll diff was reviewed:
count/time bounds, checked pointer arithmetic, stdio/negative-FD restrictions,
POLLNVAL, both native callers and fixture/build wiring. No new code blocker found.
Its head and remote text remain untouched. No new source extraction, fresh local
runtime result or fresh remote CI status is claimed. Keep the current policy hold;
there is no duplicate PR candidate to create.

G02's existing local9ec95527 core/fixture delta was reviewed as donor material.
It exposes native realtime/monotonic clocks; fd/PID clock encodings stay denied.
The old task fixture covers passage, past absolute deadlines, rejected IDs/flags,
invalid pointers and timespecs. Actual interrupted-sleep evidence and a current-main
standalone fixture remain needed for R02/R03. The source remains local and held
under #179; its patch and review limitations are retained in
`.artifacts/ginux-contribution-preparation/G02/`. Do not label it fully validated.

Proceed one task at a time with independent ready work while these publication
contracts remain unresolved. F04 must distinguish these holds and missing checks
from the independently validated G05a correction.


## Local consumer decision — 2026-11-07

On local Ginux436c4ae5, the ordinary Recipe sleep/deadline case fails because
coreutils sleep cannot read the realtime clock (ENOSYS). Upstream #179 still
retains the contract hold. The user explicitly chose to keep clock policy held
and continue independent cases. No local configured clock/sleep fix was added.
That failed case and the unrun shared-destination case using sleep remain
required/pending; neither is counted as passing or waived. See the latest F03
checkpoint and clock-held-independent-selection.json for the exact evidence.
