from pathlib import Path
import json,hashlib,datetime,subprocess
P=Path('00_nucleo/diagnosticos/correcao-custos-paragrafo-2026-09-28');load=lambda f:json.loads((P/f).read_text());checks=[]
def check(name,value):checks.append({'check':name,'passed':bool(value)})
EPS=.0005
def obs(run):
 out=[]
 for page in run.get('pages',[]):
  words=[{'text':w['text'].replace('\u00ad','-'),'box':w['box']} for l in page['lines'] for w in l['words']]
  words.sort(key=lambda w:(round(w['box']['yMin'],3),w['box']['xMin'],w['text']))
  out.append({'width':page['width'],'height':page['height'],'words':words})
 return out
def equal(a,b):
 if isinstance(a,dict):return isinstance(b,dict) and a.keys()==b.keys() and all(equal(a[k],b[k]) for k in a)
 if isinstance(a,list):return isinstance(b,list) and len(a)==len(b) and all(equal(x,y) for x,y in zip(a,b))
 if isinstance(a,(float,int)):return isinstance(b,(float,int)) and abs(a-b)<EPS
 return a==b
new=load('regression-selection/resultados.json');before={r['case']:r for r in json.loads(Path('00_nucleo/diagnosticos/correcao-seletores-paragrafo-2026-09-28/before-pdf/resultados.json').read_text())};open_new={'columns':'Variable/column width is outside the prepared subset; current geometry remains different.'}
for r in new:
 name=r['case'];a=r['runs']['tekt'];b=r['runs']['vanilla'];check(name+': same source',r['source_sha256']==before[name]['source_sha256']);check(name+': stable oracle',equal(obs(b),obs(before[name]['runs']['vanilla'])));check(name+': compiles',a['exit_code']==b['exit_code']==0)
 if name not in open_new:check(name+': physical words, styles through glyph boxes, geometry',equal(obs(a),obs(b)))
 else:check(name+': bounded fallback preserved',equal(obs(a),obs(before[name]['runs']['tekt'])))
for r in load('regression-multi/resultados.json'):
 a=r['runs']['tekt'];b=r['runs']['vanilla'];check(r['case']+': each paragraph agrees',a['exit_code']==b['exit_code']==0 and equal(obs(a),obs(b)))
opened={'native-nested','show-par-repr','paragraph-pages'}
previous={r['case']:r for r in json.loads(Path('00_nucleo/diagnosticos/correcao-composicao-paragrafos-2026-09-28/after-pdf/resultados.json').read_text())}
for r in load('regression-composition/resultados.json'):
 a=r['runs']['tekt'];b=r['runs']['vanilla'];name=r['case']
 if name in opened:
  old=previous[name]['runs'];check(name+': explicit open remains measured',a['exit_code']==old['tekt']['exit_code'] and b['exit_code']==old['vanilla']['exit_code'] and equal(obs(a),obs(old['tekt'])) and equal(obs(b),obs(old['vanilla'])))
 else:
  check(name+': regression compiles',a['exit_code']==b['exit_code']==0);check(name+': regression physical geometry agrees',equal(obs(a),obs(b)))
for r in load('regression-precedence/resultados.json'):
 a=r['runs']['tekt'];b=r['runs']['vanilla'];check(r['case']+': lexical precedence retained',a['exit_code']==b['exit_code']==0 and equal(obs(a),obs(b)))
for r in load('regression-empty/results.json')['rows']:
 check(r['name']+': empty paragraph compiles',r['tekt']['exit']==r['vanilla']['exit']==0)
 def geo(run):return [{'height':float(p['height']),'words':[{'text':w['text'],'box':{k:float(v) for k,v in w.items() if k!='text'}} for w in p['words']]} for p in run.get('pages',[])]
 check(r['name']+': empty paragraph geometry agrees',equal(geo(r['tekt']),geo(r['vanilla'])))
reg=load('regression-acceptance.json');checks+=reg['checks']
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest();binary=sha('target/release/typst')
for folder in ['regression-selection','regression-multi','regression-composition','regression-precedence','regression-policy','regression-lexical']:
 check(folder+': exact release binary',load(folder+'/proveniencia.json')['binary_sha256']['tekt']==binary)
check('oracle identity',sha('lab/typst-original/target/release/typst')=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea')
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'não commitada','diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'checks':checks,'failed':[c for c in checks if not c['passed']],'binary_sha256':binary,'bilateral_inputs':len(new)+1+34+8+7+reg['bilateral_inputs'],'new_inputs':len(new)+1,'new_geometry_agreements':len(new)-len(open_new)+1,'open_new':open_new,'controlled_linebreak':'The linebreak control agrees but does not certify arbitrary mandatory-break interactions.','preexisting_open':sorted(opened),'tolerance_pt':EPS,'scope_sha256':{f:sha(f) for pair in load('scope.json').items() for f in pair}}
(P/'regression-main-acceptance.json').write_text(json.dumps(result,indent=2,ensure_ascii=False)+'\n');print(json.dumps({'checks':len(checks),'failed':result['failed'],'inputs':result['bilateral_inputs'],'new_agreements':result['new_geometry_agreements']},ensure_ascii=False));raise SystemExit(bool(result['failed']))
