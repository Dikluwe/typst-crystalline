"""Count syntax construction and evaluation entries in the current Tekt watcher."""
import datetime, hashlib, json, os, signal, subprocess, tempfile, time
from pathlib import Path
from measure import ROOT, HERE, BINS, ENV, main_source, generation, wait_pdf

def sha(path): return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def utc(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
with tempfile.TemporaryDirectory(prefix='item5-count-') as tmp:
    folder=Path(tmp);main=folder/'main.typ';lib=folder/'lib.typ';pdf=folder/'main.pdf';log=folder/'gdb.log';script=folder/'count.gdb'
    lib.write_text(''.join(f'#let v{i} = {i}\n' for i in range(2000))+'#let value = v1999\n')
    word='Alpha';rows=8;main.write_text(main_source(word,rows))
    commands='set pagination off\nset confirm off\n'
    for fn,marker in [('typst_core::entities::source::Source::new','SOURCE_NEW'),('typst_core::compiler::eval::modules::eval_module_import','IMPORT_EVAL'),('typst_core::compiler::eval::eval_with_full_error_target_and_features','ROOT_EVAL')]:
        commands+=f'break {fn}\ncommands\nsilent\nprintf "{marker}\\n"\ncontinue\nend\n'
    commands+='run\n';script.write_text(commands)
    cmd=['gdb','--batch','-x',str(script),'--args',str(BINS['tekt']),'watch',str(main),str(pdf),'--root',str(folder),'--pdf-tagged=false','--pretty=false']
    receipt={'utc_start':utc(),'binary_sha256':sha(BINS['tekt']),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'command':cmd,'gdb_script':commands,'rows':[]}
    with log.open('w') as output:
        proc=subprocess.Popen(cmd,cwd=ROOT,env=ENV,stdout=output,stderr=output,start_new_session=True)
        try:
            if not wait_pdf(proc,pdf,None,20):raise RuntimeError('initial PDF absent')
            stages=['initial','metadata_touch','same_bytes','small','structural']
            for stage in stages:
                time.sleep(.25)
                if stage!='initial':
                    old=generation(pdf)
                    if stage=='metadata_touch':
                        s=main.stat();os.utime(main,ns=(s.st_atime_ns,s.st_mtime_ns+1_000_000_000))
                    else:
                        if stage=='small':word='Bravo'
                        elif stage=='structural':rows=12
                        t=folder/'main.new';t.write_text(main_source(word,rows));os.replace(t,main)
                    if not wait_pdf(proc,pdf,old,20):raise RuntimeError(f'no PDF at {stage}')
                    time.sleep(.2)
                output.flush();content=log.read_text()
                receipt['rows'].append({'stage':stage,'source_sha256':sha(main),'pdf_sha256':sha(pdf),**{m:content.count(m+'\n') for m in ['SOURCE_NEW','IMPORT_EVAL','ROOT_EVAL']}})
                print(receipt['rows'][-1],flush=True)
        finally:
            os.killpg(proc.pid,signal.SIGTERM)
            proc.wait(timeout=5)
    receipt['utc_end']=utc();receipt['log']=log.read_text()
(HERE/'counts.json').write_text(json.dumps(receipt,indent=2)+'\n')
