"""Recreate the candidate in a new isolated directory. Does not build or integrate."""
from pathlib import Path
import subprocess,sys
here=Path(__file__).resolve().parent
source=Path(sys.argv[1])
assert not source.exists(),'Use a new isolated destination; preserve prior sources.'
for step in ['prepare.py','implement.py']:
 subprocess.run([sys.executable,str(here/step),str(source)],check=True)
p=source/'03_infra/src/pipeline.rs';t=p.read_text();i=t.rfind('}');p.write_text(t[:i]+(here/'extra-tests.rs').read_text()+t[i:])
