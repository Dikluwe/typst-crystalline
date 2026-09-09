from pathlib import Path
import subprocess
source = Path('00_nucleo/diagnosticos/p1326-ab-tests.rs').read_text()
old = 'Value::Str("{\\\"x\\\":1}".into())'
new = 'Value::Str("{\\n  \\\"x\\\": 1\\n}".into())'
assert source.count(old) == 1
source = source.replace(old, new)
target = '00_nucleo/diagnosticos/p1326-ab-tests-r1.rs'
patch = f'*** Begin Patch\n*** Add File: {target}\n' + ''.join('+'+line+'\n' for line in source.splitlines()) + '*** End Patch\n'
subprocess.run(['apply_patch',patch],check=True)
