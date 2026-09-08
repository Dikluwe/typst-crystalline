#!/usr/bin/env python3
"""P1309 role A: fresh structural enumeration; no candidate matrix access."""
import argparse
import csv
import datetime
import hashlib
import io
import json
import os
from pathlib import Path
import subprocess
import sys
import time

sys.dont_write_bytecode = True

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / '00_nucleo/diagnosticos'
VANILLA_SHA = '7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'

def sha(path):
    return hashlib.file_digest(open(path, 'rb'), 'sha256').hexdigest()

def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def source_identity(side):
    paths = ['lab/typst-original'] if side == 'vanilla' else ['01_core','03_infra','lab/surface-inventory','Cargo.toml','Cargo.lock']
    run = subprocess.run(['git','ls-files','-z','--',*paths],cwd=ROOT,capture_output=True,check=True)
    files = sorted(x.decode() for x in run.stdout.split(b'\0') if x)
    identity = '\n'.join(path+' '+sha(ROOT/path) for path in files)
    return dict(tracked_files=len(files),sha256=hashlib.sha256(identity.encode()).hexdigest(),method='SHA256 of sorted newline-delimited git tracked relative path + space + current file SHA256', scopes=paths)

def publish(name, payload):
    path = OUT / name
    if path.exists():
        raise RuntimeError(f'refusing overwrite {path}')
    content = json.dumps(payload, ensure_ascii=False, indent=2) + '\n'
    patch = f'*** Begin Patch\n*** Add File: {path}\n' + ''.join('+' + line + '\n' for line in content.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, check=True, stdout=subprocess.PIPE)
    return {'path': str(path.relative_to(ROOT)), 'sha256': sha(path)}

def command(argv, env_delta=None, timeout=1200):
    started = utc()
    tick = time.monotonic_ns()
    try:
        run = subprocess.run(argv, cwd=ROOT, env={**os.environ, **(env_delta or {})}, capture_output=True, text=True, timeout=timeout)
        exit_code, stdout, stderr, unknown = run.returncode, run.stdout, run.stderr, None
        if exit_code < 0:
            unknown = 'signal'
    except subprocess.TimeoutExpired as e:
        exit_code, stdout, stderr, unknown = None, e.stdout or '', e.stderr or '', 'timeout'
        stdout = stdout.decode() if isinstance(stdout, bytes) else stdout
        stderr = stderr.decode() if isinstance(stderr, bytes) else stderr
    return dict(argv=argv, cwd=str(ROOT), env_delta=env_delta or {}, started_utc=started, ended_utc=utc(), duration_ns=time.monotonic_ns()-tick, exit_code=exit_code, stdout=stdout, stderr=stderr, unknown=unknown)

def main():
    p = argparse.ArgumentParser()
    p.add_argument('side', choices=['vanilla', 'crystalline'])
    p.add_argument('--target', required=True)
    p.add_argument('--binary', required=True)
    p.add_argument('--expected-sha', required=True)
    p.add_argument('--skip-build', action='store_true')
    p.add_argument('--suffix', default='')
    p.add_argument('--build-receipt')
    args = p.parse_args()
    assert sha(args.binary) == args.expected_sha
    side = args.side
    before_source = source_identity(side)
    source = ('lab/typst-original/crates/p1140-inventory' if side == 'vanilla' else 'lab/surface-inventory')
    manifest = ('lab/typst-original/Cargo.toml' if side == 'vanilla' else source + '/Cargo.toml')
    package = ('p1140-inventory' if side == 'vanilla' else 'surface-inventory-p1140')
    inputs = [source + '/src/main.rs', source + '/Cargo.toml', manifest, 'lab/surface-inventory/extra_seeds.json']
    if not args.skip_build:
        build = command(['cargo', 'build', '--release', '--locked', '--offline', '--manifest-path', manifest, '-p', package], {'CARGO_TARGET_DIR': args.target, 'CARGO_BUILD_JOBS': '4'}, timeout=2400)
        publish(f'p1309-inventory-{side}-build{args.suffix}.json', build)
        if build['exit_code'] != 0:
            raise SystemExit('inventory build failed; receipt preserved')
    enumerator = str(Path(args.target) / 'release' / package)
    for profile in ['default', 'html']:
        argv = [enumerator, '/dev/stdout']
        if side == 'vanilla':
            argv += [profile, args.binary]
        else:
            argv += [str(OUT / f'p1309-inventory-vanilla-{profile}{args.suffix}.json'), profile, 'lab/surface-inventory/extra_seeds.json', args.binary]
        receipt = command(argv, timeout=900)
        if receipt['exit_code'] != 0:
            publish(f'p1309-inventory-{side}-{profile}-execution{args.suffix}.json', receipt)
            raise SystemExit('inventory execution failed; receipt preserved')
        payload = json.loads(receipt['stdout'])
        after_source = source_identity(side)
        assert before_source == after_source, 'source changed during structural enumeration'
        receipt['stdout_sha256'] = hashlib.sha256(receipt['stdout'].encode()).hexdigest()
        receipt['stdout'] = '[integral stdout parsed without modification into inventory artifact entries/envelope]'
        payload['p1309_provenance'] = dict(author='/root/p1309_inventory', role='A', regime='executado sem atestação de isolamento técnico', baseline=command(['git','rev-parse','HEAD'])['stdout'].strip(), source_state=command(['git','diff','HEAD','--stat'])['stdout'], inherited_context='role A task, authorized P1309 step, repository instructions; historical method scripts; no candidate matrices', readable_inputs=[dict(path=x,sha256=sha(ROOT/x)) for x in sorted(set(inputs))], writable=['00_nucleo/diagnosticos/p1309-inventory-*','00_nucleo/diagnosticos/p1309-probe-catalog.json'], enumerator=dict(path=enumerator,sha256=sha(enumerator)), product=dict(path=args.binary,sha256=sha(args.binary)), receipt=receipt)
        payload['p1309_provenance']['source_identity_before'] = before_source
        payload['p1309_provenance']['source_identity_after'] = after_source
        payload['p1309_provenance']['attempt'] = args.suffix or 'invalidated-attempt-1'
        payload['p1309_provenance']['runner_sha256'] = sha(__file__)
        if args.build_receipt:
            payload['p1309_provenance']['enumerator_build_predecessor'] = dict(path=args.build_receipt,sha256=sha(args.build_receipt),use='immutable enumeration tool provenance only; not R2 product build gate')
        if args.suffix == '-r2':
            payload['p1309_provenance']['baseline_receipt'] = dict(path='00_nucleo/diagnosticos/p1309-baseline-r2.json',sha256=sha(OUT/'p1309-baseline-r2.json'))
            payload['p1309_provenance']['manifest'] = dict(path='00_nucleo/diagnosticos/p1309-manifest-r2.json',sha256=sha(OUT/'p1309-manifest-r2.json'))
        publish(f'p1309-inventory-{side}-{profile}{args.suffix}.json', payload)
        print(side, profile, len(payload['entries']), flush=True)

if __name__ == '__main__':
    main()
