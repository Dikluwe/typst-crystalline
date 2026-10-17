from pathlib import Path
import json,gzip,subprocess,sys
b=json.load(gzip.open('00_nucleo/diagnosticos/type-complexity-2026-10-15/before.json.gz'))
sys.exit(subprocess.call(['python3','/tmp/tekt-seal-owned.py',*[p for p in b['files'] if p.endswith('.rs')]]))
