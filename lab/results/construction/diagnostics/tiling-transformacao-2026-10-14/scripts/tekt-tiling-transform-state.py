from pathlib import Path
import gzip,json,hashlib,subprocess,re,datetime,sys
out=Path('00_nucleo/diagnosticos/tiling-transformacao-2026-10-14');sources=['01_core/src/compiler/layout/tiling.rs','01_core/src/entities/tiling.rs','01_core/src/compiler/stdlib/visualize.rs'];files=sources+[re.search(r'^//! @prompt (.+)$',Path(p).read_text(),re.M).group(1) for p in sources]+['00_nucleo/prompts/_nuclei/tiling-placement.toml']
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:dict(text=Path(p).read_text(),sha256=hashlib.sha256(Path(p).read_bytes()).hexdigest()) for p in files})
with gzip.open(out/(sys.argv[1]+'.json.gz'),'wt') as f:json.dump(r,f)
