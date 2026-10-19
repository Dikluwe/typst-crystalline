import json, pathlib, hashlib, subprocess, datetime, re, zlib, struct
R=pathlib.Path.cwd(); D=pathlib.Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
cmd=lambda a:subprocess.check_output(a,text=True)
def state():return {'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':cmd(['git','rev-parse','HEAD']).strip(),'status':cmd(['git','status','--short']),'diffstat':cmd(['git','diff','HEAD','--stat'])}
bins={'product':R/'lab/target/product/debug/typst','oracle':R/'lab/typst-original/target/release/typst'}
assert sha(bins['oracle'])=='9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9'
r={'before':state(),'binaries':{k:{'path':str(v),'sha256':sha(v)} for k,v in bins.items()},'oracle_commit':'bd5ebec285505889e8ff1d4f68804ab147a699f3','oracle_tree':'cac836c07e4dec2a89219370428292393a3216b5','recipe':'lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md','runs':[]}
(D/'asset.svg').write_text('<svg xmlns="http://www.w3.org/2000/svg" width="40" height="20"><rect width="40" height="20" fill="green"/></svg>')
def chunk(t,d): return struct.pack('>I',len(d))+t+d+struct.pack('>I',zlib.crc32(t+d))
(D/'asset.png').write_bytes(b'\x89PNG\r\n\x1a\n'+chunk(b'IHDR',struct.pack('>IIBBBBB',40,20,8,2,0,0,0))+chunk(b'IDAT',zlib.compress((b'\0'+b'\0\x80\0'*40)*20))+chunk(b'IEND',b''))
cases={'shape':'rect(width:40pt,height:20pt,fill:green,stroke:none)','image':'image("asset.png",width:40pt,height:20pt)','equation':'$x + y$','table':'table(columns:(40pt,),inset:0pt,stroke:none,rect(width:40pt,height:20pt,fill:green,stroke:none))','figure':'figure(rect(width:40pt,height:20pt,fill:green,stroke:none),caption:none,kind:image)','empty':'[]','metadata':'metadata("invisible")','align_image':'align(left,image("asset.png",width:40pt,height:20pt))','hide_shape':'hide(rect(width:40pt,height:20pt))','styled_image':'text(size:12pt,image("asset.png",width:40pt,height:20pt))'}
for name,body in cases.items():
 for col in ['auto','40pt']:
  src='#set page(width:200pt,height:150pt,margin:0pt)\n#set text(font:"Libertinus Serif",size:10pt)\n#grid(columns:('+col+',10pt),gutter:0pt,'+body+',rect(width:10pt,height:10pt,fill:rgb("ff0000"),stroke:none))\n'
  p=D/(name+'-'+col+'.typ');p.write_text(src)
  for k,b in bins.items():
   out=p.with_suffix('.'+k+'.svg');a=[str(b),'compile',str(p),str(out)]; q=subprocess.run(a,capture_output=True,text=True)
   r['runs'].append({'name':name+'-'+col,'binary':k,'source':src,'source_sha256':sha(p),'argv':a,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit':q.returncode,'stderr':q.stderr,'output':str(out),'output_sha256':sha(out) if out.exists() else None})
for name in ['shape','empty','metadata']:
 src='#set page(width:200pt,height:150pt,margin:0pt)\n#rotate(30deg,reflow:true,'+cases[name]+')\n'
 p=D/('transform-'+name+'.typ');p.write_text(src)
 for k,b in bins.items():
  out=p.with_suffix('.'+k+'.svg');a=[str(b),'compile',str(p),str(out)];q=subprocess.run(a,capture_output=True,text=True)
  r['runs'].append({'name':'transform-'+name,'binary':k,'source':src,'source_sha256':sha(p),'argv':a,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit':q.returncode,'stderr':q.stderr,'output':str(out),'output_sha256':sha(out) if out.exists() else None})
r['after']=state();r['asset']={'source':(D/'asset.svg').read_text(),'sha256':sha(D/'asset.svg')}
(D/'png-before.json').write_text(json.dumps(r,indent=2));print([(x['name'],x['binary'],x['exit']) for x in r['runs'] if x['exit']])
