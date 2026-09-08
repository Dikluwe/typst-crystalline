"""Prepatch refinement: preserve original historical cwd and add exact P1312 sentinel."""
import json
import subprocess
from pathlib import Path
BASE=Path(__file__).resolve().parent
p=BASE/'p1314-ab-cases.json'
old=p.read_text()
doc=json.loads(old)
doc['cases'].append(dict(id='options-p1312-exact-delimiter',kind='target',expr='csv("data.csv", delimiter: "ab")',historical_p1313=False))
new=json.dumps(doc,ensure_ascii=False,indent=2)+'\n'
patch='*** Begin Patch\n*** Update File: '+str(p)+'\n@@\n'+''.join('-'+x+'\n' for x in old.splitlines())+''.join('+'+x+'\n' for x in new.splitlines())+'*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,text=True,check=True,capture_output=True)
print(json.dumps(dict(cases=len(doc['cases']),reason='Original cwd for all 144 immutable historical expressions; exact P1312 delimiter source expression added; no observed oracle failure or candidate inspected.')))
