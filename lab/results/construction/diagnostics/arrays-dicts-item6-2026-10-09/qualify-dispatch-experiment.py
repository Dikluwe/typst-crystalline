"""Check text, word boxes, page size and low-resolution raster for item 6 fixtures."""
import datetime, hashlib, json, os, subprocess, tempfile, xml.etree.ElementTree as ET
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2];F=D/'fixtures'
B={'tekt':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
FONT=R/'00_nucleo/diagnosticos/performance-baseline/fonts'
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
env=os.environ.copy()
for k in list(env):
 if k.startswith(('TYPST_','CRYSTALLINE_','CARGO_PROFILE_')):env.pop(k)
env.update(RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0',NO_COLOR='1',TZ='UTC',LC_ALL='C')
status=subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R)
stat=subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R)
(D/'dispatch-experiment-qualification-git-status-short.txt').write_bytes(status)
(D/'dispatch-experiment-qualification-git-diff-head-stat.txt').write_bytes(stat)
state={'utc_start':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),
       'working_tree':'uncommitted','git_status_sha256':hashlib.sha256(status).hexdigest(),
       'git_diff_stat_sha256':hashlib.sha256(stat).hexdigest(),
       'binaries':{e:sha(p) for e,p in B.items()},
       'fixtures':{p.name:sha(p) for p in F.glob('*.typ')},'cases':{}}
with tempfile.TemporaryDirectory(prefix='item6-quality-') as tmp:
 for fixture in sorted(F.glob('*.typ')):
  pair={}
  for engine,bin in B.items():
   pdf=Path(tmp)/(fixture.stem+'-'+engine+'.pdf')
   cmd=[str(bin),'compile',str(fixture),str(pdf),'--root',str(F),'--pdf-tagged=false','--pretty=false','--font-path',str(FONT)]
   if engine=='vanilla':cmd+=['--jobs','1']
   p=subprocess.run(cmd,cwd=R,env=env,capture_output=True,text=True);assert p.returncode==0,(cmd,p.stderr)
   xml=ET.fromstring(subprocess.check_output(['pdftotext','-bbox',str(pdf),'-']))
   ns={'x':'http://www.w3.org/1999/xhtml'}
   words=[{'text':w.text,**{k:float(v) for k,v in w.attrib.items()}} for w in xml.findall('.//x:word',ns)]
   pages=[{k:float(v) for k,v in x.attrib.items() if k in ('width','height')} for x in xml.findall('.//x:page',ns)]
   prefix=Path(tmp)/(fixture.stem+'-'+engine)
   subprocess.run(['pdftoppm','-f','1','-l','1','-r','36','-png','-singlefile',str(pdf),str(prefix)],check=True,capture_output=True)
   pair[engine]={'command':cmd,'pdf_sha256':sha(pdf),'raster_sha256':sha(str(prefix)+'.png'),
                 'words':words,'pages':pages,'stderr':p.stderr}
  a,b=pair['tekt'],pair['vanilla']
  same_words=[w['text'] for w in a['words']]==[w['text'] for w in b['words']]
  bbox=max((abs(x[k]-y[k]) for x,y in zip(a['words'],b['words']) for k in ('xMin','xMax','yMin','yMax')),default=0) if same_words else None
  state['cases'][fixture.stem]={'tekt':a,'vanilla':b,'same_words':same_words,
                               'same_pages':a['pages']==b['pages'],'bbox_max_delta_pt':bbox,
                               'same_raster':a['raster_sha256']==b['raster_sha256']}
state['utc_end']=now()
assert {e:sha(p) for e,p in B.items()}==state['binaries']
(D/'dispatch-experiment-qualification.json').write_text(json.dumps(state,ensure_ascii=False,indent=2)+'\n')
print({k:(v['same_words'],v['same_pages'],v['bbox_max_delta_pt'],v['same_raster']) for k,v in state['cases'].items()})
