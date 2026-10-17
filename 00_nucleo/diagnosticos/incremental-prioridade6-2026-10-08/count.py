from pathlib import Path
import datetime,hashlib,importlib.util,json,os,signal,subprocess,time
R=Path.cwd();D=Path(__file__).resolve().parent;W=Path('/tmp/incremental-prioridade6-2026-10-08/counter');W.mkdir(exist_ok=True)
s=importlib.util.spec_from_file_location('b',R/'00_nucleo/diagnosticos/macro-micro-2026-10-05/watch_probe.py');b=importlib.util.module_from_spec(s);s.loader.exec_module(b)
main=W/'main.typ';pdf=W/'main.pdf';lib=W/'import.typ';lib.write_text(json.loads((D/'fixtures.json').read_text())['parse_import']);pdf.unlink(missing_ok=True)
def edit(i):
 temp=W/'main.edit';temp.write_text('#set text(font: "Libertinus Serif")\n#import "import.typ": value\n#value\nRevision '+str(i)+'\n');os.replace(temp,main)
edit(0);script=W/'count.gdb';commands='set pagination off\nset confirm off\n'
for fn,marker in [('typst_core::entities::source::Source::new','P6_SOURCE'),('typst_core::compiler::eval::modules::eval_module_import','P6_IMPORT'),('typst_core::compiler::eval::eval_with_full_error_target_and_features','P6_ROOT')]:
 commands+=f'break {fn}\ncommands\nsilent\nprintf "{marker}\\n"\ncontinue\nend\n'
commands+='run\n';script.write_text(commands);logpath=W/'gdb.log';cmd=['gdb','--batch','-x',str(script),'--args',str(b.BIN['tekt']),'watch',str(main),str(pdf),'--root',str(W),'--pdf-tagged=false','--pretty=false'];rows=[];started=datetime.datetime.now(datetime.timezone.utc).isoformat()
with logpath.open('w') as log:
 p=subprocess.Popen(cmd,cwd=R,env=b.ENV,stdout=log,stderr=log,start_new_session=True)
 try:
  b.wait_pdf(p,pdf,None)
  for i in range(4):
   time.sleep(.2)
   if i:prev=b.generation(pdf);edit(i);b.wait_pdf(p,pdf,prev);time.sleep(.15)
   text=logpath.read_text();rows.append({'revision':i,**{m:text.count(m+'\n') for m in ['P6_SOURCE','P6_IMPORT','P6_ROOT']}})
 finally:os.killpg(p.pid,signal.SIGTERM);p.wait(timeout=5)
r={'started':started,'finished':datetime.datetime.now(datetime.timezone.utc).isoformat(),'baseline_state_sha256':hashlib.sha256((D/'before.json.gz').read_bytes()).hexdigest(),'release_sha256':b.sha(b.BIN['tekt']),'command':cmd,'script':commands,'source':main.read_text(),'import':lib.read_text(),'rows':rows,'log':logpath.read_text()};(D/'count.json').write_text(json.dumps(r,indent=2));print(rows)
assert [x['P6_SOURCE'] for x in rows]==[2,4,6,8];assert [x['P6_IMPORT'] for x in rows]==[1,2,3,4];assert [x['P6_ROOT'] for x in rows]==[1,2,3,4]
