from pathlib import Path
import json,subprocess,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-walk-2026-10-16');source='01_core/src/compiler/introspect/walk.rs'
def run(label,cmd,exit):
 print(label,flush=True)
 with (out/('source-refinement-'+label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 assert r.returncode==exit,(label,r.returncode)
run('fmt',['cargo','fmt','--all'],0);run('seal-red',['python3',str(out/'workers/seal_owned.py'),source],0);run('red',['cargo','test','--offline','--locked','-p','typst-core','--lib','v16_walk_','--','--test-threads=1'],101)
assert '7 passed; 2 failed;' in (out/'source-refinement-red.log').read_text()
p=Path(source);s=p.read_text();start=s.index('        Content::Source(e) => {\n            walk(');end=s.index('        Content::ShowRecipe(e) => {',start);old=s[start:end];assert old.count('label_from_parent,')==1;new=old.replace('            walk(', '            walk_at(',1).replace('                label_from_parent,','                label_from_parent,\n                forced_location,\n                shallow_table,',1);s=s[:start]+new+s[end:];p.write_text(s)
r=json.load(open(out/'refinement.json'));r['sources'][source]=s;r['source_refinement']={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'old':old,'new':new,'red':'7 passed; 2 failed;'};(out/'refinement.json').write_text(json.dumps(r,indent=2)+'\n')
run('fmt-green',['cargo','fmt','--all'],0);run('seal-green',['python3',str(out/'workers/seal_owned.py'),source],0);run('guard',['python3',str(out/'workers/guard.py')],0)
print('Source refinement implemented after RED',flush=True)
