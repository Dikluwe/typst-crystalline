from pathlib import Path
import datetime,gzip,hashlib,json,subprocess
D=Path(__file__).resolve().parent;R=D.parents[2]
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
b=json.load(gzip.open(D/'before.json.gz','rt'))
x={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','git_status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),'git_diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'productive_hashes':{p:sha(R/p) for p in b['productive_hashes']},'owners':{p:(R/p).read_text() for p in b['owners']},'binaries':{str(p):sha(p) for p in [R/'target/release/typst',Path('/tmp/tekt-catalog-heap-benchmark/old/typst'),R/'lab/typst-original/target/release/typst']},'logs':{}}
x['changed_productive']=[p for p,h in b['productive_hashes'].items() if x['productive_hashes'][p]!=h]
assert x['changed_productive']==[],x['changed_productive']
for n in ['infra','build','release','lint-final','tests-before','red','green','materialized','scopes','tables','query','repeat','risk-repeat','restore-build','restore-release','restore-lint']:
 p=Path('/tmp/catalog-heap-'+n+'.log')
 if p.exists():x['logs'][n]={'text':p.read_text(),'sha256':sha(p)}
x['artifacts']={p.name:sha(p) for p in D.iterdir() if p.is_file() and p.name!='final.json.gz'}
with gzip.open(D/'final.json.gz','wt') as f:json.dump(x,f,ensure_ascii=False,indent=2)
print('changed productive',x['changed_productive']);print('binaries',x['binaries'])
