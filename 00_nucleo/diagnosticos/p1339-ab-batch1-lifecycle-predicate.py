"""Frozen assertions over passive actual-product observations, never an orchestrator.

The adapter supplies the typed observation JSON prescribed by its companion
fixture. This checker does not construct snapshots, decide retention, execute a
body, project a counter, or manufacture any event. Missing data fails closed.
"""
import argparse
import collections
import json
from pathlib import Path

def require(condition, label):
    if not condition:
        raise AssertionError(label)

def check(case, observation):
    require(observation['case_id'] == case['id'], 'case identity')
    require(observation['execution'] == 'Observed', 'actual execution required')
    require(observation['binding_audit'] == 'accepted_real_product_path', 'independent actual-path audit')
    events = observation['events']
    require([x['sequence'] for x in events] == list(range(len(events))), 'ordered actual events')
    def rows(kind): return [x for x in events if x['kind'] == kind]
    def at(kind, attempt):
        r=[x for x in rows(kind) if x['attempt'] == attempt]
        require(len(r)==1, (kind, attempt, 'exactly one boundary'))
        return r[0]
    starts=rows('AttemptStarted');built=rows('CandidateBuilt');returned=rows('CompilationReturned')
    require(len(returned)==1, 'one actual compilation return')
    final=returned[0]
    require([x['attempt'] for x in starts] == list(range(1,len(starts)+1)), 'no attempt reset')
    require(len(starts)<=5, 'shared ceiling, never A6')
    require(len({x['snapshot_id'] for x in built})==len(built), 'produced snapshots are actual separate generations')
    for start in starts:
        k=start['attempt']
        if k==1:
            require(start['observational_stores_empty'] is True, 'I0 observational stores empty')
        else:
            require(start['read_snapshot_id']==at('CandidateBuilt',k-1)['snapshot_id'], 'Ak reads I(k-1)')
    for candidate in built:
        require(candidate['positions_complete'] is True and candidate['page_store_complete'] is True, 'complete paginated snapshot')
        require(len(candidate['current_generation_ids'])==len(set(candidate['current_generation_ids'])), 'no duplicate current generation')
        for event in candidate['counter_events']:
            require(event['producer_generation_id'] in candidate['current_generation_ids'], 'no stale update generation')
    body_starts={x['execution_id']:x for x in rows('BodyStarted')}
    completions={x['execution_id']:x for x in rows('BodyCompleted')}
    require(len(body_starts)==len(rows('BodyStarted')), 'real execution identities unique')
    recorded={x['request_id']:x for x in rows('RequestRecorded')}
    require(len(recorded)==len(rows('RequestRecorded')), 'recorded request identities unique')
    for validation in rows('ValidationCompleted'):
        candidate=at('CandidateBuilt',validation['attempt'])
        require(validation['candidate_snapshot_id']==candidate['snapshot_id'], 'validate produced snapshot')
        require(validation['sequence']>candidate['sequence'], 'validate only after completed candidate')
        for replay in validation['replayed_requests']:
            original=recorded[replay['request_id']]
            require(replay['chain_id']==original['chain_id'], 'replay exact request chain, not final chain')
            require(replay['resources']==original['resources'], 'replay exact request resources')
            require(replay['operation']==original['operation'], 'same real operation')
            require(replay['typed_arguments']==original['typed_arguments'], 'same real typed request arguments')
    for kept in rows('ContributionRetained'):
        completion=completions[kept['original_execution_id']]
        start=body_starts[kept['original_execution_id']]
        require(kept['generation_id']==completion['generation_id']==start['generation_id'], 'retained generation')
        require(kept['output_id']==completion['output_id'], 'retain original output')
        require(kept['func_ids']==completion['func_ids'], 'retain original embedded funcs')
        require(kept['sink_id']==completion['sink_id'], 'retain original sink')
        require(kept['resources']==start['resources'], 'retain causal producer/capture/location/chain/world/metrics/target/features')
        expected=[x['request_id'] for x in rows('RequestRecorded') if x['execution_id']==kept['original_execution_id']]
        require(kept['request_ids']==expected, 'retain complete original request sequence')
    invalidations=rows('GenerationInvalidated')
    for invalidation in invalidations:
        require(invalidation['descendant_ids']==invalidation['actual_descendant_ids_before_replacement'], 'invalidate actual descendant set, no omissions')
        for candidate in built:
            if candidate['sequence']>invalidation['sequence']:
                require(not set(invalidation['descendant_ids']) & set(candidate['current_generation_ids']), 'invalidated descendant never reused')
    dispositions=rows('SinkDisposition')
    for sink_id, group in group_by(dispositions,'sink_id').items():
        require(sum(x['disposition']=='published' for x in group)<=1, 'publish contribution sink at most once')
        if any(x['origin_phase'] in ('validation','discovery_selected') for x in group):
            require(all(x['disposition']=='discarded' for x in group), 'discard validation/discovery-selected sink')
        if any(x['disposition']=='discarded' for x in group):
            require(not any(x['disposition']=='published' for x in group), 'discarded sink never published')
    if final['result']['kind']=='Err':
        require(final['document_id'] is None and final['export_permitted'] is False, 'error is not export success')
    elif final['result']['kind']=='Ok':
        require(final['document_id']==at('CandidateBuilt',final['attempt'])['document_id'], 'return actual current Dk')
    else:
        require(False,'unknown final result discriminant')

    expected=case['typed_predicates']
    if 'attempts' in expected:
        require([x['attempt'] for x in starts]==expected['attempts'], 'exact attempt sequence')
    if 'counter_values_produced' in expected:
        key=expected['counter_key_id']
        require([at('CandidateBuilt',k)['counter_values'][key] for k in expected['attempts']]==expected['counter_values_produced'], 'actual produced counter projections')
    if 'counter_values_read' in expected:
        key=expected['counter_key_id']
        require([at('AttemptStarted',k)['counter_values'][key] for k in expected['attempts']]==expected['counter_values_read'], 'actual read counter projections')
    if 'metadata_by_attempt' in expected:
        require([at('CandidateBuilt',k)['metadata_values'] for k in expected['attempts']]==expected['metadata_by_attempt'], 'actual metadata projection for each Dk')
    if 'final_metadata' in expected:
        require(final['metadata_values']==expected['final_metadata'], 'actual final metadata')
    if 'manual_set_events_per_snapshot' in expected:
        key=expected['counter_key_id']
        for candidate in built:
            selected=[e for e in candidate['counter_events'] if e['key_id']==key]
            require(len(selected)==expected['manual_set_events_per_snapshot'], 'one current update, not historical accumulation')
            require(all(e['action']['kind']=='Set' for e in selected), 'actual Set actions')
    if 'retained_context_index' in expected:
        offset=case['context_coordinates'][expected['retained_context_index']]['byte_offset']
        selected=[x for x in rows('BodyStarted') if x['attempt']>0 and x['source_context_offset']==offset]
        require(len(selected)==1,'stable producer executes once after discovery')
        original=completions[selected[0]['execution_id']]
        require(bool(original['func_ids']),'real output contains retained Func')
        require(original['output_id'] in final['current_output_ids'],'original output present at return')
        require(set(original['func_ids'])<=set(final['current_func_ids']),'original funcs present at return')
    if 'replaced_child_context_index' in expected:
        offset=case['context_coordinates'][expected['replaced_child_context_index']]['byte_offset']
        selected=[x for x in rows('BodyStarted') if x['attempt']>0 and x['source_context_offset']==offset]
        require([x['attempt'] for x in selected]==expected['attempts'],'child re-evaluates each actual replacement')
        require(len({x['generation_id'] for x in selected})==len(selected),'different child generation, even equal source')
        require(len({x['resources']['producer_id'] for x in selected})==len(selected),'actual child producers replaced')
    if 'final_error_message' in expected:
        require(final['result']['kind']=='Err','expected final error')
        require(expected['final_error_message'] in [d['message'] for d in final['result']['diagnostics']], 'exact final message')
    if 'forbidden_final_messages' in expected:
        messages=[d['message'] for d in final['result'].get('diagnostics',[])]
        require(not set(messages)&set(expected['forbidden_final_messages']),'transient message discarded')
    if expected.get('legacy_error_unselected'):
        offset=case['context_coordinates'][0]['byte_offset']
        discoveries=[x for x in rows('DiscoveryCompleted') if x['source_context_offset']==offset]
        require(len(discoveries)==1 and discoveries[0]['selected'] is False,'legacy sibling not selected')
        require(not any(x['source_context_offset']==offset and x['attempt']>0 for x in rows('BodyStarted')),'legacy sibling not reevaluated')
    if expected.get('stable_error_validation'):
        require(any(x['result']=={'kind':'Ok','value':True} for x in rows('ValidationCompleted')), 'persistent read error can validate')
        require(final['result']['kind']=='Err','validated error remains Err')
    require(not any(x['origin']=='comparator_category_test' for x in rows('FunctionInvoked')), 'no function invocation for equality/category')
    return {'case_id':case['id'],'assertions':'Preserved','execution':'Observed'}

def group_by(rows,key):
    groups=collections.defaultdict(list)
    for row in rows: groups[row[key]].append(row)
    return groups

if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--fixtures',required=True);parser.add_argument('--observations',required=True)
    args=parser.parse_args()
    fixtures=json.loads(Path(args.fixtures).read_text())
    observations=json.loads(Path(args.observations).read_text())
    by_id={x['case_id']:x for x in observations['cases']}
    require(len(by_id)==len(observations['cases']),'no duplicated case observations')
    require(set(by_id)=={x['id'] for x in fixtures['cases']},'exact required future-case set')
    for case in fixtures['cases']:
        print(json.dumps(check(case,by_id[case['id']]),ensure_ascii=True))
