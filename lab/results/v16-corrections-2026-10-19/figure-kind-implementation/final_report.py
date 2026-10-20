"""Write bounded closure only from the successful, source-bound final receipts."""
from pathlib import Path
import gzip
import importlib.util
import json
import subprocess

ROOT=Path.cwd()
P=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('validation',ROOT/'lab/tools/ci/validate_layout_migration.py')
v=importlib.util.module_from_spec(spec);spec.loader.exec_module(v)
read=lambda n: json.loads((P/n).read_text())
inputs=v.frozen_inputs()
commands=read('validation-final-5/validation-commands.json')
freeze=read('validation-final-5/validation-freeze.json')
assert len(commands)==12 and all(r['exit']==0 for r in commands) and not freeze['changed_inputs']
assert not read('final-gates-6/freeze.json')['changed_inputs']
driver=read('final-gates-6/commands.json')
assert len(driver)==6 and all(r['exit']==0 for r in driver)
state=json.loads(gzip.decompress((P/'validation-final-5/validation-state.json.gz').read_bytes()))
build=read('candidate-build-4/receipt.json')
assert state['inputs']==inputs==build['inputs'] and build['exit']==0 and not build['changed_inputs']
assert build['product_sha256']==v.digest(ROOT/'lab/target/product/debug/typst')
lint=json.loads(gzip.decompress((P/'lint-final-12/lint-state.json.gz').read_bytes()))
assert lint['inputs']==inputs and not lint['changed_inputs']
assert lint['counts']['V16:error']==18
structural=read('structural-candidate-final5/receipt.json')
assert structural['selected_e0004']==8 and structural['exit']==101 and not structural['other_errors']
replay=read('candidate-replay-4/receipt.json')
assert replay['passed'] and not replay['changed_inputs'] and len(replay['cases'])==84
assert replay['binaries']['product']['sha256']==build['product_sha256']
required=[c for c in replay['cases'] if c['disposition']=='required-closure']
assert len(required)==77 and all(c['passed'] for c in required)
refs=read('reference-corpora-final/receipt.json')
assert refs['inputs']==inputs and not refs['changed_inputs'] and refs['binary_unchanged'] and len(refs['commands'])==6
assert all(c['exit']==0 and not c['required_failures'] for c in refs['commands'])
required_refs=[]
for matrix in ('main','additional','targeted','origin','origin-edge','show-origin-selected'):
    corpus=read('reference-corpora-final/'+matrix+'/receipt.json')
    assert corpus['product_sha256']==build['product_sha256']
    assert corpus['build_receipt_sha256']==v.digest(P/'candidate-build-4/receipt.json')
    assert not corpus['required_failures']
    required_refs.extend(c for c in corpus['cases'] if c['policy']['class'].startswith('required-'))
assert len(required_refs)==40 and all(c['pass'] for c in required_refs)
controls=read('candidate-target-controls-4/receipt.json')
assert [c['name'] for c in controls['cases'] if not c['passed']]==['header-figure-boundary']
assert controls['product']['sha256']==build['product_sha256'] and not controls['changed_inputs']
native=read('native-page-running-final/receipt.json')
assert native['passed'] and native['product_sha256']==build['product_sha256']
assert native['build_receipt_sha256']==v.digest(P/'candidate-build-4/receipt.json')
ownership=read('ownership-candidate-4/receipt.json')
assert ownership['product_sha256']==build['product_sha256']
expected_ownership={'ancestor-hidden':[0,[0],[0]],'own-hidden':[1,[1],[1]],'duplicate-kind-first':['table',[1],[1]],'duplicate-numbering-first':['table',[0],[1]]}
assert len(ownership['cases'])==4 and {c['name'] for c in ownership['cases']}==set(expected_ownership)
for case in ownership['cases']:
    assert case['returncode']==0 and json.loads(case['stdout'])==expected_ownership[case['name']]
    assert case['source_sha256']==v.digest(P/'ownership-candidate-4'/(case['name']+'.typ'))
old_ownership=read('ownership-candidate-3/receipt.json')
assert {(c['name'],c['source_sha256']) for c in ownership['cases']}=={(c['name'],c['source_sha256']) for c in old_ownership['cases']}
extra_names=('final-captionless-controls','final-reference-extra-controls','final-ancillary-controls','final-math-style-controls','final-math-capability-controls','final-counter-owned-controls')
extras={name:read('agents/counters/'+name+'/receipt.json') for name in extra_names}
for extra in extras.values():
    assert extra['build_input_hashes']==inputs and extra['binary_sha256']==build['product_sha256']
    assert extra['build_receipt_sha256']==v.digest(P/'candidate-build-4/receipt.json')
    assert not extra['changed_build_inputs_after'] and not extra['changed_corpus_after'] and extra['binary_unchanged']
assert extras['final-captionless-controls']['passed'] and len(extras['final-captionless-controls']['cases'])==10
assert extras['final-reference-extra-controls']['passed'] and len(extras['final-reference-extra-controls']['cases'])==3
assert [c['name'] for c in extras['final-ancillary-controls']['cases'] if not c['passed']]==['ordinary-block-labelled-identity-labelled-block']
assert extras['final-math-style-controls']['required_passed']
assert extras['final-counter-owned-controls']['required_passed']
assert extras['final-math-capability-controls']['observation_complete']
broad=read('agents/entities/reference-acceptance/show-origin-product-final/receipt.json')
assert broad['product_sha256']==build['product_sha256'] and broad['build_receipt_sha256']==v.digest(P/'candidate-build-4/receipt.json')
assert len(broad['cases'])==4 and all(c['checks']['terminates_with_error'] and c['checks']['error_origin'] for c in broad['cases'])
assert [c['name'] for c in broad['cases'] if not c['pass']]==['show-origin-new-native-multiline','show-origin-other-alias-multiline']
scope=read('scope.json')
assert len(scope['pairs'])==64
pairs=[dict(p,source_sha256=v.digest(ROOT/p['source']),owner_sha256=v.digest(ROOT/p['owner'])) for p in scope['pairs']]
authorization=read('agents/entities/authorization-final64-review-reseal15.json')
assert authorization['scope']['count']==64
assert authorization['scope']['scope_sha256']==v.digest(P/'scope.json')
assert authorization['sealed_snapshot']['current_equals_receipt_after_for_all128']
assert not authorization['sealed_snapshot']['outside_changes']
assert {(p['source'],p['owner'],p['source_sha256'],p['owner_sha256']) for p in pairs} == {
    (p['source'],p['owner'],p['source_sha256'],p['owner_sha256'])
    for p in authorization['sealed_snapshot']['pairs']}
ledger={'utc':v.utc(),'state':'validated uncommitted working tree','baseline_head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'branch':subprocess.check_output(['git','branch','--show-current'],text=True).strip(),'diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'closed_selected_ids':['V16-055'],'incidental_lint_removal':['V16-100'],'batch_f':'bounded approved scope complete; full Figure/Grid parity is not certified','remaining_v16':18,'source_owner_pairs':pairs,'frozen_input_count':len(inputs),'product_sha256':build['product_sha256'],'oracle_commit':'bd5ebec285505889e8ff1d4f68804ab147a699f3','oracle_tree':'cac836c07e4dec2a89219370428292393a3216b5','oracle_sha256':'9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9','receipts':{n:v.digest(P/n) for n in ['approval.json','reference-phase-approval.json','scope.json','validation-final-5/validation-state.json.gz','validation-final-5/validation-commands.json','validation-final-5/validation-freeze.json','lint-final-12/lint-state.json.gz','agents/counters/lint-reference-final-12-review.json','structural-candidate-final5/receipt.json','agents/realize/structural-final5-review.json','candidate-build-4/receipt.json','candidate-replay-4/receipt.json','reference-corpora-final/receipt.json','candidate-target-controls-4/receipt.json','ownership-candidate-4/receipt.json','native-page-running-final/receipt.json','agents/realize/full-suite-phase-fixture-review.json','agents/realize/full-suite-phase-fixture-after-review.json','agents/counters/reference-legacy-fixture-green-receipt.json','agents/entities/authorization-final64-review-reseal15.json','agents/counters/math-callback-context-language-review.json','agents/realize/math-cancel-context-boundary/final-known-style-correction.json','agents/entities/counter-final-style-only-boundary-review.json','agents/realize/counter-final-capability-review.json','agents/counters/counter-final-owned-product-1/receipt.json','agents/counters/counter-final-capability-product-1/receipt.json','agents/counters/final-extra-controls-review.json','agents/entities/scope-closure-obligations-final.json','agents/entities/reference-acceptance/show-origin-product-final/receipt.json','agents/entities/reference-acceptance/show-origin-product-final-post-freeze.json'] + ['agents/counters/'+name+'/receipt.json' for name in extra_names]}}
(P/'closure-ledger.json').write_text(json.dumps(ledger,indent=2)+'\n')
gaps=[
 {'id':'set-page-header','status':'observed failing','evidence':'candidate-target-controls-4/receipt.json','scope':'SetPage operational marker; native PageRun independently passes, no repeated-physical-page certification'},
 {'id':'block-own-label','status':'observed failing','evidence':'agents/counters/final-ancillary-controls/receipt.json','historical_evidence':'agents/counters/ancillary-product-9/receipt.json','scope':'actual child traversal repaired; own Block label projection remains absent'},
 {'id':'unrestricted-ref-show-termination','status':'observed failing','evidence':'agents/entities/reference-acceptance/show-origin-product-final/receipt.json','historical_evidence':'agents/entities/reference-acceptance/show-origin-product-9/receipt.json','scope':'All four shows terminate with an error and matching source origins. Fresh/distinct unrestricted Ref shows retain two error-message mismatches against the oracle termination diagnostic.'},
 {'id':'ambiguous-label-and-bibliography','status':'observed failing','evidence':'reference-corpora-final/main/receipt.json','scope':'existing first-result/bibliography precedence differs from oracle ambiguity diagnostics'},
 {'id':'single-context-locale-fixture','status':'refuted control preserved','evidence':'reference-corpora-final/main/receipt.json','scope':'same raw Context plus intervening language sets did not establish distinct lexical insertions; separate actual insertion-site controls pass'},
 {'id':'math-cancel-context-contract','status':'existing contract conflict requires separate decision','evidence':'agents/counters/math-callback-context-language-review.json','scope':'Pinned plain outer callback has styles/no Location. Tekt keeps rendered script14pt and delayed Context result extension; pinned oracle observes lexical20pt and rejects returned Content. No new divergence ratified; old tests unchanged.'},
 {'id':'math-context-diagnostic-origin','status':'observed diagnostic mismatch','evidence':'agents/counters/final-extra-controls-review.json','scope':'Modern counter.final now rejects styles-only/no-Location contexts instead of returning a false zero. All six compared error sources retain some diagnostic difference: ordinary outside-context text.size lacks the second oracle hint; here/get differ in messages/hints/trace; five callback cases anchor at cancel instead of the inner call. Modern final message and both hints match, but full diagnostic parity is not certified.'},
 {'id':'declared-figure-grid-boundaries','status':'open','scope':'supplement-dependent preview; full caption/default alignment/floats and caption position/separator/locale/figure_location; complete function-number synthesis; HTML implicit rendering; external Dynamic fields; Grid footer validation; StateUpdate key reflection; mixed Table header/ordinary-cell supplemental order; Block style/breakability and complete physical page-running timing'}]
(P/'open-obligations.json').write_text(json.dumps({'utc':v.utc(),'no_complete_parity_claim':True,'gaps':gaps},indent=2)+'\n')
(P/'README.md').write_text(f'''# Figure/Grid and contextual reference supplements — bounded closure

The approved V16-055 causal repair is complete on `codex/tekt-mature-layout`. Batch F closes within its approved scope; complete Figure/Grid or Typst parity is not certified. The full pinned linter still exits 1 for **18 V16 errors**. No rule, severity, scan scope, suppression or ignored-test policy was changed.

## Authority and measured state

The original [Figure/Grid approval](approval.json) and [additional Ref approval](reference-phase-approval.json) bind the unchanged frozen proposals. Implementation follows L0 first and semantic RED→GREEN. The exact pre-consolidation Figure L0 remains in [prompt history](prompt-history/figure-consolidation-before.json); current L0 is an English normative specification, without pending-decision or build-step instructions.

Baseline HEAD is `{ledger['baseline_head']}`. Decisive measurements use an uncommitted working tree, with exact source/L0 hashes, changed-file lists, UTC and argv in each receipt. [The closure ledger](closure-ledger.json) binds all 64 exclusive owner/consumer pairs and {len(inputs)} frozen inputs. Oracle identity is commit `{ledger['oracle_commit']}`, tree `{ledger['oracle_tree']}`, binary SHA-256 `{ledger['oracle_sha256']}`; [the ratified build receipt](../../construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md) retains the recipe. Final product SHA-256 is `{build['product_sha256']}`. The historical baseline binary was input-equivalent to baseline HEAD, not rebuilt there; [baseline evidence](baseline.json) preserves this qualification.

## Changes and acceptance

Typed Figure kind and lexical/resolved supplements retain nominal function identity; Grid retains semantic child order. Reached occurrence preparation respects lexical/show-set styles and hidden parents. Location-based typed automatic actions and complete demanded histories preserve manual updates. Actual caption and Ref supplement output uses fresh raw occurrence frames, retained retry slots and preexisting aliases. Only actual selected output contributes child effects. Three exact approved public transport functions bridge contextual references; original Ref Source spans remain attached through constructors, shows and layout errors. Paged captionless Figure realizes an ordinary Block through existing join/native Block and recipe semantics, preserving canonical Figure identity independently of its actual product.

| Final gate | Result | Evidence |
|---|---|---|
| Complete integrated gates | 12/12 pass; frozen inputs unchanged | [commands](validation-final-5/validation-commands.json), [freeze](validation-final-5/validation-freeze.json) |
| Paths and strict forward/reverse lineage, V15/V26 | Pass | [validation directory](validation-final-5/) |
| Real enum-extension probes | All 8 selected E0004 witnesses; no other error codes | [actual compiler receipt](structural-candidate-final5/receipt.json), [independent raw-span review](agents/realize/structural-final5-review.md) |
| Frozen original and counter policies | 84/84 policy outcomes; 77 required cases pass | [replay](candidate-replay-4/receipt.json) |
| Contextual reference corpora | All 40 required cases across six matrices pass | [source/binary-bound corpus receipt](reference-corpora-final/receipt.json) |
| Extra rendered target controls | 10/11; SetPage failure retained | [raw receipt](candidate-target-controls-4/receipt.json) |
| Captionless morphology and hidden-child controls | 10/10 pass | [final receipt](agents/counters/final-captionless-controls/receipt.json) |
| Extra non-Figure, bibliography and Grid controls | 3/3 pass | [final receipt](agents/counters/final-reference-extra-controls/receipt.json) |
| Ancillary Block and source-origin controls | 4/5; Block own-label failure retained | [final receipt](agents/counters/final-ancillary-controls/receipt.json) |
| Broad unrestricted Ref shows | 2/4 strict diagnostic comparisons; all four terminate with matching origins | [final receipt](agents/entities/reference-acceptance/show-origin-product-final/receipt.json) |
| Known-style and modern counter capability controls | Required checks pass; script-style and all six measured error-payload differences retained | [complete final review](agents/counters/final-extra-controls-review.md) |
| Frozen ownership policies | 4/4 exact recorded tuples; duplicate-label oracle gaps remain separate | [ownership receipt](ownership-candidate-4/receipt.json) |
| Native PageRun source | Pass | [separate source receipt](native-page-running-final/receipt.json) |
| Full differential lint | 20 → 18 V16; V19={lint['counts']['V19:note']}, V20={lint['counts']['V20:note']}; outside-scope objects unchanged (complete inventory in the independent review) | [lint state](lint-final-12/lint-state.json.gz), [complete note review](agents/counters/lint-reference-final-12-review.md) |

The original 78 sources and expectations are unchanged: 71 required cases, five declared debts and two negative controls. Six added counter cases bring the required total to 77. Thus 84 policy outcomes do not mean 84 complete vanilla matches. V16-055 disappears; V16-100 also disappears through the associated substitution repair, while the separate V16-099 collector remains open. V19/V20 reminders remain enabled; their full message/multiplicity/location changes are reviewed.

## Preserved limits and unsuccessful work

[Open obligations](open-obligations.json) retain SetPage configured headers, Block own-label projection, unrestricted fresh/distinct Ref show termination diagnostics, duplicate-label/bibliography ambiguity and all declared caption/style/HTML/Grid/preview boundaries. No failed strict comparison is reclassified as passing. The separate two-site language control replaces a refuted inference, while the original single-Context source remains preserved. The [independent scope review](agents/entities/scope-closure-obligations-final.md) and [final extra-controls review](agents/counters/final-extra-controls-review.md) bind these boundaries to the final product. Captionless and ancillary final binary controls do not erase their own failures. In the broad show controls, all four executions terminate with errors and matching source origins; two report missing labels where the oracle reports maximum show-rule depth, so those strict comparisons remain failed.

The first full final workspace attempt failed three old phase fixtures: one eval raw-Ref representation check and two Heading references sent directly to pure layout. [Independent phase classification](agents/realize/full-suite-phase-fixture-review.md) and [byte-precise fixture review](agents/realize/full-suite-phase-fixture-after-review.md) retain those failures. The migrated eval test proves the original Ref and exact producer identity/source; both Heading source strings and Section 1/Cap 1 predicates are unchanged and now execute the existing retained contextual phase before layout. No production behavior or generic test helper changed for this migration. A subsequent full wiring run exposed three actual math-cancel compatibility regressions after the stricter field guard. The runtime adapter now marks its helper-local context known, with existing captured styles and no Location; field guards and snapshot algorithms remain unchanged. Pinned valid source confirms outer callback style access. A further valid-source capability probe found that modern counter.final silently used an empty introspector without a Location. The private Engine-language adapter now rejects that state before delegation/folding, preserving filtered-demand marking and both measured context hints. The public no-Engine materialized helper and legacy string routes remain unchanged; this is an additional internal cause repair, not a new API or phase. Existing Tekt script14pt versus pinned lexical20pt and returned-Context handling remain explicit unresolved contract boundaries in [the measured review](agents/counters/math-callback-context-language-review.json). The three old assertions remain unchanged; invalid oracle spellings and refuted hypotheses are preserved. Earlier failed/interrupted builds, raced or invalid RED attempts, preflight failures and superseded validations remain historical. `README.pending.md` and frozen proposal status statements describe those past checkpoints, not the current approval state. Final validation cleans all four product packages after the copied-workspace probe before rebuilding the actual checkout. No source/L0 changed during final gates or binary acceptance. This closes the bounded repair, not the remaining batch G work or complete language parity.
''')
print('Bounded closure report written:',len(pairs),'owner pairs;',len(inputs),'frozen inputs')
