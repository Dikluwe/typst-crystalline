#!/usr/bin/env python3
"""P1334 B oracle: full literal outputs, references measured once then pinned."""
import argparse,datetime,hashlib,importlib.util,json,pathlib,subprocess,sys,concurrent.futures
sys.dont_write_bytecode=True
ROOT=pathlib.Path(__file__).resolve().parents[2]; D=ROOT/'00_nucleo/diagnosticos'
spec=importlib.util.spec_from_file_location('p1333_history',D/'p1333-ab-cli.py')
history=importlib.util.module_from_spec(spec); spec.loader.exec_module(history)
BASE='/tmp/p1333-target.MDDkou/release/typst'; VANILLA='/usr/local/bin/typst'
PINS={'BASE':'79470612fc121fa42a6898846f85d0b0325edf517c90a830c01cde947f748ffe','VANILLA':'7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8'}
MANIFEST=D/'p1334-manifest.json'; FROZEN=D/'p1334-ab-cli-expected.json'; CACHE=D/'p1334-ab-cli-baseline.json'
NORM='05736e494ca184ce7c13b8eb5a9296275cce0e5c4605fde251b7ab2b17ec8666'
PROFILES=history.PROFILES
MIGRATE={'zero','name-empty-guard','name-with-empty-guard','precedence-empty-spread','precedence-no-positional-named-value','precedence-no-positional-named'}|{n for n,c,e in history.CASES if n.startswith('precedence-valid-')}
CASES=list(history.CASES)+[
('signature-named-value-after-other','signature-parity','calc.abs(bad: 1, value: false)'),
('signature-named-value-repeated-with','signature-parity','{let a=calc.abs.with(bad: 1).with(value: false).with(value: -1); a()}'),
('signature-named-value-arguments','signature-parity','{let aa=arguments(bad: 1,value: false); calc.abs(..aa)}'),
('signature-named-value-dict-spread','signature-parity','calc.abs(..(value: false))'),
('signature-named-value-and-valid','signature-parity','calc.abs(value: false,-1)'),
('signature-extra-before-named','signature-parity','calc.abs(-1,2,bad: 3)'),
('signature-named-before-extra','signature-parity','calc.abs(bad: 3,-1,2)'),
('signature-extra-arguments','signature-parity','{let aa=arguments(-1,2,bad: 3); calc.abs(..aa)}'),
('signature-extra-array','signature-parity','calc.abs(..(-1,2))'),
('signature-extra-with','signature-parity','{let a=calc.abs.with(-1, bad: false).with(2); a()}'),
('signature-extra-duplicate-named','signature-parity','{let a=calc.abs.with(-1,bad: false).with(bad: 2); a()}'),
('signature-missing-import','signature-parity','{import calc: abs; abs()}'),
('signature-missing-nested-with','signature-parity','{let a=calc.abs.with().with(); a()}'),
('signature-missing-none','signature-parity','calc.abs(..none)'),
('signature-missing-arguments','signature-parity','{let aa=arguments(); calc.abs(..aa)}'),
('signature-missing-math-qualified','signature-parity','$std.calc.abs()$'),
('signature-missing-utf8','signature-parity','{let café=calc.abs;\n café()}'),
('signature-warning-hint','signature-parity','{import std; calc.abs(value: false)}'),
('signature-warning-surplus','signature-parity','{import std; calc.abs(-1,2)}'),
('signature-imported-math-debt','preserved-debt-imported-math-resolution','{import calc: abs; $abs()$}'),
('signature-fake-homonym','preserved-parity','{let abs(..a)=a; abs()}'),
('signature-foreign-sqrt','preserved-debt','calc.sqrt()'),
('signature-csv-control','preserved-parity','csv()'),
('signature-json-control','preserved-parity','json.encode()'),
('signature-panic-control','preserved-parity','panic()'),
('signature-eager-control','preserved-parity','calc.abs(-1,bad:panic("later"))'),
]
def digest(p): return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def observed(r): return {k:r[k] for k in ('exit','stdout','stderr')}
def command(argv):
    at=datetime.datetime.now(datetime.timezone.utc).isoformat()
    p=subprocess.run(argv,cwd=ROOT,capture_output=True)
    return {'argv':argv,'at':at,'exit':p.returncode,'stdout':p.stdout.decode(),'stderr':p.stderr.decode()}
def save(path,value):
    p=pathlib.Path(path).resolve(); assert p.parent==D and p.name.startswith('p1334-ab-') and not p.exists()
    t=json.dumps(value,ensure_ascii=False,indent=2)+'\n'
    subprocess.run(['apply_patch'],input='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+l+'\n' for l in t.splitlines())+'*** End Patch\n',text=True,check=True,capture_output=True)
def main():
    ap=argparse.ArgumentParser(); ap.add_argument('--candidate'); ap.add_argument('--output',required=True); ap.add_argument('--freeze-from'); ap.add_argument('--order',choices=['normal','reverse'],default='normal'); a=ap.parse_args()
    assert history.normhash()==NORM
    assert digest(MANIFEST)=='2cac9aab9dc8e911e2a14e932515db3efd5be64b10b1089d06caac0b310be1da'
    for role,path in [('BASE',BASE),('VANILLA',VANILLA)]: assert digest(path)==PINS[role]
    if a.freeze_from:
        assert not a.candidate
        baseline=json.loads(pathlib.Path(a.freeze_from).read_text())
        historical={(r['case'],r['profile']):r for r in json.loads((D/'p1333-ab-cli-expected.json').read_text())['expectations']}
        assert len(historical)==712
        expected=[]; ledger=[]
        for row in baseline['cases']:
            key=(row['case'],row['profile']); before=observed(row['results']['BASE']); cls=row['classification']
            if key in historical: assert before==historical[key]['expected'],key; cls=historical[key]['classification']
            if row['case'] in MIGRATE or row['classification']=='signature-parity':
                oracle=observed(row['results']['VANILLA']); cls='signature-parity'
                assert oracle['exit']!=0,key
                ledger.append({'case':row['case'],'profile':row['profile'],'historical':key in historical,'before':before,'after':oracle})
            else:
                oracle=before
                if row['classification']=='preserved-parity': assert oracle==observed(row['results']['VANILLA']),key
            expected.append({'case':row['case'],'profile':row['profile'],'classification':cls,'expected':oracle})
        assert len(expected)==len(CASES)*len(PROFILES)
        save(a.output,{'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'manifest_sha256':digest(MANIFEST),'l0_norm_sha256':NORM,'baseline_sha256':digest(a.freeze_from),'runner_sha256':digest(__file__),'historical_expectations_sha256':digest(D/'p1333-ab-cli-expected.json'),'migration_ledger':ledger,'policy':'All 712 prior cells checked against pinned BASE; only explicit missing/valid-first guards migrate to literal VANILLA. Known math resolution and all other debts preserved. Full literal candidate exit/stdout/stderr without normalization. Cached references retain original argv/UTC, verified binary/cache/expected hashes.','expectations':expected}); return
    frozen=json.loads(FROZEN.read_text()) if a.candidate else None
    cache=json.loads(CACHE.read_text()) if a.candidate else None
    if a.candidate:
        assert digest(CACHE)==frozen['baseline_sha256']; assert digest(__file__)==frozen['runner_sha256']
        assert cache['binaries']=={k:{'path':p,'sha256':PINS[k]} for k,p in [('BASE',BASE),('VANILLA',VANILLA)]}
    expectations={(r['case'],r['profile']):r for r in frozen['expectations']} if frozen else {}
    cached={(r['case'],r['profile']):r for r in cache['cases']} if cache else {}
    cases=CASES if a.order=='normal' else list(reversed(CASES))
    def measure(cell):
        n,c,e,profile,flags=cell
        results=dict(cached[(n,profile)]['results']) if a.candidate else {k:command([p,'--color','never','eval',*flags,e]) for k,p in [('BASE',BASE),('VANILLA',VANILLA)]}
        row={'case':n,'classification':c,'expression':e,'profile':profile,'results':results}
        if a.candidate:
            results['CANDIDATE']=command([a.candidate,'--color','never','eval',*flags,e]); ex=expectations[(n,profile)]
            row.update(classification=ex['classification'],expected=ex['expected'],candidate_matches_frozen_policy=observed(results['CANDIDATE'])==ex['expected'])
        return row
    provenance=json.loads((D/'p1334-baseline-public.json').read_text())
    receipt={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':provenance['head'],'diff_stat':provenance['diff_stat'],'working_tree':'pinned BASE provenance from public baseline; candidate executable hash identifies candidate separately','manifest_sha256':digest(MANIFEST),'l0_norm_sha256':NORM,'public_baseline_sha256':digest(D/'p1334-baseline-public.json'),'runner_sha256':digest(__file__),'expected_sha256':digest(FROZEN) if frozen else None,'cache_sha256':digest(CACHE) if cache else None,'reference_mode':'cached immutable BASE/VANILLA observations' if cache else 'fresh BASE/VANILLA execution','order':a.order,'binaries':{k:{'path':p,'sha256':PINS[k]} for k,p in [('BASE',BASE),('VANILLA',VANILLA)]}}
    if a.candidate: receipt['candidate_binary']={'path':a.candidate,'sha256':digest(a.candidate)}
    with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
        receipt['cases']=list(pool.map(measure,[(n,c,e,p,f) for n,c,e in cases for p,f in PROFILES]))
    save(a.output,receipt)
    failures=[r['case']+'/'+r['profile'] for r in receipt['cases'] if r.get('candidate_matches_frozen_policy') is False]
    print(json.dumps({'observations':len(receipt['cases']),'candidate_failures':failures}))
    if failures: raise SystemExit(1)
if __name__=='__main__': main()
