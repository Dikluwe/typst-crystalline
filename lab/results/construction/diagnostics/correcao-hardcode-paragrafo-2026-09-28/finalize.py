from pathlib import Path
import json,hashlib,datetime,subprocess,collections
P=Path('00_nucleo/diagnosticos/correcao-hardcode-paragrafo-2026-09-28');sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest();load=lambda f:json.loads((P/f).read_text())
assert '6350 passed; 0 failed; 2 ignored' in (P/'core-final.log').read_text()
assert '1127 passed; 0 failed' in (P/'infra-final.log').read_text()
assert 'Finished `dev`' in (P/'build-final.log').read_text()
assert 'Finished `release`' in (P/'release-final.log').read_text()
lint=load('lint-final.sarif');levels=collections.Counter(x.get('level','warning') for r in lint['runs'] for x in r.get('results',[]));assert levels['error']==0
accept=load('acceptance.json');assert not accept['failed'];binary=sha('target/release/typst');assert binary==accept['binary_sha256']
current={}
for root in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts']:
 for f in Path(root).rglob('*'):
  if f.is_file() and f.suffix in ['.rs','.md','.toml'] and 'target' not in f.parts:current[str(f)]=sha(f)
assert current==load('after-pdf/proveniencia.json')['source_sha256']
review=load('scope-review.json');assert current==review['source_sha256'];assert not review['unexpected'];assert all(sha(f)==h for f,h in accept['scope_sha256'].items())
receipt={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'não commitada','status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'binaries':{'tekt':binary,'vanilla':sha('lab/typst-original/target/release/typst')},'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','vanilla_tree':'6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4','source_sha256':current,'unchanged_after_probes':True,'artifacts_sha256':{f:sha(P/f) for f in ['acceptance.py','acceptance.json','regressions-acceptance.json','scope-review.json','review.diff','scoped-seals.json','hardcode-inventory.json','red.log','red-certified.log','first-green.log','core-final.log','infra-final.log','build-final.log','release-final.log','lint-final.sarif']},'checks':len(accept['checks']),'inputs':accept['bilateral_inputs'],'new_geometry_agreements':len(accept['new_geometry_agreements']),'new_geometry_improvements':len(accept['geometry_improvements']),'type_errors_corrected':len(accept['type_errors_corrected']),'tests':{'core_passed':6350,'core_ignored':2,'infra_passed':1127},'lint_levels':dict(levels),'changed_productive_L0_files':len(review['changed']),'unexpected_scope':review['unexpected'],'limits':accept['limits']}
(P/'finalreceipt.json').write_text(json.dumps(receipt,ensure_ascii=False,indent=2)+'\n');print(json.dumps({k:receipt[k] for k in ['utc','binaries','checks','inputs','new_geometry_agreements','new_geometry_improvements','type_errors_corrected','lint_levels','changed_productive_L0_files']},ensure_ascii=False))
