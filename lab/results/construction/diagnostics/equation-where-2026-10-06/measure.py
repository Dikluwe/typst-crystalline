from pathlib import Path
import subprocess,hashlib,gzip,json,datetime,os,sys,shutil
R=Path('/repos/Antigravity/typst-crystalline');D=R/'00_nucleo/diagnosticos/equation-where-2026-10-06';W=Path('/tmp/tekt-equation-where');W.mkdir(exist_ok=True)
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
cases={'empty': ('$a$\n$ b $ <display>', ['eval', '--in', 'main.typ', 'query(math.equation.where()).len()'], 'main.typ'), 'inline': ('$a$\n$ b $ <display>', ['eval', '--in', 'main.typ', 'query(math.equation.where(block: false)).len()'], 'main.typ'), 'block': ('$a$\n$ b $ <display>', ['eval', '--in', 'main.typ', 'query(math.equation.where(block: true)).len()'], 'main.typ'), 'alias': ('$a$\n$ b $ <display>', ['eval', '--in', 'main.typ', '{ let eq = math.equation; query(eq.where(block: true)).len() }'], 'main.typ'), 'label': ('$a$\n$ b $ <display>', ['eval', '--in', 'main.typ', 'query(math.equation.where(label: <display>)).len()'], 'main.typ'), 'typed-miss': ('$a$\n$ b $ <display>', ['eval', '--in', 'main.typ', 'query(math.equation.where(block: 2)).len()'], 'main.typ'), 'body': ('$a$\n$ b $ <display>', ['eval', '--in', 'main.typ', '{ let b = query(math.equation).first().body; query(math.equation.where(body: b)).len() }'], 'main.typ'), 'compound': ('$a$\n$ b $ <display>', ['eval', '--in', 'main.typ', 'query(math.equation.where(block: true) | math.equation.where(block: false)).len()'], 'main.typ'), 'baseline': ('$a$\n$ b $ <display>', ['eval', '--in', 'main.typ', 'query(math.equation).len()'], 'main.typ'), 'field-block': ('', ['eval', 'math.equation.where(block: true)'], None), 'field-numbering': ('', ['eval', 'math.equation.where(numbering: none)'], None), 'field-number-align': ('', ['eval', 'math.equation.where(number-align: end + horizon)'], None), 'field-supplement': ('', ['eval', 'math.equation.where(supplement: auto)'], None), 'field-alt': ('', ['eval', 'math.equation.where(alt: none)'], None), 'field-body': ('', ['eval', 'math.equation.where(body: [a])'], None), 'field-label': ('', ['eval', 'math.equation.where(label: <display>)'], None), 'field-size': ('', ['eval', 'math.equation.where(size: auto)'], None), 'field-missing': ('', ['eval', 'math.equation.where(missing: none)'], None), 'partial': ('', ['eval', 'math.equation.with(block: true).where(block: true)'], None), 'positional': ('', ['eval', 'math.equation.where(true)'], None), 'scalar': ('', ['eval', '(x => x).where(block: true)'], None), 'selector': ('', ['eval', 'selector(math.equation.where(block: true))'], None), 'styled-fields': ('#set math.equation(numbering: "(1)", alt: "equation")\n$ x $', ['eval', '--in', 'main.typ', '(query(math.equation.where(numbering: "(1)")).len(), query(math.equation.where(alt: "equation")).len())'], 'main.typ'), 'show-block': ('#show math.equation.where(block: true): it => [DISPLAY]\n$a$\n$ b $', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'show-inline': ('#show math.equation.where(block: false): it => [INLINE]\n$a$\n$ b $', ['compile', 'main.typ', 'out.pdf'], 'main.typ')}

cases['compound-valid']=('$a$\n$ b $', ['eval','--in','main.typ','query(math.equation.where(block: true).or(math.equation.where(block: false))).len()'],'main.typ')
for name,value in [('label-none','none'),('label-string','"display"'),('label-miss','<unknown>')]:
 cases[name]=('$a$\n$ b $ <display>',['eval','--in','main.typ',f'query(math.equation.where(label: {value})).len()'],'main.typ')

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
