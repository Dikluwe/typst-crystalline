from pathlib import Path
import subprocess,json,gzip,datetime,hashlib,sys
import xml.etree.ElementTree as ET
p=Path(__file__).resolve().parent;r=p.parents[2]
def sha(f):
 h=hashlib.sha256()
 with f.open('rb') as stream:
  for block in iter(lambda:stream.read(1024*1024),b''):h.update(block)
 return h.hexdigest()
cases={'bare-style': '#set page(width:200pt,height:auto,margin:8pt)\n#set text(size:12pt)\n#show bibliography:set text(fill:red)\n@audit\n#bibliography("audit.bib",style:"mla",title:none)\n\nOUTSIDE words continue outside bibliography.', 'where-style': '#set page(width:200pt,height:auto,margin:8pt)\n#set text(size:12pt)\n#show bibliography.where():set text(fill:red)\n@audit\n#bibliography("audit.bib",style:"mla",title:none)\n\nOUTSIDE words continue outside bibliography.', 'alias-style': '#set page(width:200pt,height:auto,margin:8pt)\n#set text(size:12pt)\n#let b=bibliography("audit.bib",style:"mla",title:none)\n#show bibliography:set text(fill:red)\n@audit\n#b'}
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

colors={}
for label in cases:
 colors[label]={}
 for binary in bins:
  pdf=p/'outputs'/f'{label}-{binary}.pdf';cmd=['pdftohtml','-xml','-hidden','-i','-stdout',str(pdf)];out=subprocess.run(cmd,text=True,capture_output=True,check=True).stdout
  root=ET.fromstring(out);fonts={f.attrib['id']:f.attrib['color'] for f in root.iter('fontspec')}
  colors[label][binary]={'command':cmd,'pdf_sha256':sha(pdf),'BIBWORD':[fonts[x.attrib['font']] for x in root.iter('text') if 'BIBWORD' in ''.join(x.itertext())],'OUTSIDE':[fonts[x.attrib['font']] for x in root.iter('text') if 'OUTSIDE' in ''.join(x.itertext())]}
for label in ('bare-style','where-style'):
 for binary in bins:
  assert set(colors[label][binary]['BIBWORD'])=={'#ff4136'},(label,binary,colors)
  assert set(colors[label][binary]['OUTSIDE'])=={'#000000'},(label,binary,colors)
assert set(colors['alias-style']['tekt']['BIBWORD'])=={'#000000'}
assert set(colors['alias-style']['vanilla']['BIBWORD'])=={'#ff4136'}
(p/'colors-audit.json.gz').write_bytes(gzip.compress(json.dumps({'provenance':'colors-after.json.gz + complete-freeze.json.gz','direct_style_and_outside_preserved':True,'alias_style_still_open':True,'colors':colors},indent=2).encode()))
print(json.dumps(colors,indent=2))
