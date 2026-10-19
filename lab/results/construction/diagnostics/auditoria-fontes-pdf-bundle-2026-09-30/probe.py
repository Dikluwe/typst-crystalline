from pathlib import Path
import datetime, hashlib, json, subprocess, xml.etree.ElementTree as ET
r=Path('/repos/Antigravity/typst-crystalline');d=Path('/tmp/tekt-font-dispatch-20260930');binary=r/'target/debug/typst';fonts=r/'03_infra/fixtures/fonts';sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
cases={
 'zero':'#set page(width: 200pt, height: 100pt, margin: 10pt)\n#rect(width: 20pt, height: 20pt)\n',
 'single':'#set page(width: 200pt, height: 100pt, margin: 10pt)\n#set text(font: "Noto Sans", size: 12pt)\nHello\n',
 'multi':'#set page(width: 200pt, height: 100pt, margin: 10pt)\n#text(font: "Noto Sans", size: 12pt)[Hello]#h(8pt)#text(font: "Nimbus Sans", size: 12pt)[World]\n',
 'variable_default':'#set page(width: 200pt, height: 100pt, margin: 10pt)\n#set text(font: "Ubuntu Sans", size: 12pt)\nHello\n',
 'variable':'#set page(width: 200pt, height: 100pt, margin: 10pt)\n#set text(font: "Ubuntu Sans", weight: 700, size: 12pt)\nHello\n',
}
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip(),'binary_sha256':sha(binary),'font_sha256':{f.name:sha(f) for f in [fonts/'NotoSans-Regular.ttf',fonts/'NimbusSans-Regular.otf',fonts/'UbuntuSans-Variable.ttf']},'cases':{}}
for name,body in cases.items():
 row={'runs':{}}
 for lane in ['single','bundle']:
  src=d/(name+'-'+lane+'.typ');src.write_text(body if lane=='single' else '#document("one.pdf")['+body+']\n');out=d/(name+'-'+lane+'.pdf' if lane=='single' else name+'-bundle-out');cmd=[str(binary),'compile','--font-path',str(fonts),str(src),str(out),'--format',('pdf' if lane=='single' else 'bundle'),*(['--features','bundle'] if lane=='bundle' else [])];run=subprocess.run(cmd,capture_output=True,text=True);info={'command':cmd,'source_sha256':sha(src),'exit':run.returncode,'stderr':run.stderr};pdf=out if lane=='single' else out/'one.pdf'
  if pdf.exists():
   xml=d/(name+'-'+lane+'.xml');subprocess.run(['pdftotext','-bbox-layout',str(pdf),str(xml)],check=True,capture_output=True);pages=ET.parse(xml).getroot().findall('.//{*}page');info['page_count']=len(pages);info['words']=[{'text':''.join(w.itertext()),'page':i,**{k:float(w.attrib[k]) for k in ['xMin','yMin','xMax','yMax']}} for i,p in enumerate(pages) for w in p.findall('.//{*}word')];info['pdffonts']=subprocess.check_output(['pdffonts',str(pdf)],text=True);info['pdf_sha256']=sha(pdf)
  row['runs'][lane]=info
 a,b=row['runs']['single'],row['runs']['bundle'];row['same_exit']=a['exit']==b['exit'];row['same_pages']=a.get('page_count')==b.get('page_count');row['same_words']=a.get('words')==b.get('words');row['font_names']={lane:[line.split()[0] for line in info.get('pdffonts','').splitlines()[2:] if line.strip()] for lane,info in row['runs'].items()};record['cases'][name]=row
(d/'results.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n')
for name,row in record['cases'].items(): print(name, row['same_exit'], row['same_pages'],row['same_words'],row['font_names'],[(k,v['exit']) for k,v in row['runs'].items()])
