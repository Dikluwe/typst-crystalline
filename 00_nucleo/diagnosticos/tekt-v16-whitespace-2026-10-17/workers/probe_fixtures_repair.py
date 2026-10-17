from pathlib import Path
import json,gzip,hashlib,datetime,shutil,re,subprocess
out=Path('00_nucleo/diagnosticos/tekt-v16-whitespace-2026-10-17');archive=out/'attempt-probe-fixtures';archive.mkdir();shutil.copyfile(out/'workers/probe.py',archive/'probe.py')
for p in list(out.iterdir()):
 if p.is_file() and p.name.startswith('probe-baseline'):shutil.move(str(p),archive/p.name)
b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();allowed=b['sources']+b['owners'];assert all(sha(p)==h for p,h in b['hashes'].items() if p not in allowed);fixtures={};source=Path('03_infra/src/integration_tests.rs')
for name in re.findall(r'include_str!\(\s*"([^"]+)"',source.read_text()):
 p=(source.parent/name).resolve();rel=p.relative_to(Path.cwd());assert p.is_file();fixtures[str(rel)]=sha(p)
assert len(fixtures)==16
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),reason='18 include_str references require 16 unique lab/.typ fixtures in the disposable infra workspace. Not a functional RED.',fixtures=fixtures,sources_unchanged_outside_current_test_L0_scope=True,status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True));(out/'probe-fixtures-repair.json').write_text(json.dumps(record,indent=2)+'\n');shutil.copyfile('/tmp/tekt-whitespace-probe-repair.py',out/'workers/probe_fixtures_repair.py')
