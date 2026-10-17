from pathlib import Path
import json,gzip,datetime,hashlib,subprocess,os,re,xml.etree.ElementTree as E
from html.parser import HTMLParser
R=Path.cwd();H=R/'00_nucleo/diagnosticos/origem-fontes-expressao-2026-10-06';T=Path('/tmp/tekt-source-identity/macro');T.mkdir(exist_ok=True);bins={'tekt':Path('/tmp/tekt-source-identity/typst-final'),'vanilla':R/'lab/typst-original/target/release/typst'}
git=lambda *a:subprocess.check_output(['git',*a],text=True)
files=git('diff','--name-only').splitlines()+git('ls-files','--others','--exclude-standard').splitlines();files=[p for p in files if p.startswith(('01_core/','02_shell/','03_infra/','04_wiring/','00_nucleo/prompts/')) and p.endswith(('.rs','.md','.toml'))]
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'diff_stat':git('diff','HEAD','--stat'),'status':git('status','--short'),'source_hashes':{p:hashlib.sha256((R/p).read_bytes()).hexdigest() for p in files},'binaries':{k:hashlib.sha256(p.read_bytes()).hexdigest() for k,p in bins.items()},'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','cases':{}}
env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1',NO_COLOR='1');norm=lambda s:re.sub(r'[\s\u202a-\u202e]','',s or '')
for p,source in json.loads((H/'support.json').read_text()).items():(T/p).write_text(source)
def words(pdf):
 xml=subprocess.check_output(['pdftotext','-bbox',str(pdf),'-'],text=True);pages=[e for e in E.fromstring(xml).iter() if e.tag.endswith('}page')];ws=[{'text':norm(e.text),'page':i,**{k:float(v) for k,v in e.attrib.items()}} for i,p in enumerate(pages) for e in p.iter() if e.tag.endswith('}word')];groups=[]
 for page in range(len(pages)):
  lines=[]
  for w in [w for w in ws if w['page']==page]:
   line=next((l for l in lines if abs(l[0]['yMin']-w['yMin'])<.05),None)
   if line is None:lines.append([w])
   else:line.append(w)
  for line in lines:
   line.sort(key=lambda w:w['xMin']);gs=[]
   for w in line:
    if gs and abs(gs[-1]['xMax']-w['xMin'])<.05:gs[-1].update(text=gs[-1]['text']+w['text'],xMax=w['xMax'],yMin=min(gs[-1]['yMin'],w['yMin']),yMax=max(gs[-1]['yMax'],w['yMax']))
    else:gs.append(w.copy())
   groups.extend(gs)
 return xml,groups,[dict(p.attrib) for p in pages]
def compare(a,b):
 remaining=b.copy();deltas=[];missing=[]
 for w in a:
  choices=[z for z in remaining if z['text']==w['text'] and z['page']==w['page'] and abs(z['yMin']-w['yMin'])<.05]
  if not choices:missing.append(w);continue
  z=min(choices,key=lambda z:abs(z['xMin']-w['xMin']));remaining.remove(z);deltas.append(max(abs(w[p]-z[p]) for p in ['xMin','xMax','yMin','yMax']))
 return {'pass':not missing and not remaining and max(deltas,default=0)<.05,'max_delta_pt':max(deltas,default=0),'missing_tekt':missing,'missing_vanilla':remaining}
class Body(HTMLParser):
 def __init__(self):super().__init__();self.inside=False;self.nodes=[]
 def handle_starttag(self,t,a):
  if t=='body':self.inside=True
  if self.inside:self.nodes.append(['start',t,sorted(a)])
 def handle_endtag(self,t):
  if self.inside:self.nodes.append(['end',t])
  if t=='body':self.inside=False
 def handle_data(self,s):
  if self.inside and s.strip():self.nodes.append(['text',re.sub(r'\s+',' ',s)])
def ink(pdf,prefix):
 cmd=['pdftoppm','-f','1','-l','1','-singlefile','-r','600','-gray',str(pdf),str(prefix)];subprocess.run(cmd,check=True,capture_output=True);raw=Path(str(prefix)+'.pgm').read_bytes();b=raw.split(b'\n',3);w,h=map(int,b[1].split());pts={(i%w,i//w) for i,p in enumerate(b[3]) if p<180};return pts,{'command':cmd,'dpi':600,'threshold':180,'pgm_sha256':hashlib.sha256(raw).hexdigest(),'size':[w,h],'dark_pixels':len(pts)}
for name,c in json.loads((H/'manifest.json').read_text()).items():
 fp=T/(name+'.typ');fp.write_text(c['source']);entry={**c,'engines':{}};inks={}
 for k,binary in bins.items():
  ext='html' if c['mode']=='html' else 'pdf';out=T/(name+'-'+k+'.'+ext)
  if c['mode']=='query':cmd=[str(binary),'query',str(fp),c['selector'],'--field',c['field']]
  else:cmd=[str(binary),'compile',str(fp),str(out)]+(['--features','html','--format','html'] if c['mode']=='html' else ['--pdf-tagged=false','--pretty=false'])
  run=subprocess.run(cmd,env=env,capture_output=True,text=True);v={'command':cmd,'returncode':run.returncode,'stdout':run.stdout,'stderr':run.stderr}
  if run.returncode==0:
   if c['mode']=='query':
    try:v['query_value']=json.loads(run.stdout)
    except ValueError:v['query_parse_failure']=True
   elif c['mode']=='html':
    v['html']=out.read_text();parser=Body();parser.feed(v['html']);v['body_dom']=parser.nodes;v['artifact_sha256']=hashlib.sha256(out.read_bytes()).hexdigest()
   elif c['mode']!='error':
    v['text']=subprocess.check_output(['pdftotext',str(out),'-'],text=True);v['bbox'],v['words'],v['pages']=words(out);v['artifact_sha256']=hashlib.sha256(out.read_bytes()).hexdigest()
    if c['mode']=='raster' or c['domain']=='Matemática':inks[k],v['raster']=ink(out,T/(name+'-'+k+'-ink'))
  entry['engines'][k]=v
 a=entry['engines']['tekt'];b=entry['engines']['vanilla'];obs={}
 if c['mode']=='error':
  obs['both_reject']=a['returncode']!=0 and b['returncode']!=0;obs['exact_diagnostic_match']=a['stderr']==b['stderr'];entry['status']='MATCH' if obs['both_reject'] and obs['exact_diagnostic_match'] else 'DIFF'
 elif b['returncode']!=0:entry['status']='INVALID_PROBE'
 elif a['returncode']!=0:entry['status']='DIFF'
 elif c['mode']=='query':entry['status']='MATCH' if 'query_value' in a and a['query_value']==b.get('query_value') else 'DIFF'
 elif c['mode']=='html':entry['status']='MATCH' if a['body_dom'] and a['body_dom']==b['body_dom'] else 'DIFF'
 else:
  obs['text_match']=norm(a['text'])==norm(b['text']);obs['bbox']=compare(a['words'],b['words']);obs['pages_match']=a['pages']==b['pages'];obs['expected_text']=c['expect'] is None or (norm(a['text'])==norm(c['expect']) and norm(b['text'])==norm(c['expect']))
  if inks:
   unmatched=lambda a,b:sum(not any((x+dx,y+dy) in b for dx in [-1,0,1] for dy in [-1,0,1]) for x,y in a)
   obs['ink_unmatched_tekt_1px']=unmatched(inks['tekt'],inks['vanilla']);obs['ink_unmatched_vanilla_1px']=unmatched(inks['vanilla'],inks['tekt'])
  entry['status']='MATCH' if obs['text_match'] and obs['bbox']['pass'] and obs['pages_match'] and obs['expected_text'] and (not inks or obs['ink_unmatched_tekt_1px']==obs['ink_unmatched_vanilla_1px']==0) else 'DIFF'
 entry['observations']=obs;d['cases'][name]=entry;print(name,entry['status'],flush=True)
d['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(H/'macro.json.gz','wt') as f:json.dump(d,f,ensure_ascii=False)

