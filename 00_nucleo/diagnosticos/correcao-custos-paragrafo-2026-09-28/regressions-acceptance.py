from pathlib import Path
import json,hashlib,datetime,subprocess
P=Path('00_nucleo/diagnosticos/correcao-custos-paragrafo-2026-09-28')
load=lambda f:json.loads((P/f).read_text());checks=[]
def check(name,value):checks.append({'check':name,'passed':bool(value)})
EPS=.0005
def obs(run):
 return [{'width':page['width'],'height':page['height'],'words':sorted([{'text':w['text'].replace('\u00ad','-'),'box':w['box']} for line in page['lines'] for w in line['words']],key=lambda w:(round(w['box']['yMin'],3),w['box']['xMin'],w['text']))} for page in run.get('pages',[])]
def equal(a,b):
 if isinstance(a,dict):return isinstance(b,dict) and a.keys()==b.keys() and all(equal(a[k],b[k]) for k in a)
 if isinstance(a,list):return isinstance(b,list) and len(a)==len(b) and all(equal(x,y) for x,y in zip(a,b))
 if isinstance(a,(float,int)):return isinstance(b,(float,int)) and abs(a-b)<EPS
 return a==b
before={r['case']:r for r in load('regression-hyphen-before/resultados.json')};after=load('regression-hyphen/resultados.json');agreements=[];copy_differences=[];physical_improvements=[]
for r in after:
 name=r['case'];a,b=r['runs']['tekt'],r['runs']['vanilla'];old=before[name]['runs']
 check(name+': identical input',r['source_sha256']==before[name]['source_sha256']);check(name+': stable oracle',equal(obs(b),obs(old['vanilla'])));check(name+': both compile',a['exit_code']==b['exit_code']==0)
 same=equal(obs(a),obs(b))
 if same:agreements.append(name)
 if same and not equal(obs(old['tekt']),obs(old['vanilla'])):physical_improvements.append(name)
 if name=='mixed-word':check(name+': unsupported word fallback unchanged',equal(obs(a),obs(old['tekt'])))
 else:check(name+': physical words and glyph positions agree',same)
 if '\u00ad' in b.get('text','') and '\u00ad' not in a.get('text',''):copy_differences.append(name)
checks+=load('regression-main-acceptance.json')['checks']
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest();binary=sha('target/release/typst')
for folder in ['regression-hyphen','regression-selection','regression-multi','regression-composition','regression-precedence','regression-policy','regression-lexical']:
 check(folder+': exact binary',load(folder+'/proveniencia.json')['binary_sha256']['tekt']==binary)
for row in load('regression-empty/results.json')['rows']:
 check(row['name']+': empty regression exact binary',row['tekt']['binary_sha256']==binary)
check('pinned oracle binary',sha('lab/typst-original/target/release/typst')=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea')
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'não commitada','diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'checks':checks,'failed':[c for c in checks if not c['passed']],'binary_sha256':binary,'bilateral_inputs':len(after)+140,'new_inputs':len(after),'new_geometry_agreements':len(agreements),'new_agreement_cases':agreements,'physical_improvements':physical_improvements,'copy_identity_difference_cases':copy_differences,'open':['mixed-word style/policy inside word','PDF soft hyphen identity','configurable overhang','native-nested','show-par-repr','paragraph-pages','columns and variable width','non-Latin/UAX14/decorated runs'], 'tolerance_pt':EPS,'scope_sha256':{f:sha(f) for pair in load('scope.json').items() for f in pair}}
(P/'regressions-acceptance.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');print(json.dumps({'checks':len(checks),'failed':result['failed'],'inputs':result['bilateral_inputs'],'new_agreements':len(agreements),'new_improvements':len(physical_improvements),'copy_differences':len(copy_differences)}));raise SystemExit(bool(result['failed']))
