#!/usr/bin/env python3
"""Pin L0 normative bytes while allowing only canonical code-hash metadata repair."""
import hashlib, importlib.util, json, re
from pathlib import Path
s=importlib.util.spec_from_file_location('ab',Path(__file__).with_name('p1310-ab-suite.py'))
ab=importlib.util.module_from_spec(s); s.loader.exec_module(ab)
frozen=json.loads((ab.D/'p1310-ab-frozen-r1.json').read_text())
path=ab.ROOT/frozen['L0_path']; data=path.read_bytes()
assert ab.digest(data)==frozen['L0_sha256']=='7d8ac8c262f21a328f30df173f18fed36061b0af52025403897de4d8b5ac049b'
normalized,n=re.subn(rb'^Hash do C\xc3\xb3digo: [0-9a-f]{8}\n',b'',data,flags=re.M)
assert n==1
ab.save('l0-pin',dict(at=ab.now(),frozen_sha256=ab.sha(ab.D/'p1310-ab-frozen-r1.json'),path=frozen['L0_path'],raw_sha256=ab.digest(data),normalized_sha256=ab.digest(normalized),normalization='Remove exactly one canonical Hash do Código: eight lowercase hex digits line; preserve every other byte.',raw_text=data.decode(),candidate_source_read=False))
print(ab.digest(normalized))
