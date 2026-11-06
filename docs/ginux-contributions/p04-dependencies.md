# Source-derived dependencies and extraction boundaries — P04

Compared upstream3f2fe7df with integration7cf2638a. P03 accounts for historical
source; this record establishes extraction boundaries. These are capability
requirements, not permission to publish or claims of isolated build success.
Source-level splits below require validation at extraction; a dependency may be
removed only when the resulting source and fixtures actually work without it.

## Corrected dependencies

| Boundary | Actual owner / evidence | Extraction consequence |
| --- | --- | --- |
| G05a shared VFS offset | `VirtualFs::duplicate/read/write`; b500f859 and direct failing probe on current main | Independent current-main fix. No storage/device/process feature required. Keep it separate from G05's configured descriptor machinery. |
| G02 clocks | 9ec95527 changes `balanced_surface/balanced_arguments_allowed` and task fixtures | The core rule can be prepared directly on main with a focused launch fixture. Realtime/monotonic reads execute natively; do not advertise virtual clocks. #179 policy hold remains. |
| G18 namespace capacity before G17's budget reservation | 53a2c9cc reads `DomainConfig::storage_max_objects`; 38fa14fd introduces it and MappedArray/with_namespace | Correct the provisional table's reverse edge. G18 needs growable storage, not G17. Move restore-buffer adaptations into G21 if restore does not exist yet. |
| G17 contains two responsibilities | `LaunchFs::prepare` FD reservation versus `LaunchFs::write` optional positioned offset | Budget reservation follows G18; pwrite itself requires G09's seek/write metadata, not dynamic capacity. Extract pwrite with the relevant I/O owner or a separate focused subpacket; do not make the simple syscall depend on all namespace scaling. |
| G16 traversal | `LaunchFs::resolve/native_path`, `resolve_namespace`, `ImageRoot`, `Rewrite` | Requires owned paths/symlinks/cwd/device precedence. It does not require hardlinks merely because9e2a39e8 preceded it in history; alias interaction still needs F02 validation. |
| G19 export | `LaunchFs::export_snapshot`, `storage_export::Archive`, successful task teardown | Core export/flush needs storage and settled task/thread ownership. It can exist without file mappings or enlarged private descriptor budgets; add mapping consistency tests when G12 is composed. |
| G20 control | `checkpoint_control::{prepare,readiness,poll}`, `launch_tasks::wait_events` | Host-owned inherited Unix stream, closed in the workload; no dependency on G08 guest socketpairs/poll policy. It does require consistent capture and settled processes/threads. |
| G21 restore | `storage_restore::restore` called in launch_main before workload fork | Startup archive import does not require a checkpoint-control transport. It requires compatible archive data and filesystem identity/bounds. Do not wait for G20 merely because the original implementation followed it. |
| G22 archive alias/large bounds | `RestoreEntry`, `LaunchFs::restore_entry/export_snapshot`, Archive::hardlink | Requires hardlink identities, symlinks, dynamic capacity and matching export/import. This is one coupled archive contract; splitting producer and consumer into incompatible standalone proposals is misleading. |

## Packet ownership and joins

| Packet(s) | Defining implementation and included joins | Required review boundary |
| --- | --- | --- |
| G01 | `channel_hooks::bootstrap_poll_allowed`, balanced poll shape, focused launch fixture | Existing #207, held at e7f5f552. Account/review it without replacing its head or opening a duplicate. |
| G03 | VFS file metadata/positioning; LaunchFs assignment validation, controlled tracee I/O, configured dispatch | Extract reusable library operations first where useful; runtime joins must reject host fallthrough and preserve existing read-only launch semantics. |
| G04 | LaunchDevices/DeviceRegistry and configured assignment setup | Device path identity/absence belongs here; retain new resolver interactions in G16. |
| G05 | DescriptorTable/DescriptorCall/ObjectReferences plus LaunchChannels pipe lifetime | Separate public slot flags from shared open-object state. Bring only required backend adapters; current references to network_hooks are G29 joins, not baseline dependencies. |
| G06 | launch_tasks process creation, waits, exec events, cleanup plus existing ProcessTable | Preserve current upstream native supervision and #221 helpers/tests. Review #224/#226 overlap before shipping. Lost-stop and duplicate-death fixes belong with this owner. |
| G07 | per-task DescriptorCall and shared DescriptorGroup ownership | Thread state must not overwrite another task's syscall transaction; no parallel second process table. |
| G08 | DescriptorTable readiness, LaunchChannels peer wakeups/packet pairs | Split readiness from socketpair allocation if independently useful. Guest polling policy is separate from supervisor host poll in G20. |
| G09 | OverlayStorage/MemoryStorage backing, directory API, relative paths and access probes | Preserve upstream #216–#218 semantics. Older missing-directory normalization tests remain relevant; do not transplant the obsolete lexical resolver. |
| G10 | Executable pinning, exec_image::populate, owned exec transaction | Private sealed backing only; no old host staging library/demo. Every fail/commit/death path releases pins. |
| G11 | Script::resolve/restore/private_header/release_held, interpreter lookup | Include argument order, bounded chains, script descriptor retirement and error rollback; preserve source bytes. |
| G12 | MappingCall, supervisor transaction owner and readonly backing references | Kernel maps the requested file offsets; retain old page/offset/stale-ID cases while retiring the obsolete userspace loader ledger. Map must preserve file offsets and close private transaction FDs. |
| G13 | VFS component resolver/owned links, configured symlink syscall joins | VFS resolver API can split from runtime scripts if independently usable; #218 is already upstream and must not be submitted again. |
| G14 | cwd pin in DescriptorGroup, LaunchFs cwd selection and native path rewrite | Fork copies cwd ownership; supported threads share it. Policy group #179 is unresolved. |
| G15 | OverlayFile inode identity and link/unlink/rename pin lifetime | No data copying as a hardlink substitute; alias metadata and quota are shared. |
| G17/G18 | Capacity and private FD reservation as above; tracee public limit capture | Capture operator limit before raising supervisor soft FD limit; never widen tracee authority. Include checked allocation accounting. |
| G19–G22 | Export/archive, control, startup restore in the boundaries above | Format, generation consistency, confinement, bounds and partial failures all need real fixtures. No Recipe state/protocol/policy in Ginux. |
| G23 | Locks/flocks in LaunchFs with process identity supplied by task owner | Lock identity includes hardlink aliases; process/description lock semantics must be distinguished. Real concurrent Cargo/SQLite scenario supplements syscall fixtures. |
| G24 | VFS explicit times and archive mtime fields | Metadata follows inode identity; export and restore must agree. Seconds-only persistence must be stated. |
| G25 | tracee_io bounded vectors, owned file/channel hooks, native mediated chunking | Split owned vectors from scalar native transfer-clamping policy (#178/#163). Do not use the existing local combined test success to choose the upstream clamping contract. |
| G26 | restrict_tracee_resources before exec versus scheduler PID/affinity queries | Bootstrap RLIMIT_AS/CPU subset can be prepared on main independently of configured children; later inheritance checks compose G06/G07. Scheduler query emulation must use registered virtual identity and requires the task owner. Existing metadata-to-enforcement policy change needs review. |
| G27 | IdentityView::uname library encoder versus syscall63 configured hook | Encoder is independent; actual syscall hook needs an existing supervisor entry point, not necessarily every G06 feature. #179 hold still applies. Keep sysinfo/#219 outside this uname proposal. |
| G28 | BoundedTcp/TcpBackend/LinuxTcpBackend and endpoint metadata | Independent library backend, usable through a real caller/test before a standalone proposal; no implicit global network authority. |
| G29 | DomainConfig network policy, LaunchNetwork, descriptor slots/readiness and syscall dispatch | Requires G28 and guest task/FD/readiness owners. Network field additions/fixtures belong with this join, not an unrelated generic backend patch. |
| G30 | Owned metadata validation and chown/lchown/fchown/fchownat claims | Generic uid/gid zero/unchanged handling; keep actual cpio and nofollow/invalid-ID checks. Do not promise broad Linux ownership. |
| G31 | Existing workflow's checkpoint build/run commands | Fold essential coverage into G20/G21; it is not a later substitute for testing those proposals. Preserve every existing required job and unit module. |

## Local preparation strategy

No branches need to be created or switched to produce reviewable source snapshots,
patches, reproduction and draft bodies. Use immutable `/tmp` source exports and a
package manifest with exact base/composition/head-tree/patch hashes. Keep the
existing integration and all dirty/stashed worktrees intact until a reviewed
correction is deliberately applied on its existing branch. Reusing combined
code as donor material is not standalone proof.

Prepare independent candidates against main. Prepare dependent changes against
explicit local prerequisite compositions, fully review/validate them there, and
label standalone delivery as waiting for upstream prerequisites. The manifest
must not pretend a local prerequisite composition is upstream main. F04 presents
both classes and policy holds; publication follows user review and actual merges.

Current plan does not require making31 PRs. Freeze coherent contributions in P05
and revise their boundaries when extraction reveals real coupling. No arbitrary
line-count target, new workflow framework, compatibility adapter, extra kernel
build or upstream-policy file is part of this work.
