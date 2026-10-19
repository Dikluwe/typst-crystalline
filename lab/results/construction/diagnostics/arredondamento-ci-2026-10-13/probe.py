#!/usr/bin/env python3
"""Compare round language probes using the pinned oracle and a built Tekt."""
from pathlib import Path
import argparse,datetime,hashlib,json,subprocess
root=Path(__file__).resolve().parents[3]
parser=argparse.ArgumentParser();parser.add_argument('output',type=Path);args=parser.parse_args()
if args.output.exists():parser.error('preserve existing measurements; select a new output')
def run(cmd):
 r=subprocess.run(cmd,cwd=root,capture_output=True,text=True,timeout=20);return dict(command=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
cases=[('large-positive','1e300',2),('large-negative','-1e300',2),('near-max-positive','1e308',1),('coarse-max','1e308',-308),('coarse-positive','1.4e308',-308),('coarse-negative','-1.4e308',-308),('true-overflow','1.6e308',-308),('true-negative-overflow','-1.6e308',-308),('decimal-places','0.56553',2),('negative-decimal-places','-0.56553',2),('tie-positive','1250.0',-2),('tie-negative','-1250.0',-2),('near-zero','-0.4',0),('tiny-negative','-1e-300',-309),('tiny-positive','1e-300',-309),('nan-guard','float.nan',2),('inf-guard','float.inf',2),('inf-noop','float.inf',15),('int-coarse','1250',-2),('decimal-coarse','decimal("1250")',-2)]
for name,value in [('float','1.25'),('int','125'),('decimal','decimal("1.25")')]:
 for label,digits in [('wide-positive',4294967296),('wide-negative',-4294967296),('max',9223372036854775807),('min',-9223372036854775808)]:cases.append((name+'-'+label,value,digits))
def precision(digits):return "(-9223372036854775807 - 1)" if digits == -9223372036854775808 else str(digits)
expressions=[(name,f'repr(calc.round({value}, digits: {precision(digits)}))') for name,value,digits in cases]
expected={'large-positive':'1e300','large-negative':'-1e300','near-max-positive':'1e308','coarse-max':'1e308','coarse-positive':'1e308','coarse-negative':'-1e308','decimal-places':'0.57','negative-decimal-places':'-0.57','tie-positive':'1300.0','tie-negative':'-1300.0','int-coarse':'1300','decimal-coarse':'decimal("1300")'}
for kind,value in [('float','1.25'),('int','125'),('decimal','decimal("1.25")')]:
 for label in ['wide-positive','max']:expected[kind+'-'+label]=value
 for label in ['wide-negative','min']:expected[kind+'-'+label]={'float':'0.0','int':'0','decimal':'decimal("0")'}[kind]
for name,value,digits in cases:
 if name in expected:
  expressions.append(('semantic-'+name,f'{{ let result = calc.round({value}, digits: {precision(digits)}); (type(result) == type({value}), result == {expected[name]}) }}'))
expressions += [('wrong-digits','calc.round(1.25, digits: "x")'),('unknown-name','calc.round(1.25, other: 2)'),('wrong-arity','calc.round(1.25, 2)'),('int-overflow','calc.round(9223372036854775807, digits: -1)')]
results=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=run(['git','rev-parse','HEAD']),status=run(['git','status','--porcelain=v1']),diffstat=run(['git','diff','HEAD','--stat']),binaries={},probes=[])
for binary in ['lab/typst-original/target/release/typst','target/debug/typst']:
 path=root/binary;sha=hashlib.sha256(path.read_bytes()).hexdigest()
 if binary.startswith('lab/') and sha!='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea':raise SystemExit('wrong oracle binary')
 results['binaries'][binary]=sha
 for name,expression in expressions:
  result=run([str(path),'eval',expression,'--format','json']);results['probes'].append(dict(binary=binary,name=name,expression=expression,**result))
args.output.write_text(json.dumps(results,indent=2))
for name,_ in expressions:
 pair=[p for p in results['probes'] if p['name']==name];print(name,[(p['exit'],p['stdout'].strip()[:110],p['stderr'].splitlines()[:1]) for p in pair])
