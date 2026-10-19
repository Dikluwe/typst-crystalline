#!/usr/bin/env python3
"""Bilateral geometry/effect replay against an explicit frozen-build receipt."""
import argparse,datetime,hashlib,json,math,pathlib,re,subprocess,sys,xml.etree.ElementTree as E
P=argparse.ArgumentParser();P.add_argument('label');P.add_argument('--build-receipt',type=pathlib.Path,required=True);args=P.parse_args()
D=pathlib.Path(__file__).resolve().parent;R=pathlib.Path.cwd();receipt=args.build_receipt.resolve();assert receipt.is_file(),receipt
O=D/args.label;O.mkdir(exist_ok=False)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
cmd=lambda a:subprocess.check_output(a,text=True,timeout=30)
def state():return {'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':cmd(['git','rev-parse','HEAD']),'status':cmd(['git','status','--short']),'diffstat':cmd(['git','diff','HEAD','--stat'])}
cases=json.loads((D.parent.parent/'cases.json').read_text());scope=set()
for case in cases:
 a=case['analysis'];scope.add(a['file']);scope.add(a['owner'])
scope.update(['01_core/src/entities/content.rs','01_core/src/entities/value.rs','01_core/src/compiler/layout/transform.rs','01_core/src/compiler/layout/grid/tracks.rs','01_core/src/compiler/layout/equation.rs','01_core/src/compiler/layout/figure.rs','01_core/src/compiler/layout/sub_frame.rs'])
def hashes():return {p:sha(R/p) for p in sorted(scope)}
def affine(a,b):
 return (a[0]*b[0]+a[2]*b[1],a[1]*b[0]+a[3]*b[1],a[0]*b[2]+a[2]*b[3],a[1]*b[2]+a[3]*b[3],a[0]*b[4]+a[2]*b[5]+a[4],a[1]*b[4]+a[3]*b[5]+a[5])
def colored_boxes(root):
 boxes={'#ff0000':[],'#2ecc40':[]}
 def walk(e,parent,fill):
  mat=parent;raw=e.get('transform','');parsed=[]
  for match in re.finditer(r'(matrix|translate|scale|rotate)\(([^)]*)\)',raw):
   op,arg=match.groups();parsed.append(match.group());v=[float(z) for z in re.split(r'[,\s]+',arg.strip())]
   if op=='matrix':assert len(v)==6;t=tuple(v)
   elif op=='translate':assert len(v) in [1,2];t=(1,0,0,1,v[0],v[1] if len(v)>1 else 0)
   elif op=='scale':assert len(v) in [1,2];t=(v[0],0,0,v[1] if len(v)>1 else v[0],0,0)
   else:
    assert len(v) in [1,3];angle=math.radians(v[0]);t=(math.cos(angle),math.sin(angle),-math.sin(angle),math.cos(angle),0,0)
    if len(v)==3:t=affine(affine((1,0,0,1,v[1],v[2]),t),(1,0,0,1,-v[1],-v[2]))
   mat=affine(mat,t)
  assert re.sub(r'\s+','',''.join(parsed))==re.sub(r'\s+','',raw),'unparsed transform '+raw
  fill=e.get('fill',fill);tag=e.tag.rsplit('}',1)[-1]
  if fill in boxes and tag in ['rect','path']:
   if tag=='rect':x=float(e.get('x','0'));y=float(e.get('y','0'));w=float(e.get('width'));h=float(e.get('height'))
   else:
    m=re.fullmatch(r'M ([\d.-]+) ([\d.-]+)v ([\d.-]+)h ([\d.-]+)v [-\d.]+Z\s*',e.get('d',''))
    assert m,'unsupported colored geometry '+str(e.attrib)
    x,y,h,w=map(float,m.groups())
   pts=[(mat[0]*a+mat[2]*b+mat[4],mat[1]*a+mat[3]*b+mat[5]) for a,b in [(x,y),(x+w,y),(x,y+h),(x+w,y+h)]]
   boxes[fill].append([min(a for a,b in pts),min(b for a,b in pts),max(a for a,b in pts),max(b for a,b in pts)])
  if tag not in ['defs','symbol']:
   for child in e:walk(child,mat,fill)
 walk(root,(1,0,0,1,0,0),None);return boxes
base=json.loads((D/'probes/png-before.json').read_text())
bins={'product':R/'lab/target/product/debug/typst','oracle':R/'lab/typst-original/target/release/typst'}
r={'before':state(),'scope_before':hashes(),'runs':[],'oracle_commit':base['oracle_commit'],'oracle_tree':base['oracle_tree'],'recipe':base['recipe'],'build_receipt':{'path':str(receipt),'sha256':sha(receipt)},'runner':{'path':str(pathlib.Path(__file__).resolve()),'sha256':sha(pathlib.Path(__file__))}}
r['binaries']={k:{'path':str(p),'sha256':sha(p)} for k,p in bins.items()};assert r['binaries']['oracle']['sha256']==base['binaries']['oracle']['sha256']
(O/'runner.py').write_bytes(pathlib.Path(__file__).read_bytes());(O/'asset.png').write_bytes((D/'probes/asset.png').read_bytes())
r['asset_sha256']=sha(O/'asset.png')
sources={x['name']:x['source'] for x in base['runs']}
for name in ['image','shape','equation','figure']:
 body={'image':'image("asset.png",width:40pt,height:20pt)','shape':'rect(width:40pt,height:20pt,fill:green,stroke:none)','equation':'$x + y$','figure':'figure(rect(width:40pt,height:20pt),caption:none,kind:image)'}[name]
 sources['effect-'+name]='#set page(width:200pt,height:150pt,margin:0pt)\n#let tally=state("effect-'+name+'",0)\n#grid(columns:(auto,10pt),gutter:0pt,[#tally.update(n=>n+1)'+('' if name=='equation' else '#')+body+'],rect(width:10pt,height:10pt,fill:rgb("ff0000"),stroke:none))\n#context assert(tally.final()==1,message:"effect repeated")\n'
for name,source in sources.items():
 p=O/(name+'.typ');p.write_text(source)
 for k,b in bins.items():
  out=O/(name+'.'+k+'.svg');a=[str(b),'compile',str(p),str(out)];x={'name':name,'binary':k,'source':source,'source_sha256':sha(p),'argv':a,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'timeout_seconds':60,'output':str(out)}
  try:
   q=subprocess.run(a,text=True,capture_output=True,timeout=60);x.update(exit=q.returncode,stderr=q.stderr,stdout=q.stdout,timed_out=False)
  except subprocess.TimeoutExpired as err:x.update(exit=None,stderr=str(err.stderr),stdout=str(err.stdout),timed_out=True)
  x['output_sha256']=sha(out) if out.exists() else None
  if x['exit']==0:
   try:x['colored_boxes']=colored_boxes(E.parse(out).getroot())
   except Exception as err:x['geometry_error']=str(err)
  r['runs'].append(x)
r['after']=state();r['scope_after']=hashes();r['binary_hashes_after']={k:sha(p) for k,p in bins.items()};r['build_receipt_hash_after']=sha(receipt)
r['frozen']=r['scope_before']==r['scope_after'] and all(r['binaries'][k]['sha256']==h for k,h in r['binary_hashes_after'].items()) and r['build_receipt']['sha256']==r['build_receipt_hash_after']
pairs={name:{x['binary']:x for x in r['runs'] if x['name']==name} for name in sources};checks=[]
for name,pair in pairs.items():
 p,o=pair['product'],pair['oracle'];ok=p['exit']==o['exit']==0 and 'geometry_error' not in p and 'geometry_error' not in o
 pb=p.get('colored_boxes',{});ob=o.get('colored_boxes',{})
 if name.startswith('transform-'):
  expected=1 if name=='transform-shape' else 0;a=pb.get('#2ecc40',[]);b=ob.get('#2ecc40',[])
  ok=ok and len(a)==len(b)==expected and all(abs(x-y)<1e-6 for ab,bb in zip(a,b) for x,y in zip(ab,bb))
  check={'name':name,'pass':ok,'product_green':a,'oracle_green':b}
 else:
  a=pb.get('#ff0000',[]);b=ob.get('#ff0000',[]);ok=ok and len(a)==len(b)==1
  if ok:ok=abs(a[0][0]-b[0][0])<1e-6
  check={'name':name,'pass':ok,'product_marker':a,'oracle_marker':b}
 checks.append(check)
r['checks']=checks;(O/'receipt.json').write_text(json.dumps(r,indent=2));print(json.dumps({'frozen':r['frozen'],'checks':checks},indent=2));sys.exit(0 if r['frozen'] and all(x['pass'] for x in checks) else 1)
