from pathlib import Path
import datetime,json
out=Path(__file__).resolve().parents[1];load=lambda n:json.load(open(out/n));before=load('public-before.json')['stdout'].splitlines();after=load('public-after.json')['stdout'].splitlines();assert len(before)==len(after)==12
for i in [0,2,4]:assert after[i]==before[i] and after[i+1].replace('source=true','source=false')==after[i]
assert 'footer source=true kept_value_source=true' in after;assert 'levels source=true ok=true error=None' in after;assert next(s for s in after if s.startswith('group source=true')).replace('source=true','source=false')==next(s for s in after if s.startswith('group source=false'))
snap=load('snapshot-after.json')['stdout'].splitlines();assert len(snap)==4 and all(x.endswith('context_floor=77') for x in snap)
debts={'header-colspan','header-levels','footer-explicit-cell'};checks=[]
for family in ['cli','cli-extra','cli-controls']:
 old=load(family+'-before.json');new=load(family+'-after.json');a={(x['name'],x['binary']):x for x in old['cases']};z={(x['name'],x['binary']):x for x in new['cases']};assert a.keys()==z.keys()
 for name in sorted({x['name'] for x in new['cases']}):
  tekt=z[name,'tekt'];vanilla=z[name,'vanilla'];prior=a[name,'tekt'];assert tekt['exit']==vanilla['exit']
  status='verified'
  if name in debts:
   assert tekt.get('rows')!=vanilla.get('rows');assert tekt.get('rows')==prior.get('rows');status='pending-table-row-group-contract'
  elif tekt['exit']==0:assert tekt['rows']==vanilla['rows'],(name,tekt['rows'],vanilla['rows'])
  else:
   def clean(text,phase):
    folder=out/(family+'-'+phase)
    for prefix in [str(folder),str(folder.relative_to(Path.cwd()))]:text=text.replace(prefix,'FIXTURE')
    return text
   if name=='header-too-wide':assert tekt['stderr']==vanilla['stderr']
   assert clean(tekt['stderr'],'after')==clean(prior['stderr'],'before'),name
   if name.startswith('invalid-show-'):status='invalid-type-rejection-preserved-message-parity-not-certified'
  checks.append(dict(name=name,status=status,exit=tekt['exit'],tekt_rows=tekt.get('rows'),vanilla_rows=vanilla.get('rows')))
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),public_source_pairs_restored=True,snapshots_restored=True,cli_cases=checks,pending_table_row_group_contract=sorted(debts),scope='Language cells, attributes, origin, context and observable diagnostics. No geometry or global parity claim.')
(out/'observables-result.json').write_text(json.dumps(record,indent=2,ensure_ascii=False)+'\n');print(json.dumps({k:v for k,v in record.items() if k!='cli_cases'}),flush=True)
