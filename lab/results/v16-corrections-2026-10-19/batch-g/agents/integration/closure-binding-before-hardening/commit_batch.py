"""Commit only the exact validated batch manifest, preserving the current branch."""
from pathlib import Path
import datetime
import hashlib
import json
import subprocess
import tempfile

R = Path.cwd()
P = Path(__file__).resolve().parent
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
m = json.loads((P / 'evidence-manifest.json').read_text())
assert subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip() == m['head']
assert subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip() == m['branch']
assert not subprocess.check_output(['git', 'diff', '--cached', '--name-only'], text=True).strip()
build = json.loads((P / 'candidate-build-3/receipt.json').read_text())
assert all(sha(R / n) == digest for n, digest in build['inputs'].items())
assert all(sha(R / n) == v['sha256'] for n, v in m['files'].items())
paths = sorted(set(m['files']) | {str((P / 'evidence-manifest.json').relative_to(R))})
with tempfile.NamedTemporaryFile(prefix='tekt-batch-g-stage-', dir='/tmp') as f:
    f.write(b'\0'.join(n.encode() for n in paths) + b'\0')
    f.flush()
    subprocess.run(['git', '--literal-pathspecs', 'add', '-f', '--pathspec-from-file=' + f.name, '--pathspec-file-nul'], check=True)
staged = subprocess.check_output(['git', 'diff', '--cached', '--name-only', '-z']).decode().split('\0')
assert set(filter(None, staged)) == set(paths)
scope = json.loads((P / 'scope.json').read_text())
active_paths = scope['sources'] + scope['owners'] + ['00_nucleo/diagnostics/v16-correction-progress-2026-10-19.md']
subprocess.run(['git', 'diff', '--cached', '--check', '--', *active_paths], check=True)
subprocess.run(['git', 'commit', '-m', 'fix: close batch G V16 decisions and preserve causal projections'], check=True)
commit = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
subprocess.run(['git', 'merge-base', '--is-ancestor', m['head'], commit], check=True)
assert subprocess.check_output(['git', 'rev-parse', 'HEAD^'], text=True).strip() == m['head']
assert subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip() == m['branch']
assert not subprocess.check_output(['git', 'status', '--porcelain'], text=True).strip()
for name in paths:
    committed = subprocess.check_output(['git', 'show', commit + ':' + name])
    assert hashlib.sha256(committed).hexdigest() == sha(R / name)
receipt = {'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
           'parent': m['head'], 'commit': commit, 'branch': m['branch'],
           'committed_paths': len(paths), 'clean_status': True, 'ancestry_preserved': True,
           'all_committed_files_equal_validated_working_files': True,
           'manifest_sha256': sha(P / 'evidence-manifest.json'),
           'note': 'Local post-commit receipt is intentionally outside its own commit. The exact dirty-tree test inputs are unchanged; HEAD metadata is not a rebuilt binary identity.'}
(P / 'post-commit.json').write_text(json.dumps(receipt, indent=2) + '\n')
print(json.dumps(receipt, indent=2))
