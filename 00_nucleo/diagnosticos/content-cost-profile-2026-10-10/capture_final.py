"""Verify exact preservation of productive Rust sources and profile binaries."""
from pathlib import Path
import hashlib,json,gzip,datetime,subprocess
D=Path(__file__).resolve().parent;R=D.parents[2];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
before=json.load(gzip.open(D/'before.json.gz','rt'))
current={str(p.relative_to(R)):sha(p) for layer in ['01_core','02_shell','03_infra','04_wiring'] for p in (R/layer).rglob('*.rs')}
assert current==before['productive_sources']
assert {p:sha(R/p) for p in before['binaries']}==before['binaries']
q=json.load(gzip.open(D/'qualification.json.gz','rt'));assert len(q['across'])==14
assert all(v['same_text'] and v['same_words'] and v['same_page_size'] for v in q['across'].values())
s=json.load(gzip.open(D/'resolution-calls.json.gz','rt'));assert len(s['samples'])==2 and all(r['exit_code']==0 for r in s['samples'])
assert all(r['counts']['font_select']==48019 and r['counts']['primary_resolution']==6401 for r in s['samples'])
receipts=[p for p in D.iterdir() if p.suffix in ('.gz','.py','.c','.so','.json') and p.name!='final-verdict.json']
state={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'unchanged_productive_sources_count':len(current),'productive_sources':current,'binaries':before['binaries'],'artifact_hashes':{p.name:sha(p) for p in receipts},'decision':'next hypothesis: reuse repeated font-chain resolution; no productive implementation in this task','controls':{'count':14,'same_text_words_pages_sizes':True,'bbox_max_delta_pt':max(v['bbox_max_delta_pt'] for v in q['across'].values()),'raster_equal_count':sum(v['same_raster'] for v in q['across'].values())},'lint_exit_code':0,'lint_log':Path('/tmp/cost-profile-lint.log').read_text(),'scope':'warnings preexisting; profiler/debugger times never used as production speed metrics'}
with gzip.open(D/'final-validation.json.gz','wt') as f:json.dump(state,f,ensure_ascii=False,indent=2)
(D/'final-verdict.json').write_text(json.dumps({k:state[k] for k in ['utc','unchanged_productive_sources_count','binaries','decision','controls','lint_exit_code']},ensure_ascii=False,indent=2)+'\n')
print('productive sources preserved:',len(current),'two binary identities preserved, 14 controls recorded')
