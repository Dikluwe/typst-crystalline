"""Count production acquisitions, separate from timings; only our own process group."""
import importlib.util,json,os,signal,subprocess,time
from pathlib import Path
R=Path('/repos/Antigravity/typst-crystalline');D=Path(__file__).resolve().parent
s=importlib.util.spec_from_file_location('r',R/'00_nucleo/diagnosticos/watch-retencao-2026-10-07/run.py');r=importlib.util.module_from_spec(s);s.loader.exec_module(r)
W=Path('/tmp/watch-catalogo-2026-10-07/debugger');W.mkdir(parents=True,exist_ok=True);source=W/'main.typ';pdf=W/'main.pdf';logpath=W/'gdb.log';source.write_text(r.b.source('hello',0));pdf.unlink(missing_ok=True)
commands='set pagination off\nset confirm off\nset debuginfod enabled off\n'
for function,marker in [('typst_infra::fontdb::generation::prepare','P4_VALIDATE'),('typst_infra::fontdb::catalog_from_database','P4_ACQUIRE'),('typst_infra::world::fonts::fresh_catalog','P4_FRESH')]:
 commands+=f'break {function}\ncommands\nsilent\nprintf "{marker}\\n"\ncontinue\nend\n'
commands+='run\n';script=W/'commands.gdb';script.write_text(commands)
cmd=['gdb','--batch','-x',str(script),'--args',str(R/'target/release/typst'),'watch',str(source),str(pdf),'--root',str(W),'--pdf-tagged=false','--pretty=false'];rows=[]
with logpath.open('w') as log:
 p=subprocess.Popen(cmd,cwd=R,env=r.b.ENV,stdout=log,stderr=log,start_new_session=True)
 try:
  r.b.wait_pdf(p,pdf,None);time.sleep(.2)
  for revision in range(4):
   if revision:
    previous=r.b.generation(pdf);source.write_text(r.b.source('hello',revision));r.b.wait_pdf(p,pdf,previous);time.sleep(.2)
   text=logpath.read_text();rows.append({'revision':revision,**{m:text.count(m+'\n') for m in ('P4_VALIDATE','P4_ACQUIRE','P4_FRESH')}})
 finally:
  os.killpg(p.pid,signal.SIGTERM);p.wait(timeout=5)
assert [x['P4_VALIDATE'] for x in rows]==[1,2,3,4];assert [x['P4_ACQUIRE'] for x in rows]==[1,1,1,1];assert all(x['P4_FRESH']==0 for x in rows)
(D/'debugger.json').write_text(json.dumps({'command':cmd,'script':commands,'cycles':rows,'log':logpath.read_text(),'release_sha256':r.b.sha(R/'target/release/typst')},indent=2)+'\n');print(json.dumps(rows))
