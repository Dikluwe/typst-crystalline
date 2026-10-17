from pathlib import Path
import datetime,gzip,hashlib,json,subprocess
root=Path.cwd();out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items());folder=out/'metadata';folder.mkdir(exist_ok=False)
cases={'header-cell-raw-eq': '#let h = table.header(level: 3)[H]\n#repr(h.children.first() == table.cell[H])', 'header-json': '#json.encode(table.header(repeat: false, level: 3)[H])', 'footer-json': '#json.encode(table.footer(repeat: false)[F])', 'line-json': '#json.encode(table.hline(y: auto, end: none, stroke: none))', 'vline-json': '#json.encode(table.vline(x: 0))', 'header-func': '#repr(table.header[H].func() == table.header)', 'header-func-repr': '#repr(table.header[H].func())', 'cell-json': '#json.encode(table.cell[H])'}
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
with gzip.open(out/'metadata.json.gz','wt') as f:json.dump(record,f)
print(json.dumps(dict(cases=len(cases),runs=len(record['cases']),sources_unchanged=True)),flush=True)
