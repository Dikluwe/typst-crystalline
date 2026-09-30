from pathlib import Path
import subprocess,json,xml.etree.ElementTree as ET,hashlib,datetime
root=Path.cwd();out=root/'00_nucleo/diagnosticos/atomizacao-separadores-2026-09-28/after';rows=[]
for mode,flags in [('compact',['--compact']),('ua1',['--pdf-standard','ua-1']),('compact-ua1',['--compact','--pdf-standard','ua-1'])]:
 for case,expected in [('11-single-spaces','A B C'),('12-adjacent-control','ABC'),('13-text-literal','A B C'),('15-group','A B C'),('16-link','A B C')]:
  source=out/(case+'-titled.typ');source.write_text('#set document(title: "Separadores")\n'+(out/(case+'.typ')).read_text());pdf=out/(case+'-'+mode+'.pdf');args=[str(root/'target/release/typst'),'compile','--font-path',str(out/'fonts'),*flags,str(source),str(pdf)];r=subprocess.run(args,text=True,capture_output=True);entry={'case':case,'mode':mode,'command':args,'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'exit_code':r.returncode,'stderr':r.stderr}
  if r.returncode==0:
   xml=pdf.with_suffix('.xml');subprocess.run(['pdftotext','-bbox-layout',str(pdf),str(xml)],check=True);tree=ET.parse(xml);text=' '.join(''.join(w.itertext()) for w in tree.findall('.//{*}word'));entry.update(extracted_text=text,text_matches=text==expected,pdf_sha256=hashlib.sha256(pdf.read_bytes()).hexdigest())
  rows.append(entry)
(out/'export-modes.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'binary_sha256':hashlib.sha256((root/'target/release/typst').read_bytes()).hexdigest(),'runs':rows},ensure_ascii=False,indent=2)+'\n')
assert all(x.get('text_matches') for x in rows)
print('15/15 controles compacto e UA1 passaram')
