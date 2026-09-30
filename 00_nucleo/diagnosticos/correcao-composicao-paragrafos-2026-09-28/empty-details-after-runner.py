import json,subprocess,xml.etree.ElementTree as ET,hashlib,datetime
from pathlib import Path
r=Path.cwd();d=r/'00_nucleo/diagnosticos/correcao-composicao-paragrafos-2026-09-28/empty-details-after';d.mkdir(exist_ok=True);rows=[]
for name,body in [('start-text','#par("") AFTER'),('end-text','BEFORE #par("")'),('only-text','#par("")'),('only-empty','#par[]'),('styled-end','BEFORE #par(leading:2pt)[]'),('text-middle','BEFORE #par("") AFTER'),('two-empty','BEFORE #par[]#par[] AFTER')]:
 src=d/(name+'.typ');src.write_text('#set page(width:140pt,height:auto,margin:8pt)\n#set text(font:"Noto Sans",size:12pt)\n'+body);row={'name':name,'source':body}
 for lane,bin in [('vanilla',r/'lab/typst-original/target/release/typst'),('tekt',r/'target/release/typst')]:
  pdf=d/f'{name}-{lane}.pdf';res=subprocess.run([str(bin),'compile','--font-path',str(r/'00_nucleo/diagnosticos/correcao-composicao-paragrafos-2026-09-28/before-pdf/fonts'),str(src),str(pdf)],capture_output=True,text=True);row[lane]={'exit':res.returncode,'stderr':res.stderr,'binary_sha256':hashlib.sha256(bin.read_bytes()).hexdigest()}
  if res.returncode==0:
   xml=d/f'{name}-{lane}.xml';subprocess.run(['pdftotext','-bbox-layout',str(pdf),str(xml)],capture_output=True);doc=ET.parse(xml);row[lane]['pages']=[{'height':p.attrib['height'],'words':[dict(w.attrib,text=''.join(w.itertext())) for w in p.findall('.//{*}word')]} for p in doc.findall('.//{*}page')]
 rows.append(row);print(name,{lane:row[lane].get('pages') for lane in ['tekt','vanilla']})
(d/'results.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_HEAD_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'rows':rows},indent=2))
