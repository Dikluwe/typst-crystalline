"""Check whether explicit fill is an output-equivalent control for local set."""
import resource
resource.setrlimit(resource.RLIMIT_STACK, (64 * 1024 * 1024, resource.getrlimit(resource.RLIMIT_STACK)[1]))
import datetime,gzip,hashlib,json,os,subprocess,tempfile,xml.etree.ElementTree as ET
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2];F=D/'fixtures';FONT=R/'00_nucleo/diagnosticos/performance-baseline/fonts'
B={'before':Path('/tmp/tekt-watch-context/typst'),'vanilla':R/'lab/typst-original/target/release/typst'}
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

import itertools
x={'utc_start':utc(),'before_sha256':sha(D/'before.json.gz'),'binaries':{e:sha(b)for e,b in B.items()},'script_sha256':sha(__file__),'cases':{}}
with tempfile.TemporaryDirectory(prefix='watch-qualify-')as temp:
 out=Path(temp)
 for name,word,expanded in itertools.product(['plain','import','context300'],['Alpha','Bravo'],[False,True]):
  stem=f'{name}-{word}-{expanded}';src=(F/(name+'.typ')).read_text().replace('Alpha',word)
  if expanded:src=src.replace('range(300)','range(320)').replace('range(8)','range(12)')
  fixture=out/(stem+'.typ');fixture.write_text(src);(out/'lib.typ').write_bytes((F/'lib.typ').read_bytes());FROOT=F;F=out
  rows={e:run(e,b,fixture,out)for e,b in B.items()};F=FROOT
  comparison=compare(rows['before'],rows['vanilla']);x['cases'][stem]={'source':src,'source_sha256':sha(fixture),'engines':rows,'comparison':comparison};print(stem,comparison,flush=True)
assert x['binaries']=={e:sha(b)for e,b in B.items()};x['utc_end']=utc()
with gzip.open(D/'qualification.json.gz','wt')as f:json.dump(x,f)
