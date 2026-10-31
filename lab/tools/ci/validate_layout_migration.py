#!/usr/bin/env python3
"""Reproduce the local Linux validation of the 2026-10-18 layout migration.

Requires the pinned local oracle and CI-pinned linter. Run inside the resource
limited service documented by the receipt. Writes command receipts and logs;
does not repair source, seals, or the oracle. This supplements the normal CI.
"""
from __future__ import annotations
import datetime
import argparse
import gzip
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'lab/results/mature-layout-2026-10-18'
LINTER = ROOT / 'lab/target/linter/debug/crystalline-lint'
ORACLE = ROOT / 'lab/typst-original/target/release/typst'
PIN = '12797073972e375d9ab078cf6750c8b30b71e2b9'


def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b''):
            h.update(chunk)
    return h.hexdigest()


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def frozen_inputs():
    files = [ROOT / name for name in ('Cargo.toml', 'Cargo.lock', 'crystalline.toml', '.cargo/config.toml', 'rust-toolchain.toml')]
    for directory in ('01_core', '02_shell', '03_infra', '04_wiring', '00_nucleo/prompts', 'lab/benches'):
        files.extend(p for p in (ROOT / directory).rglob('*') if p.is_file() and p.suffix in {'.rs', '.md', '.toml'})
    return {str(p.relative_to(ROOT)): digest(p) for p in sorted(files)}


def main():
    global OUT
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--start-at', help='Resume at a named job; earlier results must be retained separately')
    parser.add_argument('--output', type=Path, help='Separate receipt directory under lab/results')
    args = parser.parse_args()
    if args.output is not None:
        OUT = (ROOT / args.output).resolve()
        OUT.relative_to(ROOT / 'lab/results')
    OUT.mkdir(parents=True, exist_ok=True)
    os.chdir(ROOT)
    assert digest(ORACLE) == '22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34', 'oracle identity mismatch'
    assert subprocess.check_output(['git', '-C', str(ROOT/'lab/scratch/tekt-linter-pinned'), 'rev-parse', 'HEAD'], text=True).strip() == PIN
    env = os.environ.copy()
    env.update(CARGO_BUILD_JOBS='1', CARGO_PROFILE_DEV_DEBUG='0', CARGO_PROFILE_TEST_DEBUG='0', CARGO_INCREMENTAL='0', TYPST_VANILLA=str(ORACLE), PATH=str(LINTER.parent)+os.pathsep+env['PATH'])
    env.pop('CRYSTALLINE_PDF_FIXED_EPOCH', None)
    observers = {}
    for name in ('rsvg-convert', 'pdftotext', 'pdftoppm', 'pdfinfo', 'qpdf', 'convert'):
        executable = shutil.which(name, path=env['PATH'])
        assert executable, f'required visual observer missing from PATH: {name}'
        observers[name] = {'path': executable, 'sha256': digest(Path(executable))}
    (OUT/'observer-environment.json').write_text(json.dumps({'utc': utc(), 'PATH': env['PATH'], 'observers': observers}, indent=2)+'\n')
    start_inputs = frozen_inputs()
    state = {'utc': utc(), 'state': 'uncommitted working tree', 'checkpoint': subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(), 'diff_head_stat': subprocess.check_output(['git','diff','HEAD','--stat'],text=True), 'status': subprocess.check_output(['git','status','--short'],text=True), 'inputs': start_inputs, 'linter_commit': PIN, 'linter_sha256': digest(LINTER), 'oracle_sha256': digest(ORACLE)}
    with gzip.open(OUT/'validation-state.json.gz','wt') as f:
        json.dump(state,f,indent=2)
    records = []
    jobs = [
        ('python-ci-tests', ['python3','-m','unittest','discover','-s','lab/tools/ci'], {}),
        ('layout', ['python3','lab/tools/ci/check_mature_layout.py'], {}),
        ('strict-lineage', [str(LINTER),'.','--checks','v1,v5,v15,v26','--fail-on','warning'], {}),
        ('reverse-lineage', ['python3','lab/tools/ci/check_reverse_lineage.py'], {}),
        ('build', ['cargo','build','--offline','--locked','--workspace'], {}),
        ('tests-domain-infra', ['cargo','test','--offline','--locked','--workspace','--exclude','typst-wiring','--lib','--bins','--tests','--no-fail-fast','--','--test-threads=1'], {'CRYSTALLINE_PDF_FIXED_EPOCH':'1792281600'}),
        ('tests-wiring', ['cargo','test','--offline','--locked','-p','typst-wiring','--bins','--tests','--no-fail-fast','--','--test-threads=1'], {}),
        ('doctests', ['cargo','test','--offline','--locked','--workspace','--doc','--no-fail-fast','--','--test-threads=1'], {}),
        ('build-all-targets', ['cargo','build','--offline','--locked','--workspace','--all-targets'], {}),
        ('clippy', ['cargo','clippy','--offline','--locked','--workspace','--all-targets','--','-D','warnings','-D','unfulfilled_lint_expectations'], {}),
        ('fmt', ['cargo','fmt','--check','--all'], {}),
        ('rustdoc', ['cargo','doc','--offline','--locked','--workspace','--no-deps'], {'RUSTDOCFLAGS':'-D warnings'}),
    ]
    if args.start_at is not None:
        names = [job[0] for job in jobs]
        if args.start_at not in names:
            parser.error(f'unknown job: {args.start_at}; choose one of {names}')
        jobs = jobs[names.index(args.start_at):]
    with tempfile.TemporaryDirectory(prefix='tekt-layout-',dir='/dev/shm') as temp:
        env['TMPDIR'] = temp
        for name, command, overrides in jobs:
            print(f'{utc()} START {name}', flush=True)
            started = time.monotonic()
            with (OUT/(name+'.log')).open('w') as log:
                result = subprocess.run(command, env={**env, **overrides}, stdout=log, stderr=subprocess.STDOUT)
            record = {'name': name, 'command': command, 'utc_end': utc(), 'seconds': time.monotonic()-started, 'exit': result.returncode, 'environment': {k: env[k] for k in ('CARGO_BUILD_JOBS','CARGO_PROFILE_DEV_DEBUG','CARGO_PROFILE_TEST_DEBUG','CARGO_INCREMENTAL','TYPST_VANILLA','TMPDIR')}, 'overrides': overrides}
            records.append(record)
            (OUT/'validation-commands.json').write_text(json.dumps(records,indent=2)+'\n')
            print(f'{utc()} END {name}: {result.returncode}',flush=True)
            if result.returncode:
                print((OUT/(name+'.log')).read_text()[-9000:],flush=True)
                break
    current = frozen_inputs()
    changed = sorted(k for k in start_inputs.keys() | current.keys() if start_inputs.get(k) != current.get(k))
    (OUT/'validation-freeze.json').write_text(json.dumps({'utc':utc(),'changed_inputs':changed,'input_count':len(start_inputs),'completed_jobs':len(records),'expected_jobs':len(jobs)},indent=2)+'\n')
    return int(bool(changed) or len(records)!=len(jobs) or any(r['exit'] for r in records))


if __name__ == '__main__':
    raise SystemExit(main())
