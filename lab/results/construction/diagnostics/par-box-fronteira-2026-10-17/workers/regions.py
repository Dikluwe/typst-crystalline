from pathlib import Path
import datetime,gzip,hashlib,json,subprocess,sys,xml.etree.ElementTree as ET
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];assert phase in ['regions-before','regions-after'];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['hashes']};folder=out/(phase+'-renders');folder.mkdir(exist_ok=False)

box='#box(width:8pt,height:7pt,fill:red)[]';prefix='#set page(width:200pt,height:auto,margin:8pt)\n#set text(font:"DejaVu Sans",size:12pt)\n'
cases={'empty-break-tall':'#par[]#pagebreak()#box(width:8pt,height:45pt,fill:red)[]AFTER', 'empty-break-small':'#par[]#pagebreak()'+box, 'text-break-small':'BEFORE#pagebreak()'+box, 'break-small-text':'#pagebreak()'+box+'AFTER', 'two-breaks-small':'#pagebreak()#pagebreak()'+box}
prefix='#set page(width:200pt,height:60pt,margin:8pt)\n#set text(font:"DejaVu Sans",size:12pt)\n'

binaries={k:Path(p).resolve() for k,p in [('tekt','target/debug/typst'),('vanilla','lab/typst-original/target/release/typst')]};assert sha(binaries['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
if phase=='regions-before':assert sha(binaries['tekt'])==json.load(gzip.open(out/'before-measurement.json.gz'))['binaries']['tekt']['sha256']
r=dict(build_source_origin='before.json.gz verified unchanged CLI binary' if phase=='regions-before' else 'current validated build', binary_build_source_hashes=b['hashes'] if phase=='regions-before' else hashes,start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=hashes,status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in binaries.items()},font=dict(path='/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf',sha256=sha('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf')),cases=[])
for name,body in cases.items():
 src=folder/(name+'.typ');src.write_text(prefix+body+'\n')
 for kind,binary in binaries.items():
  item=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),name=name,binary=kind,source=str(src),source_sha256=sha(src),runs=[])
  for fmt in ['pdf','svg']:
   dst=folder/(name+'-'+kind+('-{p}' if fmt=='svg' else '')+'.'+fmt);cmd=[str(binary),'compile',str(src),str(dst),'--font-path','/usr/share/fonts/truetype/dejavu','--format',fmt];p=subprocess.run(cmd,capture_output=True,text=True,timeout=30);item['runs'].append(dict(command=cmd,exit=p.returncode,stdout=p.stdout,stderr=p.stderr,output=str(dst),sha256=sha(dst) if dst.exists() else None,svg_outputs=[str(p) for p in sorted(folder.glob(name+'-'+kind+'-*.svg'))] if fmt=='svg' else []));assert p.returncode==0,(name,kind,fmt,p.stderr)
  pdf=folder/(name+'-'+kind+'.pdf');xml=folder/(name+'-'+kind+'.bbox.html');p=subprocess.run(['pdftotext','-bbox',str(pdf),str(xml)],capture_output=True,text=True);assert p.returncode==0;tree=ET.parse(xml);item['pages']=[dict(width=float(pg.get('width')),height=float(pg.get('height')),words=[dict(text=w.text,x0=float(w.get('xMin')),y0=float(w.get('yMin')),x1=float(w.get('xMax')),y1=float(w.get('yMax'))) for w in pg.iter('{http://www.w3.org/1999/xhtml}word')]) for pg in tree.getroot().iter('{http://www.w3.org/1999/xhtml}page')];assert item['pages'];r['cases'].append(item)
r.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources_unchanged=all(sha(p)==h for p,h in hashes.items()));assert r['sources_unchanged']
with gzip.open(out/(phase+'-measurement.json.gz'),'wt') as f:json.dump(r,f)
for name in cases:print(json.dumps(dict(name=name,observations={x['binary']:x['pages'] for x in r['cases'] if x['name']==name})),flush=True)
