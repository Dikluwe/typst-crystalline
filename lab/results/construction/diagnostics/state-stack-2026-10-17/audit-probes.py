from pathlib import Path
import gzip,json,datetime
p=Path(__file__).resolve().parent
pending={
 ('main','box-body-contract-pending','pdf'):'Box.body perde a presença no carrier; contrato separado pendente.',
 ('extra','query-equation-alias','pdf'):'Equation aninhada é duplicada no snapshot; falha também no baseline.',
 ('targets','query-equation-direct','html'):'Exportação matemática HTML não implementada.',
 ('targets','counter-equation-direct','html'):'Exportação matemática HTML não implementada.',
 ('targets','heading-query-callback','html'):'Serialização de Ref HTML não implementada.',
 ('targets','heading-local','html'):'Serialização de Ref HTML não implementada.',
 ('state','legacy-display','html'):'Display legado não realizado no pipeline HTML; baseline também falha.',
}
negative={
 'main':{'fake-page-counter','state-func-panic','heading-supplement-panic','non-element-selector','nonlocatable-counter'},
 'extra':{'empty-text-field-negative','selector-nonlocatable-negative','selector-partial-negative','counter-equation-homonym-negative','heading-second-callback-error'},
 'targets':{'state-future-error','heading-panic','heading-context-absent'},
}
rows=[];unexpected=[]
for group,name in [('main','complete-probe'),('extra','extra-probe'),('state','state-extended'),('targets','target-controls')]:
 d=json.loads(gzip.decompress((p/(name+'.json.gz')).read_bytes()))
 for c in d['cases']:
  label=c['label'];target=c.get('target','pdf');key=(group,label,target);r=c['results'];t=r['tekt'];v=r['vanilla'];b=r.get('before',r.get('tekt-before'));extension=c.get('extension',False)
  expected=c['expected_exit'] if group=='state' else int(label in negative[group])
  item={'group':group,'label':label,'target':target,'extension':extension,'source_sha256':c['source_sha256'],'expected_exit':expected,'before_exit':b['exit'] if b else None,'tekt_exit':t['exit'],'vanilla_exit':v['exit'],'pending':pending.get(key)}
  if item['pending']:
   # Existing failures remain visible and must not turn into different defects.
   if not b or b['exit']!=1 or t['exit']!=1:unexpected.append({'case':key,'reason':'pending baseline changed'})
  elif t['exit']!=expected or (not extension and v['exit']!=expected):unexpected.append({'case':key,'reason':'unexpected exit','item':item})
  if t['exit']==v['exit']==1 and not extension:
   item['tekt_message']=t['stderr'].splitlines()[0];item['vanilla_message']=v['stderr'].splitlines()[0];item['message_matches']=item['tekt_message']==item['vanilla_message']
  if group=='state' and t['exit']==0:
   expected_text=c.get('expected_text') or 'A'
   if t['text']!=expected_text:unexpected.append({'case':key,'reason':'wrong output','expected':expected_text,'actual':t['text']})
  if group=='state' and label in ['future-error','error-before-set']:
   marker='FUTURE' if label=='future-error' else 'BEFORE_SET'
   if marker not in t['stderr']:unexpected.append({'case':key,'reason':'wrong callback error'})
  if not extension and t['exit']==v['exit']==0 and t.get('text') is not None and v.get('text') is not None:
   item['text_matches']=' '.join(t['text'].split())==' '.join(v['text'].split())
   if not item['text_matches']:item['tekt_text']=t['text'];item['vanilla_text']=v['text']
  rows.append(item)
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'method':'Fixed bounded controls, not a language coverage percentage. Repeated sources across files are not independent coverage. Error text differences are reported separately from rejection.','rows':rows,'unexpected':unexpected,'pending':sum(bool(r['pending']) for r in rows),'extensions':sum(r['extension'] for r in rows),'observations':len(rows),'message_differences':[r for r in rows if r.get('message_matches') is False],'text_differences':[r for r in rows if r.get('text_matches') is False]}
(p/'probe-audit.json.gz').write_bytes(gzip.compress(json.dumps(result,indent=2,ensure_ascii=False).encode()))
print(json.dumps({k:v for k,v in result.items() if k!='rows'},indent=2,ensure_ascii=False))
assert not unexpected,unexpected
