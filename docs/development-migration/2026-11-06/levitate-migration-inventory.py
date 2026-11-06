from pathlib import Path
import subprocess,json,os
root=Path('/home/vince/Projects/LevitateOS');seen=set();queue=[root];records=[];errors=[]
def git(path,*args,ok=True):
 p=subprocess.run(['git','-C',str(path),'--work-tree='+str(path),*args],capture_output=True,text=True)
 if ok and p.returncode:raise RuntimeError((str(path),list(args),p.stderr))
 return p.stdout.strip()
while queue:
 path=queue.pop(0)
 if not (path/'.git').exists():continue
 try:
  common=git(path,'rev-parse','--git-common-dir');common=str((path/common).resolve());
  if common in seen:continue
  seen.add(common);worktrees=[]
  for block in git(path,'worktree','list','--porcelain').split('\n\n'):
   data={line.split(' ',1)[0]:line.split(' ',1)[1] if ' ' in line else True for line in block.splitlines()};wp=Path(data['worktree']);data['exists']=wp.exists();data['metadata_only']=(str(wp)==common or '.git' in wp.parts)
   if wp.exists() and not data['metadata_only']:data['status']=git(wp,'status','--porcelain=v1','--untracked-files=all');data['head']=git(wp,'rev-parse','HEAD');data['branch']=git(wp,'branch','--show-current');data['instructions']=[str(p) for p in [wp/'AGENTS.md'] if p.exists()]
   worktrees.append(data)
  if not any(w['worktree']==str(path) for w in worktrees):
   worktrees.insert(0,{'worktree':str(path),'exists':True,'metadata_only':False,'status':git(path,'status','--porcelain=v1','--untracked-files=all'),'head':git(path,'rev-parse','HEAD'),'branch':git(path,'branch','--show-current'),'instructions':[str(path/'AGENTS.md')] if (path/'AGENTS.md').exists() else []})
  heads=[]
  for line in git(path,'for-each-ref','--format=%(refname:short)\t%(objectname)\t%(upstream:short)','refs/heads/').splitlines():
   fields=line.split('\t');heads.append({'name':fields[0],'head':fields[1],'upstream':fields[2] if len(fields)>2 else ''})
  record={'path':str(path),'common_dir':common,'origin_fetch':git(path,'remote','get-url','origin',ok=False),'origin_push':git(path,'remote','get-url','--push','origin',ok=False),'worktrees':worktrees,'heads':heads,'stashes':git(path,'stash','list','--format=%H %gs'),'gitlinks':git(path,'ls-files','--stage').count('160000 ')};records.append(record)
  for wt in worktrees:
   wp=Path(wt['worktree']);gm=wp/'.gitmodules'
   if gm.exists() and not wt.get('metadata_only'):
    for line in git(wp,'config','--file',str(gm),'--get-regexp',r'^submodule\..*\.path$',ok=False).splitlines():
     _,sp=line.split(None,1);queue.append(wp/sp)
 except Exception as e:errors.append(str(e))
out={'root':str(root),'repositories':records,'errors':errors};Path('/tmp/levitate-migration-inventory.json').write_text(json.dumps(out,indent=2)+'\n');print('repos',len(records),'errors',errors)
for x in records:
 dirty=[w for w in x['worktrees'] if w.get('status')];print(x['path'],'origin',x['origin_push'],'branches',len(x['heads']),'worktrees',len(x['worktrees']),'dirty',len(dirty),'stashes',len(x['stashes'].splitlines()))
 for w in dirty:print(' DIRTY',w['worktree'],w['branch'] or '(detached)',repr(w['status'][:900]))
