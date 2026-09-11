"""Read-only aggregation of existing oracle-author publications; no product execution."""
import collections
import datetime
import hashlib
import json
import pathlib
import subprocess

D = pathlib.Path(__file__).resolve().parent
ROOT = D.parent.parent
OUT = D / 'p1339-ab-publication-budget-ledger-r1.json'

def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def utc(ts):
    return datetime.datetime.fromtimestamp(ts, datetime.timezone.utc).isoformat()

def pin(p):
    return {'path': str(p), 'sha256': sha(p)}

def vector(rows):
    counts = collections.Counter((r['product'], r['profile'], r['order'], r['execution'], r['exit']) for r in rows)
    return [dict(product=k[0], profile=k[1], order=k[2], execution=k[3], exit=k[4], count=v)
            for k, v in sorted(counts.items(), key=lambda kv: repr(kv[0]))]

assert not OUT.exists(), 'Immutable ledger already exists'
files = sorted(D.glob('p1339-ab-*'))
publications = []
for p in files:
    if not p.is_file():
        continue
    item = {**pin(p), 'bytes': p.stat().st_size,
            'filesystem_mtime_utc': utc(p.stat().st_mtime),
            'publication_utc': None,
            'timestamp_caveat': 'mtime is observed metadata, not an attested original publication time'}
    if p.suffix == '.json':
        data = json.loads(p.read_text())
        item['schema'] = data.get('schema')
        for key in ('cases', 'oracles', 'conditional_oracles', 'fixtures'):
            if isinstance(data.get(key), (list, dict)):
                item[key + '_count'] = len(data[key])
    publications.append(item)

causes = {
    'transport-probe-r1': ('Probe compile/query/encoding transports on four ancestor sources',
                         '32 baseline UnknownProcessExit=2: compile stdin dash interpreted as file; not a semantic negative'),
    'transport-probe-r2a': ('Use named immutable files and actual byte hashes',
                          '0 unknown raw observations; fixes transport only, no semantic approval'),
    'bridge-focal-r1': ('Assert integral metadata argument and complementary error while returning original metadata',
                       'Vanilla 24 equal successes and 24 complementary errors; transient assertions discarded and A5 histories retained; original bare-Strong ancestors do not gain vanilla parity from this result'),
    'w02-derived-probe-r1': ('New explicitly filtered consumers, preserving original bare-demand ancestors separately',
                           '13 vanilla and 13 baseline observations; successful query transport alone is not positive parity credit'),
    'cli-focal-r1': ('Calibrate supplement and derived-source compile/value carriers',
                    '0 raw Unknown in 1174 processes; policy defects later found in direct-show target and empty-vs-bare causal classification; no complete-suite verdict'),
    'opaque-focal-r1': ('Try a loop as deliberate bounded-observation opacity with finite control',
                       'Both products return known errors; vanilla rejects constant true and baseline reaches its loop guard; failed bilateral-opacity hypothesis'),
    'opaque-focal-r2': ('Try finite nested workload under one-second observation budget',
                       'Vanilla four raw Unknown timeouts; baseline four known loop-guard errors; finite controls eight successes; second failed bilateral-opacity hypothesis, requires design review rather than further tuning'),
}
runs = []
for p in sorted(D.glob('p1339-ab-*-runs.json')):
    data = json.loads(p.read_text())
    rows = data['rows']
    name = p.name.removeprefix('p1339-ab-').removesuffix('-runs.json')
    hypothesis, cause = causes[name]
    runs.append({**pin(p), 'hypothesis': hypothesis, 'result_and_reason': cause,
                 'processes': data['processes'], 'row_count': len(rows),
                 'wall_seconds': data['wall_seconds'],
                 'sum_child_process_seconds': sum(r['seconds'] for r in rows),
                 'measurement_start_utc': data['before']['utc'],
                 'first_child_start_utc': min(r['start'] for r in rows),
                 'last_child_end_utc': max(r['end'] for r in rows),
                 'measurement_end_utc': data['after']['utc'],
                 'head': data['before']['head'],
                 'working_tree_provenance': {'artifact': pin(p), 'selectors': ['before.status', 'before.diff_stat', 'after.status', 'after.diff_stat']},
                 'raw_vector': vector(rows),
                 'case_count': len({r['case_id'] for r in rows}),
                 'raw_unknown_count': sum(r['execution'] != 'Observed' for r in rows),
                 'phase': data['phase'], 'orders': sorted({r['order'] for r in rows}),
                 'plan_sha256': data['plan_sha256'], 'runner_sha256': data['runner_sha256'],
                 'binaries': data['binaries'],
                 'reproduction': 'Pinned runner/plan hashes above; each raw row retains full argv, env, cwd, stdin, source hash, executable hash, timestamps and outputs. Re-execution is paused, not authorized by this ledger.',
                 'positive_preserved': None, 'negative_violated': None, 'opaque_correct_unknown': None,
                 'classification_caveat': 'Raw-vector aggregation only. No new semantic classification, mutant score, equivalence verdict or preservation credit is assigned here.'})
runs.sort(key=lambda r: r['measurement_start_utc'])

def command(args):
    return subprocess.check_output(args, cwd=ROOT, text=True)

events = [
    {'id': 'transport-r2-preflight-abort', 'product_processes': 0, 'utc': None,
     'reason_code': 'fixture_terminal_newline_hash_mismatch',
     'evidence': pin(D / 'p1339-ab-transport-preparation-r1.json'),
     'detail': 'Six source hashes disagreed with apply_patch terminal-newline materialization. r2 preserved, r2a successor corrected bytes. Technical attempt is retained, not declared free.'},
    {'id': 'cli-generator-r1-abort', 'product_processes': 0, 'utc': None,
     'reason_code': 'constructor_projection_expression_schema',
     'evidence': pin(D / 'p1339-ab-prepare-cli-r1.py'),
     'detail': 'Generator encountered constructor_projection with no source/expression field; expression was in argv. 53 source fixtures had already been published (prior conversation report; no complete original stdout receipt retained). No cli-plan-r1 was published. r2 successor accepted argv fallback. Failure UTC and wall cost unavailable, not zero.'},
    {'id': 'runner-schema-revisions', 'product_processes': 0, 'utc': None,
     'reason_code': 'observation_schema_and_normative_pin_validation',
     'detail': 'executor -> v2 observation schema; v3 selected R3 contract path; v4 actual normative 31-L0 hash checks before/after. v4 checked_inputs readonly was called; no semantic run under v4 yet. All script versions remain published and count in history.'},
    {'id': 'cli-direct-show-policy-defect', 'product_processes': 0, 'utc': None,
     'reason_code': 'direct_control_kind_mismatch',
     'detail': 'Generator checked kind=control while input uses direct-control; three direct controls incorrectly target vanilla. Review requires candidate baseline outcome and baseline-specific effect, vanilla only as mutation host. Correction pending; immutable plan/oracle drafts unchanged.'},
    {'id': 'cli-empty-vs-bare-policy-defect', 'product_processes': 0, 'utc': None,
     'reason_code': 'composed_causal_obligation_not_whole_reference',
     'detail': 'where-l0-empty-vs-bare evaluates strong.where() before bare selector(strong). Authorized where success followed by preserved bare-selector error requires a composed predicate, neither whole baseline nor whole vanilla. Correction pending.'},
    {'id': 'opaque-design-review', 'product_processes': 0, 'utc': '2026-09-10T03:41:30Z',
     'evidence': pin(D / 'p1339-verifier-opacity-design-review-r1.md'),
     'reason_code': 'asymmetric_transport_opacity_admissible',
     'detail': 'Verifier permits future explicit executable-specific timeout-versus-known-error cells, zero positive credit, with finite control and repetition/reorder. Existing 32 processes are not recoded. No successor policy frozen in this paused ledger.'},
    {'id': 'shared-budget-pause', 'product_processes': 0, 'utc': None,
     'reason_code': 'shared_revision_budget_exhausted',
     'detail': 'Root instruction: contract/oracle budget 3/3 consumed; draft plans r2-r4 and opacity attempts count. Stop new executions/calibrations/freezes pending explicit canonical budget redesign. Earlier local counters semantic_oracle_revisions=0 excluded transport/drafts and do not justify extra attempts.'},
]
ledger = {
    'schema': 'p1339-oracle-publication-budget-ledger-v1',
    'created_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'author': '/root/p1319_tests', 'regime': 'executado sem atestacao de isolamento',
    'authority_manifest': pin(D / 'p1339-authority-manifest-r2.json'),
    'l0_freeze': pin(D / 'p1339-l0-freeze.json'),
    'contract': pin(D / 'p1339-contract-r3.json'),
    'scope': 'Factual ledger of this oracle author only; no semantic runner invoked, no expectation changed or frozen. Other roles must account their own prior measurements.',
    'context': 'Inherited P1319 independent A/B history. No P1339 candidate read; no productive source, local product tests or source-containing implementation receipts read.',
    'provenance_now': {'head': command(['git', 'rev-parse', 'HEAD']).strip(),
                       'status': command(['git', 'status', '--short']),
                       'diff_stat': command(['git', 'diff', 'HEAD', '--stat'])},
    'publication_inventory': publications,
    'runs': runs, 'non_run_events_and_review_findings': events,
    'totals': {'focal_batches': len(runs), 'product_processes': sum(r['processes'] for r in runs),
               'wall_seconds_sum_of_batches': sum(r['wall_seconds'] for r in runs),
               'child_process_seconds_sum': sum(r['sum_child_process_seconds'] for r in runs),
               'raw_unknown_observations': sum(r['raw_unknown_count'] for r in runs),
               'complete_preseal_matrices': 0, 'canonical_RED_runs': 0,
               'candidate_runs': 0, 'private_F_api_executions': 0,
               'known_technical_aborts_without_product_execution': 2},
    'budget_status': {'shared_revision_allowance': 3, 'shared_revisions_remaining': 0,
                      'complete_preseal_allowance': 2, 'complete_preseal_used_by_this_author': 0,
                      'new_focal_batches_authorized_by_this_ledger': 0,
                      'state': 'PAUSED_PENDING_CANONICAL_BUDGET_REDESIGN',
                      'no_free_attempts': 'Draft, transport, preparation and preflight labels do not waive revision/cost accounting. Counts above are factual attempts, not a claim that only three publications occurred.'},
    'pending_without_claim_of_coverage': ['Correct direct-show target/effect and composed empty-vs-bare predicate',
        'Asymmetric opaque cells: policy and repeat/reorder not frozen or executed',
        'F06 nominal inventory resolution and exhaustive field/variant witness map',
        'Concrete public opaque request/snapshot/body fixture',
        'F07 retention and F08 attempt/lifecycle fixtures and actual-binding templates',
        'Canonical positive/opaque oracle sets, verifier seal, actual independent RED'],
    'limitations': ['Exact original publication UTC is unavailable for files without receipt; current mtime is explicitly not an attested publication timestamp.',
                   'No mutation-score or correct-classification counts are inferred from raw exit status.',
                   'Technical-abort wall times and metadata-only script invocation costs were not retained; null/unavailable is not zero.',
                   'All prior publications remain byte-intact. This ledger does not repair their defects or grant execution authority.']
}
text = json.dumps(ledger, ensure_ascii=True, indent=2) + '\n'
patch = '*** Begin Patch\n*** Add File: ' + str(OUT) + '\n' + '\n'.join('+' + line for line in text.split('\n')[:-1]) + '\n*** End Patch\n'
subprocess.run(['apply_patch', patch], check=True, cwd=ROOT, capture_output=True, text=True)
print(json.dumps({'output': str(OUT), 'sha256': sha(OUT), 'totals': ledger['totals']}, indent=2))
