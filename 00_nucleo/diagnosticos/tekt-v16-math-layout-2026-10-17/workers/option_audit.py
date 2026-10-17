from pathlib import Path
import subprocess
out=Path(__file__).resolve().parents[1]
with (out/'all-contexts.json').open('w') as f:subprocess.run(['/tmp/tekt-math-layout-audit','.',str(out/'input.sarif.json'),str(out/'latent.json')],stdout=f,check=True)
