"""Assemble effective evidence; retain and explicitly exclude stale R1 runs."""
import hashlib
import datetime
import json
from pathlib import Path
import re
import subprocess

ROOT = Path('/repos/Antigravity/typst-crystalline')
DIAG = ROOT / '00_nucleo/diagnosticos'

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def read(name):
    return json.loads((DIAG / name).read_text())

def publish(path, value):
    assert not path.exists()
    body = json.dumps(value, ensure_ascii=False, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n' + ''.join('+' + line + '\n' for line in body.splitlines()) + '*** End Patch\n'
    p = subprocess.run(['apply_patch'], input=patch, capture_output=True, text=True)
    assert p.returncode == 0, p.stderr
    assert path.read_text() == body

if __name__ == '__main__':
    prepared = read('p1336-attacks-prepared.json')
    r1 = read('p1336-attacks-results.json')
    r2 = read('p1336-attacks-results-r2.json')
    binaries = read('p1336-attacks-r1-binaries.json')
    expected = {
        'M1': ['p1336_pure_lookup_names_and_supplied_span', 'p1336_int_ast_identifier_span'],
        'M2': ['p1336_pure_lookup_names_and_supplied_span', 'p1336_str_ast_identifier_span'],
        'M3': ['p1336_int_ast_identifier_span'],
        'M4': ['p1336_str_ast_identifier_span'],
        'M5': ['p1336_preserve_other_fallback_names_and_ast_span'],
        'M6': ['p1336_preserve_int_str_methods_and_type_namespaces'],
    }
    names = ['p1336-manifest.json', 'p1336-baseline.json', 'p1336-candidate.json',
             'p1336-tests-freeze.json', 'p1336-attacks-plan.md', 'p1336-attacks-plan-addendum.md',
             'p1336-attacks-probe.py', 'p1336-attacks-oracles.json', 'p1336-attacks-prepared.json',
             'p1336-attacks-results.json', 'p1336-attacks-r1-invalidation.md',
             'p1336-attacks-r1-binaries.json', 'p1336-attacks-instrument-revision.json',
             'p1336-attacks-mutate.py', 'p1336-attacks-r2.py', 'p1336-attacks-results-r2.json',
             'p1336-attacks-finalize.py']
    names += [f'p1336-attacks-run-{ident}.json' for ident in ['C', 'M1', 'M2', 'M3', 'M4', 'M5', 'M6']]
    names += [f'p1336-attacks-run-{ident}-r2.json' for ident in ['M2', 'M3', 'M4', 'M5', 'M6']]
    final = {k: prepared[k] for k in ['candidate_sha256', 'tests_sha256', 'manifest_sha256', 'baseline_sha256', 'mutants']}
    final.update({'regime': 'A/B executed without technical isolation attestation; no refinement seal',
                  'pins': {name: sha(DIAG / name) for name in names},
                  'final_l0_sha256': r1['final_l0_sha256'], 'execution_provenance': r1['execution_provenance'],
                  'historical_invalid_executions': [{'id': f'{ident}-R1', 'status': 'Unknown', 'reason': 'stale_compiled_mutant', 'credit': False}
                                                    for ident in ['M2', 'M3', 'M4', 'M5', 'M6']],
                  'runs': []})
    effective = [r1['runs'][0], r1['runs'][1]] + r2['runs']
    assert [item['id'] for item in effective] == ['C', 'M1', 'M2', 'M3', 'M4', 'M5', 'M6']
    for original in effective:
        item = dict(original)
        ident = item['id']
        out, err = Path(item['stdout_path']).read_text(), Path(item['stderr_path']).read_text()
        assert sha(item['stdout_path']) == item['stdout_sha256']
        assert sha(item['stderr_path']) == item['stderr_sha256']
        observed = sorted(re.findall(r'test compiler::eval::bindings::field_access::p1336_tests::(\w+) \.\.\. FAILED', out))
        item['failed_tests'] = observed
        item['witness'] = err[err.index("thread '"):] if "thread '" in err else ''
        if ident in binaries:
            item['executable_sha256'] = binaries[ident]['sha256']
            item['preserved_executable'] = binaries[ident]['preserved']
            assert sha(item['preserved_executable']) == item['executable_sha256']
        else:
            item['preserved_executable'] = str(Path(item['stdout_path']).parent / 'test-executable')
            assert sha(item['preserved_executable']) == item['executable_sha256']
        if ident == 'C':
            assert item['exit'] == 0 and '5 passed; 0 failed' in out
            item['effective_status'] = 'Preserved_control'
        else:
            item['compiled_typst_core'] = 'Compiling typst-core v0.1.0 (' + str(Path(prepared['workspace']) / '01_core') + ')' in err
            assert item['compiled_typst_core'] and item['valid_execution'] and item['exit'] == 101
            assert observed == sorted(expected[ident]), (ident, observed)
            assert item.get('executable_sha256')
            if ident in ['M1', 'M2']:
                short, long = ('int', 'integer') if ident == 'M1' else ('str', 'string')
                assert f'left: "cannot access fields on type {short}"' in err
                assert f'right: "cannot access fields on type {long}"' in err
            elif ident in ['M3', 'M4', 'M5']:
                assert 'left: Some(' in err and 'right: Some(' in err
            else:
                assert 'cannot access fields on type integer or string' in err
            item['effective_status'] = 'Violated_by_frozen_tests'
            item['status'] = 'failed_test'
            item['executable_path_policy'] = 'Original target path was mutable; executable_sha256 snapshots the run. preserved_executable is a separate retained copy verified against that hash.'
        final['runs'].append(item)
    final['valid_mutants'] = 6
    final['rejected_mutants'] = 6
    final['mutation_score'] = 1.0
    final['mandatory_unknown_effective'] = 0
    final['r1_invalid_excluded_count'] = 5
    final['elapsed_execution_seconds_all_attempts'] = sum(x['seconds'] for x in r1['runs']) + sum(x['seconds'] for x in r2['runs'])
    final['main_source_sha256_after'] = sha(ROOT / '01_core/src/compiler/eval/bindings/field_access.rs')
    assert final['main_source_sha256_after'] == final['candidate_sha256']
    final['verdict'] = 'Six valid productive mutants rejected by frozen independent tests; pending reviewer judgment; no general equivalence claim.'
    preservation = {'at_verified': datetime.datetime.now(datetime.timezone.utc).isoformat(),
                    'copy_command': 'cp -a --reflink=auto SOURCE DESTINATION',
                    'policy': 'Copies retained in each exclusive run directory after execution, before subsequent linker replacement; original target paths remain mutable. Every retained copy was verified against the SHA recorded at execution or the R1 supplemental capture.',
                    'executables': [{'id': item['id'], 'preserved': item['preserved_executable'], 'sha256': item['executable_sha256']} for item in final['runs']]}
    preservation_path = DIAG / 'p1336-attacks-executable-preservation.json'
    publish(preservation_path, preservation)
    final['pins'][preservation_path.name] = sha(preservation_path)
    path = DIAG / 'p1336-attacks-final.json'
    publish(path, final)
    print(json.dumps({'path': str(path), 'sha256': sha(path), 'valid': 6, 'rejected': 6}))
