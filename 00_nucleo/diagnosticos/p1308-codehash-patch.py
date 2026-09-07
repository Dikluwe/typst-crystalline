"""Propose only reciprocal code-hash metadata; reject normative drift."""
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
manifest = json.loads((ROOT / '00_nucleo/diagnosticos/p1308-manifest.json').read_text())
contracts = {c['path']: c['semantic_sha256'] for c in manifest['contracts']}
successor = json.loads((ROOT / '00_nucleo/diagnosticos/p1308-manifest-successor-1.json').read_text())['successor_L0']
contracts[successor['path']] = successor['semantic_sha256']
paths = subprocess.check_output(['git', 'diff', 'HEAD', '--name-only'], cwd=ROOT, text=True).splitlines()
patch = '*** Begin Patch\n'
changes = []
for name in paths:
    if not name.endswith('.rs'):
        continue
    data = (ROOT / name).read_bytes()
    header = re.search(rb'(?m)^//! @prompt (.+)$', data)
    if not header or header.group(1).decode().strip() not in contracts:
        continue
    owner = header.group(1).decode().strip()
    spec = (ROOT / owner).read_bytes()
    semantic = re.sub(rb'(?m)^Hash do C\xc3\xb3digo: [^\n]*$', b'Hash do Codigo: <metadata>', spec)
    assert hashlib.sha256(semantic).hexdigest() == contracts[owner], owner
    clean, count = re.subn(rb'(?m)^//! @prompt-hash [^\n]*\n', b'', data)
    assert count == 1, name
    digest = hashlib.sha256(clean).hexdigest()[:8]
    old = re.search(r'(?m)^Hash do Código: (\S+)$', spec.decode()).group(1)
    if old != digest:
        patch += f'*** Update File: {ROOT / owner}\n@@\n-Hash do Código: {old}\n+Hash do Código: {digest}\n'
        changes.append(dict(source=name, owner=owner, before=old, after=digest))
patch += '*** End Patch'
print(json.dumps(dict(patch=patch, changes=changes), ensure_ascii=False))
