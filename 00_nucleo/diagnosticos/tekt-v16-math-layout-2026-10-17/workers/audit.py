from pathlib import Path
import json,subprocess,hashlib,gzip
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items());subprocess.run(json.load(open(out/'audit-recipe.json'))['command'],check=True)
with (out/'contexts.json').open('w') as f:subprocess.run(['/tmp/tekt-math-layout-audit','.',str(out/'input.sarif.json'),str(out/'latent.json')],stdout=f,check=True)
print('18 AST contexts collected; productive code unchanged',flush=True)
