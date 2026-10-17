from pathlib import Path
import datetime,gzip,json,hashlib,subprocess,sys,itertools,xml.etree.ElementTree as ET
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];assert phase in ['morph-before','morph-after'];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['hashes']};folder=out/(phase+'-renders');folder.mkdir(exist_ok=False)
if phase=='before':assert hashes==b['hashes']
prefix='#set page(width:400pt,height:auto,margin:8pt)\n#set text(font:"DejaVu Sans",size:12pt)\n'
cases={'set-morph-correct':'#repr([#set text(size:18pt)\nA]==[A])','relative-vs-zero':'#repr(text(size:2em)[A]==text(size:0pt)[A])','relative-vs-resolved':'#repr(text(size:2em)[A]==text(size:24pt)[A])','relative-same':'#repr(text(size:2em)[A]==text(size:2em)[A])','affine-vs-absolute-part':'#repr(text(size:2em+3pt)[A]==text(size:3pt)[A])','none-body':'#text(none) Z','none-body-size':'#text(18pt,none) Z','string-first-size':'#text("A",18pt) Z','named-body-and-valid-pos':'#text([A],body:[B])','named-size-duplicate':'#text(size:18pt,size:20pt)[A]','set-duplicate-size':'#set text(size:18pt,size:2em)\nA Z','set-named-and-pos':'#set text(18pt,size:20pt)\nA Z','set-ordered-two':'#set text(size:18pt)\n#set text(size:2em)\nA Z','show-heading-relative':'#show heading: set text(size:2em)\n= A','set-palette':'#set text(red)\nA Z','named-size-early-invalid':'#text(size:1,size:18pt)[A]'}

bins={'tekt':Path('target/debug/typst').resolve(),'vanilla':Path('lab/typst-original/target/release/typst').resolve()};assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
r=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=hashes,status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},font=dict(path='/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf',sha256=sha('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf')),cases=[])
for name,body in cases.items():
 src=folder/(name+'.typ');src.write_text(prefix+body+'\n')
 for kind,bin in bins.items():
  dst=folder/(name+'-'+kind+'.pdf');cmd=[str(bin),'compile',str(src),str(dst),'--font-path','/usr/share/fonts/truetype/dejavu'];z=subprocess.run(cmd,capture_output=True,text=True,timeout=40);row=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),name=name,binary=kind,source=str(src),source_sha256=sha(src),argv=cmd,exit=z.returncode,stdout=z.stdout,stderr=z.stderr)
  if z.returncode==0:
   xml=dst.with_suffix('.bbox.html');cmd2=['pdftotext','-bbox',str(dst),str(xml)];y=subprocess.run(cmd2,capture_output=True,text=True);assert y.returncode==0;tree=ET.parse(xml);row.update(pdf_sha256=sha(dst),bbox_command=cmd2,bbox_sha256=sha(xml),pages=[dict(width=float(p.get('width')),height=float(p.get('height')),words=[dict(text=w.text,x0=float(w.get('xMin')),y0=float(w.get('yMin')),x1=float(w.get('xMax')),y1=float(w.get('yMax'))) for w in p.iter('{http://www.w3.org/1999/xhtml}word')]) for p in tree.getroot().iter('{http://www.w3.org/1999/xhtml}page')]);assert row['pages']
  r['cases'].append(row)
  print(json.dumps(dict(name=name,binary=kind,exit=row['exit'],pages=row.get('pages'),error=row['stderr'])),flush=True)
r.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources_unchanged=all(sha(p)==h for p,h in hashes.items()));assert r['sources_unchanged']
with gzip.open(out/(phase+'-measurement.json.gz'),'wt') as f:json.dump(r,f)
