#!/usr/bin/env python3
import sys,pathlib,importlib.util,json,hashlib,datetime,subprocess
sys.dont_write_bytecode=True
D=pathlib.Path(__file__).resolve().parent
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=json.loads((D/'p1334-ab-native-freeze-r1.json').read_text())
for section in ['inputs','artifacts']:
 for n,h in f[section].items():assert digest(D/n)==h,n
spec=importlib.util.spec_from_file_location('p1334_oracle',D/'p1334-ab-cli.py'); runner=importlib.util.module_from_spec(spec);spec.loader.exec_module(runner)
module=runner.history
while module:
 p=pathlib.Path(module.__file__); f['inputs'][p.name]=digest(p)
 if hasattr(module,'HISTORY'):
  p=pathlib.Path(module.HISTORY);f['inputs'][p.name]=digest(p)
 module=getattr(module,'history',None)
f['inputs']['p1334-ab-native-freeze-r1.json']=digest(D/'p1334-ab-native-freeze-r1.json')
for n in ['p1334-ab-cli.py','p1334-ab-cli-baseline.json','p1334-ab-cli-expected.json','p1334-ab-author.py','p1334-ab-interface-r1.py','p1334-ab-freezer.py','p1334-ab-complete-freeze.py','p1334-ab-new-native.inc']:
 f['artifacts'][n]=digest(D/n)
expected=json.loads((D/'p1334-ab-cli-expected.json').read_text())
f['utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
f['status']='Complete pre-candidate independent B freeze; native R1 only corrects public interface. No candidate/runtime/private results read. No mutation score or refinement seal.'
f['cli']={'historical_cells':712,'cells':len(expected['expectations']),'new_expressions':26,'baseline_cache_sha256':digest(D/'p1334-ab-cli-baseline.json'),'literal_expectations_sha256':digest(D/'p1334-ab-cli-expected.json'),'policy':expected['policy']}
f['limits']=['Shared filesystem without technical isolation attestation.','Historical inconsistent Some fixtures retained only as local robustness, never causal parity.','Cross-source native FileId anchors covered; real alias/import/With/spread tests currently share a Source.','No candidate execution by B.']
p=D/'p1334-ab-freeze.json';assert not p.exists()
t=json.dumps(f,indent=2,ensure_ascii=False)+'\n'
subprocess.run(['apply_patch'],input='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+l+'\n' for l in t.splitlines())+'*** End Patch\n',text=True,check=True,capture_output=True)
print(digest(p))
