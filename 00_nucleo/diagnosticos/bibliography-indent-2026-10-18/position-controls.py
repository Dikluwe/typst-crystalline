from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,sys
import xml.etree.ElementTree as ET
p=Path(__file__).resolve().parent;r=p.parents[2]
def sha(f):
 h=hashlib.sha256()
 with f.open('rb') as stream:
  for block in iter(lambda:stream.read(1024*1024),b''):h.update(block)
 return h.hexdigest()
cases={'none': '#set page(width:200pt,height:auto,margin:8pt)\n#set text(size:12pt)\n@audit\n#bibliography("audit.bib",style:"mla",title:none)\n#context metadata(query(bibliography).map(it=>it.location().position()))', 'default': '#set page(width:200pt,height:auto,margin:8pt)\n#set text(size:12pt)\n@audit\n#bibliography("audit.bib",style:"mla")\n#context metadata(query(bibliography).map(it=>it.location().position()))'}
cases={'heading-control':'#set page(margin:8pt)\n= TITLE\n#context metadata(query(heading).map(it=>it.location().position()))','block-control':'#set page(margin:8pt)\n#block[TEXT]<probe>\n#context metadata(query(<probe>).map(it=>it.location().position()))'}
bins={'tekt':r/'target/debug/typst','vanilla':r/'lab/typst-original/target/release/typst'}
meta={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'binaries':{k:{'path':str(b),'sha256':sha(b)} for k,b in bins.items()},'pin':'bd5ebec285505889e8ff1d4f68804ab147a699f3','cases':[]}

for label,source in cases.items():
 f=p/'outputs'/f'position-{label}.typ';f.write_text(source);row={'label':label,'source':source,'source_sha256':sha(f),'results':{}}
 for name,binary in bins.items():
  cmd=[str(binary),'query',str(f),'metadata','--field','value'];z=subprocess.run(cmd,text=True,capture_output=True,timeout=45)
  row['results'][name]={'command':cmd,'exit':z.returncode,'stdout':z.stdout,'stderr':z.stderr}
 meta['cases'].append(row);print(label,row['results'],flush=True)
(p/(sys.argv[1]+'.json.gz')).write_bytes(gzip.compress(json.dumps(meta,indent=2).encode()))
