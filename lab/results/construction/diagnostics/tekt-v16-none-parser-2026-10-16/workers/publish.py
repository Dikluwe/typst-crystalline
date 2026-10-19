from pathlib import Path
import subprocess,json,hashlib,datetime,gzip
out=Path('00_nucleo/diagnosticos/tekt-v16-none-parser-2026-10-16').resolve()
package=json.load(open(out/'package-result.json'));proposal=json.load(open(out/'proposed-pin.json'))
commit='12797073972e375d9ab078cf6750c8b30b71e2b9';branch='codex/v16-none-prelude'
assert package['commit']==commit and package['branch']==branch
manifest=json.load(open(out/'manifest-sha256.json'))
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert sha(out/'manifest-sha256.json')=='fd4f7669549b54e80f80ca2cfc781f32c0e3179f5db28f1e07a2d0b607db8068'
assert all(sha(out/p)==h for p,h in manifest.items())
gitroot=Path(package['checkout']);remote='https://github.com/Dikluwe/tekt-linter.git'
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
record={'start':now(),'authorization':'User approved publishing this exact branch/commit and applying the prepared consumer pin; no merge/release or consumer commit/push.','commit':commit,'branch':branch,'remote':remote,'commands':[]}
def run(label,cmd,cwd=None):
 item={'label':label,'command':cmd,'start':now()}
 p=subprocess.run(cmd,cwd=cwd,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
 item.update(exit=p.returncode,end=now(),stdout=p.stdout,stderr=p.stderr);record['commands'].append(item)
 (out/'publication-result.json').write_text(json.dumps(record,indent=2)+'\n')
 assert p.returncode==0,{'label':label,'stderr':p.stderr}
 print(json.dumps({'label':label,'exit':p.returncode}),flush=True)
 return p.stdout
assert run('local-head',['git','rev-parse','HEAD'],gitroot).strip()==commit
assert not run('local-clean',['git','status','--porcelain'],gitroot)
assert run('origin',['git','remote','get-url','origin'],gitroot).strip()==remote
for path,item in proposal.items():assert Path(path).read_text()==item['before'],path
existing=run('remote-before',['git','ls-remote',remote,'refs/heads/'+branch]).strip()
assert not existing or existing.split()[0]==commit,existing
run('push',['git','push','--porcelain','origin',commit+':refs/heads/'+branch],gitroot)
after=run('remote-after',['git','ls-remote',remote,'refs/heads/'+branch]).strip()
assert after.split()[0]==commit,after
verify=Path('/tmp/tekt-linter-none-remote-check');assert not verify.exists();verify.mkdir()
run('verify-init',['git','init','--quiet'],verify)
run('verify-fetch',['git','fetch','--quiet','--no-tags','--depth=1',remote,commit],verify)
assert run('verify-commit',['git','rev-parse','FETCH_HEAD'],verify).strip()==commit
assert run('verify-tree',['git','rev-parse','FETCH_HEAD^{tree}'],verify).strip()==package['tree']
record.update(remote_exact_commit_verified=True,remote_tree=package['tree'])
# Apply only the reviewed four replacements; refuse any concurrent file change.
for path,item in proposal.items():assert Path(path).read_text()==item['before'],path
for path,item in proposal.items():Path(path).write_text(item['after'])
record.update(end=now(),pin_applied=True,pin_files={p:sha(p) for p in proposal},consumer_committed=False,consumer_pushed=False,merged=False,released=False)
(out/'publication-result.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps({'published':commit,'tree':package['tree'],'pin_applied':True,'merge':False,'release':False}),flush=True)
