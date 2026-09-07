#!/usr/bin/env python3
"""Read-only-source runner for separately applied, real P1307 R6 Rust mutants.

validate-plan does not inspect candidate sources or build anything.
run SPEC --receipt PATH consumes an independently pinned run specification:
  mode: control|mutant; family: F01..F20/Sxx/Axx;
  candidate_root: absolute disposable copy (never repository root);
  candidate_inventory: {relative source/config/L0 path: unmutated sha256};
  candidate_freeze: {path,sha256}; target_dir: absolute cargo target;
  candidate_freeze must carry source_inventory equal to candidate_inventory;
  lease: {path,sha256}; lease JSON has granted_by='/root', exclusive=true,
    candidate_root,target_dir,expires_at (UTC ISO), and explicit no_other_builds=true;
  route: rust|cli; tests: exact Rust suffixes; public_cases: exact oracle IDs;
  build_profile: dev|release (default dev); both use debug=0, incremental=true,
    lto=false, codegen-units=256. The exact same build profile is required for
    GREEN control and mutant; production release gates remain separate.
  changes: [{path,before_sha256,after_sha256,function,defect}];
  patch: {path,sha256} for mutant; patch must already be applied with apply_patch;
  green_control: {path,sha256} for mutant (a control receipt from this runner).

Each invocation compiles once and runs only declared focal witnesses. Source edits,
copy creation/restoration and causal adjudication are external, explicit operations.
Compilation error, missing witness and Unknown are not killed mutants. Even a real
failure remains PENDING_CAUSAL_REVIEW; the verifier must link the actual deviation.
"""
import argparse
import base64
import datetime as dt
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
PLAN = HERE / 'p1307-r6-mutation-plan.json'


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def read(path):
    return json.loads(Path(path).read_text())


def now():
    return dt.datetime.now(dt.timezone.utc).isoformat()


def pinned(ref):
    path = Path(ref['path']).resolve()
    assert sha(path) == ref['sha256'], f'pin mismatch: {path}'
    return read(path)


def validate_plan():
    plan = read(PLAN)
    for name, digest in plan['inputs'].items():
        assert sha(HERE / name) == digest, name
    baseline = read(HERE / 'p1307-r6-baseline.json')
    old_path = HERE / plan['base_families']['artifact']
    predecessors = baseline['protected_predecessors']
    # The canonical baseline map uses repo-relative paths.
    old_key = str(old_path.relative_to(ROOT))
    assert sha(old_path) == predecessors[old_key]
    old = {f['id']: f for f in read(old_path)['families']}
    oracle = read(HERE / 'p1307-r6-oracle.json')
    cases = {c['id']: c for c in oracle['cases']}
    tests = set(re.findall(r'fn (p1307_\w+)\(', (HERE / 'p1307-r6-tests.patch').read_text()))
    resolved = {}
    for original in plan['families'] + plan['additional_families']:
        f = dict(original)
        if 'base_id' in f:
            base = old[f['base_id']]
            f['public_cases'] = ['p1307.' + x for x in base['witnesses']]
            f['owner'] = f.get('owner_override', base['owner_anchor']['path'])
            f.setdefault('baseline_function', base['owner_anchor']['function'])
        f['baseline_owner_sha256'] = baseline['source_inventory'][f['owner']]
        assert set(f.get('rust_tests', [])) <= tests, f['id']
        assert set(f['public_cases']) <= set(cases), f['id']
        resolved[f['id']] = f
    assert {f['base_id'] for f in plan['families']} == set(range(1, 21))
    return plan, resolved, cases


def inventory(root, expected):
    for p in expected:
        path = root / p
        assert not path.is_symlink() and path.resolve().is_relative_to(root), p
    actual = {p: sha(root / p) for p in expected}
    # Cargo-affecting source additions must not evade the frozen inventory.
    roots = ('01_core', '02_shell', '03_infra', '04_wiring')
    tracked = {p for p in expected if p.endswith('.rs') and p.split('/')[0] in roots}
    found = {str(p.relative_to(root)) for d in roots
             for p in (root / d).rglob('*.rs') if 'target' not in p.relative_to(root).parts}
    assert found == tracked, {'unlisted_source': sorted(found - tracked), 'missing_source': sorted(tracked - found)}
    return actual


def execute(argv, cwd, env, timeout):
    start = time.monotonic()
    row = {'argv': argv, 'cwd': str(cwd), 'started': now(), 'timeout_seconds': timeout}
    proc = subprocess.Popen(argv, cwd=cwd, env=env, stdout=subprocess.PIPE,
                            stderr=subprocess.PIPE, start_new_session=True)
    timed_out = False
    try:
        out, err = proc.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
        timed_out = True
        os.killpg(proc.pid, signal.SIGKILL)
        out, err = proc.communicate()
    row.update(finished=now(), elapsed_seconds=time.monotonic() - start,
               returncode=proc.returncode, timed_out=timed_out,
               stdout=out.decode('utf-8', 'replace'), stderr=err.decode('utf-8', 'replace'),
               stdout_base64=base64.b64encode(out).decode(), stderr_base64=base64.b64encode(err).decode())
    return row


def run(spec, receipt):
    plan, families, cases = validate_plan()
    rec = {'schema': 'p1307-r6-real-mutant-observation-v1', 'started': now(),
           'regime': plan['regime'], 'manifest_sha256': plan['inputs']['p1307-r6-manifest.json'],
           'plan_sha256': sha(PLAN), 'runner_sha256': sha(__file__),
           'spec': spec, 'commands': [], 'status': 'Unknown', 'source_mutant_killed': False}
    try:
        f = families[spec['family']]
        root = Path(spec['candidate_root']).resolve()
        assert root.is_absolute() and root != ROOT and not root.is_relative_to(ROOT)
        assert not ROOT.is_relative_to(root) and root.name not in ('tmp', 'shm')
        freeze = pinned(spec['candidate_freeze'])
        expected = spec['candidate_inventory']
        assert freeze['source_inventory'] == expected
        assert all(k in expected for k in ('Cargo.toml', 'Cargo.lock', '.cargo/config.toml',
                                          '01_core/src/compiler/eval/tests.rs'))
        assert any(k.startswith('00_nucleo/prompts/') for k in expected)
        before = inventory(root, expected)
        changed = {p for p in expected if before[p] != expected[p]}
        assert spec['mode'] in ('control', 'mutant')
        if spec['mode'] == 'control':
            assert not changed and not spec.get('changes')
        else:
            assert spec['changes'] and changed == {c['path'] for c in spec['changes']}
            for c in spec['changes']:
                assert c['path'] == f['owner'], 'Different owner requires explicit plan refinement before execution'
                assert c['path'].endswith('.rs') and not c['path'].endswith('/tests.rs')
                assert c['before_sha256'] == expected[c['path']]
                assert c['after_sha256'] == before[c['path']]
                assert c['function'] and c['defect']
            assert sha(spec['patch']['path']) == spec['patch']['sha256']
            rec['concrete_patch'] = Path(spec['patch']['path']).read_text()
            control = pinned(spec['green_control'])
            assert control['status'] == 'GREEN_CONTROL'
            assert control['spec']['candidate_inventory'] == expected
            assert control['spec']['candidate_freeze'] == spec['candidate_freeze']
            assert control['spec']['route'] == spec['route']
        lease = pinned(spec['lease'])
        target = str(Path(spec['target_dir']).resolve())
        assert lease['granted_by'] == '/root' and lease['exclusive'] and lease['no_other_builds']
        assert lease['candidate_root'] == str(root) and lease['target_dir'] == target
        expiry = dt.datetime.fromisoformat(lease['expires_at'])
        remaining = (expiry - dt.datetime.now(dt.timezone.utc)).total_seconds()
        assert remaining > 0, 'Expired build lease'
        rec['lease'] = lease
        rec['source_before'] = before
        env = dict(os.environ)
        env.pop('TYPST_FEATURES', None)
        env.update(CARGO_TARGET_DIR=target, NO_COLOR='1', TERM='dumb')
        rec['environment'] = {k: env.get(k) for k in ('PATH', 'RUSTFLAGS', 'RUSTUP_TOOLCHAIN',
            'CARGO_TARGET_DIR', 'CARGO_BUILD_JOBS', 'NO_COLOR', 'TERM', 'TYPST_FEATURES')}
        profile = spec.get('build_profile', 'dev')
        assert profile in ('dev', 'release')
        build_env = {f'CARGO_PROFILE_{p}_{k}': v for p in ('DEV', 'TEST', 'RELEASE')
                     for k, v in {'DEBUG': '0', 'INCREMENTAL': 'true', 'LTO': 'false',
                                  'CODEGEN_UNITS': '256'}.items()}
        env.update(build_env)
        rec['environment'].update(build_env)
        if spec['mode'] == 'mutant':
            assert control['spec'].get('build_profile', 'dev') == profile
            for key in ('RUSTFLAGS', 'RUSTUP_TOOLCHAIN', *build_env):
                assert control['environment'].get(key) == rec['environment'].get(key), 'Control build configuration differs'
        route = spec['route']
        assert route in ('rust', 'cli')
        tests = spec.get('tests', [])
        public = spec.get('public_cases', [])
        if route == 'rust':
            assert tests and len(tests) == len(set(tests)) and set(tests) <= set(f['rust_tests'])
            assert not f.get('public_required'), 'This family requires a compiled public witness'
            cmd = ['cargo', 'test', '-p', 'typst-core', '--lib', '--no-run', '--message-format=json']
        else:
            assert public and len(public) == len(set(public)) and set(public) <= set(f['public_cases'])
            cmd = ['cargo', 'build', '-p', 'typst-wiring', '--bin', 'typst', '--message-format=json']
        if profile == 'release':
            cmd.append('--release')
        # Coordinator lease provides scheduling; this lock only prevents runner-runner overlap.
        with open('/tmp/p1307-r6-mutation-build.lock', 'a') as lock:
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
            build = execute(cmd, root, env, min(remaining, spec.get('build_timeout_seconds', 900)))
            rec['commands'].append(build)
            assert not build['timed_out'] and build['returncode'] == 0, 'Compile not successful; no mutant kill'
            artifacts = []
            for line in build['stdout'].splitlines():
                try:
                    item = json.loads(line)
                except json.JSONDecodeError:
                    continue
                if item.get('reason') == 'compiler-artifact' and item.get('executable'):
                    if ((route == 'rust' and item['target']['name'] == 'typst_core' and item['profile']['test'])
                            or (route == 'cli' and item['target']['name'] == 'typst')):
                        artifacts.append(item['executable'])
            assert len(set(artifacts)) == 1, 'Unresolved compiled executable'
            binary = Path(artifacts[0]).resolve()
            assert binary.is_relative_to(Path(target))
            rec.update(binary=str(binary), binary_sha256=sha(binary))
            if route == 'rust':
                listing = execute([str(binary), '--list'], root, env, 30)
                rec['commands'].append(listing)
                assert listing['returncode'] == 0 and not listing['timed_out']
                names = [line.removesuffix(': test') for line in listing['stdout'].splitlines() if line.endswith(': test')]
                rows = []
                for suffix in tests:
                    matches = [x for x in names if x.endswith('::' + suffix) and 'p1307_independent' in x]
                    assert len(matches) == 1, 'Missing or ambiguous frozen test: ' + suffix
                    row = execute([str(binary), matches[0], '--exact', '--test-threads=1', '--nocapture'], root, env, 60)
                    row['test'] = suffix
                    assert not row['timed_out'] and row['returncode'] in (0, 101), 'Test infrastructure failure'
                    assert re.search(r'test result: (?:ok|FAILED)\. (?:1 passed; 0 failed|0 passed; 1 failed);', row['stdout']), 'Zero or unparsed tests'
                    rows.append(row)
                    rec['commands'].append(row)
                rec['witnesses'] = {r['test']: 'Preserved' if r['returncode'] == 0 else 'Violated' for r in rows}
            else:
                pattern = '^(?:' + '|'.join(re.escape(x) for x in public) + ')$'
                row = execute([sys.executable, str(HERE / 'p1307-r6-oracle.py'), 'replay', '--binary', str(binary), '--case', pattern], root, env, 300)
                rec['commands'].append(row)
                assert row['returncode'] == 0 and not row['timed_out'], 'Replay infrastructure failure'
                observed = json.loads(row['stdout'])
                assert observed['binary_sha256'] == rec['binary_sha256']
                wanted = {(c, p) for c in public for p in cases[c]['observations']}
                assert len(observed['rows']) == len(wanted)
                assert {(r['case'], r['profile']) for r in observed['rows']} == wanted
                rec['witnesses'] = {r['case'] + '/' + r['profile']: r['verdict'] for r in observed['rows']}
                rec['public_observations'] = observed
        values = set(rec['witnesses'].values())
        assert values <= {'Preserved', 'Violated'}, 'Required observation Unknown'
        if spec['mode'] == 'control':
            rec['status'] = 'GREEN_CONTROL' if values == {'Preserved'} else 'CONTROL_NOT_GREEN'
        else:
            assert all(control['witnesses'].get(k) == 'Preserved' for k in rec['witnesses']), 'Missing exact GREEN control'
            rec['status'] = 'COMPILED_WITNESS_FAILURE_PENDING_CAUSAL_REVIEW' if 'Violated' in values else 'SURVIVED_FOCAL'
        rec['source_after'] = inventory(root, expected)
        assert rec['source_after'] == before, 'Source changed during measurement'
    except (AssertionError, KeyError, ValueError, OSError) as exc:
        rec.update(status='Unknown', reason=type(exc).__name__ + ': ' + str(exc))
    finally:
        if 'before' in locals():
            try:
                rec['source_after'] = inventory(root, expected)
                rec['source_unchanged_during_attempt'] = rec['source_after'] == before
                if not rec['source_unchanged_during_attempt']:
                    rec.update(status='Unknown', reason='Source changed during attempt')
            except (AssertionError, OSError) as exc:
                rec.update(status='Unknown', source_after_error=str(exc))
        rec['finished'] = now()
        with Path(receipt).open('x') as handle:
            json.dump(rec, handle, ensure_ascii=False, indent=2)
            handle.write('\n')
    print(json.dumps({'receipt': str(receipt), 'sha256': sha(receipt), 'status': rec['status'], 'source_mutant_killed': False}))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='command', required=True)
    sub.add_parser('validate-plan')
    r = sub.add_parser('run')
    r.add_argument('spec')
    r.add_argument('--receipt', required=True)
    args = parser.parse_args()
    if args.command == 'validate-plan':
        _, families, _ = validate_plan()
        print(json.dumps({'status': 'PLAN_REFERENCES_VALID', 'families': len(families), 'source_mutants_executed': 0, 'mutation_score': None}))
    else:
        destination = Path(args.receipt).resolve()
        assert destination.parent == HERE and destination.name.startswith('p1307-r6-mutant-') and destination.suffix == '.json'
        assert not destination.exists(), 'Do not overwrite a prior attempt'
        run(read(args.spec), args.receipt)
