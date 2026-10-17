from pathlib import Path
from html.parser import HTMLParser
import gzip,json,xml.etree.ElementTree as E
D=Path(__file__).parent;d=json.loads(gzip.decompress((D/'final.json.gz').read_bytes()))
class Body(HTMLParser):
 def __init__(self):super().__init__();self.inside=False;self.nodes=[]
 def handle_starttag(self,tag,attrs):
  if tag=='body':self.inside=True
  if self.inside:self.nodes.append(['start',tag,sorted(attrs)])
 def handle_endtag(self,tag):
  if self.inside:self.nodes.append(['end',tag])
  if tag=='body':self.inside=False
 def handle_data(self,data):
  if self.inside and data.strip():self.nodes.append(['text',data])
def bbox(v):
 root=E.fromstring(v['bbox_xml']);return [(p.attrib,[(w.text,{k:float(n) for k,n in w.attrib.items()}) for w in p.iter() if w.tag.endswith('word')]) for p in root.iter() if p.tag.endswith('page')]
r={}
for name,row in d['cases'].items():
 a=row['runs']['tekt'];b=row['runs']['vanilla'];q={'status_stderr_equal':all(a[k]==b[k] for k in ['status','stderr'])}
 if 'text' in a and 'text' in b:q['pdf_text_equal']=a['text']==b['text']
 if 'bbox_xml' in a and 'bbox_xml' in b:
  aa=bbox(a);bb=bbox(b);valid=len(aa)==len(bb);delta=0
  for (pa,wa),(pb,wb) in zip(aa,bb):
   valid &= pa==pb and len(wa)==len(wb)
   for (ta,xa),(tb,xb) in zip(wa,wb):
    valid &= ta==tb;delta=max(delta,*[abs(xa[k]-xb[k]) for k in xa])
  q['pdf_bbox_within_0.05pt']=bool(valid and delta<0.05);q['max_coordinate_delta_pt']=delta
 if name=='html':
  x=Body();y=Body();x.feed(a['content']);y.feed(b['content']);q['body_dom_equal']=x.nodes==y.nodes
 q['match_measured_criteria']=all(v for k,v in q.items() if k not in ['max_coordinate_delta_pt'])
 r[name]=q
summary={'utc':d['utc'],'head':d['head'],'bin_sha256':d['bin_sha256'],'cases':r,'match':sum(v['match_measured_criteria'] for v in r.values()),'total':len(r),'limits':'SVG availability/status/stderr only; no SVG geometry or ink parity. No output validation remains open. PDF criterion uses text/pages/boxes, not bytes.'}
(D/'summary.json').write_text(json.dumps(summary,ensure_ascii=False,indent=2)+'\n');print(json.dumps(summary,ensure_ascii=False,indent=2))
