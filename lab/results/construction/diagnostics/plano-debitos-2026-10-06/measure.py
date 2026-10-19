from pathlib import Path
import subprocess,hashlib,gzip,json,datetime,os,sys
R=Path('/repos/Antigravity/typst-crystalline');D=R/'00_nucleo/diagnosticos/plano-debitos-2026-10-06';W=Path('/tmp/tekt-plano-debitos');W.mkdir(exist_ok=True)
B={'tekt':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def git(*args):return subprocess.check_output(['git',*args],cwd=R,text=True)
assert sha(B['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
files=git('ls-files','--modified').splitlines()+git('ls-files','--others','--exclude-standard','01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts').splitlines()
cases={
'output-required':('Olá', ['compile','-'],None),
'output-required-svg':('Olá',['compile','-','--format','svg'],None),
'output-required-bundle':('#document("one.pdf")[Olá]',['compile','-','--format','bundle','--features','bundle'],None),
'output-required-alias':('Olá',['c','-'],None),
'output-required-implicit':('Olá',['-'],None),
'query-clean':('= Heading',['query','main.typ','heading'], 'main.typ'),
'query-warning':('**\n= Heading',['query','main.typ','heading'], 'main.typ'),
'query-error':('#unknown',['query','main.typ','heading'], 'main.typ'),
'query-warning-error':('**\n#unknown',['query','main.typ','heading'], 'main.typ'),
'query-stdin-error':('#unknown',['query','-','heading'],None),
'compile-warning-error':('**\n#unknown',['compile','main.typ','out.pdf'], 'main.typ'),
'compile-warning-clean':('**\nOlá',['compile','main.typ','out.pdf'], 'main.typ'),
'matrix-outside':('#math.mat(a)',['compile','main.typ','out.pdf'],'main.typ'),
'frac-outside':('#math.frac(1,2)',['compile','main.typ','out.pdf'],'main.typ'),
'matrix-inside':('$mat(a)$',['compile','main.typ','out.pdf'],'main.typ'),
'matrix-query':('$mat(a)$',['eval','--in','main.typ','query(math.equation).map(it => it.body)'],'main.typ'),
'equation-query':('$a$',['eval','--in','main.typ','query(math.equation).len()'],'main.typ'),
'cell-query':('#table(columns: 1, [A])',['eval','--in','main.typ','query(table.cell).len()'],'main.typ'),
}
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'diff_stat':git('diff','HEAD','--stat'),'status':git('status','--short'),'source_hashes':{p:sha(R/p) for p in files if (R/p).is_file()},'bin_sha256':{k:sha(p) for k,p in B.items()},'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','cases':{}}
for name,(source,args,file) in cases.items():
 if file:(W/file).write_text(source)
 row={'source':source,'cwd':str(W),'runs':{}}
 for key,bin in B.items():
  (W/'out.pdf').unlink(missing_ok=True)
  cmd=[str(bin),*args];r=subprocess.run(cmd,cwd=W,input=None if file else source.encode(),capture_output=True,timeout=45,env={**os.environ,'NO_COLOR':'1','SOURCE_DATE_EPOCH':'0','RAYON_NUM_THREADS':'1'})
  v={'command':cmd,'status':r.returncode,'stderr':r.stderr.decode(),'stdout':r.stdout.decode(errors='replace')}
  if (W/'out.pdf').exists():v['pdf_text']=subprocess.check_output(['pdftotext',str(W/'out.pdf'),'-'],text=True)
  row['runs'][key]=v
 d['cases'][name]=row
 print(name, {k:(v['status'],v['stderr'],v['stdout'],v.get('pdf_text')) for k,v in row['runs'].items()},flush=True)
d['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat();(D/(sys.argv[1]+'.json.gz')).write_bytes(gzip.compress(json.dumps(d,ensure_ascii=False,indent=2).encode()))
