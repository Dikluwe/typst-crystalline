from pathlib import Path
import datetime,gzip,json,re,math,sys,hashlib,subprocess,xml.etree.ElementTree as ET
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];data=json.load(gzip.open(out/(phase+'-measurement.json.gz')));identity=(1.,0.,0.,1.,0.,0.)
def multiply(a,b):return (a[0]*b[0]+a[2]*b[1],a[1]*b[0]+a[3]*b[1],a[0]*b[2]+a[2]*b[3],a[1]*b[2]+a[3]*b[3],a[0]*b[4]+a[2]*b[5]+a[4],a[1]*b[4]+a[3]*b[5]+a[5])
def transform(s):
 m=identity
 for name,args in re.findall(r'(\w+)\s*\(([^)]*)\)',s or ''):
  v=[float(x) for x in re.split(r'[,\s]+',args.strip()) if x]
  if name=='matrix':assert len(v)==6;n=tuple(v)
  elif name=='translate':n=(1.,0.,0.,1.,v[0],v[1] if len(v)>1 else 0.)
  elif name=='scale':n=(v[0],0.,0.,v[1] if len(v)>1 else v[0],0.,0.)
  elif name=='rotate':
   angle=math.radians(v[0]);n=(math.cos(angle),math.sin(angle),-math.sin(angle),math.cos(angle),0.,0.)
   if len(v)==3:n=multiply(multiply((1.,0.,0.,1.,v[1],v[2]),n),(1.,0.,0.,1.,-v[1],-v[2]))
  else:raise ValueError((name,args))
  m=multiply(m,n)
 return m
def points(d):
 tokens=re.findall(r'[MmLlHhVvZz]|[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?',d);assert not re.sub(r'[\s,]+','',re.sub(r'[MmLlHhVvZz]|[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?','',d)),d
 i=0;command=None;x=y=0.;start=(0.,0.);rows=[]
 while i<len(tokens):
  if tokens[i].isalpha():command=tokens[i];i+=1
  assert command
  op=command.upper();relative=command.islower()
  if op=='Z':x,y=start;rows.append((x,y));command=None;continue
  if op in ['M','L']:
   nx,ny=float(tokens[i]),float(tokens[i+1]);i+=2;x,y=(x+nx,y+ny) if relative else (nx,ny)
   if op=='M':start=(x,y);command='l' if relative else 'L'
  elif op=='H':nx=float(tokens[i]);i+=1;x=x+nx if relative else nx
  elif op=='V':ny=float(tokens[i]);i+=1;y=y+ny if relative else ny
  else:raise ValueError(op)
  rows.append((x,y))
 return rows
for row in data['cases']:
 pdf=next(x['output'] for x in row['runs'] if x['output'].endswith('.pdf'));svgs=[];row['pdf_vector_pages']=[]
 for page in range(1,len(row['pages'])+1):
  svg=str(Path(pdf).with_suffix(''))+'-pdf-page-'+str(page)+'.svg';cmd=['pdftocairo','-f',str(page),'-l',str(page),'-svg',pdf,svg];result=subprocess.run(cmd,capture_output=True,text=True);assert result.returncode==0,result.stderr;svgs.append(svg);row['pdf_vector_pages'].append(dict(command=cmd,sha256=hashlib.sha256(Path(svg).read_bytes()).hexdigest()))
 bounds=[];root=None
 def visit(node,parent,fill):
  if node.tag.rsplit('}',1)[-1] in ['defs','clipPath','mask']:return
  matrix=multiply(parent,transform(node.get('transform')));fill=node.get('fill',fill);tag=node.tag.rsplit('}',1)[-1]
  red=fill=='#ff4136'
  if fill and fill.startswith('rgb('):
   components=[float(x)/100 for x in re.findall(r'([0-9.]+)%',fill)]
   red=len(components)==3 and all(abs(a-b)<0.001 for a,b in zip(components,[1.,65/255,54/255]))
  if red:
   if tag=='rect':
    x,y,w,h=[float(node.get(k,'0')) for k in ['x','y','width','height']];p=[(x,y),(x+w,y),(x+w,y+h),(x,y+h)]
   elif tag=='path':p=points(node.get('d',''))
   else:p=[]
   if p:
    world=[(matrix[0]*x+matrix[2]*y+matrix[4],matrix[1]*x+matrix[3]*y+matrix[5]) for x,y in p];bounds.append(dict(x0=min(x for x,y in world),y0=min(y for x,y in world),x1=max(x for x,y in world),y1=max(y for x,y in world)))
  for child in node:visit(child,matrix,fill)
 for page,svg in enumerate(svgs,1):
  root=ET.parse(svg).getroot();start=len(bounds);visit(root,identity,None)
  for bnd in bounds[start:]:bnd['page']=page
 row['red_bounds']=bounds;row['svg_viewbox']=root.get('viewBox')
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),method='Every PDF physical page converted by pdftocairo -svg -f/-l; red path vertices transformed by ancestor matrices, rgb percentages matched to #ff4136 within 0.001 channel. Conversion argv and SHA recorded per page. Native multi-page SVG output is archived separately; its CLI template behavior is not used as geometry evidence. PDF pages/word coordinates from pdftotext.',source_measurement=phase+'-measurement.json.gz',cases=data['cases']);(out/(phase+'-analysis.json')).write_text(json.dumps(record,indent=2)+'\n')
for name in dict.fromkeys(x['name'] for x in data['cases']):print(json.dumps(dict(name=name,values={x['binary']:dict(red=x['red_bounds'],height=x['pages'][0]['height'],words=[(w['text'],w['y0']) for w in x['pages'][0]['words']]) for x in data['cases'] if x['name']==name})),flush=True)
