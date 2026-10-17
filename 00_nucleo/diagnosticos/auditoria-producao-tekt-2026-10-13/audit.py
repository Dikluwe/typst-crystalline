from pathlib import Path
import datetime,gzip,hashlib,json,os,resource,subprocess,sys,tempfile,xml.etree.ElementTree as ET
D=Path(__file__).resolve().parent;R=D.parents[2]
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def save(name,x):
 with gzip.open(D/(name+'.json.gz'),'wt') as f:json.dump(x,f,ensure_ascii=False)
def state(name):
 paths=[p for base in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts','.github'] for p in (R/base).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml','.yml','.yaml']]
 paths += [R/f for f in ['README.md','Cargo.toml','Cargo.lock','Dockerfile','crystalline.toml','.cargo/config.toml']]
 save(name,{'utc':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'branch':subprocess.check_output(['git','branch','--show-current'],cwd=R,text=True).strip(),'status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'files':{str(p.relative_to(R)):sha(p) for p in paths},'binaries':{str(p.relative_to(R)):sha(p) for p in [R/'target/release/typst',R/'lab/typst-original/target/release/typst']}})
if sys.argv[1]=='state':state(sys.argv[2]);sys.exit()
resource.setrlimit(resource.RLIMIT_STACK,(64*1024*1024,resource.getrlimit(resource.RLIMIT_STACK)[1]))
B={'tekt':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
assert sha(B['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
base='#set page(width: 250pt, height: 220pt, margin: 25pt)\n#set text(font: "Libertinus Serif", size: 10pt)\n'
cases={
 'header-static':base+'#set page(header: [HEADER])\nAlpha\n#pagebreak()\nBravo',
 'header-context':base+'#set page(header: context [PAGE #counter(page).get().first()])\nAlpha\n#pagebreak()\nBravo',
 'footer-context':base+'#set page(footer: context [PAGE #counter(page).get().first()])\nAlpha\n#pagebreak()\nBravo',
 'table-header':base+'#table(columns: 1, table.header([HEADER]), ..range(40).map(i => [Item #i]))',
 'grid-header':base+'#grid(columns: 1, grid.header([HEADER]), ..range(40).map(i => [Item #i]))',
 'table-no-header':base+'#table(columns: 1, ..range(40).map(i => [Item #i]))',
 'footnote-overflow':base+'Alpha#footnote[#for i in range(50) [Note #i #linebreak()]]\nBravo',
 'callback-multilevel':base+'#counter(page).update((7, 2))\n#context repr(counter(page).get())\n#counter(page).update((a,b) => (a+b,b))\n#pagebreak()\n#context repr(counter(page).get())',
 'context-fragments':'#set page(width: 500pt, height: 700pt, margin: 30pt)\n#set text(font: "Libertinus Serif", size: 10pt)\nLabel Alpha.\n#for i in range(300) { context [#i:#counter(page).get().first() ] }',
 'smallcaps':base+'#smallcaps[Small Capitals] and ordinary text.',
}
env=os.environ.copy()
for k in list(env):
 if k.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_')) or k=='LD_PRELOAD':env.pop(k)
env.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1',LC_ALL='C',TZ='UTC')
x={'utc_start':now(),'state_sha256':sha(D/'before.json.gz'),'script_sha256':sha(__file__),'binaries':{e:sha(p) for e,p in B.items()},'cases':{}}
F=D/'fixtures';F.mkdir(exist_ok=True)
with tempfile.TemporaryDirectory() as tmp:
 for name,body in cases.items():
  fixture=F/(name+'.typ');fixture.write_text(body);rows={}
  for e,b in B.items():
   out=Path(tmp)/(name+'-'+e+'.pdf');cmd=[str(b),'compile',str(fixture),str(out),'--pdf-tagged=false','--pretty=false']
   p=subprocess.run(cmd,cwd=R,env=env,capture_output=True,text=True,timeout=45)
   row={'utc':now(),'command':cmd,'code':p.returncode,'stdout':p.stdout,'stderr':p.stderr}
   if p.returncode==0:
    row['text']=subprocess.check_output(['pdftotext','-layout',str(out),'-'],text=True)
    tree=ET.fromstring(subprocess.check_output(['pdftotext','-bbox',str(out),'-']));ns={'x':'http://www.w3.org/1999/xhtml'}
    row['pages']=[{'size':p.attrib,'words':[{'text':w.text,**w.attrib} for w in p.findall('.//x:word',ns)]} for p in tree.findall('.//x:page',ns)];row['pdf_sha256']=sha(out)
   rows[e]=row
  x['cases'][name]={'source':body,'source_sha256':sha(fixture),'engines':rows};print(name,{e:{'code':r['code'],'pages':len(r.get('pages',[])),'header_count':r.get('text','').count('HEADER'),'page_text_count':r.get('text','').count('PAGE'),'stderr':r['stderr'][:110]} for e,r in rows.items()},flush=True)
x['utc_end']=now();save('probes',x)
