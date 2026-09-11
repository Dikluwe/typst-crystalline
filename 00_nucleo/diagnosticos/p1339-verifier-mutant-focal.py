"""Independent real default-profile calibration; never writes a seal."""
import base64
import concurrent.futures
import datetime
import hashlib
import json
import os
from pathlib import Path
import resource
import subprocess

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def checked(ref):
    assert sha(ref['path']) == ref['sha256'], ref['path']
    return Path(ref['path'])

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def main():
    registry_path = D / 'p1339-mutation-registry.json'
    assert sha(registry_path) == '04d5578d720b4c7fff47954c29d20392b7a221f8b0cb80094a3859409add3677'
    registry = json.loads(registry_path.read_text())
    contract = json.loads(checked(registry['contract']).read_text())
    assert registry['contract']['sha256'] == 'c0cd1826679ddaf77e937a677839c5cbe64fdae6ec71bfa34e635d584d9c3e17'
    negatives = json.loads(checked(registry['negative_oracles']).read_text())
    checked(registry['final_focal'])
    checked(registry['authority_manifest'])
    checked(registry['l0_freeze'])
    for path, digest in contract['input_sha256'].items():
        assert sha(path) == digest, path
    for path, digest in contract['l0_raw_sha256'].items():
        assert sha(path) == digest, path
    inventories = []
    for host in [registry['multiplex'], registry['architecture_witness']['control'], registry['architecture_witness']['negative']]:
        checked(host['binary'])
        root = Path(host['source_root'])
        actual = {str(p.relative_to(root)): sha(p) for p in sorted(root.rglob('*'))
                  if p.is_file() and (p.suffix == '.rs' or p.name in ('Cargo.toml', 'Cargo.lock'))}
        assert actual == host['source_sha256'], str(root)
        inventories.append({'source_root': str(root), 'files': len(actual), 'exact': True})
    pinned = checked(registry['pinned_reference'])
    control = checked(registry['architecture_witness']['control']['binary'])
    multiplex = checked(registry['multiplex']['binary'])
    env = {k: os.environ[k] for k in ['PATH', 'HOME', 'LANG', 'TZ', 'FONTCONFIG_FILE', 'FONTCONFIG_PATH', 'TYPST_FONT_PATHS'] if k in os.environ}
    env.update({'LC_ALL': 'C.UTF-8', 'NO_COLOR': '1', 'TERM': 'dumb'})
    jobs = []
    for case, mutant in zip(negatives['cases'], registry['mutants']):
        assert mutant['id'] == f"M{case['family']:02d}"
        checked(mutant['binary'])
        checked(mutant['source'])
        build = json.loads(checked(mutant['build']).read_text())
        assert build['exit'] == 0
        checked(mutant['patch'])
        for role, binary, mode in [('pinned', pinned, 0), ('pure_control', control, 0),
                                   ('mode0', multiplex, 0), ('mutant', Path(mutant['binary']['path']), mutant['mode'])]:
            jobs.append((case['id'], mutant['id'], case['expr'], role, str(binary), mode))
    jobs.append(('invalid-mode', None, '1', 'invalid_mode', str(multiplex), 99))
    def run(job):
        case, family, expression, role, binary, mode = job
        run_env = {**env, 'P1339_MUTANT': str(mode)}
        argv = [binary, '--color=never', 'eval', expression, '--format', 'json']
        row = {'case_id': case, 'family': family, 'role': role, 'profile': 'default',
               'order': 'focal', 'source': expression, 'source_sha256': hashlib.sha256(expression.encode()).hexdigest(),
               'binary_sha256': sha(binary), 'argv': argv, 'env': run_env, 'cwd': str(ROOT), 'start_utc': now()}
        try:
            p = subprocess.run(argv, env=run_env, cwd=ROOT, capture_output=True, timeout=30)
            row.update(exit=p.returncode, signal=-p.returncode if p.returncode < 0 else None,
                       stdout_base64=base64.b64encode(p.stdout).decode(), stderr_base64=base64.b64encode(p.stderr).decode(),
                       stdout=p.stdout.decode('utf-8', 'surrogateescape'), stderr=p.stderr.decode('utf-8', 'surrogateescape'),
                       execution='Observed' if p.returncode in (0, 1) else 'Unknown')
        except subprocess.TimeoutExpired as e:
            row.update(exit=None, execution='Unknown', unknown_reason='timeout',
                       stdout_base64=base64.b64encode(e.stdout or b'').decode(), stderr_base64=base64.b64encode(e.stderr or b'').decode())
        except OSError as e:
            row.update(exit=None, execution='Unknown', unknown_reason=type(e).__name__, detail=str(e))
        row['end_utc'] = now()
        return row
    # Inherited by children; avoids core-file writes on deliberately abnormal M20.
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    start = now()
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        rows = list(pool.map(run, jobs))
    comparisons = []
    def observable(row):
        return tuple(row.get(k) for k in ['execution', 'exit', 'stdout_base64', 'stderr_base64'])
    for offset in range(0, 80, 4):
        reference, pure, zero, mutant = rows[offset:offset+4]
        controls = reference['execution'] == 'Observed' and observable(reference) == observable(pure) == observable(zero)
        family = mutant['family']
        if not controls:
            outcome = 'invalid_control'
        elif family == 'M20':
            outcome = 'mandatory_unknown' if mutant['execution'] == 'Unknown' and mutant.get('signal') else 'not_discriminated'
        elif family == 'M12':
            outcome = 'runtime_equal_structural_review_required' if observable(reference) == observable(mutant) else 'unexpected_runtime_delta'
        elif mutant['execution'] != 'Observed':
            outcome = 'unknown_not_a_behavioral_kill'
        else:
            outcome = 'observable_violation' if observable(reference) != observable(mutant) else 'survived'
        comparisons.append({'family': family, 'case_id': reference['case_id'], 'controls_exact': controls, 'result': outcome})
    provenance = {}
    for key, argv in [('head', ['git','rev-parse','HEAD']), ('diff_stat', ['git','diff','HEAD','--stat']),
                      ('status', ['git','status','--porcelain','--untracked-files=all'])]:
        p = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True)
        assert p.returncode == 0
        provenance[key] = {'argv': argv, 'exit': p.returncode, 'stdout': p.stdout, 'stderr': p.stderr}
    record = {'schema': 'p1339-independent-mutant-focal-v1', 'verifier': '/root/p1311_review',
              'regime': 'executado sem atestação de isolamento', 'start_utc': start, 'end_utc': now(),
              'registry': {'path': str(registry_path), 'sha256': sha(registry_path)},
              'contract': registry['contract'], 'runner_sha256': sha(__file__), 'inventories': inventories,
              'provenance': provenance, 'rows': rows, 'comparisons': comparisons,
              'invalid_mode_fail_closed': rows[-1]['exit'] not in (None, 0),
              'preseal_full_runs_used': 0, 'mutation_score': None, 'seal': None,
              'limits': ['Default-profile focal only, not full corpus/order/profile matrix.',
                         'M12 graph/source needs independent structural verdict; equal output alone is not rejection.',
                         'M20 raw Unknown is retained and gate-rejected, never preservation credit.',
                         'No candidate tested; no seal or implementation authorization.']}
    output = D / 'p1339-verifier-mutant-focal-r1.json'
    assert not output.exists()
    payload = json.dumps(record, ensure_ascii=True, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(output) + '\n' + ''.join('+'+line+'\n' for line in payload.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, cwd=ROOT, check=True, capture_output=True)
    print(json.dumps({'output': str(output), 'sha256': sha(output), 'comparisons': comparisons,
                      'invalid_mode_fail_closed': record['invalid_mode_fail_closed']}))

if __name__ == '__main__':
    main()
