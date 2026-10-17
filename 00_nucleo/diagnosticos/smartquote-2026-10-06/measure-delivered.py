from pathlib import Path
import subprocess,hashlib,gzip,json,datetime,os,sys,shutil
R=Path('/repos/Antigravity/typst-crystalline');D=R/'00_nucleo/diagnosticos/smartquote-2026-10-06';W=Path('/tmp/tekt-smartquote-probes');W.mkdir(exist_ok=True)
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
cases={'eval-markup-single': ('', ['eval', "[it's Ω]"], None), 'eval-markup-double': ('', ['eval', '["hi"]'], None), 'eval-native-single': ('', ['eval', 'smartquote(double: false)'], None), 'eval-native-disabled': ('', ['eval', 'smartquote(double: false, enabled: false)'], None), 'eval-markup-disabled': ('', ['eval', '{ set smartquote(enabled: false); ["hi"] }'], None), 'eval-native-custom': ('', ['eval', 'smartquote(double: false, alternative: true, quotes: (single: "[]"))'], None), 'compile-default': ('"hello" it\'s Ω', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-german': ('#set text(lang: "de")\n"Hallo"', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-french': ('#set text(lang: "fr")\n"Bonjour"', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-reused-german': ('#let q = ["Hallo"]\n#set text(lang: "de")\n#q', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-reused-french': ('#let q = ["Bonjour"]\n#text(lang: "fr", q)', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-reused-disabled': ('#let q = ["Hello"]\n#set smartquote(enabled: false)\n#q', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-disabled': ('#set smartquote(enabled: false)\n"Hello"', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-explicit-disabled': ('#smartquote(enabled: false)X#smartquote(enabled: false)', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-custom': ('#set smartquote(quotes: "[]")\n"hello"', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-alternative': ('#set text(lang: "de")\n#set smartquote(alternative: true)\n"Hallo"', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-primes': ('6\' 2"', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-nested': ('"outer \'inner\' tail"', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-paragraphs': ('"first"\n\n"second"', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-linebreak': ('"first\\\nsecond"', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-mixed': ('#smartquote()A" B#smartquote(double: false)C\'', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-show-smartquote': ('#show smartquote: it => [Q]\n"hi"', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-show-text': ('#show text: it => [X]\n"hi"', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-inserted-previous': ("#let x = [it]\n#x's", ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-inline-object': ("#box[X]'", ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'compile-narrow': ('#set page(width: 80pt, height: 150pt, margin: 5pt)\n"several words near a narrow line break"', ['compile', 'main.typ', 'out.pdf'], 'main.typ'), 'query-where-body': ("= it's Ω", ['query', 'main.typ', "heading.where(body: [it's Ω])", '--field', 'body'], 'main.typ')}

cases.update({
 'eval-native-default': ('', ['eval', 'smartquote()'], None),
 'eval-fields-default': ('', ['eval', 'smartquote().fields()'], None),
 'eval-fields-single': ('', ['eval', 'smartquote(double: false).fields()'], None),
 'eval-field-default': ('', ['eval', 'smartquote().double'], None),
 'eval-field-single': ('', ['eval', 'smartquote(double: false).double'], None),
 'eval-fields-disabled': ('', ['eval', 'smartquote(enabled: false).fields()'], None),
 'compile-show-native': ('#show smartquote: it => [Q]\n#smartquote()', ['compile', 'main.typ', 'out.pdf'], 'main.typ'),
})

for name, source in {
 'html-default': '"hello"',
 'html-reused-german': '#let q = ["Hallo"]\n#set text(lang: "de")\n#q',
 'html-native': '#smartquote()X#smartquote()',
 'html-title': '#html.html[#html.head[#html.title["Hi"]]#html.body[]]',
 'html-script': "#html.html[#html.head[]#html.body[#html.script['x']]]",
}.items():
 cases[name]=(source,['compile','main.typ','out.html','--format','html','--features','html'],'main.typ')

cases['compile-set-double-native'] = ('#set smartquote(double: false)\n#smartquote()x#smartquote()', ['compile','main.typ','out.pdf'], 'main.typ')
cases['compile-show-filtered-native'] = ('#show smartquote.where(double: true): it => [Q]\n#smartquote()', ['compile','main.typ','out.pdf'], 'main.typ')

cases['eval-quotes-keys'] = ('', ['eval', 'smartquote(quotes: (single: "[]")).fields().quotes.keys()'], None)
cases['eval-enabled-omitted'] = ('', ['eval', 'smartquote().enabled'], None)
cases['eval-at-default'] = ('', ['eval', 'smartquote().at("double", default: false)'], None)

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
