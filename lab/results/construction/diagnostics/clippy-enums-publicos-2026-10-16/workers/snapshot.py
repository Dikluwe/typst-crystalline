from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,re
out=Path('00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
last=json.loads(Path('00_nucleo/diagnosticos/clippy-enums-2026-10-16/final-state.json').read_text())
assert all(sha(p)==h for p,h in last['files'].items()),'Previous stage source drift'
files={str(p):p.read_text() for top in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files=files,hashes={p:hashlib.sha256(s.encode()).hexdigest() for p,s in files.items()})
with gzip.open(out/'before.json.gz','wt') as f:json.dump(record,f)
r=subprocess.run(['rg','-n',r'Tag::(Start|End)|Vec<Tag>|\[Tag\]|Transformation::Style|show_rules.to_vec|show_rules.clone|node_rules: Vec|regex_rules: Vec','01_core','02_shell','03_infra','04_wiring','benches'],capture_output=True,text=True);(out/'callers.txt').write_text(r.stdout)
print(json.dumps(dict(utc=record['utc'],head=record['head'],source_files=len(files),prior_stage_verified=True)))
