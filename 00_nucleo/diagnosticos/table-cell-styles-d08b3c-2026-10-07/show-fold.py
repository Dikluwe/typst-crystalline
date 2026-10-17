from pathlib import Path
import subprocess,hashlib,gzip,json,datetime,os,sys,shutil
R=Path.cwd();D=Path(__file__).parent;W=Path(os.environ.get('D08B3B_WORKDIR','/tmp/d08b3c-show-fold'));W.mkdir(exist_ok=True)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
B={k:R/p for k,p in {'tekt':'target/release/typst','vanilla':'lab/typst-original/target/release/typst'}.items()}
if os.environ.get('TEKT_MEASURE_BIN'):B['tekt']=Path(os.environ['TEKT_MEASURE_BIN'])
for k,p in list(B.items()):
 f=W/(k+'-'+sha(p))
 if not f.exists():shutil.copyfile(p,f);f.chmod(0o755)
 B[k]=f
assert sha(B['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
files=subprocess.check_output(['git','ls-files','--modified'],text=True).splitlines()+subprocess.check_output(['git','ls-files','--others','--exclude-standard','01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'],text=True).splitlines()
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'status':subprocess.check_output(['git','status','--short'],text=True),'source_hashes':{p:sha(R/p) for p in files if (R/p).is_file()},'bin_sha256':{k:sha(p) for k,p in B.items()},'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','environment_overrides':{'NO_COLOR':'1','SOURCE_DATE_EPOCH':'0','RAYON_NUM_THREADS':'1'},'cases':{}}
cases={}
cases['two-cell-show-sets']=('#show table.cell: it=>{metadata(it.fields());it}\n#show table.cell: set table.cell(stroke:red)\n#show table.cell: set table.cell(stroke:2pt)\n#table.cell[a]',['eval','--in','main.typ','--format','json','query(metadata).map(it=>it.value)'])
cases['two-table-show-sets']=('#show table.cell: it=>{metadata(it.fields());it}\n#show table: set table.cell(stroke:red)\n#show table: set table.cell(stroke:2pt)\n#table(columns:1,[a])',['eval','--in','main.typ','--format','json','query(metadata).map(it=>it.value)'])
for name,(source,args) in cases.items():
 (W/'main.typ').write_text(source);row={'source':source,'source_sha256':hashlib.sha256(source.encode()).hexdigest(),'cwd':str(W),'runs':{}}
 for k,b in B.items():
  (W/'out.pdf').unlink(missing_ok=True);(W/'out.html').unlink(missing_ok=True);cmd=[str(b),*args]
  try:
   r=subprocess.run(cmd,cwd=W,capture_output=True,text=True,timeout=30,env={**os.environ,**d['environment_overrides']});v={'command':cmd,'status':r.returncode,'stdout':r.stdout,'stderr':r.stderr}
   if (W/'out.html').exists():v['html']=(W/'out.html').read_text()
   if (W/'out.pdf').exists():v['pdf_text']=subprocess.check_output(['pdftotext',str(W/'out.pdf'),'-'],text=True)
  except subprocess.TimeoutExpired:v={'command':cmd,'timeout':30}
  row['runs'][k]=v
 d['cases'][name]=row
 print(name,{k:(v.get('status'),v.get('stdout'),v.get('pdf_text'),v.get('stderr','').splitlines()[:1]) for k,v in row['runs'].items()},flush=True)
d['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat();(D/(sys.argv[1]+'.json.gz')).write_bytes(gzip.compress(json.dumps(d,ensure_ascii=False,indent=2).encode()))
