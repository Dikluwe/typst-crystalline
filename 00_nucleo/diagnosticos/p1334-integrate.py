"""Integrate exact independently frozen snippets before candidate code."""
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('r',D/'p1334-record.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
before=r.state();r.verify(before);b=json.loads((D/'p1334-baseline.json').read_text())
f=json.loads((D/'p1334-ab-native-freeze.json').read_text())
assert all(r.sha(D/p)==h for p,h in {**f['inputs'],**f['artifacts']}.items())
norm=lambda s:re.sub(r'(?m)^//! @prompt-hash [0-9a-f]{8}\n','',s)
for prompt,source in r.PAIRS:assert norm((r.ROOT/source).read_text())==norm(b['original_files'][source]),source
pairs=[(f'p1333-ab-p{n}-successor.rs',f'p1334-ab-p{n}-successor.rs') for n in range(1328,1333)]
pairs.append(('p1333-ab-tests.rs','p1334-ab-p1333-successor.rs'))
owners={r.OWNER:[p for _,p in pairs]+['p1334-ab-tests.rs'],
    '01_core/src/compiler/eval/call_dispatch.rs':['p1334-ab-dispatch-tests.rs']}
for source,snippets in owners.items():
    p=r.ROOT/source;old=p.read_text();new=old
    if source==r.OWNER:
        for oldname,newname in pairs:
            previous=(D/oldname).read_text().strip();successor=(D/newname).read_text().strip()
            assert new.count(previous)==1,oldname
            new=new.replace(previous,successor)
        added='p1334-ab-tests.rs'
    else:added='p1334-ab-dispatch-tests.rs'
    snippet=(D/added).read_text().strip();assert snippet.startswith('#[cfg(test)]')
    new=new.rstrip()+'\n\n'+snippet+'\n'
    patch='*** Begin Patch\n*** Update File: '+str(p)+'\n@@\n'+''.join('-'+x+'\n' for x in old.splitlines())+''.join('+'+x+'\n' for x in new.splitlines())+'*** End Patch\n'
    subprocess.run(['apply_patch'],input=patch,text=True,check=True,capture_output=True)
    assert p.read_text()==new
after=r.state();r.verify(after)
r.save('test-integration',dict(at=r.now(),before=before,after=after,
    manifest_sha256=r.sha(D/'p1334-manifest.json'),native_freeze_sha256=r.sha(D/'p1334-ab-native-freeze.json'),
    frozen={str(D/p):h for p,h in f['artifacts'].items()},owners=owners,
    predecessors={str(D/p):r.sha(D/p) for p,_ in pairs},
    delta='Exact frozen successors and new calc/dispatcher tests; runtime baseline preserved except headers'))
