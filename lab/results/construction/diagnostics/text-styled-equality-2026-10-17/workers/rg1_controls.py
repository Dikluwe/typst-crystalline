from pathlib import Path
import datetime,gzip,json,hashlib,subprocess,re,xml.etree.ElementTree as ET
from html.parser import HTMLParser
out=Path(__file__).resolve().parents[1];old=Path('00_nucleo/diagnosticos/table-row-groups-2026-10-17');folder=out/'rg1-controls';folder.mkdir(exist_ok=False);sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();rows=[]
class ObservableHTML(HTMLParser):
 def __init__(self):super().__init__();self.rows=[];self.text=[];self.inbody=False
 def handle_starttag(self,t,a):
  if t=='body':self.inbody=True
  if self.inbody:self.rows.append(('start',t,a))
 def handle_endtag(self,t):
  if self.inbody:self.rows.append(('end',t))
  if t=='body':self.inbody=False
 def handle_data(self,d):
  if self.inbody and d.strip():self.rows.append(('text',d))
for corpus in ['measurement','supplement','lines','styles-phase','svg','metadata']:
 data=json.load(gzip.open(old/(corpus+'.json.gz')))
 for prior in data['cases']:
  assert sha(prior['source'])==prior['source_sha256'];cmd=prior['argv'].copy();cmd[0]=str(Path('target/debug/typst' if prior['binary']=='tekt' else 'lab/typst-original/target/release/typst').resolve());dst=folder/(corpus+'-'+prior['name']+'-'+prior['binary']+Path(cmd[3]).suffix);oldoutput=Path(cmd[3]);cmd[3]=str(dst.resolve());r=subprocess.run(cmd,capture_output=True,text=True,timeout=40);row=dict(corpus=corpus,name=prior['name'],binary=prior['binary'],utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),argv=cmd,binary_sha256=sha(cmd[0]),source_sha256=sha(cmd[2]),exit=r.returncode,stdout=r.stdout,stderr=r.stderr);assert (r.returncode,r.stdout,r.stderr)==(prior['exit'],prior['stdout'],prior['stderr']),(row,prior)
  if r.returncode==0:
   if dst.suffix=='.html':
    a=ObservableHTML();b=ObservableHTML();a.feed(oldoutput.read_text());b.feed(dst.read_text());assert a.rows==b.rows,(corpus,prior['name'],prior['binary'],'HTML body changed');row['html_body_unchanged']=True
   else:
    def strokes(path):
     return [dict(node.attrib) for node in ET.parse(path).getroot().iter() if node.get('stroke') in ['#ff0000','#0000ff']]
    assert strokes(oldoutput)==strokes(dst),(corpus,prior['name'],prior['binary'],'colored line geometry changed')
    row['colored_stroke_geometry_unchanged']=True
   row['output_sha256']=sha(dst)
  rows.append(row)
 print(corpus,flush=True)
assert len(rows)==210
(out/'rg1-controls.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),pairs=len(rows)//2,all_measured_observables_preserved=True,rows=rows),indent=2)+'\n')
