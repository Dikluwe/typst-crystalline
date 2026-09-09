#!/usr/bin/env python3
import pathlib,hashlib,json,subprocess,datetime,sys
D=pathlib.Path(__file__).resolve().parent
def digest(p): return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def save(name,obj):
    p=D/name
    assert not p.exists() and name.startswith('p1334-ab-')
    t=json.dumps(obj,indent=2,ensure_ascii=False)+'\n'
    subprocess.run(['apply_patch'],input='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+l+'\n' for l in t.splitlines())+'*** End Patch\n',text=True,check=True,capture_output=True)
if __name__=='__main__':
    manifest=json.loads((D/'p1334-manifest.json').read_text())
    artifacts=['p1334-ab-tests.rs','p1334-ab-dispatch-tests.rs']+[f'p1334-ab-p{n}-successor.rs' for n in range(1328,1334)]+['p1334-ab-migration-ledger.json']
    inputs=['p1334-manifest.json','p1334-baseline-public.json','p1334-public-dispatch-tests.rs','p1333-ab-tests.rs','p1333-ab-native-freeze.json']+[f'p1333-ab-p{n}-successor.rs' for n in range(1328,1333)]
    # Final formatted differences, including the one coherent-None repair.
    import difflib
    migration=[]
    for n in range(1328,1334):
        src=f'p1333-ab-p{n}-successor.rs' if n<1333 else 'p1333-ab-tests.rs'; dst=f'p1334-ab-p{n}-successor.rs'
        migration.append({'source':src,'successor':dst,'diff':''.join(difflib.unified_diff((D/src).read_text().splitlines(True),(D/dst).read_text().splitlines(True),fromfile=src,tofile=dst))})
    save('p1334-ab-migration-formatted.json',{'native':migration,'policy':'Historical contradictory Some fixtures only local robustness; missing stale fixture explicitly repaired to None. First-invalid expectations unchanged.'})
    artifacts+=['p1334-ab-migration-formatted.json']
    save('p1334-ab-native-freeze.json',{'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'regime':'A/B without technical isolation attestation or refinement seal','role':'/root/p1334_tests','manifest_sha256':digest(D/'p1334-manifest.json'),'l0_norm_sha256':manifest['prompt_norm_sha256'],'owners':manifest['owners'],'inputs':{n:digest(D/n) for n in inputs},'artifacts':{n:digest(D/n) for n in artifacts},'status':'Native frozen before candidate; B did not compile or execute native/candidate. CLI measurement pending. Complete freeze must preserve these hashes.'})
