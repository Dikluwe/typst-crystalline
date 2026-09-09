#!/usr/bin/env python3
import pathlib,subprocess,json,hashlib,datetime
D=pathlib.Path(__file__).resolve().parent
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def add(n,t):
 p=D/n; assert not p.exists()
 subprocess.run(['apply_patch'],input='*** Begin Patch\n*** Add File: '+str(p)+'\n'+''.join('+'+x+'\n' for x in t.splitlines())+'*** End Patch\n',text=True,check=True,capture_output=True)
old='p1334-ab-dispatch-tests.rs'; new='p1334-ab-dispatch-tests-r1.rs'
t=(D/old).read_text(); before='let module = crate::compiler::stdlib::make_calc_module();'; after='let Value::Module(module) = crate::compiler::stdlib::make_calc_module() else { panic!() };'; assert t.count(before)==1
add(new,t.replace(before,after))
subprocess.run(['rustfmt','--edition','2021',str(D/new)],check=True)
f=json.loads((D/'p1334-ab-native-freeze.json').read_text())
f['utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
f['inputs']['p1334-ab-native-freeze.json']=digest(D/'p1334-ab-native-freeze.json')
f['inputs'][old]=f['artifacts'].pop(old)
f['artifacts'][new]=digest(D/new)
f['revision']={'reason':'Compiler-only E0599 public API feedback from integrator before candidate; make_calc_module returns Value. Extract Value::Module before scope lookup. No semantic output or candidate read. No expectations changed.','predecessor':'p1334-ab-native-freeze.json','fixture_before':before,'fixture_after':after}
add('p1334-ab-native-freeze-r1.json',json.dumps(f,indent=2)+'\n')
