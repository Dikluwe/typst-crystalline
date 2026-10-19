from pathlib import Path
import datetime,gzip,hashlib,json,subprocess
root=Path.cwd();out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items());folder=out/'supplement';folder.mkdir(exist_ok=False)
cases={
'header-default-fields':'#repr(table.header().fields())',
'header-default-level':'#repr(table.header().level)',
'header-default-repeat':'#repr(table.header().repeat)',
'header-content-fields':'#repr(table.header[H].fields())',
'header-level-one-fields':'#repr(table.header(level: 1)[H].fields())',
'header-default-has-level':'#repr(table.header[H].has("level"))',
'header-empty-content-fields':'#repr(table.header([]).fields())',
'header-default-get':'#repr(table.header[H].at("level", default: 9))',
'footer-default-fields':'#repr(table.footer().fields())',
'footer-default-repeat':'#repr(table.footer().repeat)',
'footer-content-fields':'#repr(table.footer[F].fields())',
'footer-repeat-fields':'#repr(table.footer(repeat: false)[F].fields())',
'footer-empty-content-fields':'#repr(table.footer([]).fields())',
'hidden-valid-cell-panic':'#show table.cell: it => panic("CELL")\n#show table: none\n#table(columns: 1, [x])',
'hidden-invalid-cell-panic':'#show table.cell: it => panic("CELL")\n#show table: none\n#table(columns: 1, table.cell(colspan: 2)[x])',
'hidden-invalid-empty':'#show table: []\n#table(columns: 1, table.cell(colspan: 2)[x])',
'hidden-invalid-text':'#show table: [replacement]\n#table(columns: 1, table.cell(colspan: 2)[x])',
'hidden-invalid-table-panic':'#show table: it => panic("TABLE")\n#table(columns: 1, table.cell(colspan: 2)[x])',
'hidden-invalid-header':'#show table: none\n#table(columns: 1, table.header(table.cell(colspan: 2)[H]))',
'hidden-invalid-footer':'#show table: none\n#table(columns: 1, table.footer(table.cell(colspan: 2)[F]))',
'hidden-invalid-duplicates':'#show table: none\n#table(columns: 1, table.cell(y: 0)[a], table.cell(y: 0)[b])',
'hidden-invalid-footer-end':'#show table: none\n#table(columns: 1, table.footer[F], [B])',
'show-table-raw-header':'#show table: it => repr(it.children.first().fields())\n#table(columns: 1, table.header[H], [B])',
'show-table-raw-child':'#show table: it => repr(it.children.last().fields())\n#table(columns: 1, [B])',
'show-table-cell-show':'#show table.cell: it => panic("CELL")\n#show table: it => repr(it.children.last().fields())\n#table(columns: 1, [B])',
'show-table-default-header-get':'#show table: it => repr(it.children.first().at("level",default: 9))\n#table(columns: 1, table.header[H], [B])',
}
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
with gzip.open(out/'supplement.json.gz','wt') as f:json.dump(record,f)
print(json.dumps(dict(cases=len(cases),runs=len(record['cases']),sources_unchanged=True)),flush=True)
