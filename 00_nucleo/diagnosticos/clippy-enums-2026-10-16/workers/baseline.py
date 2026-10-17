from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,re,collections
out=Path('00_nucleo/diagnosticos/clippy-enums-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in before['all_source_hashes']};records=[];now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
base=['cargo','test','--offline','--locked','-p','typst-core']
commands=[('outline',base+['--lib','outline','--','--test-threads=1']),('context',base+['--lib','context','--','--test-threads=1','--skip','context_read_request_fits_common_value_budget']),('callbacks',base+['--lib','callback','--','--test-threads=1']),('p1405',base+['--test','p1405_outline_indented','--','--test-threads=1']),('red-size',base+['--lib','context_read_request_fits_common_value_budget','--','--test-threads=1','--nocapture']),('red-clippy',['cargo','clippy','--offline','--locked','--no-deps','-p','typst-core','--all-targets','--message-format=json','--','-A','warnings','-D','clippy::large_enum_variant','-D','unfulfilled_lint_expectations'])]
for label,cmd in commands:
 start=now()
 with (out/('baseline-'+label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 record=dict(label=label,command=cmd,start=start,end=now(),exit=r.returncode);records.append(record)
 unchanged=all(sha(p)==h for p,h in hashes.items());(out/'baseline-details.json').write_text(json.dumps(dict(commands=records,source_hashes=hashes,sources_unchanged=unchanged),indent=2)+'\n');assert unchanged
 if label!='red-clippy':
  output=(out/('baseline-'+label+'.log')).read_text();summaries=re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',output,re.M);record['summaries']=summaries
  if label=='red-size':assert r.returncode==101 and summaries==[('FAILED','0','1','0')] and 'request storage 432 exceeds' in output,record
  else:assert r.returncode==0 and summaries,record
 else:
  errors={}
  for line in (out/'baseline-red-clippy.log').read_text().splitlines():
   try:x=json.loads(line)
   except ValueError:continue
   if x.get('reason')!='compiler-message':continue
   m=x['message'];code=(m.get('code') or {}).get('code');spans=[s for s in m.get('spans',[]) if s['is_primary']]
   if code and spans and m['level']=='error':errors[(code,spans[0]['file_name'],spans[0]['byte_start'])]=m
  counts=collections.Counter(k[0] for k in errors);record.update(errors=dict(counts),files=sorted(set(k[1] for k in errors)));assert r.returncode==101 and counts=={'clippy::large_enum_variant':5},record
 (out/'baseline-details.json').write_text(json.dumps(dict(commands=records,source_hashes=hashes,sources_unchanged=True),indent=2)+'\n');print(json.dumps(record),flush=True)
