from pathlib import Path
import gzip,json,subprocess,sys
b=json.load(gzip.open('00_nucleo/diagnosticos/clippy-aridade-interna-2026-10-16/before.json.gz'))
sys.exit(subprocess.run(['python3','00_nucleo/diagnosticos/clippy-aridade-interna-2026-10-16/workers/seal_owned.py',*[p for p in b['files'] if p.endswith('.rs')]]).returncode)
