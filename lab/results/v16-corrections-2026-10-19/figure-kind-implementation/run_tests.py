"""Run a recorded, resource-limited Figure test filter against frozen inputs."""
from pathlib import Path
import argparse
import gzip
import importlib.util
import json
import os
import subprocess

ROOT = Path(__file__).resolve().parents[4]
os.chdir(ROOT)
parser = argparse.ArgumentParser()
parser.add_argument('--output', required=True)
parser.add_argument('--filter', default='figure_kind_cause_regressions')
parser.add_argument('--package', default='typst-core')
parser.add_argument('--red', action='store_true')
args = parser.parse_args()
out = ROOT / args.output
out.mkdir(parents=True, exist_ok=True)
spec = importlib.util.spec_from_file_location('validation', ROOT / 'lab/tools/ci/validate_layout_migration.py')
validation = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validation)
inputs = validation.frozen_inputs()
env = os.environ.copy()
env.update(CARGO_BUILD_JOBS='1', CARGO_PROFILE_DEV_DEBUG='0',
           CARGO_PROFILE_TEST_DEBUG='0', CARGO_INCREMENTAL='0')
command = ['cargo', 'test', '--offline', '--locked', '-p', args.package,
           '--lib', args.filter, '--', '--test-threads=1']
record = {'utc_start': validation.utc(), 'head': subprocess.check_output(
    ['git', 'rev-parse', 'HEAD'], text=True).strip(),
    'diff_head_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True),
    'command': command, 'inputs': inputs, 'expected': 'semantic RED' if args.red else 'GREEN',
    'worker_sha256': validation.digest(Path(__file__)),
    'resource_limits': {'jobs': 1, 'memory_max': '12G', 'memory_high': '8G', 'swap_max': 0}}
with (out / 'tests.log').open('w') as log:
    result = subprocess.run(command, env=env, stdout=log, stderr=subprocess.STDOUT)
after = validation.frozen_inputs()
record.update(exit=result.returncode, utc_end=validation.utc(),
              changed_inputs=[p for p in inputs.keys() | after.keys() if inputs.get(p) != after.get(p)])
with gzip.open(out / 'state.json.gz', 'wt') as file:
    json.dump(record, file, indent=2)
log = (out / 'tests.log').read_text()
print(log[-7000:])
assert not record['changed_inputs']
if args.red:
    assert result.returncode == 101 and 'test result: FAILED.' in log and 'error[E' not in log
else:
    assert result.returncode == 0 and 'test result: ok.' in log
