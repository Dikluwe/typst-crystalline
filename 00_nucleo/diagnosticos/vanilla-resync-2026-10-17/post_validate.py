from pathlib import Path
import subprocess,json,datetime,hashlib
root=Path.cwd();out=root/'00_nucleo/diagnosticos/vanilla-resync-2026-10-17'
results=[]
for label,cmd in [
 ('catalog-check',['python3',str(out/'catalog_check.py')]),
 ('matrix',['python3',str(out/'matrix.py')]),
 ('symbols',['python3','lab/parity/tools/p1495_symbols_probe.py','--crystalline',str(root/'target/debug/typst')]),
 ('language-probes',['python3','lab/parity/tools/p1497_language_probes.py','--crystalline',str(root/'target/debug/typst')]),
]:
 start=datetime.datetime.now(datetime.timezone.utc).isoformat()
 with (out/(label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 item={'label':label,'command':cmd,'start':start,'end':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit':r.returncode};results.append(item);print(json.dumps(item),flush=True)
 (out/'post-validation-results.json').write_text(json.dumps(results,indent=2)+'\n')
observations=[]
for name in ['lab/typst-original/target/release/typst','target/debug/typst']:
 p=root/name;sha=hashlib.sha256(p.read_bytes()).hexdigest()
 for args in [['--version'],['eval','sys.version','--format','json']]:
  r=subprocess.run([str(p),*args],capture_output=True,text=True,timeout=30)
  observations.append({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':[str(p),*args],'binary_sha256':sha,'exit':r.returncode,'stdout':r.stdout,'stderr':r.stderr})
  assert r.returncode==0
  if args[0]=='eval':assert r.stdout.strip()=='"version(0, 15, 1)"'
(out/'public-observations.json').write_text(json.dumps(observations,indent=2)+'\n')
assert all(r['exit'] in [0,1] for r in results),results
assert results[0]['exit']==0
