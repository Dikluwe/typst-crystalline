"""Priority 4: baseline only, generated fixtures, sequential actual CLI watchers."""
import gzip, hashlib, importlib.util, json, os, statistics, subprocess, time
from pathlib import Path
ROOT=Path('/repos/Antigravity/typst-crystalline')
OUT=ROOT/'00_nucleo/diagnosticos/watch-retencao-2026-10-07'
WORK=Path('/tmp/watch-retencao-2026-10-07'); WORK.mkdir(exist_ok=True)
spec=importlib.util.spec_from_file_location('base',ROOT/'00_nucleo/diagnosticos/macro-micro-2026-10-05/watch_probe.py'); b=importlib.util.module_from_spec(spec);spec.loader.exec_module(b)
b.OUT=WORK
b.ENV.update(TZ='UTC', LC_ALL='C.UTF-8')
for key in list(b.ENV):
    if key.startswith(('TYPST_', 'CRYSTALLINE_', 'CARGO_PROFILE_')): b.ENV.pop(key)
def save(name,obj): (OUT/name).write_text(json.dumps(obj,indent=2,ensure_ascii=False)+'\n')
def rss(pid):
    fields=dict(line.split(':',1) for line in Path(f'/proc/{pid}/status').read_text().splitlines() if ':' in line)
    return {k:int(fields[k].split()[0]) for k in ('VmRSS','VmHWM')}
def state(name):
    p=b.provenance();p['source_sha256']=b.sha(Path(__file__))
    files={str(x.relative_to(ROOT)):b.sha(x) for layer in ('01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts') for x in (ROOT/layer).rglob('*') if x.is_file() and x.suffix in ('.rs','.md','.toml')}
    p['files']=files
    fonts=set(subprocess.check_output(['fc-list','--format','%{file}\n'],text=True).splitlines())
    p['font_files']={x:b.sha(Path(x)) for x in sorted(fonts) if Path(x).is_file()}
    for x in Path('/etc/fonts').rglob('*'):
        if x.is_file():p['font_files'][str(x)]=b.sha(x)
    (OUT/(name+'.json.gz')).write_bytes(gzip.compress(json.dumps(p).encode(),mtime=0));return p

def run(name,engine,round_):
    fixture,pdf,timing=(WORK/f'{name}-{engine}-{round_}{ext}' for ext in ('.typ','.pdf','.json'))
    fixture.write_text(b.source(name,0));pdf.unlink(missing_ok=True)
    cmd=[str(b.BIN[engine]),'watch',str(fixture),str(pdf),'--root',str(WORK),'--pdf-tagged=false','--pretty=false']
    cmd+=['--timings-json',str(timing)] if engine=='tekt' else ['--jobs','1']
    rows=[]
    with (WORK/f'{name}-{engine}-{round_}.log').open('w') as log:
        start=time.perf_counter_ns();proc=subprocess.Popen(cmd,cwd=ROOT,env=b.ENV,stdout=log,stderr=log,preexec_fn=lambda:os.sched_setaffinity(0,{b.CPU}))
        try:
            b.wait_pdf(proc,pdf,None);initial={'wall_ms':(time.perf_counter_ns()-start)/1e6,'cpu_ms':sum(b.cpu_runtime(proc.pid).values())/1e6,**rss(proc.pid)}
            for rev in range(1,11):
                time.sleep(.16);prev=b.generation(pdf);before=b.cpu_runtime(proc.pid);start=time.perf_counter_ns();fixture.write_text(b.source(name,rev));b.wait_pdf(proc,pdf,prev)
                elapsed=(time.perf_counter_ns()-start)/1e6;after=b.cpu_runtime(proc.pid)
                assert set(before)==set(after)
                text=subprocess.check_output(['pdftotext','-raw',str(pdf),'-'],text=True)
                assert (f'Hello World revision {rev}' if name=='hello' else 'C000') in text
                rows.append({'revision':rev,'latency_ms':elapsed,'cpu_ms':sum(after[t]-before[t] for t in after)/1e6,**rss(proc.pid),'source':fixture.read_text(),'source_sha256':b.sha(fixture),'text':text,'timing':json.loads(timing.read_text()) if engine=='tekt' else None})
        finally:
            proc.terminate();proc.wait(timeout=5)
    return {'fixture':name,'engine':engine,'round':round_,'command':cmd,'initial':initial,'samples':rows,'log':(WORK/f'{name}-{engine}-{round_}.log').read_text()}

def mutation(engine):
    w=WORK/('mutations-'+engine);w.mkdir(exist_ok=True)
    main=w/'main.typ';imp=w/'import.typ';asset=w/'asset.txt';pdf=w/'main.pdf';logpath=w/'watch.log'
    (w/'new.typ').unlink(missing_ok=True)
    imp.write_text('#let value = "Import A"');asset.write_text('Asset A')
    src='#import "import.typ": value\n#set text(font: "Libertinus Serif")\n#value\n#read("asset.txt")\n'
    main.write_text(src);pdf.unlink(missing_ok=True)
    cmd=[str(b.BIN[engine]),'watch',str(main),str(pdf),'--root',str(w),'--pdf-tagged=false','--pretty=false']
    if engine=='vanilla':cmd+=['--jobs','1']
    rows=[]
    with logpath.open('w') as log:
        proc=subprocess.Popen(cmd,cwd=ROOT,env=b.ENV,stdout=log,stderr=log,preexec_fn=lambda:os.sched_setaffinity(0,{b.CPU}))
        def change(label,action,expected=None):
            time.sleep(.2);prev=b.generation(pdf);old=b.sha(pdf);offset=logpath.stat().st_size;action()
            if expected is None:
                deadline=time.monotonic()+15
                while time.monotonic()<deadline:
                    assert proc.poll() is None
                    tail=logpath.read_text()[offset:]
                    if 'error:' in tail:break
                    time.sleep(.01)
                else:raise RuntimeError('missing error '+label)
                time.sleep(.2);assert b.sha(pdf)==old
                rows.append({'case':label,'last_artifact_preserved':True,'diagnostics':logpath.read_text()[offset:]})
            else:
                b.wait_pdf(proc,pdf,prev);text=subprocess.check_output(['pdftotext','-raw',str(pdf),'-'],text=True);assert expected in text,(label,text)
                rows.append({'case':label,'text':text,'expected':expected})
        try:
            b.wait_pdf(proc,pdf,None)
            change('edit-main',lambda:main.write_text(src+'Main B\n'),'Main B')
            change('edit-import',lambda:imp.write_text('#let value = "Import B"'),'Import B')
            change('edit-read',lambda:asset.write_text('Asset B'),'Asset B')
            change('delete-read',lambda:asset.unlink())
            change('recreate-read',lambda:asset.write_text('Asset C'),'Asset C')
            change('syntax-error',lambda:main.write_text(src+'#let ='))
            change('syntax-recovery',lambda:main.write_text(src+'Recovered\n'),'Recovered')
            change('missing-import',lambda:main.write_text('#import "new.typ": value\n#value'))
            change('create-missing-import',lambda:(w/'new.typ').write_text('#let value = "New Import"'),'New Import')
        finally:proc.terminate();proc.wait(timeout=5)
    return {'command':cmd,'cases':rows,'log':logpath.read_text()}

if __name__=='__main__':
    before=state('before');runs=[]
    for round_ in range(3):
        for name in ('hello','grid320'):
            for engine in (('tekt','vanilla') if round_%2==0 else ('vanilla','tekt')):
                print('run',round_,name,engine,flush=True);runs.append(run(name,engine,round_))
    mutations={e:mutation(e) for e in ('tekt','vanilla')}
    after=state('after');assert before['files']==after['files'];assert before['binaries']==after['binaries'];assert before['font_files']==after['font_files']
    for name in ('hello','grid320'):
        for r in range(3):
            t=next(x for x in runs if (x['fixture'],x['engine'],x['round'])==(name,'tekt',r));v=next(x for x in runs if (x['fixture'],x['engine'],x['round'])==(name,'vanilla',r));assert [x['text'] for x in t['samples']]==[x['text'] for x in v['samples']]
    summary={name:{e:{k:statistics.median(x[k] for r in runs if r['fixture']==name and r['engine']==e for x in r['samples']) for k in ('latency_ms','cpu_ms','VmRSS','VmHWM')} for e in ('tekt','vanilla')} for name in ('hello','grid320')}
    save('results.json',{'cpu':b.CPU,'env':{k:b.ENV[k] for k in ('RAYON_NUM_THREADS','SOURCE_DATE_EPOCH','TZ','LC_ALL')},'runs':runs,'mutations':mutations,'summary':summary,'note':'Polling belongs to latency, not compile CPU. No optimization materialized. No host-wide idle guarantee.'});print(json.dumps(summary),flush=True)
