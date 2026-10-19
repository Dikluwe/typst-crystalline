from pathlib import Path
import json,hashlib,datetime,subprocess,re
P=Path('00_nucleo/diagnosticos/correcao-hardcode-paragrafo-2026-09-28');load=lambda f:json.loads((P/f).read_text());checks=[]
def check(name,value):checks.append({'check':name,'passed':bool(value)})
EPS=.0005
def obs(run):
 return [{'width':page['width'],'height':page['height'],'words':sorted([{'text':w['text'].replace('\u00ad','-'),'box':w['box']} for line in page['lines'] for w in line['words']],key=lambda w:(round(w['box']['yMin'],3),w['box']['xMin'],w['text']))} for page in run.get('pages',[])]
def equal(a,b):
 if isinstance(a,dict):return isinstance(b,dict) and a.keys()==b.keys() and all(equal(a[k],b[k]) for k in a)
 if isinstance(a,list):return isinstance(b,list) and len(a)==len(b) and all(equal(x,y) for x,y in zip(a,b))
 if isinstance(a,(float,int)):return isinstance(b,(float,int)) and abs(a-b)<EPS
 return a==b
before={r['case']:r for r in load('before-pdf/resultados.json')};after=load('after-pdf/resultados.json');geom=[];improved=[];error_fixed=[]
for r in after:
 name=r['case'];a,b=r['runs']['tekt'],r['runs']['vanilla'];old=before[name]['runs'];check(name+': source identical',r['source_sha256']==before[name]['source_sha256']);check(name+': oracle stable',b['exit_code']==old['vanilla']['exit_code'] and equal(obs(b),obs(old['vanilla'])))
 if name.startswith('invalid'):
  check(name+': rejects wrong type',a['exit_code']==b['exit_code']==1)
  def error_block(run):
   return run['stderr'][re.search(r'(?m)^error:',run['stderr']).start():]
  ea,eb=error_block(a),error_block(b)
  check(name+': primary diagnostic matches',ea.splitlines()[0]==eb.splitlines()[0])
  check(name+': causal line/column matches',re.findall(r'\.typ:(\d+):(\d+)',ea)==re.findall(r'\.typ:(\d+):(\d+)',eb))
  check(name+': full primary error block matches',ea==eb)
  if a['exit_code']==1 and old['tekt']['exit_code']==0:error_fixed.append(name)
 else:
  check(name+': compiles',a['exit_code']==b['exit_code']==0)
  check(name+': physical glyph geometry matches',equal(obs(a),obs(b)))
  check(name+': no discarded-overhang warning',"'overhang'" not in a['stderr'])
  if equal(obs(a),obs(b)):geom.append(name)
  if equal(obs(a),obs(b)) and not equal(obs(old['tekt']),obs(old['vanilla'])):improved.append(name)
checks+=load('regressions-acceptance.json')['checks'];sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest();binary=sha('target/release/typst');check('new probes exact binary',load('after-pdf/proveniencia.json')['binary_sha256']['tekt']==binary)
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'não commitada','diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'checks':checks,'failed':[c for c in checks if not c['passed']],'binary_sha256':binary,'bilateral_inputs':len(after)+207,'new_inputs':len(after),'new_geometry_agreements':geom,'geometry_improvements':improved,'type_errors_corrected':error_fixed,'tolerance_pt':EPS,'scope_sha256':{f:sha(f) for pair in load('scope.json').items() for f in pair},'limits':['hyphen plain text is still literal dash','overhang for arbitrary punctuation and incremental fallback','custom text.costs','mixed policies inside word','preexisting UAX14/width/pagination and nested par limitations']}
(P/'acceptance.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');print(json.dumps({'checks':len(checks),'failed':result['failed'],'inputs':result['bilateral_inputs'],'new_geometry_agreements':len(geom),'geometry_improvements':len(improved),'type_errors_corrected':len(error_fixed)}));raise SystemExit(bool(result['failed']))
