"""Prepare frozen semantic mutants; run only during the authorized Cargo window."""
import argparse
import datetime
import difflib
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import time

ROOT = Path('/repos/Antigravity/typst-crystalline')
DIAG = ROOT / '00_nucleo/diagnosticos'
SOURCE = Path('01_core/src/compiler/eval/bindings/field_access.rs')
PLAN_SHA = '821106df1e4f4a5aa78bd57720b1605a9c3a438ddbb4edbcb6026dc8b919f778'
TEST_SHA = 'b30cc9449bef081d51d44d664aed04c4c26c48bcde15ecf4b31938a1ecfd9934'

def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def sha(data):
    return hashlib.sha256(data).hexdigest()

def tests(text):
    start = text.index('#[cfg(test)]\nmod p1336_tests {')
    end = text.index('#[cfg(test)]\nmod p1326_tests {', start)
    return text[start:end]

def save_json(path, value):
    assert not path.exists(), f'Immutable receipt already exists: {path}'
    body = json.dumps(value, ensure_ascii=False, indent=2) + '\n'
    patch = ('*** Begin Patch\n*** Add File: ' + str(path) + '\n'
             + ''.join('+' + line + '\n' for line in body.splitlines())
             + '*** End Patch\n')
    p = subprocess.run(['apply_patch'], input=patch, capture_output=True, text=True)
    assert p.returncode == 0, p.stderr
    assert path.read_text() == body

def prepare():
    assert sha((DIAG / 'p1336-attacks-plan.md').read_bytes()) == PLAN_SHA
    candidate = (ROOT / SOURCE).read_text()
    assert sha(tests(candidate).encode()) == TEST_SHA
    base = Path(tempfile.mkdtemp(prefix='p1336-attacks-'))
    workspace = base / 'workspace'
    workspace.mkdir()
    copied = ['Cargo.toml', 'Cargo.lock', '.cargo', '01_core', '02_shell', '03_infra', '04_wiring', 'benches']
    for name in copied:
        src, dst = ROOT / name, workspace / name
        if src.is_dir():
            shutil.copytree(src, dst)
        else:
            shutil.copy2(src, dst)
    assert (workspace / SOURCE).stat().st_ino != (ROOT / SOURCE).stat().st_ino
    cases = [('C', candidate)]
    fallback = '        other @ (Value::Int(_) | Value::Str(_)) => Err(vec![SourceDiagnostic::error('
    for ident, kind, name in [('M1', 'Int', 'int'), ('M2', 'Str', 'str')]:
        insert = f'        Value::{kind}(_) => Err(vec![SourceDiagnostic::error(\n            span,\n            "cannot access fields on type {name}".to_string(),\n        )]),\n'
        assert candidate.count(fallback) == 1
        cases.append((ident, candidate.replace(fallback, insert + fallback)))
    for ident, kind in [('M3', 'Int'), ('M4', 'Str')]:
        needle = f'            | Value::{kind}(_)\n'
        assert candidate.count(needle) == 1
        cases.append((ident, candidate.replace(needle, '', 1)))
    needle = '            | Value::Str(_)\n'
    cases.append(('M5', candidate.replace(needle, needle + '            | Value::Bool(_)\n', 1)))
    needle = '        Value::Type(t) => match (t, field) {'
    assert candidate.count(needle) == 1
    insert = '        Value::Type(Type::Int | Type::Str) => Err(vec![SourceDiagnostic::error(\n            span,\n            "cannot access fields on type integer or string".to_string(),\n        )]),\n'
    cases.append(('M6', candidate.replace(needle, insert + needle)))
    record = {'at': utc(), 'base': str(base), 'workspace': str(workspace),
              'candidate_sha256': sha(candidate.encode()), 'tests_sha256': TEST_SHA,
              'manifest_sha256': sha((DIAG / 'p1336-manifest.json').read_bytes()),
              'candidate_receipt_sha256': sha((DIAG / 'p1336-candidate.json').read_bytes()),
              'baseline_sha256': sha((DIAG / 'p1336-baseline.json').read_bytes()),
              'plan_sha256': PLAN_SHA, 'copy_policy': 'shutil.copy2/copytree; distinct inodes; no mutable hardlinks',
              'copied_roots': copied, 'mutants': [], 'runs': []}
    for ident, changed in cases:
        assert sha(tests(changed).encode()) == TEST_SHA
        directory = base / ident
        directory.mkdir()
        shutil.copy2(ROOT / SOURCE, directory / 'field_access.rs')
        if ident != 'C':
            lines = list(difflib.unified_diff(candidate.splitlines(True), changed.splitlines(True)))
            hunks = ''.join(lines[2:])
            # apply_patch accepts context hunks with bare @@ markers.
            hunks = '\n'.join('@@' if line.startswith('@@') else line for line in hunks.splitlines()) + '\n'
            patch = '*** Begin Patch\n*** Update File: ' + str(directory / 'field_access.rs') + '\n' + hunks + '*** End Patch\n'
            (directory / 'change.apply_patch').write_text(patch)
            p = subprocess.run(['apply_patch'], input=patch, text=True, capture_output=True)
            (directory / 'patch.stdout').write_text(p.stdout)
            (directory / 'patch.stderr').write_text(p.stderr)
            assert p.returncode == 0, p.stderr
        assert (directory / 'field_access.rs').read_text() == changed
        record['mutants'].append({'id': ident, 'source_sha256': sha(changed.encode()),
            'path': str(directory / 'field_access.rs'), 'tests_sha256': sha(tests(changed).encode()),
            'patch_sha256': sha((directory / 'change.apply_patch').read_bytes()) if ident != 'C' else None})
    save_json(DIAG / 'p1336-attacks-prepared.json', record)
    print(json.dumps({'base': str(base), 'candidate_sha256': record['candidate_sha256'], 'mutants': len(cases)-1}))

def run(cache):
    record_path = DIAG / 'p1336-attacks-prepared.json'
    record = json.loads(record_path.read_text())
    assert not (DIAG / 'p1336-attacks-results.json').exists()
    assert all(not (DIAG / f'p1336-attacks-run-{m["id"]}.json').exists()
               for m in record['mutants'])
    record['runner_sha256'] = sha(Path(__file__).read_bytes())
    base, workspace = Path(record['base']), Path(record['workspace'])
    l0 = Path('00_nucleo/prompts/compiler/eval/bindings/field_access.md')
    (workspace / l0).parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(ROOT / l0, workspace / l0)
    record['final_l0_sha256'] = sha((workspace / l0).read_bytes())
    record['final_source_main_sha256'] = sha((ROOT / SOURCE).read_bytes())
    assert record['final_source_main_sha256'] == record['candidate_sha256']
    record['execution_provenance'] = {'at': utc()}
    for label, command in [('head', ['git', 'rev-parse', 'HEAD']), ('diff_stat', ['git', 'diff', 'HEAD', '--stat'])]:
        p = subprocess.run(command, cwd=ROOT, capture_output=True, text=True)
        record['execution_provenance'][label] = {'argv': command, 'exit': p.returncode, 'stdout': p.stdout, 'stderr': p.stderr}
    target = base / 'target'
    assert not target.exists(), 'Do not overwrite an earlier execution; preserve outputs.'
    at = utc()
    t = time.monotonic()
    cmd = ['cp', '-a', '--reflink=auto', str(Path(cache)) + '/.', str(target)]
    copy_error = None
    try:
        p = subprocess.run(cmd, capture_output=True, text=True, timeout=2700)
        copy_exit, copy_out, copy_err = p.returncode, p.stdout, p.stderr
    except (subprocess.TimeoutExpired, OSError) as exc:
        copy_error = repr(exc)
        copy_exit, copy_out, copy_err = None, '', copy_error
    record['cache_copy'] = {'argv': cmd, 'at': at, 'seconds': time.monotonic()-t,
                            'exit': copy_exit, 'stdout': copy_out, 'stderr': copy_err}
    if copy_exit != 0:
        record['status'] = 'Unknown'
        record['reason'] = 'cache_copy_failed'
        save_json(DIAG / 'p1336-attacks-results.json', record)
        return
    env = os.environ.copy()
    env.update({'CARGO_TARGET_DIR': str(target), 'CARGO_BUILD_JOBS': '2'})
    for mutant in record['mutants']:
        ident = mutant['id']
        shutil.copy2(mutant['path'], workspace / SOURCE)
        assert sha((workspace / SOURCE).read_bytes()) == mutant['source_sha256']
        cmd = ['cargo', 'test', '--release', '--locked', '--offline', '-p', 'typst-core', '--lib', 'p1336_tests', '--', '--nocapture']
        at = utc()
        t = time.monotonic()
        output = base / ident
        print(json.dumps({'starting': ident, 'at': at}), flush=True)
        run_error = None
        with (output / 'test.stdout').open('x') as stdout, (output / 'test.stderr').open('x') as stderr:
            try:
                p = subprocess.run(cmd, cwd=workspace, env=env, stdout=stdout, stderr=stderr, timeout=2700)
                run_exit = p.returncode
            except (subprocess.TimeoutExpired, OSError) as exc:
                run_error = repr(exc)
                run_exit = None
        out, err = (output / 'test.stdout').read_text(), (output / 'test.stderr').read_text()
        valid = run_error is None and 'running 5 tests' in out and 'Finished `release`' in err
        item = {'id': ident, 'argv': cmd, 'cwd': str(workspace), 'env': {'CARGO_TARGET_DIR': str(target), 'CARGO_BUILD_JOBS': '2'},
                'at_start': at, 'at_end': utc(), 'seconds': time.monotonic()-t, 'exit': run_exit,
                'execution_error': run_error,
                'source_sha256': mutant['source_sha256'], 'tests_sha256': TEST_SHA,
                'valid_execution': valid, 'stdout_path': str(output / 'test.stdout'),
                'stderr_path': str(output / 'test.stderr'), 'stdout_sha256': sha(out.encode()), 'stderr_sha256': sha(err.encode()),
                'status': 'passed' if valid and run_exit == 0 else 'failed_test' if valid and run_exit == 101 else 'Unknown'}
        record['runs'].append(item)
        receipt = {**item, 'manifest_sha256': record['manifest_sha256'],
                   'baseline_sha256': record['baseline_sha256'],
                   'candidate_sha256': record['candidate_sha256'],
                   'runner_sha256': record['runner_sha256'],
                   'plan_sha256': record['plan_sha256'],
                   'final_l0_sha256': record['final_l0_sha256'],
                   'execution_provenance': record['execution_provenance']}
        save_json(DIAG / f'p1336-attacks-run-{ident}.json', receipt)
        print(json.dumps(item), flush=True)
        if ident == 'C' and item['status'] != 'passed':
            record['reason'] = 'candidate_control_failed_mutants_not_run'
            break
    record['status'] = 'Unknown' if any(x['status'] == 'Unknown' for x in record['runs']) else 'executed_pending_independent_review'
    save_json(DIAG / 'p1336-attacks-results.json', record)

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('mode', choices=['prepare', 'run'])
    parser.add_argument('--cache')
    args = parser.parse_args()
    prepare() if args.mode == 'prepare' else run(args.cache)
