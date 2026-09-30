import json,pathlib,datetime,hashlib,subprocess
P=pathlib.Path('00_nucleo/diagnosticos/correcao-demanda-regional-2026-09-28')
checks=[]
def check(name,ok,detail=None):
 checks.append({'check':name,'passed':bool(ok),'detail':detail})
def load(p):return json.loads(p.read_text())
def obs(run):
 return {k:run.get(k) for k in ['exit_code','pages','page_count','line_count','text']}
before={x['case']:x for x in load(P/'before-pdf/resultados.json')};after={x['case']:x for x in load(P/'after-pdf/resultados.json')}
for k,a in after.items():
 b=before[k]
 check(k+': input unchanged',a['source_sha256']==b['source_sha256'])
 check(k+': oracle observables unchanged',obs(a['runs']['vanilla'])==obs(b['runs']['vanilla']))
 for engine in ['vanilla','tekt']:
  aa=a['runs'][engine]['stderr'];bb=b['runs'][engine]['stderr']
  for folder in ['after-pdf','before-pdf']:
   aa=aa.replace(str(P/folder),'<probe-dir>');bb=bb.replace(str(P/folder),'<probe-dir>')
  check(k+': '+engine+' diagnostics unchanged',aa==bb)
 if k not in {'terminal','terminal-continuation','terminal-pagebreak','terminal-pagebreak-content','terminal-two-pagebreaks','terminal-final-parbreak','terminal-columns','terminal-numbering','terminal-forced-line'}:
  check(k+': tekt observables unchanged',obs(a['runs']['tekt'])==obs(b['runs']['tekt']))
counts={'terminal':1,'terminal-continuation':2,'terminal-pagebreak':2,'terminal-pagebreak-content':2,'terminal-two-pagebreaks':3,'terminal-final-parbreak':1,'empty':1,'empty-explicit-pagebreak':2,'terminal-odd-break':3,'terminal-forced-line':1}
for k,count in counts.items():
 a=after[k];t=a['runs']['tekt'];v=a['runs']['vanilla']
 check(k+': expected pages',t['page_count']==v['page_count']==count)
 check(k+': complete text',t['text']==v['text'])
 check(k+': text inside page viewport',all(0<=line['box']['yMin'] and line['box']['yMax']<=page['height'] for page in t['pages'] for line in page['lines']))
t=after['terminal-columns']['runs']['tekt'];check('columns: one page, previously missing Three retained',t['page_count']==1 and 'Three' in t['text'])
t=after['terminal-footnote']['runs']['tekt'];check('terminal note: no extra successor and no main/note overlap',t['page_count']==2 and 'NOTE' not in ' '.join(l['text'] for l in t['pages'][0]['lines']) and 'NOTE' in ' '.join(l['text'] for l in t['pages'][1]['lines']))
audit=[]
for file in ['resultados.json','resultados-controles.json','resultados-isolamento.json']:
 old={x['case']:x for x in load(pathlib.Path('00_nucleo/diagnosticos/auditoria-quebra-paginacao-2026-09-28')/file)}
 for a in load(P/'after-audit'/file):
  k=a['case'];b=old[k]
  check('audit '+k+': input unchanged',a['source_sha256']==b['source_sha256'])
  check('audit '+k+': oracle observables unchanged',obs(a['runs']['vanilla'])==obs(b['runs']['vanilla']))
  t=a['runs']['tekt'];v=a['runs']['vanilla'];o=b['runs']['tekt']
  changed=obs(t)!=obs(o)
  audit.append({'case':k,'changed':changed,'pages_before':o.get('page_count'),'pages_after':t.get('page_count'),'pages_vanilla':v.get('page_count'),'text_equals_vanilla':t.get('text')==v.get('text')})
  if changed:
   check('audit '+k+': corrected page count and preserved full text',t.get('page_count')==v.get('page_count') and t.get('text')==v.get('text'))
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':True,'bilateral_runs':len(after)+len(audit),'checks':checks,'failed':[x for x in checks if not x['passed']],'audit':audit,'scope_sha256':{f:hashlib.sha256(pathlib.Path(f).read_bytes()).hexdigest() for pair in load(P/'scope.json').items() for f in pair}}
(P/'acceptance.json').write_text(json.dumps(result,indent=2,ensure_ascii=False)+'\n')
print(json.dumps({'checks':len(checks),'failed':result['failed'],'bilateral_runs':result['bilateral_runs']},ensure_ascii=False));raise SystemExit(bool(result['failed']))
