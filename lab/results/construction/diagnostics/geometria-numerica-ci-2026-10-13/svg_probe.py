from pathlib import Path
import json,subprocess,hashlib,datetime,argparse
root=Path(__file__).resolve().parents[3];out=Path(__file__).resolve().parent
parser=argparse.ArgumentParser();parser.add_argument('label');args=parser.parse_args();dest=out/args.label;dest.mkdir(exist_ok=False)
base='#set page(width: 200pt, height: 140pt, margin: 12pt)\n'
cases={
'lines':'#line(length: 50pt, angle: 15deg)\n#parbreak()\n#line(length: 50pt, angle: -30deg)\n#parbreak()\n#line(length: 40pt, angle: 90deg)',
'rotate':'#rotate(30deg, reflow: true)[#rect(width: 40pt, height: 20pt, fill: red)]',
'skew':'#skew(ax: 15deg, ay: 45deg)[#rect(width: 40pt, height: 20pt, fill: blue)]',
'cancel':'$ cancel(x+y, cross: #true) + cancel(1/2, inverted: #true) $',
'optimized':'#set text(lang: "pt", hyphenate: true)\n#set par(justify: true, linebreaks: "optimized")\nA representação geométrica mantém a organização dos elementos e permite verificar palavras, espaçamentos e transformações.',
}
def run(cmd):
 r=subprocess.run(cmd,cwd=root,capture_output=True,text=True,timeout=30);return dict(command=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
binary=root/'target/debug/typst';results=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=run(['git','rev-parse','HEAD']),status=run(['git','status','--porcelain=v1']),diffstat=run(['git','diff','HEAD','--stat']),binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),cases=[])
fixtures=out/'fixtures';fixtures.mkdir(exist_ok=True)
for name,body in cases.items():
 src=fixtures/(name+'.typ');text=base+body+'\n'
 if src.exists():assert src.read_text()==text
 else:src.write_text(text)
 result=run([str(binary),'compile','--format','svg',str(src),str(dest/(name+'.svg'))]);results['cases'].append(dict(name=name,source=text,**result));print(name,result['exit'],result['stderr'][:200],flush=True)
(dest/'results.json').write_text(json.dumps(results,indent=2));assert all(x['exit']==0 for x in results['cases'])
