"""Publish independent future-operation fixtures; never run a Typst executable."""
import copy
import hashlib
import json
import pathlib
import subprocess

D = pathlib.Path(__file__).resolve().parent
def pin(name):
    p = D / name
    return {'path': str(p), 'sha256': hashlib.sha256(p.read_bytes()).hexdigest()}
def publish(name, data):
    p = D / name
    assert not p.exists(), p
    text = json.dumps(data, ensure_ascii=True, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(p) + '\n' + '\n'.join('+' + x for x in text.split('\n')[:-1]) + '\n*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, capture_output=True, check=True)
    print(json.dumps(pin(name)))

common = {
    'author': '/root/p1319_tests', 'regime': 'executado sem atestacao de isolamento',
    'batch': 1, 'status': 'COMPONENT_OF_BATCH_1_NOT_A_RUN_MANIFEST',
    'authority_manifest': pin('p1339-authority-manifest-r2.json'),
    'contract': pin('p1339-contract-r3.json'), 'l0_freeze': pin('p1339-l0-freeze.json'),
    'budget_redesign': pin('p1339-budget-redesign-r1.json'),
    'budget_acceptance': pin('p1339-verifier-budget-redesign-acceptance-r1.md'),
    'phase': {'freeze': 'C', 'execute': 'F', 'preseal_state': 'NOT_EXECUTED_PRESEAL', 'runtime_credit': 0},
    'candidate_access': 'No candidate or productive source read. Inputs derived from frozen L0 and public historical sources/signature DTOs.',
}

# This supplies the concrete public request missing from the private opaque pair.
old = json.loads((D / 'p1339-ab-closed-api-fixtures-r1.json').read_text())
opaque = copy.deepcopy(next(c for c in old['cases'] if c['id'] == 'api-query-carrier-none'))
opaque['id'] = 'api-query-independent-dynamic-noncertification'
opaque['value_bindings'] = []
opaque['binding_operations'] = [
    {'kind': 'eval_binding', 'output_binding': 'body', 'expression': '[opaque]'},
    {'kind': 'dynamic_callout', 'body_binding': 'body', 'title': 'same', 'tone': 'info', 'output_binding': 'left'},
    {'kind': 'dynamic_callout', 'body_binding': 'body', 'title': 'same', 'tone': 'info', 'output_binding': 'right'},
]
opaque['snapshots'] = [
    {'id': 'base', 'origin': 'content_expression', 'content_expression': '[#metadata(left)<node>]', 'introspection_entrypoint': 'pure'},
    {'id': 'independent', 'origin': 'clone_of', 'clone_of': 'base', 'overlays': [
        {'kind': 'element', 'location': 'label:node', 'content_expression': 'metadata(right)',
         'fields': [['value', 'right'], ['label', '<node>']]}]},
]
opaque['validate_candidate_ids'] = ['independent']
opaque['history_ids'] = ['base', 'independent']
opaque['predicates'] = {
    'body': {'result': 'Ok', 'value_type': 'Array', 'length': 1},
    'has_filtered_counter_reads': True,
    'validation': {'independent': {'allowed_result_variants': ['Ok(false)', 'Err(SourceDiagnostic)'], 'forbidden': ['Ok(true)', 'Unknown', 'NotDue']}},
    'nonconvergence_diagnostics': {'allowed_result_variants': ['Ok(empty)', 'Err(SourceDiagnostic)'],
        'forbidden': ['fabricated vanilla nonconvergence warning for Unproven', 'Unknown', 'NotDue']},
}
opaque['matrix_ids'] = ['F05-projections', 'F06-closed-comparator', 'F09-errors-sinks']
opaque['role'] = 'opaque'
opaque['required_control'] = True
opaque['positive_credit'] = 0
opaque['private_pair_link'] = {'artifact': pin('p1339-ab-private-relation-fixtures-r1.json'), 'case_id': 'relation-opaque-independent-callout',
    'binding_equivalence': 'Execute the same three constructor operations once for this case. Bind private pair to these actual left/right Values, not re-created values. Private relation must be Unproven; public validation must independently not certify.'}
publish('p1339-ab-batch1-public-opaque-fixture.json', {**common, 'schema': 'p1339-public-opaque-fixture-v1', 'construction_rules': old['construction_rules'], 'cases': [opaque]})

boundaries_name = 'p1339-stabilization-boundaries-runs.json'
boundaries = json.loads((D / boundaries_name).read_text())['cases']
sources = {
    'retained-sibling-growth': '#let stable = counter(heading.where(level: 9))\n#let changing = counter(heading.where(level: 1))\n#context { let captured = stable.final().first(); metadata(x => x + captured) }\n#context changing.update(changing.final().first() + 1)',
    'replaced-capture-descendant': '#let stable = counter(heading.where(level: 9))\n#let changing = counter(heading.where(level: 1))\n#context { let captured = changing.final().first(); [#context { stable.final(); metadata(captured) }] }\n#context changing.update(changing.final().first() + 1)',
    'replaced-chain-descendant': '#let stable = counter(heading.where(level: 9))\n#let changing = counter(heading.where(level: 1))\n#context { let captured = changing.final().first(); [#set text(size: (10 + captured) * 1pt)\n#context { stable.final(); metadata(text.size) }] }\n#context changing.update(changing.final().first() + 1)',
    'request-chain-points': '#let c = counter(heading.where())\n#context { c.final(); [#set text(size: 10pt)\n#context { c.final(); metadata(text.size) }\n#set text(size: 20pt)\n#context { c.final(); metadata(text.size) }] }',
    'provisional-assert-final-panic': '#let c = counter(heading.where())\n#context c.update(12)\n#context { assert(c.final() == (12,)); panic("P1339_FINAL_PANIC") }',
    'legacy-sibling-error': '#let c = counter(heading.where())\n#context panic("P1339_LEGACY_SIBLING")\n#context c.update(12)\n#context c.final()',
    'selected-stable-error': '#let c = counter(heading.where())\n#context c.at(<missing>)',
}
for key in ['oscillating_set', 'increasing_set', 'fresh_callback_stable', 'nested_producer', 'nan_key_stable_zero']:
    sources[key] = boundaries[key]

dtos = {
    'opaque_identity': 'A lossless token backed by an actual immutable object/generation identity or independently audited reference equality. Fixture names/source labels are lookup coordinates only, never identity evidence.',
    'event_order': 'Append-only sequence number from passive hooks at actual execution boundaries; ordering must not be synthesized from the desired transcript.',
    'events': {
        'DiscoveryCompleted': ['generation_identity', 'producer_identity', 'parent_generation_identity', 'source_span', 'selected', 'body_result', 'output_identity', 'sink_identity'],
        'AttemptStarted': ['attempt_number', 'read_snapshot_identity', 'read_snapshot_projection'],
        'BodyStarted': ['attempt_number_or_discovery', 'generation_identity', 'producer_identity', 'capture_identity', 'parent_generation_identity', 'context_location', 'entry_chain_identity', 'world_identity', 'font_metrics_identity', 'target', 'features'],
        'RequestRecorded': ['generation_identity', 'request_identity', 'operation_variant', 'typed_arguments', 'context_location', 'span', 'chain_identity_at_request', 'observed_result_or_complete_diagnostic'],
        'BodyCompleted': ['generation_identity', 'body_result_or_complete_diagnostic', 'output_identity', 'embedded_func_identities', 'final_chain_identity', 'sink_identity'],
        'CandidateBuilt': ['attempt_number', 'content_identity', 'document_identity', 'produced_snapshot_identity', 'snapshot_projection', 'page_store_complete', 'positions_complete'],
        'ValidationCompleted': ['attempt_number', 'generation_identity', 'request_identities', 'candidate_snapshot_identity', 'public_result', 'private_relations', 'validation_sink_identity'],
        'ContributionRetained': ['attempt_number', 'generation_identity', 'producer_identity', 'capture_identity', 'output_identity', 'request_identities', 'original_execution_identity', 'original_sink_identity', 'resource_tuple'],
        'GenerationInvalidated': ['attempt_number', 'generation_identity', 'actual_reason', 'invalidated_descendant_identities'],
        'SinkDisposition': ['sink_identity', 'origin_generation_identity', 'origin_phase', 'actual_disposition', 'complete_diagnostics'],
        'CompilationReturned': ['attempt_number', 'document_identity_or_none', 'result_or_complete_diagnostics', 'published_sink_origins', 'export_permitted'],
    },
    'snapshot_projection': {
        'counter_requests': 'For explicitly named keys, project through the real semantic counter owner; retain full Result. Never add this probe request to product observation records.',
        'metadata_values': 'Read the actual MetadataStore/content contribution; serialize supported values losslessly and keep Func/Content identities separately. Do not compare through repr.',
        'counter_events': 'Actual ordered events with key, action variant, payload, source Location/span and current producing generation; no accumulated historical entries.',
        'parent_child_edges': 'Actual current context tree, Location and producer/generation identities.',
        'pages_positions': 'Actual completed PageStore and positions; identity plus full relevant values, never page count as stability proof.',
    },
    'complete_diagnostic': ['severity', 'span', 'message', 'ordered_hints', 'ordered_trace_with_spans'],
    'resource_tuple': ['producer_identity', 'capture_identity', 'context_location', 'entry_chain_identity', 'world_identity', 'font_metrics_identity', 'target', 'features'],
    'ports': {
        'retention_actual': 'Run the actual paginated compilation for the supplied exact source and observe real generation/retention transitions. No test driver decides retain/re-evaluate.',
        'attempt_transcript_actual': 'Observe the actual pipeline cycle and eval request/sink boundaries for the supplied source; return ordered DTO events above. No fixture-specific cycle implementation.',
    },
}

global_predicates = [
    'Every required event is backed by a reachable actual productive operation; missing event/field or ambiguous identity is a binding failure, never an expected value.',
    'Discovery-selected output and sink are discarded before A1; discovery does not consume an attempt number. Legacy sibling contributions/errors retain their ordinary handling.',
    'Each ContributionRetained has the identical original execution, producer, capture, output, requests and sink identities; no new BodyStarted is fabricated for that reuse.',
    'Every retained resource_tuple equals the one at that generation BodyStarted. A changed producer/capture/entry chain invalidates all its old descendant records even where their own snapshot observations are unchanged.',
    'Each request uses the chain captured at its RequestRecorded boundary; replay does not substitute the final body chain. Resources are the same logical World/metrics/context/target/features by construction.',
    'CandidateBuilt precedes validation against that same produced snapshot and contains completed positions/PageStore. Attempt k reads the snapshot produced by k-1, with actual empty observational stores at I0 and preserved contextual source Location.',
    'Selected generation remains selected for its lifetime even if a later branch executes no filtered request. Selection never leaks to sibling generations.',
    'A generation with current Err contributes only its marker, never a previous success. Replacement removes old descendants/updates; no duplicate source marker or old-generation contribution remains in the current tree.',
    'Validation sinks and replaced/discovery-selected execution sinks are discarded. Retained contribution sinks are published once with original ordered complete diagnostics. No fabricated, sorted or deduplicated-by-text errors.',
    'Final body Err remains error even if read validation is Ok(true). Export is forbidden for pending final Err; Unproven never becomes a vanilla nonconvergence warning.',
]

def case(key, matrix, assertions):
    source = sources[key] + ('\n' if not sources[key].endswith('\n') else '')
    return {'id': 'lifecycle-' + key, 'ports': ['retention_actual', 'attempt_transcript_actual'],
            'matrix_ids': matrix, 'obligation_ids': ['W05', 'W07', 'W08', 'W09', 'W10'],
            'source': source, 'source_sha256': hashlib.sha256(source.encode()).hexdigest(),
            'source_origin': {'artifact': pin(boundaries_name), 'selector': 'cases.' + key} if key in boundaries else {'kind': 'independent L0-derived source; not an ancestor same-graph bridge'},
            'execution': {'entrypoint': 'actual paginated compilation facade and pipeline', 'named_source_id': 'p1339-ab-' + key + '.typ', 'target': 'paged', 'profiles': ['default', 'html', 'a11y', 'html+a11y'], 'orders': ['normal', 'repeat', 'reverse'], 'world': 'fresh immutable memory World per compilation; same instance for all attempts and replay', 'metrics': 'same FixedMetrics', 'element_registry': 'empty'},
            'observation_lookup': 'Use exact source spans and actual parent-child traversal to locate context bodies; never identify generations only by numeric ContextBlock id or text.',
            'predicates': assertions, 'phase': common['phase']}

cases = []
cases.append(case('retained-sibling-growth', ['F07-retention','F08-attempts'], [
    'The first context is selected by stable.final; its read is (0,) in every snapshot. Its A1 metadata closure captures Int(0).',
    'The changing-key sibling requires A1..A5; its produced counter values are 1,2,3,4,5. The stable-key output generation from A1 is retained unchanged through return D5.',
    'Exactly one real BodyStarted for the stable first context among A1..A5; its captured Func and complete output/request/sink identities at D5 equal their A1 identities. Discovery execution is separate and discarded.',
    'No function is invoked merely to compare or certify its identity; private comparison/replay hooks must show no body invocation originating from comparator dispatch.',
]))
cases.append(case('replaced-capture-descendant', ['F07-retention','F08-attempts'], [
    'Changing key reads per A1..A5 are 0,1,2,3,4; outer selected producer captures those exact Int values.',
    'Child selected stable-key query is always (0,), yet each replacement of outer producer invalidates the old child generation and its record before new child evaluation.',
    'Current metadata values per D1..D5 are exactly [0],[1],[2],[3],[4]; final is [4], never [0] or [5]. No earlier child metadata/update remains.',
    'Child body source span is the same across replacements, while real producer/capture identities and generation provenance distinguish them; identical source/body or page count cannot certify reuse.',
]))
cases.append(case('replaced-chain-descendant', ['F07-retention','F08-attempts'], [
    'Outer changing-key reads are 0,1,2,3,4; new descendant entry text-size chains are 10pt,11pt,12pt,13pt,14pt respectively.',
    'Child stable-key read remains (0,), but every replaced outer producer/chain invalidates its prior descendants and preserves no stale request record.',
    'Actual metadata in D1..D5 is [10pt],[11pt],[12pt],[13pt],[14pt], typed Length, not text; final D5 metadata is [14pt].',
    'Chain and capture identity changes are observed at real BodyStarted/RequestRecorded boundaries, not inferred from metadata alone.',
]))
cases.append(case('request-chain-points', ['F07-retention','F09-errors-sinks'], [
    'The two child contexts enter with distinct 10pt and 20pt text-size chains; their current metadata results are [10pt,20pt] in document order.',
    'Every child counter request records its own actual entry/effective chain; validations use that exact request chain, with resource tuples preserved.',
    'The two child records remain distinct despite same filtered counter key and same result (0,); neither is replaced by a shared default/final sibling chain.',
]))
for key, reads, produced in [('increasing_set',[0,1,2,3,4],[1,2,3,4,5]), ('oscillating_set',[0,1,0,1,0],[1,0,1,0,1])]:
    cases.append(case(key, ['F08-attempts','F09-errors-sinks'], [
        {'assertion': 'exact_attempt_numbers', 'value': [1,2,3,4,5]},
        {'assertion': 'counter_read_values_by_attempt', 'value': reads},
        {'assertion': 'produced_counter_values_by_attempt', 'value': produced},
        {'assertion': 'returned_metadata_value', 'value': [[reads[-1]]]},
        'Return the identical document D5 produced while reading I4; no A6 BodyStarted and no exported output from replay on I5; do not return D4.',
        'Each I1..I5 has exactly one current manual Set event for c and zero automatic heading events; old attempt updates are absent. I0 has no c updates.',
        {'assertion': 'final_request_historical_counter_projection_I0_to_I5', 'value': [0] + produced},
        'Each final diagnostic corresponds to a request actually recorded by the final retained generation; preserve the public vanilla warning order, full history and source spans via the separately pinned CLI oracle. Unrelated requests receive no invented detail.',
        'Attempt budget is shared with actual pagination/position invalidations; the transcript must not restart numbering or add five inner rounds on a relayout.',
    ]))
cases.append(case('nested_producer', ['F07-retention','F08-attempts','F09-errors-sinks'], [
    'The outer non-demanding producer retains its ordinary contribution; its real nested context/update is discovered by Content traversal, not source scanning.',
    'Consumer selected attempt A1 reads empty counter (0,) and may fail its assert; after incorporating ordinary nested producer, candidate counter is (12,). Its next necessary evaluation succeeds with metadata (12,).',
    'Actual returned metadata is [(12,)]; final has no assertion error. New descendant discovery never resets the attempt budget and exactly one current update contributes 12, not duplicated 24.',
]))
cases.append(case('fresh_callback_stable', ['F07-retention','F08-attempts'], [
    'The producer of c.update(n => n + 10) does not demand c, so its ordinary output and callback Func identity are retained; no reconstruction solely for equality.',
    'Consumer final metadata is [(12,)]; retained callback is folded only by actual demanded counter readings, with no comparison-driven function invocation.',
    'The current snapshot contains the two actual automatic heading events and one current manual Func update in document order; no historical duplication.',
]))
cases.append(case('nan_key_stable_zero', ['F07-retention','F08-attempts','F09-errors-sinks'], [
    'The selected consumer reads (0,) against I0 and the produced candidate. Its NaN-containing request validates reflexively by identical IEEE bits, while language key association does not attach update12.',
    'First selected attempt validates and returns metadata [(0,)] without nonconvergence diagnostics; no force-repeat merely because request contains NaN.',
]))
cases.append(case('provisional-assert-final-panic', ['F08-attempts','F09-errors-sinks'], [
    'Ordinary update producer remains nonselected and contributes 12 only to produced snapshots, never artificially to empty I0.',
    'Selected A1 consumer records filtered read (0,) before its assertion Err. Candidate (12,) invalidates that observation; the replaced assertion error and its sink are discarded.',
    'Reevaluation observes (12,), passes assert and reaches the original P1339_FINAL_PANIC; if that recorded read validates, the original panic Err with its full trace is returned unchanged.',
    'Export is false, document return is None; no provisional assertion survives and no nonconvergence warning downgrades the final panic.',
]))
cases.append(case('legacy-sibling-error', ['F04-selection','F08-attempts','F09-errors-sinks'], [
    'The first context reaches no filtered demand; its P1339_LEGACY_SIBLING error remains the ordinary result. Another selected context does not mark it selected or make its error provisional.',
    'Compilation remains Err with that original complete diagnostic; export false. No extra evaluation of that legacy context is invented to repair it.',
]))
cases.append(case('selected-stable-error', ['F04-selection','F09-errors-sinks'], [
    'The c.at(<missing>) demand marks the context selected before label resolution fails. The recorded error includes the missing label, source span and ordered trace.',
    'Revalidation against the same empty label registry may return Ok(true), but the body remains its original Err and export remains false.',
    'No callback, successful content contribution or nonconvergence warning is fabricated to replace the persistent error.',
]))

publish('p1339-ab-batch1-retention-lifecycle-fixtures.json', {
    **common, 'schema': 'p1339-independent-retention-lifecycle-fixtures-v1',
    'private_interface': pin('p1339-mutant-closed-state-private-interface.md'),
    'dto_schema': dtos, 'global_predicates': global_predicates, 'cases': cases,
    'translation_limits': [
        'Source strings and assertions are frozen inputs for this component. Adapter may only construct real inputs, invoke actual operations and project their real observations.',
        'No trial execution until the single aggregate batch manifest exists. No future private operation earns preseal runtime credit.',
        'Any inaccessible event, unsupported constructor, ambiguous source generation or inconsistent chain carrier must be reported as a concrete binding gap before seal, never inferred from expected JSON.',
        'Separate compilations/profiles get separate memory Worlds and records. Cross-compilation resource/record reuse is prohibited; structural audit must trace all eight resource_tuple members and request-chain capture to real owners.',
        'These scenarios alone are not a declaration of complete W/F coverage; the exhaustive field/clause witness map is a separate required component of the same batch.',
    ],
})
