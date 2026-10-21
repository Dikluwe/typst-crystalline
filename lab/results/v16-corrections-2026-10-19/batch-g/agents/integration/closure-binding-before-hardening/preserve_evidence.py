"""Prepare an exact, reviewed Git manifest; keep compiled probes local to lab."""
from pathlib import Path
import datetime
import hashlib
import json
import subprocess

R = Path.cwd()
P = Path(__file__).resolve().parent
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert (P / 'closure-checks/receipt.json').is_file()
assert not (P / 'evidence-manifest.json').exists()
scope = json.loads((P / 'scope.json').read_text())
assert subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip() == scope['baseline_head']
assert not subprocess.check_output(['git', 'diff', '--cached', '--name-only'], text=True).strip()
files = {}
omitted = []
for p in sorted(P.rglob('*')):
    if not p.is_file():
        continue
    assert not p.is_symlink(), p
    reason = None
    if '__pycache__' in p.parts or p.suffix == '.pyc':
        reason = 'Reproducible Python bytecode cache; source scripts are preserved.'
    else:
        with p.open('rb') as f:
            if f.read(4) == b'\x7fELF':
                reason = 'Compiled local probe, retained under lab; exact SHA/recipe/runtime output are preserved in its receipt.'
    name = str(p.relative_to(R))
    item = {'sha256': sha(p), 'bytes': p.stat().st_size}
    if reason:
        omitted.append({'path': name, **item, 'reason': reason})
    else:
        files[name] = item
for name in scope['sources'] + scope['owners'] + ['00_nucleo/diagnostics/v16-correction-progress-2026-10-19.md']:
    p = R / name
    files[name] = {'sha256': sha(p), 'bytes': p.stat().st_size}
manifest = {'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
            'head': scope['baseline_head'], 'branch': scope['branch'],
            'policy': 'Exact additive local commit; no history rewrite. This manifest is additionally staged and cannot contain its own digest. No blanket ignored-directory add.',
            'files': files, 'omitted_local_artifacts': omitted,
            'diff_head_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True)}
(P / 'evidence-manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
print('Manifest files', len(files), 'bytes', sum(x['bytes'] for x in files.values()),
      'omitted local artifacts', len(omitted), 'manifest SHA', sha(P / 'evidence-manifest.json'))
