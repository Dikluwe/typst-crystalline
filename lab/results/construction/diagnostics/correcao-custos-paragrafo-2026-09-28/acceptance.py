from pathlib import Path
import json,hashlib,datetime,subprocess,re
P=Path('00_nucleo/diagnosticos/correcao-custos-paragrafo-2026-09-28');load=lambda f:json.loads((P/f).read_text());checks=[]
def check(name,value):checks.append({'check':name,'passed':bool(value)})
EPS=.0005
def obs(run):
 return [{'width':page['width'],'height':page['height'],'words':sorted([{'text':w['text'].replace('\u00ad','-'),'box':w['box']} for line in page['lines'] for w in line['words']],key=lambda w:(round(w['box']['yMin'],3),w['box']['xMin'],w['text']))} for page in run.get('pages',[])]
def equal(a,b):
 if isinstance(a,dict):return isinstance(b,dict) and a.keys()==b.keys() and all(equal(a[k],b[k]) for k in a)
 if isinstance(a,list):return isinstance(b,list) and len(a)==len(b) and all(equal(x,y) for x,y in zip(a,b))
 if isinstance(a,(float,int)):return isinstance(b,(float,int)) and abs(a-b)<EPS
 return a==b
before={r['case']:r for r in load('before-pdf/resultados.json')};after=load('after-pdf/resultados.json');geom=[];improved=[];errors=[]
for r in after:
 name=r['case'];a,b=r['runs']['tekt'],r['runs']['vanilla'];old=before[name]['runs'];check(name+': same source',r['source_sha256']==before[name]['source_sha256']);check(name+': stable oracle',b['exit_code']==old['vanilla']['exit_code'] and equal(obs(b),obs(old['vanilla'])))
 if name.startswith('invalid'):
  check(name+': wrong type/key rejected',a['exit_code']==b['exit_code']==1)
  def error_block(run):
   m=re.search(r'(?m)^error:',run['stderr']);return run['stderr'][m.start():] if m else ''
  ea,eb=error_block(a),error_block(b)
  check(name+': complete error block matches',bool(ea) and ea==eb)
  check(name+': causal line/column matches',re.findall(r'\.typ:(\d+):(\d+)',ea)==re.findall(r'\.typ:(\d+):(\d+)',eb))
  if a['exit_code']==1 and old['tekt']['exit_code']==0:errors.append(name)
 else:
  check(name+': both compile',a['exit_code']==b['exit_code']==0)
  same=equal(obs(a),obs(b));check(name+': physical words and positions match',same)
  check(name+': costs not discarded',"propriedade 'costs' ainda não suportada" not in a['stderr'])
  if same:geom.append(name)
  if same and not equal(obs(old['tekt']),obs(old['vanilla'])):improved.append(name)
witnesses=load('runt-witness/resultados.json')
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest()
for r in witnesses:
 name=r['case'];a,b=r['runs']['tekt'],r['runs']['vanilla']
 check(name+': both compile',a['exit_code']==b['exit_code']==0)
 same=equal(obs(a),obs(b));check(name+': physical words and positions match',same)
 check(name+': actual input hash',sha(P/'runt-witness'/(name+'.typ'))==r['source_sha256'])
 if same:geom.append(name)
indexed={r['case']:r for r in after}
for field in ['hyphenation','runt']:
 pairs=[]
 for width in [100,140,180,240]:
  for justify in ([True] if field=='hyphenation' else [False,True]):
   low=indexed[f'{field}-{width}-0%-{justify}']['runs'];high=indexed[f'{field}-{width}-10000%-{justify}']['runs']
   pairs.append(not equal(obs(low['tekt']),obs(high['tekt'])) and not equal(obs(low['vanilla']),obs(high['vanilla'])))
 if field=='runt':
  low,high=[r['runs'] for r in witnesses]
  pairs.append(all(not equal(obs(low[k]),obs(high[k])) for k in ['tekt','vanilla']))
  check('runt witness: text preserved at both costs',len({r['text'] for case in witnesses for r in case['runs'].values()})==1)
 check(field+': configuration changes a measured choice in both compilers',any(pairs))
checks+=load('regression-overhang-acceptance.json')['checks'];sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest();binary=sha('target/release/typst');check('new probes exact binary',load('after-pdf/proveniencia.json')['binary_sha256']['tekt']==binary)
check('runt witness exact binaries',load('runt-witness/proveniencia.json')['binary_sha256']==load('after-pdf/proveniencia.json')['binary_sha256'])
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'não commitada','diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'checks':checks,'failed':[c for c in checks if not c['passed']],'binary_sha256':binary,'bilateral_inputs':len(after)+len(witnesses)+227,'new_inputs':len(after)+len(witnesses),'additional_witnesses':{'cases':[r['case'] for r in witnesses],'added_after_implementation':True,'pre_correction_measurement':None,'search':'runt-search.json'},'new_geometry_agreements':geom,'geometry_improvements':improved,'validation_errors_corrected':errors,'tolerance_pt':EPS,'scope_sha256':{f:sha(f) for pair in load('scope.json').items() for f in pair},'limits':['widow/orphan pagination influence','soft hyphen PDF plain-text identity','nonfinite cost scenarios not certified','preexisting Unicode/variable-width and nested-par limitations']}
(P/'acceptance.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');print(json.dumps({'checks':len(checks),'failed':result['failed'],'inputs':result['bilateral_inputs'],'new_geometry_agreements':len(geom),'geometry_improvements':len(improved),'validation_errors_corrected':len(errors)}));raise SystemExit(bool(result['failed']))
