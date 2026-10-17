from pathlib import Path
import subprocess,json,hashlib,datetime,gzip,os,sys,shutil,xml.etree.ElementTree as ET
R=Path.cwd();D=Path(__file__).parent;W=Path('/tmp/tekt-raw-block-spacing');W.mkdir(exist_ok=True)
B={}
for k,p in [('tekt',R/'target/release/typst'),('vanilla',R/'lab/typst-original/target/release/typst')]:
 h=hashlib.sha256(p.read_bytes()).hexdigest();f=W/(k+'-'+h)
 if not f.exists():shutil.copyfile(p,f);f.chmod(0o755)
 B[k]=f
assert hashlib.sha256(B['vanilla'].read_bytes()).hexdigest()=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
cases={'sizes-up': 'Before\n\n#text(size:20pt)[After]', 'sizes-down': '#text(size:20pt)[Before]\n\nAfter', 'strong-cell': '#table(columns:1,strong[```txt\nabc\n```],[After])', 'plain-cell': '#table(columns:1,[```txt\nabc\n```],[After])'}
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'status':subprocess.check_output(['git','status','--short'],text=True),'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','bin_sha256':{k:hashlib.sha256(p.read_bytes()).hexdigest() for k,p in B.items()},'cases':{}}
for n,s in cases.items():
 row={'source':s,'runs':{}}
 for k,p in B.items():
  cw=W/n/k;cw.mkdir(parents=True,exist_ok=True);(cw/'main.typ').write_text(s);cmd=[str(p),'compile','main.typ','out.pdf'];env={**os.environ,'NO_COLOR':'1','SOURCE_DATE_EPOCH':'0','RAYON_NUM_THREADS':'1'};r=subprocess.run(cmd,cwd=cw,env=env,text=True,capture_output=True,timeout=45)
  v={'command':cmd,'cwd':str(cw),'status':r.returncode,'stdout':r.stdout,'stderr':r.stderr}
  if r.returncode==0:
   v['pdf_sha256']=hashlib.sha256((cw/'out.pdf').read_bytes()).hexdigest();v['text']=subprocess.check_output(['pdftotext',str(cw/'out.pdf'),'-'],text=True);xml=subprocess.check_output(['pdftohtml','-xml','-stdout','-i',str(cw/'out.pdf')],text=True);tree=ET.fromstring(xml);v['fonts']=[e.attrib for e in tree.iter('fontspec')];v['text_runs']=[{'attrs':e.attrib,'text':''.join(e.itertext())} for e in tree.iter('text')];box=subprocess.check_output(['pdftotext','-bbox',str(cw/'out.pdf'),'-'],text=True);v['words']=[{'text':e.text,'box':e.attrib} for e in ET.fromstring(box).iter() if e.tag.endswith('word')]
  row['runs'][k]=v
 d['cases'][n]=row;print(n,{k:(v['status'],v.get('fonts')) for k,v in row['runs'].items()},flush=True)
d['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat();(D/(sys.argv[1]+'.json.gz')).write_bytes(gzip.compress(json.dumps(d,ensure_ascii=False,indent=2).encode()))
