from pathlib import Path
import datetime,gzip,hashlib,json,subprocess
out=Path(__file__).resolve().parents[1];old=json.load(gzip.open(out/'before-measurement.json.gz'));allowed={'invalid-base','invalid-both','huge-exp-invalid-base','decimal-exp','decimal-both'};rows=[]
for previous in old['cases']:
 if previous['binary']!='tekt':continue
 r=subprocess.run(previous['argv'],capture_output=True,text=True);row=dict(previous,name=previous['name'],utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),exit=r.returncode,stdout=r.stdout,stderr=r.stderr);rows.append(row)
 assert r.returncode==previous['exit'] and r.stdout==previous['stdout']
 if previous['name'] not in allowed:assert r.stderr==previous['stderr'],previous['name']
 if previous['name']=='huge-exp-invalid-base':assert 'base:' in r.stderr and 'expoente demasiado' not in r.stderr
 if previous['name'] in ['decimal-exp','decimal-both']:assert 'hint:' in r.stderr and 'float(value)' in r.stderr
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),binary_sha256=hashlib.sha256(Path('target/debug/typst').read_bytes()).hexdigest(),source_sha256=hashlib.sha256(Path('01_core/src/compiler/stdlib/calc/power.rs').read_bytes()).hexdigest(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),cases=rows,allowed_changed_diagnostics=sorted(allowed))
with gzip.open(out/'after-measurement.json.gz','wt') as f:json.dump(record,f)
print('20 CLI cases validated; changed diagnostics limited to five prescribed cases')
