from pathlib import Path
import gzip,json,hashlib,datetime,subprocess,xml.etree.ElementTree as ET,sys
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];assert phase in ['supplement-before','supplement-after'];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['hashes']};folder=out/(phase+'-pdf');folder.mkdir(exist_ok=False)
if phase=='before':assert hashes==b['hashes']
prefix='#set page(width:200pt,height:auto,margin:8pt)\n#set text(font:"DejaVu Sans",size:12pt)\n'
cases={'font18': '#set text(size:18pt)\nBEFORE#par[]#box(width:8pt,height:7pt,fill:red)[]AFTER', 'font8': '#set text(size:8pt)\nBEFORE#par[]#box(width:8pt,height:7pt,fill:red)[]AFTER', 'gap-zero': '#set par(spacing:0pt)\nBEFORE#par[]#box(width:8pt,height:7pt,fill:red)[]AFTER', 'gap-five': '#set par(spacing:5pt)\nBEFORE#par[]#box(width:8pt,height:7pt,fill:red)[]AFTER', 'gap-two-em': '#set par(spacing:2em)\nBEFORE#par[]#box(width:8pt,height:7pt,fill:red)[]AFTER', 'top-zero': '#set text(top-edge:0pt)\n#par[]#box(width:8pt,height:7pt,fill:red)[]AFTER', 'shift-only': '#par[]#box(width:8pt,height:7pt,fill:red,baseline:10pt)[]', 'block-zero-box-only': '#block(height:8pt,below:0pt)[]#box(width:8pt,height:7pt,fill:red)[]', 'block-large-box-only': '#block(height:8pt,below:20pt)[]#box(width:8pt,height:7pt,fill:red)[]', 'finite60': '#set page(height:60pt)\nBEFORE#par[]#box(width:8pt,height:7pt,fill:red)[]AFTER', 'finite70': '#set page(height:70pt)\nBEFORE#par[]#box(width:8pt,height:7pt,fill:red)[]AFTER', 'finite-initial': '#set page(height:30pt)\n#par[]#box(width:8pt,height:7pt,fill:red)[]AFTER', 'finite-shift': '#set page(height:60pt)\nBEFORE#par[]#box(width:8pt,height:7pt,fill:red,baseline:10pt)[]AFTER'}
bins={'tekt':Path('target/debug/typst').resolve(),'vanilla':Path('lab/typst-original/target/release/typst').resolve()};assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea';record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=hashes,status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},font=dict(path='/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf',sha256=sha('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf')),cases=[])
for name,body in cases.items():
 src=folder/(name+'.typ');src.write_text(prefix+body+'\n')
 for kind,binary in bins.items():
  pdf=folder/(name+'-'+kind+'.pdf');cmd=[str(binary),'compile',str(src),str(pdf),'--font-path','/usr/share/fonts/truetype/dejavu'];r=subprocess.run(cmd,capture_output=True,text=True,timeout=30);item=dict(name=name,binary=kind,utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source=str(src),source_sha256=sha(src),argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
  if r.returncode==0:
   xml=folder/(name+'-'+kind+'.bbox.html');cmd2=['pdftotext','-bbox',str(pdf),str(xml)];z=subprocess.run(cmd2,capture_output=True,text=True);assert z.returncode==0,z.stderr;tree=ET.parse(xml);pages=[]
   for page in tree.getroot().iter('{http://www.w3.org/1999/xhtml}page'):
    words=[dict(text=w.text,x0=float(w.get('xMin')),y0=float(w.get('yMin')),x1=float(w.get('xMax')),y1=float(w.get('yMax'))) for w in page.iter('{http://www.w3.org/1999/xhtml}word')];pages.append(dict(width=float(page.get('width')),height=float(page.get('height')),words=words))
   assert pages,item;item.update(pdf_sha256=sha(pdf),bbox_command=cmd2,bbox_sha256=sha(xml),pages=pages)
  record['cases'].append(item)
assert all(x['exit']==0 for x in record['cases']),[(x['name'],x['binary'],x['stderr']) for x in record['cases'] if x['exit']]
record.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources_unchanged=all(sha(p)==h for p,h in hashes.items()));assert record['sources_unchanged']
with gzip.open(out/(phase+'-measurement.json.gz'),'wt') as f:json.dump(record,f)
for name in cases:
 pair=[r for r in record['cases'] if r['name']==name];print(json.dumps(dict(name=name,observations={x['binary']:x['pages'] for x in pair})),flush=True)
