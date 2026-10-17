from pathlib import Path
import json
import gzip
import re
import hashlib
import datetime

out = Path('00_nucleo/diagnosticos/tekt-v16-show-selectors-2026-10-16')
before = json.load(gzip.open(out / 'before.json.gz'))
baseline = json.loads((out / 'baseline-result.json').read_text())
materialization = json.loads((out / 'materialization.json').read_text())
scope = json.loads((out / 'scope.json').read_text())
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
inventory = {str(p) for top in ['01_core', '02_shell', '03_infra', '04_wiring', 'benches', '00_nucleo/prompts']
             for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs', '.md', '.toml']}
inventory.update(['Cargo.toml', 'Cargo.lock', 'crystalline.toml', 'rustfmt.toml', 'clippy.toml',
                  '.cargo/config.toml', 'rust-toolchain.toml', '.github/workflows/ci.yml', 'tools/ci/README.md'])
assert inventory == set(before['hashes']), 'productive inventory changed'
normalize_source = lambda t: re.sub(r'^//! @prompt-hash [0-9a-f]{8}\n', '', t, count=1, flags=re.M)
normalize_owner = lambda t: re.sub(r'^Hash do Código: [0-9a-f]{8}\n', '', t, count=1, flags=re.M)
changed = [p for p, h in before['hashes'].items() if sha(p) != h]
assert sorted(changed) == sorted(scope['sources'] + scope['owners']), changed
for path in scope['sources']:
    assert normalize_source(Path(path).read_text()) == normalize_source(materialization['files'][path]['expected_after']), path
for path in scope['owners']:
    assert normalize_owner(Path(path).read_text()) == normalize_owner(baseline['files'][path]), path
assert len(materialization['changes']) == 6
record = {'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'changed': changed,
          'pattern_only_changes_and_prescribed_predicate': True, 'unrelated_sources_unchanged': True,
          'expected_sources_match': True, 'owner_bodies_preserved_after_precode_specification': True}
(out / 'code-guard.json').write_text(json.dumps(record, indent=2) + '\n')
print(json.dumps(record))
