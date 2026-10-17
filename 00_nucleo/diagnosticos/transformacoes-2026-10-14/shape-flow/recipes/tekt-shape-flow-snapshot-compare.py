from pathlib import Path
import subprocess,json,base64,hashlib,difflib,datetime
from PIL import Image,ImageChops
base=Path('00_nucleo/diagnosticos/transformacoes-2026-10-14/shape-flow/snapshots');rows=[]
for stem in ['04-shapes','07-multi-feature']:
 pdfs={key:base/(stem+'-'+key+'.pdf') for key in ['before','after']}
 objs={};texts={};imgs={};checks={}
 for key,pdf in pdfs.items():
  obj=json.loads(subprocess.check_output(['qpdf','--json','--json-stream-data=inline',str(pdf)],text=True));objs[key]=obj['qpdf'][1]
  (base/(stem+'-'+key+'-objects.json')).write_text(json.dumps(obj,indent=2))
  r=subprocess.run(['qpdf','--check',str(pdf)],capture_output=True,text=True);checks[key]=dict(exit=r.returncode,stdout=r.stdout,stderr=r.stderr);assert r.returncode==0
  texts[key]=subprocess.check_output(['pdftotext','-layout',str(pdf),'-'],text=True)
  (base/(stem+'-'+key+'.txt')).write_text(texts[key])
  png=pdf.with_suffix('.png');subprocess.run(['pdftocairo','-png','-singlefile','-r','144',str(pdf),str(png.with_suffix(''))],check=True,capture_output=True);imgs[key]=Image.open(png).convert('RGB')
 changes=[]
 for objid in objs['before'].keys()|objs['after'].keys():
  a=objs['before'].get(objid);b=objs['after'].get(objid)
  if a==b:continue
  entry=dict(object=objid,before=a,after=b)
  if a and b and 'stream' in a and 'stream' in b:
   ad=base64.b64decode(a['stream'].get('data',''));bd=base64.b64decode(b['stream'].get('data',''))
   try:entry['stream_diff']=''.join(difflib.unified_diff(ad.decode().splitlines(True),bd.decode().splitlines(True)))
   except UnicodeDecodeError:entry['binary_stream_changed']=ad!=bd
  changes.append(entry)
 a,b=imgs['before'],imgs['after'];assert a.size==b.size
 diff=ImageChops.difference(a,b);diff.save(base/(stem+'-delta.png'));pixels=[max(v) for v in diff.getdata()]
 row=dict(stem=stem,sha256={key:hashlib.sha256(p.read_bytes()).hexdigest() for key,p in pdfs.items()},sizes={key:p.stat().st_size for key,p in pdfs.items()},checks=checks,text_identical=texts['before']==texts['after'],text_words_identical=texts['before'].split()==texts['after'].split(),changed_objects=changes,max_channel=max(pixels),different_pixels=sum(v>0 for v in pixels));rows.append(row)
 print(stem,'objects',[x['object'] for x in changes],'words',row['text_words_identical'],flush=True)
 for e in changes:
  if 'stream_diff' in e:print(e['object'],e['stream_diff'],flush=True)
(base/'comparison.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),rows=rows),indent=2))
