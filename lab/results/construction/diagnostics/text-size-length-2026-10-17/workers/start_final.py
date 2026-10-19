from pathlib import Path
import datetime,gzip,hashlib,json,shutil,subprocess
out=Path(__file__).resolve().parents[1]
assert '14 passed; 0 failed;' in (out/'heading-green-2.log').read_text()
frozen=json.load(gzip.open(out/'heading-green-2-state.json.gz'))
assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in frozen['source_hashes'].items())
archive=json.load(open(out/'candidate-1-archive.json'))
assert hashlib.sha256((out/'candidate-1.tar.gz').read_bytes()).hexdigest()==archive['sha256']
shutil.copy(out/'validation-result.json',out/'candidate-1-validation.json')
for name in ['after-renders','after-extended','parbox-regressions','workspace']:
 p=out/name
 assert p.is_dir() and not p.is_symlink(),name
 shutil.rmtree(p)
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=frozen['head'],source_hashes=frozen['source_hashes'],all_four_commands_green=json.load(open(out/'heading-green-result.json')),test_result='14 passed; zero failed')
with gzip.open(out/'heading-final-green-state.json.gz','wt') as f:json.dump(record,f)
r=subprocess.run(['python3',str(out/'workers/validate.py')]);raise SystemExit(r.returncode)
