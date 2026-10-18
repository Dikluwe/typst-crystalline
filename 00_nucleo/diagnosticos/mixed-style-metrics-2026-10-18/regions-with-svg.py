from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,xml.etree.ElementTree as ET,sys
p=Path(__file__).resolve().parent;r=p.parents[2]
def sha(f):
 with Path(f).open('rb') as stream:return hashlib.file_digest(stream,'sha256').hexdigest()
cases={}
for strategy in ['simple','optimized']:
 for height in ['39.44pt','39.60pt','55.14pt','55.30pt','70pt']:
  for label,body in [('forced','ROMAN\\\n#emph[ITALIC]\\\nROMAN'),('wrap','ROMAN #emph[italic long title with several words that wrap onto more lines in the paragraph] LAST')]:
   cases[f'region-{strategy}-{height}-{label}']=f'#set page(width:160pt,height:{height},margin:8pt)\n#set text(size:12pt,font:"Libertinus Serif",hyphenate:false)\n#set par(linebreaks:"{strategy}")\n'+body
bins={'tekt':p/'bin/typst-before' if sys.argv[1].endswith('before') else r/'target/debug/typst','vanilla':r/'lab/typst-original/target/release/typst'}
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),status=subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),pin='bd5ebec285505889e8ff1d4f68804ab147a699f3',binaries={n:dict(path=str(b),sha256=sha(b)) for n,b in bins.items()},cases=[],source_proof='before-sources.json.gz for preserved before binary; working-tree source below may already include candidate, whose CLI has not replaced the preserved binary.')
for label,source in cases.items():
 f=p/'outputs'/(label+'.typ');f.write_text(source);row=dict(label=label,source=source,source_sha256=sha(f),results={})
 for actor,bin in bins.items():
  result={}
  for fmt in ['pdf','svg']:
   out=f.with_name(label+'-'+actor+'.'+fmt);cmd=[str(bin),'compile',str(f),str(out)];z=subprocess.run(cmd,text=True,capture_output=True,timeout=60);result[fmt]=dict(command=cmd,exit=z.returncode,stderr=z.stderr)
   if z.returncode==0:
    result[fmt]['sha256']=sha(out)
    if fmt=='pdf':
     bbox=out.with_suffix('.bbox');subprocess.run(['pdftotext','-bbox',str(out),str(bbox)],check=True,capture_output=True);root=ET.parse(bbox).getroot();ns={'x':'http://www.w3.org/1999/xhtml'};result[fmt]['pages']=[dict(width=float(pg.attrib['width']),height=float(pg.attrib['height']),words=[dict(text=w.text,bounds=[float(w.attrib[k]) for k in ['xMin','yMin','xMax','yMax']]) for w in pg.findall('.//x:word',ns)]) for pg in root.findall('.//x:page',ns)]
  row['results'][actor]=result
 record['cases'].append(row);print(label,[(n,{f:v['exit'] for f,v in z.items()}) for n,z in row['results'].items()],flush=True)
assert all(sha(b)==record['binaries'][n]['sha256'] for n,b in bins.items());(p/(sys.argv[1]+'.json.gz')).write_bytes(gzip.compress(json.dumps(record,indent=2).encode()))
