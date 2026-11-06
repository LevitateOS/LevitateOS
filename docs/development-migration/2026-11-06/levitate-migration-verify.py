from pathlib import Path
import subprocess,json,tarfile,io,os,shutil
root=Path('/home/vince/Projects/LevitateOS/docs/development-migration/2026-11-06');m=json.loads((root/'manifest.json').read_text());v=Path('/tmp/levitate-migration-verify');assert not v.exists();v.mkdir();bare=v/'objects.git';subprocess.run(['git','init','--bare','--quiet',str(bare)],check=True);git=['git','--git-dir='+str(bare)];p=subprocess.run(git+['fetch','--quiet',str(root/'ginux.bundle'),'refs/migration/*:refs/migration/*'],capture_output=True,text=True);assert p.returncode==0,p.stderr
for name,sha in m['ginux_bundle_refs'].items():assert subprocess.check_output(git+['rev-parse',name],text=True).strip()==sha
with tarfile.open(root/'recovery.tar.gz') as tar:tar.extractall(v/'recovery',filter='data')
records=[]
for snap in m['worktree_snapshots']:
 d=v/('checkout-'+Path(snap['snapshot']).name);d.mkdir()
 with tarfile.open(fileobj=io.BytesIO(subprocess.check_output(git+['archive',snap['head']]))) as tar:tar.extractall(d,filter='data')
 src=v/'recovery'/snap['snapshot']
 for e in snap['entries']:
  dst=d/e['path'];source=src/'files'/e['path']
  if dst.exists() or dst.is_symlink():
   assert not dst.is_dir() or dst.is_symlink(),dst;dst.unlink()
  if e.get('deleted'):continue
  dst.parent.mkdir(parents=True,exist_ok=True)
  if 'symlink' in e:dst.symlink_to(e['symlink'])
  else:shutil.copy2(source,dst)
 idx=v/(d.name+'.index');shutil.copyfile(src/'index',idx);(bare/'HEAD').write_text(snap['head']+'\n');env={**os.environ,'GIT_INDEX_FILE':str(idx)};status=subprocess.check_output(git+['--work-tree='+str(d),'-c','core.bare=false','status','--porcelain=v1','--untracked-files=all'],env=env,text=True);assert status==snap['status'],(snap['original_path'],status,snap['status']);records.append({'original_path':snap['original_path'],'head':snap['head'],'status_identical':True,'unmerged_entries_preserved':bool(snap['unmerged_index_entries'])});print('restored exact staged/unstaged/conflict state',snap['original_path'],flush=True)
for r in m['retired_repositories']:
 p=subprocess.run(['git','--git-dir='+str(bare),'bundle','verify',str(root/r['bundle'])],capture_output=True,text=True);assert p.returncode==0,(r['bundle'],p.stderr)
proof={'ginux_reference_objects_imported':len(m['ginux_bundle_refs']),'worktree_restore_checks':records,'retired_self_contained_bundles_verified':len(m['retired_repositories']),'scope':'fresh isolated Git store and disposable checkouts; original worktree/index/ref/stash state unchanged'};(root/'restore-verification.json').write_text(json.dumps(proof,indent=2)+'\n');print('independent migration recovery verified',len(records),flush=True)
