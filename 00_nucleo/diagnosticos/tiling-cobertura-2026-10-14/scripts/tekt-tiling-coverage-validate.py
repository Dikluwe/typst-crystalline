from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,collections,os,re,sys
out=Path('00_nucleo/diagnosticos/tiling-cobertura-2026-10-14')
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
with gzip.open(out/'candidate-final.json.gz','rt') as f:paths=list(json.load(f)['files'])
pre=dict(utc=now(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:dict(text=Path(p).read_text(),sha256=sha(p)) for p in paths})
with gzip.open(out/'pre-validation.json.gz','wt') as f:json.dump(pre,f)
linter='/tmp/tekt-linter-ci-target/debug/crystalline-lint'
commands=[('build',['cargo','build','--offline','--locked']),('rustdoc',['cargo','doc','--offline','--locked','--workspace','--no-deps']),('check',['cargo','check','--offline','--locked','--workspace','--all-targets']),('clippy',['cargo','clippy','--offline','--locked','--workspace','--all-targets','--message-format=json','--','--cap-lints','warn']),('fmt',['cargo','fmt','--check','--all']),('lineage',[linter,'.','--checks','v1,v5,v15,v26','--fail-on','warning']),('reverse',['python3','tools/ci/check_reverse_lineage.py']),('architecture',[linter,'.','--format','sarif']),('diff',['git','diff','--check'])]
results=[]
for name,cmd in commands:
 start=now()
 with (out/f'followup-{name}.log').open('w') as log:
  r=subprocess.run(cmd,stdout=log,stderr=subprocess.STDOUT,env=dict(os.environ, **({"RUSTDOCFLAGS":"-D warnings"} if name=="rustdoc" else {})))
 result=dict(start=start,end=now(),command=cmd,exit=r.returncode,name=name)
 if name=='clippy':result['note']='Inventory only, not the -D warnings CI gate.'
 results.append(result);(out/'followup-validation-results.json').write_text(json.dumps(results,indent=2));print(result,flush=True)
 if r.returncode:sys.exit(r.returncode)
assert all(sha(p)==x['sha256'] for p,x in pre['files'].items())
unique={}
for line in (out/'followup-clippy.log').read_text().splitlines():
 try:x=json.loads(line)
 except ValueError:continue
 if x.get('reason')!='compiler-message':continue
 m=x['message'];code=m.get('code');spans=[s for s in m.get('spans',[]) if s.get('is_primary')]
 if not code or not spans or not all(s['file_name'].startswith(('01_core/','02_shell/','03_infra/','04_wiring/','benches/')) for s in spans):continue
 key=(code['code'],m['message'],tuple((s['file_name'],s['byte_start'],s['byte_end']) for s in spans))
 unique[key]=dict(code=code['code'],message=m['message'],spans=spans)
inventory=list(unique.values());(out/'followup-clippy-inventory.json').write_text(json.dumps(inventory,indent=2))
prior=json.loads(Path('00_nucleo/diagnosticos/stroke-paint-2026-10-14/followup-clippy-inventory.json').read_text());a=collections.Counter(x['code'] for x in prior);b=collections.Counter(x['code'] for x in inventory)
sarif=json.loads((out/'followup-architecture.log').read_text());levels=collections.Counter(x['level'] for x in sarif['runs'][0]['results'])
summary=dict(end=now(),sources_unchanged=True,clippy_before=len(prior),clippy_after=len(inventory),clippy_delta={k:b[k]-a[k] for k in a.keys()|b.keys() if b[k]!=a[k]},architecture=dict(levels))
(out/'followup-summary.json').write_text(json.dumps(summary,indent=2));print(summary,flush=True)
