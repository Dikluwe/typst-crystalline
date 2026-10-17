from pathlib import Path
import subprocess,json,collections
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-origins-2026-10-16');binary='/tmp/tekt-linter-none-target/debug/crystalline-lint'
with (out/'preview-architecture.log').open('w') as log:r=subprocess.run([binary,'.','--format','sarif'],stdout=log,stderr=subprocess.STDOUT)
assert r.returncode==1
rows=json.loads((out/'preview-architecture.log').read_text())['runs'][0]['results'];print(json.dumps(dict(collections.Counter(x['ruleId']+':'+x['level'] for x in rows))),flush=True)
for item in rows:
 if item['ruleId']=='V20' and item['locations'][0]['physicalLocation']['artifactLocation']['uri']=='./01_core/src/compiler/layout/realized.rs':print(json.dumps(item),flush=True)
