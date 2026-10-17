from pathlib import Path
import datetime,gzip,json,hashlib,subprocess,sys,xml.etree.ElementTree as ET
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();b=json.load(gzip.open(out/'before.json.gz'));folder=out/(phase+'-extended')
if phase=='before' and (out/'before-extended.json.gz').exists():
 previous=json.load(gzip.open(out/'before-extended.json.gz'));main=json.load(gzip.open(out/'before-measurement.json.gz'));assert previous['binaries']['tekt']['sha256']==main['binaries']['tekt']['sha256']==sha('target/debug/typst');assert all(sha(x['source'])==x['source_sha256'] for x in previous['cases']);print('Reuse immutable pre-build binary baseline, distinct source snapshot retained');sys.exit(0)
folder.mkdir(exist_ok=False)
cases={
 'two-relative-sets':'#set text(size:2em)\n#set text(size:0.5em+3pt)\nA Z',
 'set-spread':'#set text(..(18pt,red))\nA Z',
 'set-spread-named':'#set text(..(size:2em))\nA Z',
 'constructor-spread':'#text(..(2em,red,[A])) Z',
 'capture-heading':'#show heading: set text(size:2em)\n= A\nZ',
 'capture-heading-affine':'#show heading: set text(size:2em+3pt)\n= A\nZ',
 'nested-set':'#set text(size:18pt)\nA #{set text(size:2em); [B]} C',
 'nested-set-affine':'#set text(size:18pt)\nA #{set text(size:2em+3pt); [B]} C',
 'siblings':'#text(size:2em)[A] #text(size:3em)[B] C',
 'context':'#set text(size:2em)\n#context text(size:0.5em)[A] Z',
 'raw-default':'#set text(size:2em)\n`A` Z',
 'raw-override':'#show raw: set text(size:2em)\n`A` Z',
 'raw-block':'#show raw: set text(size:2em)\n```\nA\n```\nZ',
 'math-control':'#text(size:18pt)[A $x^2$ B] Z',
 'math-relative':'#text(size:2em)[A $x^2$ B] Z',
 'scripts':'#text(size:2em)[A #sub[B] #super[C]] Z',
 'box-inset':'#text(size:2em)[A #box(inset:0.5em)[B] C] Z',
 'prepared-par':'#set text(size:2em)\n#par[A B C] Z',
 'zero-relative':'#text(size:0em)[A] Z',
 'negative-relative':'#text(size:-0.5em)[A] Z',
 'bound':'#let t=text.with(size:2em)\n#t[A] Z',
 'once':'#let n=0\n#text({n+=1; 18pt})[#{n+=1; [A]}]\n#assert.eq(n,2)',
 'once-set':'#let n=0\n#set text({n+=1; 18pt})\n#assert.eq(n,1)\nA',
 'set-leftover':'#set text(18pt,20pt)\nA',
 'set-invalid-size-before-extra':'#set text(18pt,size:12)\nA',
 'dupes-last':'#let d1=(size:2em)\n#let d2=(size:3em)\n#text(..d1,..d2)[A] Z',
 'dupes-invalid-first':'#let d1=(size:12)\n#let d2=(size:2em)\n#text(..d1,..d2)[A]',
 'set-dupes-last':'#let d1=(size:2em)\n#let d2=(size:3em)\n#set text(..d1,..d2)\nA Z',
 'set-dupes-invalid-first':'#let d1=(size:12)\n#let d2=(size:2em)\n#set text(..d1,..d2)\nA',
}
cases.update({
 'scripts-abs':'#text(size:24pt)[A #sub[B] #super[C]] Z',
 'heading-default-child':'= A #text(size:2em)[B]\nZ',
 'heading-affine-child':'#show heading: set text(size:2em+3pt)\n= A #text(size:2em)[B]\nZ',
 'heading-child-absolute-default':'= A #text(size:33.6pt)[B]\nZ',
 'heading-child-absolute-affine':'#show heading: set text(size:2em+3pt)\n= A #text(size:73.2pt)[B]\nZ',
 'heading-level2':'#show heading: set text(size:2em+3pt)\n== A\nZ',
 'heading-level3':'#show heading: set text(size:2em+3pt)\n=== A\nZ',
 'heading-absolute':'#show heading: set text(size:18pt)\n= A\nZ',
 'heading-multiple':'#show heading: set text(size:2em)\n#show heading: set text(size:0.5em+3pt)\n= A\nZ',
 'heading-outer-affine':'#text(size:2em+3pt)[#heading[A]] Z',
 'heading-outer-absolute':'#text(size:18pt)[#heading[A]] Z',
 'heading-replaced':'#show heading: it => [A]\n#show heading: set text(size:2em+3pt)\n= X\nZ',
})
if phase.startswith('refuters-'):
 cases={k:v for k,v in cases.items() if k in ['scripts','scripts-abs','heading-default-child','heading-affine-child','heading-absolute','heading-multiple','heading-outer-absolute','heading-replaced','capture-heading-affine','heading-level2','heading-level3','heading-child-absolute-default','heading-child-absolute-affine']}
prefix='#set page(width:400pt,height:auto,margin:8pt)\n#set text(font:"DejaVu Sans",size:12pt)\n';bins={'tekt':Path('target/debug/typst').resolve(),'vanilla':Path('lab/typst-original/target/release/typst').resolve()};assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea';hashes={p:sha(p) for p in b['hashes']};r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=hashes,binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},cases=[])
for name,body in cases.items():
 src=folder/(name+'.typ');src.write_text(prefix+body+'\n')
 for kind,bin in bins.items():
  dst=folder/(name+'-'+kind+'.pdf');cmd=[str(bin),'compile',str(src),str(dst),'--font-path','/usr/share/fonts/truetype/dejavu'];z=subprocess.run(cmd,capture_output=True,text=True,timeout=40);row=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),name=name,binary=kind,source=str(src),source_sha256=sha(src),argv=cmd,exit=z.returncode,stdout=z.stdout,stderr=z.stderr)
  if z.returncode==0:
   xml=dst.with_suffix('.bbox.html');cmd2=['pdftotext','-bbox',str(dst),str(xml)];y=subprocess.run(cmd2,capture_output=True,text=True);assert y.returncode==0;tree=ET.parse(xml);row.update(pdf_sha256=sha(dst),bbox_command=cmd2,pages=[dict(width=float(p.get('width')),height=float(p.get('height')),words=[dict(text=w.text,x0=float(w.get('xMin')),y0=float(w.get('yMin')),x1=float(w.get('xMax')),y1=float(w.get('yMax'))) for w in p.iter('{http://www.w3.org/1999/xhtml}word')]) for p in tree.getroot().iter('{http://www.w3.org/1999/xhtml}page')])
  r['cases'].append(row);print(json.dumps(dict(name=name,binary=kind,exit=z.returncode,error=[x for x in z.stderr.splitlines() if x.startswith('error:') or 'hint:' in x],pages=row.get('pages'))),flush=True)
r.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources_unchanged=all(sha(p)==h for p,h in hashes.items()));assert r['sources_unchanged']
with gzip.open(out/(phase+'-extended.json.gz'),'wt') as f:json.dump(r,f)
