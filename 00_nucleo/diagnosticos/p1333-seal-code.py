"""Seal only reciprocal code metadata; frozen normative prompt stays identical."""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('r',D/'p1333-record.py')
r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
p=r.ROOT/r.PROMPT;s=(r.ROOT/r.OWNER).read_text();old=p.read_text()
a=hashlib.sha256(re.sub(r'(?m)^Hash do Código: [0-9a-f]{8}\n','',old).encode()).hexdigest()
assert a==json.loads((D/'p1333-manifest.json').read_text())['prompt_norm_sha256']
b=hashlib.sha256(re.sub(r'(?m)^//! @prompt-hash [0-9a-f]{8}\n','',s).encode()).hexdigest()
before=re.search(r'(?m)^Hash do Código: [0-9a-f]{8}$',old).group()
after='Hash do Código: '+b[:8]
assert before!=after
subprocess.run(['apply_patch'],input='*** Begin Patch\n*** Update File: '+str(p)+'\n@@\n-'+before+'\n+'+after+'\n*** End Patch\n',text=True,check=True)
print('norm',a,'code',b)
