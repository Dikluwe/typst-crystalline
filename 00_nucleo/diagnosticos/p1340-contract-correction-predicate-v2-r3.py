"""P1340 R3 explicit successor of v2: positive completeness, witnessed negative prefix."""
import argparse, collections, importlib.util, json
from pathlib import Path

P=Path(__file__).with_name('p1339-ab-batch1-lifecycle-predicate.py')
spec=importlib.util.spec_from_file_location('p1339_predicate_predecessor',P)
prior=importlib.util.module_from_spec(spec);spec.loader.exec_module(prior)
require=prior.require

def identity(value,label):
    require(isinstance(value,str) and bool(value),(label,'identity required'))
def result(value,label):
    require(isinstance(value,dict),(label,'Result object'))
    if value.get('kind')=='Ok':require('value' in value,(label,'Ok payload'))
    elif value.get('kind')=='Err':
        require(isinstance(value.get('diagnostics'),list) and value['diagnostics'],(label,'Err must have actual diagnostics'))
        for d in value['diagnostics']:
            require(set(['severity','span','message','hints','trace'])<=set(d),(label,'complete diagnostic'))
            require(d['severity'] in ['Error','Warning'] and isinstance(d['message'],str),(label,'diagnostic types'))
            require(isinstance(d['hints'],list) and all(isinstance(x,str) for x in d['hints']),(label,'hints'))
            require(isinstance(d['trace'],list),(label,'ordered trace'))
    else:require(False,(label,'unknown Result variant'))


def check_validation(v, expected_ids):
    """Actual traces only. relation is observed from the existing comparator call."""
    replayed = v['replayed_requests']
    actual_ids = [r['request_id'] for r in replayed]
    require(actual_ids == expected_ids[:len(actual_ids)], 'R3-replay-exact-ordered-prefix')
    for r in replayed:
        result(r['result'], 'R3-complete-replay-result')
        require(r['relation'] in ('Same', 'Different', 'Unproven'), 'R3-real-relation-required')
    outcome = v['result']
    if outcome.get('kind') == 'Ok':
        require(type(outcome.get('value')) is bool, 'R3-validation-boolean')
        if outcome['value']:
            require(actual_ids == expected_ids, 'R3-positive-complete-original-sequence')
            require(all(r['relation'] == 'Same' for r in replayed), 'R3-positive-all-Same')
        else:
            require(bool(replayed), 'R3-negative-has-real-witness')
            require(all(r['relation'] == 'Same' for r in replayed[:-1]), 'R3-negative-preceding-Same')
            require(replayed[-1]['relation'] in ('Different', 'Unproven'), 'R3-negative-last-nonSame')
    else:
        result(outcome, 'R3-mechanism-error')
        require(outcome.get('kind') == 'Err', 'R3-mechanism-error-not-success')
        require(v['mechanism_error'] == outcome, 'R3-exact-actual-mechanism-error')
        require(all(r['relation'] == 'Same' for r in replayed), 'R3-completed-prefix-before-mechanism-error')

def check(case,observed):
    require(observed['source_sha256']==case['source_sha256'],'source identity')
    require(observed['profile'] in case['execution']['profiles'],'profile')
    require(observed['order'] in case['execution']['orders'],'order')
    require(isinstance(observed['binary_sha256'],str) and len(observed['binary_sha256'])==64,'binary identity')
    require(isinstance(observed['compiled_configuration'],dict) and observed['compiled_configuration'],'compiled config')
    require(isinstance(observed['port_source_pins'],list) and observed['port_source_pins'],'real port source pins')
    events=observed['events'];require(isinstance(events,list) and events,'actual transcript nonempty')
    by=collections.defaultdict(list)
    known={'DiscoveryCompleted','AttemptStarted','BodyStarted','RequestRecorded','BodyCompleted','CandidateBuilt','ValidationCompleted','ContributionRetained','GenerationInvalidated','SinkDisposition','FunctionInvoked','CompilationReturned'}
    for e in events:
        require(isinstance(e,dict) and e.get('kind') in known,'known event variant')
        require(type(e['sequence']) is int and e['sequence']>=0,'sequence type')
        by[e['kind']].append(e)
        if 'attempt' in e:require(type(e['attempt']) is int and 0<=e['attempt']<=5,'attempt type')
        for k,v in e.items():
            if k.endswith('_id') and v is not None:identity(v,(e['kind'],k))
            if k.endswith('_ids'):
                require(isinstance(v,list),(e['kind'],k,'identity array'))
                for item in v:identity(item,(e['kind'],k))
        if 'result' in e:result(e['result'],e['kind'])
    require(by['DiscoveryCompleted'],'actual ordinary discovery required')
    require(by['BodyStarted'] and by['BodyCompleted'],'actual body execution required')
    require(by['SinkDisposition'],'actual sink dispositions required')
    start_by={e['execution_id']:e for e in by['BodyStarted']}
    finish_by={e['execution_id']:e for e in by['BodyCompleted']}
    require(len(start_by)==len(by['BodyStarted']) and set(start_by)==set(finish_by),'body start/completion bijection')
    require(len(finish_by)==len(by['BodyCompleted']),'no duplicate body completion')
    for x,y in [(start_by[k],finish_by[k]) for k in start_by]:
        require(x['sequence']<y['sequence'],'body boundary order')
        require(x['generation_id']==y['generation_id'],'body generation stable')
        require(type(y['selected']) is bool,'actual completed selection bit required')
        require(isinstance(x['resources'],dict),'resource tuple')
        require(set(x['resources'])=={'producer_id','capture_id','context_location','entry_chain_id','world_id','metrics_id','target','features'},'all causal resource fields')
        for k,v in x['resources'].items():
            if k not in ['target','features']:identity(v,('resource',k))
        require(x['resources']['target']=='paged','actual target')
        require(x['resources']['features']=={'default':[],'html':['html'],'a11y':['a11y-extras'],'html+a11y':['html','a11y-extras']}[observed['profile']],'actual profile features')
    legacy=case['typed_predicates'].get('legacy_error_unselected',False)
    if not legacy:
        require(by['AttemptStarted'] and by['CandidateBuilt'],'selected attempts and candidates required')
        require(by['RequestRecorded'] and by['ValidationCompleted'],'recorded requests and validation required')
    require(len(by['AttemptStarted'])==len(by['CandidateBuilt']),'each actual attempt has completed candidate')
    require(len({e['attempt'] for e in by['CandidateBuilt']})==len(by['CandidateBuilt']),'one candidate per attempt')
    requests={e['request_id']:e for e in by['RequestRecorded']}
    for r in requests.values():
        require(r['execution_id'] in start_by,'request belongs to actual body')
        require(start_by[r['execution_id']]['sequence']<r['sequence']<finish_by[r['execution_id']]['sequence'],'request during actual body')
    # The selected-current-generation set is a passive projection of actual
    # current pipeline records, not reconstructed by this checker.
    for c in by['CandidateBuilt']:
        require(isinstance(c['selected_generation_ids'],list) and c['selected_generation_ids'],'nonempty actual selected generation set')
        require(len(c['selected_generation_ids'])==len(set(c['selected_generation_ids'])),'unique selected generation set')
        require(set(c['selected_generation_ids'])<=set(c['current_generation_ids']),'selected records belong to current tree')
        validations=[v for v in by['ValidationCompleted'] if v['attempt']==c['attempt']]
        require(collections.Counter(v['generation_id'] for v in validations)==collections.Counter(c['selected_generation_ids']),'exact validation/current-selected-generation bijection')
        for v in validations:
            original=[x for x in finish_by.values() if x['generation_id']==v['generation_id'] and x['selected']]
            require(len(original)==1,'validation is tied to one original selected execution')
            expected_ids=[r['request_id'] for r in by['RequestRecorded'] if r['execution_id']==original[0]['execution_id']]
            require(expected_ids,'selected generation has real causal requests')
            check_validation(v, expected_ids)
            if original[0]['attempt']<c['attempt']:
                kept=[k for k in by['ContributionRetained'] if k['attempt']==c['attempt'] and k['generation_id']==v['generation_id']]
                require(len(kept)==1,'retained generation requires actual retention event')
    dispositions=collections.defaultdict(list)
    for e in by['SinkDisposition']:
        require(e['disposition'] in ['published','discarded'],'sink disposition variant')
        require(isinstance(e['diagnostics'],list),'complete sink diagnostic vector')
        dispositions[e['sink_id']].append(e)
    required_sinks={e['sink_id'] for e in by['BodyCompleted']+by['ValidationCompleted']}
    require(required_sinks<=set(dispositions),'all actual body and validation sinks have disposition')
    for sink in required_sinks:require(len(dispositions[sink])==1,'exactly one real final disposition per execution sink')
    if not legacy:
        selected_discovery=[e for e in by['DiscoveryCompleted'] if e['selected']]
        require(selected_discovery,'selected ordinary discovery has actual witness')
        for discovery in selected_discovery:
            require(discovery['sink_id'] in dispositions,'discovery-selected sink tracked')
            require(dispositions[discovery['sink_id']][0]['disposition']=='discarded','selected discovery discarded before seed')
    expected=case['typed_predicates']
    if 'retained_context_index' in expected:
        offset=case['context_coordinates'][expected['retained_context_index']]['byte_offset']
        originals=[e for e in by['BodyStarted'] if e['attempt']==1 and e['source_context_offset']==offset]
        require(len(originals)==1,'one actual stable A1 producer')
        retained=[e for e in by['ContributionRetained'] if e['generation_id']==originals[0]['generation_id']]
        require([e['attempt'] for e in retained]==[2,3,4,5],'explicit retention A2..A5')
    if 'replaced_child_context_index' in expected:
        offset=case['context_coordinates'][expected['replaced_child_context_index']]['byte_offset']
        children=[e for e in by['BodyStarted'] if e['attempt']>0 and e['source_context_offset']==offset]
        require(len(children)==5,'five actual child generations')
        for child in children[:-1]:
            removed=[e for e in by['GenerationInvalidated'] if child['generation_id'] in e['descendant_ids'] or child['generation_id']==e['generation_id']]
            require(len(removed)>=1,'old child has actual invalidation event')
    return prior.check(case,observed)

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--fixtures',required=True);p.add_argument('--observations',required=True);a=p.parse_args()
    fixtures=json.loads(Path(a.fixtures).read_text());obs=json.loads(Path(a.observations).read_text())['cases']
    lookup={c['id']:c for c in fixtures['cases']}
    expected={(c['id'],profile,order) for c in lookup.values() for profile in c['execution']['profiles'] for order in c['execution']['orders']}
    actual=[(x['case_id'],x['profile'],x['order']) for x in obs]
    require(len(actual)==len(set(actual)) and set(actual)==expected,'exact complete cases x profiles x orders, no duplicate or missing observation')
    for row in obs:print(json.dumps(check(lookup[row['case_id']],row),ensure_ascii=True))
