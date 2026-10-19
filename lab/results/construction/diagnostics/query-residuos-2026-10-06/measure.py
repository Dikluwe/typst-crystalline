from pathlib import Path
import subprocess,hashlib,gzip,json,datetime,os,sys
R=Path('/repos/Antigravity/typst-crystalline');D=R/'00_nucleo/diagnosticos/query-residuos-2026-10-06';W=Path('/tmp/tekt-query-residuos');W.mkdir(exist_ok=True)
(W/'lib.typ').write_text('#let bad() = unknown\n')
B={'tekt':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def git(*args):return subprocess.check_output(['git',*args],cwd=R,text=True)
assert sha(B['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
files=git('ls-files','--modified').splitlines()+git('ls-files','--others','--exclude-standard','01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts').splitlines()
cases={'query-clean': ('= Heading', ['query', 'main.typ', 'heading'], 'main.typ'), 'query-warning': ('**\n= Heading', ['query', 'main.typ', 'heading'], 'main.typ'), 'query-error': ('#unknown', ['query', 'main.typ', 'heading'], 'main.typ'), 'query-warning-error': ('**\n#unknown', ['query', 'main.typ', 'heading'], 'main.typ'), 'query-stdin-clean': ('= Heading', ['query', '-', 'heading'], None), 'query-stdin-error': ('#unknown', ['query', '-', 'heading'], None), 'query-stdin-warning-error': ('**\n#unknown', ['query', '-', 'heading'], None), 'query-stdin-callback': ('#let bad() = unknown\n#context bad()', ['query', '-', 'heading'], None), 'query-stdin-import': ('#import "lib.typ": bad\n#bad()', ['query', '-', 'heading'], None), 'query-one': ('= Heading', ['query', 'main.typ', 'heading', '--one'], 'main.typ'), 'query-field': ('= Heading', ['query', 'main.typ', 'heading', '--field', 'body'], 'main.typ'), 'query-one-field': ('= Heading', ['query', 'main.typ', 'heading', '--field', 'body', '--one'], 'main.typ'), 'query-field-special': ('#unknown', ['query', 'main.typ', 'heading', '--field', 'a b'], 'main.typ'), 'query-field-quotes': ('#unknown', ['query', 'main.typ', 'heading', '--field', 'a"b\nc', '--one'], 'main.typ'), 'query-selector-quotes': ('#unknown', ['query', 'main.typ', "heading.where(body: [it's Ω])"], 'main.typ'), 'query-cardinality': ('= A\n= B', ['query', 'main.typ', 'heading', '--one'], 'main.typ'), 'query-missing-field': ('= A', ['query', 'main.typ', 'heading', '--field', 'missing', '--one'], 'main.typ'), 'query-unicode-path': ('= A', ['query', 'pasta Ω/a b.typ', 'heading'], 'pasta Ω/a b.typ'), 'query-missing-file': ('', ['query', 'missing.typ', 'heading'], None), 'compile-warning-error': ('**\n#unknown', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-warning-only': ('**\nOlá', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-html-warning-error': ('**\n#unknown', ['compile', 'main.typ', 'out.html', '--format', 'html', '--features', 'html'], 'main.typ'), 'compile-html-warning-only': ('**\nOlá', ['compile', 'main.typ', 'out.html', '--format', 'html', '--features', 'html'], 'main.typ')}

cases.update({
 'query-context-success': ('#context metadata("context-value")', ['query', '-', 'metadata', '--field', 'value'], None),
 'query-context-nested-error': ('#context [#context unknown]', ['query', '-', 'heading'], None),
 'query-where-level': ('= A\n== B', ['query', 'main.typ', 'heading.where(level: 2)', '--field', 'level'], 'main.typ'),
 'query-where-body': ("= it's Ω", ['query', 'main.typ', "heading.where(body: [it's Ω])", '--field', 'body'], 'main.typ'),
 'query-document-error-invalid-selector': ('#unknown', ['query', 'main.typ', 'unknown_selector'], 'main.typ'),
 'query-valid-document-invalid-selector': ('= A', ['query', 'main.typ', 'unknown_selector'], 'main.typ'),
 'query-valid-document-text-selector': ('= A', ['query', 'main.typ', '"A"'], 'main.typ'),
 'query-dot-missing-file': ('', ['query', './missing.typ', 'heading'], None),
 'query-absolute-missing-file': ('', ['query', str(W/'missing.typ'), 'heading'], None),
 'eval-context-reference': ('#let bad() = unknown\n#context bad()', ['eval', '--in', 'main.typ', 'query(heading)'], 'main.typ'),
 'eval-where-reference': ('= A\n== B', ['eval', '--in', 'main.typ', 'query(heading.where(level: 2))'], 'main.typ'),
})

d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'diff_stat':git('diff','HEAD','--stat'),'status':git('status','--short'),'source_hashes':{p:sha(R/p) for p in files if (R/p).is_file()},'bin_sha256':{k:sha(p) for k,p in B.items()},'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','cases':{}}
for name,(source,args,file) in cases.items():
 if file:
  (W/file).parent.mkdir(parents=True,exist_ok=True);(W/file).write_text(source)
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
