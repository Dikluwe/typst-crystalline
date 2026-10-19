import datetime,gzip,hashlib,json,os,re,subprocess
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2];O=Path('/tmp/celulas-prioridade5-2026-10-07')
rec={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'cases':[]}
for engine,exe in {'previous':O/'previous-fc5b2571','current':R/'target/release/typst'}.items():
 names=[x.split()[-1] for x in subprocess.check_output(['nm',str(exe)],text=True).splitlines() if 'layout_sub_frame_with_origin' in x and ' t ' in x]
 assert names
 script=O/(engine+'.gdb');script.write_text('set pagination off\nset breakpoint pending off\nset $frame_calls = 0\n'+'\n'.join("break '"+n+"'\ncommands\nsilent\nset $frame_calls = $frame_calls + 1\ncontinue\nend" for n in names)+'\nrun\nprintf "\\nFRAME_CALLS=%d\\n", $frame_calls\n')
 for name in ['grid_literal_punctuation','table_literal_punctuation','grid_literal_alnum','table_literal_alnum','grid_punctuation','table_spaced','table_unicode','styled','wrapped']:
  fixture=D/'fixtures'/(name+'.typ');cmd=['gdb','--batch','-x',str(script),'--args',str(exe),'compile',str(fixture),str(O/(name+'-count-'+engine+'.pdf')),'--pdf-tagged=false','--pretty=false']
  p=subprocess.run(cmd,cwd=R,env=dict(os.environ,RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0'),capture_output=True,text=True,timeout=90)
  m=re.search(r'FRAME_CALLS=(\d+)',p.stdout);calls=int(m[1]) if m else None
  rec['cases'].append({'engine':engine,'fixture':name,'binary_sha256':hashlib.sha256(exe.read_bytes()).hexdigest(),'command':cmd,'script':script.read_text(),'exit':p.returncode,'calls':calls,'stdout':p.stdout,'stderr':p.stderr})
  print(engine,name,calls,p.stderr[-160:],flush=True)
rec['candidate_state_sha256']=hashlib.sha256((D/'candidate-state.json.gz').read_bytes()).hexdigest()
rec['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(D/'count.json.gz','wt') as f:json.dump(rec,f,indent=2)
assert all(x['exit']==0 and x['calls'] for x in rec['cases'])
