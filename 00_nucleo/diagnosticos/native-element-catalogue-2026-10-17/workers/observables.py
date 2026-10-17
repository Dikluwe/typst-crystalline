from pathlib import Path
import gzip,json,collections
out=Path(__file__).resolve().parents[1];load=lambda f:json.load(gzip.open(out/f));before=load('before-measurement.json.gz');after=load('after-measurement.json.gz');schema=json.load(open(out/'batch-schema.json'))['cases'];allowed={c['expression'] for c in schema}|{'std.format.bundle.asset','std.op'};obs=lambda r:(r['exit'],r['stdout'],r['stderr']);warning='warning: ficheiro vazio: sem conteúdo\n\n';warning_controls=sum(r['exit']==0 and r['stderr']==warning for a in before['cases'] for r in a['queries']['tekt'].values());assert warning_controls>0;changed=[];preserved=0;repaired=0
assert before['binaries']['vanilla']['sha256']==after['binaries']['vanilla']['sha256']
assert len(before['cases'])==len(after['cases'])==3956
for a,z in zip(before['cases'],after['cases']):
 assert (a['expression'],a['profile'])==(z['expression'],z['profile'])
 for bin in a['queries']:
  assert a['queries'][bin].keys()==z['queries'][bin].keys()
  for op,av in a['queries'][bin].items():
   zv=z['queries'][bin][op]
   if obs(av)==obs(zv):preserved+=1;continue
   assert bin=='tekt' and op in ['where','kind'] and a['expression'] in allowed,(a['expression'],op,bin)
   assert av['exit']==1 and zv['exit']==0 and zv['stderr']==warning,(a['expression'],op,zv)
   v=z['queries']['vanilla'];vv=v.get(op)
   if vv is not None and v['type']['exit']==0:
    assert (zv['exit'],zv['stdout'])==(vv['exit'],vv['stdout']) and vv['stderr']=='',(a['expression'],op,zv,vv);repaired+=1
   changed.append(dict(expression=a['expression'],profile=a['profile'],operation=op,oracle_available=vv is not None))
assert changed
b=load('before-batch.json.gz');a=load('after-batch.json.gz');residual=[];exact=0;headlines=0;counter_preserved=0
assert len(b['cases'])==len(a['cases'])==110
for old,new in zip(b['cases'],a['cases']):
 assert (old['expression'],old['label'])==(new['expression'],new['label'])
 t=new['results']['tekt'];v=new['results']['vanilla'];assert obs(old['results']['vanilla'])==obs(v)
 if new['label']=='counter':
  assert obs(old['results']['tekt'])==obs(t);counter_preserved+=1
  residual.append(dict(expression=new['expression'],label='counter',reason='Existing counter native admission is separate from metadata, unchanged',tekt=t,vanilla=v));continue
 if (t['exit'],t['stdout'])==(v['exit'],v['stdout']) and t['exit']==0 and t['stderr']==warning and v['stderr']=='':exact+=1;continue
 if new['label']=='unknown':
  assert t['exit']==v['exit']==1 and t['stderr'].splitlines()[0]==v['stderr'].splitlines()[0];headlines+=1
  residual.append(dict(expression=new['expression'],label='unknown',reason='Correct message; existing diagnostic anchor differs',tekt=t,vanilla=v));continue
 raise AssertionError((new,t,v))
result=dict(matrix_cases=len(a['cases']),runtime_cases=3956,changed_observables=len(changed),pin_matching_language_values=repaired,existing_empty_fixture_warning_controls=warning_controls,new_successes_keep_exact_existing_warning=warning,preserved_observables=preserved,changes=changed,batch_pin_matching_language_values=exact,unknown_headlines_match=headlines,counter_controls_preserved=counter_preserved,residuals=residual)
(out/'observables-result.json').write_text(json.dumps(result,indent=2)+'\n');print({k:v for k,v in result.items() if k not in ['changes','residuals']})
