from pathlib import Path
import json,subprocess,datetime
out=Path('00_nucleo/diagnosticos/clippy-default-2026-10-16')
red=json.loads((out/'red-details.json').read_text());assert len(red['commands'])==2
assert all(c['exit']==101 for c in red['commands'])
pkg=Path('00_nucleo/materialization/clippy-default-2026-10-16')
for item in json.loads((pkg/'proposal.json').read_text())['public_default_implementations']:
 p=Path(item['source']);s=p.read_text();name=item['type'];anchor=f'impl {name} {{'
 addition=f'impl std::default::Default for {name} {{\n    fn default() -> Self {{\n        Self::new()\n    }}\n}}\n\n'
 assert s.count(anchor)==1 and addition not in s;p.write_text(s.replace(anchor,addition+anchor,1))
sources=['01_core/src/compiler/eval/mod.rs','01_core/src/compiler/eval/tests.rs','03_infra/src/world/font_session.rs','03_infra/src/world/source_session.rs']
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true',*sources],check=True)
(out/'materialization.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),approved_manifest=json.loads((pkg/'approval-receipt.json').read_text())['manifest_sha256'],sources=sources),indent=2)+'\n')
print('Três impls Default delegando a Self::new; quatro consumers formatados.')
