from pathlib import Path
import json,subprocess,gzip,hashlib,datetime,re
for n in ['entities/content/construction','compiler/stdlib/structural/markup']:
 p=Path('00_nucleo/prompts/'+n+'.md');s=p.read_text();line=re.search(r'^Hash do Código: [^\n]+\n',s,re.M).group();s=s.replace(line,'');i=s.index('\n');s=s[:i+1]+line+s[i+1:];p.write_text(s)
m=json.loads(Path('00_nucleo/materialization/raw-presenca-2026-10-14/manifest.json').read_text());out=Path('00_nucleo/diagnosticos/raw-presenca-2026-10-14');(out/'seal.log').rename(out/'seal-metadata-blocked.log')
with (out/'seal.log').open('w') as f:subprocess.run(['python3','/tmp/tekt-seal-owned.py']+list(m['consumers'].values()),stdout=f,stderr=subprocess.STDOUT,check=True)
paths=list(m['base_files'])+['00_nucleo/prompts/_nuclei/raw-field-presence.toml'];r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={n:dict(text=Path(n).read_text(),sha256=hashlib.sha256(Path(n).read_bytes()).hexdigest()) for n in paths})
with gzip.open(out/'candidate-final.json.gz','wt') as f:json.dump(r,f)
