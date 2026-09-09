"""Transport-only adaptation: preserve frozen corpus and predicates verbatim.

The full final receipt exceeds the OS per-argument limit. Deliver exactly the
same apply_patch text through stdin; do not edit the frozen runner.
"""
import json, subprocess
from importlib.machinery import SourceFileLoader
runner = SourceFileLoader('p1326_ab_cli', '00_nucleo/diagnosticos/p1326-ab-cli.py').load_module()
assert runner.sha('00_nucleo/diagnosticos/p1326-ab-cli.py') == '37b71be5139517054a3e94af584832785f865725c0198f94df2df1a6ef8687b1'
def save_stdin(name, data):
    path = runner.BASE / name
    assert not path.exists(), path
    content = json.dumps(data, ensure_ascii=False, indent=2) + '\n'
    patch = f'*** Begin Patch\n*** Add File: {path}\n' + ''.join('+' + x + '\n' for x in content.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, check=True, capture_output=True)
    return runner.sha(path)
runner.save = save_stdin
runner.main()
