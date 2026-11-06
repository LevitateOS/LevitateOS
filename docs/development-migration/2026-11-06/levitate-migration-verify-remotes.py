from pathlib import Path
import subprocess,json,os,concurrent.futures,sys
inv=json.loads(Path('/tmp/levitate-migration-inventory.json').read_text());root=inv['root'];skip='--skip-root' in sys.argv;env={**os.environ,'GIT_SSH_COMMAND':'ssh -o BatchMode=yes -o ConnectTimeout=10'}
def check(r):
 path=r['path'];git=['git','-C',path,'--work-tree='+path];p=subprocess.run(git+['ls-remote','--heads','--tags','origin'],env=env,capture_output=True,text=True,timeout=45);assert p.returncode==0,(path,p.stderr);remote={line.split()[1]:line.split()[0] for line in p.stdout.splitlines()};heads=[]
 for line in subprocess.check_output(git+['for-each-ref','--format=%(refname)\t%(objectname)','refs/heads/'],text=True).splitlines():
  ref,sha=line.split('\t');there=remote.get(ref);state='exact' if there==sha else 'contained' if there and subprocess.run(git+['merge-base','--is-ancestor',sha,there],capture_output=True).returncode==0 else 'MISSING OR DIVERGENT';heads.append({'ref':ref,'local':sha,'origin':there,'verified':state});assert state!='MISSING OR DIVERGENT',(path,ref,sha,there)
 tags=[]
 for line in subprocess.check_output(git+['for-each-ref','--format=%(refname)\t%(objectname)','refs/tags/'],text=True).splitlines():
  ref,sha=line.split('\t');assert remote.get(ref)==sha,(path,ref,sha,remote.get(ref));tags.append({'ref':ref,'sha':sha,'verified':'exact'})
 return {'path':path.removeprefix(root+'/') if path!=root else '.','origin':r['origin_push'],'branches':heads,'tags':tags,'worktree_status':subprocess.check_output(git+['status','--porcelain=v1'],text=True)}
with concurrent.futures.ThreadPoolExecutor(max_workers=5) as pool:results=list(pool.map(check,[r for r in inv['repositories'] if not skip or r['path']!=root]))
report={'scope':'all active origin repository branch/tag tips, with one older local main retained as an origin ancestor; no force updates','repositories':results,'repository_count':len(results),'branch_count':sum(len(r['branches']) for r in results),'tag_count':sum(len(r['tags']) for r in results)};dest=Path('/tmp/levitate-migration-final-verification.json' if not skip else '/tmp/levitate-migration-submodule-verification.json');dest.write_text(json.dumps(report,indent=2)+'\n');print('verified',report['repository_count'],'origins',report['branch_count'],'branches',report['tag_count'],'tags',flush=True)
for r in results:
 if r['worktree_status']:print('remaining live changes',r['path'],repr(r['worktree_status']),flush=True)
