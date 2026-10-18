from pathlib import Path
import json,gzip,subprocess,datetime,hashlib,xml.etree.ElementTree as ET
p=Path(__file__).resolve().parent;r=p.parents[2];out=p/'outputs/style-order-candidate9';out.mkdir(parents=True,exist_ok=True)
base='#set text(fill:blue)\n'
cases={
 'old-style-new-constant':base+'#show "A":set text(fill:red)\n#show "A":[B]\nP A Q',
 'old-constant-new-style':base+'#show "A":[B]\n#show "A":set text(fill:red)\nP A Q',
 'old-style-new-function':base+'#show "A":set text(fill:red)\n#show "A":it=>[B]\nP A Q',
 'old-function-new-style':base+'#show "A":it=>[B]\n#show "A":set text(fill:red)\nP A Q',
 'old-regex-style-new-literal':base+'#show regex("[A-Z]"):set text(fill:red)\n#show "A":it=>[B]\nP A Q',
 'old-literal-new-regex-style':base+'#show "A":it=>[B]\n#show regex("[A-Z]"):set text(fill:red)\nP A Q',
}
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest();bins={'tekt':r/'target/debug/typst','vanilla':r/'lab/typst-original/target/release/typst'}
d=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_state='candidate9-sources.json.gz',pin='bd5ebec285505889e8ff1d4f68804ab147a699f3',binaries={n:dict(path=str(b),sha256=sha(b)) for n,b in bins.items()},cases=[])
for label,source in cases.items():
 f=out/(label+'.typ');f.write_text(source);row=dict(label=label,source=source,source_sha256=sha(f),results={})
 for name,binary in bins.items():
  target=out/(label+'-'+name+'.svg');cmd=[str(binary),'compile',str(f),str(target)];z=subprocess.run(cmd,text=True,capture_output=True,timeout=60);result=dict(command=cmd,exit=z.returncode,stderr=z.stderr,glyphs=[])
  if z.returncode==0:
   def walk(node,fill=None):
    fill=node.attrib.get('fill',fill)
    if node.tag.rsplit('}',1)[-1]=='use':result['glyphs'].append(dict(fill=fill,x=node.attrib.get('x'),y=node.attrib.get('y')))
    if node.tag.rsplit('}',1)[-1]!='defs':
     for child in node:walk(child,fill)
   walk(ET.parse(target).getroot());result['sha256']=sha(target)
  row['results'][name]=result
 a,b=[row['results'][n] for n in ['tekt','vanilla']];row['same']=a['exit']==b['exit']==0 and [g['fill'] for g in a['glyphs']]==[g['fill'] for g in b['glyphs']];d['cases'].append(row);print(label,row['same'],[(n,[g['fill'] for g in v['glyphs']]) for n,v in row['results'].items()],flush=True)
d['end']=datetime.datetime.now(datetime.timezone.utc).isoformat();assert all(sha(b)==d['binaries'][n]['sha256'] for n,b in bins.items());(p/'style-order-candidate9.json.gz').write_bytes(gzip.compress(json.dumps(d,indent=2).encode()));assert all(row['same'] for row in d['cases'])
