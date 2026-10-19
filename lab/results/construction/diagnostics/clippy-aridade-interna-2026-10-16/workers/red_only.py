from pathlib import Path
import json,gzip,hashlib,subprocess,re,collections,datetime
out=Path('00_nucleo/diagnosticos/clippy-aridade-interna-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert all(sha(p)==before['files'][p]['sha256'] for p in before['owners'])
records=[]
for line in (out/'baseline.log').read_text().splitlines():
 try:x=json.loads(line)
 except ValueError:continue
 if x.get('label') in ['export','pdf'] and x['exit']==0:records.append(x)
assert len(records)==2
cmd=['cargo','clippy','--offline','--locked','--no-deps','-p','typst-infra','--all-targets','--message-format=json','--','-A','warnings','-D','clippy::too_many_arguments','-D','unfulfilled_lint_expectations']
with (out/'red-only.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
errors={}
for line in (out/'red-only.log').read_text().splitlines():
 try:x=json.loads(line)
 except ValueError:continue
 if x.get('reason')!='compiler-message':continue
 m=x['message'];code=(m.get('code') or {}).get('code');spans=[s for s in m.get('spans',[]) if s['is_primary']]
 if code and spans and m['level']=='error':errors[(code,spans[0]['file_name'],spans[0]['byte_start'])]=m
counts=collections.Counter(k[0] for k in errors);names=[]
for (code,path,offset),m in errors.items():
 span=next(s for s in m['spans'] if s['is_primary']);fragment=Path(path).read_bytes()[span['byte_start']:span['byte_end']].decode();names.append((path,re.search(r'\bfn (\w+)',fragment).group(1)))
selected={(c['source'],c['function']) for c in before['cases']};record=dict(label='red',command=cmd,exit=r.returncode,errors=dict(counts),functions=names,selected_all_observed=selected<=set(names));records.append(record)
(out/'baseline-details.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),commands=records,source_hashes={p:sha(p) for p in before['all_source_hashes']},original_rust_sources_unchanged=all(sha(p)==before['files'][p]['sha256'] for p in before['owners']),note='Controles export/pdf aprovados são recuperados dos logs da primeira execução; apenas RED é repetido com --no-deps após o bloqueio em core dependente. Nenhuma edição Rust.'),ensure_ascii=False,indent=2)+'\n')
assert r.returncode==101 and set(counts)=={'clippy::too_many_arguments'} and selected<=set(names),(r.returncode,counts,names)
print(json.dumps(record),flush=True)
