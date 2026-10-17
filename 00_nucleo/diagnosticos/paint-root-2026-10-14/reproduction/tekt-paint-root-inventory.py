from pathlib import Path
import subprocess,json,gzip,re,hashlib,datetime
out=Path('00_nucleo/diagnosticos/paint-root-2026-10-14');out.mkdir(exist_ok=True)
paths=subprocess.check_output(['rg','--files','01_core','02_shell','03_infra','04_wiring','benches','-g','*.rs'],text=True).splitlines();paths=[p for p in paths if 'FrameItem::Group' in Path(p).read_text()]
x=subprocess.run(['/tmp/tekt-group-syntax/target/debug/tekt-group-syntax']+paths,capture_output=True,text=True,check=True);nodes=[json.loads(s) for s in x.stdout.splitlines()]
(out/'syntax-nodes.json').write_text(json.dumps(nodes,indent=2));selected={r['path'] for r in nodes if r['kind']=='expr' or not r.get('rest',True)};selected.add('01_core/src/entities/layout_types.rs')
owners={p:re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M).group(1) for p in sorted(selected) if not p.startswith('benches/')};(out/'owners.json').write_text(json.dumps(owners,indent=2))
allpaths=sorted(selected)+list(owners.values());sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:dict(text=Path(p).read_text(),sha256=sha(p)) for p in allpaths})
with gzip.open(out/'before-source.json.gz','wt') as f:json.dump(r,f)
for p in sorted(selected):
 rows=[r for r in nodes if r['path']==p and (r['kind']=='expr' or not r.get('rest',True))];print(p,[(r['kind'],r['line']) for r in rows])
