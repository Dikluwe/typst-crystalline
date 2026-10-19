"""Distinguish block wrappers from table-cell and generic set-rule costs."""
import datetime,gzip,hashlib,json,os,statistics,subprocess,tempfile,xml.etree.ElementTree as ET
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2];F=D/'fixtures';FONT=R/'00_nucleo/diagnosticos/performance-baseline/fonts';SO=D/'alloc_probe.so';CPU=min(os.sched_getaffinity(0))
B={'tekt':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
ENV=os.environ.copy()
for k in list(ENV):
 if k.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_')):ENV.pop(k)
ENV.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1',TZ='UTC',LC_ALL='C')
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
variants={'block-no-set':'{ table.cell[Local] }','text-set':'{ set text(fill: black); table.cell[Local] }','rect-set':'{ set rect(fill: red); table.cell[Local] }','cell-set-none':'{ set table.cell(fill: none); table.cell[Local] }'}
head='#set page(width: 400pt, height: 600pt, margin: 24pt)\n#set text(font: "Libertinus Serif", size: 10pt)\n'
paths=[]
for n in (100,1600):
 for name,expr in variants.items():
  p=F/f'{name}-{n}.typ';p.write_text(head+f'#let c = {expr}\n#table(columns: 4, ..range({n}).map(_ => c))\n');paths.append(p)
status=subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R)
stat=subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R)
state={'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','git_status':status.decode(),'git_diff_stat':stat.decode(),'binaries':{e:sha(b) for e,b in B.items()},'interposer_sha256':sha(SO),'fixtures':{p.name:{'text':p.read_text(),'sha256':sha(p)} for p in [*paths,*[F/f'local-set-{n}.typ' for n in (100,1600)]]},'script_sha256':sha(D/'extra_probe.py'),'samples':[],'output':{}}
with tempfile.TemporaryDirectory(prefix='table-local-extra-') as d:
 out=Path(d)
 for f in [*paths,*[F/f'local-set-{n}.typ' for n in (100,1600)]]:
  state['output'][f.stem]={}
  for e,b in B.items():
   for rep in range(3):
    pdf=out/(f.stem+e+str(rep)+'.pdf');prof=out/(f.stem+e+str(rep)+'.json');timing=out/(f.stem+e+str(rep)+'-timing.json')
    env=ENV.copy();env.update(LD_PRELOAD=str(SO),ALLOC_PROFILE_OUT=str(prof))
    cmd=[str(b),'compile',str(f),str(pdf),'--root',str(F),'--pdf-tagged=false','--pretty=false','--font-path',str(FONT)]+(['--jobs','1'] if e=='vanilla' else [])
    if e=='tekt':cmd+=['--timings-json',str(timing)]
    p=subprocess.run(cmd,cwd=R,env=env,capture_output=True,text=True,timeout=120,preexec_fn=lambda:os.sched_setaffinity(0,{CPU}));assert p.returncode==0,(f,e,p.stderr)
    c=json.loads(prof.read_text());c['total_calls']=sum(c[k] for k in ('malloc_calls','calloc_calls','realloc_calls','memalign_calls'))
    state['samples'].append({'fixture':f.stem,'engine':e,'rep':rep,'utc':utc(),'command':cmd,'profile':c,'timings':json.loads(timing.read_text()) if e=='tekt' else None,'pdf_sha256':sha(pdf)})
    if rep==0:
     layout=subprocess.check_output(['pdftotext','-layout',str(pdf),'-'],text=True)
     root=ET.fromstring(subprocess.check_output(['pdftotext','-bbox',str(pdf),'-']))
     ns={'x':'http://www.w3.org/1999/xhtml'}
     words=[{'text':w.text,'box':w.attrib} for w in root.findall('.//x:word',ns)]
     prefix=out/(f.stem+e)
     subprocess.run(['pdftoppm','-r','36','-png',str(pdf),str(prefix)],check=True,capture_output=True)
     raster=[sha(x) for x in sorted(out.glob(prefix.name+'-*.png'))]
     state['output'][f.stem][e]={'layout_text_sha256':hashlib.sha256(layout.encode()).hexdigest(),'words':words,'page_count':len(root.findall('.//x:page',ns)),'raster_sha256':raster}
  print(f.stem,'done',flush=True)
state['summary']={}
for f in state['output']:
 state['summary'][f]={}
 for e in B:
  g=[x for x in state['samples'] if x['fixture']==f and x['engine']==e]
  state['summary'][f][e]={'total_calls':statistics.median(x['profile']['total_calls'] for x in g),'requested_bytes':statistics.median(x['profile']['requested_bytes'] for x in g),'timings':{k:statistics.median(x['timings'][k] for x in g) for k in g[0]['timings']} if e=='tekt' else None}
 for e in B:
  original=state['output'][f'local-set-{f.split("-")[-1]}'][e];current=state['output'][f][e]
  state['summary'][f][e]['vs_local']={'same_text':original['layout_text_sha256']==current['layout_text_sha256'],'same_words':[w['text'] for w in original['words']]==[w['text'] for w in current['words']],'same_bbox':[w['box'] for w in original['words']]==[w['box'] for w in current['words']],'same_raster':original['raster_sha256']==current['raster_sha256']}
 print(f,{e:{k:v for k,v in x.items() if k in ('total_calls','requested_bytes','vs_local')} for e,x in state['summary'][f].items()},flush=True)
assert state['binaries']=={e:sha(b) for e,b in B.items()}
assert state['interposer_sha256']==sha(SO)
state['utc_end']=utc()
with gzip.open(D/'extra-probe.json.gz','wt') as p:json.dump(state,p,ensure_ascii=False,indent=2)
