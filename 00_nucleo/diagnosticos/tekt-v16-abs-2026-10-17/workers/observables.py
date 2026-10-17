from pathlib import Path
import json,gzip,datetime
out=Path(__file__).resolve().parents[1];rows=[];open_cases=[]
for prefix in ['', 'supplement-']:
 old=json.load(gzip.open(out/(prefix+'before-measurement.json.gz')));new=json.load(gzip.open(out/(prefix+'after-measurement.json.gz')));key=lambda r:(r['name'],r['binary']);a={key(r):r for r in old['cases']};b={key(r):r for r in new['cases']};assert a.keys()==b.keys()
 for name in dict.fromkeys(r['name'] for r in new['cases']):
  for kind in ['tekt','vanilla']:
   x,y=a[name,kind],b[name,kind];assert (x['expression'],x['exit'],x['stdout'],x['stderr'])==(y['expression'],y['exit'],y['stdout'],y['stderr']),(name,kind,'observable changed')
  t,v=b[name,'tekt'],b[name,'vanilla'];err=lambda r:[s for s in r['stderr'].splitlines() if s.startswith('error:') or 'hint:' in s];same=(t['exit'],t['stdout'],err(t))==(v['exit'],v['stdout'],err(v))
  if not prefix and name in ['overflow','overflow-extra']:
   assert not same and 'cannot write minimum integer manually' in v['stderr'];open_cases.append(dict(name=name,reason='Literal construction differs before calc.abs; int.min supplementary cases certify actual overflow',tekt_exit=t['exit'],vanilla_exit=v['exit']));continue
  assert same,(prefix,name,t,v);rows.append(dict(corpus=prefix,name=name,exit=t['exit']))
assert len(rows)==37 and len(open_cases)==2
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),pairs=39,certified_abs_pairs=37,retained_literal_debt=open_cases,runtime_observables_unchanged_in_both_binaries=True,criteria='Before/after each binary: exact expression, exit, stdout and full stderr. Tekt/oracle: exact exit/stdout and primary error/hint lines. Two literal-construction minimum-literal cases preserved as debt, not abs witnesses.',cases=rows);(out/'observables-result.json').write_text(json.dumps(r,indent=2)+'\n');print({k:v for k,v in r.items() if k!='cases'})
