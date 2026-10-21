"""Build the candidate binary with an exact immutable-input receipt."""
from pathlib import Path
import argparse
import gzip
import importlib.util
import json
import os
import subprocess

ROOT = Path.cwd()
parser = argparse.ArgumentParser()
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
assert not args.output.exists()
args.output.mkdir(parents=True)
spec = importlib.util.spec_from_file_location('validation', ROOT / 'lab/tools/ci/validate_layout_migration.py')
validation = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validation)
inputs = validation.frozen_inputs()
env = dict(os.environ, CARGO_BUILD_JOBS='1', CARGO_PROFILE_DEV_DEBUG='0',
           CARGO_PROFILE_TEST_DEBUG='0', CARGO_INCREMENTAL='0')
command = ['cargo', 'build', '--offline', '--locked', '--workspace']
record = {'utc_start': validation.utc(), 'head': subprocess.check_output(
    ['git', 'rev-parse', 'HEAD'], text=True).strip(),
    'diff_head_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True),
    'inputs': inputs, 'command': command}
with (args.output / 'build.log').open('w') as log:
    run = subprocess.run(command, env=env, stdout=log, stderr=subprocess.STDOUT)
after = validation.frozen_inputs()
record.update(exit=run.returncode, utc_end=validation.utc(),
              changed_inputs=[p for p in inputs.keys() | after.keys() if inputs.get(p) != after.get(p)])
if run.returncode == 0:
    record['product_sha256'] = validation.digest(ROOT / 'lab/target/product/debug/typst')
(args.output / 'receipt.json').write_text(json.dumps(record, indent=2) + '\n')
print((args.output / 'build.log').read_text()[-7000:])
assert run.returncode == 0 and not record['changed_inputs']
