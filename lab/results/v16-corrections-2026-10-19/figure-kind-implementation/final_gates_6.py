"""Run final real-workspace gates only after the immutable extension probe."""
from pathlib import Path
import importlib.util
import json
import os
import subprocess

ROOT = Path.cwd()
HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('validation', ROOT / 'lab/tools/ci/validate_layout_migration.py')
v = importlib.util.module_from_spec(spec)
spec.loader.exec_module(v)
out = HERE / 'final-gates-6'
assert not out.exists()
out.mkdir()
before = v.frozen_inputs()
state = {'utc_start': v.utc(), 'head': subprocess.check_output(['git','rev-parse','HEAD'], text=True).strip(), 'diff_head_stat': subprocess.check_output(['git','diff','HEAD','--stat'], text=True), 'inputs': before, 'worker_sha256': v.digest(Path(__file__))}
(out / 'state.json').write_text(json.dumps(state, indent=2)+'\n')
commands = [
    ('preflight', ['python3','lab/tools/ci/validate_layout_migration.py','--start-at','clippy','--output',str(HERE/'preflight-final-7')]),
    ('structural', ['python3', str(HERE / 'structural.py'), 'candidate', 'final5']),
    ('clean-product-packages', ['cargo','clean','-p','typst-core','-p','typst-shell','-p','typst-infra','-p','typst-wiring']),
    ('integrated12', ['python3','lab/tools/ci/validate_layout_migration.py','--output',str(HERE/'validation-final-5')]),
    ('cli-acceptance', ['python3',str(HERE/'final_acceptance.py')]),
    ('references', ['python3',str(HERE/'run_reference_acceptance.py'),'--build',str(HERE/'candidate-build-4/receipt.json'),'--output',str(HERE/'reference-corpora-final')]),
]
records = []
for name, command in commands:
    assert before == v.frozen_inputs(), 'Productive inputs changed during final freeze'
    print(v.utc(), 'START', name, flush=True)
    start = v.utc()
    run = subprocess.run(command)
    records.append({'name': name, 'argv':command,'utc_start':start,'utc_end':v.utc(),'exit':run.returncode})
    (out / 'commands.json').write_text(json.dumps(records,indent=2)+'\n')
    print(v.utc(), 'END', name, run.returncode, flush=True)
    assert run.returncode == 0, name
    assert before == v.frozen_inputs(), 'Productive inputs changed during final freeze'
(out / 'freeze.json').write_text(json.dumps({'utc_end':v.utc(),'changed_inputs':[], 'input_count':len(before),'commands_passed':len(records)},indent=2)+'\n')
