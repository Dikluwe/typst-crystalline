"""Compile actual extended domains in complete copied workspaces; never stubs.

Each selected decision needs its own primary E0004 span. Unrelated failures are
retained but cannot discharge that obligation. Baseline and candidate anchors
are separately reviewed when a semantic repair moves the same owner decision.
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
OUT = ROOT / 'lab/results/v16-corrections-2026-10-19/batch-c'
MODE = sys.argv[1]
assert MODE in ('baseline', 'baseline-native', 'green', 'green-final')
PHASE = 'baseline' if MODE.startswith('baseline') else 'green'
BASE = ROOT / 'lab/scratch/v16-batch-c-base-2026-10-19' if PHASE == 'baseline' else ROOT
STAGE = ROOT / ('lab/scratch/v16-batch-c-structural-' + MODE)
RESULT = OUT / ('structural-' + MODE)
RESULT.mkdir(parents=True, exist_ok=False)
SENTINEL = 'TektV16EvolutionProbe'
DOMAINS = {
    'Content': ('01_core/src/entities/content.rs', 'pub enum Content {'),
    'Value': ('01_core/src/entities/value.rs', 'pub enum Value {'),
    'Type': ('01_core/src/entities/value.rs', 'pub enum Type {'),
    'Selector': ('01_core/src/entities/selector.rs', 'pub enum Selector {'),
    'ElementKind': ('01_core/src/entities/element_kind.rs', 'pub enum ElementKind {'),
    'Expr': ('01_core/src/entities/ast/expr.rs', "pub enum Expr<'a> {"),
    'P1293HtmlNative': ('01_core/src/compiler/eval/call_dispatch/call_spans.rs', 'pub(super) enum P1293HtmlNative {'),
}
if MODE != 'baseline':
    DOMAINS['FuncRepr'] = ('01_core/src/entities/func.rs', 'pub(crate) enum FuncRepr {')


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def gz(path, data):
    with gzip.open(path, 'wt') as f:
        json.dump(data, f)


def locate(target):
    spec = target.get(PHASE, target)
    text = (STAGE / target['file']).read_text()
    start = text.index(spec['after']) if spec.get('after') else 0
    pos = text.index(spec['anchor'], start)
    return dict(target, line=text[:pos].count('\n') + 1,
                byte_start=len(text[:pos].encode()),
                byte_end=len(text[:pos + len(spec['anchor'])].encode()))


scope = json.loads((OUT / 'scope.json').read_text())
targets = json.loads((OUT / 'structural-targets.json').read_text())
native_target = {
    'id': 'V16-019-native-identity',
    'file': '01_core/src/compiler/eval/operators/equality.rs',
    'baseline': {'after': 'fn selectors_eq(', 'anchor': 'match (a.native_fn_addr(), b.native_fn_addr()) {'},
    'green': {'after': 'fn selectors_eq(', 'anchor': 'match (a.repr(), b.repr()) {'},
}
if MODE != 'baseline':
    targets.append(native_target)
frozen = {p: sha(BASE / p) for p in scope['sources'] + scope['owners']}
for top in ['01_core', '02_shell', '03_infra', '04_wiring', 'lab/benches', 'lab/corpus', 'lab/parity/corpus', 'lab/typst-original/tests']:
    shutil.copytree(BASE / top, STAGE / top, dirs_exist_ok=True)
for path in ['Cargo.toml', 'Cargo.lock', 'clippy.toml', 'rustfmt.toml', 'rust-toolchain.toml', '.cargo/config.toml']:
    dest = STAGE / path
    dest.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(BASE / path, dest)
original = {str(p.relative_to(STAGE)): p.read_text() for top in ['01_core', '02_shell', '03_infra', '04_wiring'] for p in (STAGE / top).rglob('*.rs')}
gz(RESULT / 'original-sources.json.gz', original)
gz(RESULT / 'inputs.json.gz', {'utc': now(), 'base': str(BASE), 'inputs': {str(p.relative_to(STAGE)): sha(p) for p in STAGE.rglob('*') if p.is_file()}, 'frozen_owners': frozen})
selector_text = original[DOMAINS['Selector'][0]]
start = selector_text.index(DOMAINS['Selector'][1])
end = selector_text.index('\n}', start)
variants = re.findall(r'^    (\w+)(\(|\s*\{|,)', selector_text[start:end], re.M)
assert len(variants) == 12, variants
old = ' | '.join('Selector::' + name + ('(..)' if shape == '(' else ' { .. }' if '{' in shape else '') for name, shape in variants)
pair_insertions = {
    'new-old': f'(_, Selector::{SENTINEL}) => false,',
    'old-new': f'(Selector::{SENTINEL}, _) => false,',
    'new-new': f'(Selector::{SENTINEL}, {old}) | ({old}, Selector::{SENTINEL}) => false,',
}
jobs = [('all-domains', list(DOMAINS), targets, None)]
jobs += [(name, ['Selector'], [t for t in targets if t['id'] == 'V16-019'], insertion) for name, insertion in pair_insertions.items()]
if MODE == 'baseline-native':
    jobs = [('native-domain', ['FuncRepr'], [native_target], None)]
if MODE.startswith('green'):
    text = original[DOMAINS['FuncRepr'][0]]
    start = text.index(DOMAINS['FuncRepr'][1])
    end = text.index('\n}', start)
    variants = re.findall(r'^    (\w+)(\(|\s*\{|,)', text[start:end], re.M)
    assert len(variants) == 7, variants
    old = ' | '.join('crate::entities::func::FuncRepr::' + name + ('(..)' if shape == '(' else ' { .. }' if '{' in shape else '') for name, shape in variants)
    for name, insertion in {
        'native-new-old': f'(_, crate::entities::func::FuncRepr::{SENTINEL}) => false,',
        'native-old-new': f'(crate::entities::func::FuncRepr::{SENTINEL}, _) => false,',
        'native-new-new': f'(crate::entities::func::FuncRepr::{SENTINEL}, {old}) | ({old}, crate::entities::func::FuncRepr::{SENTINEL}) => false,',
    }.items():
        jobs.append((name, ['FuncRepr'], [native_target], insertion))
records = []
for name, domains, selected, insertion in jobs:
    for path, text in original.items():
        (STAGE / path).write_text(text)
    for domain in domains:
        path, declaration = DOMAINS[domain]
        p = STAGE / path
        text = p.read_text()
        pos = text.index('\n}', text.index(declaration))
        p.write_text(text[:pos] + f'\n    {SENTINEL},' + text[pos:])
    if insertion:
        p = STAGE / '01_core/src/compiler/eval/operators/equality.rs'
        text = p.read_text()
        start = text.index('fn selectors_eq(')
        anchor = 'match (a.repr(), b.repr()) {' if name.startswith('native-') else 'match (a, b) {'
        pos = text.index(anchor, start) + len(anchor)
        p.write_text(text[:pos] + '\n' + insertion + text[pos:])
    selected = [locate(t) for t in selected if PHASE in t or 'anchor' in t]
    cmd = ['cargo', 'test', '--offline', '--locked', '-p', 'typst-core', '--lib', '--no-run', '--message-format=json']
    env = dict(os.environ, CARGO_BUILD_JOBS='1', CARGO_PROFILE_DEV_DEBUG='0', CARGO_PROFILE_TEST_DEBUG='0', CARGO_INCREMENTAL='0', CARGO_TARGET_DIR=str(ROOT / 'lab/target/product'))
    changes = {path: (STAGE / path).read_text() for path, text in original.items() if text != (STAGE / path).read_text()}
    gz(RESULT / (name + '-mutations.json.gz'), changes)
    rec = {'name': name, 'phase': PHASE, 'utc_start': now(), 'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(), 'cwd': str(STAGE), 'command': cmd, 'targets': selected, 'mutated_paths': sorted(changes), 'environment': {k: env[k] for k in ['CARGO_BUILD_JOBS', 'CARGO_PROFILE_DEV_DEBUG', 'CARGO_PROFILE_TEST_DEBUG', 'CARGO_INCREMENTAL', 'CARGO_TARGET_DIR']}}
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
    for target in selected:
        target['diagnostics'] = [e for e in errors if (e.get('code') or {}).get('code') == 'E0004' and any(s['is_primary'] and s['file_name'].removeprefix(str(STAGE) + '/') == target['file'] and s['line_start'] == target['line'] and target['byte_start'] <= s['byte_start'] < target['byte_end'] for s in e['spans'])]
    rec.update(utc_end=now(), exit=run.returncode, selected_e0004=sum(bool(t['diagnostics']) for t in selected), other_errors=[e for e in errors if (e.get('code') or {}).get('code') != 'E0004'], error_count=len(errors))
    if insertion and PHASE == 'green':
        rec['isolated_axis'] = name
    records.append(rec)
    (RESULT / 'results.json').write_text(json.dumps(records, indent=2) + '\n')
    print(now(), name, 'END', run.returncode, 'selected', rec['selected_e0004'], 'other', len(rec['other_errors']), flush=True)
    assert not rec['other_errors'], rec['other_errors']
    assert rec['selected_e0004'] == (0 if PHASE == 'baseline' else len(selected)), selected
    assert run.returncode == 101
    if insertion and PHASE == 'green':
        assert all(SENTINEL in e['message'] for e in selected[0]['diagnostics'])
changed = [p for p, value in frozen.items() if sha(BASE / p) != value]
(RESULT / 'freeze.json').write_text(json.dumps({'utc': now(), 'changed': changed}, indent=2) + '\n')
assert not changed, changed
print('PASS', PHASE, 'selected decisions and isolated selector pair classes', flush=True)
