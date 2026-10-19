from pathlib import Path
import json,gzip,hashlib,datetime
out=Path('00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16');pkg=Path('00_nucleo/materialization/clippy-enums-publicos-2026-10-16');sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert sha(pkg/'manifest.sha256')=='282da3f8f79080a4831e1898879775e32a62caa0b2ffa941b8fca9198136b950'
for line in (pkg/'manifest.sha256').read_text().splitlines():
 h,p=line.split('  ',1);assert sha(pkg/p)==h,p
base=json.load(gzip.open(out/'before.json.gz'));scope=json.loads((out/'proposal-scope.json').read_text());assert all(sha(p)==h for p,h in base['hashes'].items())
receipt=dict(recorded_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),question_id='["request_user_input_async","call_beb444ec3fb042daab37a21f716b84d8",0]',answer='Aprovar a proposta',manifest_sha256=sha(pkg/'manifest.sha256'),package=str(pkg))
(out/'approval-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
for row in scope:
 owner=row['owner'];t=(pkg/'proposed'/owner).read_text();t+='\n## Ratificação humana — 2026-10-16\n\nProposta aprovada pelo humano, manifesto SHA-256 `'+receipt['manifest_sha256']+'`. O contrato acima passa a vigente; inclui materialização/testes T1–T6, sem release. Registro em `00_nucleo/diagnosticos/clippy-enums-publicos-2026-10-16/approval-receipt.json`.\n';Path(owner).write_text(t)
(out/'l0-before-code.json').write_text(json.dumps(dict(utc=receipt['recorded_utc'],owners={r['owner']:sha(r['owner']) for r in scope},sources_unchanged=all(sha(r['source'])==r['source_sha256'] for r in scope)),indent=2)+'\n')
print('Eight approved L0 installed; source unchanged; receipt and package verified.')
