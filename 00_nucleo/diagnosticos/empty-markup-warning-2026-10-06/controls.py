from pathlib import Path
import json,subprocess,datetime,hashlib
h=Path('00_nucleo/diagnosticos/empty-markup-warning-2026-10-06');old=json.load(open(h/'before.json'));bins={'tekt':Path('/tmp/tekt-empty-warning/typst-final'),'vanilla':Path('lab/typst-original/target/release/typst').resolve()}
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'status':subprocess.check_output(['git','status','--short'],text=True),'binaries':{k:hashlib.sha256(p.read_bytes()).hexdigest() for k,p in bins.items()},'cases':{}}
for n,c in old['cases'].items():
 p=Path(c['engines']['tekt']['command'][3]);p.parent.mkdir(parents=True,exist_ok=True);p.write_text(c['source'])
 e={'source':c['source'],'engines':{}}
 for k,b in bins.items():
  cmd=[str(b),*c['engines'][k]['command'][1:]];r=subprocess.run(cmd,capture_output=True,text=True);e['engines'][k]={'command':cmd,'returncode':r.returncode,'stdout':r.stdout,'stderr':r.stderr}
 e['exact_match']=all(e['engines']['tekt'][f]==e['engines']['vanilla'][f] for f in ['returncode','stderr','stdout']);d['cases'][n]=e;print(n,e['exact_match'])
(h/'controls.json').write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n')
