"""Choose an actual-root artifact and run the prepared public invariant probe."""
from pathlib import Path
import subprocess
P=Path(__file__).resolve().parent;R=Path.cwd().resolve();deps=R/'lab/target/product/debug/deps'
choices=[]
for p in deps.glob('libtypst_core-*.rlib'):
 d=p.with_name(p.stem.removeprefix('lib')+'.d')
 if not d.exists():continue
 text=d.read_text()
 roots=[Path(t).resolve() for t in text.replace('\\\n',' ').split() if t.endswith('01_core/src/lib.rs')]
 if roots and all(t==R/'01_core/src/lib.rs' for t in roots):choices.append(p)
assert len(choices)==1,choices
raise SystemExit(subprocess.call(['python3',str(P/'agents/core/run_public_hash_probe.py'),'--build-receipt',str(P/'candidate-build/receipt.json'),'--rlib',str(choices[0]),'--out',str(P/'hash-before')]))
