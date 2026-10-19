from pathlib import Path
import subprocess,hashlib,gzip,json,datetime,os,sys,shutil
R=Path('/repos/Antigravity/typst-crystalline');D=R/'00_nucleo/diagnosticos/math-residuos-table-cell-2026-10-06';W=Path('/tmp/tekt-math-residuos-table-cell');W.mkdir(exist_ok=True)
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
cases={'bare-none': ('$mat(delim:none,a)$', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ'), 'code-none': ('$mat(delim:#none,a)$', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ'), 'bare-auto': ('$mat(delim:auto,a)$', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ'), 'code-auto': ('$mat(delim:#auto,a)$', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ'), 'bare-true': ('$mat(delim:true,a)$', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ'), 'code-true': ('$mat(delim:#true,a)$', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ'), 'bare-false': ('$mat(delim:false,a)$', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ'), 'code-false': ('$mat(delim:#false,a)$', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ'), 'bare-left': ('$mat(delim:left,a)$', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ'), 'code-left': ('$mat(delim:#left,a)$', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ'), 'bare-center': ('$mat(delim:center,a)$', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ'), 'code-center': ('$mat(delim:#center,a)$', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ'), 'bare-align-left': ('$mat(align:left,a)$', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ'), 'code-align-left': ('$mat(align:#left,a)$', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ'), 'local-left': ('#let left = [q]\n$mat(left)$', ['eval', '--in', 'main.typ', '--format', 'json', 'query(math.equation).map(it=>it.body)'], 'main.typ'), 'augment-array': ('', ['eval', 'math.mat(augment:(),[a])'], None), 'augment-array-keys': ('', ['eval', 'math.mat(augment:(),[a]).fields().keys()'], None), 'delim-int': ('', ['eval', 'math.mat(delim:2,[a])'], None), 'align-int': ('', ['eval', 'math.mat(align:2,[a])'], None), 'gap-str': ('', ['eval', 'math.mat(gap:"x",[a])'], None), 'cell-query': ('#table(columns:2,[a],[b])', ['eval', '--in', 'main.typ', 'query(table.cell)'], 'main.typ'), 'cell-selector': ('#table(columns:2,[a],[b])', ['eval', '--in', 'main.typ', 'selector(table.cell)'], 'main.typ'), 'cell-where': ('#table(columns:2,[a],[b])', ['eval', '--in', 'main.typ', 'table.cell.where(x:0)'], 'main.typ'), 'cell-selector-where': ('#table(columns:2,[a],[b])', ['eval', '--in', 'main.typ', 'selector(table.cell.where(x:0))'], 'main.typ'), 'cell-show': ('#show table.cell: it => [CELL #it.body]\n#table(columns:2,[a],[b])', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'cell-where-show': ('#show table.cell.where(x:0): it => [FIRST #it.body]\n#table(columns:2,[a],[b])', ['compile', 'main.typ', 'out.pdf'], 'main.typ')}
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
