"""Archive final diagnostic evidence; never reseal productive owners."""
import datetime,gzip,hashlib,json,subprocess
from pathlib import Path
H=Path(__file__).resolve().parent; R=H.parents[2]
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
git=lambda *a:subprocess.check_output(['git',*a],cwd=R,text=True)
b=json.load(gzip.open(H/'before-state.json.gz'))
checked={n:s for n,s in b['source_hashes'].items() if n.endswith('.rs') and n.startswith(('01_core/','02_shell/','03_infra/','04_wiring/'))}
assert all(sha(R/n)==s for n,s in checked.items())
assert sha(R/'target/release/typst')==b['cli_sha256']
logs={p.name:{'text':p.read_text(),'sha256':sha(p),'mtime_utc':datetime.datetime.fromtimestamp(p.stat().st_mtime,datetime.timezone.utc).isoformat()} for p in Path('/tmp/tekt-cmap-ranges-2026-10-05').glob('*.log')}
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'productive_sources_unchanged_since_before':checked,'cli_sha256':sha(R/'target/release/typst'),'public_l0_sha256':sha(R/'00_nucleo/prompts/entities/font_book.md'),'diagnostic_binary_after_tests_sha256':sha(R/'target/release/cmap-ranges-probe'),'logs':logs,'helpers':{str(p.relative_to(H)):p.read_text() for p in (H/'probe').rglob('*') if p.is_file()}}
with gzip.open(H/'execution.json.gz','wt') as f:json.dump(d,f,ensure_ascii=False,indent=2)
print('public L0:',d['public_l0_sha256']); print('unchanged productive sources:',len(checked)); print('CLI:',d['cli_sha256'])
