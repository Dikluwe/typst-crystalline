#!/usr/bin/env python3
"""Post-candidate policy correction; all expected bytes come from pre-candidate evidence."""
import argparse, copy, importlib.util, json, re, time
from pathlib import Path
s=importlib.util.spec_from_file_location('ab',Path(__file__).with_name('p1310-ab-suite.py'))
ab=importlib.util.module_from_spec(s); s.loader.exec_module(ab)
PIN='7f110b464745b880c357a9676fa302995873df5e67c5127adf0229cd25c5c146'
BAD={'named-prefix','with-named-prefix','args-named-prefix'}
def affected(c): return c['id'].split('.')[-1] in BAD
def verify_pins(f):
    assert ab.sha(ab.D/'p1310-ab-suite.py')==f['suite_sha256']
    pin=json.loads((ab.D/'p1310-ab-l0-pin.json').read_text())
    data=(ab.ROOT/pin['path']).read_bytes()
    normalized,n=re.subn(rb'^Hash do C\xc3\xb3digo: [0-9a-f]{8}\n',b'',data,flags=re.M)
    assert n==1 and ab.digest(normalized)==pin['normalized_sha256']
    for k,v in f['fixtures'].items(): assert ab.sha(Path(f['fixture_dir'])/k)==v['sha256']
def main():
    p=argparse.ArgumentParser(); p.add_argument('phase',choices=['refine','focal','normal','repeat','reverse','stability']); p.add_argument('--candidate'); a=p.parse_args()
    tick=time.monotonic(); before=ab.state()
    if a.phase=='refine':
        old=json.loads((ab.D/'p1310-ab-frozen-r1.json').read_text()); verify_pins(old)
        measurement=json.loads((ab.D/'p1310-ab-freeze-r1-measurement.json').read_text())
        observed={(r['case'],r['profile']):r['observable'] for r in measurement['rows'] if r['side']=='baseline'}
        out=copy.deepcopy(old); changed=[]
        for c in out['cases']:
            if not affected(c): continue
            changed.append(c['id']); c['policy']='baseline'; c['required_message']=None
            c['policy_reason']='Pre-candidate L0 keeps named argument rejection and validation order out of the repair. R1 incorrectly required vanilla source cast to precede the existing named rejection.'
            for profile in ab.PROFILES:
                expected=observed[c['id'],profile]
                assert expected['kind']=='diagnostic' and expected['messages'][0].startswith('argumento nomeado inesperado')
                c['expected'][profile]=copy.deepcopy(expected)
        assert len(changed)==15
        out.update(schema='p1310-ab-policy-successor-r2',at=ab.now(),before=before,after=ab.state(),post_candidate_policy_correction=True,
          all_expectations_frozen_before_candidate=False,expected_literal_data_precedes_candidate=True,candidate_source_read=False,
          preceding_frozen_sha256=ab.sha(ab.D/'p1310-ab-frozen-r1.json'),r2_runner_sha256=ab.sha(__file__),baseline_red_cells=old['baseline_red_cells']-60,
          correction=dict(changed_cases=changed,changed_cells=60,expectation_source='p1310-ab-freeze-r1-measurement.json baseline rows',source_sha256=ab.sha(ab.D/'p1310-ab-freeze-r1-measurement.json'),candidate_output_used_to_author_expectations=False,
            cause='Test-author policy error: named-prefix cases are preservation controls under the original L0, not new vanilla ordering obligations.',
            authorization='Root/reviewer required correction to pre-existing L0 after R1 normal run began; candidate has not changed.',
            disclosure='R2 classification was authored after candidate binary and R1 results existed. It is not a completely pre-candidate frozen suite. R1 originals remain immutable.',
            prior_result='p1310-ab-normal.json retains 976 Preserved / 60 Violated / 0 Unknown under the erroneous R1 policy.',
            focal_requirement='Affected cases plus direct integer, valid Bytes and missing-argument controls, all four profiles, before full normal/repeat/reverse.',seconds=time.monotonic()-tick))
        ab.save('frozen-r2',out); print(json.dumps(dict(sha256=ab.sha(ab.D/'p1310-ab-frozen-r2.json'),changed_cases=len(changed),changed_cells=60)))
        return
    f=json.loads((ab.D/'p1310-ab-frozen-r2.json').read_text()); verify_pins(f); assert ab.sha(__file__)==f['r2_runner_sha256']
    if a.phase=='stability':
        runs={phase:json.loads((ab.D/('p1310-ab-r2-'+phase+'.json')).read_text()) for phase in ('normal','repeat','reverse')}
        keys={phase:{(r['case'],r['profile']):r for r in result['rows']} for phase,result in runs.items()}
        delta=[]
        for key in keys['normal']:
            observations=[keys[p][key] for p in runs]
            stable=all(all(o[k]==observations[0][k] for k in ('observable','exit','stdout_base64','stderr_base64','verdict','binary')) for o in observations[1:])
            if not stable: delta.append(key)
        counts={phase:result['counts'] for phase,result in runs.items()}
        result=dict(at=ab.now(),before=before,after=ab.state(),counts=counts,unstable=delta,cells_per_order=len(keys['normal']),frozen_sha256=ab.sha(ab.D/'p1310-ab-frozen-r2.json'),runner_sha256=ab.sha(__file__),inputs={p:ab.sha(ab.D/('p1310-ab-r2-'+p+'.json')) for p in runs},seconds=time.monotonic()-tick)
        ab.save('r2-stability',result); print(json.dumps(result)); return
    assert a.candidate and ab.sha(a.candidate)==PIN
    if a.phase!='focal':
        focal=json.loads((ab.D/'p1310-ab-r2-focal.json').read_text()); assert focal['counts']=={'Preserved':len(focal['rows'])}
    cases=f['cases']
    if a.phase=='focal': cases=[c for c in cases if affected(c) or c['id'].split('.',1)[-1] in ('type.integer','valid.bytes','preserve.missing')]
    candidate={'path':a.candidate,'sha256':PIN}
    rows=ab.run(cases,{'candidate':candidate},Path(f['fixture_dir']),a.phase=='reverse')
    lookup={c['id']:c for c in cases}; counts={}
    for r in rows:
        expected=lookup[r['case']]['expected'][r['profile']]; observed=r['observable']
        r['verdict']='Unknown' if observed['kind']=='Unknown' else 'Preserved' if observed==expected else 'Violated'
        counts[r['verdict']]=counts.get(r['verdict'],0)+1
    assert ab.sha(a.candidate)==PIN
    ab.save('r2-'+a.phase,dict(at=ab.now(),phase=a.phase,before=before,after=ab.state(),candidate=candidate,frozen_sha256=ab.sha(ab.D/'p1310-ab-frozen-r2.json'),runner_sha256=ab.sha(__file__),suite_sha256=f['suite_sha256'],seconds=time.monotonic()-tick,counts=counts,rows=rows))
    print(json.dumps(dict(counts=counts,failures=[r for r in rows if r['verdict']!='Preserved'])))
if __name__=='__main__': main()
