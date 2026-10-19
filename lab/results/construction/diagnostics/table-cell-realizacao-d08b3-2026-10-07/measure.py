from pathlib import Path
import subprocess,hashlib,gzip,json,datetime,os,sys,shutil
R=Path('/repos/Antigravity/typst-crystalline');D=R/'00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07';W=Path('/tmp/tekt-table-cell-d08b3');W.mkdir(exist_ok=True)
(W/'lib.typ').write_text('#let bad() = unknown\n')
B={'tekt':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
# Freeze executable paths so later cargo builds cannot replace a measured binary.
for key, executable in list(B.items()):
 digest = hashlib.sha256(executable.read_bytes()).hexdigest()
 frozen = W/'binaries'/f'{key}-{digest}'
 frozen.parent.mkdir(exist_ok=True)
 if not frozen.exists(): shutil.copyfile(executable, frozen); frozen.chmod(0o755)
 B[key] = frozen

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def git(*args):return subprocess.check_output(['git',*args],cwd=R,text=True)
assert sha(B['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
files=git('ls-files','--modified').splitlines()+git('ls-files','--others','--exclude-standard','01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts').splitlines()
cases={'raw': ('', ['eval', 'table.cell([a])'], None), 'fields': ('', ['eval', 'table.cell([a]).fields()'], None), 'body': ('', ['eval', 'table.cell([a]).body'], None), 'x-omitted': ('', ['eval', 'table.cell([a]).x'], None), 'x-auto': ('', ['eval', 'table.cell(x:auto)[a].fields()'], None), 'x-explicit': ('', ['eval', 'table.cell(x:0,y:1,colspan:2,rowspan:3)[a].fields()'], None), 'fill-none': ('', ['eval', 'table.cell(fill:none)[a].fields()'], None), 'stroke-none': ('', ['eval', 'table.cell(stroke:none)[a].fields()'], None), 'align-auto': ('', ['eval', 'table.cell(align:auto)[a].fields()'], None), 'inset-auto': ('', ['eval', 'table.cell(inset:auto)[a].fields()'], None), 'breakable-auto': ('', ['eval', 'table.cell(breakable:auto)[a].fields()'], None), 'func': ('', ['eval', 'table.cell([a]).func()'], None), 'func-eq': ('', ['eval', 'table.cell([a]).func()==table.cell'], None), 'fields-keys': ('', ['eval', 'table.cell([a]).fields().keys()'], None), 'raw-options': ('', ['eval', 'table.cell(fill:red,stroke:2pt,align:left,inset:3pt,breakable:false)[a].fields()'], None), 'show-bare': ('#show table.cell: it=>[CELL #it.body]\n#table(columns:2,[a],[b])', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'show-explicit': ('#show table.cell: it=>[CELL #it.body]\n#table(columns:2,table.cell([a]),table.cell([b]))', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'show-selector-explicit': ('#show selector(table.cell): it=>[CELL #it.body]\n#table(columns:2,table.cell([a]),table.cell([b]))', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'show-where-explicit': ('#show table.cell.where(x:0): it=>[FIRST #it.body]\n#table(columns:2,table.cell(x:0)[a],table.cell(x:1)[b])', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'show-padding': ('#show table.cell: it=>[CELL #it.body]\n#table(columns:2,[a],[b],[c])', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'show-outside': ('#show table.cell: it=>[CELL #it.body]\n#table.cell([a])', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'show-identity': ('#show table.cell: it=>it\n#table(columns:2,[a],[b])', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'show-body': ('#show table.cell: it=>it.body\n#table(columns:2,[a],[b])', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'local-scope': ('#table(columns:2,[a],[b])\n#{show table.cell: it=>[LOCAL #it.body]; table(columns:2,[c],[d])}\n#table(columns:2,[e],[f])', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'late-scope': ('#table(columns:2,[a],[b])\n#show table.cell: it=>[LATE #it.body]\n#table(columns:2,[c],[d])', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'reused': ('#let tab=table(columns:2,[a],[b])\n#show table.cell: it=>[CELL #it.body]\n#tab\n#tab', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'nested': ('#show table.cell: it=>[CELL #it.body]\n#table(columns:1,table(columns:1,[a]))', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'header-footer': ('#show table.cell: it=>[CELL #it.body]\n#table(columns:2,table.header([h1],[h2]),[a],[b],table.footer([f1],[f2]))', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'resolved': ('#show table.cell: it=>{metadata(it.fields());it}\n#table(columns:2,[a],[b])', ['eval', '--in', 'main.typ', '--format', 'json', 'query(metadata).map(it=>it.value)'], 'main.typ'), 'resolved-padding': ('#show table.cell: it=>{metadata(it.fields());it}\n#table(columns:2,[a],[b],[c])', ['eval', '--in', 'main.typ', '--format', 'json', 'query(metadata).map(it=>it.value)'], 'main.typ'), 'resolved-colspan': ('#show table.cell: it=>{metadata(it.fields());it}\n#table(columns:2,table.cell(colspan:2)[a],[b],[c])', ['eval', '--in', 'main.typ', '--format', 'json', 'query(metadata).map(it=>it.value)'], 'main.typ'), 'resolved-rowspan': ('#show table.cell: it=>{metadata(it.fields());it}\n#table(columns:2,table.cell(rowspan:2)[a],[b],[c])', ['eval', '--in', 'main.typ', '--format', 'json', 'query(metadata).map(it=>it.value)'], 'main.typ'), 'resolved-styles': ('#show table.cell: it=>{metadata(it.fields());it}\n#table(columns:2,fill:red,align:left,inset:3pt,stroke:none,[a],[b])', ['eval', '--in', 'main.typ', '--format', 'json', 'query(metadata).map(it=>it.value)'], 'main.typ'), 'resolved-explicit-position': ('#show table.cell: it=>{metadata(it.fields());it}\n#table(columns:2,[a],table.cell(x:0,y:1)[b],[c])', ['eval', '--in', 'main.typ', '--format', 'json', 'query(metadata).map(it=>it.value)'], 'main.typ'), 'resolved-fixed-rows': ('#show table.cell: it=>{metadata(it.fields());it}\n#table(columns:1,rows:(10pt,auto),[a],[b])', ['eval', '--in', 'main.typ', '--format', 'json', 'query(metadata).map(it=>it.value)'], 'main.typ')}

def resolved_case(name, source):
 cases[name]=( '#show table.cell: it=>{metadata(it.fields());it}\n'+source, ['eval','--in','main.typ','--format','json','query(metadata).map(it=>it.value)'], 'main.typ')
for name,source in {
 'order-auto-then-fixed':'#table(columns:2,[a],table.cell(x:0,y:0)[b])',
 'order-fixed-then-auto':'#table(columns:2,table.cell(x:0,y:0)[a],[b])',
 'order-x-only':'#table(columns:2,[a],table.cell(x:0)[b],[c])',
 'order-y-only':'#table(columns:2,[a],table.cell(y:1)[b],[c])',
 'position-gap':'#table(columns:2,table.cell(x:1,y:2)[a])',
 'row-padding':'#table(columns:2,rows:(auto,auto),[a])',
 'zero-children':'#table(columns:2)',
 'span-padding':'#table(columns:3,table.cell(colspan:2)[a],[b],[c])',
 'rowspan-fixed':'#table(columns:2,rows:(10pt,10pt),table.cell(rowspan:2)[a],[b],[c])',
 'cell-overrides':'#table(columns:2,fill:red,stroke:2pt,align:right,inset:4pt,table.cell(fill:none,stroke:none,align:left,inset:1pt,breakable:false)[a],table.cell(fill:auto,align:auto,inset:auto,breakable:auto)[b])',
 'stroke-inherited':'#table(columns:1,stroke:2pt,[a])',
 'stroke-cell-partial':'#table(columns:1,stroke:red,table.cell(stroke:2pt)[a])',
 'em-inset':'#set text(size:20pt)\n#table(columns:1,inset:1em,[a])',
 'fraction-rows':'#table(columns:1,rows:(1fr,auto),[a],[b])',
 'header-fields':'#table(columns:2,table.header([h1],[h2]),[a],[b],table.footer([f1],[f2]))',
 'invalid-column':'#table(columns:2,table.cell(x:2)[a])',
 'invalid-span':'#table(columns:2,table.cell(colspan:3)[a])',
 'alias-late-cell':'#let c=table.cell([a])\n#table(columns:1,c)',
}.items(): resolved_case(name,source)
for name,source in {
 'show-cell-late-alias':'#let c=table.cell([a])\n#show table.cell: it=>[NEW #it.body]\n#table(columns:1,c)',
 'show-table-body':'#show table: it=>it\n#show table.cell: it=>[CELL #it.body]\n#table(columns:2,[a],[b])',
 'show-generated-table':'#show strong: it=>table(columns:1,[a])\n#show table.cell: it=>[CELL #it.body]\n*trigger*',
 'show-cell-reconstruct':'#show table.cell: it=>table.cell(it.body)\n#table(columns:1,[a])',
 'show-cell-location':'#show table.cell: it=>[L#repr(it.location()) X#it.x Y#it.y]\n#table(columns:2,[a],[b])',
 'show-cell-filter-position':'#show table.cell.where(x:1): it=>[SECOND #it.body]\n#table(columns:2,[a],[b],[c],[d])',
 'show-cell-no-default-escape':'#show table.cell: it=>[X#it.x B#it.breakable]\n#table(columns:2,[a])',
 'context-table':'#show table.cell: it=>[CELL #it.body]\n#context table(columns:2,[a],[b])',
 'raw-cell-no-placement':'#show table.cell: it=>[X#it.x]\n#table.cell([a])',
}.items():cases[name]=(source,['compile','main.typ','out.pdf'],'main.typ')
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'diff_stat':git('diff','HEAD','--stat'),'status':git('status','--short'),'source_hashes':{p:sha(R/p) for p in files if (R/p).is_file()},'bin_sha256':{k:sha(p) for k,p in B.items()},'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','cases':{}}
for name,(source,args,file) in cases.items():
 if file:
  (W/file).parent.mkdir(parents=True,exist_ok=True);(W/file).write_text(source)
 row={'source':source,'cwd':str(W),'runs':{}}
 for key,bin in B.items():
  (W/'out.pdf').unlink(missing_ok=True);(W/'out.html').unlink(missing_ok=True)
  cmd=[str(bin),*args];r=subprocess.run(cmd,cwd=W,input=None if file else source.encode(),capture_output=True,timeout=45,env={**os.environ,'NO_COLOR':'1','SOURCE_DATE_EPOCH':'0','RAYON_NUM_THREADS':'1'})
  v={'command':cmd,'status':r.returncode,'stderr':r.stderr.decode(),'stdout':r.stdout.decode(errors='replace')}
  if (W/'out.pdf').exists():v['pdf_text']=subprocess.check_output(['pdftotext',str(W/'out.pdf'),'-'],text=True)
  if (W/'out.html').exists():v['html']=(W/'out.html').read_text()
  row['runs'][key]=v
 d['cases'][name]=row
 print(name, {k:(v['status'],v['stderr'],v['stdout'],v.get('pdf_text')) for k,v in row['runs'].items()},flush=True)
d['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat();(D/(sys.argv[1]+'.json.gz')).write_bytes(gzip.compress(json.dumps(d,ensure_ascii=False,indent=2).encode()))
