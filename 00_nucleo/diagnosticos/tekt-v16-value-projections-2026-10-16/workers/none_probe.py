from pathlib import Path
import tempfile
import subprocess
import json
import datetime
import hashlib

out = Path('00_nucleo/diagnosticos/tekt-v16-value-projections-2026-10-16')
source = 'enum Mode { A, B }\nfn classify(v: Option<Mode>) -> u8 {\n    match v {\n        Some(Mode::A) => 0,\n        Some(Mode::B) => 1,\n        None => 2,\n    }\n}\n'
record = {'start': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'source': source, 'commands': []}
with tempfile.TemporaryDirectory(prefix='tekt-none-probe-') as temp:
    root = Path(temp)
    (root / '01_core').mkdir()
    path = root / '01_core/case.rs'
    path.write_text(source)
    for label, command in [
        ('rustc', ['rustc', '--edition=2021', '--crate-type', 'lib', '-Dunreachable_patterns', str(path), '-o', str(root/'case.rlib')]),
        ('lint', ['/tmp/tekt-linter-update-target/debug/crystalline-lint', str(root), '--checks', 'v16', '--fail-on', 'warning', '--format', 'sarif']),
    ]:
        result = subprocess.run(command, capture_output=True, text=True)
        record['commands'].append({'label': label, 'command': command, 'exit': result.returncode,
                                   'stdout': result.stdout, 'stderr': result.stderr})
    assert record['commands'][0]['exit'] == 0
    diagnostic = json.loads(record['commands'][1]['stdout'])['runs'][0]['results']
    assert len(diagnostic) == 1 and diagnostic[0]['ruleId'] == 'V16'
    assert diagnostic[0]['locations'][0]['physicalLocation']['region']['startLine'] == 6
    path.write_text(source.replace('A, B', 'A, B, Future'))
    command = ['rustc', '--edition=2021', '--crate-type', 'lib', '-Dunreachable_patterns', str(path), '-o', str(root/'future.rlib')]
    result = subprocess.run(command, capture_output=True, text=True)
    assert result.returncode != 0 and 'E0004' in result.stderr
    record['commands'].append({'label':'future', 'command':command, 'exit':result.returncode,
                               'stdout': result.stdout, 'stderr': result.stderr})
record.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),
              explicit_none_is_reported_as_wildcard=True, compiler_already_requires_future_variant=True,
              conclusion='V16 falso positivo sobre None explícito; parser deve distinguir variante de binding. Nenhum consumer produtivo alterado para esconder a emissão.')
(out / 'none-probe-result.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n')
print(json.dumps({'None_false_positive_reproduced':True,'future_already_exhaustive':True}))
