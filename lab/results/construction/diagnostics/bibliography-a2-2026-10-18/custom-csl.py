from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,sys
import xml.etree.ElementTree as ET
p=Path(__file__).resolve().parent;r=p.parents[2]
def sha(f):
 h=hashlib.sha256()
 with f.open('rb') as stream:
  for block in iter(lambda:stream.read(1024*1024),b''):h.update(block)
 return h.hexdigest()
cases={'custom-base': '#set page(width:200pt,height:auto,margin:8pt)\n#set text(size:12pt)\n@audit\n#bibliography("audit.bib",style:"custom.csl",title:none)', 'custom-default-title': '#set page(width:200pt,height:auto,margin:8pt)\n#set text(size:12pt)\n@audit\n#bibliography("audit.bib",style:"custom.csl")', 'custom-style': '#set page(width:200pt,height:auto,margin:8pt)\n#set text(size:12pt)\n#show bibliography:set text(fill:red)\n@audit\n#bibliography("audit.bib",style:"custom.csl",title:none)', 'custom-identity': '#set page(width:200pt,height:auto,margin:8pt)\n#set text(size:12pt)\n#show bibliography:it=>it\n@audit\n#bibliography("audit.bib",style:"custom.csl",title:none)', 'custom-hidden': '#set page(width:200pt,height:auto,margin:8pt)\n#set text(size:12pt)\n#show bibliography:it=>[]\n@audit\n#bibliography("audit.bib",style:"custom.csl",title:none)', 'custom-replace': '#set page(width:200pt,height:auto,margin:8pt)\n#set text(size:12pt)\n#show bibliography:it=>[REPLACEMENT]\n@audit\n#bibliography("audit.bib",style:"custom.csl",title:none)'}
bins={'tekt':r/'target/debug/typst','vanilla':r/'lab/typst-original/target/release/typst'}
meta={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),'binaries':{k:{'path':str(b),'sha256':sha(b)} for k,b in bins.items()},'pin':'bd5ebec285505889e8ff1d4f68804ab147a699f3','cases':[]}
for label,source in cases.items():
 f=p/'outputs'/f'{label}.typ';f.write_text(source);row={'label':label,'source':source,'source_sha256':sha(f),'results':{}}
 for name,binary in bins.items():
  out=f.with_name(f.stem+'-'+name+'.pdf');cmd=[str(binary),'compile',str(f),str(out)];z=subprocess.run(cmd,text=True,capture_output=True,timeout=45);txt=subprocess.check_output(['pdftotext',str(out),'-'],text=True) if not z.returncode else ''
  row['results'][name]={'command':cmd,'exit':z.returncode,'stderr':z.stderr,'text':' '.join(txt.split())}
  if not z.returncode:
   bbox=out.with_suffix('.bbox');subprocess.run(['pdftotext','-bbox',str(out),str(bbox)],check=True,capture_output=True)
   ns={'x':'http://www.w3.org/1999/xhtml'};root=ET.parse(bbox).getroot()
   row['results'][name]['pages']=[{'width':float(page.attrib['width']),'height':float(page.attrib['height']),'words':[{'text':w.text,'bounds':[float(w.attrib[k]) for k in ('xMin','yMin','xMax','yMax')]} for w in page.findall('.//x:word',ns)]} for page in root.findall('.//x:page',ns)]
   row['results'][name]['artifact_sha256']=sha(out)
 meta['cases'].append(row);print(label,[(k,v['exit'],v['text'] if not v['exit'] else v['stderr'].splitlines()[0]) for k,v in row['results'].items()],flush=True)
assert all(sha(b)==meta['binaries'][n]['sha256'] for n,b in bins.items())
(p/(sys.argv[1]+'.json.gz')).write_bytes(gzip.compress(json.dumps(meta,indent=2).encode()))

# New differential evidence keeps the default-title layout case RED and proves it is unchanged.
from collections import Counter
baseline=json.loads(gzip.decompress((p/'custom-csl-before.json.gz').read_bytes()))
old={row['label']:row for row in baseline['cases']};classified=[]
for row in meta['cases']:
 t,v=(row['results'][kind] for kind in ('tekt','vanilla'))
 assert t['exit']==v['exit']==0,(row['label'],t,v)
 if row['label']=='custom-default-title':
  previous=old[row['label']]['results']['tekt']
  assert previous['exit']==0
  assert t['text']==previous['text'] and t['pages']==previous['pages'],'new default-title regression'
  assert t['text']!=v['text'],'known default-title layout changed: requires new classification'
  assert Counter(t['text'].split())==Counter(v['text'].split()),'custom CSL lost words'
  state='OPEN_PREEXISTING_LAYOUT; not parity GREEN'
 else:
  assert t['text']==v['text'],(row['label'],t['text'],v['text'])
  state='equal extracted text; no claim of complete layout parity'
 assert 'CUSTOM-CITE:' in t['text'],(row['label'],t['text'])
 if row['label'] not in ('custom-hidden','custom-replace'):assert 'CUSTOM-BIB:' in t['text']
 classified.append({'label':row['label'],'state':state})
(p/'custom-csl-audit.json.gz').write_bytes(gzip.compress(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'classified':classified,'strict_first_attempt':'final-checks-attempt1.json.gz preserves initial failing six-case parity assertion; no code/test expectation was relaxed','counterfactual':'counterfactual-proof.json verifies exact before/restored source and binary states'},indent=2).encode()))
print('custom CSL: 5 text controls equal; default-title layout remains RED and unchanged from exact before state',flush=True)
