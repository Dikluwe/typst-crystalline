"""Capture the candidate's exact sources and validation logs before verdict."""
from pathlib import Path
import gzip,json,datetime,subprocess,hashlib
D=Path(__file__).resolve().parent;R=D.parents[2]
files=[p for pair in json.loads((D/'owners.json').read_text()) for p in pair]
logs=['release','core-tests','lint','qualified-extended','qualified-effects']
state={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'files':{p:{'text':(R/p).read_text(),'sha256':hashlib.sha256((R/p).read_bytes()).hexdigest()} for p in files},'binary_sha256':hashlib.sha256((R/'target/release/typst').read_bytes()).hexdigest(),'logs':{n:Path('/tmp/preflights-'+n+'.log').read_text() for n in logs}}
with gzip.open(D/'validation.json.gz','wt') as f:json.dump(state,f,ensure_ascii=False,indent=2)
