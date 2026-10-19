from pathlib import Path
import json,gzip,re,hashlib,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-origins-2026-10-16');before=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));baseline=json.load(open(out/'baseline-result.json'));materialization=json.load(open(out/'materialization.json'))
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
inventory={str(p) for top in ['01_core','02_shell','03_infra','04_wiring','benches','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']};inventory.update(['Cargo.toml','Cargo.lock','crystalline.toml','rustfmt.toml','clippy.toml','.cargo/config.toml','rust-toolchain.toml','.github/workflows/ci.yml','tools/ci/README.md'])
assert inventory==set(before['hashes'])
changed=sorted(p for p,h in before['hashes'].items() if sha(p)!=h);assert changed==sorted(scope['sources']+scope['owners']),changed
normsource=lambda s:re.sub(r'^//! @prompt-hash [0-9a-f]{8}\n','',s,count=1,flags=re.M)
normowner=lambda s:re.sub(r'^Hash do Código: [0-9a-f]{8}\n','',s,count=1,flags=re.M)
for source in scope['sources']:assert normsource(Path(source).read_text())==normsource(materialization['files'][source]['expected_after']),source
expected_owners={p:baseline['files'][p] for p in scope['owners']}
refinement=out/'l0-refinement.json'
if refinement.exists():expected_owners.update(json.load(open(refinement))['owners'])
for owner in scope['owners']:assert normowner(Path(owner).read_text())==normowner(expected_owners[owner]),owner
assert len(materialization['changes'])==6
normal=lambda t:'\n'.join(line for line in t.splitlines() if not line.lstrip().startswith(('//!','///')))
source='01_core/src/compiler/introspect/locatable.rs'
assert normal(Path(source).read_text())==normal(before['files'][source]),'classifier executable code changed'
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'changed':changed,'only_six_prescribed_decisions_and_Source_normalization_changed':True,'tests_preserved_since_baseline':True,'L0_bodies_preserved_since_prescription':True,'unrelated_sources_and_config_unchanged':True}
(out/'code-guard.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
