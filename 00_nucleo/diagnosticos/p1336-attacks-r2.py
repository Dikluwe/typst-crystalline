"""Focal retry of R1's stale M2..M6, preserving every prior receipt."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import time

ROOT = Path('/repos/Antigravity/typst-crystalline')
DIAG = ROOT / '00_nucleo/diagnosticos'
SOURCE = Path('01_core/src/compiler/eval/bindings/field_access.rs')

def sha(data):
    return hashlib.sha256(data).hexdigest()

def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def publish(path, value):
    assert not path.exists(), path
    body = json.dumps(value, ensure_ascii=False, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n' + ''.join('+' + line + '\n' for line in body.splitlines()) + '*** End Patch\n'
    p = subprocess.run(['apply_patch'], input=patch, capture_output=True, text=True)
    assert p.returncode == 0, p.stderr
    assert path.read_text() == body

if __name__ == '__main__':
    prepared = json.loads((DIAG / 'p1336-attacks-prepared.json').read_text())
    prior = json.loads((DIAG / 'p1336-attacks-results.json').read_text())
    base, workspace = Path(prepared['base']), Path(prepared['workspace'])
    script_sha = sha(Path(__file__).read_bytes())
    env = os.environ.copy()
    env.update({'CARGO_TARGET_DIR': str(base / 'target'), 'CARGO_BUILD_JOBS': '2'})
    aggregate = {'at': utc(), 'runner_sha256': script_sha, 'revision_reason': 'stale_compiled_mutant',
                 'prior_results_sha256': sha((DIAG / 'p1336-attacks-results.json').read_bytes()), 'runs': []}
    for mutant in prepared['mutants']:
        ident = mutant['id']
        if ident in ['C', 'M1']:
            continue
        output = base / (ident + '-r2')
        output.mkdir()
        shutil.copy2(mutant['path'], workspace / SOURCE)
        before_mtime = (workspace / SOURCE).stat().st_mtime_ns
        os.utime(workspace / SOURCE, None)
        after_mtime = (workspace / SOURCE).stat().st_mtime_ns
        assert sha((workspace / SOURCE).read_bytes()) == mutant['source_sha256']
        command = ['cargo', 'test', '--release', '--locked', '--offline', '-p', 'typst-core', '--lib', 'p1336_tests', '--', '--nocapture']
        started, t = utc(), time.monotonic()
        print(json.dumps({'starting': ident, 'revision': 2, 'at': started}), flush=True)
        run_error = None
        with (output / 'test.stdout').open('x') as stdout, (output / 'test.stderr').open('x') as stderr:
            try:
                p = subprocess.run(command, cwd=workspace, env=env, stdout=stdout, stderr=stderr, timeout=2700)
                exit_code = p.returncode
            except (subprocess.TimeoutExpired, OSError) as exc:
                exit_code, run_error = None, repr(exc)
        out, err = (output / 'test.stdout').read_text(), (output / 'test.stderr').read_text()
        compiled = 'Compiling typst-core v0.1.0 (' + str(workspace / '01_core') + ')' in err
        valid = compiled and run_error is None and 'Finished `release`' in err and 'running 5 tests' in out
        executable = re.search(r'Running unittests src/lib.rs \(([^)]+)\)', err)
        executable_path = executable.group(1) if executable else None
        executable_sha = sha(Path(executable_path).read_bytes()) if executable_path else None
        record = {**mutant, 'revision': 2, 'at_start': started, 'at_end': utc(), 'seconds': time.monotonic()-t,
                  'argv': command, 'cwd': str(workspace), 'env': {'CARGO_TARGET_DIR': str(base / 'target'), 'CARGO_BUILD_JOBS': '2'},
                  'mtime_before_ns': before_mtime, 'mtime_after_ns': after_mtime,
                  'exit': exit_code, 'execution_error': run_error, 'compiled_typst_core': compiled,
                  'valid_execution': valid, 'executable_path': executable_path, 'executable_sha256': executable_sha,
                  'stdout_path': str(output / 'test.stdout'), 'stderr_path': str(output / 'test.stderr'),
                  'stdout_sha256': sha(out.encode()), 'stderr_sha256': sha(err.encode()),
                  'runner_sha256': script_sha, 'manifest_sha256': prepared['manifest_sha256'],
                  'baseline_sha256': prepared['baseline_sha256'], 'candidate_sha256': prepared['candidate_sha256'],
                  'tests_sha256': prepared['tests_sha256'], 'final_l0_sha256': prior['final_l0_sha256'],
                  'execution_provenance': prior['execution_provenance'],
                  'status': 'failed_test_pending_witness_review' if valid and exit_code == 101 else 'survived' if valid and exit_code == 0 else 'Unknown'}
        publish(DIAG / f'p1336-attacks-run-{ident}-r2.json', record)
        aggregate['runs'].append(record)
        print(json.dumps({'finished': ident, 'status': record['status'], 'seconds': record['seconds'], 'binary_sha256': executable_sha}), flush=True)
        if not valid:
            aggregate['stop_reason'] = 'same instrumentation cause requires method review before another run'
            break
    publish(DIAG / 'p1336-attacks-results-r2.json', aggregate)
