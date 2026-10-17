from pathlib import Path
import json,gzip,datetime,hashlib,subprocess,os,re,xml.etree.ElementTree as E
H=Path(__file__).resolve().parent;R=H.parents[2];T=Path('/tmp/tekt-rtl-numeros');initial=json.load(gzip.open(H/'initial.json.gz','rt'))
bins={'baseline':T/'baseline-typst','candidate':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'binaries':{k:hashlib.sha256(p.read_bytes()).hexdigest() for k,p in bins.items()},'cases':{}}
env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1');norm=lambda s:re.sub(r'[\s\u202a-\u202e]','',s)
f={n:x['source'] for n,x in initial['cases'].items()};common='#set page(width: 500pt, height: 1600pt)\n#set text(font: "DejaVu Sans", dir: rtl, lang: "ar")\n'
f['160_joined']=common+' '.join('مرحبا بالعالم كلمة'+str(i) for i in range(160))
f['selected_160']=common+'#let c = counter(heading.where(level: 1))\n#context { let _ = c.final(); ['+' '.join('مرحبا بالعالم كلمة'+str(i) for i in range(160))+'] }'
f['explicit_breaks']=common+'مرحبا0\\\nعالم1\\\nكلمة2'
f['mixed_ltr']=common+'مرحبا0\\\n#text(dir: ltr)[ABC 123]\\\nكلمة1'
f['punctuation']=common+'مرحبا.\\\nبالعالم!\\\nكلمة؟'
f['ltr']='Latin words 123 with punctuation.\n\nSecond paragraph.'
for name,source in f.items():
 fpath=T/(name+'.typ');fpath.write_text(source);case={'source':source,'engines':{}}
 for k,b in bins.items():
  p=T/(name+'-'+k+'.pdf');cmd=[str(b),'compile',str(fpath),str(p),'--pdf-tagged=false','--pretty=false'];r=subprocess.run(cmd,env=env,capture_output=True,text=True);assert r.returncode==0,r.stderr
  text=subprocess.check_output(['pdftotext',str(p),'-'],text=True);xml=subprocess.check_output(['pdftotext','-bbox',str(p),'-'],text=True);words=[{'text':e.text,**e.attrib} for e in E.fromstring(xml).iter() if e.tag.endswith('}word')]
  stem=T/(name+'-'+k);subprocess.run(['pdftoppm','-r','36','-png',str(p),str(stem)],capture_output=True,check=True)
  case['engines'][k]={'command':cmd,'stderr':r.stderr,'text':text,'bbox':xml,'words':words,'numbers':re.findall(r'\d+',text),'rasters':[hashlib.sha256(x.read_bytes()).hexdigest() for x in sorted(T.glob(stem.name+'-*.png'))]}
 a,b,v=[case['engines'][k] for k in bins];case.update({'before_text_match':norm(a['text'])==norm(v['text']),'after_text_match':norm(b['text'])==norm(v['text']),'same_baseline_candidate_bbox':a['bbox']==b['bbox'],'same_baseline_candidate_raster':a['rasters']==b['rasters'],'same_number_order':b['numbers']==v['numbers']});d['cases'][name]=case
 print(name,'text',case['before_text_match'],'->',case['after_text_match'],'numbers',case['same_number_order'],'bbox unchanged',case['same_baseline_candidate_bbox'],flush=True)
gzip.open(H/'verification.json.gz','wt').write(json.dumps(d,ensure_ascii=False))
assert all(x['after_text_match'] and x['same_number_order'] for x in d['cases'].values())
