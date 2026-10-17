from pathlib import Path
import datetime,gzip,hashlib,json,subprocess
root=Path.cwd();out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items());folder=out/'lines';folder.mkdir(exist_ok=False)
cases={'line-default': '#repr(table.hline().fields())', 'line-y-zero': '#repr(table.hline(y: 0).fields())', 'line-y-auto': '#repr(table.hline(y: auto).fields())', 'line-y-none': '#repr(table.hline(y: none).fields())', 'line-y-negative': '#repr(table.hline(y: -1).fields())', 'line-end-none': '#repr(table.hline(end: none).fields())', 'line-end-auto': '#repr(table.hline(end: auto).fields())', 'line-position-auto': '#repr(table.hline(position: auto).fields())', 'line-position-none': '#repr(table.hline(position: none).fields())', 'line-full': '#repr(table.hline(y: 0, start: 0, end: 1, stroke: none, position: bottom).fields())', 'vline-default': '#repr(table.vline().fields())', 'vline-x-zero': '#repr(table.vline(x: 0).fields())', 'vline-x-auto': '#repr(table.vline(x: auto).fields())', 'vline-x-none': '#repr(table.vline(x: none).fields())', 'vline-x-negative': '#repr(table.vline(x: -1).fields())', 'line-invalid-range': '#table(columns: 2, table.hline(start: 2, end: 1), [a])', 'line-invalid-y': '#table(columns: 2, table.hline(y: 5), [a])', 'line-multiple': '#table(columns: 2, table.header(table.hline(), table.cell(colspan: 2)[H], table.hline()), [a], [b], table.hline())'}
record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=b['hashes'],binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in {'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'}.items()},cases=[])
# Parse the full exported body as data; preserve raw HTML alongside it.
from html.parser import HTMLParser
class Read(HTMLParser):
 def __init__(self):super().__init__();self.body=False;self.text=[]
 def handle_starttag(self,t,a):
  if t=='body':self.body=True
 def handle_endtag(self,t):
  if t=='body':self.body=False
 def handle_data(self,s):
  if self.body:self.text.append(s)
for name,text in cases.items():
 src=folder/(name+'.typ');src.write_text(text+'\n')
 for kind,info in record['binaries'].items():
  dst=folder/(name+'-'+kind+'.html');cmd=[info['path'],'compile',str(src),str(dst),'--features','html','--format','html'];r=subprocess.run(cmd,capture_output=True,text=True,timeout=30);item=dict(name=name,binary=kind,utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source=str(src),source_sha256=sha(src),argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
  if dst.exists():
   p=Read();p.feed(dst.read_text());item.update(html=str(dst),html_sha256=sha(dst),body_text=''.join(p.text))
  record['cases'].append(item)
record.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources_unchanged=all(sha(p)==h for p,h in b['hashes'].items()));assert record['sources_unchanged'];assert record['binaries']['vanilla']['sha256']=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
with gzip.open(out/'lines.json.gz','wt') as f:json.dump(record,f)
print(json.dumps(dict(cases=len(cases),runs=len(record['cases']),sources_unchanged=True)),flush=True)
