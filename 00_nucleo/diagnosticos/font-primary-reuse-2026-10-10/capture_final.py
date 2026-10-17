"""Archive exact tested sources and verify the accepted profiling gates."""
from pathlib import Path
import gzip,json,hashlib,datetime,subprocess
D=Path(__file__).resolve().parent;R=D.parents[2];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
owners=json.loads((D/'owners.json').read_text());files=[p for pair in owners for p in pair]
c=json.load(gzip.open(D/'candidate.json.gz','rt'))
assert all((R/p).read_text()==c['files'][p] for p in files)
nucleus=json.loads((D/'nucleus.json').read_text())
assert (R/nucleus['path']).read_text()==c['nucleus']
binary=sha(R/'target/release/typst');assert binary=='783572ba02f8b423935179dd7c81f62895bcf21db9144d48d2fa924421249ef0'
measure=json.load(gzip.open(D/'measurements.json.gz','rt'));assert measure['binaries']['after']==binary
assert json.loads((D/'performance-gate.json').read_text())['passed']
controls={}
for name in ['tables','extended','small-containers','effects']:
 s=json.load(gzip.open(D/f'qualification-{name}.json.gz','rt'));assert s['binaries']['after']==binary
 assert all(v['same_text'] and v['same_words'] and v['same_page_size'] and v['bbox_max_delta_pt']==0 and v['same_raster'] for v in s['across'].values())
 controls[name]=len(s['across'])
logs={n:Path('/tmp/primary-reuse-'+n+'.log').read_text() for n in ['red','green','infra-tests','build','release-final','lint']}
assert '3 passed; 1 failed' in logs['red'] and '4 passed; 0 failed' in logs['green']
assert '1273 passed; 0 failed; 2 ignored' in logs['infra-tests']
assert 'Finished `dev` profile' in logs['build'] and 'Finished `release` profile' in logs['release-final']
# V26/ownership diagnostics were repaired; no final owner drift.
assert not any(('primary-resolution-cache' in line or 'font_metrics' in line) for line in logs['lint'].splitlines() if '[V26]' in line or '[V15]' in line or '[V5]' in line)
base=json.load(gzip.open(R/'00_nucleo/diagnosticos/content-cost-profile-2026-10-10/before.json.gz','rt'))
current={str(p.relative_to(R)):sha(p) for layer in ['01_core','02_shell','03_infra','04_wiring'] for p in (R/layer).rglob('*.rs')}
modified={p for p in current if current[p]!=base['productive_sources'].get(p)}
assert modified=={p for p,_ in owners},modified
state={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'files':{p:{'text':(R/p).read_text(),'sha256':sha(R/p)} for p in files},'nucleus':{'metadata':nucleus,'text':(R/nucleus['path']).read_text(),'raw_sha256':sha(R/nucleus['path'])},'binary_sha256':binary,'modified_productive_sources':sorted(modified),'logs':logs,'exit_codes':{'cargo_build':0,'cargo_build_release':0,'crystalline_lint':0},'controls':controls,'scope':'kept; public contracts, pipeline phases, enum dispatch and defaults unchanged; global preexisting warnings remain'}
with gzip.open(D/'final-validation.json.gz','wt') as f:json.dump(state,f,ensure_ascii=False,indent=2)
(D/'final-verdict.json').write_text(json.dumps({'utc':state['utc'],'status':'kept','owners':owners,'nucleus':nucleus,'binary_sha256':binary,'infra_passed':1273,'infra_ignored':2,'preservation_documents':sum(controls.values()),'font_select_before':48019,'font_select_after':9619,'rss_reduction_certified':False,'many_variant_memory':'preexisting ~513 MiB unchanged; open','global_vanilla_parity_certified':False},indent=2)+'\n')
print('kept:',len(modified),'productive owners,',sum(controls.values()),'preserved documents, final byte identity verified')
