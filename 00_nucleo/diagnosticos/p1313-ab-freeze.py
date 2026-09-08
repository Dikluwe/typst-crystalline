"""Freeze public observations before candidate; evaluate fixed expectations afterwards."""
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
    with open(path,'rb') as f:
        return hashlib.file_digest(f,'sha256').hexdigest()
def normative():
    raw,count=re.subn(rb'(?m)^Hash do C\xc3\xb3digo: [0-9a-f]+\r?\n',b'',L0.read_bytes())
    assert count==1
    return hashlib.sha256(raw).hexdigest()
def obs(row):
    return {k:row[k] for k in ('exit','stdout','stderr')}
def save(path, result):
    path=Path(path).resolve()
    assert not path.exists()
    raw=json.dumps(result,ensure_ascii=False,indent=2)+'\n'
    patch='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+x+'\n' for x in raw.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,check=True,capture_output=True)
def main():
    p=argparse.ArgumentParser()
    p.add_argument('--measurement',required=True)
    p.add_argument('--candidate-runs')
    p.add_argument('--output',required=True)
    args=p.parse_args()
    path=Path(args.measurement).resolve()
    doc=json.loads(path.read_text())
    cases_path=BASE/'p1313-ab-cases.json'
    cases=json.loads(cases_path.read_text())['cases']
    stamp=datetime.datetime.now(datetime.timezone.utc).isoformat()
    if not args.candidate_runs:
        assert doc['inputs']['cases_sha256']==sha(cases_path)
        assert doc['inputs']['runner_sha256']==sha(BASE/'p1313-ab-runner.py')
        assert doc['inputs']['l0_sha256']==sha(L0)
        expected=[]
        for profile in ('default','html','a11y','html+a11y'):
            for case in cases:
                source='baseline' if case['kind'] in ('control','legacy-bytes') else 'vanilla'
                run,=[r for r in doc['runs'] if r['id']==case['id'] and r['profile']==profile and r['product']==source and r['order']=='normal']
                assert run['argv'][2]==(case['surrogate'] if case['kind']=='legacy-bytes' else case['expr'])
                observation=obs(run)
                assert observation['exit'] in (0,1), ('Unknown',case['id'])
                if case['kind']=='cast':
                    assert observation['exit']==1 and observation['stdout']==''
                    assert observation['stderr'].startswith('error: expected path, string, or bytes, found '),case['id']
                elif case['kind']=='value':
                    assert observation['exit']==0 and observation['stderr']=='',case['id']
                    assert isinstance(json.loads(observation['stdout']),list),case['id']
                elif case['kind']=='symbol':
                    assert observation['stderr'].startswith('error: file not found (searched at /tmp/p1313-ab-fixtures/α)\n')
                    observation['stderr']='error: expected path, string, or bytes, found symbol\n'+observation['stderr'].split('\n',1)[1]
                    source='normative Symbol rejection: L0 message, measured vanilla origin and trace; not equality'
                elif case['kind']=='legacy-bytes':
                    assert observation['stderr']=='' or case['surrogate'] in observation['stderr']
                    observation['stderr']=observation['stderr'].replace(case['surrogate'],case['expr'])
                    source='baseline Path/Str equivalent; only displayed source expression replaced, legacy message and anchor unchanged'
                if case['kind'] in ('cast','value'):
                    baseline,=[r for r in doc['runs'] if r['id']==case['id'] and r['profile']==profile and r['product']=='baseline']
                    assert obs(baseline)!=observation,('baseline not RED',case['id'])
                expected.append(dict(id=case['id'],kind=case['kind'],profile=profile,oracle=source,expected=observation))
        paths=[cases_path,BASE/'p1313-ab-runner.py',Path(__file__),path,BASE/'p1313-implementation-baseline.json',ROOT/'00_nucleo/materialization/typst-passo-1313.md',ROOT/'lab/typst-original/crates/typst-library/src/loading/csv.rs',ROOT/'lab/typst-original/crates/typst-library/src/loading/mod.rs']
        inputs={str(f.relative_to(ROOT)):sha(f) for f in paths}
        inputs.update(doc['inputs']['fixtures'])
        result=dict(schema='p1313-ab-freeze-v1',utc=stamp,regime='A/B executado sem atestação de isolamento técnico; sem selo completo',capabilities=dict(executor='/root/p1313_tests',reads=['skill and both references','loading L0','authorized exact P1313 step','implementation baseline JSON','vanilla csv.rs and loading/mod.rs','historical p1312-ab-*','binary CLI observations','git HEAD/diff stat/status'],writes=['00_nucleo/diagnosticos/p1313-ab-*','/tmp/p1313-ab-fixtures/*'],context='Role-scoped new context. No crystalline loading.rs, candidate diff or local owner tests read. Shared filesystem has broader capabilities than declared allowlist; no technical isolation attestation.'),unknown_policy='Any required missing, duplicate, malformed, ambiguous, timeout, crash or unverified input blocks; Unknown never converts to success.',budget='Initial 142-case observation; prepatch revision adds two exact historical sentinels and equal-data single.csv for legacy success paths. Final baseline once, candidate normal/repeat/reverse. Two consecutive failures with the same cause require review before a third attempt.',l0=dict(path=str(L0.relative_to(ROOT)),raw_sha256=sha(L0),normative_sha256=normative(),allowed_mutation='Exactly one canonical Hash do Código metadata line; all other bytes protected.'),baseline_binaries=doc['binaries'],provenance=doc['start'],inputs=inputs,expected=expected,limitations=['Symbol rejection is normative, not vanilla equality.','CSV parsing/options/named/missing/excess/duplicates intentionally retain baseline behavior and diagnostic anchors.','CLI path-looking Bytes tests reject opening bytes as filename; strict zero World calls needs owner tests and source review.','Synthetic Rust Args and explicitly detached Rust carriers require owner evidence; public arguments.map route covers detached language origin.','Invalid UTF-8 fixture is not present in this CLI corpus; malformed CSV unequal fields exercises legacy decoder errors.','No general language equivalence or technical isolation attested.'])
    else:
        for name,digest in doc['inputs'].items():
            assert sha(ROOT/name)==digest,('frozen drift',name)
        assert normative()==doc['l0']['normative_sha256']
        candidate=json.loads(Path(args.candidate_runs).read_text())
        assert candidate['inputs']['cases_sha256']==sha(cases_path)
        assert candidate['inputs']['runner_sha256']==sha(BASE/'p1313-ab-runner.py')
        required={(e['id'],e['profile'],o) for e in doc['expected'] for o in ('normal','repeat','reverse')}
        actual=[(r['id'],r['profile'],r['order']) for r in candidate['runs']]
        assert set(actual)==required and len(actual)==len(required), 'Unknown: missing or duplicate observation'
        failures=[]
        for run in candidate['runs']:
            case,=[c for c in cases if c['id']==run['id']]
            assert run['argv'][2]==case['expr'] and run['product']=='candidate'
            expect,=[e for e in doc['expected'] if e['id']==run['id'] and e['profile']==run['profile']]
            if obs(run)!=expect['expected']:
                failures.append(dict(id=run['id'],kind=run['kind'],profile=run['profile'],order=run['order'],expected=expect['expected'],actual=obs(run)))
        result=dict(schema='p1313-ab-comparison-v1',utc=stamp,freeze_sha256=sha(path),candidate_runs_sha256=sha(args.candidate_runs),candidate=candidate['binaries'],comparisons=len(actual),failures=failures,unknown=0,status='PASS' if not failures else 'FAIL')
    save(args.output,result)
    print(json.dumps(dict(output=args.output,sha256=sha(args.output),items=len(result.get('expected',result.get('failures',[]))))))
if __name__=='__main__':
    main()
