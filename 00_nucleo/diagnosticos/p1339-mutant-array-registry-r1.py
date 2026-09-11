"""Record compiled isolated modes and prospective witnesses; never execute them."""
import datetime, hashlib, json, pathlib
ROOT=pathlib.Path(__file__).resolve().parents[2]
D=ROOT/'00_nucleo/diagnosticos'
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def pin(name):
    p=D/name
    return {'path':str(p),'sha256':sha(p)}
build=json.loads((D/'p1339-mutant-array-build-r1.json').read_text())
assert sha(D/'p1339-mutant-array-build-r1.json')=='b972819d26f9f1465484dc0f3cfece79ea4e5a11d8dedfa1f84345594906bad6'
inputs=json.loads((D/'p1339-contract-array-inputs-r2.json').read_text())
assert sha(D/'p1339-contract-array-inputs-r2.json')=='8a0c77118c80d220b58dd8070fe0da2399353c4ae0f46610b245224b389bb3a0'
assert build['exit']==0
assert sha(build['binary']['path'])==build['binary']['sha256']
for name in ['typst-core','typst-shell','typst-infra','typst-wiring']:
    assert f'Compiling {name} ' in build['stderr'], 'workspace rebuild not observed'
actual=[
    'Real Array route returns its absent-constructor error for Bytes before conversion.',
    'Real Array route returns the original Value::Bytes rather than Value::Array.',
    'Each byte is converted through i8 before i64, making high unsigned octets negative.',
    'The octet buffer is reversed before actual Value::Array construction.',
    'pop removes the last octet before constructing actual Value::Array.',
    'A 256-entry seen set removes repeated octets while retaining first occurrence order.',
    'UTF8 decoding with chars constructs scalar codepoints rather than octets.',
    'Skip the remaining-argument rejection for both positional and named surplus.',
    'Always use Args.span even when a known individual surplus occurrence span exists.',
    'NonBytes route accepts Array and Version, and also other/missing/named-only forms, as actual Array values.',
    'Search any later positional Bytes and remove earlier positional values before conversion.',
    'After resolving an actual Type::Array callee, evaluate the same callee AST again before ordinary arguments.',
]
families=[]
for mode,(definition,description) in enumerate(zip(inputs['discrimination_inputs'],actual),1):
    families.append({
        'id':definition['id'],'mode':mode,'expected_cause_from_author':definition['cause'],
        'actual_mutation':description,'prospective_witness_ids':definition['witness_ids'],
        'environment_delta':{'P1339_ARRAY_MUTANT':str(mode)},
        'compiled_binary':build['binary'],
        'classification':'CompiledPendingIndependentEligibilityAndExecution',
        'runtime_evidence':None,'rejection_verdict':None,
    })
record={
    'schema':'p1339-array-compiled-negative-registry-v1',
    'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'author':'/root/p1336_tests','regime':'executado sem atestacao de isolamento',
    'authority_manifest_sha256':'842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b',
    'status':'COMPILED_NOT_DISCRIMINATED_NOT_SEALED',
    'input_pins':[pin(x) for x in ['p1339-contract-array-proposal-r1.md','p1339-contract-array-inputs-r2.json','p1339-contract-array-supplement-r2.json','p1339-mutant-array-budget-r1.json','p1339-mutant-array-build-r1.py','p1339-mutant-array-build-r1.json','p1339-mutant-array-multiplex-r1.patch','p1339-mutation-registry.json']],
    'l0_raw_pins':{str(p):sha(ROOT/p) for p in ['00_nucleo/prompts/compiler/stdlib/primitives-constructors/array.md','00_nucleo/prompts/compiler/stdlib/primitives-constructors.md','00_nucleo/prompts/compiler/stdlib/_comum.md','00_nucleo/prompts/compiler/eval/call_dispatch.md']},
    'baseline_source_commit':build['baseline_commit'],'baseline_archive':build['archive'],
    'source_and_config_inventory':'Complete file pins in build receipt; patch has exactly isolated call_dispatch instrumentation and new isolated helper.',
    'binary':build['binary'],
    'control':{'mode':0,'environment_delta':{'P1339_ARRAY_MUTANT':'0'},'implementation':'Real restricted conversion path on baseline source, actual Args/Values/diagnostics; no fixture/source-name lookup, process wrapper or output substitution. Never adopted as productive solution.','eligibility':'Pending full28 default control against frozen independent oracles.'},
    'isolation':{'source_root':build['source_root'],'target':str(pathlib.Path(build['source_root']).parent/'target'),'cache':build['cache'],'product_writes':0,'candidate_source_read':False,'attestation':False,'limits':'Environment-driven selector exists only in isolated apparatus. Core impurity and helper placement here are not proposed or certified productive architecture.'},
    'modes':families,
    'exclusivity':'One integer process configuration; mode0 takes no negative branch. Modes1..12 select only their defined behavior. Invalid values panic when an actual Array route resolves, giving abnormal raw transport rather than invented diagnostics.',
    'focal_proposal_not_GO':{'profile':'default','order':'normal','control_cases':len(inputs['cases']),'negative_witness_processes':sum(len(f['prospective_witness_ids']) for f in families),'invalid_mode_processes':1,'policy':'No execution before oracle freeze plus independent focal GO. At least one real distinguishing witnessed output required for every independently eligible family; unexercised branch or unsupported input is not rejection.'},
    'cost':{'builds':1,'build_exit':0,'cargo_reported_seconds':55.22,'source_start':build['start'],'source_end':build['end'],'semantic_processes':0,'new_focal_cycles':0,'old20_registry':'Preserved byte-identical as ancestry pin; denominator not diluted.'},
    'limits':['Original70 float-to-bytes cases are outside this old-baseline direct Array apparatus and remain mandatory on actual candidate F.','No mutation score or positive-control pass inferred from compilation.','Source/control algorithms and future runtime outputs remain separate from independent expectation/verdict authorship.'],
}
out=D/'p1339-mutant-array-registry-r1.json'
with out.open('x') as f:json.dump(record,f,ensure_ascii=False,indent=2);f.write('\n')
print(json.dumps({'registry':pin(out.name),'focal_proposal':record['focal_proposal_not_GO']}))
