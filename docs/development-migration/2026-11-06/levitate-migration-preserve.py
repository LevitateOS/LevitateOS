from pathlib import Path
import subprocess,json,os,shutil,tarfile,hashlib,re,tempfile
root=Path('/home/vince/Projects/LevitateOS');out=root/'docs/development-migration/2026-11-06';assert not out.exists();out.mkdir(parents=True);tmp=Path('/tmp/levitate-migration-recovery');assert not tmp.exists();tmp.mkdir();inv=json.loads(Path('/tmp/levitate-migration-inventory.json').read_text());retired=json.loads(Path('/tmp/levitate-migration-retired.json').read_text());manifest={'date':'2026-11-06','purpose':'source and paused development recovery on another computer; no PR publication','repositories':[],'worktree_snapshots':[],'retired_repositories':[],'evidence_files':[],'excluded':'Credentials/private keys, compiled binaries, build/package/image caches and regenerable large build outputs are not Git migration inputs.'}
def run(cmd,**kw):return subprocess.check_output(cmd,**kw)
def safe(data,path):
 if re.search(rb'-----BEGIN (?:RSA |EC |OPENSSH |DSA )?PRIVATE KEY-----|(?:ghp_|github_pat_|sk-proj-)[A-Za-z0-9_\-]{20,}',data):raise RuntimeError('Sensitive material requires exclusion: '+str(path))
def copy_file(src,dest):
 data=src.read_bytes();safe(data,src);dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(data);shutil.copymode(src,dest)
# Keep complete metadata inventory; no SSH configuration or credential content.
for r in inv['repositories']:
 manifest['repositories'].append({'path':r['path'].removeprefix(str(root)+'/') if r['path']!=str(root) else '.','origin':r['origin_push'],'heads':r['heads']})
gx=next(x for x in inv['repositories'] if x['path'].endswith('/tools/ginux'));sourcegit=['git','--git-dir='+gx['common_dir'],'--work-tree='+str(root/'tools/ginux')];aux=tmp/'bundle.git';subprocess.run(['git','init','--bare','--quiet',str(aux)],check=True);(aux/'objects/info/alternates').write_text(str(Path(gx['common_dir'])/'objects')+'\n');ag=['git','--git-dir='+str(aux)];targets={}
def ref(name,sha):
 if sha in targets:return targets[sha]
 target='refs/migration/'+name;subprocess.run(ag+['update-ref',target,sha],check=True);targets[sha]=target;return target
for line in run(sourcegit+['for-each-ref','--format=%(refname)\t%(objectname)','refs/heads/','refs/tags/'],text=True).splitlines():
 name,sha=line.split('\t');ref('local/'+name.removeprefix('refs/'),sha)
for n,line in enumerate(gx['stashes'].splitlines()):ref('stashes/'+str(n),line.split()[0])
for i,w in enumerate(w for w in gx['worktrees'] if w.get('exists') and not w.get('metadata_only')):
 path=Path(w['worktree']);git=['git','-C',str(path),'--work-tree='+str(path)];head=run(git+['rev-parse','HEAD'],text=True).strip();ref('worktrees/'+str(i),head)
 status=run(git+['status','--porcelain=v1','--untracked-files=all'],text=True)
 if not status:continue
 slug=str(i)+'-'+path.name+'-'+hashlib.sha256(str(path).encode()).hexdigest()[:8];dest=tmp/'worktrees'/slug;dest.mkdir(parents=True);index=Path(run(git+['rev-parse','--path-format=absolute','--git-path','index'],text=True).strip());shutil.copyfile(index,dest/'index')
 stages=run(git+['ls-files','--stage'],text=True)
 for line in stages.splitlines():
  fields=line.split(None,3);sha=fields[1]
  if fields[0]!='160000' and sha!='0'*40:ref('index-objects/'+sha,sha)
 changed=run(git+['diff','--name-only','-z','HEAD']).decode().split('\0');untracked=run(git+['ls-files','--others','--exclude-standard','-z']).decode().split('\0');files=sorted(set(x for x in changed+untracked if x));entries=[]
 for name in files:
  rel=Path(name);assert not rel.is_absolute() and '..' not in rel.parts,name;src=path/rel;dst=dest/'files'/rel
  if src.is_symlink():dst.parent.mkdir(parents=True,exist_ok=True);dst.symlink_to(os.readlink(src));entries.append({'path':name,'symlink':os.readlink(src)})
  elif src.is_file():copy_file(src,dst);entries.append({'path':name,'sha256':hashlib.sha256(dst.read_bytes()).hexdigest()})
  elif not src.exists():entries.append({'path':name,'deleted':True})
  else:raise RuntimeError('Unexpected changed path '+str(src))
 data={'original_path':str(path),'head':head,'branch':w.get('branch',''),'status':status,'unmerged_index_entries':run(git+['ls-files','--unmerged'],text=True),'entries':entries,'snapshot':'worktrees/'+slug};(dest/'snapshot.json').write_text(json.dumps(data,indent=2)+'\n');manifest['worktree_snapshots'].append(data)
# Preserve prepared commit/composition objects, including held G07 and exact F02.
json_files=list((root/'docs/ginux-contributions').glob('*.json'))+list(Path('/tmp/ginux-contribution-preparation').glob('*/manifest.json'))+list(Path('/tmp/ginux-contribution-preparation').glob('*/state.json'))+list(Path('/tmp/ginux-contribution-preparation').glob('*/correction-manifest.json'))
shas=set()
for path in json_files:shas.update(re.findall(r'\b[0-9a-f]{40}\b',path.read_text()))
for sha in sorted(shas):
 p=subprocess.run(sourcegit+['cat-file','-t',sha],capture_output=True,text=True)
 if p.returncode==0 and p.stdout.strip()=='commit':ref('prepared/'+sha,sha)
manifest['ginux_bundle_refs']={name:sha for sha,name in targets.items()};bundle=out/'ginux.bundle';subprocess.run(ag+['bundle','create',str(bundle),'--all'],check=True);subprocess.run(ag+['bundle','verify',str(bundle)],check=True,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL);manifest['ginux_bundle_sha256']=hashlib.file_digest(bundle.open('rb'),'sha256').hexdigest();print('Ginux bundle',bundle.stat().st_size,'bytes',len(targets),'references',flush=True)
# Preserve orphaned former-submodule histories without changing obsolete origins.
(out/'retired').mkdir()
for rec in retired:
 gd=Path(rec['git_dir']);slug=str(gd.relative_to(root/'.git/modules')).replace('/','--');p=out/'retired'/(slug+'.bundle');git=['git','--git-dir='+str(gd),'--work-tree='+str(root)];subprocess.run(git+['bundle','create',str(p),'--branches','--tags'],check=True);subprocess.run(git+['bundle','verify',str(p)],check=True,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL);manifest['retired_repositories'].append({**rec,'bundle':'retired/'+p.name,'sha256':hashlib.file_digest(p.open('rb'),'sha256').hexdigest(),'bytes':p.stat().st_size});print('retired',slug,p.stat().st_size,flush=True)
# Development evidence and helpers, excluding binary roots/private keys/caches.
allowed={'.py','.sh','.rs','.c','.S','.json','.log','.txt','.patch','.index','.md','.out','.err','.audit','.stdout','.toml'}
art=root/'.artifacts/ginux-contribution-preparation'
for folder in sorted(p for p in art.iterdir() if p.is_dir()):
 for p in folder.iterdir():
  if p.is_file() and (p.suffix in allowed or p.name=='index'):
   dest=tmp/'evidence/ginux-contribution-preparation'/folder.name/p.name;copy_file(p,dest);manifest['evidence_files'].append(str(dest.relative_to(tmp)))
for p in Path('/tmp').glob('ginux-*.py'):
 if p.is_file():copy_file(p,tmp/'helpers'/p.name)
for folder in [Path('/tmp/ginux-contribution-preparation/F01'),Path('/tmp/ginux-contribution-preparation/F02'),Path('/tmp/ginux-contribution-preparation/F03')]:
 for p in folder.iterdir():
  if p.is_file() and (p.name in ['state.json','catalog.json'] or p.name.endswith(('-receipt.json','-provenance.json')) or p.suffix in ['.log','.txt','.patch']):copy_file(p,tmp/'evidence/latest'/folder.name/p.name)
# The exact Recipe test source overlay is small and cannot be reconstructed from HEAD alone.
f3=Path('/tmp/ginux-contribution-preparation/F03');state=json.loads((f3/'state.json').read_text())
for name in state.get('tracked_dirty_overlay_sha256',{}):copy_file(f3/'recipe-source'/name,tmp/'recipe-snapshot-overlay'/name)
for name in ['Cargo.toml','Cargo.lock']:copy_file(f3/'recipe-source'/name,tmp/'recipe-snapshot-overlay'/name)
for folder,label in [(root/'.artifacts/untangle-ui','untangle-ui'),(root/'tools/recipe/.untangle','recipe-untangle')]:
 if folder.exists():
  for p in folder.rglob('*'):
   if p.is_file() and not p.is_symlink() and p.suffix in {'.py','.sh','.md','.txt','.json','.log'} and p.stat().st_size<2_000_000:copy_file(p,tmp/'other-development'/label/p.relative_to(folder))
# Save inventories and portable state; all original worktree/stash files remain intact.
for p in Path('/tmp').glob('levitate-migration-*.json'):copy_file(p,tmp/'inventory'/p.name)
with tarfile.open(out/'recovery.tar.gz','w:gz',compresslevel=6) as tar:
 for p in sorted(tmp.iterdir()):
  if p.name!='bundle.git':tar.add(p,arcname=p.name,recursive=True)
manifest['recovery_archive_sha256']=hashlib.file_digest((out/'recovery.tar.gz').open('rb'),'sha256').hexdigest();manifest['recovery_archive_bytes']=(out/'recovery.tar.gz').stat().st_size;manifest['recipe_snapshot']=state;manifest['source_helper']='levitate-migration-preserve.py';shutil.copyfile('/tmp/levitate-migration-preserve.py',out/'levitate-migration-preserve.py');(out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n');assert all(p.stat().st_size<95_000_000 for p in out.rglob('*') if p.is_file());print('recovery archive',manifest['recovery_archive_bytes'],'bytes; snapshots',len(manifest['worktree_snapshots']),'retired',len(retired),flush=True)
