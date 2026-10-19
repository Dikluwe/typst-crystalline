from pathlib import Path
import json,hashlib,subprocess,datetime,re,collections
P=Path('00_nucleo/diagnosticos/correcao-viuvas-orfas-2026-09-28');load=lambda n:json.loads((P/n).read_text());sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest()
a=load('acceptance.json');review=load('scope-review.json');v=load('validation-final-state.json');assert not a['failed'] and not review['unexpected']
assert review['source_sha256']==v['source_sha256'];assert all(sha(f)==h for f,h in review['source_sha256'].items());assert all(sha(f)==h for f,h in a['scope_sha256'].items())
assert load('after-pdf/proveniencia.json')['source_sha256']==review['source_sha256']
assert a['binary_sha256']==load('after-pdf/proveniencia.json')['binary_sha256']
assert a['binary_sha256']['tekt']==sha('target/release/typst');assert a['binary_sha256']['vanilla']==sha('lab/typst-original/target/release/typst')=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
tests={}
for name in ['core','infra']:
 s=(P/(name+'-final.log')).read_text();m=re.search(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored',s);assert m and int(m[2])==0;tests[name]={'passed':int(m[1]),'failed':0,'ignored':int(m[3])}
for n in ['build-final.log','release-final.log']:assert 'Finished `' in (P/n).read_text()
levels=collections.Counter(x.get('level') for run in load('lint-final.sarif')['runs'] for x in run.get('results',[]));assert not levels['error']
scope=load('scope.json');bad=[]
for run in load('lint-final.sarif')['runs']:
 for r in run.get('results',[]):
  paths=[z.get('physicalLocation',{}).get('artifactLocation',{}).get('uri','').removeprefix('./') for z in r.get('locations',[])]
  if r['ruleId'] in ['V5','V6','V15','V26'] and any(f in scope or f in scope.values() for f in paths):bad.append(r)
assert not bad
artifacts=['initial.json','acceptance.json','scope-review.json','review.diff','scope.json','scoped-seals.json','validation-final-state.json','red.log','red-state.json','first-green.log','fixture-observation.log','fixture-red.log','fixture-red-state.json','green.log','core-first.log','core-refinement.log','core-final.log','infra-final.log','build-final.log','release-final.log','lint-final.sarif','lint-final.stderr','lint-seal.sarif','seal-canonicalization.md','first-round/acceptance.json','second-round/acceptance.json','before-pdf/proveniencia.json','before-extra/proveniencia.json','after-pdf/proveniencia.json','after-pdf/resultados.json','after-extra/proveniencia.json','after-extra/resultados.json','additional-scopes/proveniencia.json','additional-scopes/resultados.json']
for folder in ['regression-costs','regression-runt','regression-overhang','regression-hyphen','regression-selection','regression-multi','regression-composition','regression-precedence','regression-policy','regression-lexical']:
 artifacts.extend([folder+'/proveniencia.json',folder+'/resultados.json'])
artifacts.append('regression-empty/results.json')
r={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'não commitada','status':subprocess.check_output(['git','status','--porcelain=v1','-uall'],text=True),'diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'binary_sha256':a['binary_sha256'],'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','vanilla_tree':'6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4','vanilla_recipe':'P1489','source_sha256':review['source_sha256'],'source_unchanged_since_final_checks':True,'changed_productive_L0_files':review['changed'],'unexpected_changes':review['unexpected'],'tests':tests,'build':'passed','release':'passed','lint':{'errors':0,'warnings':levels['warning'],'notes':levels['note'],'V5_V6_V15_V26_in_scope':[]},'bilateral_inputs':a['bilateral_inputs'],'new_inputs':a['new_inputs'],'regression_inputs':a['regression_inputs'],'checks':len(a['checks']),'failed_checks':0,'geometry_agreements_certified_new_controls':len(a['new_geometry_agreements']),'geometry_improvements_with_baseline':a['geometry_improvements'],'regression_geometry_newly_matching':a['regression_geometry_newly_matching'],'additional_inputs_added_after_implementation':a['additional_inputs_added_after_implementation'],'preserved_fallback_controls':a['fallback_controls'],'regression_open_fallbacks':a['regression_open_fallbacks'],'artifacts_sha256':{n:sha(P/n) for n in artifacts}}
(P/'final-receipt.json').write_text(json.dumps(r,ensure_ascii=False,indent=2)+'\n');print(json.dumps({k:r[k] for k in ['utc','binary_sha256','tests','lint','bilateral_inputs','new_inputs','checks','geometry_agreements_certified_new_controls','geometry_improvements_with_baseline','regression_geometry_newly_matching']}))
