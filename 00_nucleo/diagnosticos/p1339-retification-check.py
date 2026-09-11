"""Read-only product/antecedent check; emit a new provenance receipt via apply_patch."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parents[2]
diagnostics = root / '00_nucleo/diagnosticos'

def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

def git(*args):
    return subprocess.check_output(['git', *args], cwd=root, text=True)

start = datetime.datetime.now(datetime.timezone.utc).isoformat()
a0 = json.loads((diagnostics / 'p1339-a0.json').read_text())
resume = json.loads((diagnostics / 'p1339-resume-r1.json').read_text())
product_changed = [p for p, h in a0['product_inventory'].items() if sha(root / p) != h]
antecedent_changed = [p for p, h in a0['antecedents'].items() if sha(root / p) != h]
historical_changed = [p for p, h in resume['preserved_inputs'].items() if sha(root / p) != h]
assert not product_changed, product_changed
assert not antecedent_changed, antecedent_changed
assert not historical_changed, historical_changed
assert sha(root / resume['step_after']['path']) == resume['step_after']['sha256']
assert not git('diff', 'HEAD', '--stat')
receipt = dict(
    start=start,
    end=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    role='operator provenance check; not independent seal or verdict',
    regime='executed without isolation attestation',
    head=git('rev-parse', 'HEAD').strip(),
    branch=git('branch', '--show-current').strip(),
    status_short=git('status', '--short'),
    tracked_diff_stat=git('diff', 'HEAD', '--stat'),
    product_inventory_count=len(a0['product_inventory']),
    product_changed=product_changed,
    antecedent_changed=antecedent_changed,
    historical_changed=historical_changed,
    step=resume['step_after'],
    script_sha256=sha(Path(__file__)),
    command='python3 00_nucleo/diagnosticos/p1339-retification-check.py',
    artifact_hashes={str(p.relative_to(root)): sha(p) for p in sorted(diagnostics.glob('p1339-*')) if p.is_file()},
    decision='Await owner clarification of Angle NaN obligation before phase B; no L0/product edits or commit',
)
target = diagnostics / 'p1339-retification-check.json'
assert not target.exists(), target
body = json.dumps(receipt, ensure_ascii=False, indent=2) + '\n'
patch = '*** Begin Patch\n*** Add File: ' + str(target) + '\n' + ''.join('+' + line + '\n' for line in body.splitlines()) + '*** End Patch\n'
subprocess.run(['apply_patch'], input=patch, cwd=root, text=True, check=True)
print(json.dumps(dict(product_inventory_count=receipt['product_inventory_count'], product_changed=product_changed, antecedent_changed=antecedent_changed, historical_changed=historical_changed, receipt_sha256=sha(target))))
