"""Pre-candidate integration of the B-owned compile-only fixture correction."""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('r',D/'p1334-record.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
i=json.loads((D/'p1334-test-integration.json').read_text());before=r.state()
assert before['product_inventory']==i['after']['product_inventory']
f=json.loads((D/'p1334-ab-native-freeze-r1.json').read_text())
assert all(r.sha(D/p)==h for p,h in {**f['inputs'],**f['artifacts']}.items())
source='01_core/src/compiler/eval/call_dispatch.rs';p=r.ROOT/source
old=p.read_text();a=(D/'p1334-ab-dispatch-tests.rs').read_text().strip()
b=(D/'p1334-ab-dispatch-tests-r1.rs').read_text().strip()
assert old.count(a)==1;new=old.replace(a,b)
subprocess.run(['apply_patch'],input='*** Begin Patch\n*** Update File: '+str(p)+'\n@@\n'+''.join('-'+x+'\n' for x in old.splitlines())+''.join('+'+x+'\n' for x in new.splitlines())+'*** End Patch\n',text=True,check=True,capture_output=True)
i.update(at=r.now(),before=before,after=r.state(),native_freeze_sha256=r.sha(D/'p1334-ab-native-freeze-r1.json'),delta='B compile-only Value::Module extraction; no expectation or runtime change')
i['owners'][source]=['p1334-ab-dispatch-tests-r1.rs']
i['frozen'].update({str(D/p):h for p,h in f['artifacts'].items()})
r.save('test-integration-r1',i)
