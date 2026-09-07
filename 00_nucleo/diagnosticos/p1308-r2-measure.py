#!/usr/bin/env python3
"""Authorized trace-only migration: all protected cases checked before editing."""
import concurrent.futures, copy, importlib.util, json, pathlib, subprocess

HERE = pathlib.Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('frozen_r4', HERE/'p1307-r4-oracle.py')
base = importlib.util.module_from_spec(spec); spec.loader.exec_module(base)
PINS = {
    'p1307-r6-oracle.json':'99d2a67984a468c8e86e20010ecc1bd76a364f1d1adebbb5b873fc341f7790a6',
    'p1307-r6-oracle.py':'62c4decfb6f07088ff2fb2aa1513ef1f62e2681edbb1431b42cd781eac56ef1a',
    'p1307-r4-oracle.py':'ef102f3a800475855b0cb21f40db666312cdb2f96fd9c867ea625b13c22b8e68',
    'p1308-verification-final.json':'06a57344f4e8039c1cd96a1df1aa2b2b2c159797e8080f1841844541e012ac60',
}
for name,pin in PINS.items(): assert base.sha(HERE/name)==pin
BINARY='/dev/shm/p1307-r4-target.8W1BEA/release/typst'
BINARY_SHA='09725fff8b4c472ee6b50a9ca6105d90268b2a0da8a22f1d8abd0f45e20c4c50'
assert base.sha(BINARY)==BINARY_SHA
oracle=base.read('p1307-r6-oracle.json')
receipt=base.read('p1308-verification-final.json')
deltas={d['case']:d for d in receipt['trace_deltas']}
assert len(deltas)==19
cases=[copy.deepcopy(c) for c in oracle['cases'] if c['id'] in deltas]
for c in cases:
    assert set(c['observations'])==set(deltas[c['id']]['profiles'])
    for cell in c['observations'].values():
        cell['future_expected']['stderr'] += deltas[c['id']]['exact_added_trace']

for family in ('grid','table'):
    for member in ('cell','header','footer'):
        expression=f'repr({family}.{member}())'
        message=f'{family}_{member}() exige '+('body como argumento posicional' if member=='cell' else 'pelo menos uma célula como argumento posicional')
        stderr=f'error: {message}\n\n  while calling `{member}` at <input-expression>:1:5\n    {family}.{member}()\n\n'
        expected=dict(kind='diagnostic',exit=1,stdout='',stderr=stderr,messages=[message],hints=[])
        cases.append(dict(id='wiring.'+family+'.'+member,expression=expression,route='eval',source_sha256=base.source_hash(expression),observations={p:dict(future_expected=copy.deepcopy(expected)) for p in base.PROFILES}))
expression='repr((grid.hline([x]),grid.hline(nope:1),grid.vline([x]),grid.vline(nope:1),table.hline([x]),table.hline(nope:1),table.vline([x]),table.vline(nope:1)))'
value='(\n  grid.hline,\n  grid.hline,\n  grid.vline,\n  grid.vline,\n  table.hline,\n  table.hline,\n  table.vline,\n  table.vline,\n)'
cases.append(dict(id='wiring.serializer-success',expression=expression,route='eval',source_sha256=base.source_hash(expression),observations={p:dict(future_expected=dict(kind='value',value=value,stderr='',exit=0)) for p in base.PROFILES}))

lookup={c['id']:c for c in cases}
jobs=[(order,c,p) for order,seq in [('normal',cases),('reverse',list(reversed(cases)))] for c in seq for p in c['observations']]
def run(job):
    order,c,p=job
    row=base.run(BINARY,'authorized-measurement',c,p)
    row['order']=order
    row['verdict']=base.classify(c['observations'][p]['future_expected'],row['observable'])
    return row
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool: rows=list(pool.map(run,jobs))
state={k:subprocess.check_output(cmd,text=True,cwd=base.ROOT) for k,cmd in {
    'head':['git','rev-parse','HEAD'],'status':['git','status','--short'],
    'diff_stat':['git','diff','HEAD','--stat'],'staged_stat':['git','diff','--cached','--stat']}.items()}
out=dict(schema='p1308-r2-authorized-trace-only-measurement-v1',at=base.now(),
    authorization='Faça o commite do que falta e autorizo',regime='executado sem atestação de isolamento técnico',
    productive_candidate_source_read=False,pins=PINS,script_sha256=base.sha(__file__),state=state,
    binary=BINARY,binary_sha256=BINARY_SHA,
    test_pre_sha256=base.sha(base.ROOT/'04_wiring/tests/p1293_contract.rs'),
    l0_pins={p:base.sha(base.ROOT/p) for p in ['00_nucleo/prompts/compiler/eval/tests.md','00_nucleo/prompts/wiring/tests/p1293_contract.md']},
    decision_basis='R6 expected plus exact independently verified trace suffix; not observed-to-expected copying',
    cases=cases,rows=rows,counts={v:sum(r['verdict']==v for r in rows) for v in ('Preserved','Violated','Unknown')})
base.save('p1308-r2-measure.json',out)
print(json.dumps(dict(runs=len(rows),counts=out['counts'],sha256=base.sha(HERE/'p1308-r2-measure.json'))))
