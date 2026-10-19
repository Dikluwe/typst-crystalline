#!/usr/bin/env python3
"""Repeat the exact PNG language corpus into a fresh directory; compare geometry."""
import pathlib,sys,subprocess,json,datetime,hashlib,xml.etree.ElementTree as E,re
D=pathlib.Path(__file__).resolve().parent; R=pathlib.Path.cwd(); label=sys.argv[1] if len(sys.argv)>1 else 'after'
O=D/label;O.mkdir(exist_ok=False)
base=json.loads((D/'probes/png-before.json').read_text())
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
cmd=lambda a:subprocess.check_output(a,text=True)
def state():return {'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':cmd(['git','rev-parse','HEAD']),'status':cmd(['git','status','--short']),'diffstat':cmd(['git','diff','HEAD','--stat'])}
def affine(a,b):
 return (a[0]*b[0]+a[2]*b[1],a[1]*b[0]+a[3]*b[1],a[0]*b[2]+a[2]*b[3],a[1]*b[2]+a[3]*b[3],a[0]*b[4]+a[2]*b[5]+a[4],a[1]*b[4]+a[3]*b[5]+a[5])
def transform_boxes(root):
 boxes=[]
 def walk(e,parent):
  mat=parent
  for op,args in re.findall(r'(matrix|translate|scale)\(([^)]*)\)',e.get('transform','')):
   v=[float(z) for z in re.split(r'[,\s]+',args.strip())]
   t=tuple(v) if op=='matrix' else (1,0,0,1,v[0],v[1] if len(v)>1 else 0) if op=='translate' else (v[0],0,0,v[1] if len(v)>1 else v[0],0,0)
   mat=affine(mat,t)
  if e.get('fill')=='#2ecc40':
   if e.tag.endswith('rect'):x=float(e.get('x','0'));y=float(e.get('y','0'));w=float(e.get('width'));h=float(e.get('height'))
   else:
    m=re.fullmatch(r'M ([\d.-]+) ([\d.-]+)v ([\d.-]+)h ([\d.-]+)v [-\d.]+Z\s*',e.get('d',''))
    if not m:raise ValueError('unsupported green shape: '+str(e.attrib))
    x,y,h,w=map(float,m.groups())
   pts=[(mat[0]*a+mat[2]*b+mat[4],mat[1]*a+mat[3]*b+mat[5]) for a,b in [(x,y),(x+w,y),(x,y+h),(x+w,y+h)]]
   boxes.append([min(a for a,b in pts),min(b for a,b in pts),max(a for a,b in pts),max(b for a,b in pts)])
  for child in e:walk(child,mat)
 walk(root,(1,0,0,1,0,0));return boxes
r={'before':state(),'runs':[],'oracle_commit':base['oracle_commit'],'oracle_tree':base['oracle_tree'],'recipe':base['recipe'],'binary_source_receipt':'coordinator final build receipt required; source freeze must match this run'}
bins={'product':R/'lab/target/product/debug/typst','oracle':R/'lab/typst-original/target/release/typst'}
r['binaries']={k:{'path':str(p),'sha256':sha(p)} for k,p in bins.items()}
assert r['binaries']['oracle']['sha256']==base['binaries']['oracle']['sha256']
(O/'asset.png').write_bytes((D/'probes/asset.png').read_bytes())
sources={x['name']:x['source'] for x in base['runs']}
for name in ['image','shape','equation','figure']:
 body={'image':'image("asset.png",width:40pt,height:20pt)','shape':'rect(width:40pt,height:20pt,fill:green,stroke:none)','equation':'$x + y$','figure':'figure(rect(width:40pt,height:20pt),caption:none,kind:image)'}[name]
 sources['effect-'+name]='#set page(width:200pt,height:150pt,margin:0pt)\n#let tally=state("effect-'+name+'",0)\n#grid(columns:(auto,10pt),gutter:0pt,[#tally.update(n=>n+1)'+('' if name=='equation' else '#')+body+'],rect(width:10pt,height:10pt,fill:rgb("ff0000"),stroke:none))\n#context assert(tally.final()==1,message:"effect repeated")\n'
for name,source in sources.items():
 p=O/(name+'.typ');p.write_text(source)
 for k,b in bins.items():
  out=O/(name+'.'+k+'.svg');a=[str(b),'compile',str(p),str(out)];q=subprocess.run(a,text=True,capture_output=True)
  x={'name':name,'binary':k,'source':source,'source_sha256':sha(p),'argv':a,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit':q.returncode,'stderr':q.stderr,'output':str(out),'sha256':sha(out) if out.exists() else None}
  if q.returncode==0:
   root=E.parse(out).getroot()
   if name.startswith('transform-'):x['green_boxes']=transform_boxes(root)
   for e in root.iter():
    if e.get('fill')=='#ff0000':
     m=re.search(r'translate\(([-.\d]+)',e.get('transform',''))
     x['marker_x']=float(e.get('x','0'))+(float(m[1]) if m else 0)
  r['runs'].append(x)
r['after']=state()
pairs={name:{x['binary']:x for x in r['runs'] if x['name']==name} for name in sources}
checks=[]
for name,pair in pairs.items():
 p,o=pair['product'],pair['oracle']; ok=p['exit']==o['exit']==0
 if 'marker_x' in p and 'marker_x' in o:ok=ok and abs(p['marker_x']-o['marker_x'])<1e-6
 if name.startswith('transform-') and ok:
  pb,ob=p['green_boxes'],o['green_boxes'];ok=len(pb)==len(ob) and all(abs(a-b)<1e-6 for pbox,obox in zip(pb,ob) for a,b in zip(pbox,obox))
 checks.append({'name':name,'pass':ok,'product_x':p.get('marker_x'),'oracle_x':o.get('marker_x')})
r['checks']=checks;(O/'receipt.json').write_text(json.dumps(r,indent=2))
print(json.dumps(checks,indent=2));sys.exit(0 if all(x['pass'] for x in checks) else 1)
