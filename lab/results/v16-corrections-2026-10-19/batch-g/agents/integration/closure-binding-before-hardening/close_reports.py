"""Write closure reports only after the final recorded executable gates pass."""
from pathlib import Path
import datetime
import gzip
import hashlib
import json
import subprocess

P = Path(__file__).resolve().parent
R = Path.cwd()
load = lambda name: json.loads((P / name).read_text())
sha = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()
utc = datetime.datetime.now(datetime.timezone.utc).isoformat()
scope = load('scope.json')
commands = load('validation-final-3/validation-commands.json')
freeze = load('validation-final-3/validation-freeze.json')
assert load('gates-3-launch.json')['exit'] == 0
assert load('acceptance-3-launch.json')['exit'] == 0
acceptance_worker_state = load('acceptance-worker-state-3.json')
assert all(sha(R / name) == digest for name, digest in acceptance_worker_state['files'].items())
assert len(commands) == 12 and all(c['exit'] == 0 for c in commands)
assert freeze['changed_inputs'] == [] and freeze['completed_jobs'] == 12
build = load('candidate-build-3/receipt.json')
assert build['exit'] == 0 and build['changed_inputs'] == []
assert all(sha(R / n) == d for n, d in build['inputs'].items())
assert sha(R / 'lab/target/product/debug/typst') == build['product_sha256']
focused = [load(f'focused-green-3-{pkg}/receipt.json') for pkg in ('typst-core', 'typst-infra')]
assert [len(r['tests']) for r in focused] == [20, 36]
assert all(r['exit'] == 0 and not r['failed'] and not r['changed_inputs'] for r in focused)
assert all(not r['compiler_error_codes'] and all(t['status'] == 'ok' for t in r['tests']) for r in focused)
with gzip.open(P / 'validation-final-3/validation-state.json.gz', 'rt') as f:
    suite_state = json.load(f)
assert suite_state['inputs'] == build['inputs']
assert all(r['inputs'] == build['inputs'] for r in focused)
delta = load('lint-delta-3.json')
assert delta['outside_scope_identical'] and delta['candidate_counts'] == {'V19:note': 1339, 'V20:note': 905}
assert delta['outside_scope_objects'] == 2174
structural = load('structural-green-3/receipt.json')
assert structural['selected_core'] == 5 and structural['selected_infra'] == 13
assert len(structural['targets']) == 18 and all(len(t['witnesses']) == 1 for t in structural['targets'])
assert not structural['actual_inputs_changed_during_probe']
axes = load('tuple-axes-green-3/receipt.json')
assert len(axes['commands']) == 6
accept = load('acceptance-3/commands.json')
assert len(accept) == 6 and all(c['exit'] == c['expected_exit'] for c in accept)
eq = load('equality-candidate-3/receipt.json')
assert not eq['required_failures'] and not eq['changed_inputs'] and eq['binary_unchanged']
expected_eq_rows = [(group, case['name']) for group in ('equality-probes', 'context-intent-probes')
                    for case in load(f'agents/core/{group}/receipt.json')['cases']]
assert len(expected_eq_rows) == 34 and [(c['group'], c['name']) for c in eq['cases']] == expected_eq_rows
assert sum(c['required'] for c in eq['cases']) == 33
assert [c['name'] for c in eq['cases'] if not c['required']] == ['html-tristate-fields']
html = load('html-candidate-3/receipt.json')
html_policy = load('agents/html/cli-policies.json')
assert [c['name'] for c in html['cases']] == [c['name'] for c in html_policy['cases']]
assert len(html['cases']) == 16 and sum(c['required'] for c in html['cases']) == 13
assert not html['required_failures']
hash_before = load('hash-before/receipt.json')
hash_after = load('hash-candidate-3/receipt.json')
assert len(hash_before['rows']) == len(hash_after['rows']) == 366
assert hash_before['source_sha256'] == hash_after['source_sha256']
assert not any(hash_after['violations'].values()) and not hash_after['changed_inputs'] and hash_after['artifact_unchanged']
assert hash_after['candidate_acceptance']['pass']
assert hash_after['compile']['exit'] == hash_after['run']['exit'] == 0
assert hash_before['compile']['exit'] == hash_before['run']['exit'] == 0
assert len(hash_before['violations']['public_hash']) == 248
assert len(hash_before['violations']['std_hash']) == 168
api = load('agents/core/api-scope-review-final13-3.json')
assert api['pair_count'] == 13 and api['source_owner_file_count'] == 26
assert not api['public_signature_entity_trait_enum_default_phase_change']
assert not api['new_allow_expect_ignore_or_config_change']
assert not api['outside_productive_or_normative_changes']
assert api['all_frozen_inputs_match_named_seal'] == 2142 and api['unique_effective_nucleus_count'] == 25
assert api['scope_sha256'] == sha(P / 'scope.json')
assert api['seal_sha256'] == sha(P / 'reseal-candidate-3/receipt.json.gz')
assert all(sha(R / p['source']) == p['source_sha256'] and sha(R / p['owner']) == p['owner_sha256'] for p in api['pairs'])
sr = load('agents/contexts/final13-structural-review-3.json')
assert (sr['selected_total'], sr['selected_core_primary_witnesses'], sr['selected_infra_primary_witnesses']) == (18, 5, 13)
assert not sr['current_source_drift_from_reviewed_candidate_snapshot']
supplemental = sr['supplemental_identity_witnesses']
assert len(supplemental) == 2 and all(w['original_inventory_id'] is None and len(w['raw_primary_e0004']) == 1 for w in supplemental)
assert {(w['target']['after'], w['target']['anchor']) for w in supplemental} == {
    ('impl fmt::Debug for Content', 'match content {'), ('fn fmt_content(', 'match c {')}
tr = sr['tuple_results']
assert tr == {'baseline_original_absorbs_each_direction': 6, 'candidate_each_direction_primary_e0004': 6,
              'all_logs_only_e0004_errors': True, 'exact_mask_only_target_mutations': True}
lr = load('agents/contexts/final13-lint-review-3.json')
assert lr['scope_pairs'] == 13 and lr['outside_scope_objects'] == 2174
assert not lr['current_input_drift'] and lr['no_added_errors_or_warnings']
for review in (sr, lr):
    assert all(sha(P / name) == digest for name, digest in review['evidence_sha256'].items())
assert (P / 'open-obligations.json').is_file()
assert subprocess.check_output(['git', 'branch', '--show-current'], text=True).strip() == scope['branch']
head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
assert head == scope['baseline_head']
subprocess.run(['git', 'merge-base', '--is-ancestor', head, 'HEAD'], check=True)

evidence_names = [
    'scope.json', 'cases.json', 'baseline-inputs.json.gz',
    'reseal-candidate-3/receipt.json.gz', 'lint-final-3/lint-state.json.gz',
    'lint-delta-3.json', 'structural-green-3/receipt.json',
    'tuple-axes-green-3/receipt.json', 'validation-final-3/validation-state.json.gz',
    'validation-final-3/validation-commands.json', 'validation-final-3/validation-freeze.json',
    'candidate-build-3/receipt.json', 'hash-before/receipt.json',
    'hash-candidate-3/receipt.json', 'equality-candidate-3/receipt.json',
    'html-candidate-3/receipt.json', 'figure-policy-replay-3/receipt.json',
    'reference-corpora-3/receipt.json', 'figure-target-controls-3/receipt.json',
    'acceptance-3/commands.json', 'agents/core/api-scope-review-final13-3.json',
    'gates-3-launch.json', 'acceptance-3-launch.json',
    'acceptance-worker-state-3.json',
    'agents/core/hash-final-review-3.json', 'agents/html/final-acceptance-review-3.json',
    'agents/contexts/final13-structural-review-3.json', 'agents/contexts/final13-lint-review-3.json',
    'agents/html/final-verifier-hardening.json',
    'agents/html/header-debt-verifier-hardening.json',
    'open-obligations.json',
]
for name in evidence_names:
    assert (P / name).is_file(), name
sources = {pair['source']: pair for pair in api['pairs']}
ledger = {
    'utc': utc, 'state': 'uncommitted working tree', 'head': head,
    'branch': scope['branch'], 'diff_head_stat': subprocess.check_output(['git', 'diff', 'HEAD', '--stat'], text=True),
    'status': 'closed within the recorded G scope; no complete parity claim',
    'source_owner_pairs': api['pairs'],
    'evidence': {name: sha(P / name) for name in evidence_names},
    'findings': [
        {'id': t['id'], 'source': t['file'], 'owner': sources[t['file']]['owner'],
         'current_match_line': t['line'], 'status': 'closed',
         'source_sha256': sources[t['file']]['source_sha256'],
         'owner_sha256': sources[t['file']]['owner_sha256'],
         'primary_e0004_count': len(t['witnesses']),
         'acceptance': ['focused-green-3-typst-core/receipt.json', 'focused-green-3-typst-infra/receipt.json',
                        'structural-green-3/receipt.json', 'lint-delta-3.json',
                        'validation-final-3/validation-commands.json']}
        for t in structural['targets']
    ],
    'additional_causes': scope['extensions'] + [scope['depth_scope_extension']],
    'supplemental_identity_witnesses': supplemental,
    'remaining_obligations': 'open-obligations.json',
}
assert {x['id'] for x in ledger['findings']} == set(scope['ids'])
(P / 'closure-ledger.json').write_text(json.dumps(ledger, indent=2) + '\n')

report = f'''# Batch G — closed decisions and preserved causal identity

The eighteen remaining V16 findings are repaired on `{scope['branch']}`.
The complete pinned linter reports **0 errors and 0 warnings**, with **1,339 V19
and 905 V20 informational notes still enabled**. All 2,174 diagnostic objects
outside the thirteen source/owner pairs are identical to the baseline, including
messages, multiplicities and locations. This closes the recorded V16 inventory,
not complete Typst language parity. No linter, configuration, severity or
suppression was changed.

## Provenance and scope

Measurements used HEAD `{head}` plus the exact uncommitted changes recorded
in each receipt. The final [build receipt](candidate-build-3/receipt.json),
[full-suite state](validation-final-3/validation-state.json.gz) and
[API/lineage audit](agents/core/api-scope-review-final13-3.md) bind all 2,142
inputs, the changed-file list, UTC and commands. The final product SHA-256 is
`{build['product_sha256']}`. Measurements precede the closure commit; source
hashes, rather than a version string, identify the tested implementation.

The oracle remains commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Its [ratified build receipt](../..//construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md)
is the language reference. The linter remains commit
`12797073972e375d9ab078cf6750c8b30b71e2b9`, SHA-256
`421d0797e7ae1a5102e9c5bcd87b488fea70a1997394c38765a03239fe579121`.

[The immutable baseline](baseline-inputs.json.gz), [original eighteen entries](cases.json),
[final scope](scope.json) and [closure ledger](closure-ledger.json) retain exact
ownership and evidence. V16-100 was already removed in F; it is not counted again.
Thirteen productive files have thirteen canonical L0 owners. Their twenty-five
effective Nuclei and all forward/reverse seals were checked. No public signature,
entity field, trait, default, compatibility boundary or pipeline phase changed.
The two scope extensions repair measured internal causes under ADR-0127's
continuous flow, with L0 first and actual RED→GREEN evidence.

## Causes repaired

| Findings | Decision and implementation |
|---|---|
| 049–050 | A failed bare-math absence proof triggers full traversal; it never proves absence. A mapper's `None` descends rather than discarding children. Preserve equation/location boundaries and selected Table visuals. |
| 051–052 | Explicit normalization and observation catalogues retain protected products, original/visual identity reservations, recipe floors and canonical sequence markers. |
| 074 | Structural Title/HTML/frame comparisons, ordered HTML attributes and Context-owned mechanical IDs are explicit. Both tuple axes demand a decision for future kinds. Language-only Context opacity is a private comparison projection; callbacks and original IDs remain untouched. |
| 088–089, 092, 095 | Reclassify suppressed visual/callback roots locally; retain strict unresolved/textual errors. Project actual Table row-cell versus standalone visuals correctly. Preserve frame islands and once-only frame/style/ID collection. |
| 093–094, 099 | Both independent contextual collectors recover Source-wrapped producers below Located roots without broadening the global mapper. Duplicate IDs retain first encounter order and last payload/style chain. |
| 096–098 | Font, context and retained-prefix checks use explicit conservative proof boundaries; the prefix proof preserves Arc identity and both future tuple axes. |
| 101–103 | Explicit ElementKind/list/enum complements preserve documented approximate query behavior; this does not certify compiled-query parity. |

An additional real-root public probe exposed a pre-existing Eq/Hash defect:
invisible carriers consumed Debug's visible depth budget. Across the same 366
fixed comparisons, public-hash violations changed from **248 to 0** and standard
Hash violations from **168 to 0**. The fix borrows and iterates four invisible
carriers before accounting for visible depth. It retains the visible depth limit,
nominal formatting, algorithms, seeds and causal/observation identity. Exact
[before](hash-before/receipt.json) and [after](hash-candidate-3/receipt.json)
receipts bind a real-root compiled library and unchanged probe source; this is
bounded evidence, not a universal Hash proof. The suspected nested Value Float
counterexample was **refuted** by actual execution and is not an observed debt.

The raw Table renderer's existing footer omission was also measured. Retained
footer data is still converted, but an unrendered branch cannot create a phantom
deep-heading diagnostic. A private emitted-state flag remains monotonic through
all twenty-one recursive edges, independently of cell selection. Positive
standalone and actual public resolved-footer controls still emit the exact
warning once. This neither suppresses emitted diagnostics nor certifies raw
footer language support; the omission remains open.

## Executable acceptance

- [Focused GREEN](integration-3/commands.json): **20 core + 36 infra tests**,
  no ignored tests, compiler errors or input drift. Valid semantic RED runs and
  all unsuccessful harnesses remain separate under `focused-*`.
- [Complete final run](validation-final-3/validation-commands.json): **12/12 gates**,
  including canonical paths, strict forward/reverse lineage, ownership/Nuclei,
  workspace build, domain/infra/wiring tests, doctests, all targets, strict Clippy,
  formatting and warnings-as-errors Rustdoc. The [freeze](validation-final-3/validation-freeze.json)
  records no input changes.
- [Actual enum evolution](structural-green-3/receipt.json): all **18 original
  selected matches** produce primary E0004 diagnostics. The immutable baseline
  absorbed each. [Six isolated tuple-axis checks](tuple-axes-green-3/receipt.json)
  separately demand decisions for new/old, old/new and new/new on both matches.
  Only copied core dependencies received compiler-suggested bridges, after raw
  witness recording; no copied infra target was repaired or executed. Two
  additional identity matches are reviewed separately, without inventing V16 IDs.
- [Equality CLI](equality-candidate-3/receipt.json) and [HTML CLI](html-candidate-3/receipt.json)
  replay fixed source-backed policies against the exact fresh product. Existing
  representation, unsupported Link and diagnostic differences remain explicitly
  open. DOM comparison retains DOCTYPE and checks successful diagnostic errors;
  its heading-warning allowance is confined to the previously declared debt.
- [Figure policies](figure-policy-replay-3/receipt.json) retain **84 accepted policy
  cases: 77 required, 5 debts and 2 negative controls**. [Contextual references](reference-corpora-3/receipt.json)
  retain **40 required cases across six matrices**. [Target controls](figure-target-controls-3/receipt.json)
  pass **10/11**: the existing SetPage header failure remains a strict failure.
  These counts are policy acceptance, not complete oracle equality.
- [Full diagnostic delta](lint-delta-3.json): eighteen V16 removals; twenty-five
  V19 and four V20 relocations retain identical payloads. Twenty-four additional
  V19 reminders remain visible. Grouped alternatives are not claimed individually
  covered at runtime.

The first integrated run exposed the public Table cell-selection defect; its
four actual CLI probes and failure are preserved. The first full-suite run
failed strict Clippy on five identity conversions in new fixtures; only those
conversions were removed, without changing assertions or adding allowances.
Earlier selector/private-Location/private-export harness errors and the artifact
path-guard failure did not execute semantic tests and are not labeled RED.

The next complete run passed all twelve gates and the actual hash/equality
controls, but the frozen HTML replay still rejected one required case:
`bundle-cell-heading` emitted `<p><h2>H</h2></p>` instead of a block heading.
That [failed replay](html-candidate/receipt.json) and the preceding successful
[twelve-gate snapshot](validation-final-2/validation-commands.json) remain
historical. Four additional controls reproduced two semantic failures while two
preservation controls passed. The document owner now selects the same borrowed
standalone visual/body as the existing inline renderer when deciding block flow,
without unwrapping cells globally or altering Table row selection. The unchanged
HTML policy is replayed against the newly rebuilt product, and all gates are
repeated for that exact final source state.

Independent verifier review also exposed a JSON-parser false-positive path.
The original Lab scripts and their failing negative controls were preserved;
candidate output parsing and complete prior-policy cardinality checks were
hardened before the final replay. No case, oracle value or acceptance policy was
changed to accommodate a candidate result.

## Remaining work and history

[Open obligations](open-obligations.json) retain the bounded G gaps and hash-pinned
inherited ledgers. Zero V16 does not close unsupported HTML Link, approximate
queries, diagnostic/source-origin differences, raw-footer support, existing
Metadata/State mechanical comparisons, full Figure/Grid/physical-page parity or
earlier math callback decisions. No new runtime or performance improvement was
measured. Source/Located orientation controls establish a valid domain case,
not that the frontend currently generates every tested orientation.

The organization helped trace each repair to its exclusive owner and separate
current L0 contracts from historical measurements. All build/probe artifacts and
unsuccessful attempts stay in `lab/`; the same branch and ancestry are preserved.
The [previous diagnostic](diagnostic-history/v16-correction-progress-2026-10-19.md)
and [its provenance](diagnostic-history/provenance.json) retain the A–F checkpoint.
'''
report = report.replace('../..//construction/', '../../construction/')
(P / 'README.md').write_text(report)

diagnostic = R / '00_nucleo/diagnostics/v16-correction-progress-2026-10-19.md'
old = (P / 'diagnostic-history/v16-correction-progress-2026-10-19.md').read_text()
assert diagnostic.read_text() == old
old_status = '''Status: batches A–F are complete within their recorded acceptance scopes.
The full pinned linter reports **18 remaining V16 errors**, assigned to the
remaining batch G work. This is neither a zero-violation result nor complete
Typst language parity. No alerts, severities or contracts were weakened.'''
new_status = '''Status: batches A–G are complete within their recorded acceptance scopes.
The full pinned linter reports **0 errors and 0 warnings**, with V19=1,339 and
V20=905 informational notes still visible. All 105 original V16 findings are
closed. This does not establish complete Typst language parity. No alerts,
severities or contracts were weakened.'''
assert old_status in old
updated = old.replace(old_status, new_status)
marker = '\nThe current Figure measurement starts at HEAD'
assert marker in updated
g_entry = '| G: explicit decisions and causal/visual projections | 18 → 0 | [Final report](../../lab/results/v16-corrections-2026-10-19/batch-g/README.md), [closure ledger](../../lab/results/v16-corrections-2026-10-19/batch-g/closure-ledger.json), [open obligations](../../lab/results/v16-corrections-2026-10-19/batch-g/open-obligations.json) |\n'
updated = updated.replace(marker, g_entry + '''
G's final measurement uses HEAD `4382891f0d5f0839669597cb5c90a98f24cfb8bd`
plus the exact dirty-state sources/L0 recorded in its receipts. All 2,174
outside-scope diagnostic objects are unchanged. The thirteen exclusive owners,
twenty-five effective Nuclei and 2,142 frozen inputs passed forward/reverse
lineage and canonical path checks. All 56 focused tests and twelve complete
integrated gates pass. Actual enum evolution demands all eighteen original
decisions and six isolated tuple-axis decisions; additional identity witnesses
are recorded separately. A real-root 366-comparison probe confirms the repaired
invisible-carrier depth defect no longer violates either measured Eq/Hash law.
No complete Hash, language or performance claim follows from these controls.

The fresh G product also replays the previous Figure policies: 77 required,
five debts and two negative controls retain their distinct meanings; all forty
required reference cases across six matrices pass. Target controls remain 10/11,
with SetPage headers still failing. Current G CLI policies and their existing
HTML, diagnostic and representation gaps remain separate from those inherited
limits. Exact binary/source identities, UTC, failed attempts and independent
review are in the G report.

The following Figure paragraphs retain the preceding F checkpoint and its
scope; their counts and source state are historical, not the current G lint.
''' + marker.replace('current Figure', 'Figure checkpoint'))
updated = updated.replace('The separate V16-099 collector is still open.', 'The separate V16-099 collector remained open at F and is now repaired in G.')
updated += '''
The [G gap ledger](../../lab/results/v16-corrections-2026-10-19/batch-g/open-obligations.json)
retains all inherited ledgers by hash and records query approximation, unsupported
HTML/diagnostic boundaries, raw-footer behavior and unmeasured performance.
The [complete A–F diagnostic](../../lab/results/v16-corrections-2026-10-19/batch-g/diagnostic-history/v16-correction-progress-2026-10-19.md)
preserves the state before G; the original inventory remains historical.
'''
diagnostic.write_text(updated)
print('Wrote scoped closure ledger, report and current diagnostic after all final executable assertions.')
