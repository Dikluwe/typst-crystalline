from pathlib import Path
import subprocess,json,datetime
out=Path(__file__).resolve().parents[1];assert json.load(open(out/'red-resources.json'))['exit']==101;assert '1 passed; 4 failed;' in (out/'red.log').read_text();rows=[]
commands=[('cast-controls-before',['cargo','test','--offline','--locked','-p','typst-core','--lib','measure_domain_','--','--test-threads=1'],0),('red-probe',['python3',str(out/'workers/probe.py'),'red'],0),('extend-tests',['python3',str(out/'workers/extend_tests.py')],0),('red-located',['cargo','test','--offline','--locked','-p','typst-core','--lib','figure_domain_located_','--','--test-threads=1'],101)]
for label,cmd,expected in commands:
 with (out/(label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 rows.append(dict(label=label,command=cmd,exit=r.returncode,utc=datetime.datetime.now(datetime.timezone.utc).isoformat()));(out/'post-red-result.json').write_text(json.dumps(rows,indent=2)+'\n');assert r.returncode==expected,(label,r.returncode)
assert '1 passed; 0 failed;' in (out/'cast-controls-before.log').read_text();assert '0 passed; 1 failed;' in (out/'red-located.log').read_text();print('POST RED PASS')
