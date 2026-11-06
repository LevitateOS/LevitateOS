# G01/G02 preparation disposition

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
