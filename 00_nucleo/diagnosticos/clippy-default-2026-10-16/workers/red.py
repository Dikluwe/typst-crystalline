from pathlib import Path
import subprocess,json,datetime,sys
out=Path('00_nucleo/diagnosticos/clippy-default-2026-10-16');records=[]
for package in ['typst-core','typst-infra']:
 cmd=['cargo','test','--offline','--locked','-p',package,'--lib','default_contract','--','--test-threads=1']
 with (out/('red-'+package+'.log')).open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 text=(out/('red-'+package+'.log')).read_text();types=['EvalContext'] if package=='typst-core' else ['FontCatalogSession','SourceSession']
 assert r.returncode==101,(package,r.returncode)
 assert all(t in text for t in types)
 assert 'E0277' in text
 records.append(dict(package=package,command=cmd,exit=r.returncode,types_without_default=types))
(out/'red-details.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),commands=records),indent=2)+'\n')
print(json.dumps(records))
