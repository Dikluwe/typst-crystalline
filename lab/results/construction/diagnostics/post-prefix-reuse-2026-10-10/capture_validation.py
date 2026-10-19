"""Capture final productive source, hashes and test/build/lint evidence."""
from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,difflib
D=Path(__file__).resolve().parent;R=D.parents[2]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
old=json.load(gzip.open(D/'before.json.gz','rt'))
state={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','git_status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),'git_diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'owners':{p:(R/p).read_text() for pair in json.loads((D/'owners.json').read_text()) for p in pair},'productive_hashes':{p:sha(R/p) for p in old['productive_hashes']},'binaries':{k:sha(p) for k,p in [('before','/tmp/tekt-post-prefix-benchmark/old/typst'),('after',R/'target/release/typst'),('vanilla',R/'lab/typst-original/target/release/typst')]},'logs':{},'commands':['cargo test -p typst-infra fontdb::selective::tests --lib','cargo test -p typst-infra --lib','cargo build','cargo build --release','crystalline-lint .']}
for name in ['green-final','infra-final','build-final','release-final','lint-final','corpus-final','qual-tables-final','qual-scopes-final','qual-query-final','allocation-green-final']:
 state['logs'][name]=Path('/tmp/post-prefix-'+name+'.log').read_text()
state['changed_productive']=[p for p,h in old['productive_hashes'].items() if state['productive_hashes'][p]!=h]
assert state['changed_productive']==['03_infra/src/fontdb/selective.rs']
assert state['binaries']['after']==sha('/tmp/tekt-post-prefix-benchmark/new/typst')
state['output_verification']={}
for name in ['qualification-tables','qualification-scopes']:
 data=json.load(gzip.open(D/(name+'.json.gz'),'rt'))
 assert data['binaries']['after']==state['binaries']['after']
 assert all(c['same_text'] and c['same_words'] and c['same_page_size'] and c['bbox_max_delta_pt']==0 and c['same_raster'] for c in data['across'].values())
 state['output_verification'][name]={'cases':len(data['across']),'all_equal':True,'receipt_sha256':sha(D/(name+'.json.gz'))}
for name in ['corpus-validation','allocation-equal-red','allocation-equal-green','qualification-query']:
 state['evidence_'+name]=sha(D/(name+'.json.gz'))
with gzip.open(D/'final-validation.json.gz','wt') as f:json.dump(state,f)
p='03_infra/src/fontdb/selective.rs'
(D/'owner-diff.patch').write_text(''.join(difflib.unified_diff(old['owners'][p].splitlines(True),state['owners'][p].splitlines(True),fromfile='before/'+p,tofile='after/'+p)))
print(state['binaries'],state['changed_productive'])
