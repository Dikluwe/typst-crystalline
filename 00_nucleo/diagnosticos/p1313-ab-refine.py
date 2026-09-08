"""Prepatch addition of exact historical replay sentinels and equivalent file data."""
import json
import subprocess
from pathlib import Path
base=Path(__file__).resolve().parent
p=base/'p1313-ab-cases.json'
old=p.read_text()
doc=json.loads(old)
doc['cases'] += [dict(id='bytes-simple', kind='value', expr='csv(bytes("a,b"))'), dict(id='cast-prefix-false', kind='cast', expr='csv(delimiter: ";", false)')]
for c in doc['cases']:
    if c['kind']=='legacy-bytes':
        c['surrogate']=c['surrogate'].replace('data.csv','single.csv')
new=json.dumps(doc,ensure_ascii=False,indent=2)+'\n'
patch='*** Begin Patch\n*** Update File: '+str(p)+'\n@@\n'+''.join('-'+x+'\n' for x in old.splitlines())+''.join('+'+x+'\n' for x in new.splitlines())+'*** Add File: /tmp/p1313-ab-fixtures/single.csv\n+a,b\n*** End Patch\n'
subprocess.run(['apply_patch'],input=patch,text=True,check=True,capture_output=True)
