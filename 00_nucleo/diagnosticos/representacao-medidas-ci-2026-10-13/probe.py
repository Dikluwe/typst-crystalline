#!/usr/bin/env python3
"""Language representation and round-trip probes; preserves exact provenance."""
from pathlib import Path
import argparse,datetime,hashlib,json,subprocess
root=Path(__file__).resolve().parents[3]
parser=argparse.ArgumentParser();parser.add_argument('output',type=Path);args=parser.parse_args()
if args.output.exists():parser.error('select a new output to preserve measurements')
def run(cmd):
 r=subprocess.run(cmd,cwd=root,capture_output=True,text=True,timeout=20);return dict(command=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
values=['1e308em','-1e308em','1e307deg','-1e307deg','1e307%','-1e307%','9007199254740992em','1.125em','-1.125em','1.23456deg','12.3456%','0em','-0em','float.inf * 1em','-float.inf * 1em','float.nan * 1em']
expressions=[(f'repr-{i}',f'repr({v})','representation') for i,v in enumerate(values)]
expressions += [(f'roundtrip-{i}',f'{{ let value = {v}; let restored = eval(repr(value), mode: "code"); (type(restored) == type(value), restored == value) }}','roundtrip') for i,v in enumerate(values[:7])]
expressions += [('large-pt-repr','repr(1e307pt)','construction-control'),('large-pt-value','1e307pt / 1pt','construction-control'),('large-em-value','1e308em / 1em','value-control')]
results=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=run(['git','rev-parse','HEAD']),status=run(['git','status','--porcelain=v1']),diffstat=run(['git','diff','HEAD','--stat']),binaries={},probes=[])
for binary in ['lab/typst-original/target/release/typst','target/debug/typst']:
 path=root/binary;sha=hashlib.sha256(path.read_bytes()).hexdigest()
 if binary.startswith('lab/') and sha!='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea':raise SystemExit('wrong oracle binary')
 results['binaries'][binary]=sha
 for name,expression,kind in expressions:
  result=run([str(path),'eval',expression,'--format','json']);results['probes'].append(dict(binary=binary,name=name,expression=expression,kind=kind,**result))
args.output.write_text(json.dumps(results,indent=2))
for name,_,kind in expressions:
 pair=[p for p in results['probes'] if p['name']==name];print(name,kind,[(p['exit'],p['stdout'].strip()[:85],p['stderr'].splitlines()[:1]) for p in pair])
