from pathlib import Path
from html.parser import HTMLParser
import datetime,gzip,hashlib,json,subprocess
root=Path.cwd();out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();assert all(sha(p)==h for p,h in b['hashes'].items());folder=out/'cases';folder.mkdir(exist_ok=False)
cases={
'control':'#table(columns: 2, [a], [b], [c])',
'header-colspan':'#table(columns: 2, table.header(table.cell(colspan: 2)[H]), [a], [b])',
'footer-colspan':'#table(columns: 2, [a], [b], table.footer(table.cell(colspan: 2)[F]))',
'header-rowspan':'#table(columns: 2, table.header(table.cell(rowspan: 2)[H], [h]), [a], [b])',
'footer-rowspan':'#table(columns: 2, [a], [b], table.footer(table.cell(rowspan: 2)[F], [f]))',
'header-area':'#table(columns: 2, table.header(table.cell(colspan: 2, rowspan: 2)[H]), [a], [b])',
'footer-area':'#table(columns: 2, [a], [b], table.footer(table.cell(colspan: 2, rowspan: 2)[F]))',
'header-after-body':'#table(columns: 2, [a], [b], table.header[H], [c], [d])',
'header-after-partial':'#table(columns: 2, [a], table.header[H], [c])',
'headers-adjacent-equal':'#table(columns: 2, table.header[H1], table.header[H2], [a], [b])',
'headers-separated-equal':'#table(columns: 2, table.header[H1], [a], [b], table.header[H2], [c], [d])',
'headers-ascending':'#table(columns: 2, table.header(level: 2)[H2], table.header(level: 3)[H3], [a], [b])',
'headers-descending':'#table(columns: 2, table.header(level: 3)[H3], [a], [b], table.header(level: 1)[H1], [c])',
'headers-mixed-repeat':'#table(columns: 2, table.header(repeat: false, level: 2)[H2], [a], table.header(repeat: true, level: 3)[H3], [b])',
'header-fixed-y':'#table(columns: 2, table.header(table.cell(y: 2)[H]), [a], [b])',
'header-fixed-y-body-zero':'#table(columns: 2, table.header(table.cell(y: 2)[H]), table.cell(y: 0)[B])',
'header-fixed-x':'#table(columns: 2, [a], table.header(table.cell(x: 1)[H]), [b])',
'header-body-row-conflict':'#let c = table.cell(y: 1)[B]\n#table(columns: 2, table.header(table.cell(rowspan: 2)[H]), c)',
'header-group-fixed-spread':'#table(columns: 2, table.header(table.cell(y: 1)[H1], table.cell(y: 3)[H3]), [a])',
'footer-body-after-fixed':'#table(columns: 2, [a], table.footer(table.cell(y: 3)[F]), table.cell(y: 0, x: 1)[b])',
'footer-body-after-auto':'#table(columns: 2, [a], table.footer(table.cell(y: 3)[F]), [b])',
'footer-body-after-full':'#table(columns: 2, [a], [b], table.footer[F], [c])',
'footer-fixed-too-early':'#table(columns: 2, table.cell(y: 3)[B], table.footer(table.cell(y: 1)[F]))',
'footer-fixed-rows':'#table(columns: 2, rows: (auto,auto,auto,auto), [a], table.footer[F])',
'empty-header':'#table(columns: 2, table.header(), [a])',
'empty-footer':'#table(columns: 2, [a], table.footer())',
'only-empty-header':'#table(columns: 2, table.header())',
'only-empty-footer':'#table(columns: 2, table.footer())',
'header-line':'#table(columns: 2, table.header(table.hline(), [H]), [a], [b])',
'footer-line':'#table(columns: 2, [a], [b], table.footer([F], table.hline()))',
'header-line-only':'#table(columns: 2, table.header(table.hline()), [a])',
'footer-duplicate':'#table(columns: 2, table.footer[F1], table.footer[F2])',
'header-invalid-zero':'#table(columns: 2, table.header(level: 0)[H], [a])',
'header-raw-level':'#repr(table.header(level: 3)[H].level)',
'header-raw-fields':'#repr(table.header(repeat: false, level: 3)[H].fields())',
'raw-table-children':'#repr(table(columns: 2, table.header[H], [B], table.footer[F]).children)',
'raw-table-fields':'#repr(table(columns: 2, table.header[H], [B], table.footer[F]).fields())',
'group-effects-order':'#show table.cell: it => [#repr((it.x,it.y,it.colspan,it.rowspan)):#it.body]\n#table(columns: 2, [a], table.header[H], [b], table.footer[F])',
'hidden-invalid-occupancy':'#show table: none\n#show table.cell: it => panic("hidden")\n#table(columns: 1, table.header(table.cell(colspan: 2)[H]), table.footer[F])',
'grid-control':'#grid(columns: 2, grid.header[H], [a], [b], grid.footer[F])',
}
class Observe(HTMLParser):
 def __init__(self):super().__init__();self.rows=[];self.cell=None;self.section=None;self.body=False;self.text=[]
 def handle_starttag(self,tag,attrs):
  if tag=='body':self.body=True
  if tag in ['thead','tbody','tfoot']:self.section=tag
  if tag=='tr':self.rows.append(dict(section=self.section,cells=[]))
  if tag in ['td','th']:self.cell=dict(tag=tag,attrs=dict(attrs),text='')
 def handle_data(self,data):
  if self.body:self.text.append(data)
  if self.cell is not None:self.cell['text']+=data
 def handle_endtag(self,tag):
  if tag in ['td','th'] and self.cell is not None:
   if self.rows:self.rows[-1]['cells'].append(self.cell)
   self.cell=None
  if tag in ['thead','tbody','tfoot']:self.section=None
  if tag=='body':self.body=False
bins={'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'};assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea';record=dict(start=now(),head=b['head'],source_hashes=b['hashes'],pin='9f2b6e8715237cb086899a42873660fe744622e8',binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},cases=[])
for name,text in cases.items():
 src=folder/(name+'.typ');src.write_text(text+'\n')
 for label,binary in bins.items():
  dst=folder/(name+'-'+label+'.html');cmd=[str(binary),'compile',str(src),str(dst),'--features','html','--format','html'];r=subprocess.run(cmd,capture_output=True,text=True,timeout=30);item=dict(name=name,binary=label,utc=now(),source=str(src),source_sha256=sha(src),argv=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
  if dst.exists():
   view=Observe();view.feed(dst.read_text());item.update(html=str(dst),html_sha256=sha(dst),rows=view.rows,body_text=''.join(view.text))
  record['cases'].append(item)
 record['end']=now()
 with gzip.open(out/'measurement.json.gz','wt') as f:json.dump(record,f)
record['sources_unchanged']=all(sha(p)==h for p,h in b['hashes'].items());assert record['sources_unchanged'];pairs={}
for item in record['cases']:pairs.setdefault(item['name'],{})[item['binary']]=item
summary=[]
for name,pair in pairs.items():
 a,z=pair['tekt'],pair['vanilla'];summary.append(dict(name=name,exit_tekt=a['exit'],exit_vanilla=z['exit'],rows_equal=a.get('rows')==z.get('rows'),text_equal=a.get('body_text')==z.get('body_text'),tekt=a.get('rows',a['stderr'].splitlines()[:4]),vanilla=z.get('rows',z['stderr'].splitlines()[:4])))
(out/'summary.json').write_text(json.dumps(summary,indent=2,ensure_ascii=False)+'\n')
with gzip.open(out/'measurement.json.gz','wt') as f:json.dump(record,f)
print(json.dumps(dict(cases=len(cases),runs=len(record['cases']),sources_unchanged=True,exits_differ=sum(x['exit_tekt']!=x['exit_vanilla'] for x in summary),rows_differ=sum(not x['rows_equal'] for x in summary))),flush=True)
