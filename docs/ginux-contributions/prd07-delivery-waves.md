# PRD07 — dependency-ordered delivery (open)

PRD07 is active and remains **open**. Preparation, a draft, a published PR and a
merged/upstream-equivalent capability are distinct states. Completion requires
actual delivery dispositions for every required capability; no collection of
local green results closes this item. No push, PR or upstream comment is yet
authorized in this task.

## Current source and publication boundary

Read-only refresh finds main `53a54addea67969abaa240ea478def1b94130eee`.
Required existing #232/#233/#234/#236/#238/#243 remain open. Each maintainer
comment says review follows foundation work and requests no additional changes.
Keep their published heads unchanged while inspecting current-main compositions.
#207 remains separately policy/race-held; PRD08's user-approved downstream
contract is not maintainer consent to publish policy expansions.

The storage feature remains prepared at `9802ef03` over local prerequisite
composition `f1c3d3a7`. It is not standalone against main. Its chain is:
shared descriptions #232; mkdirat #233; signal-driven-I/O guard #238;
resolved contract/audit #243; zero-fill; inherited directories #236;
explicit configured catalog 2; typed VFS/assigned storage with catalog 3.
Each actual merge must be inspected and the next contribution revalidated on
that source. #234 is an independent process owner, not a storage prerequisite.

The frozen integration `5300f5ef`, storage `9802ef03`, PRD04 `5449e2a1` and
PRD08 successor `4aa51c1d` remain donor/evidence sources, not a merged release.
PRD04's failed final-export publication and required focused Daybreak retest
still block acceptance of its export family. PRD05 owns actual merged-source
composition. PRD01 owns final maintained-source packaging; PRD08 did not
silently authorize a fork publication or replacing the upstream route.

## Concrete waves and every retained capability

These are delivery boundaries, not permission to publish a batch. A wave may
contain several separate PRs; share a PR only for inseparable behavior/tests.
Every successor is prepared against actual then-current main after its required
merges. A local stack on an imagined future base is not a standalone candidate.

| Wave | Contributions / complete capability coverage | Required boundary and present delivery state |
| --- | --- | --- |
| 0: existing foundation review | #232 shared descriptions (G05a); #233 bootstrap mkdirat (G03a); #238 signal-driven-I/O refusal; #243 resolved contract/audit; #236 inherited host-directory protection. Independent #234 orphan accounting (G06a). | All open; leave published heads unchanged. [Complete diff/failure-path review and exact current-main CI](prd07-candidate-review.md) passed for all six prospective merge trees. Wait for maintainer review/merges, inspect actual merge diffs. No repost, duplicate PR or comment is currently proposed. |
| 1: independent library | G28 bounded supervisor TCP, separate from configured socket dispatch. | Direct main candidate `59f855ae`, full diff, failure review and Linux CI complete. [Exact PR draft/publication request](prd07-tcp-pr-draft.md). Publication permission pending; upstream/equivalent delivery still absent. |
| 2: storage foundation | Separate zero-fill/empty-write fix; versioned configured contract/catalog 2; PRD03 typed VFS + assigned-storage launcher/catalog 3 (replaces G03b/G03c). | Zero-fill `a6dd2f9a` passes exact-source CI and red/green controls but conflicts with #232; wait for #232 merge, rebase and revalidate. Catalog 2 and storage 3 are presently local prerequisites `f1c3d3a7` / feature `9802ef03`. #233/#236/#238/#243 must be delivered before storage. Catalog-2 configured policy is user-approved downstream, not yet an upstream-accepted extension. A permissioned maintainer proposal/disposition is still needed before treating those catalog versions as upstream. No storage PR is ready on current main. |
| 3: descriptors and task ownership | G04 assigned null; G05 public FD/pipe owner; G06b configured child/exec lifecycle joined with #234; G07 per-task transactions/shared resources/32-task and musl clone forms; G01/G08a bounded readiness; G08b owned local packets. | Storage wave + #234 required. Port current owners, not obsolete G07 snapshot. Include full blocking writes, creation-mask inheritance, empty-path/reopen rights, bootstrap signal-mask clearing and thread-directed SIGPIPE with their introducing owners. Preserve birth rollback, final alias/slot reuse, parked retries and cancellation. Current donor `5300f5ef` is reviewed/accounted, not standalone delivery. |
| 4: data, namespace and transfer | G09a/b generic + growable storage together; G09c truncate; G09d directory operations; G18 bounded capacity; G17a pwrite; G17b private reserve/backing; G10 owned ELF; G11 scripts/interpreters; G12 mappings; G13 symlinks; G14 cwd; G15 hardlinks; G16 unified image/assigned traversal. | Introduce each useful capability separately after its real predecessors, preserve typed flags/modes/seek and existing PRD03 tests. Include index extent/NUL hardening, mapped metadata, sealed transfer/backing and shared argument/kernel-use holds under the respective owner. Port T-02 full 9,000-byte copy-up/mid-4,096 rollback and reclaim; port T-01 alias mmap position=3. PRD04's ABI/read classification, assignment prefixes and inherited-budget fixes are retained; join Recipe's declared scratch lease lifetime (R-02) without widening public limits. Exact post-redesign extraction/CI still pending. |
| 5: persistence | G19 flush/quiesced export; G20 private checkpoint/control/event waiting; G21 sealed startup restore; G22 hardlink archive producer/consumer together; G31 feature-owned CI wiring. | Task/storage/namespace/transfer waves required. Include pinned output inode/parent, structured sticky close errors, sealed restore identity and original corruption/failure controls. F-01 is failed: final export publication must wait for successful required cleanup. User-authorized separate correction and focused Daybreak item-5/affected-controls retest are required before export acceptance/publication. Preserve frozen `5449e2a1` and its failed receipts. No acceptance from local error reporting alone. |
| 6: consumer semantics | G23 virtual file locks; G24 inode times/archive mtime; G25 bounded vectors/chunked regular I/O; G26a child resource bootstrap and G26b queries; G27 configured identity; G30 virtual owned uid/gid and target metadata. | Use final inode/task/storage owners. Preserve contention/release/reuse, partial progress, per-process AS/real eligible CPU IDs, exact 64 MiB configured read bound, creation masks, metadata round trips and real SQLite/Cargo/APK/RPM/cpio consumers. Carry PRD08 `4aa51c1d` configured-only corrections; preserve native bounds and catalog 1. These are explicit retained requirements awaiting extraction, not deferred product exclusions. |
| 7: configured networking and acceptance | G29 numeric TCP FD/syscall/readiness join, using G28; full HTTP/TLS/Git acquisition and cancel/revoke behavior. Final G01/G02 configured clock/readiness contract. | Descriptor/task/readiness + delivered backend required. G29 stays default-denied and needs its own reviewed versioned configured contract. PRD08 clock source and bounded configured contract have local acceptance; native #179/#207/#163/#178/#219 decisions remain held separately. Final maintained-source packaging is PRD01, merged composition PRD05, security PRD09 and real product acceptance PRD10–19; none is replaced by these drafts. |

The [PRD06 per-hunk/test ledger](prd06-reconciliation.md) remains the exhaustive
accounting of every correction, packet, dirty-file hunk and stash; this table
assigns its retained implementation families to delivery owners. T-03's six
orphan tests are already validated in #234's main composition, but the later
configured process join must also retain current aggregate tests. No old
checkbox closes T-01/T-02/T-03, F-01, R-02 or final-source joins.

Existing native #241/#242/#244–248 remain separate open contributions with their
recorded limitations and maintainer review boundary. They are not compile
prerequisites of the storage redesign. Do not import native boot scratch
`b75edda7`, silently widen wildcard/native policy, duplicate other owners'
#224/#226/#227, or infer general ABI/cgroup support. N-01 ioctl authority/audit
and N-02 native limits remain concrete open findings in PRD06/08/09/19; required
both-distro libvirt boot/install/PTY/A-B acceptance must establish which native
corrections the shipped product needs. Their open state is not a delivery
disposition of a proven required capability.

## Current review boundary

Both new candidates and all six existing prerequisite compositions pass current
Linux static/build/unit/launch/native/process/setup checks. Complete patches,
findings, exact source/tree identities and commands are in the
[candidate review](prd07-candidate-review.md); the
[TCP draft](prd07-tcp-pr-draft.md) specifies the sole proposed external action.
No merge has occurred and no required capability is declared delivered by an
unpublished candidate. Permission for TCP wave 1 does not authorize later waves,
editing existing heads, comments, native policy or a source-packaging choice.

Active skill: `hdae-development` at `.agents/skills/hdae-development/SKILL.md`.
Reload the full skill, reference, instructions and current queue after context
shift. Exact next action: obtain explicit permission for the reviewed wave-1
TCP push/PR, then perform only that authorized action, inspect its CI/review and
wait for required merges. Independently retain/review storage prerequisite and
failure controls; no next storage PR can be called standalone before its real
base merges. PRD07 remains unchecked throughout.
