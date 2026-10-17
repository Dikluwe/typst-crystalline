from pathlib import Path
import json,gzip,datetime,hashlib,subprocess,os,re,xml.etree.ElementTree as E
R=Path.cwd();H=R/'00_nucleo/diagnosticos/rtl-numeros-2026-10-05';T=Path('/tmp/tekt-rtl-numeros');H.mkdir(exist_ok=True);T.mkdir(exist_ok=True)
git=lambda *a:subprocess.check_output(['git',*a],text=True);names=git('diff','--name-only').splitlines()+git('ls-files','--others','--exclude-standard').splitlines();names=sorted(set(n for n in names if n.startswith(('01_core/','03_infra/','00_nucleo/prompts/')) and n.endswith(('.rs','.md','.toml'))))
bins={'tekt':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'sources':{n:(R/n).read_text() for n in names},'binaries':{k:hashlib.sha256(p.read_bytes()).hexdigest() for k,p in bins.items()},'cases':{}}
env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1')
for n in [1,2,4,8,12,16]:
 for suffix in ['joined','spaced']:
  source='#set page(width: 500pt, height: 1600pt)\n#set text(font: "DejaVu Sans", dir: rtl, lang: "ar")\n'+' '.join('مرحبا بالعالم كلمة'+(' ' if suffix=='spaced' else '')+str(i) for i in range(n));name=f'{n}_{suffix}';f=T/(name+'.typ');f.write_text(source);case={'source':source,'engines':{}}
  for k,b in bins.items():
   p=T/(name+'-'+k+'.pdf');cmd=[str(b),'compile',str(f),str(p),'--pdf-tagged=false','--pretty=false'];r=subprocess.run(cmd,env=env,capture_output=True,text=True);assert r.returncode==0,r.stderr
   text=subprocess.check_output(['pdftotext',str(p),'-'],text=True);xml=subprocess.check_output(['pdftotext','-bbox',str(p),'-'],text=True);words=[{'text':e.text,**e.attrib} for e in E.fromstring(xml).iter() if e.tag.endswith('}word')];case['engines'][k]={'command':cmd,'stderr':r.stderr,'text':text,'bbox':xml,'words':words,'numbers':re.findall(r'\d+',text)}
  a,b=case['engines'].values();case['same_number_order']=a['numbers']==b['numbers'];case['same_text']=re.sub(r'[\s\u202a-\u202e]','',a['text'])==re.sub(r'[\s\u202a-\u202e]','',b['text']);d['cases'][name]=case;print(name,case['same_text'],a['numbers'],b['numbers'],flush=True)
gzip.open(H/'initial.json.gz','wt').write(json.dumps(d,ensure_ascii=False));(H/'audit.py').write_text(Path(__file__).read_text())
