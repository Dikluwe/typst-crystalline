from pathlib import Path
from html.parser import HTMLParser
import json,gzip,subprocess,hashlib,datetime
p=Path(__file__).resolve().parent;root=p.parents[2];work=p/'outputs';now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
class Text(HTMLParser):
 def __init__(self):super().__init__();self.parts=[];self.skip=0
 def handle_starttag(self,tag,attrs):
  if tag in ['style','script','title']:self.skip+=1
 def handle_endtag(self,tag):
  if tag in ['style','script','title']:self.skip-=1
 def handle_data(self,data):
  if not self.skip:self.parts.append(data)
sources={'empty-footnotes': '#let f=[].func()\n#show f.where(children:()):it=>[FOOTER]\nBODY', 'custom-body': '#let f=[].func()\n#show f.where(children:()):it=>[FOOTER]\n#html.body[BODY]'}
bins={'tekt':root/'target/debug/typst','vanilla':root/'lab/typst-original/target/release/typst'};rows=[]
for label,source in sources.items():
 f=work/('target-'+label+'.typ');f.write_text(source+'\n')
 for target in ['pdf','html']:
  row={'label':label,'target':target,'source':source,'source_sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'results':{}}
  for name,binary in bins.items():
   out=work/('target-'+label+'-'+name+'.'+target);cmd=[str(binary),'compile',str(f),str(out)]+(['--features','html'] if target=='html' else []);start=now();r=subprocess.run(cmd,text=True,capture_output=True,timeout=45);text=''
   if r.returncode==0:
    if target=='pdf':text=subprocess.run(['pdftotext',str(out),'-'],text=True,capture_output=True).stdout
    else:
     parser=Text();parser.feed(out.read_text());text=' '.join(parser.parts)
   row['results'][name]={'command':cmd,'start':start,'end':now(),'exit':r.returncode,'stderr':r.stderr,'text':' '.join(text.split())}
  rows.append(row);print(label,target,[(n,v['exit'],v['text']) for n,v in row['results'].items()],flush=True)
(p/'footnotes-controls.json.gz').write_bytes(gzip.compress(json.dumps({'utc':now(),'pin':'bd5ebec285505889e8ff1d4f68804ab147a699f3','head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'binaries':{k:{'path':str(v),'sha256':hashlib.sha256(v.read_bytes()).hexdigest()} for k,v in bins.items()},'cases':rows},indent=2).encode()))
