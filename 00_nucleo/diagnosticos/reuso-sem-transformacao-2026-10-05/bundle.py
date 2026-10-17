"""Mixed target bundle control with pinned oracle and provenance."""
import datetime,gzip,hashlib,json,os,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;ROOT=HERE.parents[2]
OUT=Path('/tmp/tekt-noop-reuse-bundle-2026-10-05');OUT.mkdir(exist_ok=True)
source='#document("one.pdf")[#show heading: it => upper(it.body)\n= Original]\n#document("two.html")[#show strong: it => emph(it.body)\nHello *world*]'
fixture=OUT/'main.typ';fixture.write_text(source)
engines={'baseline':Path('/tmp/tekt-noop-reuse-2026-10-05/typst-baseline'),'candidate':ROOT/'target/release/typst','vanilla':ROOT/'lab/typst-original/target/release/typst'}
r={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True),'source':source,'cases':{}}
for name,exe in engines.items():
 output=OUT/name
 cmd=[str(exe),'compile',str(fixture),str(output),'--format','bundle','--features','bundle,html','--pdf-tagged=false','--pretty=false']
 p=subprocess.run(cmd,cwd=ROOT,env=dict(os.environ,SOURCE_DATE_EPOCH='0',NO_COLOR='1'),capture_output=True,text=True,timeout=60)
 case={'command':cmd,'sha256':hashlib.sha256(exe.read_bytes()).hexdigest(),'returncode':p.returncode,'stdout':p.stdout,'stderr':p.stderr,'files':{}}
 if p.returncode==0:
  for f in output.rglob('*'):
   if f.is_file():case['files'][str(f.relative_to(output))]=subprocess.check_output(['pdftotext',str(f),'-'],text=True) if f.suffix=='.pdf' else f.read_text()
 r['cases'][name]=case
 print(name,p.returncode,case['files'],p.stderr if p.returncode else '')
r['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(HERE/'bundle.json.gz','wt') as f:json.dump(r,f,ensure_ascii=False,indent=2)
