import datetime,gzip,hashlib,json,os,subprocess,xml.etree.ElementTree as ET
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2];O=Path('/tmp/p7-snapshot-cli');O.mkdir(exist_ok=True)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
git=lambda *a:subprocess.check_output(['git',*a],cwd=R,text=True)
engines={'previous':Path('/tmp/p7-snapshot-before-17e72149'),'current':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
env=dict(os.environ,RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1')
rec={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD'),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'binaries':{k:sha(v) for k,v in engines.items()},'cases':{}}
assert rec['binaries']['current'] != rec['binaries']['previous']
assert rec['binaries']['vanilla']=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
for f in sorted((D/'fixtures').glob('*.typ')):
 if f.stem.startswith('module'):continue
 group={}
 for name,exe in engines.items():
  pdf=O/(f.stem+'-'+name+'.pdf');cmd=[str(exe),'compile',str(f),str(pdf),'--pdf-tagged=false','--pretty=false']
  p=subprocess.run(cmd,cwd=R,env=env,capture_output=True,text=True,timeout=60)
  x={'command':cmd,'exit':p.returncode,'stderr':p.stderr}
  if p.returncode==0:
   bbox=subprocess.check_output(['pdftotext','-bbox',str(pdf),'-'],text=True);root=ET.fromstring(bbox);ns={'x':'http://www.w3.org/1999/xhtml'}
   x['pages']=[{'dimensions':page.attrib,'words':[{'box':w.attrib,'text':w.text} for w in page.findall('.//x:word',ns)]} for page in root.findall('.//x:page',ns)]
   x['text']=subprocess.check_output(['pdftotext','-layout',str(pdf),'-'],text=True)
   x['fonts']=subprocess.check_output(['pdffonts',str(pdf)],text=True)
   prefix=O/(f.stem+'-'+name+'-raster');subprocess.run(['pdftoppm','-r','36',str(pdf),str(prefix)],capture_output=True,check=True)
   x['rasters']=[sha(v) for v in sorted(O.glob(prefix.name+'-*.ppm'))]
  group[name]=x
 a,b=group['previous'],group['current']
 equal=all(a.get(k)==b.get(k) for k in ['exit','text','pages','rasters','stderr'])
 rec['cases'][f.stem]={'source_sha256':sha(f),'engines':group,'previous_current_equal':equal,'vanilla_text_equal':b.get('text')==group['vanilla'].get('text')}
 print(f.stem,equal,'vanilla text',rec['cases'][f.stem]['vanilla_text_equal'],flush=True)
 assert equal and (a['exit']!=0 if f.stem.endswith('error') else a['exit']==0),(f.stem,a.get('stderr'),b.get('stderr'))
rec['candidate_state_sha256']=hashlib.sha256((D/'final-state.json.gz').read_bytes()).hexdigest()
rec['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(D/'geometry.json.gz','wt') as f:json.dump(rec,f,ensure_ascii=False,indent=2)
