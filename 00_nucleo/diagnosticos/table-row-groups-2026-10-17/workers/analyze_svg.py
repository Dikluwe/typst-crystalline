from pathlib import Path
import datetime,gzip,hashlib,json,re,xml.etree.ElementTree as ET
out=Path(__file__).resolve().parents[1];record=json.load(gzip.open(out/'svg.json.gz'));num=r'[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?';ml=re.compile(r'^\s*M\s*('+num+r')[ ,]+('+num+r')\s*L\s*('+num+r')[ ,]+('+num+r')\s*$');axis=re.compile(r'^\s*M\s*('+num+r')[ ,]+('+num+r')\s*([hHvV])\s*('+num+r')\s*$');results=[]
for item in record['cases']:
 p=Path(item['svg']);assert hashlib.sha256(p.read_bytes()).hexdigest()==item['svg_sha256'];lines=[]
 for node in ET.parse(p).getroot().iter():
  color=node.get('stroke');tag=node.tag.split('}')[-1]
  if color not in ['#ff0000','#0000ff']:continue
  tr=node.get('transform','');assert not tr or re.fullmatch(r'translate\([-.\deE+, ]+\)',tr),tr
  if tag=='line':coords=[float(node.get(a,'0')) for a in ['x1','y1','x2','y2']]
  elif tag=='path':
   data=node.get('d','');m=ml.fullmatch(data);a=axis.fullmatch(data)
   if m:coords=list(map(float,m.groups()))
   elif a:
    x,y=float(a[1]),float(a[2]);cmd=a[3];v=float(a[4]);coords=[x,y,x+(v if cmd=='h' else 0) if cmd in 'hHvV' and cmd!='H' else v,y+(v if cmd=='v' else 0)]
    if cmd=='H':coords=[x,y,v,y]
    if cmd=='V':coords=[x,y,x,v]
   else:raise AssertionError(data)
  else:raise AssertionError(tag)
  x0,y0,x1,y1=coords;assert x0==x1 or y0==y1;lines.append(dict(stroke=color,horizontal=y0==y1,length=abs(x1-x0)+abs(y1-y0),node=dict(node.attrib)))
 totals={color:sum(x['length'] for x in lines if x['horizontal'] and x['stroke']==color) for color in ['#ff0000','#0000ff']};results.append(dict(name=item['name'],binary=item['binary'],svg=str(p),svg_sha256=item['svg_sha256'],horizontal_by_color=totals,lines=lines))
by={(r['name'],r['binary']):r['horizontal_by_color'] for r in results};assert by['base','tekt']['#ff0000']==by['erase-top','tekt']['#ff0000'];assert by['erase-top','vanilla']['#ff0000']<by['base','vanilla']['#ff0000'];assert by['blue-top','tekt']['#0000ff']==by['erase-blue','tekt']['#0000ff']>0;assert by['erase-blue','vanilla']['#0000ff']==0
(out/'svg-analysis.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_measurement='svg.json.gz',method='All colored stroke nodes are straight axis-aligned lines or single h/v path segments; transforms are translation only. Compare paired changes within each binary, not counts/bytes across renderers.',results=results,erasure_defect_confirmed=True,first_parser_incomplete_not_used_for_decisions=True),indent=2)+'\n')
for r in results:print(r['name'],r['binary'],r['horizontal_by_color'])
