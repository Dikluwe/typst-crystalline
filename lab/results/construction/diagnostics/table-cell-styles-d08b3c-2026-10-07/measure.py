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
def query(name,source,expr='query(metadata).map(it=>it.value)'):cases[name]=(source,['eval','--in','main.typ','--format','json',expr])
def fields(name,source):query(name,'#show table.cell: it=>{metadata(it.fields());it}\n'+source)
def compile(name,source):cases[name]=(source,['compile','main.typ','out.pdf'])
fields('isolated-set','#set table.cell(fill:red,inset:3pt,breakable:false)\n#table.cell[a]')
fields('table-set','#set table.cell(fill:red,inset:3pt,breakable:false)\n#table(columns:2,[a],[b])')
for field,value in {'x':'2','y':'1','colspan':'2','rowspan':'2','fill':'red','stroke':'none','align':'right','inset':'3pt','breakable':'false'}.items():
 fields('isolated-'+field,'#set table.cell('+field+':'+value+')\n#table.cell[a]')
 fields('table-'+field,'#set table.cell('+field+':'+value+')\n#table(columns:3,[a])')
for field in ['x','y','fill','align','inset','breakable']:
 fields('auto-'+field,'#set table.cell('+field+':auto)\n#table.cell[a]')
fields('explicit-wins','#set table.cell(fill:red,inset:3pt,breakable:false)\n#table.cell(fill:blue,inset:4pt,breakable:true)[a]')
fields('none-wins','#set table.cell(fill:red,stroke:2pt)\n#table(columns:1,fill:green,stroke:3pt,table.cell(fill:none,stroke:none)[a])')
fields('auto-table-fallback','#set table.cell(fill:red,inset:3pt,align:right,breakable:false)\n#table(columns:1,fill:green,inset:7pt,align:left,table.cell(fill:auto,inset:auto,align:auto,breakable:auto)[a])')
fields('alias-set','#let c=table.cell\n#set c(fill:red)\n#c[a]')
fields('late-set','#let c=table.cell[a]\n#set table.cell(fill:red)\n#c')
fields('local-set','#table.cell[a]\n#{set table.cell(fill:red);table.cell[b]}\n#table.cell[c]')
fields('reused-different-scopes','#let c=table.cell[a]\n#{set table.cell(fill:red);c}\n#{set table.cell(fill:blue);c}')
fields('raw-fields-unchanged','#set table.cell(fill:red)\n#metadata(table.cell[a].fields())')
fields('padding-span-one','#set table.cell(colspan:2)\n#table(columns:3,[a])')
fields('empty-fixed-rows','#set table.cell(fill:red,colspan:2,rowspan:2)\n#table(columns:3,rows:(20pt,20pt))')
fields('stroke-fold','#set table.cell(stroke:red)\n#table(columns:1,stroke:2pt,[a])')
fields('align-fold','#set table.cell(align:bottom)\n#table(columns:1,align:right,[a])')
fields('inset-dict','#set table.cell(inset:(left:3pt,top:2pt))\n#table.cell[a]')
fields('show-set-cell','#show table: set table.cell(fill:red)\n#table(columns:1,[a])\n#table.cell[b]')
fields('show-cell-set-self','#show table.cell: set table.cell(fill:red)\n#table.cell[a]\n#table(columns:1,[b])')
fields('show-cell-where-style','#set table.cell(fill:red)\n#show table.cell.where(fill:red): it=>{metadata([RED]);it}\n#table.cell[a]')
query('evaluation-once','#set table.cell(fill:{metadata([SET]);red})\n#show table.cell: it=>{metadata([CELL]);it}\n#table(columns:2,[a],[b])')
compile('alias-inside-table','#show table.cell: it=>table(columns:1,it)\n#table.cell[a]')
for field,value in {'x':'-1','y':'none','colspan':'0','rowspan':'auto','fill':'false','stroke':'auto','align':'none','inset':'none','breakable':'0','body':'[a]','unknown':'1'}.items():
 compile('invalid-'+field,'#set table.cell('+field+':'+value+')\n#table.cell[a]')
compile('invalid-positional','#set table.cell([a])\n#table.cell[b]')
fields('conditional-set','#set table.cell(fill:red) if false\n#table.cell[a]')
fields('scoped-set-and-table','#set table.cell(fill:red)\n#table(columns:1,[a])\n#{set table.cell(fill:blue);table(columns:1,[b])}\n#table(columns:1,[c])')
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
