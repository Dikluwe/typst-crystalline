"""Record the owner's approval before the integration L0 amendments."""
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
receipt_path = diagnostics / 'p1339-where-l0-gate-receipt.json'
receipt = json.loads(receipt_path.read_text())
assert all(sha(root / p) == h for p, h in receipt['l0_sha256'].items())
assert all(sha(root / p) == h for p, h in receipt['unchanged_consumers_sha256'].items())
data = {
    'recorded_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'user_message': 'Autorizo',
    'approval_context': 'Approval of the immediately preceding Element + NativeElement proposal and continuation of integration under the remaining P1339 gates.',
    'approved_variants': ['entities::selector::Selector::Element { function: Func, fields: EcoVec<(EcoString, Value)> }', 'entities::show::Selector::NativeElement(Func)'],
    'approved_l0_sha256': receipt['l0_sha256'],
    'gate_receipt_sha256': sha(receipt_path),
    'head': git('rev-parse', 'HEAD').strip(),
    'status': git('status', '--short'),
    'diff_stat': git('diff', 'HEAD', '--stat'),
    'limits': ['No additional public fields, variants, traits or signatures approved.', 'No waiver of contract, mutation, RED, scope or final verification gates.', 'No implementation or seal is attested by this approval record.'],
    'regime': 'executed without isolation attestation',
    'recorder_sha256': sha(Path(__file__)),
}
target = diagnostics / 'p1339-where-approval.json'
assert not target.exists(), target
body = json.dumps(data, ensure_ascii=False, indent=2) + '\n'
patch = '*** Begin Patch\n*** Add File: ' + str(target) + '\n' + ''.join('+' + line + '\n' for line in body.splitlines()) + '*** End Patch\n'
subprocess.run(['apply_patch'], cwd=root, input=patch, text=True, check=True)
print(sha(target))
