from pathlib import Path
import subprocess,hashlib,gzip,json,datetime,os,sys,shutil
R=Path.cwd();D=Path(__file__).parent;W=Path(os.environ.get('D08B3B_WORKDIR','/tmp/d08b3c-controls-main'));W.mkdir(exist_ok=True)
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
def compile(name,source):cases[name]=(source,['compile','main.typ','out.pdf'])
def fields(name,source):query(name,'#show table.cell: it=>{metadata(it.fields());it}\n'+source)
fields('isolated-default','#table.cell[a]')
fields('isolated-explicit','#table.cell(x:2,y:3,colspan:2,rowspan:3,fill:red,stroke:none,align:left,inset:3pt,breakable:false)[a]')
fields('isolated-auto','#table.cell(x:auto,y:auto,fill:auto,align:auto,inset:auto,breakable:auto)[a]')
fields('isolated-none','#table.cell(fill:none,stroke:none)[a]')
fields('isolated-table-style','#set table(fill:red,inset:8pt,stroke:none)\n#table.cell[a]')
fields('table-control','#table(columns:1,[a])')
fields('reused','#let c=table.cell[a]\n#c\n#c')
fields('late','#table.cell[a]\n#show table.cell: it=>{metadata(it.fields());it}\n#table.cell[b]')
query('raw-fields','', 'table.cell([a]).fields()')
query('raw-x','', 'table.cell([a]).x')
query('isolated-x','#show table.cell: it=>{metadata((it.x,it.y,it.fill,it.align,it.inset,it.breakable,it.colspan,it.rowspan));it.body}\n#table.cell[a]')
query('raw-vs-prepared','#let c=table.cell[a]\n#show table.cell: it=>{metadata((it==c,it.func()==table.cell));it.body}\n#c')
query('isolated-hidden','#show heading: it=>panic("hidden")\n#show table.cell: it=>[REPLACED]\n#table.cell[= hidden]')
for name,recipe in {'identity':'it','body':'it.body','alias':'{let alias=it;alias}','fresh':'table.cell(it.body)','fresh-growing':'table.cell([X#it.body])','fresh-block':'block(table.cell(it.body))','fresh-styled':'text(red,table.cell(it.body))','fresh-conditional':'if it.body==[a] {table.cell([b])} else {it.body}','fresh-hidden':'{let c=table.cell(it.body);[HIDDEN]}','nested-table':'if it.body==[a] {table(columns:1,[b])} else {it.body}'}.items():
 for place,body in {'isolated':'#table.cell[a]','table':'#table(columns:1,[a])'}.items():compile(name+'-'+place,'#show table.cell: it=>'+recipe+'\n'+body)
compile('nested-isolated','#show table.cell: it=>[CELL #it.body]\n#table.cell[#table.cell[a]]')
compile('selector-auto','#show table.cell.where(x:auto): it=>[AUTO #it.body]\n#table.cell[a]\n#table(columns:1,[b])')
compile('selector-zero','#show table.cell.where(x:0): it=>[ZERO #it.body]\n#table.cell[a]\n#table(columns:1,[b])')
fields('cell-set', '#set table.cell(fill:red,inset:3pt,breakable:false)\n#table.cell[a]')
query('show-set', '#show table.cell: set text(red)\n#show table.cell: it=>{metadata(it.fields());it}\n#table.cell[a]')
query('nested-effects', '#show table.cell: it=>{metadata(it.body);it}\n#table.cell[#table.cell[a]]')
query('hidden-fresh-effects', '#show table.cell: it=>{let fresh=table.cell[it.body];metadata([ONE]);[HIDDEN]}\n#table.cell[a]')
query('body-effects', '#show heading: it=>{metadata([HEADING]);it.body}\n#show table.cell: it=>{metadata([CELL]);it}\n#table.cell[= a]')
query('hidden-body-effects', '#show heading: it=>{metadata([HEADING]);it.body}\n#show table.cell: it=>{metadata([CELL]);[HIDDEN]}\n#table.cell[= a]')
query('isolated-location', '#show table.cell: it=>{metadata(it.location());it}\n#table.cell[a]')
query('isolated-query', '#table.cell[a]', 'query(table.cell)')
query('local-rule', '#show table.cell: it=>[OUT #it.body]\n#{show table.cell: it=>[LOCAL #it.body];table.cell[a]}\n#table.cell[b]')
query('late-rule', '#table.cell[a]\n#show table.cell: it=>{metadata(it.body);it}\n#table.cell[b]')
cases['html-fields']=('#show table.cell: it=>[X #it.x #it.body]\n#table.cell[a]', ['compile','main.typ','out.html','--features','html'])
for name, rules in {'multi-return':'#show table.cell: it=>[A #it.body]\n#show table.cell: it=>[B #it]', 'multi-identity':'#show table.cell: it=>[A #it.body]\n#show table.cell: it=>it', 'multi-fresh':'#show table.cell: it=>it.body\n#show table.cell: it=>table.cell(it.body)'}.items():
 for place,body in {'isolated':'#table.cell[a]','table':'#table(columns:1,[a])'}.items():compile(name+'-'+place,rules+'\n'+body)
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
