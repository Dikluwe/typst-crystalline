from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,os,tempfile,re,collections
out=Path('00_nucleo/diagnosticos/clippy-retencao-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in before['all_source_hashes']};records=[];now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
with tempfile.TemporaryDirectory(prefix='tekt-retention-baseline-',dir='/dev/shm') as temp:
 env=dict(os.environ,TMPDIR=temp,TYPST_VANILLA=str(Path('lab/typst-original/target/release/typst').resolve()));env['PATH']='/tmp/tekt-linter-ci-target/debug:'+env['PATH']
 commands=[('retention',['cargo','test','--offline','--locked','-p','typst-infra','--lib','memory_snapshot_tests::','--','--test-threads=1']),('subset',['cargo','test','--offline','--locked','-p','typst-infra','--lib','export::subset::','--','--test-threads=1']),('red',['cargo','clippy','--offline','--locked','--no-deps','-p','typst-infra','--all-targets','--message-format=json','--','-A','warnings','-D','dead_code','-D','clippy::vec_box','-D','unfulfilled_lint_expectations'])]
 for label,cmd in commands:
  start=now()
  with (out/('baseline-'+label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT,env=env)
  record=dict(label=label,command=cmd,start=start,end=now(),exit=r.returncode);records.append(record)
  unchanged=all(sha(p)==h for p,h in hashes.items());(out/'baseline-details.json').write_text(json.dumps(dict(commands=records,source_hashes=hashes,sources_unchanged=unchanged),indent=2)+'\n');assert unchanged
  if label!='red':
   summaries=re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;', (out/('baseline-'+label+'.log')).read_text(),re.M);record['summaries']=summaries;assert r.returncode==0 and summaries,record
  else:
   errors={}
   for line in (out/'baseline-red.log').read_text().splitlines():
    try:x=json.loads(line)
    except ValueError:continue
    if x.get('reason')!='compiler-message':continue
    m=x['message'];code=(m.get('code') or {}).get('code');spans=[s for s in m.get('spans',[]) if s['is_primary']]
    if code and spans and m['level']=='error':errors[(code,spans[0]['file_name'],spans[0]['byte_start'])]=m
   counts=collections.Counter(k[0] for k in errors);record.update(errors=dict(counts),messages=[m['message'] for m in errors.values()]);assert r.returncode==101 and counts=={'dead_code':3,'clippy::vec_box':1},record
  (out/'baseline-details.json').write_text(json.dumps(dict(commands=records,source_hashes=hashes,sources_unchanged=True),indent=2)+'\n');print(json.dumps(record),flush=True)
