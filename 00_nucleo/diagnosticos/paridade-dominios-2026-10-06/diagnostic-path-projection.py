from pathlib import Path
import json,gzip,os,shutil
H=Path('00_nucleo/diagnosticos/paridade-dominios-2026-10-06');T=Path('/tmp/tekt-parity-macro');R=Path.cwd();shutil.copy(H/'measurement.json.gz',H/'measurement-raw.json.gz');j=json.loads(gzip.open(H/'measurement.json.gz','rt').read())
for n,c in j['cases'].items():
 if c['mode']!='error':continue
 fp=T/(n+'.typ');project=lambda text:text.replace(str(fp),'<source>').replace(os.path.relpath(fp,R),'<source>');a=c['engines']['tekt'];b=c['engines']['vanilla'];c['observations']['message_span_hint_match_after_source_path_projection']=a['returncode']!=0 and b['returncode']!=0 and project(a['stderr'])==project(b['stderr'])
with gzip.open(H/'measurement.json.gz','wt') as f:json.dump(j,f,ensure_ascii=False)
(H/'diagnostic-path-projection.py').write_text(Path(__file__).read_text())
