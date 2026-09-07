#!/usr/bin/env python3
"""Create, never overwrite, the authorized trace-only successor artifacts."""
import copy, datetime, difflib, hashlib, json, pathlib, subprocess
HERE=pathlib.Path(__file__).resolve().parent
ROOT=HERE.parents[1]
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
pins={
 'p1307-r6-oracle.json':'99d2a67984a468c8e86e20010ecc1bd76a364f1d1adebbb5b873fc341f7790a6',
 'p1307-r6-oracle.py':'62c4decfb6f07088ff2fb2aa1513ef1f62e2681edbb1431b42cd781eac56ef1a',
 'p1307-r4-oracle.py':'ef102f3a800475855b0cb21f40db666312cdb2f96fd9c867ea625b13c22b8e68',
 'p1308-verification-final.json':'06a57344f4e8039c1cd96a1df1aa2b2b2c159797e8080f1841844541e012ac60',
 'p1308-r2-measure.json':'b507c4a71345012a2e6b2ccd7ad3c7c89f130f1d66c32c853e5d0d28d4637079',
}
for name,pin in pins.items():assert sha(HERE/name)==pin
old=json.loads((HERE/'p1307-r6-oracle.json').read_text())
receipt=json.loads((HERE/'p1308-verification-final.json').read_text())
measurement=json.loads((HERE/'p1308-r2-measure.json').read_text())
assert measurement['counts']==dict(Preserved=208,Violated=0,Unknown=0)
new=copy.deepcopy(old)
changes={d['case']:d for d in receipt['trace_deltas']}
count=0
for before,after in zip(old['cases'],new['cases']):
    if before['id'] not in changes:continue
    delta=changes[before['id']]
    for profile,cell in after['observations'].items():
        assert profile in delta['profiles']
        cell['future_expected']['stderr']+=delta['exact_added_trace']
        count+=1
assert count==76 and len(changes)==19
new['schema']='p1308-r2-trace-only-public-oracle-v1'
new['at']=datetime.datetime.now(datetime.timezone.utc).isoformat()
new['executor']='/root/p1307_contract'
new['script_sha256']=sha(HERE/'p1308-r2-oracle.py')
new['trace_only_successor']=dict(
    authorization='Faça o commite do que falta e autorizo',
    predecessor='p1307-r6-oracle.json',protected_inputs=pins,
    productive_candidate_source_read=False,product_binary_observation_explicitly_authorized=True,
    policy='Only future_expected.stderr receives the exact authorized causal suffix; all cases, observations, fixtures, classifier and Unknown rules otherwise unchanged.',
    changed_cases=changes,changed_cells=count,
    l0_pins=measurement['l0_pins'],regime='executado sem atestação de isolamento técnico')

test=ROOT/'04_wiring/tests/p1293_contract.rs'
text=test.read_text()
assert sha(test)==measurement['test_pre_sha256'],'test predecessor changed before freeze'
start=text.index('fn p1293_d_preexisting_arity_and_serializer_transcript_is_frozen() {')
end=text.index('\n#[derive',start)
body=text[start:end]
for family in ('grid','table'):
    for member in ('cell','header','footer'):
        message=f'{family}_{member}() exige '+('body como argumento posicional' if member=='cell' else 'pelo menos uma célula como argumento posicional')
        expected=message+f'\n\n  while calling `{member}` at <input-expression>:1:5\n    {family}.{member}()'
        old_literal=json.dumps(message,ensure_ascii=False)
        new_literal=json.dumps(expected,ensure_ascii=False)
        assert body.count(old_literal)==1
        body=body.replace(old_literal,new_literal)
updated=text[:start]+body+text[end:]
patch=''.join(difflib.unified_diff(text.splitlines(True),updated.splitlines(True),fromfile='a/04_wiring/tests/p1293_contract.rs',tofile='b/04_wiring/tests/p1293_contract.rs'))

outputs={'p1308-r2-oracle.json':json.dumps(new,ensure_ascii=False,indent=2)+'\n','p1308-r2-tests.patch':patch}
for name in outputs:assert not (HERE/name).exists(),'Never overwrite a frozen successor'
request='*** Begin Patch\n'
for name,content in outputs.items():
    request+='*** Add File: '+str(HERE/name)+'\n'+''.join('+'+line+'\n' for line in content.splitlines())
request+='*** End Patch\n'
subprocess.run(['apply_patch'],input=request,text=True,check=True,cwd=ROOT,capture_output=True)
print(json.dumps({name:sha(HERE/name) for name in outputs}))
