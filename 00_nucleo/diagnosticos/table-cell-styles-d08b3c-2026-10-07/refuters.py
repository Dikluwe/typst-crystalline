from pathlib import Path
import subprocess,hashlib,gzip,json,datetime,os,sys,shutil
R=Path.cwd();D=Path(__file__).parent;W=Path(os.environ.get('D08B3B_WORKDIR','/tmp/d08b3c-styles'));W.mkdir(exist_ok=True)
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
def fields(n,s):cases[n]=('#show table.cell: it=>{metadata(it.fields());it}\n'+s,['eval','--in','main.typ','--format','json','query(metadata).map(it=>it.value)'])
def compile(n,s):cases[n]=(s,['compile','main.typ','out.pdf'])
fields('spread','#set table.cell(..(fill:red,inset:3pt))\n#table.cell[a]')
fields('false-skips-args','#set table.cell(fill:panic("IGNORED")) if false\n#table.cell[a]')
compile('all-args-before-cast','#set table.cell(x:-1,fill:panic("FIELD"))\n#table.cell[a]')
fields('headers','#set table.cell(fill:red)\n#table(columns:2,table.header([h1],[h2]),[a])')
compile('header-y-style-conflict','#set table.cell(y:0)\n#table(columns:1,table.header([h]),[a])')
fields('stroke-constructor-fold','#set table.cell(stroke:red)\n#table.cell(stroke:2pt)[a]')
fields('align-constructor-fold','#set table.cell(align:bottom)\n#table.cell(align:right)[a]')
fields('partial-inset-table','#set table.cell(inset:(left:3pt,top:2pt))\n#table(columns:1,inset:7pt,[a])')
fields('local-prepared-inside-table','#let c={set table.cell(fill:red);table.cell[a]}\n#set table.cell(fill:blue)\n#table(columns:1,c)')
fields('ctor-empty-inset-dict','#set table.cell(inset:3pt)\n#table.cell(inset:())[a]')
fields('self-set-does-not-change-placement','#show table.cell: set table.cell(colspan:2)\n#table(columns:3,[a])')
compile('one-argument-evaluation','#let f()={let ignored=metadata([SIDE]); red}\n#set table.cell(fill:f())\n#table.cell[a]')
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
