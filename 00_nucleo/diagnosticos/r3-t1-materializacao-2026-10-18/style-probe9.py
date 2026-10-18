from pathlib import Path
import json,gzip,subprocess,datetime,hashlib,sys,xml.etree.ElementTree as ET
p=Path(__file__).resolve().parent;r=p.parents[2];mode=sys.argv[1];out=p/'outputs'/mode;out.mkdir(exist_ok=True,parents=True)
cases={
 'literal-confined':'#set text(fill:blue)\n#show "A":set text(fill:red)\nP A Q',
 'regex-confined':'#set text(fill:blue)\n#show regex("A"):set text(fill:red)\nP A Q',
 'constant-product':'#set text(fill:blue)\n#show "B":set text(fill:red)\n#show "A":[B]\nP A Q',
 'function-product':'#set text(fill:blue)\n#show "B":set text(fill:red)\n#show "A":it=>[B]\nP A Q',
 'local-confined':'#set text(fill:blue)\n#let part=[#show "A":set text(fill:red)\nA]\nP #part Q',
 'partial-match':'#set text(fill:blue)\n#show "AB":set text(fill:red)\nX ABC Y',
 'remaining-rule':'#set text(fill:blue)\n#show "B":set text(fill:red)\n#show "A":it=>[B]\nP A B Q',
 'overlapping-style':'#set text(fill:blue)\n#show "A":set text(fill:red)\n#show "A":set text(fill:green)\nP A Q',
}
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest()
bins={'tekt':p/'bin/typst-before' if mode=='style-before' else r/'target/debug/typst','vanilla':r/'lab/typst-original/target/release/typst'}
rows=[]
for label,source in cases.items():
 f=out/(label+'.typ');f.write_text(source);row=dict(label=label,source=source,source_sha256=sha(f),results={})
 for actor,bin in bins.items():
  target=f.with_name(label+'-'+actor+'.svg');cmd=[str(bin),'compile',str(f),str(target)];z=subprocess.run(cmd,capture_output=True,text=True,timeout=60);result=dict(command=cmd,exit=z.returncode,stderr=z.stderr)
  if z.returncode==0:
   root=ET.parse(target).getroot();glyphs=[]
   def walk(node,fill=None):
    fill=node.attrib.get('fill',fill)
    if node.tag.rsplit('}',1)[-1]=='use':glyphs.append(dict(fill=fill,x=node.attrib.get('x'),y=node.attrib.get('y')))
    if node.tag.rsplit('}',1)[-1]!='defs':
     for child in node:walk(child,fill)
   walk(root);result.update(sha256=sha(target),glyphs=glyphs)
  row['results'][actor]=result
 rows.append(row);print(label,[(k,v['exit'],[g['fill'] for g in v.get('glyphs',[])]) for k,v in row['results'].items()],flush=True)
(p/(mode+'.json.gz')).write_bytes(gzip.compress(json.dumps(dict(source_state='candidate9-sources.json.gz',utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),binaries={k:dict(path=str(v),sha256=sha(v)) for k,v in bins.items()},cases=rows),indent=2).encode()))
