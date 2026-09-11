"""Focal A.1 measurement; no implementation, L0 edits, oracle seal or PASS claim."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def sha(p):
    with Path(p).open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()

def save(name, data):
    path = D / ('p1339-' + name + '.json')
    assert not path.exists(), path
    body = json.dumps(data, ensure_ascii=True, indent=2) + '\n'
    patch = '*** Begin Patch\n*** Add File: ' + str(path) + '\n' + ''.join('+' + l + '\n' for l in body.splitlines()) + '*** End Patch\n'
    subprocess.run(['apply_patch'], input=patch, text=True, cwd=ROOT, capture_output=True, check=True)
    print(name, sha(path), flush=True)

def main():
    a0 = json.loads((D/'p1339-a0.json').read_text())
    assert a0['baseline_matches_p1338']
    step = a0['step']
    assert sha(ROOT/step['path']) == step['sha256']
    save('authority-manifest', dict(
        at=now(), phase='A.1 focal, before L0/contract/candidate',
        predecessor=dict(path='p1339-a0.json',sha256=sha(D/'p1339-a0.json')),
        step=step, regime='executado sem atestação de isolamento',
        capabilities_enforced=False,
        roles={
            'operator_and_future_implementer':dict(executor='/root',environment='shared workspace',context='existing conversation',read=['step','baseline','current L0/product','pinned source','public probe outputs'],write=['p1339 measurement scripts, manifests and receipts'],forbidden=['contract, oracle or verification authorship combined with implementation'],inputs=dict(a0=sha(D/'p1339-a0.json'),step=step['sha256']),output_hashes='referenced by subsequent receipts'),
            'preliminary_reviewer':dict(executor='/root/p1339_preflight_review',environment='shared workspace',context='fresh task only; no inherited conversation',read=['explicit P1339 step','P1339 A0','probe manifest, runner and outputs','relevant pinned vanilla source','relevant ADRs and skill'],write=['00_nucleo/diagnosticos/p1339-review-*'],forbidden=['edit judged inputs','L0 or product writes','candidate implementation'],inputs='to be pinned in reviewer receipt before reading',output_hashes='reviewer receipt'),
            'contract_author':dict(executor=None,status='not started; assign only after A and frozen L0'),
            'oracle_author_and_ab_tester':dict(executor=None,status='not started; fresh isolated-context task required'),
            'adversary':dict(executor=None,status='not started; baseline and contract required'),
            'final_verifier':dict(executor=None,status='not started; may not edit verified material')},
        unknown='Mandatory unexpected Unknown blocks progression; opaque controls remain Unknown.',
        budget=dict(contract_revisions=3,full_discrimination_runs=2,no_gain_revisions=2),
        limits='Only A.1 focal probes. Full A.1, source classification, L0, contract, mutants and implementation remain gated.'))
    cases = []
    routes = ['angle.deg','angle.rad','float.inf','float.nan','float.signum','float.from-bytes','float.to-bytes','function.with','function.where','version.at']
    for route in routes:
        cases.append(dict(id='discovery-'+route,expression=f'repr((type({route}), repr({route})))',preclassification='Violated',basis='P1335 absence measurement; rechecking successor'))
    for unit in ('deg','rad'):
        for value in ('0','1','-1','0.0','-0.0','1.5','90deg','-90deg','1rad','-0deg'):
            cases.append(dict(id=f'angle-{unit}-static-{value}',expression=f'repr(angle.{unit}({value}))',preclassification='Unknown',basis='Call contract not established by inventory'))
        for value in ('90deg','1rad','-0deg'):
            cases.append(dict(id=f'angle-{unit}-bound-{value}',expression=f'repr(({value}).{unit}())',preclassification='Unknown',basis='Bound surface must be measured'))
        for args in ('','90deg, 1','90deg, other: 1','"bad"'):
            cases.append(dict(id=f'angle-{unit}-invalid-{len(cases)}',expression=f'repr(angle.{unit}({args}))',preclassification='Unknown',basis='Diagnostic probe; errors are observables, not runner failures'))
        cases.append(dict(id=f'angle-{unit}-step-equivalence',expression=f'repr(angle.{unit}(1) == 1 * 1{unit})',preclassification='Unknown',basis='Literal equivalence claimed in step, not assumed true'))
        cases.append(dict(id=f'angle-{unit}-multiply-control',expression=f'repr((type(1 * 1{unit}), 1 * 1{unit}))',preclassification='Preserved',basis='Existing multiplication control'))
    profiles = {'default':[], 'html':['--features','html'], 'a11y':['--features','a11y-extras'], 'html+a11y':['--features','html,a11y-extras']}
    save('probe-manifest',dict(at=now(),authority_manifest_sha256=sha(D/'p1339-authority-manifest.json'),a0_sha256=sha(D/'p1339-a0.json'),step=step,runner_sha256=sha(__file__),scope='A.1 focal: ten discoveries and angle contract; not complete A.1 suite',profiles=profiles,orders=['normal','reverse'],cases=cases,timeout_seconds=15,unknown='timeout/invalid runner/ambiguous observation are Unknown and never success; preclassifications are hypotheses, not final verdicts'))
    env = dict(os.environ, LC_ALL='C', NO_COLOR='1')
    for key in list(env):
        if key.startswith('TYPST_'):
            del env[key]
    results = {}
    for name, binary_key in [('vanilla','vanilla'),('crystalline-before','candidate')]:
        binary = a0['binaries'][binary_key]
        assert sha(binary['path']) == binary['sha256']
        start = now()
        rows = []
        for order in ('normal','reverse'):
            for case in (cases if order=='normal' else list(reversed(cases))):
                for profile, flags in profiles.items():
                    argv=[binary['path'],'eval','--format','json',*flags,case['expression']]
                    tick=time.monotonic(); begun=now()
                    try:
                        p=subprocess.run(argv,cwd=ROOT,env=env,capture_output=True,timeout=15)
                        row=dict(exit=p.returncode,stdout=p.stdout.decode(),stderr=p.stderr.decode(),execution='Observed')
                    except subprocess.TimeoutExpired as e:
                        row=dict(exit=None,stdout=(e.stdout or b'').decode(),stderr=(e.stderr or b'').decode(),execution='Unknown')
                    rows.append(dict(id=case['id'],order=order,profile=profile,argv=argv,cwd=str(ROOT),start=begun,end=now(),seconds=time.monotonic()-tick,**row))
        result=dict(start=start,end=now(),head=a0['head'],tracked_diff_stat='',binary=binary,environment=dict(LC_ALL='C',NO_COLOR='1',TYPST_variables='removed'),a0_sha256=sha(D/'p1339-a0.json'),authority_manifest_sha256=sha(D/'p1339-authority-manifest.json'),probe_manifest_sha256=sha(D/'p1339-probe-manifest.json'),runner_sha256=sha(__file__),rows=rows)
        save(name+'-runs',result)
        results[name] = {(r['id'],r['order'],r['profile']):r for r in rows}
    comparisons=[]
    for key,v in results['vanilla'].items():
        c=results['crystalline-before'][key]
        unknown=v['execution']=='Unknown' or c['execution']=='Unknown'
        same=all(v[k]==c[k] for k in ('exit','stdout','stderr'))
        comparisons.append(dict(id=key[0],order=key[1],profile=key[2],classification='Unknown' if unknown else 'Preserved' if same else 'Violated',basis='exact public exit/stdout/stderr comparison, including diagnostic spans'))
    instability=[]
    for name,rows in results.items():
        for case in cases:
            for profile in profiles:
                n,r=rows[(case['id'],'normal',profile)],rows[(case['id'],'reverse',profile)]
                if any(n[k]!=r[k] for k in ('execution','exit','stdout','stderr')):
                    instability.append((name,case['id'],profile))
    assert not subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT)
    save('comparison-before',dict(at=now(),scope='Focal A.1 only; not a contract seal or full RED',inputs={name:sha(D/('p1339-'+name+'.json')) for name in ['a0','authority-manifest','probe-manifest','vanilla-runs','crystalline-before-runs']},comparisons=comparisons,instability=instability,product_unchanged=all(sha(ROOT/p)==h for p,h in a0['product_inventory'].items()),counts={label:sum(r['classification']==label for r in comparisons) for label in ['Preserved','Violated','Unknown']}))

if __name__ == '__main__':
    main()
