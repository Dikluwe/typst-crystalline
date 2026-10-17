import datetime,gzip,hashlib,json,subprocess,os,re
from pathlib import Path
H=Path(__file__).resolve().parent;R=H.parents[2];T=Path('/tmp/tekt-context-reconciliation')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1',NO_COLOR='1')
binaries={'baseline':T/'baseline-typst','candidate':R/'target/release/typst','vanilla':R/'lab/typst-original/target/release/typst'}
record={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'binaries':{n:sha(p) for n,p in binaries.items()},'cases':{}}
fixtures=json.loads((H/'fixtures.json').read_text())
fixtures['selected_math_callback']='#let c = counter(heading.where(level: 1))\n#context [#c.final() $cancel(x, angle: #(a => a))$]'
fixtures['selected_rtl']='#set text(font: "DejaVu Sans", dir: rtl, lang: "ar")\n#let c = counter(heading.where(level: 1))\n#context { let _ = c.final(); [مرحبا بالعالم مرحبا بالعالم] }'
for name,source in fixtures.items():
 fixture=T/(name+'.typ');fixture.write_text(source);entry={'source':source,'engines':{}}
 for engine,binary in binaries.items():
  pdf=T/(name+'-'+engine+'.pdf');cmd=[str(binary),'compile',str(fixture),str(pdf),'--pdf-tagged=false','--pretty=false'];run=subprocess.run(cmd,capture_output=True,text=True,env=env,timeout=60);assert run.returncode==0,(name,engine,run.stderr)
  text=subprocess.check_output(['pdftotext',str(pdf),'-'],text=True);bbox=subprocess.check_output(['pdftotext','-bbox',str(pdf),'-'],text=True)
  stem=T/(name+'-'+engine+'-render');subprocess.run(['pdftoppm','-r','36','-png',str(pdf),str(stem)],capture_output=True,check=True)
  entry['engines'][engine]={'command':cmd,'stderr':run.stderr,'text':text,'bbox':bbox,'rasters':[sha(p) for p in sorted(T.glob(stem.name+'-*.png'))],'pdf_sha256':sha(pdf)}
 a,b=entry['engines']['baseline'],entry['engines']['candidate'];assert a['text']==b['text'],name;assert a['bbox']==b['bbox'],name;assert a['rasters']==b['rasters'],name;assert a['stderr']==b['stderr'],name
 entry['vanilla_text_match']=re.sub(r'\s+','',b['text'])==re.sub(r'\s+','',entry['engines']['vanilla']['text']);assert entry['vanilla_text_match'],name
 record['cases'][name]=entry;print(name,'text/bbox/raster/diagnostics preserved; vanilla text matched',flush=True)
errors={'independent_error':'#let c = counter(heading.where(level: 1))\n#context c.final()\n#context panic("independent")','numbering_error':'#set page(numbering: (current, total) => panic("numbering"))\n#let c = counter(heading.where())\n#context c.final()\nText'}
for name,source in errors.items():
 fixture=T/(name+'.typ');fixture.write_text(source);entries={}
 for engine,binary in binaries.items():
  cmd=[str(binary),'compile',str(fixture),str(T/(name+'-'+engine+'.pdf'))];r=subprocess.run(cmd,capture_output=True,text=True,env=env);entries[engine]={'command':cmd,'returncode':r.returncode,'stderr':r.stderr}
 assert entries['baseline']['returncode']==entries['candidate']['returncode']!=0,name
 assert entries['baseline']['stderr']==entries['candidate']['stderr'],name
 record['cases'][name]={'source':source,'engines':entries};print(name,'baseline/candidate error preserved',flush=True)
record['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
gzip.open(H/'verification.json.gz','wt').write(json.dumps(record,ensure_ascii=False))
