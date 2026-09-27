"""Untimed equivalent-input geometry probes; no production writes."""
from pathlib import Path
import hashlib,json,subprocess,datetime,shutil,os
ROOT=Path(__file__).resolve().parents[3];R=Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def fingerprint():
 files={str(p.relative_to(ROOT)):sha(p) for base in ['01_core/src','02_shell','03_infra/src','04_wiring/src','00_nucleo/prompts','benches'] for p in (ROOT/base).rglob('*') if p.is_file() and 'target' not in p.parts};return files
before=fingerprint();head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
vanilla=ROOT/'lab/typst-original/target/release/typst';tekt=ROOT/'target/debug/typst';assert sha(vanilla)=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea'
shutil.copyfile(ROOT/'00_nucleo/diagnosticos/infra-performance-followup/fixtures/tiny.jpg',R/'inputs/tiny.jpg')
records=[]
for name,body in {'image':'#image("tiny.jpg", width: 80pt, height: 40pt)', 'shape':'#rect(width: 80pt, height: 40pt, fill: blue)', 'placed_shape':'#place(top + left, dx: 3pt, dy: 5pt, rect(width: 80pt, height: 40pt, fill: blue))','image_then_text':'#image("tiny.jpg", width: 80pt, height: 40pt)\n\nText'}.items():
 for edge in ['default','0pt','ascender']:
  source=R/'inputs'/f'{name}-{edge}.typ';source.write_text('#set page(width: 160pt, height: 180pt, margin: 8pt)\n'+('' if edge=='default' else f'#set text(top-edge: {edge})\n')+body+'\n')
  for engine,binary in [('tekt',tekt),('vanilla',vanilla)]:
   out=R/f'{name}-{edge}-{engine}.svg';command=[str(binary),'compile',str(source),str(out),'--root',str(R/'inputs'),'--font-path',str(ROOT/'00_nucleo/diagnosticos/infra-performance-followup/fonts')]
   if engine=='vanilla':command+=['--jobs','1']
   env=os.environ.copy();env['RAYON_NUM_THREADS']='1'
   for k in list(env):
    if k.startswith(('TYPST_','CRYSTALLINE_')):env.pop(k)
   result=subprocess.run(command,cwd=ROOT,env=env,capture_output=True,text=True);(R/(out.stem+'.stderr')).write_text(result.stderr);records.append({'fixture':name,'edge':edge,'engine':engine,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':command,'returncode':result.returncode,'source_sha256':sha(source),'output':out.name,'output_sha256':sha(out) if out.exists() else None})
after=fingerprint();assert before==after
(R/'results.json').write_text(json.dumps(records,indent=2)+'\n')
(R/'provenance.json').write_text(json.dumps({'head':head,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exact_status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],cwd=ROOT,text=True),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=ROOT,text=True),'tekt_debug_not_a_performance_run':True,'tekt_sha256':sha(tekt),'vanilla_sha256':sha(vanilla),'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','fingerprint_before':before,'fingerprint_after':after},indent=2)+'\n')
print('geometry probes',len(records),'failed',sum(x['returncode']!=0 for x in records))
