from pathlib import Path
import tempfile
import shutil
import subprocess
import os
import json
import gzip
import hashlib
import datetime
import re
import sys

root = Path.cwd()
out = root / '00_nucleo/diagnosticos/tekt-v16-math-value-2026-10-16'
phase = sys.argv[1]
assert phase in ['baseline', 'green']
rows = json.loads((out / 'scope.json').read_text())['targets']
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
now = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
before = json.load(gzip.open(out / 'before.json.gz'))
hashes = {p: sha(p) for p in before['hashes']}
record = {'phase': phase, 'start': now(), 'sources': hashes, 'targets': []}
with tempfile.TemporaryDirectory(prefix='tekt-math-value-probe-') as temp:
    stage = Path(temp)
    for top in ['01_core', '02_shell', '03_infra', '04_wiring', 'benches']:
        shutil.copytree(root / top, stage / top, ignore=shutil.ignore_patterns('target', '.git', '__pycache__'))
    for name in ['Cargo.toml', 'Cargo.lock', 'clippy.toml', 'rustfmt.toml', 'rust-toolchain.toml', '.cargo/config.toml']:
        dest = stage / name
        dest.parent.mkdir(exist_ok=True, parents=True)
        shutil.copyfile(root / name, dest)
    definition = stage / '01_core/src/entities/value.rs'
    text = definition.read_text()
    assert text.count('pub enum Value {') == 1
    definition.write_text(text.replace('pub enum Value {', 'pub enum Value {\n    TektMathValueProbe,'))
    inputs = {'01_core/src/entities/value.rs': definition.read_text()}
    for row in rows:
        text = (stage / row['source']).read_text()
        matches = list(re.finditer(r'^\s*(?:pub(?:\([^)]*\))? )?fn ' + row['function'] + r'\(', text, re.M))
        assert len(matches) == 1, row
        start = matches[0].start()
        end = re.search(r'^\s*(?:pub(?:\([^)]*\))? )?fn |^#\[cfg\(test\)\]', text[matches[0].end():], re.M)
        end_byte = matches[0].end() + end.start() if end else len(text)
        record['targets'].append({'id': row['id'], 'source': row['source'], 'function': row['function'],
                                  'start_line': text[:start].count('\n') + 1,
                                  'end_line': text[:end_byte].count('\n') + 1})
        inputs[row['source']] = text
    record['workspace'] = str(stage)
    record['command'] = ['cargo', 'check', '--offline', '--locked', '-p', 'typst-core', '--message-format=json']
    record['environment'] = {'CARGO_TARGET_DIR': str(root / 'target'), 'CARGO_BUILD_JOBS': '1'}
    with gzip.open(out / ('probe-' + phase + '-input.json.gz'), 'wt') as f:
        json.dump({'record': record, 'mutated_sources': inputs}, f)
    env = dict(os.environ, **record['environment'])
    with (out / ('probe-' + phase + '.log')).open('w') as log:
        result = subprocess.run(record['command'], cwd=stage, env=env, stdout=log, stderr=subprocess.STDOUT)
    errors = []
    for line in (out / ('probe-' + phase + '.log')).read_text().splitlines():
        try:
            item = json.loads(line)
        except ValueError:
            continue
        if item.get('reason') == 'compiler-message' and item['message']['level'] == 'error':
            errors.append(item['message'])
    for target in record['targets']:
        target['e0004'] = any((error.get('code') or {}).get('code') == 'E0004'
            and any(span['is_primary'] and span['file_name'].removeprefix(str(stage) + '/') == target['source']
                    and target['start_line'] <= span['line_start'] < target['end_line'] for span in error['spans'])
            for error in errors)
    record.update(end=now(), exit=result.returncode, total_e0004=sum((e.get('code') or {}).get('code') == 'E0004' for e in errors),
                  selected_e0004=sum(x['e0004'] for x in record['targets']),
                  other_errors=[e for e in errors if (e.get('code') or {}).get('code') != 'E0004'],
                  productive_sources_unchanged=all(sha(p) == h for p, h in hashes.items()))
    (out / ('probe-' + phase + '-result.json')).write_text(json.dumps(record, indent=2) + '\n')
    print(json.dumps({k: record[k] for k in ['phase', 'exit', 'selected_e0004', 'total_e0004', 'productive_sources_unchanged']}), flush=True)
    assert result.returncode != 0 and not record['other_errors'] and record['productive_sources_unchanged']
    assert record['selected_e0004'] == (0 if phase == 'baseline' else 4), record['targets']
