#!/usr/bin/env python3
"""One focal pre-candidate refinement for Symbol boundary and named-prefix origin."""
import copy, importlib.util, json, time
from pathlib import Path
s=importlib.util.spec_from_file_location('ab',Path(__file__).with_name('p1310-ab-suite.py'))
ab=importlib.util.module_from_spec(s); s.loader.exec_module(ab)
tick=time.monotonic(); before=ab.state()
old=json.loads((ab.D/'p1310-ab-frozen.json').read_text())
raw=json.loads((ab.D/'p1310-ab-freeze-measurement.json').read_text())
assert old['blockers'] and all(x[0].endswith('.type.symbol') for x in old['blockers'])
assert all(r['observable']['kind']!='Unknown' for r in raw['rows'])
cases=copy.deepcopy(old['cases']); symbols=[c for c in cases if c['id'].endswith('.type.symbol')]
for c in symbols:
    c['policy']='L0-Symbol-rejected-domain'; c['required_message']='expected path, string, or bytes, found symbol'
    c['policy_reason']='Explicit L0 domain remains Path/Str/Bytes. Vanilla coerces Symbol to Str; retaining crystalline rejection is not vanilla parity. Only the new diagnostic grammar and existing value origin are required.'
    for profile,o in c['expected'].items():
        assert o['messages'][0].startswith('file not found (searched at ')
        # Vanilla measured the same expression and value-span; retain literal
        # display/underline and substitute solely the independently specified
        # diagnostic message. No candidate output or Rust is consulted.
        o['stderr']=o['stderr'].replace('error: '+o['messages'][0]+'\n','error: '+c['required_message']+'\n',1)
        o['messages']=[c['required_message']]
extra=[]
for f in ('cbor','json','toml','xml','yaml'):
    for name,expr in {
        'named-prefix':f+'(nope: true, 42)',
        'with-named-prefix':f+'.with(nope:true,42)()',
        'args-named-prefix':f+'(..arguments(nope:true,42))',
        'first-invalid':f+'(42,true)',
    }.items():
        extra.append(dict(id=f+'.route.'+name,expression=expr,policy='vanilla',kind='diagnostic',required_message='expected path, string, or bytes, found integer',source_sha256=ab.digest(expr.encode())))
for b in ab.BINS.values(): assert ab.sha(b['path'])==b['sha256']
rows=ab.run(extra,ab.BINS,Path(old['fixture_dir']))
lookup={(r['case'],r['profile'],r['side']):r['observable'] for r in rows}
for c in extra:
    c['expected']={p:lookup[c['id'],p,'vanilla'] for p in ab.PROFILES}
    assert all(o['kind']=='diagnostic' and o['messages']==[c['required_message']] for o in c['expected'].values())
cases+=extra
allrows=raw['rows']+rows
lookup={(r['case'],r['profile'],r['side']):r['observable'] for r in allrows}
out={**old,'schema':'p1310-ab-frozen-r1','at':ab.now(),'before':before,'after':ab.state(),'cases':cases,'blockers':[],
     'L0_sha256':ab.sha(ab.ROOT/old['L0_path']),'suite_sha256':ab.sha(ab.D/'p1310-ab-suite.py'),
     'baseline_red_cells':sum(lookup[c['id'],p,'baseline']!=c['expected'][p] for c in cases for p in ab.PROFILES),
     'predecessor_sha256':ab.sha(ab.D/'p1310-ab-frozen.json'),'predecessor_measurement_sha256':ab.sha(ab.D/'p1310-ab-freeze-measurement.json'),
     'revision':{'number':1,'cause':'Symbol coercion in vanilla exceeds retained crystalline input domain','candidate_read':False,'full_corpus_repeated':False,'focal_processes':len(rows),'unknown_before':0,'unknown_after':0,'invalid_expectation_cells_before':20,'invalid_expectation_cells_after':0,'added_named_prefix_and_first_positional_cells':len(extra)*4,'seconds':time.monotonic()-tick},
     'focal_author_script_sha256':ab.sha(__file__)}
ab.save('freeze-r1-measurement',dict(before=before,after=ab.state(),rows=rows,seconds=time.monotonic()-tick))
ab.save('frozen-r1',out)
print(json.dumps(dict(cases=len(cases),cells=len(cases)*4,baseline_red_cells=out['baseline_red_cells'],blockers=[],sha256=ab.sha(ab.D/'p1310-ab-frozen-r1.json'))))
