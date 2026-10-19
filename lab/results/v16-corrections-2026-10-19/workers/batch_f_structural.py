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
OUT = ROOT / 'lab/results/v16-corrections-2026-10-19/batch-f'
MODE = sys.argv[1]
assert MODE in ('baseline', 'green', 'final-green', 'version-green')
PHASE = 'baseline' if MODE.startswith('baseline') else 'green'
BASE = ROOT / 'lab/scratch/v16-batch-f-base-2026-10-19' if PHASE == 'baseline' else ROOT
STAGE = ROOT / ('lab/scratch/v16-batch-f-structural-' + MODE)
RESULT = OUT / ('structural-' + MODE)
RESULT.mkdir(parents=True, exist_ok=False)
SENTINEL = 'TektV16EvolutionProbe'
DOMAINS = {
    'Content': ('01_core/src/entities/content.rs', 'pub enum Content {'),
    'Value': ('01_core/src/entities/value.rs', 'pub enum Value {'),
    'Type': ('01_core/src/entities/value.rs', 'pub enum Type {'),
    'CounterKey': ('01_core/src/entities/counter.rs', 'pub enum CounterKey {'),
    'Selector': ('01_core/src/entities/selector.rs', 'pub enum Selector {'),
}


CG = Path('/sys/fs/cgroup') / Path('/proc/self/cgroup').read_text().strip().split('::', 1)[1].lstrip('/')
def memory():
    return {k: (CG / k).read_text().strip() for k in ['memory.max', 'memory.high', 'memory.swap.max', 'memory.peak', 'memory.events']}
assert memory()['memory.max'] == str(12 * 1024**3)
assert memory()['memory.swap.max'] == '0'

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
frozen = {p: sha(BASE / p) for p in scope['sources'] + scope['owners']}
for top in ['01_core', '02_shell', '03_infra', '04_wiring', 'lab/benches', 'lab/corpus', 'lab/parity/corpus', 'lab/typst-original/tests']:
    shutil.copytree(BASE / top, STAGE / top, dirs_exist_ok=True)
for path in ['Cargo.toml', 'Cargo.lock', 'clippy.toml', 'rustfmt.toml', 'rust-toolchain.toml', '.cargo/config.toml']:
    dest = STAGE / path
    dest.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(BASE / path, dest)
original = {str(p.relative_to(STAGE)): p.read_text() for top in ['01_core', '02_shell', '03_infra', '04_wiring'] for p in (STAGE / top).rglob('*.rs')}
gz(RESULT / 'original-sources.json.gz', original)
gz(RESULT / 'inputs.json.gz', {'utc': now(), 'base': str(BASE), 'inputs': {str(p.relative_to(STAGE)): sha(p) for p in STAGE.rglob('*') if p.is_file()}, 'frozen_owners': frozen, 'memory': memory(), 'worker_sha256': sha(Path(__file__))})
jobs = [('all-domains', list(DOMAINS), targets, None)]
if PHASE == 'green':
    arity_file = '01_core/src/compiler/stdlib/foundations/selector.rs'
    for name, retained in [('arity-zero', '[_, _, ..]'), ('arity-many', '[]')]:
        target = {'id': 'V16-056-' + name, 'file': arity_file,
                  'after': 'fn parse_selector_arg(', 'anchor': 'match items {'}
        jobs.append((name, [], [target], retained))
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
        p = STAGE / '01_core/src/compiler/stdlib/foundations/selector.rs'
        text = p.read_text()
        start = text.index('fn parse_selector_arg(')
        needle = '[] | [_, _, ..]'
        pos = text.index(needle, start)
        p.write_text(text[:pos] + insertion + text[pos + len(needle):])
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
    rec.update(utc_end=now(), memory=memory(), exit=run.returncode, selected_e0004=sum(bool(t['diagnostics']) for t in selected), other_errors=[e for e in errors if (e.get('code') or {}).get('code') != 'E0004'], error_count=len(errors))
    records.append(rec)
    (RESULT / 'results.json').write_text(json.dumps(records, indent=2) + '\n')
    print(now(), name, 'END', run.returncode, 'selected', rec['selected_e0004'], 'other', len(rec['other_errors']), flush=True)
    assert not rec['other_errors'], rec['other_errors']
    assert rec['selected_e0004'] == (0 if PHASE == 'baseline' else len(selected)), selected
    assert run.returncode == 101
changed = [p for p, value in frozen.items() if sha(BASE / p) != value]
(RESULT / 'freeze.json').write_text(json.dumps({'utc': now(), 'changed': changed}, indent=2) + '\n')
assert not changed, changed
print('PASS', PHASE, 'selected decisions; slice arity uses separate proof', flush=True)
