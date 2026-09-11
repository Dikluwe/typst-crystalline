"""Deterministic pre-candidate expected derivation from the pinned focal controls.

No product execution, output normalization or choice based on candidate data.
"""
import argparse, hashlib, json, pathlib, re, subprocess

D=pathlib.Path(__file__).resolve().parent
PLAN=D/'p1339-ab-batch1-cli-plan.json'
PLAN_SHA='55a46cf8390ea7914aba115afaf17e9ca9d07cedfb94e5cf7ce8ad3c0ce1d693'
BASELINE_SHA='f7c8085f8453ecb6b10841b698092d41e7fbec64d9d46f9341b9b9b538bfefa1'
VANILLA_SHA='7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'

def derive(raw):
    assert hashlib.sha256(PLAN.read_bytes()).hexdigest()==PLAN_SHA
    plan=json.loads(PLAN.read_text())
    original=next(c for c in plan['cases'] if c['id']=='where-l0-empty-vs-bare')
    recipe=original['composed_predicate'];source=recipe['original_expression'];control=recipe['calibration_expression']
    assert len(source.encode())==len(control.encode())
    assert source.index('selector(strong)')==control.index('selector(strong)')
    assert raw['plan_sha256']==PLAN_SHA and raw['phase']=='calibration'
    assert raw['binaries']['baseline']['sha256']==BASELINE_SHA and raw['binaries']['vanilla']['sha256']==VANILLA_SHA
    results=[]
    for profile in ['default','html','a11y','html+a11y']:
        expected_by_order=[]
        for order in ['normal','repeat','reverse']:
            def row(id,product):
                found=[(i,r) for i,r in enumerate(raw['rows']) if r['id']==id and r['product']==product and r['profile']==profile and r['order']==order]
                assert len(found)==1,(id,product,profile,order,'missing/duplicate control')
                i,r=found[0];assert r['execution']=='Observed' and r['unknown_reason'] is None
                return i,r
            index,r=row(recipe['calibration_case_id'],'baseline')
            _,isolated=row('causal-bare-strong-isolated','baseline')
            assert r['exit']==isolated['exit']==1 and r['stdout']==isolated['stdout']==''
            first=r['stderr'].split('\n',1)[0];assert first==isolated['stderr'].split('\n',1)[0]
            assert first.startswith('error: ') and 'selector where não suportado' not in first
            positions=re.findall(r'<input-expression>:1:([0-9]+)',r['stderr'])
            assert len(positions)==1,('ambiguous diagnostic context',positions)
            lo,hi=recipe['expected_baseline_calibration']['diagnostic_primary_span_within_utf8_byte_range']
            assert lo<=int(positions[0])-1<hi,('diagnostic outside preserved bare call',positions)
            old='1 │ '+control;new='1 │ '+source
            assert r['stderr'].count(old)==1
            # The sole permitted replacement covers a complete source line;
            # no auxiliary context/source occurrence may be normalized.
            rendered=[line for line in r['stderr'].split('\n') if line.startswith('1 │ ')]
            assert rendered==[old],('additional source context',rendered)
            expected={'exit':r['exit'],'stdout':r['stdout'],'stderr':r['stderr'].replace(old,new,1)}
            _,allowed=row('causal-strong-empty-isolated','vanilla')
            _,heading=row('causal-heading-empty-vs-bare','vanilla')
            assert allowed['exit']==0 and allowed['stderr']==''
            assert heading['exit']==0 and heading['stdout']=='"false"\n' and heading['stderr']==''
            expected_by_order.append(expected)
            results.append({'profile':profile,'order':order,'raw_row_index':index,'source_case':recipe['calibration_case_id'],'target_case':'where-l0-empty-vs-bare','candidate_expected':expected,'byte_mapping':{'old':old,'new':new,'replacements':1,'all_other_bytes':'identical'}})
        assert expected_by_order[0]==expected_by_order[1]==expected_by_order[2],('unstable control diagnostic',profile)
    return results

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--raw',required=True);p.add_argument('--output',required=True);a=p.parse_args()
    raw_path=pathlib.Path(a.raw).resolve();out=pathlib.Path(a.output).resolve()
    assert raw_path.parent==D and raw_path.name=='p1339-ab-batch1-focal-runs.json'
    assert out.parent==D and out.name=='p1339-ab-batch1-causal-derived-expectations.json' and not out.exists()
    raw=json.loads(raw_path.read_text());results=derive(raw)
    receipt={'schema':'p1339-causal-derived-expectations-v1','authority_manifest_sha256':'842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b','contract_sha256':'c0cd1826679ddaf77e937a677839c5cbe64fdae6ec71bfa34e635d584d9c3e17','raw':{'path':str(raw_path),'sha256':hashlib.sha256(raw_path.read_bytes()).hexdigest()},'recipe':{'path':str(pathlib.Path(__file__).resolve()),'sha256':hashlib.sha256(pathlib.Path(__file__).read_bytes()).hexdigest()},'plan_sha256':PLAN_SHA,'rows':results,'candidate_observed':False,'credit':'diagnostic calibration only; no candidate result or semantic workaround'}
    text=json.dumps(receipt,ensure_ascii=True,indent=2)+'\n';patch='*** Begin Patch\n*** Add File: '+str(out)+'\n'+'\n'.join('+'+s for s in text.split('\n')[:-1])+'\n*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)
    print(hashlib.sha256(out.read_bytes()).hexdigest())
