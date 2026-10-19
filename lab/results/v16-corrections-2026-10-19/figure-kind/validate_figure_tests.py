"""Record the existing Figure test baseline without changing product inputs."""
from pathlib import Path
import gzip
import importlib.util
import json
import os
import subprocess

ROOT = Path(__file__).resolve().parents[4]
OUT = Path(__file__).resolve().parent / 'validation'
os.chdir(ROOT)
spec = importlib.util.spec_from_file_location('validation', ROOT / 'lab/tools/ci/validate_layout_migration.py')
validation = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validation)
inputs = validation.frozen_inputs()
env = os.environ.copy()
env.update(CARGO_BUILD_JOBS='1', CARGO_PROFILE_DEV_DEBUG='0',
           CARGO_PROFILE_TEST_DEBUG='0', CARGO_INCREMENTAL='0')
command = ['cargo', 'test', '--offline', '--locked', '-p', 'typst-core',
           '--lib', 'figure', '--', '--test-threads=1']
record = {'utc_start': validation.utc(), 'head': subprocess.check_output(
    ['git', 'rev-parse', 'HEAD'], text=True).strip(),
    'diff_head_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True),
    'command': command, 'inputs': inputs,
    'resource_limits': {'jobs': 1, 'memory_max': '12G', 'memory_high': '8G', 'swap_max': 0}}
with (OUT / 'figure-tests.log').open('w') as log:
    result = subprocess.run(command, env=env, stdout=log, stderr=subprocess.STDOUT)
after = validation.frozen_inputs()
record.update(exit=result.returncode, utc_end=validation.utc(),
              changed_inputs=[p for p in inputs.keys() | after.keys() if inputs.get(p) != after.get(p)])
with gzip.open(OUT / 'figure-tests-state.json.gz', 'wt') as file:
    json.dump(record, file, indent=2)
print((OUT / 'figure-tests.log').read_text()[-1200:])
assert result.returncode == 0 and not record['changed_inputs']
