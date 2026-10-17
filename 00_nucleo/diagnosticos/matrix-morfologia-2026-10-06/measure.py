from pathlib import Path
import subprocess,hashlib,gzip,json,datetime,os,sys,shutil
R=Path('/repos/Antigravity/typst-crystalline');D=R/'00_nucleo/diagnosticos/matrix-morfologia-2026-10-06';W=Path('/tmp/tekt-matrix-morfologia');W.mkdir(exist_ok=True)
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
cases={'single-json': ('', ['eval', '--format', 'json', 'math.mat([a])'], None), 'single-fields': ('', ['eval', 'math.mat([a]).fields()'], None), 'two-json': ('', ['eval', '--format', 'json', 'math.mat(([a],[b]),([c],[d]))'], None), 'two-fields': ('', ['eval', 'math.mat(([a],[b]),([c],[d])).fields()'], None), 'ragged-json': ('', ['eval', '--format', 'json', 'math.mat(([a],[b]),([c],))'], None), 'ragged-fields': ('', ['eval', 'math.mat(([a],[b]),([c],)).fields()'], None), 'empty-json': ('', ['eval', '--format', 'json', 'math.mat()'], None), 'empty-fields': ('', ['eval', 'math.mat().fields()'], None), 'options-json': ('', ['eval', '--format', 'json', 'math.mat(delim:none, align:left, augment:none, gap:1em, ([a],[b]))'], None), 'options-fields': ('', ['eval', 'math.mat(delim:none, align:left, augment:none, gap:1em, ([a],[b])).fields()'], None), 'augment-json': ('', ['eval', '--format', 'json', 'math.mat(augment:(hline:1, vline:1, stroke:2pt), ([a],[b]),([c],[d]))'], None), 'augment-fields': ('', ['eval', 'math.mat(augment:(hline:1, vline:1, stroke:2pt), ([a],[b]),([c],[d])).fields()'], None), 'field-rows': ('', ['eval', 'math.mat([a]).rows'], None), 'field-delim': ('', ['eval', 'math.mat([a]).delim'], None), 'field-align': ('', ['eval', 'math.mat([a]).align'], None), 'field-augment': ('', ['eval', 'math.mat([a]).augment'], None), 'field-row-gap': ('', ['eval', 'math.mat([a]).row-gap'], None), 'field-column-gap': ('', ['eval', 'math.mat([a]).column-gap'], None), 'field-gap': ('', ['eval', 'math.mat([a]).gap'], None), 'field-missing': ('', ['eval', 'math.mat([a]).missing'], None), 'func': ('', ['eval', 'math.mat([a]).func()'], None), 'func-equality': ('', ['eval', 'math.mat([a]).func()==math.mat'], None), 'keys': ('', ['eval', 'math.mat([a]).fields().keys()'], None), 'has-omitted': ('', ['eval', 'math.mat([a]).has("delim")'], None), 'has-explicit': ('', ['eval', 'math.mat(delim:none,[a]).has("delim")'], None), 'at-default': ('', ['eval', 'math.mat([a]).at("delim",default:42)'], None), 'query-inline': ('$mat(a,b;c,d)$', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ'), 'query-bare': ('#math.mat(([a],[b]),([c],[d]))', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ'), 'query-options': ('$mat(delim:none, align:left, augment:none,a,b;c,d)$', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ')}

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
