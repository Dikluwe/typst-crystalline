from pathlib import Path
import json,gzip,os
H=Path('00_nucleo/diagnosticos/show-recursao-auditoria-2026-10-06/repair');T=Path('/tmp/tekt-show-occurrence/macro');R=Path.cwd();d=json.load(gzip.open(H/'macro.json.gz','rt'))
for n,c in d['cases'].items():
 if c['mode']!='error':continue
 f=T/(n+'.typ');paths=sorted([str(f),os.path.relpath(f,R)],key=len,reverse=True)
 def project(text):
  for path in paths:text=text.replace(path,'<source>')
  return text
 a=c['engines']['tekt'];b=c['engines']['vanilla'];c['observations']['message_span_hint_match_after_source_path_projection']=a['returncode']!=0 and b['returncode']!=0 and project(a['stderr'])==project(b['stderr']);print(n,c['observations']['message_span_hint_match_after_source_path_projection'])
with gzip.open(H/'macro-with-path-projection.json.gz','wt') as f:json.dump(d,f,ensure_ascii=False)
