from pathlib import Path
import json,subprocess,collections,datetime
out=Path('00_nucleo/diagnosticos/clippy-excecoes-2026-10-16');records=[]
for label,package,rule in [('arity','typst-core','clippy::too_many_arguments'),('counter','typst-core','clippy::mutable_key_type'),('ranges','typst-infra','clippy::single_range_in_vec_init')]:
 cmd=['cargo','clippy','--offline','--locked','-p',package,'--all-targets','--message-format=json','--','-A','warnings','-D',rule,'-D','unfulfilled_lint_expectations']
 with (out/('red-'+label+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 messages={}
 for line in (out/('red-'+label+'.log')).read_text().splitlines():
  try:x=json.loads(line)
  except ValueError:continue
  if x.get('reason')!='compiler-message':continue
  m=x['message'];s=[s for s in m.get('spans',[]) if s['is_primary']];code=(m.get('code') or {}).get('code')
  if m['level']=='error' and s and code:messages[(code,s[0]['file_name'],s[0]['byte_start'])]=m
 counts=collections.Counter(k[0] for k in messages);assert r.returncode==101 and set(counts)=={rule},(label,r.returncode,counts)
 if label=='counter':assert counts[rule]==1
 if label=='ranges':assert counts[rule]==4
 record=dict(label=label,command=cmd,exit=r.returncode,errors=dict(counts));records.append(record);print(json.dumps(record),flush=True)
(out/'red-details.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),commands=records,note='RED de aridade observa o core produtivo; Cargo pode interromper antes dos alvos cfg(test). O inventário anterior all-targets preserva as 57 ocorrências reais.'),ensure_ascii=False,indent=2)+'\n')
