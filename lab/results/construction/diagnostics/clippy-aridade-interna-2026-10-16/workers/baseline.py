from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,os,tempfile,re,collections
out=Path('00_nucleo/diagnosticos/clippy-aridade-interna-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in before['all_source_hashes']};records=[]
with tempfile.TemporaryDirectory(prefix='tekt-arity-baseline-',dir='/dev/shm') as temp:
 env=dict(os.environ,TMPDIR=temp,TYPST_VANILLA=str(Path('lab/typst-original/target/release/typst').resolve()));env['PATH']='/tmp/tekt-linter-ci-target/debug:'+env['PATH'];assert sha(env['TYPST_VANILLA'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
 for label,cmd in [('export',['cargo','test','--offline','--locked','-p','typst-infra','--lib','export::','--','--test-threads=1']),('pdf',['cargo','test','--offline','--locked','-p','typst-infra','--lib','pipeline::pdf::','--','--test-threads=1']),('red',['cargo','clippy','--offline','--locked','--no-deps','-p','typst-infra','--all-targets','--message-format=json','--','-A','warnings','-D','clippy::too_many_arguments','-D','unfulfilled_lint_expectations'])]:
  with (out/('baseline-'+label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT,env=env)
  record=dict(label=label,command=cmd,exit=r.returncode);records.append(record)
  if label!='red':assert r.returncode==0,record
  else:
   errors={}
   for line in (out/'baseline-red.log').read_text().splitlines():
    try:x=json.loads(line)
    except ValueError:continue
    if x.get('reason')!='compiler-message':continue
    m=x['message'];code=(m.get('code') or {}).get('code');spans=[s for s in m.get('spans',[]) if s['is_primary']]
    if code and spans and m['level']=='error':errors[(code,spans[0]['file_name'],spans[0]['byte_start'])]=m
   counts=collections.Counter(k[0] for k in errors);names=[]
   for (code,path,offset),m in errors.items():
    span=next(s for s in m['spans'] if s['is_primary']);fragment=Path(path).read_bytes()[span['byte_start']:span['byte_end']].decode();names.append((path,re.search(r'\bfn (\w+)',fragment).group(1)))
   selected={(c['source'],c['function']) for c in before['cases']};assert r.returncode==101 and set(counts)=={'clippy::too_many_arguments'} and selected<=set(names),(r.returncode,counts,names)
   record.update(errors=dict(counts),functions=names,selected_all_observed=True)
  print(json.dumps(record),flush=True);assert all(sha(p)==h for p,h in hashes.items())
(out/'baseline-details.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),commands=records,source_hashes=hashes,sources_unchanged=True),indent=2)+'\n')
