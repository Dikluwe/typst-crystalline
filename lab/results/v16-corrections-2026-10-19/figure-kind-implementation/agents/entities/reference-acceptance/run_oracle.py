"""Measure the frozen Ref acceptance sources using the exact ratified oracle."""
from pathlib import Path
import datetime
import hashlib
import json
import subprocess

ROOT = Path.cwd()
BASE = Path(__file__).resolve().parent
OUT = BASE / 'oracle'
OUT.mkdir(exist_ok=False)
BINARY = ROOT / 'lab/typst-original/target/release/typst'
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
assert sha(BINARY) == '9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
matrix = json.loads((BASE / 'matrix.json').read_text())
receipt = {
    'utc_start': utc(),
    'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
    'working_tree': 'uncommitted source freeze; exact git diff HEAD stat captured below',
    'diff_head_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True),
    'oracle_commit': 'bd5ebec285505889e8ff1d4f68804ab147a699f3',
    'oracle_tree': 'cac836c07e4dec2a89219370428292393a3216b5',
    'binary_sha256': sha(BINARY),
    'matrix_sha256': sha(BASE / 'matrix.json'),
    'runner_sha256': sha(Path(__file__)),
    'cases': [],
}
for case in matrix['cases']:
    source = BASE / (case['name'] + '.typ')
    output = OUT / (case['name'] + '.' + case['target'])
    commands = [['compile', '-j', '1', str(source), str(output)]]
    if case['target'] == 'html':
        commands[0] += ['--features', 'html']
    if case['query']:
        commands.append(['query', str(source), '<probe>', '--field', 'value'])
    record = {'name': case['name'], 'goal': case['goal'], 'source_sha256': sha(source),
              'extra_sha256': {p: sha(BASE / p) for p in case['extras']}, 'runs': []}
    for args in commands:
        start = utc()
        argv = [str(BINARY), *args]
        try:
            run = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True, timeout=30)
            result = {'exit': run.returncode, 'stdout': run.stdout, 'stderr': run.stderr}
        except subprocess.TimeoutExpired as error:
            result = {'timeout': True, 'stdout': str(error.stdout), 'stderr': str(error.stderr)}
        record['runs'].append({'utc_start': start, 'utc_end': utc(), 'argv': argv, **result})
    if output.exists():
        record['output_sha256'] = sha(output)
        if case['target'] == 'pdf':
            extract = subprocess.run(['pdftotext', '-layout', str(output), '-'], capture_output=True, text=True)
            record['rendered_text'] = extract.stdout
        else:
            record['rendered_html'] = output.read_text()
    receipt['cases'].append(record)
    print(case['name'], [(r.get('exit'), r.get('timeout', False)) for r in record['runs']], flush=True)
receipt['utc_end'] = utc()
(OUT / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
