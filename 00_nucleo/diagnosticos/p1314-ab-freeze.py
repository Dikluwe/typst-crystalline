"""Freeze measured, normative public expectations before the implementation exists."""
import argparse
import datetime
import hashlib
import json
from pathlib import Path
import re
import subprocess

BASE=Path(__file__).resolve().parent
ROOT=BASE.parents[1]
L0=ROOT/'00_nucleo/prompts/compiler/stdlib/loading.md'
def sha(path):
    with open(path,'rb') as f: return hashlib.file_digest(f,'sha256').hexdigest()
def normative():
    raw,n=re.subn(rb'(?m)^Hash do C\xc3\xb3digo: [0-9a-f]+\r?\n',b'',L0.read_bytes())
    assert n==1
    return hashlib.sha256(raw).hexdigest()
def obs(row): return {k:row[k] for k in ('exit','stdout','stderr')}
def save(path,doc):
    path=Path(path).resolve()
    assert not path.exists(),path
    raw=json.dumps(doc,ensure_ascii=False,indent=2)+'\n'
    patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+x+'\n' for x in raw.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,check=True,capture_output=True)

def main():
    p=argparse.ArgumentParser()
    p.add_argument('--measurement',required=True)
    p.add_argument('--candidate-runs')
    p.add_argument('--output',required=True)
    a=p.parse_args()
    src=Path(a.measurement).resolve()
    doc=json.loads(src.read_text())
    cases_path=BASE/'p1314-ab-cases.json'
    cases=json.loads(cases_path.read_text())['cases']
    now=datetime.datetime.now(datetime.timezone.utc).isoformat()
    if not a.candidate_runs:
        assert doc['inputs']['cases_sha256']==sha(cases_path)
        assert doc['inputs']['runner_sha256']==sha(BASE/'p1314-ab-runner.py')
        assert doc['inputs']['l0_sha256']==sha(L0)
        expected=[]
        historical=json.loads((BASE/'p1313-ab-candidate-runs.json').read_text())
        for profile in ('default','html','a11y','html+a11y'):
            for c in cases:
                source='baseline' if c['kind']=='control' else 'vanilla'
                row,=[r for r in doc['runs'] if r['id']==c['id'] and r['profile']==profile and r['product']==source and r['order']=='normal']
                assert row['argv'][2]==c['expr']
                observation=obs(row)
                assert observation['exit'] in (0,1),('Unknown process',c['id'])
                if c['kind'] in ('target','symbol'):
                    assert observation['exit']==1 and observation['stdout']=='',('not option failure',c['id'],observation)
                    assert observation['stderr'].startswith(('error: expected ', 'error: delimiter must be an ASCII character')),('wrong error stratum',c['id'],observation)
                if c['kind']=='value': assert observation['exit']==0 and observation['stderr']==''
                if c['kind']=='symbol':
                    assert observation['stderr'].startswith('error: delimiter must be an ASCII character\n')
                    observation['stderr']='error: expected string, found symbol\n'+observation['stderr'].split('\n',1)[1]
                    source='L0 normative Symbol rejection; measured vanilla origin/trace, not coercion parity'
                baseline,=[r for r in doc['runs'] if r['id']==c['id'] and r['profile']==profile and r['product']=='baseline' and r['order']=='normal']
                if c['historical_p1313']:
                    prior,=[r for r in historical['runs'] if r['id']==c['id'] and r['profile']==profile and r['order']=='normal']
                    assert baseline['cwd']==prior['cwd']=='/tmp/p1313-ab-fixtures'
                    assert baseline['argv'][2]==prior['argv'][2]
                    assert obs(baseline)==obs(prior),('historical baseline drift',c['id'],profile)
                expected.append(dict(id=c['id'],expr=c['expr'],cwd=row['cwd'],kind=c['kind'],profile=profile,oracle=source,historical_p1313=c['historical_p1313'],baseline_red=obs(baseline)!=observation,expected=observation))
        protected=[cases_path,BASE/'p1314-ab-runner.py',Path(__file__),BASE/'p1314-ab-prepare.py',src,BASE/'p1314-baseline.json',BASE/'p1314-measurement.json',BASE/'p1313-ab-cases.json',BASE/'p1313-ab-freeze.json',ROOT/'lab/typst-original/crates/typst-library/src/loading/csv.rs',ROOT/'lab/typst-original/crates/typst-library/src/foundations/args.rs',ROOT/'lab/typst-original/crates/typst-library/src/foundations/value.rs']
        protected.extend([BASE/'p1314-ab-refine.py',BASE/'p1313-ab-candidate-runs.json'])
        inputs={str(x.relative_to(ROOT)):sha(x) for x in protected}
        inputs.update(doc['inputs']['fixtures'])
        result=dict(schema='p1314-ab-freeze-v1',utc=now,regime='A/B executado sem atestação de isolamento técnico; sem protocolo completo ou selo de refinamento',capabilities=dict(executor='/root/p1314_tests',reads=['skill + two references','loading L0','baseline/measurement/review-precontract','vanilla csv/args/value','historical A/B scripts/cases/outputs','binary CLI','git HEAD/status/diff stat'],writes=['00_nucleo/diagnosticos/p1314-ab-*','/tmp/p1314-ab-fixtures/*'],context='Role-scoped task. No P1314 candidate, owner loading.rs or local tests inspected. Authorized p1314-baseline.json was printed and incidentally exposed embedded historical P1310-P1313 diff; no P1314 candidate existed. Shared filesystem permits broader access; no isolation attestation.'),unknown_policy='Any missing, duplicate, malformed, timeout, crash, ambiguous or unverified required observation blocks. Unknown never silently passes.',budget='One initial baseline plus focal calibration if needed, then final baseline, candidate normal/repeat/reverse. Two failed revisions with same cause stop for review.',l0=dict(path=str(L0.relative_to(ROOT)),raw_sha256=sha(L0),normative_sha256=normative(),allowed_mutation='Exactly one canonical Hash do Código line excluded; every normative byte protected.'),inputs=inputs,baseline_binaries=doc['binaries'],provenance=doc['start'],expected=expected,historical_deltas=sorted({e['id'] for e in expected if e['historical_p1313'] and e['baseline_red']}),limitations=['CSV options only; no general parity claimed.','Symbol delimiter rejection is normative and not vanilla coercion equality.','Synthetic Rust Args and explicitly detached Rust carriers require owner tests/source review; CLI map route covers language-detached transport.','Absent paths discriminate option-before-I/O; strict zero World calls requires owner evidence.','Historical suite preserved verbatim; fresh fixtures directory changes only path provenance.','Tactical step status is not a protected normative input.'])
        result['historical_replay_policy'] = 'All 144 original P1313 ids/expressions run in /tmp/p1313-ab-fixtures, unchanged. Every fresh baseline observation must equal the P1313 candidate normal output literally. Only historical_deltas may change under the precise expected records; all remaining exit/stdout/stderr stay literal.'
        result['historical_case_mapping'] = [dict(id=c['id'],expr=c['expr'],cwd='/tmp/p1313-ab-fixtures',changed_policy='P1314 option value origin and all-occurrence validation' if c['kind']=='target' else 'literal P1313 exit/stdout/stderr preservation') for c in cases if c['historical_p1313']]
        result['calibration'] = dict(revisions=1,reason='Added exact P1312 sentinel and restored original cwd for all historical cases at owner request; preliminary option-stratum checks all passed; no candidate was inspected.',preliminary_measurement_sha256=sha(BASE/'p1314-ab-pre-measurement.json'),initial_processes=1864,final_processes=len(doc['runs']),failed_oracle_revisions=0)
    else:
        for name,digest in doc['inputs'].items(): assert sha(ROOT/name)==digest,('Unknown frozen drift',name)
        assert normative()==doc['l0']['normative_sha256'], 'Unknown normative drift'
        candidate=json.loads(Path(a.candidate_runs).read_text())
        assert candidate['inputs']['cases_sha256']==sha(cases_path)
        assert candidate['inputs']['runner_sha256']==sha(BASE/'p1314-ab-runner.py')
        required={(e['id'],e['profile'],o) for e in doc['expected'] for o in ('normal','repeat','reverse')}
        actual=[(r['id'],r['profile'],r['order']) for r in candidate['runs']]
        assert set(actual)==required and len(actual)==len(required),'Unknown missing or duplicate observation'
        expected={(e['id'],e['profile']):e for e in doc['expected']}
        by_id={c['id']:c for c in cases}
        failures=[]
        for row in candidate['runs']:
            assert row['argv'][2]==by_id[row['id']]['expr'] and row['product']=='candidate'
            e=expected[row['id'],row['profile']]
            assert row['cwd']==e['cwd'],('Unknown cwd drift',row['id'])
            if obs(row)!=e['expected']: failures.append(dict(id=row['id'],kind=row['kind'],profile=row['profile'],order=row['order'],expected=e['expected'],actual=obs(row)))
        result=dict(schema='p1314-ab-comparison-v1',utc=now,freeze_sha256=sha(src),candidate_runs_sha256=sha(a.candidate_runs),candidate=candidate['binaries'],comparisons=len(actual),unknown=0,failures=failures,status='FAIL' if failures else 'PASS')
    save(a.output,result)
    print(json.dumps(dict(output=a.output,sha256=sha(a.output),items=len(result.get('expected',result.get('failures',[]))),historical_deltas=result.get('historical_deltas'))))
if __name__=='__main__': main()
