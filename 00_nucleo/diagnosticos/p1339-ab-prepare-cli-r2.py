"""Generate independent P1339 CLI inputs from frozen contract and measurements.

No candidate source or output is consumed. New outputs are immutable and are
written through apply_patch. New expectations are supplied by frozen reference
measurements; this prepares inputs, mappings and reference selection only.
"""
import hashlib
import json
from pathlib import Path
import subprocess

D=Path('/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos')
CONTRACT=D/'p1339-contract-r3.json'
CSHA='c0cd1826679ddaf77e937a677839c5cbe64fdae6ec71bfa34e635d584d9c3e17'
MSHA='842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b'
inputs={};cases=[]
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def read(name):
    p=D/name;inputs[str(p)]=sha(p);return json.loads(p.read_text())
def write(p,s):
    assert p.parent==D and p.name.startswith('p1339-ab-'),str(p)
    if p.exists():
        assert p.read_bytes()==s.encode(),('Immutable artifact differs',str(p))
        return
    assert s.endswith('\n')
    patch='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+l+'\n' for l in s[:-1].split('\n'))+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)
def fixture(cid,source):
    p=D/('p1339-ab-cli-'+cid+'.typ');source=source.rstrip('\n')+'\n';write(p,source)
    return dict(source_path=str(p),source_sha256=sha(p))
def add(cid,mode,policy,obligations,**kw):
    assert cid not in {c['id'] for c in cases}
    cases.append(dict(id=cid,mode=mode,reference_policy=policy,obligation_ids=obligations,**kw))
def source_probe(cid,src,policy,obligations,ancestor,reference_row=None):
    """Original named compile plus either value assertions or value dump.

    Baseline value dump is a separate expected-error preservation control: it
    exposes the original argument, never substitutes a vanilla value. Pure
    legacy ancestors retain an unmodified compile and default-query control.
    """
    metadata_calls=src.count('metadata(')
    if policy=='baseline':
        f=fixture(cid+'-original',src)
        add(cid+'-original','compile',policy,obligations,adapter='compile_witness',ancestor=ancestor,**f)
        if ancestor.get('historical_mode')=='query':
            add(cid+'-legacy-query','query',policy,obligations,adapter='query_values',profiles=['default'],ancestor=ancestor,**f)
        if metadata_calls==1:
            prefix='#let p1339-original-metadata = metadata\n#let metadata(value) = panic("P1339_BASELINE_VALUE:" + json.encode(value, pretty: false))\n'
            add(cid+'-value-dump','compile',policy,obligations,adapter='compile_witness',ancestor=ancestor,transformation=dict(kind='separate baseline observation control, not positive parity bridge',prefix=prefix),**fixture(cid+'-value-dump',prefix+src))
        return
    assert reference_row is not None
    exit_code=reference_row.get('exit',reference_row.get('exit_code'))
    if exit_code!=0:
        add(cid+'-error','compile',policy,obligations,adapter='causal_successor',ancestor=ancestor,transformation={'kind':'named compile of original error source; original read graph unchanged'},**fixture(cid+'-error',src))
        return
    assert metadata_calls==1,(cid,'needs explicit metadata adapter',metadata_calls)
    values=json.loads(reference_row['stdout']);assert len(values)==1,(cid,values)
    expected=json.dumps(values[0],ensure_ascii=False,separators=(',',':'))
    for relation,op in [('equal','=='),('different','!=')]:
        prefix='#let p1339-original-metadata = metadata\n#let metadata(value) = { assert(json.encode(value, pretty: false) '+op+' '+json.dumps(expected,ensure_ascii=False)+'); p1339-original-metadata(value) }\n'
        add(cid+'-'+relation,'compile',policy,obligations,adapter='causal_successor',ancestor=ancestor,assertion=relation,expected_metadata_json=expected,calibration_expected_exit=0 if relation=='equal' else 1,transformation=dict(kind='metadata assertion transport; no new context/query/demand',prefix=prefix,complementary_error_control=relation=='different'),**fixture(cid+'-'+relation,prefix+src))

contract=read(CONTRACT.name);assert inputs[str(CONTRACT)]==CSHA
catalog=read('p1339-full-catalog.json')['cases'];index={(c['stage'],c['id']):c for c in contract['historical_case_index']}
for entry in catalog:
    c=entry['case'];rule=index[(entry['stage'],c['id'])]
    if entry['stage']=='show-final':continue
    stage='final' if entry['stage']=='final' else 'boundaries'
    refs={side:'p1339-full-'+stage+'-'+('vanilla' if side=='vanilla' else 'crystalline-before')+'-runs.json' for side in ('vanilla','baseline')}
    for fn in refs.values():
        if str(D/fn) not in inputs:read(fn)
    add('historical-'+c['id'],'eval',rule['target'],[rule['obligation']],expression=c['expression'],adapter='eval_exact',historical_case_id=c['id'],historical_stage=entry['stage'],mandatory=rule['mandatory'],reference=dict(kind='historical_rows',by_product=refs,case_id=c['id']),raw_input_provenance=dict(catalog_sha256=inputs[str(D/'p1339-full-catalog.json')],case_id=c['id']))
show=read('p1339-show-witness-final-manifest.json')
for c in show['cases']:
    assert sha(c['path'])==c['sha256']
    add('show-'+c['id'],'compile','baseline' if c['kind']=='control' else 'vanilla',['W01'],adapter='compile_witness',source_path=c['path'],source_sha256=c['sha256'],historical_case_id=c.get('prior_id'),show_effect=c['expected_effect'],show_marker=c.get('marker'),reference=dict(kind='fresh'),ancestor=dict(path=str(D/'p1339-show-witness-final-manifest.json'),case_id=c['id']))

mixed={}
for rule in contract['preserved_exceptions']['mixed_ancestor_policies']:
    for cid in rule['case_ids']:mixed[(Path(rule['artifact']).name,cid)]=rule['target']
for group in contract['supplement_sets']:
    name=Path(group['artifact']).name;d=read(name);rows=d.get('rows',d.get('runs',[]));raw=d.get('cases')
    if raw is None:
        raw=[dict(id=r['id'],expression=r['argv'][2]) for r in rows if r.get('order','normal')=='normal']
    elif isinstance(raw,dict):raw=[dict(id=k,source=v) for k,v in raw.items()]
    for c in raw:
        cid=c['id'];matches=[(i,r) for i,r in enumerate(rows) if r.get('id',r.get('case'))==cid and r.get('binary','vanilla') in ('vanilla','/usr/local/bin/typst')]
        assert matches,(name,cid);ri,row=matches[0]
        command=c.get('command',c.get('argv',row.get('command',row.get('argv',[]))));mode='eval' if (command and command[1]=='eval') or c.get('expression') and not command else 'compile' if group['id']=='context-dependency' else 'query'
        policy='baseline' if mixed.get((name,cid))=='baseline_whole_ancestor' or cid in ('query_heading_bare','counter_heading_bare') else 'vanilla'
        source=c.get('source',c.get('stdin'));source=source.get('source') if isinstance(source,dict) else source
        ancestor=dict(path=str(D/name),sha256=inputs[str(D/name)],case_id=cid,row_index=ri,historical_mode=mode,causal_policy=mixed.get((name,cid),'authorized filtered obligation'))
        fullid=group['id']+'-'+cid
        if mode=='eval':add(fullid,'eval',policy,group['obligation'].split(','),expression=c.get('expression') or command[2],adapter='eval_exact',ancestor=ancestor,reference=dict(kind='fresh'))
        elif mode=='compile':add(fullid,'compile',policy,group['obligation'].split(','),adapter='compile_witness',ancestor=ancestor,**fixture(fullid,source))
        else:source_probe(fullid,source,policy,group['obligation'].split(','),ancestor,row)

derived=read('p1339-ab-w02-derived-inputs-r1.json');derived_runs=read('p1339-ab-w02-derived-probe-r1-runs.json')
for c in derived['cases']:
    row=next(r for r in derived_runs['rows'] if r['id']==c['id'] and r['product']=='vanilla')
    assert sha(c['source_path'])==c['source_sha256']
    ancestor=dict(path=str(D/'p1339-ab-w02-derived-inputs-r1.json'),sha256=inputs[str(D/'p1339-ab-w02-derived-inputs-r1.json')],case_id=c['id'],derivation='New semantic source, not same-graph ancestor bridge',historical_mode='query')
    source_probe(c['id'],Path(c['source_path']).read_text(),'vanilla',['W02'],ancestor,row)

# Independent outside-scope producer and ordinary-error controls.
for cid,expr in [('producer-explicit-delta','repr(strong(delta: 100)[custom])'),('producer-raw-default','repr((strong[inner].fields(), emph[inner].fields()))'),('producer-text-styles','repr((text(weight: "bold")[weight-only], text(style: "italic")[style-only], text(weight: "bold")[strong[semantic]]))')]:
    add(cid,'eval','baseline',['W02','W10'],expression=expr,adapter='eval_exact',reference=dict(kind='fresh'),independent_control=True)
for cid,src in [('producer-set-delta','#set strong(delta: 100)\n#strong[style-default]\n'),('ordinary-unrelated-panic','#context panic("unrelated-error")\n')]:
    add(cid,'compile','baseline',['W05','W10'],adapter='compile_witness',reference=dict(kind='fresh'),independent_control=True,**fixture(cid,src))
scope_catalog=read('p1335-probe-catalog.json');scope_matrix=read('p1335-matrix-normal.json')
for route in contract['scopeout18']:
    c=next(p for p in scope_catalog['probes'] if p['path']==route)
    add('scopeout-'+route,'eval','baseline',['scopeout18'],expression=c['expression'],adapter='eval_exact',reference=dict(kind='scopeout_historical_matrix',artifact='p1335-matrix-normal.json',case_id=c['id']),scopeout_route=route)

plan=dict(schema='p1339-ab-cli-plan-v1',author='/root/p1319_tests',authority_manifest_sha256=MSHA,contract_path=str(CONTRACT),contract_sha256=CSHA,l0_freeze_sha256='397c136fc8710d44b2f7537193fe5b9c44ab89d40a99bf6b994296e05e7c4d04',input_sha256=inputs,cases=cases,regime='executado sem atestação de isolamento',policy='Plan and reference selection are independent; fresh required observations and all expectations must be frozen before seal. Conditional cases receive no success credit. No candidate read.',orders=['normal','repeat','reverse'])
write(D/'p1339-ab-cli-plan-r2.json',json.dumps(plan,ensure_ascii=True,indent=2)+'\n')
print('Prepared',len(cases),'CLI cases; no product run or canonical RED.')
