from pathlib import Path
import json,subprocess,datetime,sys,hashlib,gzip
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];plan=json.load(open('00_nucleo/diagnosticos/tekt-v16-introspect-carriers-2026-10-17/projection-plan.json'))['content'];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();frozen={p:sha(p) for p in b['hashes']}
if phase=='before':source='01_core/src/compiler/introspect/bibliography.rs';index=0
else:source='01_core/src/entities/content/traversal.rs';index=next(i for i,m in enumerate(json.loads(subprocess.check_output(['/tmp/tekt-math-layout-latent',source,'matches'],text=True))) if m['scrutinee']=='self' and 'Content::PageRun' in str(m['arms']) and 'Content::TableCell' in str(m['arms']) and all('=>' not in a['pattern'] for a in m['arms']))
matches=json.loads(subprocess.check_output(['/tmp/tekt-math-layout-latent',source,'matches'],text=True));arms=matches[index]['arms'];declarations=[]
for v in plan:
 declarations.append(v['name']+('(())' if v['kind']=='(' else '{'+','.join(f['name']+':()' for f in v['fields'])+'}' if v['kind']=='{' else ''))
results=[]
for future in [False,True]:
 path=out/(phase+('-future' if future else '-control')+'.rs');path.write_text('#![allow(dead_code,unused_variables,unreachable_patterns)]\nenum Content{'+','.join(declarations+(['FutureBibliography'] if future else []))+'}\nfn classify(input:&Content){match input{'+''.join(a['pattern']+'=>(),' for a in arms)+'}}\nfn main(){}\n');cmd=['rustc','--edition=2021','--crate-name','bibliography_future','--error-format=json',str(path),'-o','/tmp/tekt-bib-static-probe'];r=subprocess.run(cmd,capture_output=True,text=True);errors=[]
 for line in r.stderr.splitlines():
  try:m=json.loads(line)
  except ValueError:continue
  if m.get('level')=='error' and m.get('code'):errors.append({'code':m['code']['code'],'message':m['message']})
 results.append(dict(future=future,command=cmd,exit=r.returncode,errors=errors,source_sha256=sha(path)))
 assert r.returncode==(1 if future and phase=='after' else 0),(phase,future,r.stderr)
 if future and phase=='after':assert [e['code'] for e in errors]==['E0004']
assert all(sha(p)==h for p,h in frozen.items());record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=frozen,pattern_source=source,source_sha256=sha(source),patterns=[a['pattern'] for a in arms],domain_variants=len(plan),results=results,method='Actual match patterns; opaque payloads and unchanged enum names/shapes. Structural exhaustiveness only, not language or payload proof.');(out/(phase+'-future-result.json')).write_text(json.dumps(record,indent=2)+'\n');print(phase,[(x['future'],x['exit'],x['errors']) for x in results])
