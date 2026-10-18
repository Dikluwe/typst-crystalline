from pathlib import Path
import json,gzip,xml.etree.ElementTree as ET,re,datetime
p=Path(__file__).resolve().parent
identity=(1.,0.,0.,1.,0.,0.)
def mul(a,b):
 return (a[0]*b[0]+a[2]*b[1],a[1]*b[0]+a[3]*b[1],a[0]*b[2]+a[2]*b[3],a[1]*b[2]+a[3]*b[3],a[0]*b[4]+a[2]*b[5]+a[4],a[1]*b[4]+a[3]*b[5]+a[5])
def transform(t):
 m=identity
 for name,args in re.findall(r'(matrix|translate|scale)\(([^)]*)\)',t):
  v=[float(x) for x in re.split(r'[ ,]+',args.strip())]
  if name=='matrix':n=tuple(v)
  elif name=='translate':n=(1.,0.,0.,1.,v[0],v[1] if len(v)>1 else 0.)
  else:n=(v[0],0.,0.,v[1] if len(v)>1 else v[0],0.,0.)
  m=mul(m,n)
 return m
def glyphs(f):
 out=[]
 def walk(e,m):
  m=mul(m,transform(e.attrib.get('transform','')))
  if e.tag.rsplit('}',1)[-1]=='use':
   x=float(e.attrib.get('x',0));y=float(e.attrib.get('y',0));out.append([m[0]*x+m[2]*y+m[4],m[1]*x+m[3]*y+m[5]])
  if e.tag.rsplit('}',1)[-1]!='defs':
   for c in e:walk(c,m)
 walk(ET.parse(f).getroot(),identity);return out
rows=[]
for label in ['first-after','first-bib-after']:
 d=json.loads(gzip.decompress((p/(label+'.json.gz')).read_bytes()))
 for c in d['cases']:
  a=glyphs(p/'outputs'/(c['label']+'-tekt.svg'));b=glyphs(p/'outputs'/(c['label']+'-vanilla.svg'));row=dict(label=c['label'],glyph_count_equal=len(a)==len(b),glyph_count=[len(a),len(b)],tekt=a,vanilla=b)
  if len(a)==len(b):row.update(max_x=max([abs(x[0]-y[0]) for x,y in zip(a,b)],default=0),max_y=max([abs(x[1]-y[1]) for x,y in zip(a,b)],default=0))
  rows.append(row);print({k:v for k,v in row.items() if k not in ['tekt','vanilla']})
(p/'audit-candidate1.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),method='Compose SVG ancestor transforms and use x/y; ordered glyph positions, independent of PDF descriptors and glyph IDs.',rows=rows),indent=2)+'\n')
