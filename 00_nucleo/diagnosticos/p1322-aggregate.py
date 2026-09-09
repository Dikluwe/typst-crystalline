"""Root descriptive aggregation; independent reviewer owns the verdict."""
from collections import Counter
import importlib.util
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('matrix',D/'p1322-matrix.py')
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
r=m.r

def read(name):
    return json.loads((D/('p1322-'+name+'.json')).read_text())

def exact_orders(prefix, field):
    phases={phase:read(prefix+'-'+phase) for phase in ['normal','repeat','reverse']}
    maps={phase:{(row['id'],row['profile']):m.stable_key(row) for row in data[field]} for phase,data in phases.items()}
    for phase,data in phases.items():
        assert len(maps[phase])==len(data[field]),'duplicate key'
    deltas={phase:[list(key) for key in sorted(set(maps['normal'])|set(rows)) if rows.get(key)!=maps['normal'].get(key)] for phase,rows in maps.items()}
    counts=dict(Counter(row['runtime_class'] for row in phases['normal'][field]))
    return dict(pairs=len(maps['normal']),counts=counts,deltas=deltas,stable=not any(deltas.values()),
        inputs={str(D/('p1322-'+prefix+'-'+phase+'.json')):r.sha(D/('p1322-'+prefix+'-'+phase+'.json')) for phase in phases})

def main():
    before=r.state();r.verify(before)
    principal=exact_orders('matrix','results')
    supplement=exact_orders('sentinels','rows')
    rows=read('matrix-normal')['results']
    per_profile={p:dict(Counter(x['runtime_class'] for x in rows if x['profile']==p)) for p in m.PROFILES}
    equal=sum(principal['counts'].get(k,0) for k in ['MATCH_VALUE','MATCH_DIAGNOSTIC'])
    paths={row['path'] for row in rows}
    all_equal=[path for path in paths if all(row['runtime_class'].startswith('MATCH_') for row in rows if row['path']==path)]
    historical=json.loads((D/'p1309-matrix-normal.json').read_text())
    old={(x['id'],x['profile']):x['runtime_class'] for x in historical['results']}
    transitions=[dict(id=x['id'],path=x['path'],profile=x['profile'],before=old[(x['id'],x['profile'])],after=x['runtime_class']) for x in rows if (x['id'],x['profile']) in old and old[(x['id'],x['profile'])]!=x['runtime_class']]
    r.save('aggregate',dict(at=r.now(),before=before,after=r.state(),manifest_sha256=r.sha(D/'p1322-manifest.json'),
        runner_sha256=r.sha(__file__),principal=principal,supplement=supplement,per_profile=per_profile,
        raw_equality=dict(numerator=equal,denominator=len(rows),formula='(MATCH_VALUE + MATCH_DIAGNOSTIC) / all principal cells',percent=100*equal/len(rows)),
        paths=dict(observed=len(paths),all_profiles_raw_equal=len(all_equal),not_all_profiles_raw_equal=len(paths)-len(all_equal),limits='Paths/probes are not complete features; negative historical controls are included.'),
        historical_transition_source=dict(path=str(D/'p1309-matrix-normal.json'),sha256=r.sha(D/'p1309-matrix-normal.json')),
        transitions=transitions,regression_candidates=[x for x in transitions if x['before'].startswith('MATCH_') and not x['after'].startswith('MATCH_')],
        limits='No adjusted-extension decision here; causal classifier and independent reviewer own semantic decisions. Supplement excluded from principal denominator.'))

if __name__=='__main__':main()
