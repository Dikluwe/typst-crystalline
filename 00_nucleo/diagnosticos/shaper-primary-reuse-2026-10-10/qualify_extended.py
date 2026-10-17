"""Check whether explicit fill is an output-equivalent control for local set."""
import resource
resource.setrlimit(resource.RLIMIT_STACK, (64 * 1024 * 1024, resource.getrlimit(resource.RLIMIT_STACK)[1]))
import datetime,gzip,hashlib,json,os,subprocess,tempfile,xml.etree.ElementTree as ET
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2];F=R/'00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/change-aware-fixtures';FONT=R/'00_nucleo/diagnosticos/performance-baseline/fonts'
B={'before':Path('/tmp/tekt-shaper-primary-benchmark/old/typst'),'after':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
ENV=os.environ.copy()
for k in list(ENV):
 if k.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_')):ENV.pop(k)
ENV.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1',TZ='UTC',LC_ALL='C')
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def run(e,b,fixture,out):
 pdf=out/(fixture.stem+'-'+e+'.pdf');cmd=[str(b),'compile',str(fixture),str(pdf),'--root',str(F),'--pdf-tagged=false','--pretty=false','--font-path',str(FONT)]+(['--jobs','1'] if e=='vanilla' else [])
 p=subprocess.run(cmd,cwd=R,env=ENV,capture_output=True,text=True,timeout=120);assert p.returncode==0,(cmd,p.stderr)
 layout=subprocess.check_output(['pdftotext','-layout',str(pdf),'-'],text=True)
 root=ET.fromstring(subprocess.check_output(['pdftotext','-bbox',str(pdf),'-']))
 ns={'x':'http://www.w3.org/1999/xhtml'};pages=[]
 for page in root.findall('.//x:page',ns):
  pages.append({'size':{k:float(v) for k,v in page.attrib.items()},'words':[{'text':w.text,'box':{k:float(v) for k,v in w.attrib.items()}} for w in page.findall('.//x:word',ns)]})
 prefix=out/(fixture.stem+'-'+e)
 subprocess.run(['pdftoppm','-r','36','-png',str(pdf),str(prefix)],check=True,capture_output=True)
 rasters=[sha(x) for x in sorted(out.glob(prefix.name+'-*.png'))]
 return {'command':cmd,'stderr':p.stderr,'pdf_sha256':sha(pdf),'layout_text_sha256':hashlib.sha256(layout.encode()).hexdigest(),'pages':pages,'raster_sha256':rasters}
def compare(a,b):
 wa=[w for p in a['pages'] for w in p['words']];wb=[w for p in b['pages'] for w in p['words']]
 samewords=[w['text'] for w in wa]==[w['text'] for w in wb]
 bbox=max((abs(x['box'][k]-y['box'][k]) for x,y in zip(wa,wb) for k in ('xMin','xMax','yMin','yMax')),default=0) if samewords else None
 return {'pages_a':len(a['pages']),'pages_b':len(b['pages']),'word_count_a':len(wa),'word_count_b':len(wb),'same_text':a['layout_text_sha256']==b['layout_text_sha256'],'same_words':samewords,'same_page_size':[p['size'] for p in a['pages']]==[p['size'] for p in b['pages']],'bbox_max_delta_pt':bbox,'same_raster':a['raster_sha256']==b['raster_sha256']}
status=subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R)
stat=subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R)
state={'stack_limit_bytes':resource.getrlimit(resource.RLIMIT_STACK)[0],'utc_start':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','git_status':status.decode(),'git_diff_stat':stat.decode(),'binaries':{e:sha(b) for e,b in B.items()},'fixtures':{f.name:{'text':f.read_text(),'sha256':sha(f)} for f in F.glob('*.typ')},'script_sha256':sha(D/'qualify_extended.py'),'cases':{},'within':{},'across':{}}
with tempfile.TemporaryDirectory(prefix='table-local-control-') as d:
 out=Path(d)
 for fixture in sorted(p for p in F.glob('*.typ') if not p.name.startswith('_')):
  state['cases'][fixture.stem]={e:run(e,b,fixture,out) for e,b in B.items()}
  state['across'][fixture.stem]=compare(state['cases'][fixture.stem]['before'],state['cases'][fixture.stem]['after'])
  print('across',fixture.stem,state['across'][fixture.stem],flush=True)
assert state['binaries']=={e:sha(b) for e,b in B.items()}
assert all(sha(F/name)==v['sha256'] for name,v in state['fixtures'].items())
state['utc_end']=utc();state['passed']=all(v['same_text'] and v['same_words'] and v['same_page_size'] and v['bbox_max_delta_pt']==0 and v['same_raster'] for v in state['across'].values())
with gzip.open(D/'qualification-extended.json.gz','wt') as p:json.dump(state,p,ensure_ascii=False,indent=2)

assert state["passed"], state["across"]
