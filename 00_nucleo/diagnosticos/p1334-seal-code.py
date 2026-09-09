"""Update only reciprocal code metadata, never the frozen normative text."""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
def load(n):
    spec=importlib.util.spec_from_file_location(n,D/(n+'.py'))
    m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
r=load('p1334-record');l=load('p1334-lineage-lib')
m=json.loads((D/'p1334-manifest.json').read_text())
for owner in m['owners']:
    h=l.hashes(owner['prompt'],owner['source'])
    assert h['norm_sha256']==owner['norm_sha256'] and h['effective_a']==owner['effective_a']
    old='Hash do Código: '+h['recorded_b'];new='Hash do Código: '+h['code_b'][:8]
    if old!=new:
        subprocess.run(['apply_patch'],input='*** Begin Patch\n*** Update File: '+owner['prompt']+'\n@@\n-'+old+'\n+'+new+'\n*** End Patch\n',text=True,check=True)
    print(owner['source'],h['effective_a'],h['code_b'])
