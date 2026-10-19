from pathlib import Path
import datetime,gzip,hashlib,json,re,subprocess,xml.etree.ElementTree as ET
root=Path.cwd();out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items());folder=out/'svg';folder.mkdir(exist_ok=False)
prefix='#set page(width:100pt,height:100pt,margin:0pt)\n';red='rgb("#ff0000")';blue='rgb("#0000ff")';base=f'columns: (20pt,20pt), rows: (20pt,), inset: 0pt, stroke: 1pt+{red}'
cases={'base':prefix+f'#table({base}, [],[])','erase-top':prefix+f'#table({base}, table.hline(stroke:none), [],[])','blue-top':prefix+f'#table({base},table.hline(stroke:1pt+{blue}),[],[])','erase-blue':prefix+f'#table({base},table.hline(stroke:1pt+{blue}),table.hline(stroke:none),[],[])','erase-half':prefix+f'#table({base},table.hline(stroke:1pt+{blue}),table.hline(start:0,end:1,stroke:none),[],[])'}
num=r'[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?';pattern=re.compile(r'^\s*M\s*('+num+r')[ ,]+('+num+r')\s*L\s*('+num+r')[ ,]+('+num+r')\s*$')
record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=b['hashes'],binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in {'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'}.items()},cases=[])
assert record['binaries']['vanilla']['sha256']=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
for name,text in cases.items():
 src=folder/(name+'.typ');src.write_text(text+'\n')
 for kind,info in record['binaries'].items():
  dst=folder/(name+'-'+kind+'.svg');cmd=[info['path'],'compile',str(src),str(dst),'--format','svg'];r=subprocess.run(cmd,capture_output=True,text=True,timeout=30);item=dict(name=name,binary=kind,utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source=str(src),source_sha256=sha(src),argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
  if dst.exists():
   tree=ET.parse(dst);paths=[];lines=[]
   for node in tree.getroot().iter():
    tag=node.tag.split('}')[-1]
    if tag=='path':
     paths.append(dict(node.attrib));m=pattern.fullmatch(node.get('d',''))
     if m:
      x0,y0,x1,y1=map(float,m.groups());lines.append(dict(stroke=node.get('stroke'),horizontal=y0==y1,length=((x1-x0)**2+(y1-y0)**2)**.5,coords=[x0,y0,x1,y1]))
   item.update(svg=str(dst),svg_sha256=sha(dst),paths=paths,straight_lines=lines,horizontal_by_color={color:sum(x['length'] for x in lines if x['horizontal'] and x['stroke']==color) for color in ['#ff0000','#0000ff']})
  record['cases'].append(item)
record.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources_unchanged=all(sha(p)==h for p,h in b['hashes'].items()));assert record['sources_unchanged'];assert all(x['exit']==0 for x in record['cases'])
with gzip.open(out/'svg.json.gz','wt') as f:json.dump(record,f)
for x in record['cases']:print(json.dumps(dict(name=x['name'],binary=x['binary'],horizontal=x.get('horizontal_by_color'),paths=len(x.get('paths',[])),parsed=len(x.get('straight_lines',[])))),flush=True)
