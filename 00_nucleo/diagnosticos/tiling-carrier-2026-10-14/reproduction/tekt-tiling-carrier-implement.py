from pathlib import Path
import json,gzip,datetime,hashlib,subprocess
out=Path('00_nucleo/diagnosticos/tiling-carrier-2026-10-14');owners=json.loads((out/'owners.json').read_text());paths=list(owners)+list(owners.values())+['00_nucleo/prompts/_nuclei/tiling-frame-transport.toml']
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),exit=101,command=['cargo','test','--offline','--locked','-p','typst-core','entities::frame_paint::tests','--','--test-threads=1'],red_kind='Compile failure: the approved carrier types do not yet exist, not a rendered-behavior failure.',files={n:dict(text=Path(n).read_text(),sha256=hashlib.sha256(Path(n).read_bytes()).hexdigest()) for n in paths})
with gzip.open(out/'red-state.json.gz','wt') as f:json.dump(r,f)
p=Path('01_core/src/entities/frame_paint.rs');s=p.read_text();i=s.index('#[cfg(test)]');p.write_text(s[:i]+Path('/tmp/tekt-tiling-carrier-implementation.txt').read_text()+'\n'+s[i:])
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true']+list(owners),check=True)
