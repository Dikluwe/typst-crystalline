"""Independent seal writer; requires completed C evidence and exact frozen inputs.

No compiler execution, input rewrite, or automatic retry. Candidate internal
definitions remain NotDue with zero C runtime credit.
"""
import collections, datetime, hashlib, importlib.util, json, pathlib, subprocess, sys
sys.dont_write_bytecode = True
ROOT = pathlib.Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'
def read(p): return json.loads(pathlib.Path(p).read_text())
def sha(p): return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def pin(p): return {'path':str(pathlib.Path(p).resolve()), 'sha256':sha(p)}
def now(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def write_new(name, obj):
    p = D/name
    assert not p.exists(), str(p)
    body = json.dumps(obj, ensure_ascii=True, indent=2)+'\n'
    patch = '*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+l+'\n' for l in body.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True,cwd=ROOT)
    return pin(p)
def main():
    assert sys.argv[1:] == ['--seal-after-complete-C']
    spec=importlib.util.spec_from_file_location('full_runner',D/'p1339-verifier-full-c-r2.py')
    run=importlib.util.module_from_spec(spec);spec.loader.exec_module(run)
    assert sha(D/'p1339-verifier-full-c-r2.py')=='7ef3cf8701afaf23592513fd4159298f600bbea2626b4c9e17917020803d3b5f'
    comp=read(D/'p1339-verifier-full-c-comparison-r1.json')
    assert comp['PASS_COMPLETE_C_COMPONENTS'] is True and not comp['failures']
    assert comp['full_preseal_runs_used']==1 and comp['mutation_score']==1.0
    manifest=read(D/'p1339-ab-batch2-manifest.json')
    contract=read(D/'p1339-contract-r3.json')
    plan=read(D/'p1339-ab-batch2-cli-plan.json')
    transport=run.module('frozen_transport_seal','p1339-ab-executor-v4.py')
    run.validate_inputs(plan,transport)
    state=run.state()
    public=read(run.checked(comp['raw_public']))
    mutants=read(run.checked(comp['raw_mutants']))
    acceptance=read(run.checked(comp['focal_acceptance']))
    focal=read(run.checked(acceptance['raw']));derived=read(run.checked(acceptance['derived']))
    predicate=run.module('frozen_predicate_seal','p1339-ab-batch2-public-predicate.py')
    assert predicate.check(public,'calibration',[c['id'] for c in plan['cases']],focal,derived)==comp['public_results']
    neg, failures=run.judge_mutants(mutants)
    assert neg==comp['mutant_comparisons'] and not failures
    groups=collections.defaultdict(dict)
    for row in public['rows']:
        key=(row['case_id'],row['product'],row['profile'])
        assert row['order'] not in groups[key]
        groups[key][row['order']]=tuple(row.get(k) for k in ['execution','exit','stdout','stderr','unknown_reason'])
    for key, orders in groups.items():
        assert set(orders)=={'normal','repeat','reverse'},key
        assert len(set(orders.values()))==1,('nondeterministic',key)
    hist={(v['stage'],v['id']) for v in contract['historical_case_index']}
    hist_by_id={v['id']:v['stage'] for v in contract['historical_case_index']}
    assert len(hist_by_id)==539
    actual_hist={(v.get('historical_stage',hist_by_id[v['historical_case_id']]),v['historical_case_id'])
                 for v in plan['cases'] if v.get('historical_case_id') in hist_by_id}
    assert len(hist)==539 and actual_hist==hist
    positive=read(D/'p1339-ab-batch2-positive-oracles.json')
    opaque=read(D/'p1339-ab-batch2-opaque-oracles.json')
    public_defs=positive['oracles']+opaque['oracles']
    future=positive['candidate_internal_oracles']+opaque['candidate_internal_oracles']
    # Four explicitly conditional Angle controls live in the frozen plan and
    # opaque count/policy, not in the 719 positive + one required opaque list.
    conditional=[v for v in plan['cases'] if v['reference_policy']=='conditional_unactivated']
    assert len(public_defs)==720 and len(conditional)==4
    assert {v['id'] for v in public_defs+conditional}=={v['id'] for v in plan['cases']}
    assert len(future)==len({v['id'] for v in future})==119
    assert all(v['execution_phases']==['F'] and v['phase']['credit']==0 for v in future)
    pins={}
    def add(path,h=None):
        p=pathlib.Path(path)
        if not p.is_absolute():p=ROOT/p
        p=p.resolve()
        # Candidate-owned source is recorded in baseline provenance, not made
        # immutable as an oracle. L0 has the separate explicit metadata policy.
        if any(str(p).startswith(str(ROOT/x)+'/') for x in ['01_core','02_shell','03_infra','04_wiring','tests','00_nucleo/prompts']):return
        if '/00_nucleo/materialization/' in str(p):
            assert p==ROOT/'00_nucleo/materialization/typst-passo-1339.md'
        assert '/00_nucleo/context/' not in str(p)
        actual=sha(p)
        if h is not None:assert actual==h,str(p)
        if str(p) in pins:assert pins[str(p)]==actual
        pins[str(p)]=actual
    for ref in manifest['artifacts']+manifest['authorities']:add(ref['path'],ref['sha256'])
    for p,h in contract['input_sha256'].items():add(p,h)
    extra=['p1339-contract.json','p1339-contract-r2.json','p1339-contract-r3.json',
      'p1339-contract-design-r3.md','p1339-contract-author-receipt-r3.json',
      'p1339-ab-batch2-manifest.json','p1339-ab-batch1-manifest-r2.json',
      'p1339-verifier-batch2-manifest-acceptance-r1.json',
      'p1339-verifier-batch2-focal-acceptance-r1.json',
      'p1339-ab-batch2-focal-runs.json','p1339-ab-batch2-focal-evaluation.json',
      'p1339-ab-batch2-causal-derived-expectations.json',
      'p1339-verifier-full-c-start-r1.json','p1339-verifier-full-c-public-r1.json',
      'p1339-verifier-full-c-mutants-r1.json','p1339-verifier-full-c-comparison-r1.json',
      'p1339-verifier-full-c-r1.py','p1339-verifier-full-c-r2.py',
      'p1339-verifier-mutant-focal-r1.json','p1339-verifier-mutant-focal-review-r1.md',
      'p1339-verifier-seal-r1.py']
    for n in extra:add(D/n)
    add(ROOT/'00_nucleo/materialization/typst-passo-1339.md','817c3a1476897fb0a847c90183e9a9fe690994f60023126997c8022c4e8b86a9')
    def refs(obj):
        if isinstance(obj,dict):
            if isinstance(obj.get('sha256'),str):
                p=obj.get('path',obj.get('artifact'))
                if isinstance(p,str):add(p,obj['sha256'])
            for value in obj.values():refs(value)
        elif isinstance(obj,list):
            for value in obj:refs(value)
    for obj in [positive,opaque,manifest,read(D/'p1339-mutation-registry.json')]:refs(obj)
    registry=read(D/'p1339-mutation-registry.json')
    for host in [registry['multiplex'],registry['architecture_witness']['control'],registry['architecture_witness']['negative']]:
        for p,h in host['source_sha256'].items():add(pathlib.Path(host['source_root'])/p,h)
    for ref in manifest['l0']:
        assert sha(ref['path'])==ref['raw_sha256']
    counts=dict(collections.Counter(v['classification'] for v in comp['public_results']))
    unknowns=[v for v in comp['public_results'] if v['classification']=='Unknown']
    assert all(v['reason'] in ['conditional_angle_receiver_unactivated','declared_observation_budget_opacity'] for v in unknowns)
    negative_summary=[{'id':f'M{i:02d}','valid':True,'rejected':True,'cells':12,
        'reason':next(v['reason'] for v in neg if v['family']==f'M{i:02d}')} for i in range(1,21)]
    authority=pin(D/'p1339-authority-manifest-r2.json')
    disc={'schema':'p1339-independent-discrimination-v1','utc':now(),'verifier':'/root/p1311_review',
      'regime':'executado sem atestacao de isolamento','authority_manifest':authority,
      'contract':pin(D/'p1339-contract-r3.json'),'verdict':'PASS_DISCRIMINATION_C_SCOPED',
      'comparison':pin(D/'p1339-verifier-full-c-comparison-r1.json'),'raw_public':comp['raw_public'],
      'raw_mutants':comp['raw_mutants'],'start':comp['start'],'state_at_decision':state,
      'processes':{'public':17106,'negative_and_controls':972,'total':18078},
      'public_classifications':counts,'explicit_unknowns':unknowns,'unknown_preservation_credit':0,
      'determinism':{'groups':len(groups),'orders':['normal','repeat','reverse'],'exact_required_channels':True,'pdf_bytes_excluded_as_non_oracle':True},
      'negative_families':negative_summary,'mutation_score':{'valid':20,'rejected':20,'score':1.0},
      'M12':{'basis':pin(D/'p1339-verifier-mutant-focal-review-r1.md'),'runtime_equal_all_12_cells':True,'actual_compiled_source_graph_rejection':True},
      'M20':{'raw_execution':'Unknown','signal':6,'gate':'mandatory_unknown','preservation_credit':0},
      'cost':{k:{f:raw[f] for f in ['start_utc','end_utc','wall_seconds','processes']}|{'child_seconds_sum':sum(r['seconds'] for r in raw['rows'])} for k,raw in [('public',public),('mutants',mutants)]},
      'budget':{'contract_revisions_used':3,'additional_focal_batches_used':2,'additional_focal_batches_limit':2,'full_preseal_runs_used':1,'full_preseal_runs_limit':2,'automatic_retries':0},
      'future':{'definitions':119,'required_F_raw_cells':1428,'execution_C':0,'credit_C':0,'state':'NOT_EXECUTED_PRESEAL'},
      'limits':['C calibration only; no candidate preservation or general equivalence.', 'M12 rejection requires actual graph evidence, not runtime difference.', 'M20 is a deliberately invalid negative rejected by mandatory_unknown; no Unknown success default.']}
    disc_pin=write_new('p1339-discrimination-runs.json',disc)
    add(disc_pin['path'],disc_pin['sha256'])
    seal={'schema':'p1339-independent-seal-v1','utc':now(),'verifier':'/root/p1311_review',
      'regime':'executado sem atestacao de isolamento','verdict':'SEALED_R3_C_SCOPED',
      'authority_manifest':authority,'contract':pin(D/'p1339-contract-r3.json'),
      'predecessors':[pin(D/'p1339-contract.json'),pin(D/'p1339-contract-r2.json')],
      'residual_r2_labels':'The two inherited textual r2 labels mean this exact R3 successor under its specific phase/authority clauses; R2 is not sealed and no file was silently edited.',
      'discrimination':disc_pin,'mutation_score':1.0,'l0':manifest['l0'],
      'l0_hash_policy':contract['l0_hash_policy'],'binaries':manifest['binaries'],
      'immutable_inputs':[{'path':p,'sha256':h} for p,h in sorted(pins.items())],
      'active_oracles':manifest['active_public_wrappers'],'active_public_predicate':manifest['active_public_predicate'],
      'active_show_effect_table':manifest['active_show_effect_table'],
      'active_causal_derived':acceptance['derived'],'active_execution_manifest':pin(D/'p1339-ab-batch2-manifest.json'),
      'phase_C':{'existing_public_cases':724,'historical_IDs':539,'routes':contract['routes'],'scopeout18':contract['scopeout18'],'supplements':contract['supplement_sets'],'processes':18078,'future_runtime_credit':0},
      'phase_F_NotDue_ledger':[{'id':v['id'],'obligation_ids':v['obligation_ids'],'matrix_ids':v['matrix_ids'],'definition':v['source_ref_or_literal_with_sha256'],'execution_phases':['F'],'state':'NOT_EXECUTED_PRESEAL','execution_C':0,'credit_C':0} for v in future],
      'final_coverage_map':manifest['future_definition_slice']['coverage_map'],
      'final_internal_raw_cells':1428,'final_requirements':contract['gates']['final'],
      'implementation_gate':'Seal permits D only. Require independent actual baseline RED on ten discovery probes with positive controls; absent future private API build failure is not RED. Product implementation remains forbidden until valid D receipt.',
      'mechanical_binding_policy':contract['candidate_test_binding'],
      'budget':disc['budget'],'state_at_seal':state,
      'invalidation':'Any protected semantic input change invalidates the chain from its affected phase. Only the exact preauthorized single L0 Hash do Codigo metadata-line update is excluded under the pinned normative policy. Product changes are candidate output, never oracle changes.',
      'no_claims':['No technical isolation attestation; procedural capabilities only.', 'Prior P1311 context is unrelated and gives no P1339 evidence credit.', 'No general equivalence, candidate GREEN, final certification, or F internal runtime claimed.']}
    seal_pin=write_new('p1339-seal.json',seal)
    print(json.dumps({'discrimination':disc_pin,'seal':seal_pin,'immutable_inputs':len(pins),'public_counts':counts,'determinism_groups':len(groups)}),flush=True)
if __name__=='__main__':main()
