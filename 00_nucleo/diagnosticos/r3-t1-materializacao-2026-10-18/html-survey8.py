from pathlib import Path
from html.parser import HTMLParser
import json,gzip,subprocess,datetime,hashlib
p=Path(__file__).resolve().parent;r=p.parents[2];out=p/'outputs/html-after-candidate8';out.mkdir(parents=True,exist_ok=True)
sha=lambda f:hashlib.sha256(Path(f).read_bytes()).hexdigest()
class Text(HTMLParser):
 def __init__(self):super().__init__();self.body=False;self.parts=[]
 def handle_starttag(self,t,a):
  if t=='body':self.body=True
  if self.body and t in ['p','br','div']:self.parts.append(' ')
 def handle_endtag(self,t):
  if t=='body':self.body=False
  if self.body and t in ['p','div']:self.parts.append(' ')
 def handle_data(self,d):
  if self.body:self.parts.append(d)
bins={'tekt':r/'target/debug/typst','vanilla':r/'lab/typst-original/target/release/typst'}
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_state='candidate8-sources.json.gz',pin='bd5ebec285505889e8ff1d4f68804ab147a699f3',binaries={k:dict(path=str(v),sha256=sha(v)) for k,v in bins.items()},cases=[])
for case in json.loads((r/'00_nucleo/materialization/textual-realization-2026-10-18/acceptance.json').read_text()):
 f=out/(case['id']+'.typ');f.write_text(case['source']);assert sha(f)==case['source_sha256'];row={k:case[k] for k in ['id','label','source','source_sha256']};row['results']={}
 for name,bin in bins.items():
  target=f.with_name(f.stem+'-'+name+'.html');cmd=[str(bin),'compile',str(f),str(target),'--features','html'];z=subprocess.run(cmd,text=True,capture_output=True,timeout=60);result=dict(command=cmd,exit=z.returncode,stderr=z.stderr,text='')
  if z.returncode==0:
   parser=Text();parser.feed(target.read_text());result.update(sha256=sha(target),html=target.read_text(),text=' '.join(''.join(parser.parts).split()))
  row['results'][name]=result
 a,b=[row['results'][n] for n in ['tekt','vanilla']];row['same']=bool(a['exit']==b['exit'] and (a['text']==b['text'] if a['exit']==0 else a['stderr'].splitlines()[0]==b['stderr'].splitlines()[0]));record['cases'].append(row);print(case['id'],row['same'],[(n,v['exit'],v['text']) for n,v in row['results'].items()],flush=True)
(p/'html-after-candidate8.json.gz').write_bytes(gzip.compress(json.dumps(record,indent=2,ensure_ascii=False).encode()))
assert all(row['same'] for row in record['cases'])
