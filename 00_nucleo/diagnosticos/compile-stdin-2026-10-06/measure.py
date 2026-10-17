import datetime, gzip, hashlib, json, os, subprocess, sys
from pathlib import Path
R=Path('/repos/Antigravity/typst-crystalline'); D=R/'00_nucleo/diagnosticos/compile-stdin-2026-10-06'; W=Path('/tmp/tekt-compile-stdin'); W.mkdir(exist_ok=True)
(W/'lib.typ').write_text('#let value = [Importado]\n'); (W/'data.txt').write_text('Recurso'); (W/'-').write_text('#unknown_physical\n'); (W/'project').mkdir(exist_ok=True); (W/'project/lib.typ').write_text('#let value = [Raiz]\n'); (W/'project/data.txt').write_text('Recurso da raiz')
bins={'tekt':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
def git(*args): return subprocess.check_output(['git',*args],cwd=R,text=True)
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
files=git('ls-files','--modified').splitlines()+git('ls-files','--others','--exclude-standard','01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts').splitlines()
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'working_tree':'uncommitted','diff_stat':git('diff','HEAD','--stat'),'status':git('status','--short'),'source_hashes':{p:sha(R/p) for p in files if (R/p).is_file()},'bin_sha256':{k:sha(p) for k,p in bins.items()},'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','cases':{}}
cases=[('pdf','Olá stdin\n',[], 'out.pdf'),('svg','Olá stdin\n',['--format','svg'],'out.svg'),('html','Olá stdin\n',['--features','html','--format','html'],'out.html'),('import_read','#import "lib.typ": value\n#value #read("data.txt")\n',[],'out.pdf'),('warning','**\n',[],'out.pdf'),('error','#unknown\n',[],'out.pdf'),('callback','#let bad() = unknown\n#context bad()\n',[],'out.pdf'),('stdout','Olá stdin\n',[],'-'),('invalid_utf8',b'\xff',[],'out.pdf'),('invalid_root','Olá\n',['--root','missing'],'out.pdf'),('no_output','Olá\n',[],None),('timings','Olá\n',['--timings','timings.json'],'out.pdf'),('compact','Olá stdin\n',['--compact'],'out.pdf'),('root','#import "lib.typ": value\n#value #read("data.txt")\n',['--root','project'],'out.pdf'),('root_error','#unknown\n',['--root','project'],'out.pdf'),('physical_stdout',None,[],'-')]
env={**os.environ,'NO_COLOR':'1','SOURCE_DATE_EPOCH':'0','RAYON_NUM_THREADS':'1'}
for name,source,flags,out in cases:
 row={'source_hex':source.hex() if isinstance(source,bytes) else source.encode().hex() if source is not None else None,'cwd':str(W),'runs':{}}
 for key,b in bins.items():
  dest=W/(key+'-'+out) if out not in ('-',None) else out
  if isinstance(dest,Path): dest.unlink(missing_ok=True)
  actual_flags=[('--timings-json' if key=='tekt' and f=='--timings' else f) for f in flags if not (key=='vanilla' and f=='--compact')]
  if source is None: (W/'main.typ').write_text('Olá arquivo'); input_path='main.typ'
  else: input_path='-'
  cmd=[str(b),'compile',input_path,*([str(dest)] if dest is not None else []),*actual_flags]
  result=subprocess.run(cmd,input=source if isinstance(source,bytes) or source is None else source.encode(),cwd=W,env=env,capture_output=True,timeout=60)
  v={'command':cmd,'status':result.returncode,'stderr':result.stderr.decode(errors='replace'),'stdout_hex':result.stdout.hex() if out!='-' else None}
  if isinstance(dest,Path) and dest.exists():
   v['artifact_sha256']=sha(dest)
   if dest.suffix=='.pdf': v['text']=subprocess.check_output(['pdftotext',str(dest),'-'],text=True);v['bbox_xml']=subprocess.check_output(['pdftotext','-bbox',str(dest),'-'],text=True)
   else: v['content']=dest.read_text()
  elif out=='-' and result.stdout.startswith(b'%PDF'):
   pdf=W/(key+'-stdout.pdf');pdf.write_bytes(result.stdout);v['pdf_stdout']=True;v['text']=subprocess.check_output(['pdftotext',str(pdf),'-'],text=True);v['bbox_xml']=subprocess.check_output(['pdftotext','-bbox',str(pdf),'-'],text=True)
  row['runs'][key]=v
 d['cases'][name]=row
out=D/(sys.argv[1]+'.json.gz'); out.write_bytes(gzip.compress(json.dumps(d,ensure_ascii=False,indent=2).encode()))
print(json.dumps({n:{k:{'status':v['status'],'stderr':v['stderr'],'text':v.get('text')} for k,v in r['runs'].items()} for n,r in d['cases'].items()},ensure_ascii=False,indent=2))
