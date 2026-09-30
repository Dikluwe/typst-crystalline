from pathlib import Path
import json,hashlib,re,datetime,subprocess
P=Path('00_nucleo/diagnosticos/correcao-viuvas-orfas-2026-09-28');OLD=Path('00_nucleo/diagnosticos/correcao-custos-paragrafo-2026-09-28');load=lambda f:json.loads(Path(f).read_text());sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest();checks=[];EPS=.0005
check=lambda name,v:checks.append({'check':name,'passed':bool(v)})
def obs(run):
 return [{'width':page['width'],'height':page['height'],'words':sorted([{'text':w['text'].replace('\u00ad','-'),'box':w['box']} for line in page['lines'] for w in line['words']],key=lambda w:(round(w['box']['yMin'],3),w['box']['xMin'],w['text']))} for page in run.get('pages',[])]
def equal(a,b):
 if isinstance(a,dict):return isinstance(b,dict) and a.keys()==b.keys() and all(equal(a[k],b[k]) for k in a)
 if isinstance(a,list):return isinstance(b,list) and len(a)==len(b) and all(equal(x,y) for x,y in zip(a,b))
 if isinstance(a,(float,int)):return isinstance(b,(float,int)) and abs(a-b)<EPS
 return a==b
def err(run):
 s=run.get('stderr','');m=re.search(r'(?m)^error:',s)
 return re.sub(r'(?m)(┌─ |--> )[^\n]*?\.typ:',r'\1<input>.typ:',s[m.start():]) if m else ''
changed=[];improved=[];agreements=[];fallback=[];all_new=[];regressions=0
for before,after in [('before-pdf','after-pdf'),('before-extra','after-extra')]:
 old={r['case']:r for r in load(P/before/'resultados.json')};current=load(P/after/'resultados.json');all_new+=current
 for r in current:
  name=r['case'];a,b=r['runs']['tekt'],r['runs']['vanilla'];prev=old[name]['runs'];check(name+': same input',r['source_sha256']==old[name]['source_sha256']);check(name+': stable oracle',b['exit_code']==prev['vanilla']['exit_code'] and equal(obs(b),obs(prev['vanilla'])))
  check(name+': both compile',a['exit_code']==b['exit_code']==0)
  same=equal(obs(a),obs(b));preserved=equal(obs(a),obs(prev['tekt']))
  if name in ['footnote','columns','tracking','mixed-size','linebreak']:
   check(name+': fallback preserved',preserved);fallback.append(name)
  else:
   check(name+': complete word positions and pages match',same)
   if same:agreements.append(name)
   if same and not equal(obs(prev['tekt']),obs(prev['vanilla'])):improved.append(name)
  if not preserved:changed.append(name)
  check(name+': warning no longer claims absence of all support','influência de widow/orphan na paginação ainda não implementada' not in a['stderr'])
additional=load(P/'additional-scopes/resultados.json')
for r in additional:
 name=r['case'];a,b=r['runs']['tekt'],r['runs']['vanilla']
 check(name+': both compile',a['exit_code']==b['exit_code']==0)
 check(name+': exact additional input hash',sha(P/'additional-scopes'/(name+'.typ'))==r['source_sha256'])
 same=equal(obs(a),obs(b));check(name+': full scoped geometry matches',same)
 if same:agreements.append(name)
all_new+=additional
index={r['case']:r for r in all_new}
for lane in ['tekt','vanilla']:
 check(lane+': widow changes 3+1 into 2+2',not equal(obs(index['page-65-0%-0%-False']['runs'][lane]),obs(index['page-65-100%-0%-False']['runs'][lane])))
 check(lane+': all positive ratios enable same protection',equal(obs(index['tiny-positive']['runs'][lane]),obs(index['large-positive']['runs'][lane])) and equal(obs(index['default']['runs'][lane]),obs(index['tiny-positive']['runs'][lane])))
 check(lane+': negative disables protection',equal(obs(index['negative']['runs'][lane]),obs(index['page-65-0%-0%-False']['runs'][lane])))
 check(lane+': three-line protection changes partition',not equal(obs(index['three-65-0%-0%']['runs'][lane]),obs(index['three-65-100%-100%']['runs'][lane])))
reg_closed=[];reg_fallback=[]
for folder,oldfolder in [('regression-costs','after-pdf'),('regression-runt','runt-witness')]+[(s,s) for s in ['regression-overhang','regression-hyphen','regression-selection','regression-multi','regression-composition','regression-precedence','regression-policy','regression-lexical']]:
 old={r['case']:r for r in load(OLD/oldfolder/'resultados.json')};current=load(P/folder/'resultados.json');regressions+=len(current)
 for r in current:
  name=folder+'/'+r['case'];a,b=r['runs']['tekt'],r['runs']['vanilla'];prev=old[r['case']]['runs'];check(name+': same input',r['source_sha256']==old[r['case']]['source_sha256']);check(name+': stable oracle',b['exit_code']==prev['vanilla']['exit_code'] and equal(obs(b),obs(prev['vanilla'])))
  check(name+': compile status preserved',a['exit_code']==prev['tekt']['exit_code'])
  if a['exit_code']!=0:
   check(name+': full causal error preserved',bool(err(a)) and err(a)==err(prev['tekt']))
   continue
  same=equal(obs(a),obs(b));old_same=equal(obs(prev['tekt']),obs(prev['vanilla']));preserved=equal(obs(a),obs(prev['tekt']))
  if old_same:check(name+': previous language parity preserved',same)
  elif same:check(name+': formerly open geometry now matches',same);reg_closed.append(name)
  else:check(name+': previously open fallback preserved',preserved);reg_fallback.append(name)
  # Source warnings may accurately change; causal errors must not.
  check(name+': no new error diagnostic',not err(a))
# Separate historical empty-par observer, preserving complete height and word boxes.
def empty_obs(run):
 return [{'height':float(p['height']),'words':[dict(text=w['text'],**{k:float(v) for k,v in w.items() if k!='text'}) for w in p['words']]} for p in run.get('pages',[])]
old={r['name']:r for r in load(OLD/'regression-empty/results.json')['rows']};current=load(P/'regression-empty/results.json')['rows'];regressions+=len(current)
for r in current:
 name='empty/'+r['name'];check(name+': same source',r['source']==old[r['name']]['source'])
 check(name+': exact complete input hash',sha(P/'regression-empty'/(r['name']+'.typ'))==sha(OLD/'regression-empty'/(r['name']+'.typ')))
 for lane in ['tekt','vanilla']:
  check(name+': '+lane+' empty geometry preserved',r[lane]['exit']==old[r['name']][lane]['exit'] and equal(empty_obs(r[lane]),empty_obs(old[r['name']][lane])))
 check(name+': both empty geometries match',equal(empty_obs(r['tekt']),empty_obs(r['vanilla'])))
binary={k:sha(f) for k,f in {'tekt':'target/release/typst','vanilla':'lab/typst-original/target/release/typst'}.items()};sources=load(P/'scope-review.json')['source_sha256']
for folder in ['after-pdf','after-extra','additional-scopes','regression-costs','regression-runt','regression-overhang','regression-hyphen','regression-selection','regression-multi','regression-composition','regression-precedence','regression-policy','regression-lexical']:
 prov=load(P/folder/'proveniencia.json');check(folder+': exact binaries',prov['binary_sha256']==binary);check(folder+': identical complete source snapshot',prov['source_sha256']==sources)
check('empty probes exact binaries',all(r[lane]['binary_sha256']==binary[lane] for r in current for lane in ['tekt','vanilla']))
check('source unchanged since probes',all(sha(f)==h for f,h in sources.items()));check('scope review contains no unexpected change',not load(P/'scope-review.json')['unexpected'])
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'não commitada','diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'checks':checks,'failed':[c for c in checks if not c['passed']],'binary_sha256':binary,'new_inputs':len(all_new),'additional_inputs_added_after_implementation':{'cases':[r['case'] for r in additional],'pre_correction_measurement':None},'regression_inputs':regressions,'bilateral_inputs':len(all_new)+regressions,'new_geometry_agreements':agreements,'geometry_improvements':improved,'changed_new_cases':changed,'fallback_controls':fallback,'regression_geometry_newly_matching':reg_closed,'regression_open_fallbacks':reg_fallback,'tolerance_pt':EPS,'scope_sha256':{f:sha(f) for pair in load(P/'scope.json').items() for f in pair}}
(P/'acceptance.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n');print(json.dumps({k:len(result[k]) if isinstance(result[k],list) else result[k] for k in ['checks','failed','bilateral_inputs','new_geometry_agreements','geometry_improvements','regression_geometry_newly_matching']}));print(result['failed']);raise SystemExit(bool(result['failed']))
