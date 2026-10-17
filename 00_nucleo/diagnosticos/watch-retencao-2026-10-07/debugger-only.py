"""Behavior and debugger probes, deliberately outside the performance run."""
import importlib.util,json,os,signal,subprocess,time,zlib,struct
from pathlib import Path
s=importlib.util.spec_from_file_location('run',Path(__file__).with_name('run.py'));r=importlib.util.module_from_spec(s);s.loader.exec_module(r);b=r.b
W=r.WORK/'debugger';W.mkdir(exist_ok=True);src=W/'main.typ';pdf=W/'main.pdf';logpath=W/'gdb.log'
src.write_text(b.source('hello',0));pdf.unlink(missing_ok=True)
gdb=W/'commands.gdb';gdb.write_text('set pagination off\nset confirm off\nset debuginfod enabled off\nset breakpoint pending on\nbreak typst_infra::fontdb::load_system_fonts\ncommands\nsilent\nprintf "P4_CATALOG_HIT\\n"\ncontinue\nend\nrun\n')
cmd=['gdb','--batch','-x',str(gdb),'--args',str(b.BIN['tekt']),'watch',str(src),str(pdf),'--root',str(W),'--pdf-tagged=false','--pretty=false']
result={'command':cmd,'debugger_commands':gdb.read_text(),'cycles':[]}
with logpath.open('w') as log:
    p=subprocess.Popen(cmd,cwd=r.ROOT,env=b.ENV,stdout=log,stderr=log,start_new_session=True)
    try:
        b.wait_pdf(p,pdf,None);time.sleep(.2);result['cycles'].append({'revision':0,'cumulative_catalog_hits':logpath.read_text().count('P4_CATALOG_HIT\n')})
        for rev in range(1,4):
            prev=b.generation(pdf);src.write_text(b.source('hello',rev));b.wait_pdf(p,pdf,prev);time.sleep(.2)
            result['cycles'].append({'revision':rev,'cumulative_catalog_hits':logpath.read_text().count('P4_CATALOG_HIT\n')})
    except Exception as e:result['limitation']=repr(e)
    finally:
        try:os.killpg(p.pid,signal.SIGTERM)
        except ProcessLookupError:pass
        p.wait(timeout=5)
result['log']=logpath.read_text();r.save('debugger-escalated.json',result);print(json.dumps(result['cycles']),flush=True)

