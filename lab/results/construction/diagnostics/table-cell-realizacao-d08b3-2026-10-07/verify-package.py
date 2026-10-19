from pathlib import Path
import hashlib,json,gzip,subprocess,datetime,tomllib,re
R=Path.cwd(); D=R/'00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07'; M=R/'00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07'
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
m=json.loads((M/'manifest.json').read_text()); errors=[]
for e in m['owners']:
 for key,path in [('candidate_sha256',M/e['owner'])]+([] if e['new_consumer'] else [('base_source_sha256',R/e['consumer']),('base_l0_sha256',R/e['owner'])]):
  if sha(path)!=e[key]: errors.append(str(path))
 if e['new_consumer'] and ((R/e['consumer']).exists() or (R/e['owner']).exists()): errors.append('new owner already installed')
 text=(M/e['owner']).read_text()
 if text.count(m['nucleus']['path']+' sha256:'+m['nucleus']['pin'])!=1: errors.append('pin '+e['owner'])
 if e['consumer'] not in text: errors.append('owner '+e['owner'])
n=M/m['nucleus']['path']; parsed=tomllib.loads(n.read_text()); h=hashlib.sha256(n.read_bytes());h.update(b'\0TEKT-NUCLEUS-DEPS-V1\0')
assert h.hexdigest()==m['nucleus']['pin'];assert sha(n)==m['nucleus']['candidate_sha256'];assert not (R/m['nucleus']['path']).exists()
assert len({e['owner'] for e in m['owners']})==len(m['owners'])==len({e['consumer'] for e in m['owners']})
for p,v in m['evidence'].items():
 if sha(R/p)!=v:errors.append(p)
b=json.loads(gzip.decompress((D/'baseline-state.json.gz').read_bytes()))
roots=('01_core/','02_shell/','03_infra/','04_wiring/','00_nucleo/prompts/')
bh={p:v for p,v in b['sha256'].items() if p.startswith(roots)}
paths=set(subprocess.check_output(['git','ls-files','--modified'],text=True).splitlines()+subprocess.check_output(['git','ls-files','--others','--exclude-standard','01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'],text=True).splitlines())
ch={p:sha(R/p) for p in paths if p.startswith(roots) and (R/p).is_file()}
changed=sorted(p for p in bh.keys()|ch.keys() if bh.get(p)!=ch.get(p))
if changed:errors+=changed
co=[]
for name in ['before','additional','order','aliases']:
 d=json.loads(gzip.decompress((D/(name+'.json.gz')).read_bytes())); co.append({'name':name,'cases':len(d['cases']),'utc':d['utc'],'finished_utc':d['finished_utc'],'bin_sha256':d['bin_sha256']})
a={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'manifest_sha256':sha(M/'manifest.json'),'owner_count':len(m['owners']),'productive_or_canonical_l0_changes_this_turn':changed,'errors':errors,'collections':co,'environment':'Measurement subprocesses inherited the session environment; no task overrides. Exact environment not frozen; binary and sources frozen/hashed.','tests_this_turn':'No build or cargo tests: measurements/specification only.'}
(D/'package-audit.json').write_text(json.dumps(a,ensure_ascii=False,indent=2)+'\n');assert not errors,errors
print(json.dumps(a,ensure_ascii=False,indent=2))
