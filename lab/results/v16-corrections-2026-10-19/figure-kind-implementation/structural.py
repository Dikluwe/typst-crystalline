"""Check domain extension against full baseline/candidate copied workspaces."""
from pathlib import Path
import datetime
import gzip
import hashlib
import importlib.util
import json
import os
import shutil
import subprocess
import sys

ROOT = Path.cwd()
OUT = Path(__file__).resolve().parent
mode = sys.argv[1]
assert mode in ('baseline', 'candidate')
suffix = ('-' + sys.argv[2]) if len(sys.argv) > 2 else ''
stage = ROOT / ('lab/scratch/figure-kind-structural-' + mode + suffix)
result = OUT / ('structural-' + mode + suffix)
assert not stage.exists() and not result.exists()
result.mkdir()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
for top in ('01_core', '02_shell', '03_infra', '04_wiring', 'lab/benches', 'lab/corpus', 'lab/parity/corpus', 'lab/typst-original/tests'):
    shutil.copytree(ROOT / top, stage / top)
for name in ('Cargo.toml', 'Cargo.lock', 'clippy.toml', 'rustfmt.toml', 'rust-toolchain.toml', '.cargo/config.toml'):
    path = stage / name
    path.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(ROOT / name, path)
if mode == 'baseline':
    with gzip.open(OUT / 'baseline-inputs.json.gz', 'rt') as file:
        baseline = json.load(file)
    for top in ('01_core', '02_shell', '03_infra', '04_wiring'):
        for file in (stage / top).rglob('*.rs'):
            if str(file.relative_to(stage)) not in baseline['files']:
                file.unlink()
    for name, text in baseline['files'].items():
        if not name.startswith('00_nucleo/'):
            path = stage / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text)
    targets = [{'file': '01_core/src/compiler/stdlib/figure_image.rs',
                'after': 'fn infer_kind_from_body(', 'anchor': 'match body {'}]
    domains = [('01_core/src/entities/content.rs', 'pub enum Content {')]
else:
    targets = json.loads((OUT / 'structural-targets.json').read_text())
    domains = [('01_core/src/entities/content.rs', 'pub enum Content {'),
               ('01_core/src/entities/value.rs', 'pub enum Value {'),
               ('01_core/src/entities/elements/figure.rs', 'pub enum FigureKind {'),
               ('01_core/src/entities/elements/grid.rs', 'pub enum GridChildOrder {')]
original = {str(p.relative_to(stage)): p.read_text() for top in ('01_core', '02_shell', '03_infra', '04_wiring') for p in (stage / top).rglob('*.rs')}
with gzip.open(result / 'original-sources.json.gz', 'wt') as file:
    json.dump(original, file)
inputs = {str(p.relative_to(stage)): sha(p) for p in stage.rglob('*') if p.is_file()}
for name, declaration in domains:
    path = stage / name
    text = path.read_text()
    position = text.index('\n}', text.index(declaration))
    path.write_text(text[:position] + '\n    TektV16EvolutionProbe,' + text[position:])
for target in targets:
    text = (stage / target['file']).read_text()
    start = text.index(target['after']) if target.get('after') else 0
    pos = text.index(target['anchor'], start)
    target.update(line=text[:pos].count('\n') + 1,
                  byte_start=len(text[:pos].encode()),
                  byte_end=len(text[:pos + len(target['anchor'])].encode()))
mutations = {p: (stage / p).read_text() for p, text in original.items() if (stage / p).read_text() != text}
with gzip.open(result / 'mutations.json.gz', 'wt') as file:
    json.dump(mutations, file)
env = dict(os.environ, CARGO_BUILD_JOBS='1', CARGO_PROFILE_DEV_DEBUG='0',
           CARGO_PROFILE_TEST_DEBUG='0', CARGO_INCREMENTAL='0',
           CARGO_TARGET_DIR=str(ROOT / 'lab/target/product'))
command = ['cargo', 'check', '--offline', '--locked', '-p', 'typst-core', '--lib', '--message-format=json']
record = {'utc_start': utc(), 'mode': mode,
          'baseline_head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
          'cwd': str(stage), 'command': command, 'inputs': inputs,
          'worker_sha256': sha(Path(__file__)), 'targets': targets}
with (result / 'compiler.log').open('w') as log:
    run = subprocess.run(command, cwd=stage, env=env, stdout=log, stderr=subprocess.STDOUT)
errors = []
for line in (result / 'compiler.log').read_text().splitlines():
    try:
        value = json.loads(line)
    except ValueError:
        continue
    if value.get('reason') == 'compiler-message' and value['message']['level'] == 'error':
        errors.append(value['message'])
for target in targets:
    target['witnesses'] = [error for error in errors if (error.get('code') or {}).get('code') == 'E0004' and any(
        span['is_primary'] and span['file_name'].removeprefix(str(stage) + '/') == target['file']
        and span['line_start'] == target['line'] and target['byte_start'] <= span['byte_start'] < target['byte_end']
        for span in error['spans'])]
record.update(utc_end=utc(), exit=run.returncode,
              selected_e0004=sum(bool(t['witnesses']) for t in targets),
              other_errors=[e for e in errors if (e.get('code') or {}).get('code') != 'E0004'],
              error_count=len(errors))
(result / 'receipt.json').write_text(json.dumps(record, indent=2) + '\n')
assert run.returncode == 101 and errors and not record['other_errors']
assert record['selected_e0004'] == (0 if mode == 'baseline' else len(targets))
print('PASS', mode, 'exact selected E0004', record['selected_e0004'])
