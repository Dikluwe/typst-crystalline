#!/usr/bin/env python3
"""Freeze measured exact channels; unresolved normative E08 never gains credit."""
import hashlib, json, pathlib, subprocess, datetime
D=pathlib.Path('/repos/Antigravity/typst-crystalline/00_nucleo/diagnosticos')
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def save(name,value):
    p=D/name; assert not p.exists(); s=json.dumps(value,ensure_ascii=False,indent=2)+'\n'
    patch='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+x+'\n' for x in s.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,capture_output=True,check=True)
    print(json.dumps({'path':str(p),'sha256':sha(p)}))
inputs=D/'p1339-contract-array-inputs-r2.json'; raw=D/'p1339-contract-array-focal-raw-r1.json'
spec=json.loads(inputs.read_text()); data=json.loads(raw.read_text()); assert data['inputs_sha256']==sha(inputs)
lookup={(r['id'],r['binary']):r for r in data['runs']}
oracles=[]
for c in spec['cases']:
    v,b=lookup[c['id'],'vanilla'],lookup[c['id'],'baseline']; policy=c['policy']
    channels=lambda r:{k:r[k] for k in ('exit','stdout','stderr')}
    valid=all(r['exit'] is not None for r in (v,b))
    if c['role']=='positive': valid=valid and v['exit']==0 and b['exit']==1 and b['stderr'].startswith('error: type array does not have a constructor\n')
    chosen=v if policy=='vanilla' else b
    if policy=='normative_l0': status='Unknown'; expected=None
    elif not valid: status='Unknown'; expected=None
    else: status='FrozenLiteralDefault'; expected=channels(chosen)
    oracles.append({**c,'profile':'default','source_sha256':hashlib.sha256(c['source'].encode()).hexdigest(),'status':status,'expected':expected,'vanilla_observed':channels(v),'baseline_observed':channels(b),'baseline_differs_from_expected':None if expected is None else channels(b)!=expected,'interpretation':'Only exact measured channels, not candidate result; E08 requires separately proven normative detached origin and has no guessed expected output.'})
save('p1339-contract-array-oracles-focal-r1.json',{'schema':'p1339-array-focal-literal-oracles-v1','utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'author':'/root/p1312_tests combined contract and oracle authority','inputs_sha256':sha(inputs),'raw_sha256':sha(raw),'freezer_sha256':sha(pathlib.Path(__file__)),'cases':oracles,'scope':'Default focal only. Other profiles/orders not executed by this freeze, no extrapolated values. No candidate or real mutation execution. Original 70 public oracles unchanged.'})
print(json.dumps({'counts':{s:sum(o['status']==s for o in oracles) for s in set(o['status'] for o in oracles)},'direct_red_ids':[o['id'] for o in oracles if o['role']=='positive' and o['baseline_differs_from_expected'] is True]}))
