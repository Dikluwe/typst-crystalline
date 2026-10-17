from pathlib import Path
import gzip,json,re
out=Path(__file__).resolve().parents[1];load=lambda name:json.load(gzip.open(out/name));before=load('before-measurement.json.gz');after=load('after-measurement.json.gz');schema=json.load(open(out/'batch-schema.json'))['cases'];allowed={c['expression'] for c in schema};obs=lambda r:(r['exit'],r['stdout'],r['stderr']);warning='warning: ficheiro vazio: sem conteúdo\n\n';preserved=0;changed=[];repairs=0;missing_before=set();missing_after=set()
assert before['binaries']['vanilla']['sha256']==after['binaries']['vanilla']['sha256'];assert len(before['cases'])==len(after['cases'])==3956
for a,z in zip(before['cases'],after['cases']):
 assert (a['expression'],a['profile'])==(z['expression'],z['profile'])
 for row,missing in [(a,missing_before),(z,missing_after)]:
  t=row['queries']['tekt'];v=row['queries']['vanilla']
  if t['type']['exit']==v['type']['exit']==0 and t['type']['stdout']==v['type']['stdout'] and t['where']['exit']!=0 and v['where']['exit']==0:missing.add(row['expression'])
 for bin in a['queries']:
  assert a['queries'][bin].keys()==z['queries'][bin].keys()
  for op,av in a['queries'][bin].items():
   zv=z['queries'][bin][op]
   if obs(av)==obs(zv):preserved+=1;continue
   assert bin=='tekt' and op in ['where','kind'] and a['expression'] in allowed,(bin,op,a['expression'])
   assert av['exit']==1 and zv['exit']==0 and zv['stderr']==warning,(a['expression'],op,zv)
   vv=z['queries']['vanilla'].get(op)
   if vv is not None and z['queries']['vanilla']['type']['exit']==0:
    assert (zv['exit'],zv['stdout'])==(vv['exit'],vv['stdout']) and vv['stderr']=='';repairs+=1
   changed.append(dict(expression=a['expression'],profile=a['profile'],operation=op,oracle_available=vv is not None))
assert missing_before==allowed and not missing_after
b=load('before-batch.json.gz');a=load('after-batch.json.gz');assert len(b['cases'])==len(a['cases'])==144;excluded={c['expression']:set(c['excluded']) for c in schema};counts=dict(values=0,headlines=0,counter=0,partial=0,root_calls=0);residual=[]
for old,new in zip(b['cases'],a['cases']):
 assert (old['expression'],old['label'])==(new['expression'],new['label']);t=new['results']['tekt'];v=new['results']['vanilla'];assert obs(old['results']['vanilla'])==obs(v)
 if new['label'] in ['counter','partial','root-call','root-args']:
  assert obs(old['results']['tekt'])==obs(t);counts['counter' if new['label']=='counter' else 'partial' if new['label']=='partial' else 'root_calls']+=1
  if new['label']=='counter':residual.append(dict(expression=new['expression'],label='counter',reason='Separate native counter admission unchanged',tekt=t,vanilla=v))
  continue
 if (t['exit'],t['stdout'])==(v['exit'],v['stdout']) and t['exit']==0 and t['stderr']==warning and v['stderr']=='':counts['values']+=1;continue
 if new['label']=='unknown' or new['label'] in excluded[new['expression']]:
  assert t['exit']==v['exit']==1 and t['stderr'].splitlines()[0]==v['stderr'].splitlines()[0];counts['headlines']+=1;residual.append(dict(expression=new['expression'],label=new['label'],reason='Correct message; diagnostic anchor residual',tekt=t,vanilla=v));continue
 raise AssertionError((new,t,v))
cb=load('before-compat.json.gz');ca=load('after-compat.json.gz');assert len(cb['cases'])==len(ca['cases'])==456
for x,y in zip(cb['cases'],ca['cases']):
 assert (x['profile'],x['expression'],x['label'])==(y['profile'],y['expression'],y['label'])
 for bin in x['results']:assert obs(x['results'][bin])==obs(y['results'][bin]),(x['expression'],x['label'],bin)
layouts=[]
for name in ['red.log','validation-green.log']:
 found=re.findall(r'FORMAT_LAYOUT (\{[^\n]+\})',(out/name).read_text());assert len(found)==1;layouts.append(json.loads(found[0]))
assert layouts[0]==layouts[1],layouts
result=dict(runtime_cases=3956,batch_cases=144,compat_cases=456,all_compat_observables_preserved=True,changed_observables=len(changed),pin_matching_language_values=repairs,preserved_observables=preserved,changes=changed,missing_paths_before=sorted(missing_before),missing_paths_after=sorted(missing_after),batch_counts=counts,residuals=residual,layout_before=layouts[0],layout_after=layouts[1],layout_unchanged=True);(out/'observables-result.json').write_text(json.dumps(result,indent=2)+'\n');print({k:v for k,v in result.items() if k not in ['changes','residuals','missing_paths_before','missing_paths_after']})
