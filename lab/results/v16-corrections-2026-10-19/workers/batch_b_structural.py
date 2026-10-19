"""Attributable batch-B closure experiments in complete disposable workspaces.

Run serially through the resource-limited coordinator. No production mutation.
The baseline contains new controls and unchanged original observer decisions.
"""
from pathlib import Path
import datetime
import gzip
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys

ROOT = Path.cwd()
OUT = ROOT / 'lab/results/v16-corrections-2026-10-19/batch-b'
PHASE = sys.argv[1]
assert PHASE in ('baseline', 'green')
BASE = ROOT / 'lab/scratch/v16-batch-b-base-2026-10-19' if PHASE == 'baseline' else ROOT
STAGE = ROOT / ('lab/scratch/v16-batch-b-structural-' + PHASE)
RESULT = OUT / ('structural-' + PHASE)
RESULT.mkdir(parents=True, exist_ok=False)
SENTINEL = 'TektV16EvolutionProbe'
ENUM = '01_core/src/entities/content.rs'
PAIR = '01_core/src/entities/content/traversal.rs'
INFRA = '03_infra/src/integration_tests.rs'
TARGETS = {
    'V16-013': ('01_core/src/compiler/eval/math_cases_evaluation_tests.rs', 'find_cases'),
    'V16-014': ('01_core/src/compiler/eval/math_frac_evaluation_tests.rs', 'find_frac'),
    'V16-015': ('01_core/src/compiler/eval/math_matrix_evaluation_tests.rs', 'find_mat'),
    'V16-016': ('01_core/src/compiler/eval/math_root_evaluation_tests.rs', 'find_root'),
    'V16-025': ('01_core/src/compiler/eval/rules.rs', 'collect_custom_into'),
    'V16-075': ('01_core/src/entities/content/tests/traversal.rs', 'map_content_bottom_up_pai_ve_filhos_transformados'),
    'V16-076': (PAIR, 'same_storage'),
}


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def gz(path, data):
    with gzip.open(path, 'wt') as f:
        json.dump(data, f)


def locate(path, function):
    text = (STAGE / path).read_text()
    start = re.search(r'\bfn ' + re.escape(function) + r'\b', text)
    assert start, function
    pos = text.index('match ', start.start())
    end = text.index('{', pos) + 1
    return {'file': path, 'function': function, 'line': text[:pos].count('\n') + 1,
            'byte_start': len(text[:pos].encode()), 'byte_end': len(text[:end].encode()),
            'anchor': text[pos:end]}


scope = json.loads((OUT / 'scope.json').read_text())
frozen = {p: sha(BASE / p) for p in scope['sources'] + scope['owners']}
for top in ['01_core', '02_shell', '03_infra', '04_wiring', 'lab/benches', 'lab/corpus', 'lab/parity/corpus', 'lab/typst-original/tests']:
    shutil.copytree(BASE / top, STAGE / top, dirs_exist_ok=True)
for path in ['Cargo.toml', 'Cargo.lock', 'clippy.toml', 'rustfmt.toml', 'rust-toolchain.toml', '.cargo/config.toml']:
    dest = STAGE / path
    dest.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(BASE / path, dest)
original = {str(p.relative_to(STAGE)): p.read_text() for top in ['01_core', '02_shell', '03_infra', '04_wiring'] for p in (STAGE / top).rglob('*.rs')}
# Preserve complete original compilation sources, not extracted matches or stubs.
gz(RESULT / 'original-sources.json.gz', original)
inputs = {str(p.relative_to(STAGE)): sha(p) for p in STAGE.rglob('*') if p.is_file()}
gz(RESULT / 'inputs.json.gz', {'utc': now(), 'base': str(BASE), 'inputs': inputs, 'frozen_owners': frozen})
enum_text = original[ENUM]
decl_start = enum_text.index('pub enum Content {')
decl_end = enum_text.index('\n}', decl_start)
variants = re.findall(r'^    (\w+)(\(|\s*\{|,)', enum_text[decl_start:decl_end], re.M)
assert len(variants) == 120, variants
patterns = ['Content::' + name + ('(..)' if shape == '(' else ' { .. }' if '{' in shape else '') for name, shape in variants]
old = ' | '.join(patterns)
pair_insertions = {
    'new-old': f'(_, Content::{SENTINEL}) => false,',
    'old-new': f'(Content::{SENTINEL}, _) => false,',
    'new-new': f'(Content::{SENTINEL}, {old}) | ({old}, Content::{SENTINEL}) => false,',
}
jobs = [('core-domain', list(TARGETS), None)]
jobs += [(name, ['V16-076'], insertion) for name, insertion in pair_insertions.items()]
jobs += [('infra-omission', ['V16-091'], None)]
records = []
for name, ids, insertion in jobs:
    for path, text in original.items():
        (STAGE / path).write_text(text)
    if name != 'infra-omission':
        (STAGE / ENUM).write_text(enum_text[:decl_end] + f'\n    {SENTINEL},' + enum_text[decl_end:])
        if insertion:
            p = STAGE / PAIR
            text = p.read_text()
            start = text.index('fn same_storage(')
            pos = text.index('match (a, b) {', start) + len('match (a, b) {')
            p.write_text(text[:pos] + '\n' + insertion + text[pos:])
        targets = [dict(id=key, **locate(*TARGETS[key])) for key in ids]
        package = 'typst-core'
    else:
        p = STAGE / INFRA
        text = p.read_text()
        # The extracted shared observer name is recorded by its owner agent.
        function = json.loads((OUT / 'infra-observer.json').read_text())['function']
        start = text.index('fn ' + function)
        end = text.index('\n    }', start)
        section = text[start:end]
        section, count = re.subn(r'^\s*Content::Styled\(styled\) => [^\n]+,\n', '', section, count=1, flags=re.M)
        assert count == 1, 'exact positive Styled arm must exist once'
        p.write_text(text[:start] + section + text[end:])
        targets = [dict(id='V16-091', **locate(INFRA, function))]
        package = 'typst-infra'
    cmd = ['cargo', 'test', '--offline', '--locked', '-p', package, '--lib', '--no-run', '--message-format=json']
    env = dict(os.environ, CARGO_BUILD_JOBS='1', CARGO_PROFILE_DEV_DEBUG='0', CARGO_PROFILE_TEST_DEBUG='0', CARGO_INCREMENTAL='0', CARGO_TARGET_DIR=str(ROOT / 'lab/target/product'))
    changes = {path: (STAGE / path).read_text() for path, text in original.items() if text != (STAGE / path).read_text()}
    gz(RESULT / (name + '-mutations.json.gz'), changes)
    rec = {'name': name, 'phase': PHASE, 'utc_start': now(), 'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(), 'cwd': str(STAGE), 'command': cmd, 'targets': targets, 'mutated_paths': sorted(changes), 'environment': {k: env[k] for k in ['CARGO_BUILD_JOBS', 'CARGO_PROFILE_DEV_DEBUG', 'CARGO_PROFILE_TEST_DEBUG', 'CARGO_INCREMENTAL', 'CARGO_TARGET_DIR']}}
    print(now(), name, 'START', flush=True)
    with (RESULT / (name + '.log')).open('w') as f:
        run = subprocess.run(cmd, cwd=STAGE, env=env, stdout=f, stderr=subprocess.STDOUT)
    errors = []
    for line in (RESULT / (name + '.log')).read_text().splitlines():
        try:
            item = json.loads(line)
        except ValueError:
            continue
        if item.get('reason') == 'compiler-message' and item['message']['level'] == 'error':
            errors.append(item['message'])
    for target in targets:
        target['diagnostics'] = [e for e in errors if (e.get('code') or {}).get('code') == 'E0004' and any(s['is_primary'] and s['file_name'].removeprefix(str(STAGE) + '/') == target['file'] and s['line_start'] == target['line'] and target['byte_start'] <= s['byte_start'] < target['byte_end'] for s in e['spans'])]
    rec.update(utc_end=now(), exit=run.returncode, selected_e0004=sum(bool(t['diagnostics']) for t in targets), other_errors=[e for e in errors if (e.get('code') or {}).get('code') != 'E0004'], error_count=len(errors))
    records.append(rec)
    (RESULT / 'results.json').write_text(json.dumps(records, indent=2) + '\n')
    print(now(), name, 'END', run.returncode, 'selected', rec['selected_e0004'], 'other', len(rec['other_errors']), flush=True)
    assert not rec['other_errors'], rec['other_errors']
    assert rec['selected_e0004'] == (0 if PHASE == 'baseline' else len(targets)), targets
    assert run.returncode == (0 if PHASE == 'baseline' and name == 'infra-omission' else 101)
    if insertion and PHASE == 'green':
        messages = [e['message'] for e in targets[0]['diagnostics']]
        assert all(SENTINEL in m for m in messages), messages
        # Isolating arms cover the other pair classes; compiler witness is local.
        rec['isolated_axis'] = name
changed = [p for p, value in frozen.items() if sha(BASE / p) != value]
(RESULT / 'freeze.json').write_text(json.dumps({'utc': now(), 'changed': changed}, indent=2) + '\n')
assert not changed, changed
print('PASS', PHASE, '8 decisions plus three isolated storage pair classes', flush=True)
