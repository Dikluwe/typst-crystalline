from pathlib import Path
from html.parser import HTMLParser
import datetime,gzip,hashlib,json,subprocess,sys
out=Path(__file__).resolve().parents[1];root=Path.cwd();phase=sys.argv[1];assert phase in ['before','after'];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes=b['hashes'] if phase=='before' else {p:sha(p) for p in b['hashes']};assert all(sha(p)==h for p,h in hashes.items());folder=out/('cli-controls-'+phase);folder.mkdir(exist_ok=False)
cases={'invalid-show-int': '#show strong: 1\n*gone*', 'invalid-show-auto': '#show strong: auto\n*gone*', 'hidden-footer-content': '#show table.cell: it => panic("hidden-cell")\n#show table: []\n#table(columns: 2, table.footer([a#strong[b]]))', 'hidden-header-content': '#let h = table.cell(colspan: 2)[H]\n#show table.cell: it => panic("hidden-cell")\n#show table: []\n#table(columns: 2, table.header(h))'}
class Table(HTMLParser):
 def __init__(self):super().__init__();self.rows=[];self.cell=None
 def handle_starttag(self,tag,attrs):
  if tag=='tr':self.rows.append([])
  if tag in ['td','th']:self.cell=dict(tag=tag,attrs=dict(attrs),text='')
 def handle_data(self,data):
  if self.cell is not None:self.cell['text']+=data
 def handle_endtag(self,tag):
  if tag in ['td','th'] and self.cell is not None:
   if self.rows:self.rows[-1].append(self.cell)
   self.cell=None
bins={'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'};assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea';record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=hashes,pin='9f2b6e8715237cb086899a42873660fe744622e8',binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},cases=[])
for name,text in cases.items():
 src=folder/(name+'.typ');src.write_text(text+'\n')
 for label,binary in bins.items():
  target=folder/(name+'-'+label+'.html');cmd=[str(binary),'compile',str(src),str(target),'--features','html','--format','html'];r=subprocess.run(cmd,capture_output=True,text=True);item=dict(name=name,binary=label,utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source=str(src),source_sha256=sha(src),command=cmd,exit=r.returncode,stdout=r.stdout,stderr=r.stderr)
  if target.exists():
   parser=Table();parser.feed(target.read_text());item.update(output=str(target),output_sha256=sha(target),rows=parser.rows)
  record['cases'].append(item)
record.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources_unchanged=all(sha(p)==h for p,h in hashes.items()));(out/('cli-controls-'+phase+'.json')).write_text(json.dumps(record,indent=2)+'\n');assert record['sources_unchanged'];print(json.dumps([dict(name=x['name'],binary=x['binary'],exit=x['exit'],rows=x.get('rows'),stderr=x['stderr'] if x['exit'] else '') for x in record['cases']],indent=2),flush=True)
