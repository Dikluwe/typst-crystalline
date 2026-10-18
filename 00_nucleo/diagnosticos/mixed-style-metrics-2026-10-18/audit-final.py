from pathlib import Path
import json,gzip,datetime,xml.etree.ElementTree as ET,re,collections
p=Path(__file__).resolve().parent;q=p.parent/'bibliography-hanging-2026-10-18'
read=lambda d,n:json.loads(gzip.decompress((d/(n+'.json.gz')).read_bytes()))
def pdf(c):
 a=c['results']['tekt'];b=c['results']['vanilla']
 if 'pdf' in a:a=a['pdf'];b=b['pdf']
 out=dict(compile_ok=a['exit']==b['exit']==0)
 if not out['compile_ok']:return out
 pa=a['pages'];pb=b['pages'];wa=[(i,w) for i,page in enumerate(pa) for w in page['words']];wb=[(i,w) for i,page in enumerate(pb) for w in page['words']]
 out.update(words_equal=[w['text'] for _,w in wa]==[w['text'] for _,w in wb],page_count=[len(pa),len(pb)])
 out['page_sizes_equal']=len(pa)==len(pb) and all(abs(a[k]-b[k])<=.001 for a,b in zip(pa,pb) for k in ['width','height'])
 if out['words_equal']:
  out['word_pages_equal']=[i for i,w in wa]==[i for i,w in wb]
  for name,indices in [('bbox',[0,1,2,3]),('x',[0,2]),('y',[1,3])]:out[name+'_max_error']=max([abs(a['bounds'][k]-b['bounds'][k]) for (_,a),(_,b) in zip(wa,wb) for k in indices],default=0)
  out['first_difference']=next((dict(tekt=dict(page=i,**a),vanilla=dict(page=j,**b)) for (i,a),(j,b) in zip(wa,wb) if i!=j or max(abs(x-y) for x,y in zip(a['bounds'],b['bounds']))>.001),None)
 out['full_geometry_equal']=out['words_equal'] and out['page_sizes_equal'] and out.get('word_pages_equal',False) and out.get('bbox_max_error',float('inf'))<=.001
 return out
rows=[]
groups=[(p,'before','after','generic'),(p,'bib-before','bib-after','bibliography'),(p,'regions-before','regions-after','regions'),(q,'after','corpus-after','hanging'),(q,'extra-after','corpus-extra-after','hanging'),(q,'boundary-after','corpus-boundary-after','hanging'),(q,'repeat-after','corpus-repeat-after','hanging'),(q,'controls-after','controls-after','margins'),(q,'controls-extra-after','controls-extra-after','margins')]
for base,before,after,group in groups:
 b=read(base,before);a=read(p,after);assert [c['source_sha256'] for c in b['cases']]==[c['source_sha256'] for c in a['cases']]
 for old,new in zip(b['cases'],a['cases']):
  row=dict(label=new['label'],group=group,before=pdf(old),after=pdf(new));rows.append(row)
  assert row['after']['compile_ok'],row
  assert not row['before']['full_geometry_equal'] or row['after']['full_geometry_equal'],row
identity=(1.,0.,0.,1.,0.,0.)
def mul(a,b):return (a[0]*b[0]+a[2]*b[1],a[1]*b[0]+a[3]*b[1],a[0]*b[2]+a[2]*b[3],a[1]*b[2]+a[3]*b[3],a[0]*b[4]+a[2]*b[5]+a[4],a[1]*b[4]+a[3]*b[5]+a[5])
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
svg=[];oldsvg={r['label']:r for r in json.loads((p/'audit-before.json').read_text())['rows']}
for label in ['after','bib-after']:
 for c in read(p,label)['cases']:
  a=glyphs(p/'outputs'/(c['label']+'-tekt.svg'));b=glyphs(p/'outputs'/(c['label']+'-vanilla.svg'));row=dict(label=c['label'],glyph_count_equal=len(a)==len(b),glyph_count=[len(a),len(b)],tekt=a,vanilla=b,before={k:v for k,v in oldsvg[c['label']].items() if k not in ['tekt','vanilla']})
  if len(a)==len(b):row.update(max_x=max([abs(x[0]-y[0]) for x,y in zip(a,b)],default=0),max_y=max([abs(x[1]-y[1]) for x,y in zip(a,b)],default=0))
  svg.append(row)
deco=[]
for old,new in zip(read(q,'deco-after')['cases'],read(p,'deco-after')['cases']):
 assert old['source_sha256']==new['source_sha256']
 a=old['results']['tekt'];b=new['results']['tekt'];assert a['lines']==b['lines'],new['label']
 assert a['artifact_sha256']==b['artifact_sha256'],new['label']
 deco.append(dict(label=new['label'],origins_retained=True,tekt_svg_unchanged=True))
empty=read(p,'deco-empty-after')['cases'];assert len(empty)==9 and all(v['stroke_count']==0 for c in empty for v in c['results'].values())
summary={}
for group in dict.fromkeys(r['group'] for r in rows):
 rr=[r for r in rows if r['group']==group];summary[group]=dict(pairs=len(rr),full_before=sum(r['before']['full_geometry_equal'] for r in rr),full_after=sum(r['after']['full_geometry_equal'] for r in rr),open=[r['label'] for r in rr if not r['after']['full_geometry_equal']])
result=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),tolerance_pt=.001,pdf_pairs=len(rows),svg_pairs=len(svg),decorations=len(deco),empty_decorations=len(empty),summary=summary,rows=rows,svg=svg,deco=deco)
(p/'audit-final.json.gz').write_bytes(gzip.compress(json.dumps(result,indent=2).encode()))
print(json.dumps({k:v for k,v in result.items() if k not in ['rows','svg','deco']},indent=2))
for r in svg:print(r['label'],{k:r.get(k) for k in ['glyph_count_equal','max_x','max_y']})
