"""Publish precise predicate locations and conjunctive structural proof duties.
This is artifact authoring, not a semantic test or an execution of the checker.
"""
import copy, hashlib, json, subprocess
from pathlib import Path
D = Path(__file__).resolve().parent
def pin(name):
    p = D / name
    return {'path': str(p), 'sha256': hashlib.sha256(p.read_bytes()).hexdigest()}
def read(name): return json.loads((D/name).read_text())
def publish(name, data):
    p = D/name
    if p.exists(): raise SystemExit('immutable output exists: '+str(p))
    body=json.dumps(data,indent=2,ensure_ascii=True)+'\n'
    patch='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+line+'\n' for line in body.split('\n')[:-1])+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,check=True,capture_output=True)
    print(name, pin(name)['sha256'])
programs=['p1339-ab-batch1-lifecycle-predicate.py','p1339-ab-batch1-lifecycle-predicate-v2.py','p1339-ab-batch1-lifecycle-predicate-v3.py']
def at(label):
    found=[]
    for name in programs:
        for n,line in enumerate((D/name).read_text().split('\n'),1):
            if ('\''+label+'\'') in line and 'require(' in line:
                found.append({'artifact':pin(name),'line':n,'assertion_label':label,'exact_statement':line.strip()})
    if len(found)!=1: raise ValueError((label,found))
    return found[0]

# Each SW below has a narrow named residual, actual source evidence and a
# falsifier. Runtime predicates remain independently mandatory conjuncts.
proofs={
 'capture-zero': (
  'For retained-sibling-growth context 0, map actual stable.final result -> .first -> local captured -> closure captured environment field -> metadata Value::Func. Require the environment stores Int(0) from that exact read, without substituting a fresh closure on A2..A5.',
  'Exact callsite/AST span and all assignment/capture construction def-use edges; actual capture field name/type and immutable lifetime; link to runtime request_id, capture_id and func_id. A constant unrelated zero, missing edge or reconstructed closure fails.'),
 'capture-replacement': (
  'For replaced-capture-descendant outer context 0 and child context 1, each actual changing.final read 0..4 reaches the new child captured environment as the same typed Int. Child source identity is fixed; resource capture identities are five distinct runtime identities.',
  'Enumerate actual lexical-binding capture construction and producer replacement paths with path/line; bind each of the five body execution_ids to its captured field and originating outer generation. Require no stale capture, not merely distinct producer IDs or metadata values.'),
 'chain-replacement': (
  'For replaced-chain-descendant, the actual style setter computes (10 + captured)*1pt from outer reads 0..4. Actual descendant entry chain text-size projection is respectively 10/11/12/13/14pt, and each real request uses that entry chain.',
  'Exact style delta construction, push, descendant capture and request-record/replay def-use path/line. Bind actual entry_chain_id to the actual StyleChain::size projection and Int capture from the same outer generation; all five assignments must appear. Metadata alone does not discharge chain identity or projection.'),
 'sibling-chain-sizes': (
  'For request-chain-points child contexts 1 and 2, actual entry StyleChain text-size projects 10pt and 20pt in that order. Distinct actual records share the same filtered key and result but cannot be interned by that equality.',
  'Map both set text source spans through real push and descendant entry capture; quote actual record insertion key/ownership and request chain paths; bind each chain_id to actual 10pt/20pt projection. Reject any final/default-chain substitution or record keyed solely by equal request.'),
 'invocation-origin-completeness': (
  'For all lifecycle fixtures, FunctionInvoked must exhaust all actual callback invocations reachable from private comparison, validation, diagnosis and counter folding. Comparator/category/reflexivity dispatch invokes no Func; counter callbacks run only on actual counter-fold operations.',
  'Enumerate callsites to actual Func invocation in reachable comparison/validation/diagnosis/fold callgraph; every callsite must have one truthful passive origin hook. Empty event list without this exhaustive callsite audit fails. Counter fold edges retain operation provenance; test-projection callbacks cannot leak to the product.'),
 'final-request-diagnostic-lineage': (
  'For increasing_set and oscillating_set, every generated final request diagnostic is for a request in the final current generation. The actual diagnosis replays that request against I0,I1,I2,I3,I4,I5 in order; no unrelated request detail or fabricated history.',
  'Exact final retained record -> request iterator -> historical snapshots -> SourceDiagnostic vector def-use, with snapshot/order and request identity at each edge. Enumerate all diagnostic append sites and prove their request belongs to final current records. Conjoin exact pinned CLI equal/different raw diagnostic oracles for warning bytes/order/spans.'),
 'ordinary-nested-producer': (
  'For nested_producer source contexts 0 and 1, neither update construction reaches a filtered read. Their actual ordinary content is retained and the nested context/update is obtained by traversal of produced Content, not scanning source/AST for demand.',
  'Quote every actual registration/discovery callsite and closed Content arm reached by this nested source; trace ordinary output/producer identity through selected-attempt seed and descendant insertion. Prove selection flags false at both construction-only contexts and one actual Set(12) contribution in each produced store. Reject recreated ordinary producer or duplicate nested update.'),
 'nested-counter-transition': (
  'For nested_producer consumer context 2, the first selected read is [0] from empty I0; the next necessary evaluation reads [12] after one retained ordinary Set(12). It succeeds with metadata [12]; no stale assertion survives.',
  'Bind consumer RequestRecorded/BodyCompleted execution IDs and actual snapshot projections to source get/assert/get spans. Enumerate registry fold events, requiring exactly one Set(12), no automatic heading event and no historical duplicate. Empty I0 construction and next invalidation edge must be concrete def-use, not page-count inference.'),
 'ordinary-func-retention': (
  'For fresh_callback_stable context 0, c.update creates but does not demand the filtered counter. Its ordinary callback Func and complete output remain the exact immutable original through all selected attempts.',
  'Bind discovery output, callback Func identity and captured environment to actual current contribution/store event IDs; enumerate writes/reconstruction branches and prove none recreate or re-evaluate that producer solely for equality. Show construction route never reaches filtered demand recorder.'),
 'fresh-event-order': (
  'For fresh_callback_stable, each actual current store event order is automatic heading A, manual Func update n=>n+10, automatic heading B; exactly three events, no older generation duplication.',
  'List actual Introspector store event traversal entries with Location/Span/action and producer generation; show closed automatic/manual classification paths and exact document-order iterator. Actual fold from [0] uses Step(1), retained Func(+10), Step(1) and yields [12]. Reject order sorting by callback identity or repeated old event.'),
 'nan-language-vs-request': (
  'For nan_key_stable_zero, original and replayed request contain the same NaN IEEE bits and the private relation is Same, while language counter-key matching does not associate the ordinary update12 with that NaN key. Both actual read projections are [0].',
  'Map exact request field bits through relation leaf and recursion; map the distinct language selector association callsite and nonassociation branch. Bind actual counter read/replay results and I0/produced stores. No unification of these two relations, zero special-case fallback or forced retry is allowed.'),
 'provisional-error-transition': (
  'For provisional-assert-final-panic, construction-only context 0 remains ordinary and contributes one Set(12) only after empty I0. Consumer A1 records [0] before assertion Err; validation against [12] is false, invalidation discards that generation and its sink; next necessary execution reads [12], passes assert and produces original panic Err.',
  'Bind each request/body/validation/invalidation/sink ID at exact source spans and enumerate the two real control-flow branches. Require complete provisional Err be absent from final retained error and publication; show ordinary update cannot populate I0. Require recorded value [12] precedes panic on replacement, without evaluation after panic.'),
 'final-error-trace-preservation': (
  'For all three error fixtures, final complete diagnostic vectors are the actual retained body Err without sorting, textual reclassification, trace rewriting or omission. Persistent error never becomes document output/export.',
  'Exact body Result -> retained contribution -> final Result -> export rejection def-use; audit every map/filter/diagnostic append on that path. For legacy-sibling-error the chosen origin must be context 0 ordinary execution; for provisional panic the replacement execution; for stable missing label the originally recorded body error.'),
 'selection-before-missing-label': (
  'For selected-stable-error, filtered c.at demand sets selection and records the request before label resolution fails; stored Err includes the actual missing-label diagnostic Span and ordered trace. Revalidation of this same error can return Ok(true).',
  'Enumerate real method dispatch, selection write, request registration, label lookup and Err propagation callsites with ordering/dominance. Bind actual RequestRecorded span/typed label/error fields and actual ValidationCompleted result. Prove no pre-resolution early return bypasses selection/recording.'),
}

# Explicit one-to-one mapping of all 50 original propositions. Each item is
# (runtime assertion labels, exact residual SW ids). No wildcard check() refs.
M={
'lifecycle-retained-sibling-growth':[
 (['V3-stable-zero-every-read-and-candidate','V3-stable-requests-exist','V3-stable-request-results-zero'],['capture-zero']),
 (['exact attempt sequence','actual produced counter projections','explicit retention A2..A5','original output present at return'],[]),
 (['stable producer executes once after discovery','retain original output','retain original embedded funcs','retain original sink','retain complete original request sequence','retain causal producer/capture/location/chain/world/metrics/target/features','selected discovery discarded before seed'],[]),
 (['no function invocation for equality/category'],['invocation-origin-completeness'])],
'lifecycle-replaced-capture-descendant':[
 (['actual read counter projections'],['capture-replacement']),
 (['V3-stable-request-results-zero','V3-old-child-invalidated-before-new-body','invalidate actual descendant set, no omissions','invalidated descendant never reused'],[]),
 (['actual metadata projection for each Dk','actual final metadata','one current update, not historical accumulation','no stale update generation'],[]),
 (['child re-evaluates each actual replacement','different child generation, even equal source','actual child producers replaced','V3-replacement-capture-identities-distinct','V3-replacement-parent-generations-distinct','V3-child-request-capture-resources-at-boundary'],['capture-replacement'])],
'lifecycle-replaced-chain-descendant':[
 (['actual read counter projections','V3-replacement-entry-chain-identities-distinct'],['chain-replacement']),
 (['V3-stable-request-results-zero','V3-old-child-invalidated-before-new-body','invalidated descendant never reused','every actual registered request replayed in original order'],[]),
 (['actual metadata projection for each Dk','actual final metadata'],[]),
 (['V3-replacement-entry-chain-identities-distinct','V3-replacement-capture-identities-distinct','V3-child-request-capture-resources-at-boundary','V3-child-request-actual-entry-chain'],['chain-replacement'])],
'lifecycle-request-chain-points':[
 (['actual final metadata','V3-sibling-entry-chains-disjoint'],['sibling-chain-sizes']),
 (['V3-sibling-request-exists','V3-sibling-request-chain-and-zero-result','replay exact request chain, not final chain','replay exact request resources'],[]),
 (['V3-sibling-record-generations-disjoint','V3-sibling-entry-chains-disjoint','V3-sibling-request-chain-and-zero-result'],['sibling-chain-sizes'])],
'lifecycle-nested_producer':[
 ([],['ordinary-nested-producer']),
 (['I0 observational stores empty','actual final metadata'],['nested-counter-transition']),
 (['actual final metadata','V3-stable-success-not-error','no attempt reset','no stale update generation'],['ordinary-nested-producer','nested-counter-transition'])],
'lifecycle-fresh_callback_stable':[
 ([],['ordinary-func-retention']),
 (['actual final metadata','no function invocation for equality/category'],['invocation-origin-completeness','ordinary-func-retention']),
 (['no stale update generation'],['fresh-event-order'])],
'lifecycle-nan_key_stable_zero':[
 ([],['nan-language-vs-request']),
 (['exact attempt sequence','actual final metadata','V3-stable-success-not-error','V3-stable-no-published-warning'],['nan-language-vs-request'])],
'lifecycle-provisional-assert-final-panic':[
 (['I0 observational stores empty'],['provisional-error-transition']),
 (['request during actual body','all actual body and validation sinks have disposition','discarded sink never published'],['provisional-error-transition']),
 (['exact final message','V3-final-complete-error-equals-real-body-error'],['provisional-error-transition','final-error-trace-preservation']),
 (['error is not export success','transient message discarded','V3-error-not-downgraded-by-published-warning'],['final-error-trace-preservation'])],
'lifecycle-legacy-sibling-error':[
 (['legacy sibling not selected','legacy sibling not reevaluated','exact final message'],['final-error-trace-preservation']),
 (['V3-final-complete-error-equals-real-body-error','error is not export success','legacy sibling not reevaluated'],['final-error-trace-preservation'])],
'lifecycle-selected-stable-error':[
 (['V3-missing-label-body-selected','V3-missing-label-real-error-requests','exact final message'],['selection-before-missing-label']),
 (['persistent read error can validate','validated error remains Err','V3-final-complete-error-equals-real-body-error','error is not export success'],['final-error-trace-preservation']),
 (['V3-missing-label-no-callback','V3-missing-label-no-success-output','V3-error-not-downgraded-by-published-warning'],['invocation-origin-completeness'])],
}
for name in ['increasing_set','oscillating_set']:
 M['lifecycle-'+name]=[
  (['exact attempt sequence'],[]), (['actual read counter projections'],[]),
  (['actual produced counter projections'],[]), (['actual final metadata'],[]),
  (['V3-return-at-A5','V3-no-A6-body','return actual current Dk','Ak reads I(k-1)'],['SW-F08-shared-pagination-budget']),
  (['V3-one-current-Set-zero-automatic-events','I0 observational stores empty','no stale update generation'],[]),
  (['V3-exact-historical-counter-projection-I0-I5'],['final-request-diagnostic-lineage']),
  ([],['final-request-diagnostic-lineage']),
  (['no attempt reset','shared ceiling, never A6'],['SW-F08-shared-pagination-budget'])]

old=read('p1339-ab-batch1-retention-lifecycle-fixtures.json')
typed=read('p1339-ab-batch1-retention-lifecycle-typed-r2.json')
coverage=read('p1339-ab-batch1-coverage-map.json')
precise=[]
for case in old['cases']:
    entries=M[case['id']]
    if len(entries)!=len(case['predicates']): raise ValueError(case['id'])
    for index,(labels,sws) in enumerate(entries):
        precise.append({'case_id':case['id'],'predicate_index':index,'original_predicate':case['predicates'][index],
          'typed_assertions':[at(label) for label in labels],
          'conjunctive_structural_witnesses':['SW-LC-'+s if s in proofs else s for s in sws],
          'phase':'F_NOT_EXECUTED_PRESEAL_ZERO_CREDIT',
          'failure_policy':'Every listed assert and case-specific SW is mandatory; unsupported/missing source binding or observed mismatch fails final coverage. No generic check() reference discharges this proposition.'})
if len(precise)!=50: raise ValueError(len(precise))
extra=[]
for key,(claim,evidence) in proofs.items():
    affected=[{'case_id':r['case_id'],'predicate_index':r['predicate_index']} for r in precise if 'SW-LC-'+key in r['conjunctive_structural_witnesses']]
    extra.append({'id':'SW-LC-'+key,'phase':'F','mandatory':True,'exact_claim':claim,'required_evidence':evidence,
      'affected_propositions':affected,'evidence_format':{'actual_source_edges':'array of {path,sha256,line,symbol,caller,callee,def,use,branch}; nonempty and exhaustive for named route',
        'runtime_bindings':'array of {case_id,profile,order,execution_id,event_sequence,field,actual_source_edge_ref}; all four profiles and three orders when the claim binds runtime identities',
        'audit':'independent verifier accepted/rejected with counterexample, never author self-attestation'},
      'failure_policy':'Any absent edge, contradictory binding, uncovered branch or Unknown fails this mandatory final witness; no inferred identity from producer ID/metadata/address alone.'})
    if key=='final-request-diagnostic-lineage':
        extra[-1]['exact_cli_conjunct']={'artifact':pin('p1339-ab-batch1-cli-oracles.json'),'selectors':['oracles[id=stabilization-boundaries-'+n+'-'+s+']' for n in ['increasing_set','oscillating_set'] for s in ['equal','different']],'channels':['exit','stdout','stderr','artifact_presence'],'normalization':'none'}
typed['predecessor']=pin('p1339-ab-batch1-retention-lifecycle-typed-r2.json')
typed['predicate_program']=pin('p1339-ab-batch1-lifecycle-predicate-v3.py')
typed['structural_witnesses']+=extra
typed['original_lifecycle_predicate_map']=precise
typed['predicate_precedence']='All original 50 propositions remain conjunctive. Each exact assertion and narrow SW is mapped below; no prose rationale removes an obligation. Future runtime and actual-source proof remain NOT_EXECUTED_PRESEAL.'
typed['authoring_corrections']=['Same aggregate batch1: v3 adds explicit capture/entry-chain identities, stable read vectors, causal invalidation order, actual published sinks, complete final body-error equality and exact missing-label predicates. No semantic process or new contract revision.']
publish('p1339-ab-batch1-retention-lifecycle-typed-r3.json',typed)
coverage['predecessor']=pin('p1339-ab-batch1-coverage-map.json')
coverage['original_lifecycle_predicate_map']=precise
coverage['structural_witness_definitions']+=extra
coverage['exact_lifecycle_fixture']=pin('p1339-ab-batch1-retention-lifecycle-typed-r3.json')
coverage['authoring_delta']='Replace generic 50x check()+SW mapping with exact immutable line/predicate pins and 14 narrowly falsifiable residual witnesses; all final-only and no proof claimed executed.'
publish('p1339-ab-batch1-coverage-map-r2.json',coverage)
