"""Fresh-process paired CLI measurements, with optional separate instrumentation."""
import json, os, pathlib, resource, subprocess, threading, time
import run as c

def one(engine, fixture, directory, label, instrumented):
    artifacts=c.identity_check()
    exe=artifacts['typst']['path'] if engine=='tekt' else str(c.ROOT/'lab/typst-original/target/release/typst')
    pdf=directory/(label+'.pdf'); trace=directory/(label+'.json')
    cmd=[exe,'compile',str(fixture),str(pdf),'--root',str(fixture.parent),'--pdf-tagged=false','--pretty=false','--font-path',str(c.OUT/'fonts')]
    if engine=='vanilla': cmd+=['--jobs','1']
    if instrumented: cmd+=['--timings-json' if engine=='tekt' else '--timings',str(trace)]
    with open(directory/(label+'.stdout'),'wb') as stdout, open(directory/(label+'.stderr'),'wb') as stderr:
        start=c.utc(); t=time.perf_counter_ns()
        proc=subprocess.Popen(cmd,cwd=c.ROOT,env=c.ENV,stdout=stdout,stderr=stderr,preexec_fn=lambda:os.sched_setaffinity(0,{c.CPU}))
        pid,status,usage=os.wait4(proc.pid,0)
        elapsed=(time.perf_counter_ns()-t)/1e6; proc.returncode=os.waitstatus_to_exitcode(status)
    result={'engine':engine,'fixture':fixture.name,'label':label,'start_utc':start,'end_utc':c.utc(),'command':cmd,'wall_ms':elapsed,'user_ms':usage.ru_utime*1000,'system_ms':usage.ru_stime*1000,'maxrss_kib':usage.ru_maxrss,'voluntary_context_switches':usage.ru_nvcsw,'involuntary_context_switches':usage.ru_nivcsw,'exit_code':proc.returncode,'instrumented':instrumented,'executable_sha256':c.sha(exe),'fixture_sha256':c.sha(fixture),'output':str(pdf.relative_to(c.OUT))}
    if pdf.exists(): result.update(output_sha256=c.sha(pdf),output_bytes=pdf.stat().st_size)
    if proc.returncode: raise RuntimeError('CLI failed: '+str(directory/(label+'.stderr')))
    if instrumented and engine=='tekt': result['timings']=json.loads(trace.read_text())
    return result

def run(rounds,instrumented):
    mode='instrumented' if instrumented else 'plain'
    for round_number in rounds:
        for fixture in sorted((c.OUT/'fixtures').glob('*.typ')):
            label=f'e2e-{mode}-r{round_number}-{fixture.stem}'
            directory=c.OUT/'e2e'/mode/f'r{round_number}'/fixture.stem
            if c.resume_or_retire(label,directory):continue
            directory.mkdir(parents=True,exist_ok=False)
            before=c.snapshot(label+'-before');c.require_built_state(before); contamination=[]; stop=threading.Event(); telemetry=[]
            def monitor():
                while not stop.is_set():
                    rows=c.processes(); hits=c.blockers(rows,None)
                    # The executing CLI is a direct child of this Python process.
                    if hits: contamination.extend(hits)
                    telemetry.append({'utc':c.utc(),'loadavg':pathlib.Path('/proc/loadavg').read_text().strip(),'blockers':hits})
                    stop.wait(.1)
            bad=c.blockers(c.processes(),None)
            if bad: raise RuntimeError('Concurrent workload before '+label)
            thread=threading.Thread(target=monitor,daemon=True);thread.start(); samples=[]
            try:
                for warm in range(2):
                    for engine in ['tekt','vanilla']:
                        one(engine,fixture,directory,f'warm{warm}-{engine}',instrumented)
                pairs=6 if instrumented else 12
                for pair in range(pairs):
                    order=['tekt','vanilla'] if (pair+round_number)%2==0 else ['vanilla','tekt']
                    for engine in order:
                        row=one(engine,fixture,directory,f'p{pair:02d}-{engine}',instrumented)
                        row.update(pair=pair,round=round_number); samples.append(row)
            finally:
                stop.set();thread.join()
            after=c.snapshot(label+'-after');c.identity_check()
            stable=before['sha256']==after['sha256'] and before['head']==after['head']
            valid=stable and not contamination
            c.save(pathlib.Path('e2e')/mode/f'r{round_number}'/fixture.stem/'samples.json',{'valid':valid,'fingerprint_before':before['sha256'],'fingerprint_after':after['sha256'],'samples':samples})
            c.save(pathlib.Path('receipts')/(label+'.json'),{'valid':valid,'source_stable':stable,'contamination':contamination,'telemetry':telemetry,'head':before['head'],'fingerprint_before':before['sha256'],'fingerprint_after':after['sha256']})
            print(label,'VALID' if valid else 'DISCARDED',flush=True)
            if not valid: raise RuntimeError('Contaminated CLI block; samples discarded')
