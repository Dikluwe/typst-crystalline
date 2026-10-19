from pathlib import Path
import json,gzip,tarfile,xml.etree.ElementTree as ET,datetime
p=Path(__file__).resolve().parent
before=json.loads((p/'deco-inspection.json').read_text())['cases'];after=json.loads(gzip.decompress((p/'deco-after.json.gz').read_bytes()))['cases'];rows=[]
def without_lines(data):
 root=ET.fromstring(data)
 for parent in root.iter():
  for child in list(parent):
   if child.tag.rsplit('}',1)[-1]=='line':parent.remove(child)
 return ET.tostring(root)
with tarfile.open(p/'deco-before-artifacts.tar.gz','r:gz') as tar:
 for old,new in zip(before,after):
  assert old['label']==new['label'];label=new['label'];amount=int(label.split('-')[2]);forced=label.endswith('True')
  oldlines=[r['attributes'] for r in old['observations']['tekt']];newlines=new['results']['tekt']['lines'];assert len(oldlines)==len(newlines)
  for i,(a,b) in enumerate(zip(oldlines,newlines)):
   assert {k:v for k,v in a.items() if k!='x1'}=={k:v for k,v in b.items() if k!='x1'},(label,a,b)
   assert float(b['x1'])==8+(amount if forced or i>0 else 0),(label,b)
  file=p/'outputs'/(label+'-tekt.svg');oldsvg=tar.extractfile('outputs/'+file.name).read();assert without_lines(oldsvg)==without_lines(file.read_bytes()),label
  rows.append(dict(label=label,origins_correct=True,other_line_attributes_unchanged=True,all_non_line_svg_content_unchanged=True))
(p/'deco-audit.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),pairs=len(rows),scope='LTR decoration starts only; y, ends, thickness, caps, glyphs and other SVG content exactly preserved relative to pre-refinement candidate. Full decoration geometry is not claimed.',rows=rows),indent=2)+'\n');print(len(rows),'decoration pairs passed')
