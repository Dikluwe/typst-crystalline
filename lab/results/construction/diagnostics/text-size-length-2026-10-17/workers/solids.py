from pathlib import Path
import subprocess,json,re,datetime,hashlib,xml.etree.ElementTree as ET,gzip
out=Path(__file__).resolve().parents[1];folder=out/'solid-vectors';folder.mkdir(exist_ok=True);data=json.load(gzip.open(out/'after-measurement.json.gz'));names={x['name'] for x in data['cases'] if x['name'].startswith('perm-')}|{'named-fill','alias'};rows=[]
for name in sorted(names):
 pair=[]
 for kind in ['tekt','vanilla']:
  pdf=out/'after-renders'/(name+'-'+kind+'.pdf');svg=folder/(name+'-'+kind+'.svg');cmd=['pdftocairo','-svg',str(pdf),str(svg)];r=subprocess.run(cmd,capture_output=True,text=True);assert r.returncode==0,r.stderr;root=ET.parse(svg).getroot();colors=[]
  for node in root.iter():
   fill=node.get('fill','');m=re.fullmatch(r'rgb\(([\d.]+)%,\s*([\d.]+)%,\s*([\d.]+)%\)',fill)
   if m:
    color=tuple(float(v)/100 for v in m.groups())
    if max(color)-min(color)>.01:colors.append(color)
  assert colors,(name,kind,'No visible colored glyph paint');row=dict(name=name,binary=kind,utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),argv=cmd,pdf_sha256=hashlib.sha256(pdf.read_bytes()).hexdigest(),svg_sha256=hashlib.sha256(svg.read_bytes()).hexdigest(),colored_paints=colors);pair.append(row);rows.append(row)
 assert len(pair[0]['colored_paints'])==len(pair[1]['colored_paints'])
 assert all(max(abs(a-b) for a,b in zip(x,y))<2e-5 for x,y in zip(pair[0]['colored_paints'],pair[1]['colored_paints'])),name
(out/'solid-paint-result.json').write_text(json.dumps(dict(pairs=len(names),criterion='Non-gray glyph paint exists and its RGB agrees between PDF-derived vectors within 2e-5; Typst red is a palette color, not pure RGB red. Text geometry checked separately.',rows=rows),indent=2)+'\n');print(len(names),'solid-color pairs passed')
