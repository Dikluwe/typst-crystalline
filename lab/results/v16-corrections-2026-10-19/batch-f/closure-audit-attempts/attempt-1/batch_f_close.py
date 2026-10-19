#!/usr/bin/env python3
"""Fail closed: independently audit Batch F logs, witnesses and frozen replays.

Run from repository root after final validation and all three accepted replays.
Optional --classification paths register additional genuine precode RED receipts.
No productive file, linter configuration, test expectation or old receipt is edited.
"""
from pathlib import Path
from collections import Counter
import argparse, copy, datetime, difflib, gzip, hashlib, json, re, subprocess

ROOT = Path.cwd().resolve()
OUT = ROOT / 'lab/results/v16-corrections-2026-10-19/batch-f'
ORACLE = '9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
LINTER = '421d0797e7ae1a5102e9c5bcd87b488fea70a1997394c38765a03239fe579121'
EVIDENCE = {}
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
now = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
canonical = lambda x: json.dumps(x, sort_keys=True, ensure_ascii=False, separators=(',', ':'), allow_nan=False)

def equal(a, b):
    # bool and int must never compare equal through Python's == coercion.
    return canonical(a) == canonical(b)

def file(path):
    p = Path(path)
    p = p if p.is_absolute() else OUT / p
    p = p.resolve()
    p.relative_to(ROOT)
    assert p.is_file(), f'missing required evidence: {p}'
    EVIDENCE[str(p.relative_to(ROOT))] = sha(p)
    return p

def read(path):
    p = file(path)
    return json.loads(gzip.decompress(p.read_bytes()) if p.suffix == '.gz' else p.read_bytes())

def text(path):
    return file(path).read_text()

def same_inputs(inputs):
    assert isinstance(inputs, dict) and inputs
    for p, h in inputs.items():
        assert (ROOT / p).is_file() and sha(ROOT / p) == h, p

def resource(row):
    mem = row['memory']
    assert mem['memory.max'] == str(12 * 1024**3)
    assert mem['memory.high'] == str(8 * 1024**3)
    assert mem['memory.swap.max'] == '0'
    events = dict(line.split() for line in mem['memory.events'].splitlines())
    assert all(int(events[k]) == 0 for k in ['oom', 'oom_kill', 'oom_group_kill'])

def test_outcome(log, name):
    return re.findall(r'^test [^\n]*::' + re.escape(name) + r' \.\.\. (ok|FAILED|ignored)\s*$', log, re.M)

def compiler_errors(log):
    errors, finished = [], []
    for line in log.splitlines():
        try:
            item = json.loads(line)
        except ValueError:
            continue
        if item.get('reason') == 'compiler-message' and item['message']['level'] == 'error':
            errors.append(item['message'])
        if item.get('reason') == 'build-finished':
            finished.append(item.get('success'))
    assert finished == [False], finished
    assert errors and all((e.get('code') or {}).get('code') == 'E0004' for e in errors)
    return errors

def locate(content, spec):
    start = content.index(spec['after']) if spec.get('after') else 0
    pos = content.index(spec['anchor'], start)
    return content[:pos].count('\n') + 1, len(content[:pos].encode()), len(content[:pos + len(spec['anchor'])].encode())

def local_errors(errors, target, location, cwd):
    line, begin, end = location
    return [e for e in errors if any(s['is_primary'] and s['file_name'].removeprefix(cwd + '/') == target['file'] and s['line_start'] == line and begin <= s['byte_start'] < end for s in e['spans'])]

def observe_eval(raw, value_key='value'):
    assert not raw.get('timeout')
    result = {k: raw[k] for k in ['returncode', 'stderr']}
    result[value_key if raw['returncode'] == 0 else 'stdout'] = json.loads(raw['stdout']) if raw['returncode'] == 0 else raw['stdout']
    return result

def observe_geometry(raw, case):
    assert not raw.get('timeout')
    stderr = raw['stderr']
    if 'source' in case:
        document = Path(raw['argv'][2])
        stderr = stderr.replace(str(document), '<probe-document>')
        if document.is_absolute() and document.is_relative_to(ROOT):
            stderr = stderr.replace(str(document.relative_to(ROOT)), '<probe-document>')
    result = {'returncode': raw['returncode'], 'stdout': raw['stdout'], 'stderr': stderr}
    if 'source' in case and raw['returncode'] == 0:
        result['value'] = json.loads(result.pop('stdout'))
    return result

def bind_state(replay, state):
    path = OUT / 'validation/validation-state.json.gz'
    assert replay['validation_state'] == {'path': str(path), 'sha256': sha(path)}
    assert replay['head'] == state['checkpoint']
    if 'head_after' in replay:
        assert replay['head_after'] == replay['head']
    if 'validation_state_sha256_after' in replay:
        assert replay['validation_state_sha256_after'] == sha(path)
    assert replay.get('utc_end')
    assert 'changed_build_inputs' in replay or 'changed_inputs' in replay
    assert replay.get('changed_build_inputs', replay.get('changed_inputs')) == []

def bind_binaries(replay):
    for name, p in [('product', ROOT / 'lab/target/product/debug/typst'), ('oracle', ROOT / 'lab/typst-original/target/release/typst')]:
        assert replay['binaries'][name] == {'path': str(p), 'sha256': sha(p)}
        assert replay['binary_sha256_after'][name] == sha(p)
    assert replay['binaries']['oracle']['sha256'] == ORACLE

def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--classification', action='append', default=[], help='Additional classification JSON path relative to batch-f; baseline is always audited.')
    ap.add_argument('--output', default='closure-ledger.json')
    args = ap.parse_args()
    destination = OUT / args.output
    assert not destination.exists(), 'closure evidence is append-only; select a fresh output'
    assert not equal(True, 1) and not equal({'v': False}, {'v': 0})
    scope = read('scope.json')
    planned = {f'V16-{n:03}' for n in range(53, 71)}
    ids, pending = set(scope['ids']), set(scope['pending_ids'])
    assert len(ids) == len(scope['ids']) and len(pending) == len(scope['pending_ids'])
    assert ids.isdisjoint(pending) and ids | pending == planned == set(scope['planned_ids'])
    assert pending == {'V16-055', 'V16-065'}, 'Figure and Circle may not be silently certified'
    assert scope['shared_nuclei'] == []
    sources = set(scope['sources'])
    owners = {p.replace('01_core/src/', '00_nucleo/prompts/core/').removesuffix('.rs') + '.md' for p in sources}
    assert len(sources) == len(scope['sources']) == len(scope['owners']) and owners == set(scope['owners'])
    assert all(p.startswith('01_core/src/') and p.endswith('.rs') for p in sources)
    decision = read('scope-decision.json')
    assert set(decision['pending']) == {'V16-055'}
    revision = read('scope-revision.json')
    assert pending == set(revision['pending_ids']) and ids == set(revision['accepted_ids'])
    # Deferred owners are unchanged, including public contracts and test contents.
    for p in ['01_core/src/compiler/stdlib/figure_image.rs', '01_core/src/compiler/stdlib/shapes/ellipse.rs', '00_nucleo/prompts/core/compiler/stdlib/figure_image.md', '00_nucleo/prompts/core/compiler/stdlib/shapes/ellipse.md']:
        assert p not in sources | owners
        assert (ROOT / p).read_bytes() == subprocess.check_output(['git', 'show', 'HEAD:' + p])
    state = read('validation/validation-state.json.gz')
    same_inputs(state['inputs'])
    assert len(state['inputs']) == 2132
    assert state['checkpoint'] == subprocess.check_output(['git','rev-parse','HEAD'], text=True).strip()
    assert state['oracle_sha256'] == ORACLE and state['linter_sha256'] == LINTER
    assert sha(ROOT / 'lab/typst-original/target/release/typst') == ORACLE
    assert sha(ROOT / 'lab/target/linter/debug/crystalline-lint') == LINTER
    assert sources | owners <= set(state['inputs'])
    gates = ['python-ci-tests','layout','strict-lineage','reverse-lineage','build','tests-domain-infra','tests-wiring','doctests','build-all-targets','clippy','fmt','rustdoc']
    commands = {
        'python-ci-tests':['python3','-m','unittest','discover','-s','lab/tools/ci'],
        'layout':['python3','lab/tools/ci/check_mature_layout.py'],
        'strict-lineage':[str(ROOT/'lab/target/linter/debug/crystalline-lint'),'.','--checks','v1,v5,v15,v26','--fail-on','warning'],
        'reverse-lineage':['python3','lab/tools/ci/check_reverse_lineage.py'],
        'build':['cargo','build','--offline','--locked','--workspace'],
        'tests-domain-infra':['cargo','test','--offline','--locked','--workspace','--exclude','typst-wiring','--lib','--bins','--tests','--no-fail-fast','--','--test-threads=1'],
        'tests-wiring':['cargo','test','--offline','--locked','-p','typst-wiring','--bins','--tests','--no-fail-fast','--','--test-threads=1'],
        'doctests':['cargo','test','--offline','--locked','--workspace','--doc','--no-fail-fast','--','--test-threads=1'],
        'build-all-targets':['cargo','build','--offline','--locked','--workspace','--all-targets'],
        'clippy':['cargo','clippy','--offline','--locked','--workspace','--all-targets','--','-D','warnings','-D','unfulfilled_lint_expectations'],
        'fmt':['cargo','fmt','--check','--all'],
        'rustdoc':['cargo','doc','--offline','--locked','--workspace','--no-deps'],
    }
    jobs = read('validation/validation-commands.json')
    assert [r['name'] for r in jobs] == gates
    logs = {}
    for row in jobs:
        name = row['name']
        assert row['exit'] == 0 and row['command'] == commands[name], name
        for k,v in {'CARGO_BUILD_JOBS':'1','CARGO_PROFILE_DEV_DEBUG':'0','CARGO_PROFILE_TEST_DEBUG':'0','CARGO_INCREMENTAL':'0','TYPST_VANILLA':str(ROOT/'lab/typst-original/target/release/typst')}.items():
            assert row['environment'][k] == v, (name,k)
        overrides = {'CRYSTALLINE_PDF_FIXED_EPOCH':'1792281600'} if name == 'tests-domain-infra' else {'RUSTDOCFLAGS':'-D warnings'} if name == 'rustdoc' else {}
        assert row['overrides'] == overrides
        logs[name] = text('validation/' + name + '.log')
        assert logs[name].strip(), name
        assert 'error[E' not in logs[name] and 'could not compile' not in logs[name], name
    freeze = read('validation/validation-freeze.json')
    assert freeze['changed_inputs'] == [] and freeze['completed_jobs'] == freeze['expected_jobs'] == 12
    assert freeze['input_count'] == len(state['inputs'])
    plan, outer = read('validation-jobs.json'), read('validation-run/commands.json')
    assert len(plan) == len(outer) == 3
    assert outer[2]['command'] == ['python3','lab/results/v16-corrections-2026-10-19/workers/batch_f_replays.py']
    assert outer[0]['command'] == ['cargo','clean','-p','typst-core','-p','typst-shell','-p','typst-infra','-p','typst-wiring']
    assert outer[1]['command'] == ['python3','lab/tools/ci/validate_layout_migration.py','--output',str(OUT/'validation')]
    for row, required in zip(outer, plan):
        for key in ['name','command','cwd','environment','expected_exit']:
            assert row.get(key, 0 if key == 'expected_exit' else None) == required.get(key, 0 if key == 'expected_exit' else None)
        assert row['exit'] == row.get('expected_exit', 0) == 0
        resource(row)
        text('validation-run/' + row['name'] + '.log')
    assert read('validation-run/freeze.json')['changed'] == []
    same_inputs(read('validation-run/state.json.gz')['inputs'])
    # Genuine precode test outcomes; initial compile setup failure grants no RED.
    assert read('red-run/freeze.json')['changed'] == []
    red_jobs, red_plan = read('red-run/commands.json'), read('red-jobs.json')
    assert len(red_jobs) == len(red_plan) == 2
    for row, required in zip(red_jobs, red_plan):
        assert row['name'] == required['name'] and row['command'] == required['command']
        assert row['exit'] == required['expected_exit'] == 101
        resource(row)
    original = read('baseline-test-classification.json')
    assert original['counts'] == {'semantic RED':25,'preservation':31}
    assert len(original['tests']) == 56
    classifications = [original] + [read(p) for p in args.classification]
    rows = [row for receipt in classifications for row in receipt['tests']]
    assert len({row['name'] for row in rows}) == len(rows)
    for receipt in classifications:
        assert dict(Counter(row['classification'] for row in receipt['tests'])) == receipt['counts']
        for row in receipt['tests']:
            log = text(row['log'])
            assert 'error[E' not in log and 'could not compile' not in log
            expected = 'FAILED' if row['classification'] == 'semantic RED' else 'ok'
            assert row['classification'] in ['semantic RED','preservation']
            assert row['baseline_outcome'] == expected and test_outcome(log,row['name']) == [expected]
            if 'log_sha256' in row:
                assert row['log_sha256'] == sha(OUT/row['log'])
    tests = read('new-tests.json')
    assert len({t['name'] for t in tests}) == len(tests)
    baseline_by_name = {row['name']:row for row in rows}
    observed = []
    final = logs['tests-domain-infra']
    assert 'test result: FAILED' not in final
    assert not re.findall(r'^test .* \.\.\. (FAILED|ignored)\s*$', '\n'.join(line for line in final.splitlines() if 'batch_f_' in line), re.M)
    for t in tests:
        assert t['file'] in sources
        row = baseline_by_name[t['name']]
        assert t['file'] == row['file'] and t['expected_baseline'] == row['expected_baseline']
        assert row['classification'] == ('semantic RED' if t['expected_baseline'] == 'semantic RED' else 'preservation')
        assert test_outcome(final,t['name']) == ['ok']
        observed.append({**t,'baseline_outcome':row['baseline_outcome'],'classification':row['classification'],'final_outcome':'ok'})
    excluded = [r for r in original['tests'] if r['name'] not in {t['name'] for t in tests}]
    assert len(excluded) == 4 and all(r['file'].endswith('/shapes/ellipse.rs') for r in excluded)
    old_test = 'p881_type_as_callable_mapeia_tipos_construtores'
    assert test_outcome(text('red-run/existing-adapter-red.log'), old_test) == ['FAILED']
    assert test_outcome(final, old_test) == ['ok']
    # The first full run exposed one obsolete legacy expectation. Keep its
    # failure visible and require a separate true RED for its stronger successor.
    legacy_validated_state = read('version-ordering-failure/validation/validation-state.json.gz')
    def historical_inputs(inputs):
        for p,h in inputs.items():
            assert p in legacy_validated_state['inputs'] and h == legacy_validated_state['inputs'][p], p
    legacy = read('agents/geometry/legacy-grid-test.json')
    legacy_name = 'p633_grid_columns_invalid_string_rejected'
    assert legacy['new_test'] == legacy_name and legacy['old_test'] == 'p633_grid_columns_string_silent_auto'
    assert legacy['source'] == '01_core/src/compiler/eval/tests.rs' and legacy['source'] in sources
    assert legacy['owner'] == '00_nucleo/prompts/core/compiler/eval/tests.md' and legacy['owner'] in owners
    assert legacy_name not in {t['name'] for t in tests}
    assert test_outcome(final,legacy_name) == ['ok'] and test_outcome(final,legacy['old_test']) == []
    assert sha(file(ROOT/legacy['measurement']['path'])) == legacy['measurement']['sha256']
    measurement = read(ROOT/legacy['measurement']['path'])
    assert measurement['oracle_binary']['sha256'] == ORACLE and measurement['returncode'] == 1
    assert measurement['expected_utf8_range'] == [23,28]
    legacy_original = read('agents/geometry/stale-grid-regression/before-source.json.gz')
    assert set(legacy_original) == {legacy['source'],legacy['owner']}
    for p,original_text in legacy_original.items():
        assert hashlib.sha256(original_text.encode()).hexdigest() == legacy['before'][p]
        pattern = r'^//! @prompt-hash .*$' if p.endswith('.rs') else r'^Hash do Código: .*$'
        original_header = re.findall(pattern,original_text,re.M)
        assert len(original_header) == 1
        current = (ROOT/p).read_text()
        assert len(re.findall(pattern,current,re.M)) == 1
        before_reseal = re.sub(pattern,lambda _:original_header[0],current,flags=re.M)
        assert hashlib.sha256(before_reseal.encode()).hexdigest() == legacy['after'][p], p
    archived_state = read('validation-legacy-grid-failure/validation-state.json.gz')
    for p,h in legacy['before'].items():
        assert archived_state['inputs'][p] == h
    archive_mapping = read('validation-legacy-grid-failure/path-mapping.json')
    assert archive_mapping['validation'] == 'validation-legacy-grid-failure'
    assert archive_mapping['validation-run'] == 'validation-run-legacy-grid-failure'
    archived_jobs = read('validation-legacy-grid-failure/validation-commands.json')
    assert [r['name'] for r in archived_jobs] == gates[:6]
    assert [r['exit'] for r in archived_jobs] == [0,0,0,0,0,101]
    archive_freeze = read('validation-legacy-grid-failure/validation-freeze.json')
    assert archive_freeze['changed_inputs'] == [] and archive_freeze['completed_jobs'] == 6 and archive_freeze['expected_jobs'] == 12
    archived_log = text('validation-legacy-grid-failure/tests-domain-infra.log')
    assert test_outcome(archived_log,legacy['old_test']) == ['FAILED']
    assert re.findall(r'^test ([^\n]+) \.\.\. FAILED\s*$',archived_log,re.M) == ['compiler::eval::tests::regressions::'+legacy['old_test']]
    assert 'error[E' not in archived_log and 'could not compile' not in archived_log
    assert read('validation-run-legacy-grid-failure/freeze.json')['changed'] == []
    for row in read('validation-run-legacy-grid-failure/commands.json'):
        resource(row)
    # Audit the complete copied workspace and only the two explicit HEAD restores.
    legacy_setup = read('legacy-grid-red/inputs.json.gz')
    legacy_result = read('legacy-grid-red/result.json')
    legacy_command = ['cargo','test','--offline','--locked','-p','typst-core','--lib',legacy_name,'--','--test-threads=1']
    assert legacy_setup['head'] == state['checkpoint']
    assert legacy_setup['cwd'] == legacy_result['cwd'] == str(ROOT/'lab/scratch/v16-batch-f-legacy-grid-red')
    restore_paths = {'01_core/src/compiler/stdlib/layout/grid.rs','01_core/src/compiler/stdlib/structural/table_grid.rs'}
    assert set(legacy_setup['restores']) == restore_paths == set(legacy['red_recipe']['restore_from_head'])
    for p,h in legacy_setup['inputs_before_restore'].items():
        if p in legacy_validated_state['inputs']:
            assert h == legacy_validated_state['inputs'][p], p
    assert legacy_setup['inputs_before_restore'][legacy['source']] == legacy_validated_state['inputs'][legacy['source']]
    for p,r in legacy_setup['restores'].items():
        assert r['original_current_sha256'] == legacy_setup['inputs_before_restore'][p] == legacy_validated_state['inputs'][p]
        original_bytes = subprocess.check_output(['git','show','HEAD:'+p])
        assert r['content'].encode() == original_bytes
        assert r['restored_sha256'] == hashlib.sha256(original_bytes).hexdigest()
    assert legacy_result['command'] == legacy_command and legacy_result['exit'] == 101
    assert legacy_result['outcomes'] == ['FAILED'] and legacy_result['changed'] == []
    assert legacy_result['inputs_sha256'] == sha(OUT/'legacy-grid-red/inputs.json.gz')
    assert legacy_result['log_sha256'] == sha(file('legacy-grid-red/test.log'))
    legacy_red_log = text('legacy-grid-red/test.log')
    assert test_outcome(legacy_red_log,legacy_name) == ['FAILED']
    assert 'error[E' not in legacy_red_log and 'could not compile' not in legacy_red_log
    assert 'called `Result::unwrap_err()` on an `Ok` value' in legacy_red_log
    legacy_plan,legacy_jobs = read('legacy-grid-jobs.json'),read('legacy-grid-run/commands.json')
    assert len(legacy_plan) == 3 and len(legacy_jobs) == 2
    assert [r['command'] for r in legacy_plan] == [
        ['python3','lab/results/v16-corrections-2026-10-19/workers/batch_f_legacy_red.py'],
        legacy_command,
        ['python3','lab/results/v16-corrections-2026-10-19/workers/batch_f_structural.py','final-green'],
    ]
    assert [r['exit'] for r in legacy_jobs] == [0,101]
    for row,required in zip(legacy_jobs,legacy_plan):
        assert row['name'] == required['name'] and row['command'] == required['command']
        resource(row)
        text('legacy-grid-run/'+row['name']+'.log')
    assert read('legacy-grid-run/freeze.json')['changed'] == []
    historical_inputs(read('legacy-grid-run/state.json.gz')['inputs'])
    rejected_green = text('legacy-grid-run/legacy-grid-green.log')
    assert test_outcome(rejected_green,legacy_name) == ['FAILED']
    assert 'called `Result::unwrap_err()` on an `Ok` value' in rejected_green
    assert not re.search(r'^\s*Compiling typst-core ',rejected_green,re.M)
    reuse = read('legacy-grid-cache-reuse.json')
    assert reuse['status'] == 'Rejected GREEN attempt; shared Cargo artifact reused after isolated RED'
    assert reuse['evidence'] == 'legacy-grid-run/legacy-grid-green.log'
    # Only the new clean compilation may supply focused GREEN evidence.
    clean_plan,clean_jobs = read('legacy-grid-clean-green-jobs.json'),read('legacy-grid-clean-green-run/commands.json')
    assert len(clean_plan) == len(clean_jobs) == 3
    assert [r['command'] for r in clean_plan] == [
        ['cargo','clean','-p','typst-core','-p','typst-shell','-p','typst-infra','-p','typst-wiring'],
        legacy_command,
        ['python3','lab/results/v16-corrections-2026-10-19/workers/batch_f_structural.py','final-green'],
    ]
    assert [r['name'] for r in clean_plan] == ['clean-product-packages','legacy-grid-green','structural-final-green']
    for row,required in zip(clean_jobs,clean_plan):
        assert row['name'] == required['name'] and row['command'] == required['command'] and row['exit'] == 0
        resource(row)
        text('legacy-grid-clean-green-run/'+row['name']+'.log')
    assert read('legacy-grid-clean-green-run/freeze.json')['changed'] == []
    historical_inputs(read('legacy-grid-clean-green-run/state.json.gz')['inputs'])
    accepted_legacy_green = text('legacy-grid-clean-green-run/legacy-grid-green.log')
    assert test_outcome(accepted_legacy_green,legacy_name) == ['ok']
    assert re.search(r'^\s*Compiling typst-core .* \(' + re.escape(str(ROOT/'01_core')) + r'\)',accepted_legacy_green,re.M)
    file(ROOT/'lab/results/v16-corrections-2026-10-19/workers/batch_f_legacy_red.py')
    # Version prerequisite: retain the failed exact policy and add a supplemental
    # RED for the already counted sorted-key test, never a second classification.
    version_name = 'batch_f_higher_order_sorted_key'
    assert sum(t['name'] == version_name for t in tests) == 1
    version_before = read('agents/adapters/version-ordering-repair/before.json')
    version_sources = read('agents/adapters/version-ordering-repair/before-sources.json.gz')
    version_paths = set(version_before['before'])
    assert len(version_paths) == 4 and version_paths <= sources | owners
    assert set(version_sources) == version_paths
    for p,h in version_before['evidence'].items():
        assert sha(file(ROOT/p)) == h
    for p,original_text in version_sources.items():
        assert hashlib.sha256(original_text.encode()).hexdigest() == version_before['before'][p] == legacy_validated_state['inputs'][p]
    assert set(state['inputs']) == set(legacy_validated_state['inputs'])
    assert {p for p in state['inputs'] if state['inputs'][p] != legacy_validated_state['inputs'][p]} == version_paths
    old_replay = read('version-ordering-failure/agents/adapters/accepted-replay/receipt.json')
    rejected_cases = [c for c in old_replay['cases'] if not c['passed']]
    assert len(old_replay['cases']) == 102 and old_replay['counts']['failed'] == 1
    assert len(rejected_cases) == 1 and rejected_cases[0]['id'] == 'baseline.json::version-key'
    rejected = rejected_cases[0]
    assert rejected['policy'] == 'oracle repair'
    assert rejected['observations']['required'] == {'returncode':0,'stderr':'','value':[2,10]}
    assert 'cannot compare version and version' in rejected['results']['product']['stderr']
    assert rejected['results']['product']['returncode'] == 1
    prior_commands = read('version-ordering-failure/validation/validation-commands.json')
    assert [r['name'] for r in prior_commands] == gates and all(r['exit'] == 0 for r in prior_commands)
    assert read('version-ordering-failure/validation/validation-freeze.json')['changed_inputs'] == []
    assert [r['exit'] for r in read('version-ordering-failure/accepted-replay-commands.json')] == [1,0,0]
    assert read('version-ordering-failure/validation-run/freeze.json')['changed'] == []
    read('version-ordering-failure/path-mapping.json')
    version_command = ['cargo','test','--offline','--locked','-p','typst-core','--lib',version_name,'--','--test-threads=1']
    version_red_plan,version_red_jobs = read('version-red-jobs.json'),read('version-red-run/commands.json')
    assert len(version_red_plan) == len(version_red_jobs) == 1
    assert version_red_jobs[0]['command'] == version_red_plan[0]['command'] == version_command
    assert version_red_jobs[0]['exit'] == version_red_plan[0]['expected_exit'] == 101
    resource(version_red_jobs[0])
    version_red_log = text('version-red-run/version-sorted-red.log')
    assert test_outcome(version_red_log,version_name) == ['FAILED']
    assert 'cannot compare version and version' in version_red_log
    assert 'error[E' not in version_red_log and 'could not compile' not in version_red_log
    assert re.search(r'^\s*Compiling typst-core .* \(' + re.escape(str(ROOT/'01_core')) + r'\)',version_red_log,re.M)
    assert read('version-red-run/freeze.json')['changed'] == []
    version_red_state = read('version-red-run/state.json.gz')
    for p,h in version_red_state['inputs'].items():
        if p not in version_paths:
            assert h == state['inputs'][p], p
    version_arm = '        (Value::Version(a), Value::Version(b)) => Ok(a.cmp(b)),\n'
    comparator = '01_core/src/compiler/stdlib/collections/array_predicate.rs'
    version_red_sources = read('agents/encoding/version-red-sources.json.gz')
    assert set(version_red_sources) == version_paths
    for p,t in version_red_sources.items():
        assert hashlib.sha256(t.encode()).hexdigest() == version_red_state['inputs'][p]
    strip_source_header = lambda t: re.sub(r'^//! @prompt-hash .*$', '', t, flags=re.M)
    assert strip_source_header(version_red_sources[comparator]) == strip_source_header(version_sources[comparator])
    for p in version_paths:
        pattern = r'^//! @prompt-hash .*$' if p.endswith('.rs') else r'^Hash do Código: .*$'
        old_header = re.findall(pattern,version_red_sources[p],re.M)
        current = (ROOT/p).read_text()
        assert len(old_header) == len(re.findall(pattern,current,re.M)) == 1
        if p == comparator:
            assert current.count(version_arm) == 1
            current = current.replace(version_arm,'')
        pre_reseal = re.sub(pattern,lambda _:old_header[0],current,flags=re.M)
        assert hashlib.sha256(pre_reseal.encode()).hexdigest() == version_red_state['inputs'][p], p
    version_review = read('agents/geometry/version-ordering-review.json')
    assert version_review['head'] == state['checkpoint'] and version_review['findings'] == []
    assert version_review['status'].startswith('No actionable finding in bounded Version ordering repair')
    for bindings in [version_review['files'],version_review['evidence']]:
        for p,h in bindings.items():
            assert sha(file(ROOT/p)) == h
    version_green_plan,version_green_jobs = read('version-green-jobs.json'),read('version-green-run/commands.json')
    assert len(version_green_jobs) == len(version_green_plan) == 2
    assert [r['command'] for r in version_green_plan] == [version_command,['cargo','build','--offline','--locked','-p','typst-wiring','--bin','typst']]
    for row,required in zip(version_green_jobs,version_green_plan):
        assert row['name'] == required['name'] and row['command'] == required['command'] and row['exit'] == 0
        resource(row)
        text('version-green-run/'+row['name']+'.log')
    assert read('version-green-run/freeze.json')['changed'] == []
    same_inputs(read('version-green-run/state.json.gz')['inputs'])
    assert test_outcome(text('version-green-run/version-sorted-green.log'),version_name) == ['ok']
    assert test_outcome(final,version_name) == ['ok']
    # Reparse actual compiler JSON, including corrected baseline063, independently.
    targets = read('structural-targets.json')
    assert len({t['id'] for t in targets}) == len(targets)
    assert {t['id'] for t in targets if t['id'] in planned} == ids - {'V16-056'}
    assert {'V16-054-selector','V16-059-extract-tracks-rejection'} <= {t['id'] for t in targets}
    t063 = next(t for t in targets if t['id'] == 'V16-063')
    assert t063['baseline']['after'] == t063['green']['after'] == '// Campos de payload cuja presença é estrutural.'
    correction = read('agents/adapters/structural-baseline-063-correction.json')
    assert correction['primary_e0004_count_at_exact_payload_match'] == 0
    assert correction['corrected_target']['baseline'] == t063['baseline']
    for p,h in correction['bound_artifacts'].items():
        # Current target roster subsequently removes deferred065; original raw
        # logs/sources remain immutable and are reparsed rather than trusted.
        if p.endswith('/batch-f/structural-targets.json'):
            continue
        assert sha(file(ROOT/p)) == h
    baseline = read('structural-baseline/results.json')
    assert len(baseline) == 1 and baseline[0]['name'] == 'all-domains' and baseline[0]['exit'] == 101
    assert read('structural-baseline/freeze.json')['changed'] == []
    baseline_original = read('structural-baseline/original-sources.json.gz')
    baseline_changes = read('structural-baseline/all-domains-mutations.json.gz')
    baseline_errors = compiler_errors(text('structural-baseline/all-domains.log'))
    baseline_state = read('baseline-state.json.gz')
    for p, content in baseline_original.items():
        assert hashlib.sha256(content.encode()).hexdigest() == baseline_state['inputs'][p]
    for t in targets:
        if 'baseline' in t:
            content = baseline_changes.get(t['file'], baseline_original[t['file']])
            position = locate(content, t['baseline'])
            assert local_errors(baseline_errors,t,position,baseline[0]['cwd']) == [], t['id']
    structural = read('structural-version-green/results.json')
    assert [r['name'] for r in structural] == ['all-domains','arity-zero','arity-many']
    assert read('structural-version-green/freeze.json')['changed'] == []
    structural_original = read('structural-version-green/original-sources.json.gz')
    structural_inputs = read('structural-version-green/inputs.json.gz')
    for p, content in structural_original.items():
        digest = hashlib.sha256(content.encode()).hexdigest()
        assert digest == structural_inputs['inputs'][p] == state['inputs'][p], p
    for p,h in structural_inputs['frozen_owners'].items():
        assert h == state['inputs'][p], p
    for p,h in structural_inputs['inputs'].items():
        if p in state['inputs']:
            assert h == state['inputs'][p], p
    domains = [('Content','01_core/src/entities/content.rs'),('Value','01_core/src/entities/value.rs'),('Type','01_core/src/entities/value.rs'),('CounterKey','01_core/src/entities/counter.rs'),('Selector','01_core/src/entities/selector.rs')]
    witnesses = []
    for record in structural:
        name = record['name']
        resource(record)
        assert record['exit'] == 101 and not record['other_errors']
        assert record['command'] == ['cargo','test','--offline','--locked','-p','typst-core','--lib','--no-run','--message-format=json']
        for k,v in {'CARGO_BUILD_JOBS':'1','CARGO_PROFILE_DEV_DEBUG':'0','CARGO_PROFILE_TEST_DEBUG':'0','CARGO_INCREMENTAL':'0','CARGO_TARGET_DIR':str(ROOT/'lab/target/product')}.items():
            assert record['environment'][k] == v
        changes = read('structural-version-green/' + name + '-mutations.json.gz')
        expected_changes = {}
        if name == 'all-domains':
            for domain,p in domains:
                content = expected_changes.get(p,structural_original[p])
                at = content.index('\n}', content.index('pub enum ' + domain + ' {'))
                expected_changes[p] = content[:at] + '\n    TektV16EvolutionProbe,' + content[at:]
            expected_targets = [t for t in targets if 'green' in t]
        else:
            p = '01_core/src/compiler/stdlib/foundations/selector.rs'
            content = structural_original[p]
            start = content.index('fn parse_selector_arg(')
            pos = content.index('[] | [_, _, ..]',start)
            retained = '[_, _, ..]' if name == 'arity-zero' else '[]'
            expected_changes[p] = content[:pos] + retained + content[pos+len('[] | [_, _, ..]'):]
            expected_targets = [{'id':'V16-056-'+name,'file':p,'after':'fn parse_selector_arg(','anchor':'match items {'}]
        assert changes == expected_changes, name + ': unexpected compiler mutant'
        assert [t['id'] for t in record['targets']] == [t['id'] for t in expected_targets]
        errors = compiler_errors(text('structural-version-green/' + name + '.log'))
        assert len(errors) == record['error_count']
        assert record['selected_e0004'] == len(expected_targets)
        for actual,required in zip(record['targets'],expected_targets):
            for k,v in required.items():
                assert actual[k] == v
            content = changes.get(actual['file'],structural_original[actual['file']])
            position = locate(content,required.get('green',required))
            assert position == (actual['line'],actual['byte_start'],actual['byte_end'])
            found = local_errors(errors,actual,position,record['cwd'])
            assert len(found) == 1 and found == actual['diagnostics'], actual['id']
            if name == 'all-domains':
                assert 'TektV16EvolutionProbe' in found[0]['message']
            witnesses.append({'id':actual['id'],'file':actual['file'],'line':actual['line'],'method':name})
    assert ids - {'V16-056'} <= {w['id'] for w in witnesses}
    independent = read('agents/adapters/structural-version-green-independent.json')
    assert independent['status'] == 'PASS independent raw compiler and mutation reparse'
    for bindings in [independent['bound'],*[a['bound'] for a in independent['audits']]]:
        for p,h in bindings.items():
            assert sha(file(ROOT/p)) == h
    assert [a['name'] for a in independent['audits']] == [r['name'] for r in structural]
    assert [len(a['exact_selected']) for a in independent['audits']] == [r['selected_e0004'] for r in structural]
    # Exact SARIF error/warning delta, permitting only line movement of untouched
    # retained findings inside changed consumers; outside-scope notes are exact.
    lint = read('lint-delta.json')
    lint_state = read('lint-state.json.gz')
    same_inputs(lint_state['inputs'])
    assert lint_state['linter_sha256'] == LINTER and lint_state['exit'] == 1
    before = read('baseline.sarif.gz')['runs'][0]['results']
    after = read('after.sarif.gz')['runs'][0]['results']
    cases = read('cases.json')
    assert {c['id'] for c in cases} == ids and len(cases) == len(ids)
    sarif_path = lambda r: r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./')
    locations = {(c['analysis']['file'],c['analysis']['line']) for c in cases}
    removed = [r for r in before if r['ruleId'] == 'V16' and (sarif_path(r),r['locations'][0]['physicalLocation']['region']['startLine']) in locations]
    assert len(removed) == len(ids)
    adjusted = copy.deepcopy(before)
    movements = []
    for r in adjusted:
        if r['ruleId'] != 'V16' or r['level'] not in ['error','warning']:
            continue
        p = sarif_path(r)
        loc = r['locations'][0]['physicalLocation']
        old = loc['region']['startLine']
        if (p,old) in locations or p not in sources:
            continue
        original_lines = subprocess.check_output(['git','show','HEAD:'+p],text=True).splitlines()
        current_lines = (ROOT/p).read_text().splitlines()
        blocks = difflib.SequenceMatcher(a=original_lines,b=current_lines,autojunk=False).get_matching_blocks()
        mapped = [b.b + old-1-b.a + 1 for b in blocks if b.a <= old-1 < b.a+b.size]
        assert len(mapped) == 1, (p,old)
        if old != mapped[0]:
            movements.append((p,old,mapped[0]))
            loc['region']['startLine'] = mapped[0]
    before_bad = Counter(canonical(r) for r in adjusted if r['level'] in ['error','warning'])
    after_bad = Counter(canonical(r) for r in after if r['level'] in ['error','warning'])
    assert before_bad - after_bad == Counter(map(canonical,removed)) and not after_bad - before_bad
    assert Counter(canonical(r) for r in before if sarif_path(r) not in sources) == Counter(canonical(r) for r in after if sarif_path(r) not in sources)
    remaining = sum(r['ruleId'] == 'V16' for r in after)
    assert remaining == sum(r['ruleId'] == 'V16' for r in before) - len(ids) == 21
    assert all(r['ruleId'] == 'V16' and r['level'] == 'error' for r in after if r['level'] in ['error','warning'])
    assert lint['remaining_v16'] == remaining and lint['new_errors_or_warnings'] == 0 and lint['outside_scope_exact']
    assert set(lint['closed_ids']) == ids and set(lint['pending_ids']) == pending
    note_review = read('note-review.json')
    text('note-review.md')
    assert note_review['lint_delta_sha256'] == sha(OUT/'lint-delta.json')
    def substantive_notes(rows):
        return Counter((r['ruleId'],r['message']['text'],r['locations'][0]['physicalLocation']['artifactLocation']['uri']) for r in rows if r['level'] == 'note')
    old_notes,new_notes = substantive_notes(before),substantive_notes(after)
    for key,delta in [('new_substantive_notes',new_notes-old_notes),('removed_substantive_notes',old_notes-new_notes)]:
        reviewed = note_review[key]
        assert len({(r['rule'],r['message'],r['file']) for r in reviewed}) == len(reviewed)
        assert Counter({(r['rule'],r['message'],r['file']):r['count'] for r in reviewed}) == delta
    final_note_review = read('agents/encoding/final-version-note-review.json')
    assert final_note_review['substantive_changes_since_pre_version'] == 0
    assert final_note_review['substantive_added'] == sum((new_notes-old_notes).values())
    assert final_note_review['substantive_removed'] == sum((old_notes-new_notes).values())
    for p,h in final_note_review['bound'].items():
        assert sha(file(ROOT/p)) == h
    accepted_build = read('accepted-build-receipt.json')
    assert accepted_build['head'] == state['checkpoint']
    assert accepted_build['state'] == {'path':str(OUT/'validation/validation-state.json.gz'),'sha256':sha(OUT/'validation/validation-state.json.gz')}
    assert accepted_build['commands'] == {'path':str(OUT/'validation/validation-commands.json'),'sha256':sha(OUT/'validation/validation-commands.json')}
    assert accepted_build['product'] == {'path':str(ROOT/'lab/target/product/debug/typst'),'sha256':sha(ROOT/'lab/target/product/debug/typst')}
    replay_jobs = read('accepted-replay-commands.json')
    required_replays = [
        ['python3',str(OUT/'agents/adapters/replay.py'),'--output',str(OUT/'agents/adapters/accepted-replay'),'--validation-state',str(OUT/'validation/validation-state.json.gz'),'--build-receipt',str(OUT/'accepted-build-receipt.json')],
        ['python3',str(OUT/'agents/geometry/replay.py'),'--output',str(OUT/'agents/geometry/accepted-replay'),'--validation-state',str(OUT/'validation/validation-state.json.gz')],
        ['python3',str(OUT/'agents/encoding/replay.py'),'accepted-replay.json',str(OUT/'validation/validation-state.json.gz'),str(OUT/'accepted-build-receipt.json'),sha(ROOT/'lab/target/product/debug/typst')],
    ]
    assert replay_jobs == [{'command':command,'exit':0} for command in required_replays]
    # Strict adapter replay: frozen cases and outcomes, independent raw parsing.
    adapter_root = OUT/'agents/adapters'
    manifest = read('agents/adapters/replay-manifest.json')
    corpus = []
    for entry in manifest['baseline_receipts']:
        p = adapter_root/entry['path']
        assert sha(file(p)) == entry['sha256']
        receipt = read(p)
        assert len(receipt['cases']) == entry['case_count']
        corpus.extend((entry['path']+'::'+c['name'],c) for c in receipt['cases'])
    for entry in manifest['supporting_evidence']:
        assert sha(file(ROOT/entry['path'])) == entry['sha256']
    for entry in manifest['fixtures']:
        p = ROOT/entry['path']
        if entry.get('absent'):
            assert not p.exists()
        else:
            assert sha(file(p)) == entry['sha256']
    assert len(corpus) == len(manifest['cases']) == 102
    assert {key for key,_ in corpus} == set(manifest['cases'])
    adapters = read('agents/adapters/accepted-replay/receipt.json')
    bind_state(adapters,state)
    bind_binaries(adapters)
    assert adapters['passed'] is True and adapters['counts']['failed'] == 0
    assert adapters['runner_sha256'] == sha(file(adapter_root/'replay.py'))
    assert adapters['manifest'] == {'path':str(adapter_root/'replay-manifest.json'),'sha256':sha(adapter_root/'replay-manifest.json')}
    assert adapters['manifest_sha256_after'] == adapters['manifest']['sha256']
    assert adapters['build_receipt']['path'] == str(OUT/'accepted-build-receipt.json')
    assert sha(file(adapters['build_receipt']['path'])) == adapters['build_receipt']['sha256'] == adapters['build_receipt_sha256_after']
    assert [c['id'] for c in adapters['cases']] == [key for key,_ in corpus]
    adapter_counts = Counter()
    for actual,(key,baseline_case) in zip(adapters['cases'],corpus):
        expected = manifest['cases'][key]
        assert actual['expression'] == baseline_case['expression']
        digest = hashlib.sha256(baseline_case['expression'].encode()).hexdigest()
        assert actual['expression_sha256'] == expected['expression_sha256'] == digest
        assert actual['policy'] == expected['policy'] and actual['reason'] == expected['reason']
        for role in ['product','oracle']:
            argv = [adapters['binaries'][role]['path'],*baseline_case['results'][role]['argv'][1:]]
            assert argv[1:] == ['eval',baseline_case['expression'],'--format','json']
            assert actual['results'][role]['argv'] == argv and actual['results'][role]['cwd'] == str(ROOT)
        observations = {'baseline_product':observe_eval(baseline_case['results']['product']), 'baseline_oracle':observe_eval(baseline_case['results']['oracle']), 'candidate_product':observe_eval(actual['results']['product']), 'fresh_oracle':observe_eval(actual['results']['oracle']), 'required':expected['expected_observation']}
        assert equal(expected['baseline_observation'],observations['baseline_product'])
        assert equal(actual['observations'],observations)
        assert equal(observations['fresh_oracle'],observations['baseline_oracle'])
        assert equal(observations['candidate_product'],observations['required']),key
        if expected['policy'] == 'baseline preservation':
            assert equal(observations['required'],observations['baseline_product'])
        elif expected['policy'] == 'oracle repair':
            assert equal(observations['required'],observations['baseline_oracle'])
        else:
            assert expected['policy'] == 'native-constructor residual retained'
        new,old,oracle = (observations[k] for k in ['candidate_product','baseline_product','fresh_oracle'])
        classification = 'PRESERVED' if equal(new,old) else 'REPAIRED' if equal(new,oracle) else 'CHANGED_WITH_RESIDUAL'
        assert actual['classification'] == classification
        assert actual['oracle_preserved'] is True and actual['required_outcome_met'] is True and actual['passed'] is True
        assert actual['matches_oracle'] is equal(new,oracle)
        adapter_counts[classification] += 1
        adapter_counts['oracle_matches'] += equal(new,oracle)
    assert adapters['counts'] == {**{k:adapter_counts[k] for k in ['PRESERVED','REPAIRED','CHANGED_WITH_RESIDUAL','oracle_matches']},'failed':0}
    # Strict geometry replay uses a frozen expectation for every case, including
    # unchanged deferred Circle observations. No new failure is accepted generically.
    geometry_root = OUT/'agents/geometry'
    manifest = read('agents/geometry/replay-manifest.json')
    corpus = []
    for entry in manifest['receipts']:
        p = geometry_root/entry['file']
        assert sha(file(p)) == entry['sha256']
        corpus.extend(read(p)['cases'])
    assert len(corpus) == len({c['name'] for c in corpus}) == 182
    policy_path = geometry_root/manifest['expected_policies']['file']
    assert sha(file(policy_path)) == manifest['expected_policies']['sha256']
    policies_receipt = read(policy_path)
    assert policies_receipt['schema'] == 'batch-f-geometry-expectations-v1'
    policies = {r['name']:r for r in policies_receipt['cases']}
    assert len(policies) == len(policies_receipt['cases']) == len(corpus)
    assert set(policies) == {c['name'] for c in corpus}
    deferral = geometry_root/manifest['deferral_receipt']['file']
    assert sha(file(deferral)) == manifest['deferral_receipt']['sha256']
    read(deferral)
    geometry = read('agents/geometry/accepted-replay/receipt.json')
    bind_state(geometry,state)
    bind_binaries(geometry)
    assert geometry['runner_sha256'] == sha(file(geometry_root/'replay.py'))
    assert geometry['manifest_sha256'] == sha(geometry_root/'replay-manifest.json')
    assert geometry['passed'] is True and [c['name'] for c in geometry['cases']] == [c['name'] for c in corpus]
    geometry_counts = Counter()
    for actual,baseline_case in zip(geometry['cases'],corpus):
        key = baseline_case['name']
        expected = policies[key]
        assert actual['policy'] == expected['policy'] and actual['reason'] == expected['reason']
        assert equal(actual['expected'],expected['expected_observation'])
        if 'source' in baseline_case:
            p = geometry_root/'accepted-replay/sources'/(key+'.typ')
            assert text(p) == actual['source'] == baseline_case['source']
            assert sha(p) == actual['source_sha256'] == baseline_case['source_sha256']
        else:
            assert actual['expression'] == baseline_case['expression']
        for role in ['product','oracle']:
            argv = list(baseline_case['results'][role]['argv'])
            argv[0] = geometry['binaries'][role]['path']
            if 'source' in baseline_case:
                argv[2] = str(p)
            assert actual['results'][role]['argv'] == argv
        old = observe_geometry(baseline_case['results']['product'],baseline_case)
        old_oracle = observe_geometry(baseline_case['results']['oracle'],baseline_case)
        new = observe_geometry(actual['results']['product'],baseline_case)
        oracle = observe_geometry(actual['results']['oracle'],baseline_case)
        assert equal(old_oracle,oracle) and equal(actual['observation'],new)
        assert equal(new,expected['expected_observation']), key
        if expected['policy'] in ['preserve','preserve-deferred-circle']:
            assert equal(new,old)
        elif expected['policy'] == 'repair-oracle-diagnostic':
            assert equal(new,old_oracle)
        else:
            assert expected['policy'] == 'repair-local-legacy-type'
        if key.startswith('circle-'):
            assert expected['policy'] == 'preserve-deferred-circle'
        assert actual['oracle_preserved'] is True and actual['expected_matched'] is True and actual['pass'] is True
        assert actual['matches_oracle'] is equal(new,oracle) and actual['unchanged'] is equal(new,old)
        geometry_counts[expected['policy']] += 1
    assert geometry['counts'] == {'cases':len(corpus),'passed':len(corpus),'oracle_matches':sum(c['matches_oracle'] for c in geometry['cases'])}
    assert geometry_counts['preserve-deferred-circle'] == 25
    # Encoding replay must preserve every product observation and every oracle
    # observation independently; no cross-binary parity is inferred.
    encoding_root = OUT/'agents/encoding'
    encoding = read('agents/encoding/accepted-replay.json')
    bind_state(encoding,state)
    assert encoding['runner_sha256'] == sha(file(encoding_root/'replay.py'))
    assert encoding['passed'] is True and encoding['failures'] == []
    assert encoding['binaries'] == {'product':sha(ROOT/'lab/target/product/debug/typst'),'oracle':ORACLE}
    assert encoding['binaries_after'] == encoding['binaries']
    assert encoding['build_receipt_path'] == str(OUT/'accepted-build-receipt.json')
    build = file(encoding['build_receipt_path'])
    assert encoding['build_receipt_sha256'] == encoding['build_receipt_sha256_after'] == sha(build)
    assert encoding['build_receipt'] == read(build)
    corpus = []
    for name in ['baseline-enabled-probes.json','baseline-html-projection.json']:
        path = encoding_root/name
        corpus.extend((name,sha(file(path)),c) for c in read(path)['cases'])
    assert len(corpus) == len(encoding['cases']) == encoding['preserved_cases'] == 51
    for actual,(name,digest,baseline_case) in zip(encoding['cases'],corpus):
        assert actual['name'] == baseline_case['name'] and actual['expression'] == baseline_case['expression']
        assert actual['baseline_file'] == name and actual['baseline_sha256'] == digest
        for role in ['product','oracle']:
            raw = actual['results'][role]
            argv = list(baseline_case['results'][role]['argv'])
            argv[0] = str(ROOT/('lab/target/product/debug/typst' if role == 'product' else 'lab/typst-original/target/release/typst'))
            assert raw['argv'] == argv
            assert equal(observe_eval(raw,'stdout'),observe_eval(baseline_case['results'][role],'stdout'))
            assert raw['preserved'] is True
    # Pending debt remains explicit; a final receipt cannot convert a prototype
    # proposal into approval or certify the whole repository while21 V16 remain.
    read('open-obligations.json')
    assert read('fg1-proposal/status.json')['status'].startswith('not ready')
    text('fg1-proposal/readiness.md')
    same_inputs(state['inputs'])
    closure = {'utc':now(),'baseline_commit':state['checkpoint'],'branch':subprocess.check_output(['git','branch','--show-current'],text=True).strip(),'validated_input_count':len(state['inputs']),'validation_completed':freeze['utc'],'full_linter_exit':1,'remaining_v16':remaining,'selected_v16_closed':len(ids),'closed_ids':scope['ids'],'pending_ids':scope['pending_ids'],'owner_pairs':len(sources),'new_tests':len(tests),'test_classifications':dict(Counter(r['classification'] for r in observed)),'deferred_original_tests':[{k:r[k] for k in ['file','name','classification','baseline_outcome']} for r in excluded],'existing_adapter_test':'actual semantic RED and final PASS; not counted as a new test','supplemental_version_test':{'name':version_name,'actual_RED':'cannot compare version and version','early_GREEN':'version-green-run','final_outcome':'ok','duplicate_count':False},'strengthened_legacy_grid_test':{'name':legacy_name,'actual_RED':'isolated HEAD grid/table accepts invalid string','final_outcome':'ok','counted_in_new_tests':False},'structural_witnesses':witnesses,'replay_cases':{'adapters':len(adapters['cases']),'geometry':len(geometry['cases']),'encoding':len(encoding['cases'])},'adapter_classifications':adapters['counts'],'geometry_policies':dict(geometry_counts),'encoding_preserved':51,'source_hashes':{p:sha(ROOT/p) for p in sorted(sources|owners)},'evidence':EVIDENCE,'limits':'Scoped repair only. Deferred Figure055 and Circle065 remain visible; preserved baseline semantic debt and informational lint notes are not parity certification.'}
    destination.write_text(json.dumps(closure,indent=2)+'\n')
    print(json.dumps({k:v for k,v in closure.items() if k not in ['source_hashes','evidence','structural_witnesses']},indent=2))

if __name__ == '__main__':
    main()
