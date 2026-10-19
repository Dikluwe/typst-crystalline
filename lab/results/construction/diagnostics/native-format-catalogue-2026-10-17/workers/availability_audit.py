from pathlib import Path
import json,gzip,datetime,hashlib
out=Path(__file__).resolve().parents[1];before=json.load(gzip.open(out/'before-measurement.json.gz'));after=json.load(gzip.open(out/'after-measurement.json.gz'));rows=[];obs=lambda r:(r['exit'],r['stdout'],r['stderr'])
for old,new in zip(before['cases'],after['cases']):
 if not(new['profile']&1) and (new['expression']=='std.html' or new['expression'].startswith('std.html.')):
  t=new['queries']['tekt']['type'];v=new['queries']['vanilla']['type'];a=old['queries'];assert obs(a['tekt']['type'])==obs(t) and obs(a['vanilla']['type'])==obs(v)
  if t['exit']==0 and v['exit']!=0:
   assert v['stderr'].splitlines()[0]=='error: cannot access field `html` because the `html` feature is not enabled'
   rows.append(dict(expression=new['expression'],profile=new['profile'],tekt=t,vanilla=v))
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),after_measurement_sha256=hashlib.sha256((out/'after-measurement.json.gz').read_bytes()).hexdigest(),source_hashes=after['source_hashes'],binaries=after['binaries'],type_cases=len(rows),unique_paths=len({r['expression'] for r in rows}),preexisting_before_after_identical=True,cases=rows);assert r['type_cases']==460 and r['unique_paths']==115;(out/'html-off-availability-audit.json').write_text(json.dumps(r,indent=2)+'\n')
with gzip.open(out/'html-off-availability-audit.json.gz','wt') as f:json.dump(r,f)
print('HTML off',r['type_cases'],r['unique_paths'])
