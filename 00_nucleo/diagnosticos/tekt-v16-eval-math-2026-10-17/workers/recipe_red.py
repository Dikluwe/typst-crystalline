from pathlib import Path
import json,subprocess,hashlib,gzip,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['hashes']};cmd=['cargo','test','--offline','--locked','-p','typst-core','--lib','v16_eval_math_','--','--test-threads=1']
with (out/'recipe-refinement-red.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':cmd,'exit':r.returncode,'source_hashes':hashes,'sources_unchanged':all(sha(p)==h for p,h in hashes.items())};(out/'recipe-red-result.json').write_text(json.dumps(record,indent=2)+'\n');assert r.returncode==101 and record['sources_unchanged'];assert '4 passed; 2 failed;' in (out/'recipe-refinement-red.log').read_text();print('Recipe refinement RED: four pass, two fail',flush=True)
