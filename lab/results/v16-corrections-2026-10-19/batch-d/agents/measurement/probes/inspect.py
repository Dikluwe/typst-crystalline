import pathlib,json,xml.etree.ElementTree as E,re,subprocess,datetime,hashlib
D=pathlib.Path(__file__).resolve().parent
r=json.loads((D/'png-before.json').read_text()); summaries=[]
for x in r['runs']:
 if x['exit']:continue
 root=E.parse(x['output']).getroot()
 for e in root.iter():
  if e.get('fill')=='#ff0000':
   transform=e.get('transform',''); m=re.search(r'translate\(([-.\d]+)',transform)
   summaries.append({'case':x['name'],'binary':x['binary'],'marker_x':float(e.get('x','0'))+(float(m[1]) if m else 0),'element':e.attrib})
(D/'geometry-before.json').write_text(json.dumps(summaries,indent=2))
print([(s['case'],s['binary'],s['marker_x']) for s in summaries if s['case'].endswith('auto')])
