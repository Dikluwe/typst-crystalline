#!/usr/bin/env python3
import pathlib,json,hashlib,datetime,subprocess
D=pathlib.Path(__file__).resolve().parent
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
f=json.loads((D/'p1334-ab-freeze.json').read_text())
for key in ['inputs','artifacts']:
 for n,h in f[key].items():assert digest(D/n)==h,n
f['inputs']['p1334-ab-freeze.json']=digest(D/'p1334-ab-freeze.json')
for n in ['p1334-ab-fixtures/origins.typ','p1334-ab-cross-cli.py','p1334-ab-cross-baseline.json','p1334-ab-cross-expected.json','p1334-ab-supplement-freeze.py']:
 f['artifacts'][n]=digest(D/n)
f['utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
f['revision']={'reason':'Pre-candidate coverage gap identified against L0: real cross-source import needed in addition to native distinct FileIds. No candidate/RED/GREEN observations read. No prior artifact or expectation changed.','predecessor':'p1334-ab-freeze.json','supplemental_cells':12,'total_cli_cells':828,'cost':'24 new reference processes (3 expressions x 4 profiles x BASE/VANILLA), one execution per cell; all existing 816 cells reused byte-for-byte.','coverage':'Imported With first named value with duplicate; imported arguments first positional extra; imported With first named surplus. Full literal diagnostics and abs trace against vanilla.'}
f['limits']=[x for x in f['limits'] if not x.startswith('Cross-source native')]
f['status']='Complete freeze R1 before candidate: native interface R1 + immutable 816-cell suite + 12-cell real cross-source supplement. No candidate/runtime/private results read; no refinement seal.'
p=D/'p1334-ab-freeze-r1.json';assert not p.exists();t=json.dumps(f,indent=2,ensure_ascii=False)+'\n'
subprocess.run(['apply_patch'],input='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+l+'\n' for l in t.splitlines())+'*** End Patch\n',text=True,check=True,capture_output=True)
print(digest(p))
