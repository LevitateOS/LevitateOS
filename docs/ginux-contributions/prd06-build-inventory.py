#!/usr/bin/env python3
"""Read-only source reconciliation. Does not mutate any Ginux checkout/ref/index."""
from pathlib import Path
import subprocess,json,re,hashlib,collections,difflib
ROOT=Path('/home/vince/Projects/LevitateOS')
REPO=Path('/home/vince/levitate-development/ginux-recipe-integration')
OUT=ROOT/'.artifacts/ginux-contribution-preparation/prd06'
DOC=ROOT/'docs/ginux-contributions'
OUT.mkdir(parents=True,exist_ok=True)
def git(*args,repo=REPO):
 return subprocess.check_output(['git','-C',str(repo),*args],text=True,stderr=subprocess.PIPE)
def rev(x):return git('rev-parse',x).strip()
def sha(x):return hashlib.sha256(x.encode()).hexdigest()
def show(ref,p):return git('show',ref+':'+p)
MAIN=rev('53a54add');CURRENT=rev('5300f5ef');STORAGE=rev('9802ef03');FIX=rev('5449e2a1')
p03=json.loads((DOC/'p03-catalog.json').read_text())
p05=json.loads((DOC/'p05-package-plan.json').read_text())
# These are semantic destinations, not blanket instructions to replay files.
GROUPS={
 'U-BASE':('upstream','current main baseline setup/runtime/native fixtures and restored incoming unit modules; keep current owner, never copy interrupted merge markers','all existing baseline/native tests and main unit cases remain authoritative'),
 'U-CLI':('upstream','current main rejects invalid commands before workload execution; original 4632b5b2 not replayed','main CLI command validation and retained existing launch/native runner checks'),
 'G01':('retained','open #207 at 5ae058f2; configured console/readiness joins','scripts/test-launch.py; tests/fixtures/launch-fixture.c'),
 'G02':('downstream','configured-only clock_hooks.rs at a7ce73cd/5300f5ef; native clock proposal excluded pending PRD08','scripts/test-launch-tasks.py; configured clock/sleep fixtures'),
 'G03':('replaced','PRD03 typed VFS/storage foundation 9802ef03; later capacity/backing joins retained in G09/G18','PRD03 56 VFS tests and complete storage fixture; typed flags/modes/copy-up tests replace old advisory-mode assertions'),
 'G03a':('retained','existing upstream #233 f751cd6f; do not duplicate','scripts/test-launch.py missing-workdir regression'),
 'G03b':('replaced','PRD03 integrated VFS/consumer 9802ef03','PRD03 VFS and launch-storage fixtures; preserve corrected expected errors/modes'),
 'G03c':('replaced','PRD03 configured storage foundation; integration-only later joins remain retained','PRD03 full storage fixture plus later integration tests'),
 'G04':('retained','device_hooks.rs, explicit assigned null device and namespace hiding','scripts/test-launch-devices.py; launch-device-fixture.c'),
 'G05':('retained','descriptors.rs public table and channel_hooks.rs pipe ownership; adapt to PRD03 shared descriptions','scripts/test-launch-fds.py and test-launch-pipes.py'),
 'G05a':('retained','existing upstream #232 5e79a172 shared descriptions; included in PRD03 base','three shared-offset VFS regressions; all alias/slot-reuse/CLOEXEC cases'),
 'G06':('retained','launch_tasks.rs configured task/exec lifecycle, current typed ProcessTable; preserve upstream native owner','test-launch-tasks.py, test-launch-exec.py; current native process suite'),
 'G06a':('retained','existing upstream #234 78a534c1 orphan adoption; independent of storage','process-table orphan units and orphan process fixtures'),
 'G06b':('retained','configured child/wait/exec owner; distinct from native upstream process supervision','task/exec copy faults, lost-stop and adopted-zombie fixtures'),
 'G07':('retained','current per-task DescriptorCall and shared DescriptorGroup, replaces incomplete 5fa497c1 snapshot','test-launch-threads.py; all guarded path/exec/mapping/signal/TID scenarios; PRD09 broader acceptance open'),
 'G08':('retained','descriptor readiness and bounded channel owner; no separate raw public backend ABI','pipe/poll/thread/network fixture joins'),
 'G08a':('retained','DescriptorTable poll readiness, current per-task snapshots','test-launch-poll.py plus concurrent poll snapshots'),
 'G08b':('retained','bounded stream/packet channels, peer wake/retry/cancellation owners','test-launch-pipes.py; packet/private/parked close-reuse regressions'),
 'G09':('retained','generic OverlayStorage/MemoryStorage, grown files and later namespace operations composed onto PRD03','VFS units; test-launch-storage.py and test-launch-directories.py'),
 'G09a':('retained','OverlayStorage/truncate; PRD03 replaces basic positioning but not generic backing; include index hardening','VFS backing/gap/truncate/index tests'),
 'G09b':('retained','storage_memory.rs bounded memfd growth; original 64KiB-inline backend remains valid for small users','growth/bounds/quota and backing failure tests'),
 'G09c':('retained','later directory/rename descriptions; PRD03 replaces basic directory operations, retain richer operations','directory cookie/rename/pinned-object unit and runtime cases'),
 'G09d':('retained','directory/cwd runtime joins, adapt PRD03 owner rather than duplicate resolver','test-launch-directories.py and cwd budget/relative path controls'),
 'G10':('retained','exec_image.rs and owned exec transaction, sealed private memfd and pinned ELF metadata','test-launch-owned-exec.py; rollback/CLOEXEC/interpreter tests'),
 'G11':('retained','exec_script.rs bounded script/interpreter chains and held-pin release','test-launch-scripts.py; vector/cross-page/source-byte/retirement regressions'),
 'G12':('retained','file_mapping.rs readonly transfer and kernel mappings; retain offset/alias/fault/lifetime oracles','mapping/thread/FD fixtures, including historical 3fac8e05 offset assertion (current omission: T-01)'),
 'G13':('retained','owned symlink creation/traversal; keep shared component resolver','owned-link VFS and storage/script fixtures'),
 'G14':('retained','DescriptorGroup cwd lifetime, fork copy and thread sharing; merge with PRD03 cwd owner','directory/cwd/leader-exit/cwd-budget tests'),
 'G15':('retained','shared hardlink inode/pin/rename identity','hardlink unit/directory/archive tests'),
 'G16':('retained','path_hooks.rs stable image/owned namespace selection; adapt PRD03 launch_paths rather than duplicate','missing/file-parent/device-hidden/cross-boundary/path-guard tests'),
 'G17':('retained','separate pwrite G17a and private reserve G17b, with PRD04 inherited-FD correction','positioned/FD-budget/lifetime tests'),
 'G17a':('retained','positioned write in existing file I/O owner, independent of namespace sizing','positioned/shared-offset/append/negative-offset/quota tests'),
 'G17b':('retained','captured public bound + private memfd capacity; PRD04 cleanup corrects inherited occupancy','low-limit/scale/inherited-pressure/lifetime controls; export-close failure F-01 remains open'),
 'G18':('retained','mmap-backed namespace/backing metadata and checked config bounds','namespace scale/capacity/unlinked-reclamation tests; do not restore giant stack arrays'),
 'G19':('retained','storage_export.rs archive writer, pinned parent/inode publication; final publication ordering requires F-01 fix','export/flush/mutation/fault/cancellation cases; no successful-completion claim on failed cleanup'),
 'G20':('retained','checkpoint_control.rs coordinator channel, event waits and quiesced capture','test-launch-checkpoints.py; valid/framing/cancel/task-event tests'),
 'G21':('retained','storage_restore.rs sealed snapshot parsing and startup import','test-launch-restore.py; corruption/replacement/close/error tests'),
 'G22':('retained','coupled archive producer/consumer format, hardlink identity and large checked bounds','archive hardlink/large-file/metadata/reimport tests'),
 'G23':('retained','file_locks.rs virtual lock ownership and descriptor/process lifetime','lock fixture units/runtime partial/split/close/reuse/contention tests'),
 'G24':('retained','inode timestamps and archive mtime; explicit resource policy required','timestamps/NOW/archive round-trip tests'),
 'G25':('retained','bounded vectors/chunked reads/writes and partial-copy accounting','ordinary-vectors/ordinary-large-io/positioned and pipe partial-progress tests'),
 'G26':('retained','resource bootstrap and task-owned AS/affinity guards; not aggregate RSS/CPU accounting','process memory/affinity/high-width tests'),
 'G26a':('retained','tracee child-only resource bootstrap; supervisor capacity remains separate','resource fixture and inherited-limit tests'),
 'G26b':('retained','calling-task affinity/self AS guard under inherited public bounds','task/resource cross-owner and width tests'),
 'G27':('retained','configured platform/filesystem/identity queries under explicit supported surface','image-io/native-query/RPM fixture coverage'),
 'G28':('retained','bounded TCP backend in ginux-devices, independently owned sockets/endpoints','TCP unit loops/metadata/quota/revoke/close/drop/failure tests'),
 'G29':('retained','network_hooks.rs syscall/public-FD/readiness joins and policy','test-launch-network.py; aliases/peer-death/readiness/budget tests'),
 'G30':('retained','virtual uid/gid and mode ownership in VFS/runtime/archive boundary','ownership and target/archive round-trip tests'),
 'G31':('replaced','CI invocations folded into G20/G21 introducing owners; no wiring-only duplicate','checkpoint and restore commands remain once each in current CI'),
 'L-THREAD':('retained','12c176d4/4a8d50f7 per-task/shared-thread implementation and 32-task diagnostic/admission fixes','thread/task/consumer compiler regressions'),
 'L-GUARD':('retained','seize-before-guest and held kernel arguments through path/exec/map/clone/SIGCHLD use','all current shared-path/mapping/exec/TID/SIGCHLD fixtures'),
 'L-UMASK':('retained','436c4ae5 creation mask in process/thread owner; restore shared process semantics after PRD03 fixed umask','creation-mask SQLite/parent-child/thread fixture'),
 'L-INDEX':('retained','b168c2c5 extent overflow and 2914deaf NUL rejection in Index::new','named VFS overflow/NUL regressions and malformed-index/backend-fault receipts'),
 'L-IMAGE':('retained','86e95083 large image reads and virtual queries; 8a7eb37c prevents memory/network misdispatch','image-io fixture and finite 64MiB bound units; shared native-core limit policy remains PRD08'),
 'L-SIGNAL':('retained','d24cd514 empty signal-mask bootstrap; e4a43468 caller-thread SIGPIPE','signal-mask and thread-directed packet/TCP SIGPIPE controls'),
 'L-CONTRACT':('retained','PRD02 configured catalog 2 and central contract/provenance/audit gate; PRD03 storage selects catalog 3','test-launch-contract.py, contract/instance units; versioned old-table/digest preservation'),
 'L-NATIVE':('retained','existing upstream #236/#238 plus native SIGCHLD/value-length fixes; match exact source before any duplicate','native supervision/signal-driven-I/O and overlay-write tests'),
 'L-PRD04':('retained','5449e2a1 FD policy/inherited cleanup/prefix/structured close-error corrections','Daybreak items 1-4 and selected 6 pass; item 5 failed final-export publication F-01'),
 'WIRING':('retained','feature-owned Make/CI/README/config coverage; preserve main checks and unit jobs','all existing CI steps plus introducing feature invocations; no duplicate wiring-only packet'),
 'HISTORY':('downstream','historical plans/evidence/cache cleanup; no downstream policy/plans in Ginux product contributions','source objects and evidence preserved, generated Python bytecode intentionally excluded'),
}
# Explicit net-source ownership. Mixed large files additionally list every symbol.
FILES={
 'gate1-setup-fixture.c':['U-BASE'], 'managed-payload.S':['U-BASE'], 'runtime-fixture.S':['U-BASE'], 'process-exec-target.S':['L-NATIVE'], 'process-review-fixture.c':['L-NATIVE'], 'test-process-native.py':['L-NATIVE'], 'launch-device-fixture.c':['G04'], 'launch-script-fixture.c':['G11'],
 'launch-fd-fixture.c':['G05','G12','G26b','L-PRD04'], 'launch-storage-fixture.c':['G09','G12','G15','G17','G18','G22','G23','G24','G25','G30'], 'launch-thread-fixture.c':['G07','L-GUARD','G12','L-UMASK','L-SIGNAL'],
 'launch_paths.rs':['G16','G03'], 'launch_storage.rs':['G03','G18'], 'process_hooks.rs':['G06','G12','G26'],
 'channel_hooks.rs':['G05','G08','G08b'], 'checkpoint_control.rs':['G20'], 'clock_hooks.rs':['G02'],
 'descriptors.rs':['G05','G08','G12','G26b'], 'device_hooks.rs':['G04'], 'dispatch.rs':['G04','G05','G07','G29','L-NATIVE'],
 'exec_image.rs':['G10'], 'exec_script.rs':['G11'], 'file_locks.rs':['G23'], 'file_mapping.rs':['G12'],
 'fs_hooks.rs':['G03','G09','G10','G11','G12','G14','G15','G16','G17','G18','G19','G22','G23','G24','G25','G30'],
 'launch.rs':['G03a','G26a','L-CONTRACT','L-SIGNAL','L-GUARD'], 'launch_tasks.rs':['G06','G07','G08','L-THREAD','L-GUARD','L-UMASK','G19','G20'],
 'network_hooks.rs':['G29'], 'path_hooks.rs':['G16','L-GUARD'], 'storage_export.rs':['G19','G22','G24'],
 'storage_memory.rs':['G09b','G17b','G18'], 'storage_restore.rs':['G21','G22','G24'], 'supervisor.rs':['L-NATIVE'], 'tracee_io.rs':['G03','G25','G26a'],
}
UNRECOGNIZED_PATHS=set()
def owners(path,context=''):
 name=Path(path).name
 if name in FILES:return FILES[name]
 if path=='crates/ginux-vfs/src/lib.rs':return ['G03','G05a','G09','G10','G11','G12','G13','G14','G15','G18','G24','G30','L-INDEX']
 if path=='crates/ginux-core/src/lib.rs':return ['L-NATIVE','L-IMAGE']
 if path=='crates/ginux-runtime/src/lib.rs':return ['U-BASE']
 if path=='crates/ginux-process/src/lib.rs':return ['G06a']
 if path=='crates/ginux-domain/src/lib.rs':return ['L-CONTRACT','G18','G26','G29']
 if path=='crates/ginux-instance/src/lib.rs':return ['G18','G26','G29','L-CONTRACT']
 if path.startswith('crates/ginux-contract/'):return ['L-CONTRACT']
 if path.startswith('crates/ginux-devices/'):return ['G28']
 if path.startswith('crates/ginux-kernel-views/'):return ['G27']
 if name=='main.rs' and path.startswith('crates/ginux-init/'):return ['WIRING','G26b','G27']
 for stem,group in [('checkpoints','G20'),('restore','G21'),('contract','L-CONTRACT'),('devices','G04'),('directory','G09d'),('directories','G09d'),('owned-exec','G10'),('interpreter','G11'),('scripts','G11'),('exec','G06'),('fds','G05'),('fd-','G05'),('pipe','G05'),('poll','G08'),('thread','G07'),('task','G06'),('image-io','L-IMAGE'),('network','G29'),('storage','G09')]:
  if ('launch-'+stem) in name:return [group]
 if name in ('launch-fixture.c','test-launch.py'):return ['G01','G03a','L-CONTRACT','L-SIGNAL']
 if name in ('native-fixture.c','test-native-supervision.py'):return ['L-NATIVE']
 if path.startswith(('hdae/','.hdae/','.agents/','.claude/')):return ['HISTORY']
 if name in ('Makefile','README.md','.gitignore','check.yml') or path.startswith(('config/','docs/')):return ['WIRING']
 if '__pycache__/' not in path:UNRECOGNIZED_PATHS.add(path)
 return ['HISTORY']
# Refine symbols inside the mixed runtime/VFS files using the actual operation.
def symbol_owners(path,name):
 n=name.lower()
 if path=='crates/ginux-init/src/main.rs':
  if n.startswith('instance_') and any(x in n for x in ['validate','inspect','explain','diff']):return ['L-CONTRACT']
  if n=='rust_main':return ['WIRING','G19','G20','G21']
  if n=='vfs_main' or n in ('native_main','native_tracee','supervise_native'):return ['L-NATIVE','L-CONTRACT']
  return ['U-BASE']
 if path=='crates/ginux-init/src/supervisor.rs' and n in ('write_tracee_bytes_strict','process_vm_write_bytes'):return ['L-GUARD']
 if 'fs_hooks' in path or 'ginux-vfs/' in path:
  for words,group in [('executable|pin_object|interpreter|is_script','G10'),('creation_mask','L-UMASK'),('cwd|chdir|getcwd','G14'),('hard_link|hardlink|link_count','G15'),('rename','G09c'),('symlink|read_link','G13'),('export|snapshot|archive','G19'),('restore','G21'),('file_lock|flock|locks','G23'),('timestamp|set_times|fset_times|set_object_times','G24'),('mapping|reopen_readonly','G12'),('vector|large_io','G25'),('namespace_capacity|supplied_namespace|with_namespace|capacity','G18'),('nul|extent_overflow','L-INDEX'),('duplicate|shared_offset|shared_alias|seek_shares','G05a')]:
   if re.search(words,n):return [group]
  return ['G09']
 if 'launch_tasks' in path:
  if re.search('argument_guard|path_call',n):return ['L-GUARD']
  if re.search('creation|birth',n):return ['L-THREAD','L-GUARD']
  if re.search('exec|image',n):return ['G06','G10','G11','L-GUARD']
  if re.search('teardown|fail|supervise',n):return ['G06','G19','G20']
  return ['G07','G06']
 return owners(path)
CACHE={}
def paths(ref):
 if ref not in CACHE:CACHE[ref]=git('ls-tree','-r','--name-only',ref).splitlines()
 return CACHE[ref]
RECORDS=[];SYMBOLS=[];PATCHES=[]
def inventory(label,base,head,forced=None,disposition=None):
 patch=git('diff','--no-ext-diff','--unified=0',base,head,'--')
 (OUT/(label+'.patch')).write_text(patch)
 path='';index=0;symbolcache={}
 for block in re.split(r'(?=^diff --git )',patch,flags=re.M):
  if not block.startswith('diff --git'):continue
  m=re.search(r'^\+\+\+ b/(.*)$',block,re.M) or re.search(r'^--- a/(.*)$',block,re.M)
  if not m:continue
  path=m.group(1)
  try:s=show(head,path)
  except subprocess.CalledProcessError:s=''
  funcs=[(match.start(),match.group(1),s[:match.start()].count('\n')+1) for match in re.finditer(r'^\s*(?:pub(?:\([^\n]*?\))?\s+)?(?:(?:async|unsafe|const|extern\s+"[^"]+")\s+)*fn\s+(\w+)',s,re.M)]
  for i,(pos,name,line) in enumerate(funcs):
   endline=(funcs[i+1][2]-1 if i+1<len(funcs) else len(s.splitlines()))
   symbolcache[(name,line)]={'name':name,'start':line,'end':endline,'owners':forced or symbol_owners(path,name)}
  for hunk in re.split(r'(?=^@@ )',block,flags=re.M)[1:]:
   header=hunk.splitlines()[0];ranges=re.match(r'@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@',header)
   assert ranges,header
   start=int(ranges[3]);count=int(ranges[4] or 1);end=start+max(count-1,0)
   syms=[d for d in symbolcache.values() if d['start']<=end and d['end']>=start]
   context=re.search(r'\bfn\s+(\w+)',header)
   groups=forced or (symbol_owners(path,context[1]) if context else sorted(set(g for d in syms for g in d['owners']))) or owners(path)
   state=disposition or ('downstream' if groups==['G02'] else ('replaced' if label=='original-donor' else 'retained'))
   if label=='storage-foundation':state='retained'
   index+=1
   RECORDS.append({'id':label+':'+str(index),'path':path,'range':header,'sha256':sha(hunk),'disposition':state,'owners':groups,'symbols':[d['name']+'@'+str(d['start']) for d in syms],'base':base,'source':head})
  if label in ('current-integration','prd04-corrections','storage-foundation'):
   for d in symbolcache.values():SYMBOLS.append(dict(d,source=head,path=path,label=label))
  symbolcache={}
 PATCHES.append({'label':label,'base':base,'head':head,'sha256':sha(patch),'hunks':index})
 return index
inventory('current-integration',MAIN,CURRENT)
inventory('prd04-corrections',CURRENT,FIX,forced=['L-PRD04'])
inventory('storage-foundation',rev('f1c3d3a7'),STORAGE)
inventory('storage-prerequisites',MAIN,rev('f1c3d3a7'),forced=['G05a','G03a','L-NATIVE','L-CONTRACT'])
inventory('original-donor',rev(p03['base']),rev(p03['integration']))
# Each historical donor/prepared commit is represented by every exact diff hunk.
COMMITS=[]
for kind,rows in [('original',p03['commits']),('prototype',p03['other_worktree_commits'])]:
 for c in rows:
  h=rev(c['commit']);parent=rev(h+'^');group=c.get('packet','HISTORY')
  gs=re.findall(r'G\d\d[a-z]?',group) or ['U-CLI']
  if kind=='original' and 'already upstream' in group:state='upstream'
  elif kind=='prototype':state='replaced'
  else:state='downstream' if gs==['G02'] else ('replaced' if gs==['G03'] else 'retained')
  if kind=='prototype' and c['disposition'] in ('existing-poll','partly equivalent; strengthen assertion'):state='retained'
  inventory(kind+'-'+h[:8],parent,h,forced=gs,disposition=state)
  COMMITS.append({'source':h,'subject':c['subject'],'kind':kind,'disposition':state,'owners':gs,'prior_rationale':c.get('semantic_disposition'),'destination':[GROUPS[g][1] for g in gs if g in GROUPS]})

CURRENT_OWNERS = {
 '12c176d4':['L-THREAD','G08'], '4a8d50f7':['L-THREAD','G07'], '8981bf77':['L-CONTRACT'],
 '12c4ea80':['L-THREAD'], '062fb5ef':['L-THREAD'], 'e9a2f8a0':['G07'], '6a19a70a':['G08b'],
 '436c4ae5':['L-UMASK'], 'c7b4b73a':['G11','G16'], 'b4c69551':['G12'], '6d290d48':['G14','G27'],
 'f9a584e9':['L-GUARD'], 'e0519b48':['L-GUARD','G16'], '1c2b1333':['L-GUARD','G10','G11'],
 'ad364bd7':['L-GUARD','G12'], 'e354efd5':['G19'], 'cde7dc22':['L-GUARD'], '0d23d34d':['L-GUARD'],
 'a0419e24':['L-GUARD','G07'], '719aac91':['L-NATIVE'], '432ff7be':['L-NATIVE'], 'c150b802':['L-NATIVE'],
 '1104d5ea':['L-NATIVE'], '50204a29':['L-CONTRACT'], '158e11a6':['G05','L-NATIVE'], 'b168c2c5':['L-INDEX'],
 'f4b2e240':['L-GUARD'], '86e95083':['L-IMAGE','G27'], '8a7eb37c':['L-IMAGE','G29'],
 'dbb356f9':['HISTORY'], '2914deaf':['L-INDEX'], 'd24cd514':['L-SIGNAL'], '76b086e1':['G21'],
 '881d46fa':['G21'], 'ccba6747':['G19'], '3dae449a':['G19'], '020f969b':['G29'],
 'e4a43468':['L-SIGNAL','G08b','G29'], 'a7ce73cd':['G02'], '5300f5ef':['L-CONTRACT'],
}
for key in ['167f57b5','c3a3f7b5','4075d9d5','fdeae41b','836061e1','20b246ce']:CURRENT_OWNERS[key]=['G12','L-GUARD']
for key in ['8601b606','6cc21e69','b91b69c1']:CURRENT_OWNERS[key]=['G10','G11','L-GUARD']
for key in ['5dc0fd8c','75562753','ebcdaee0']:CURRENT_OWNERS[key]=['G07','L-GUARD']
for key in ['75d5dcc3','e91bba31','eb4a342e','5449e2a1']:CURRENT_OWNERS[key]=['L-PRD04']

for line in git('log','--reverse','--no-merges','--format=%H %s',MAIN+'..'+FIX).splitlines():
 h,subject=line.split(' ',1)
 changed=git('diff-tree','--no-commit-id','--name-only','-r',h).splitlines()
 gs=sorted(set(g for p in changed for g in owners(p)))
 if h in [rev(x) for x in ['75d5dcc3','e91bba31','eb4a342e','5449e2a1']]:gs=['L-PRD04']
 if h[:8] in CURRENT_OWNERS:gs=CURRENT_OWNERS[h[:8]]
 COMMITS.append({'source':h,'subject':subject,'kind':'current','disposition':'downstream' if gs in (['G02'],['HISTORY']) else 'retained','owners':gs,'files':changed,'destination':[GROUPS[g][1] for g in gs]})
# Prepared packet heads retain their specific reviewed source; not full current-main delivery.
PACKETS=[]
for packet in p05['packets']:
 key=packet['packet'];h=packet.get('head');base=packet.get('base')
 row={'packet':key,'purpose':packet.get('purpose'),'disposition':GROUPS.get(key,('retained',))[0],'destination':GROUPS.get(key,('','See introducing feature owner'))[1],'historical_status':packet.get('status'),'head':h,'base':base,'source_commits':packet.get('source_commits',[]),'review':packet.get('evidence')}
 if h and base:
  h=rev(h);base=rev(base);row.update(head=h,base=base)
  inventory('prepared-'+key,base,h,forced=[key],disposition=row['disposition'])
 PACKETS.append(row)
# Current branches and registered worktrees may contain useful work beyond old P03.
BRANCHES=[]
for line in git('for-each-ref','--format=%(refname) %(objectname)','refs/heads','refs/remotes/origin').splitlines():
    name,h=line.split();BRANCHES.append({'ref':name,'source':h})
worktrees=[]
for block in git('worktree','list','--porcelain').strip().split('\n\n'):
    row={line.split(' ',1)[0]:line.split(' ',1)[1] for line in block.splitlines() if ' ' in line}
    if 'HEAD' in row:worktrees.append(row)
known=[MAIN,CURRENT,FIX,STORAGE]+[c['source'] for c in COMMITS]+[p['head'] for p in PACKETS if p.get('head')]
extra=git('rev-list','--reverse','--no-merges',*[b['source'] for b in BRANCHES],*[w['HEAD'] for w in worktrees],'--not',*known).splitlines()
GROUPS['N-BOOT']=('retained','separate native/systemd fixes on registered fix/native-* branches; scratch AcornOS policy is downstream-only until reviewed/versioned','native fixture waitid/TCGETS2/cgroup/PIDFD controls; full boot claim remains PRD19')
for h in extra:
    subject=git('show','-s','--format=%s',h).strip()
    if 'scratch:' in subject:gs=['N-BOOT'];state='downstream'
    elif h==rev('5e79a172') or 'share offsets' in subject:gs=['G05a'];state='retained'
    elif h==rev('78a534c1'):gs=['G06a'];state='retained'
    elif 'use mkdirat' in subject:gs=['G03a'];state='retained'
    elif any(w in subject for w in ['cgroup','waitid','PIDFD_GET_INFO','TCGETS2']):gs=['N-BOOT'];state='retained'
    elif 'SIGCHLD' in subject or 'host directories' in subject or 'signal-driven' in subject or 'overlay write' in subject:gs=['L-NATIVE'];state='retained'
    elif 'contract' in subject or 'audit mode' in subject:gs=['L-CONTRACT'];state='replaced'
    elif 'Integrate configured-launch' in subject:gs=sorted(set(g for f in git('diff-tree','--no-commit-id','--name-only','-r',h).splitlines() for g in owners(f)));state='replaced'
    else:gs=['G03'];state='replaced'
    inventory('extra-'+h[:8],rev(h+'^'),h,forced=gs,disposition=state)
    COMMITS.append({'source':h,'subject':subject,'kind':'additional-ref','disposition':state,'owners':gs,'destination':[GROUPS[g][1] for g in gs]})
if rev('5fa497c1') not in {c['source'] for c in COMMITS}:
 h=rev('5fa497c1');inventory('reviewed-thread-snapshot',rev(h+'^'),h,forced=['G07'],disposition='replaced')
 COMMITS.append({'source':h,'subject':git('show','-s','--format=%s',h).strip(),'kind':'review-only','disposition':'replaced','owners':['G07'],'destination':[GROUPS['G07'][1]]})
# Cover every remaining non-main ancestor of registered refs, even when a net patch already covers it.
represented={c['source'] for c in COMMITS}|{p['head'] for p in PACKETS if p.get('head')}|{p['head'] for p in PATCHES}
all_local=git('rev-list','--reverse','--no-merges',*[b['source'] for b in BRANCHES],*[w['HEAD'] for w in worktrees],'--not',MAIN).splitlines()
GROUPS['P-STORAGE']=('retained','exact PRD03 9802ef03 foundation and its pinned f1c3d3a7 prerequisite composition','PRD03 138-unit, 40-launch and complete local CI receipts; not Recipe integration')
for h in all_local:
 if h in represented:continue
 subject=git('show','-s','--format=%s',h).strip()
 if 'share offsets' in subject:gs=['G05a']
 elif 'mkdirat' in subject:gs=['G03a']
 elif 'signal-driven' in subject or 'host directories' in subject:gs=['L-NATIVE']
 elif 'contract' in subject or 'audit mode' in subject:gs=['L-CONTRACT']
 else:gs=['P-STORAGE']
 inventory('ancestor-'+h[:8],rev(h+'^'),h,forced=gs,disposition='retained')
 COMMITS.append({'source':h,'subject':subject,'kind':'covered-ancestor','disposition':'retained','owners':gs,'destination':[GROUPS[g][1] for g in gs]})
assert set(all_local)<={c['source'] for c in COMMITS}|{p['head'] for p in PACKETS if p.get('head')}|{p['head'] for p in PATCHES}
# Stored patch exports are independent retained inputs, not assumed identical to a ref.
PATCH_EXPORTS=[]
for path in sorted((DOC/'patches').glob('*.patch')):
    key=path.stem;data=path.read_text();chunks=re.split(r'(?=^diff --git )',data,flags=re.M)
    count=0
    for block in chunks:
        m=re.search(r'^\+\+\+ b/(.*)$',block,re.M) or re.search(r'^--- a/(.*)$',block,re.M)
        if not m:continue
        for hunk in re.split(r'(?=^@@ )',block,flags=re.M)[1:]:
            count+=1;RECORDS.append({'id':'patch-'+key+':'+str(count),'path':m[1],'range':hunk.splitlines()[0],'sha256':sha(hunk),'disposition':GROUPS[key][0],'owners':[key],'source':str(path.relative_to(ROOT)),'symbols':[]})
    PATCH_EXPORTS.append({'path':str(path.relative_to(ROOT)),'sha256':sha(data),'hunks':count,'owners':[key],'disposition':GROUPS[key][0]})
# Preserved uncommitted snapshots: every file and its content hash, without changing it.
PRESERVED=[]
for meta in sorted((ROOT/'.artifacts/development-migration/2026-11-06/recovery/worktrees').glob('*/snapshot.json')):
    data=json.loads(meta.read_text());label=meta.parent.name
    for entry in data['entries']:
        path=entry['path'];file=meta.parent/'files'/path
        if not file.is_file():continue
        content=file.read_text(errors='replace')
        if label.startswith('3-'):state='upstream';groups=['G03']
        elif label.startswith('2-'):state='downstream';groups=['G05','G08']
        elif label.startswith(('8-','9-')):state='replaced';groups=['G12','G26']
        else:state='upstream';groups=['U-BASE']
        if label.startswith(('10-','11-')) and '<<<<<<<' in content:state='downstream'
        assert hashlib.sha256(file.read_bytes()).hexdigest()==entry['sha256'],str(file)
        original=meta.parent/'files'/path
        oldhead=data['head']
        try:baseline=show(oldhead,path)
        except subprocess.CalledProcessError:baseline=''
        raw=''.join(difflib.unified_diff(baseline.splitlines(True),content.splitlines(True),fromfile='a/'+path,tofile='b/'+path,n=0))
        for ordinal,hunk in enumerate(re.split(r'(?=^@@ )',raw,flags=re.M)[1:],1):
            RECORDS.append({'id':'snapshot-'+label+':'+path+':'+str(ordinal),'path':path,'range':hunk.splitlines()[0],'sha256':sha(hunk),'disposition':state,'owners':groups,'source':str(original.relative_to(ROOT)),'base':oldhead,'symbols':[]})
        current=ROOT/'.artifacts/development-migration/2026-11-06/worktrees'/label/path
        assert current.is_file() and hashlib.sha256(current.read_bytes()).hexdigest()==entry['sha256'],str(current)
        PRESERVED.append({'snapshot':str(meta.relative_to(ROOT)),'original':data['original_path'],'path':path,'sha256':hashlib.sha256(file.read_bytes()).hexdigest(),'disposition':state,'owners':groups,'unmerged':bool(data.get('unmerged_index_entries')),'destination':[GROUPS[g][1] for g in groups]})
# Both old interpreter stashes are intentionally retained, not applied.
STASHES=[]
for ref in ['2b887da6','a94d56fb']:
    h=rev(ref);patch=git('diff',h+'^',h);(OUT/('stash-'+ref+'.patch')).write_text(patch)
    STASHES.append({'source':h,'sha256':sha(patch),'disposition':'replaced','owners':['G11'],'destination':'exec_script::Script::resolve and held pin release; preserve both originals'})
# Rust unit cases, by test function with all source versions/locations. Main tests must survive.
ref_labels={MAIN:['upstream'],CURRENT:['current'],FIX:['prd04'],STORAGE:['storage']}
for p in PACKETS:
 if p.get('head'):ref_labels.setdefault(p['head'],[]).append(p['packet'])
for b in BRANCHES:ref_labels.setdefault(b['source'],[]).append('local-ref')
for w in worktrees:ref_labels.setdefault(w['HEAD'],[]).append('registered-worktree')
# Every commit contributes changed test files, including cases removed before a later head.
changed_test_paths={}
for c in COMMITS:
 ref=c['source']
 if ref not in ref_labels:
  ref_labels[ref]=['historical-commit'];changed_test_paths[ref]=set(git('diff-tree','--no-commit-id','--name-only','-r',ref).splitlines())
for ref in [rev('5fa497c1'),rev('7cf2638a'),rev('ebd68025'),rev('022294a9'),rev('3fac8e05'),rev('c4a0820d'),rev('f8ce5274'),rev('2eac10bf')]:ref_labels.setdefault(ref,[]).append('historical')
TESTS={};CFILES=[]
for ref,labels in ref_labels.items():
 for path in paths(ref):
  if ref in changed_test_paths and path not in changed_test_paths[ref]:continue
  if not(path.endswith('.rs') or path.startswith('tests/fixtures/')):continue
  source=show(ref,path)
  if path.endswith('.rs'):
   matches=list(re.finditer(r'#\[test\]\s*fn\s+(\w+)',source))
   for i,m in enumerate(matches):
    key=path+'::'+m[1];rec=TESTS.setdefault(key,{'path':path,'test':m[1],'owners':symbol_owners(path,m[1]),'versions':[]})
    indent=source[source.rfind('\n',0,m.start())+1:m.start()]
    close=re.search(r'^'+re.escape(indent)+r'}',source[m.end():],re.M)
    assert close,(ref,path,m[1])
    body=source[m.start():m.end()+close.end()]
    rec['versions'].append({'source':ref,'labels':labels,'line':source[:m.start()].count('\n')+1,'sha256':sha(body)})
  elif path.endswith(('.c','.S')):
   key=(path,sha(source))
   old=next((x for x in CFILES if (x['path'],x['sha256'])==key),None)
   if old:old['sources'].append({'source':ref,'labels':labels})
   else:CFILES.append({'path':path,'sha256':sha(source),'sources':[{'source':ref,'labels':labels}],'owners':owners(path),'checks':len(re.findall(r'\bCHECK\s*\(',source)),'functions':re.findall(r'^(?:static\s+)?(?:long|int|void)\s+(\w+)\s*\(',source,re.M),'disposition':'retained','destination':'current feature fixture or source-retained oracle with named owner; changed ABI expectations use PRD03/PRD04 oracles'})
for f in CFILES:
 current=any(set(v['labels'])&{'upstream','current','storage','prd04'} for v in f['sources'])
 f['disposition']='retained' if current else 'replaced'
 f['destination']=('Preserve exact current fixture functions and assertions' if current else 'Preserve this exact source/hash as a regression input for '+','.join(f['owners'])+'; translate old task/channel/mapping APIs and PRD03 mode/error expectations before combining; no automatic coverage equivalence')
for t in TESTS.values():
 labels=set(l for v in t['versions'] for l in v['labels'])
 t['disposition']='upstream' if 'upstream' in labels else ('retained' if labels&{'current','prd04','storage'} else 'replaced')
 t['destination']=('Preserve current-main case; incoming tests are authoritative' if t['disposition']=='upstream' else ('Preserve exact source case in current integration/storage' if t['disposition']=='retained' else 'Retain historical oracle with '+','.join(t['owners'])+'; translate obsolete APIs/expected modes, do not discard assertion intent'))
TEST_REPLACEMENTS={
 'thread_reservations_cancel_only_in_the_running_parent_group':['crates/ginux-process/src/lib.rs::thread_reservation_rollback_retains_process_identity'],
 'owned_storage_syscalls_follow_their_semantic_rules':['crates/ginux-contract/src/lib.rs::storage_catalog_preserves_older_authority','crates/ginux-contract/src/lib.rs::syscall_semantic_operations_are_catalogued'],
 'mkdir_creates_directories_that_cannot_be_opened_or_unlinked':['crates/ginux-vfs/src/lib.rs::mkdir_creates_openable_directories_but_unlink_requires_rmdir'],
 'mkdir_keeps_the_sticky_bit_and_drops_setuid':['crates/ginux-vfs/src/lib.rs::mkdir_keeps_all_supported_mode_bits'],
 'files_without_write_bits_refuse_writable_opens':['crates/ginux-vfs/src/lib.rs::existing_overlay_modes_constrain_new_opens'],
 'identity_and_namespaces_are_domain_owned_and_consistent':['crates/ginux-kernel-views/src/lib.rs::identity_and_namespaces_are_instance_owned_and_consistent'],
 'overlays_are_isolated_between_domain_instances':['crates/ginux-vfs/src/lib.rs::overlays_are_isolated_between_instances'],
 'duplicated_descriptors_share_offsets_without_aliasing_reused_slots':['crates/ginux-vfs/src/lib.rs::duplicated_offset_survives_close_slot_reuse_and_cloexec','crates/ginux-vfs/src/lib.rs::duplicated_reads_share_offset_while_separate_opens_do_not'],
 'supervisor_reservation_wait_peek_and_orphan_adoption':['crates/ginux-process/src/lib.rs::fork_reservation_cancellation_preserves_published_state','crates/ginux-process/src/lib.rs::wait_inspection_preserves_typed_outcomes_until_consumed','crates/ginux-process/src/lib.rs::exited_and_signaled_parents_reparent_children'],
 'threads_share_child_waits_and_reap_without_consuming_processes':['crates/ginux-process/src/lib.rs::thread_reservation_rollback_retains_process_identity','crates/ginux-process/src/lib.rs::orphan_adoption_waits_for_the_last_group_member'],
 'signaled_child_peek_preserves_status_and_adopts_orphans':['crates/ginux-process/src/lib.rs::wait_inspection_preserves_typed_outcomes_until_consumed','crates/ginux-process/src/lib.rs::exited_and_signaled_parents_reparent_children'],
}
for key,t in TESTS.items():
 if t['disposition']!='replaced':continue
 if t['path']=='crates/ginux-domain/src/lib.rs':
  replacement='crates/ginux-instance/src/lib.rs::'+t['test']
  if replacement in TESTS:
   t['owners']=['L-CONTRACT'];t['replacement_tests']=[replacement];t['destination']='upstream instance terminology/schema owner; same named assertion retained after the crate rename';continue
 if t['test'] in TEST_REPLACEMENTS:
  t['replacement_tests']=TEST_REPLACEMENTS[t['test']]
  assert all(name in TESTS for name in t['replacement_tests'])
  t['destination']='replaced by concrete named current tests';continue
 if t['test']=='chunked_copy_up_preserves_all_bytes_and_reclaims_partial_failure':
  t['disposition']='retained';t['destination']='exact test retained in tracked patches/G09a.patch; port its >4096-byte middle-copy failure oracle with G09a/G09b in PRD05/PRD07 (T-02), not yet in current integration';continue
 if t['path']=='crates/ginux-process/src/lib.rs' and any(v['source']==rev('78a534c1') for v in t['versions']):
  t['disposition']='retained';t['owners']=['G06a'];t['destination']='Exact six-case orphan suite retained in pending upstream #234 at 78a534c1; include on integration, not present at 5300f5ef';continue
 t['destination']='REQUIRES_EXPLICIT_TEST_MAPPING'
assert not [t for t in TESTS.values() if t['destination']=='REQUIRES_EXPLICIT_TEST_MAPPING']
for h in RECORDS:
 h['disposition']={g:('replaced' if h['id'].startswith('original-donor:') else (GROUPS[g][0] if h['id'].startswith('current-integration:') else h['disposition'])) for g in h['owners']}
main_changed=[]
for t in TESTS.values():
 old=next((v for v in t['versions'] if 'upstream' in v['labels']),None)
 new=next((v for v in t['versions'] if 'current' in v['labels']),None)
 if old and new and old['sha256']!=new['sha256']:
  main_changed.append(t['test']);t['current_assertion_disposition']='Shared duplicate offset now consumes o through old and ld through alias, then EOF; old-byte/unlink/recreate/CLOEXEC assertions preserved, changed to the #232 description contract'
assert main_changed==['unlink_retains_open_bytes_and_recreated_names_are_independent'],main_changed
main_tests={k for k,t in TESTS.items() if any('upstream' in v['labels'] for v in t['versions'])}
local_tests={k for k,t in TESTS.items() if any('current' in v['labels'] for v in t['versions'])}
assert not main_tests-local_tests,sorted(main_tests-local_tests)
UNKNOWN=sorted(set(g for r in RECORDS for g in r['owners'])-set(GROUPS))
assert not UNKNOWN,UNKNOWN
# Preserve current upstream native copyout implementation and runtime fault/fallback cases.
native_paths=['scripts/test-process-native.py','tests/fixtures/process-review-fixture.c']
assert not git('diff','--name-only',MAIN,CURRENT,'--',*native_paths).strip()
native_functions=['supervise_process_native','wake_waiters_for_child','read_tracee_pipefds','store_wait_results','store_tracee_status','write_wait_fault','store_tracee_tid']
def top_function(source,name):
 match=re.search(r'^fn '+name+r'\b',source,re.M);assert match,name
 next_fn=re.search(r'^fn ',source[match.end():],re.M)
 return source[match.start():match.end()+next_fn.start() if next_fn else len(source)]
for name in native_functions:assert top_function(show(MAIN,'crates/ginux-init/src/main.rs'),name)==top_function(show(CURRENT,'crates/ginux-init/src/main.rs'),name),name
result={'sources':{'upstream':MAIN,'integration':CURRENT,'prd04':FIX,'storage':STORAGE},'upstream_refresh':'upstream.json in evidence directory','groups':{k:{'disposition':v[0],'destination':v[1],'tests':v[2]} for k,v in GROUPS.items()},'commits':COMMITS,'prepared_packets':PACKETS,'diffs':PATCHES,'hunks':RECORDS,'symbols':SYMBOLS,'rust_tests':list(TESTS.values()),'fixture_versions':CFILES,'preserved_work':PRESERVED,'stashes':STASHES,'refs':BRANCHES,'registered_worktrees':worktrees,'patch_exports':PATCH_EXPORTS,'coverage':{'hunks':len(RECORDS),'commits':len(COMMITS),'prepared_packets':len(PACKETS),'symbols':len(SYMBOLS),'rust_test_names':len(TESTS),'main_tests':len(main_tests),'current_tests':len(local_tests),'main_tests_missing_from_integration':sorted(main_tests-local_tests),'main_tests_changed_body':main_changed,'fixture_versions':len(CFILES),'stored_patch_exports':len(PATCH_EXPORTS),'additional_ref_commits':len(extra),'preserved_snapshot_files':len(PRESERVED),'stashes':len(STASHES),'unknown_owners':UNKNOWN,'unrecognized_paths':sorted(UNRECOGNIZED_PATHS)}}
# Refresh duplicate-diff evidence for every inventoried commit.
def patch_ids(args):
 data=subprocess.check_output(['git','-C',str(REPO),*args])
 value=subprocess.check_output(['git','patch-id','--stable'],input=data).decode()
 return {line.split()[1]:line.split()[0] for line in value.splitlines()}
up_ids=patch_ids(['log','--no-merges','-p',MAIN])
local_ids=patch_ids(['show','--format=medium',*[c['source'] for c in COMMITS]])
reverse={v:k for k,v in up_ids.items()}
equivalent={k:reverse[v] for k,v in local_ids.items() if v in reverse}
duplicate={'method':'git patch-id --stable on complete commit patches; equal patch-id is a duplicate diff, not runtime acceptance','upstream_main':MAIN,'upstream_commit_patch_ids':up_ids,'local_commit_patch_ids':local_ids,'matches_on_main':equivalent}
(OUT/'patch-equivalence.json').write_text(json.dumps(duplicate,indent=2)+'\n')
result['upstream_metadata']=json.loads((OUT/'upstream.json').read_text())
result['exact_diff_equivalents_on_main']=equivalent
assert not UNRECOGNIZED_PATHS,sorted(UNRECOGNIZED_PATHS)
result['upstream_native_preservation']={'identical_paths':native_paths,'identical_main_functions':native_functions,'historical_net':'original-donor net diff is replaced by current 5300f5ef composition; never replay its negative incoming native copyout/test hunks. Individual retained feature commits and current net diff are separately indexed.'}
result['consumer_sources']={'recipe':git('rev-parse','HEAD',repo=ROOT/'tools/recipe').strip(),'builder':git('rev-parse','HEAD',repo=ROOT/'distro/builder').strip()}
assert result['consumer_sources']=={'recipe':'b41443880fe585a27bc799d80382c9ff5b07047b','builder':'0044634d616d7888b93d048b4bd39cb0d2db0241'}
result['schema_notes']={'hunks':'Each hunk disposition is a map from semantic owner to retained/upstream/replaced/downstream; mixed hunks must be split by these owners, never blindly replayed.','tests':'A source-retained historical oracle is not a current runtime pass; replacement_tests names the actual replacement cases.','scope':'Every non-main non-merge ancestor of registered heads/refs is represented as a commit, packet or exact net diff. Counts intentionally overlap.','open_work':'prd06-reconciliation.md#concrete-open-work-no-acceptance-hidden-by-old-boxes'}
(OUT/'inventory-draft.json').write_text(json.dumps(result,indent=2)+'\n')
# Keep each record on one line in the committed review ledger.
lines=['{']
for key,value in result.items():
 if isinstance(value,list):
  rendered='[\n'+',\n'.join('    '+json.dumps(row,separators=(',',':'),ensure_ascii=False) for row in value)+'\n  ]' if value else '[]'
 else:rendered=json.dumps(value,separators=(',',':'),ensure_ascii=False)
 lines.append('  '+json.dumps(key)+': '+rendered+',')
lines[-1]=lines[-1][:-1];lines.append('}')
(DOC/'prd06-source-inventory.json').write_text('\n'.join(lines)+'\n')
print(json.dumps(result['coverage'],indent=2))
print('current hunks',next(p['hunks'] for p in PATCHES if p['label']=='current-integration'))
print('historical-only tests',sum(t['disposition']=='replaced' for t in TESTS.values()))
