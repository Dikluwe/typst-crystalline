"""Commit the frozen review evidence only, preserving the current branch/history."""
from pathlib import Path
import datetime
import hashlib
import importlib.util
import json
import subprocess

ROOT = Path.cwd()
HERE = Path(__file__).resolve().parent
PREFIX = str(HERE.relative_to(ROOT)) + '/'
DIAGNOSTIC = '00_nucleo/diagnostics/set-page-header-2026-10-21.md'
EXPECTED_HEAD = '63b7b96b99109b61655b373dc0d248beb7838f86'
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
branch = subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip()
assert head == EXPECTED_HEAD and branch == 'codex/tekt-mature-layout'
assert subprocess.run(['git', 'diff', '--cached', '--quiet']).returncode == 0
changed = subprocess.check_output(['git', 'diff', 'HEAD', '--name-only'], text=True).splitlines()
assert all(p == DIAGNOSTIC or p.startswith(PREFIX) for p in changed)
spec = importlib.util.spec_from_file_location('validation', ROOT / 'lab/tools/ci/validate_layout_migration.py')
validation = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validation)
inputs = validation.frozen_inputs()
assert inputs == json.loads((HERE / 'baseline-build/receipt.json').read_text())['inputs']
checks = json.loads((HERE / 'review-checks-final/receipt.json').read_text())
assert checks['changed_inputs'] == [] and all(c['exit'] == 0 for c in checks['commands'])
assert checks['complete_lint_multiset_unchanged'] and checks['history_unchanged']
assert all(sha(ROOT / item['resolved']) == item['sha256'] for item in checks['links'])
excluded = {'evidence-manifest.json', 'post-commit.json'}
files = sorted(p for p in HERE.rglob('*') if p.is_file()
               and '__pycache__' not in p.parts and p.name not in excluded)
files.append(ROOT / DIAGNOSTIC)
assert all(not p.is_symlink() for p in files)
manifest = {'utc': utc(), 'parent_head': head, 'branch': branch,
            'status': 'review committed; productive repair pending exact ADR-0127 approval',
            'source_inputs_unchanged': True,
            'files': {str(p.relative_to(ROOT)): sha(p) for p in sorted(files)}}
path = HERE / 'evidence-manifest.json'
assert not path.exists()
path.write_text(json.dumps(manifest, indent=2) + '\n')
paths = sorted([*manifest['files'], str(path.relative_to(ROOT))])
subprocess.run(['git', 'add', '-f', '--', *paths], check=True)
staged = subprocess.check_output(['git', 'diff', '--cached', '--name-only', '-z']).decode().strip('\0').split('\0')
assert set(staged) == set(paths)
assert all(sha(ROOT / p) == h for p, h in manifest['files'].items())
subprocess.run(['git', 'commit', '-m', 'docs: specify SetPage physical occurrence repair'], check=True)
new_head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
assert subprocess.check_output(['git', 'rev-parse', 'HEAD^'], text=True).strip() == head
assert subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip() == branch
assert validation.frozen_inputs() == inputs
for p, h in manifest['files'].items():
    blob = subprocess.check_output(['git', 'show', new_head + ':' + p])
    assert hashlib.sha256(blob).hexdigest() == h
record = {'utc': utc(), 'parent': head, 'commit': new_head, 'branch': branch,
          'manifest_sha256': sha(path), 'committed_paths': len(paths),
          'all_committed_bytes_match_manifest': True,
          'productive_inputs_unchanged': True,
          'status': subprocess.check_output(['git', 'status', '--short'], text=True),
          'qualification': 'This local post-commit receipt is intentionally outside its own commit.'}
(HERE / 'post-commit.json').write_text(json.dumps(record, indent=2) + '\n')
assert record['status'] == ''
print(json.dumps({k: record[k] for k in ('commit', 'branch', 'committed_paths', 'status')}))
