from pathlib import Path
import json,hashlib,subprocess,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16');source='01_core/src/compiler/eval/call_dispatch/content_methods.rs';p=Path(source);old=p.read_text();a=old.index('\n#[cfg(test)]\nmod v16_walk_receiver_tests');b=old.index('\nfn outline_entry_receiver(',a);head,tests,helper=old[:a],old[a:b],old[b:];new=head+helper+'\n'+tests;p.write_text(new)
r=json.load(open(out/'refinement.json'));r['sources'][source]=new;r['layout_cleanup']={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'reason':'private helper before cfg(test) module; unchanged helper and test tokens','before_sha256':hashlib.sha256(old.encode()).hexdigest(),'after_sha256':hashlib.sha256(new.encode()).hexdigest()};(out/'refinement.json').write_text(json.dumps(r,indent=2)+'\n')
for label,cmd in [('fmt',['cargo','fmt','--all']),('seal',['python3',str(out/'workers/seal_owned.py'),source]),('guard',['python3',str(out/'workers/guard.py')])]:
 with (out/('layout-cleanup-'+label+'.log')).open('w') as f:result=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 assert result.returncode==0,label
print('Private helper placed before tests; code/L0 guards PASS',flush=True)
