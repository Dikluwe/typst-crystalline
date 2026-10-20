"""Repair only manifested owner seals after checking the complete owner graph."""
from pathlib import Path
import argparse
import gzip
import importlib.util
import json
import re
import subprocess

ROOT = Path.cwd()
HERE = Path(__file__).resolve().parent
parser = argparse.ArgumentParser()
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
assert not args.output.exists()
args.output.mkdir(parents=True)
spec = importlib.util.spec_from_file_location('validation', ROOT / 'lab/tools/ci/validate_layout_migration.py')
validation = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validation)
scope = json.loads((HERE / 'scope.json').read_text())
pairs = {(pair['source'], pair['owner']) for pair in scope['pairs']}
allowed = set(scope['sources']) | set(scope['owners'])
linter = ROOT / 'lab/target/linter/debug/crystalline-lint'
assert validation.digest(linter) == '421d0797e7ae1a5102e9c5bcd87b488fea70a1997394c38765a03239fe579121'
before = validation.frozen_inputs()
record = {'utc_start': validation.utc(), 'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
          'diff_head_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True), 'before': before}
preflight = subprocess.run([str(linter), '.', '--checks', 'v15,v26', '--fail-on', 'warning'], capture_output=True, text=True)
(args.output / 'ownership.log').write_text(preflight.stdout + preflight.stderr)
assert preflight.returncode == 0
dry = subprocess.run([str(linter), '--fix-hashes', '--dry-run', '.'], capture_output=True, text=True)
(args.output / 'dry-run.log').write_text(dry.stdout + dry.stderr)
planned = re.findall(r'^Would fix \./(\S+) prompt=(\S+)', dry.stdout, re.M)
assert dry.returncode == 0 and planned and len(planned) == len(set(planned))
assert set(planned) <= pairs, set(planned) - pairs
assert before == validation.frozen_inputs()
run = subprocess.run([str(linter), '--fix-hashes', '.'], capture_output=True, text=True)
(args.output / 'applied.log').write_text(run.stdout + run.stderr)
after = validation.frozen_inputs()
changed = [p for p in before.keys() | after.keys() if before.get(p) != after.get(p)]
record.update(utc_end=validation.utc(), exit=run.returncode, planned=planned,
              changed_inputs=changed, after=after,
              outside_changes=sorted(set(changed) - allowed))
with gzip.open(args.output / 'receipt.json.gz', 'wt') as file:
    json.dump(record, file, indent=2)
assert run.returncode == 0 and not record['outside_changes']
print('Scoped seal repairs:', len(planned), 'pairs; outside changes:', record['outside_changes'])
