#!/usr/bin/env python3
"""Controlled fresh-process font lifecycle comparison; original baseline stays immutable."""
import argparse, hashlib, json, os, pathlib, resource, shutil, subprocess, threading, time
import run as c
OUT = pathlib.Path(__file__).resolve().parent
original_files = c.source_files

def inputs():
    files = set(original_files())
    files.update(OUT / name for name in ['experiment.py', 'protocol.md', 'seal_owned.py', 'catalog_probe.rs', 'coverage-design.md', 'analyze.py', 'validate.py'])
    files.update(c.ROOT / name for name in json.loads((OUT/'initial.json').read_text())['originals'] if name.endswith('.md'))
    files.update(p for p in (OUT/'controls').glob('*') if p.is_file())
    return sorted(files)
c.source_files = inputs


def variants():
    return json.loads((OUT/'variants.json').read_text())

def identities():
    for name, row in variants().items():
        assert c.sha(row['path']) == row['sha256'], name+' executable changed'
    assert c.sha(c.ROOT/'lab/typst-original/target/release/typst') == c.VANILLA_SHA


def build(name):
    receipt = c.monitored(['cargo','build','--release','--offline','--locked','-p','typst-wiring','--message-format=json'], 'build-'+name, affinity=False)
    messages = []
    for line in (OUT/'logs'/('build-'+name+'.log')).read_text().splitlines():
        try:
            row = json.loads(line)
        except ValueError:
            continue
        if row.get('reason') == 'compiler-artifact' and row.get('executable') and row['target']['name']=='typst':
            messages.append(row)
    row = messages[-1]
    assert row['profile']['opt_level']=='3' and not row['profile']['debug_assertions']
    path=OUT/'bin'/(name+'-typst'); shutil.copy2(row['executable'],path)
    values=variants(); values[name]={'path':str(path),'sha256':c.sha(path),'profile':row['profile'],'source_before':receipt['fingerprint_before'],'source_after':receipt['fingerprint_after'],'command':receipt['command']}
    c.save(pathlib.Path('variants.json'), values)
    source = c.snapshot(name+'-built')
    c.save(pathlib.Path('build-state.json'), source)
    archive=OUT/'state'/(name+'-sources.tar.gz')
    import tarfile
    with tarfile.open(archive,'w:gz') as t:
        for p in c.source_files():
            if p.is_relative_to(c.ROOT): t.add(p,arcname=str(p.relative_to(c.ROOT)))
    print(name,'release executable archived',flush=True)


def build_probe(name):
    artifacts=[]
    for line in (OUT/'logs'/('build-'+name+'.log')).read_text().splitlines():
        try:row=json.loads(line)
        except ValueError:continue
        if row.get('reason')=='compiler-artifact' and row['target']['name']=='typst_infra':artifacts.append(row)
    row=artifacts[-1]
    library=next(p for p in row['filenames'] if p.endswith('.rlib'))
    path=OUT/'bin'/('catalog-'+name)
    cmd=['rustc','--edition=2021','-C','opt-level=3','-C','debug-assertions=no','-C','overflow-checks=no',str(OUT/'catalog_probe.rs'),'-L','dependency='+str(c.ROOT/'target/release/deps'),'--extern','typst_infra='+library,'-o',str(path)]
    receipt=c.monitored(cmd,'build-probe-'+name,affinity=False)
    saved=json.loads((OUT/'probes.json').read_text()) if (OUT/'probes.json').exists() else {}
    saved[name]={'path':str(path),'sha256':c.sha(path),'command':cmd,'infra_library':library,'infra_library_sha256':c.sha(library),'source':c.sha(OUT/'catalog_probe.rs'),'fingerprint':receipt['fingerprint_after'],'profile':row['profile']}
    c.save(pathlib.Path('probes.json'),saved)


def one(name, fixture, directory, label, traced=False, limit=False):
    values=variants(); exe=str(c.ROOT/'lab/typst-original/target/release/typst') if name=='vanilla' else values[name]['path']
    pdf=directory/(label+'.pdf'); trace=directory/(label+'.json')
    cmd=[exe,'compile',str(fixture),str(pdf),'--root',str(fixture.parent),'--pdf-tagged=false','--pretty=false','--font-path',str(OUT/'fonts')]
    if name=='vanilla': cmd+=['--jobs','1']
    if traced: cmd+=['--timings' if name=='vanilla' else '--timings-json',str(trace)]
    def setup():
        os.sched_setaffinity(0,{c.CPU})
        if limit: resource.setrlimit(resource.RLIMIT_AS,(512*1024*1024,512*1024*1024))
    with open(directory/(label+'.stdout'),'wb') as stdout,open(directory/(label+'.stderr'),'wb') as stderr:
        started=c.utc(); clock=time.perf_counter_ns()
        proc=subprocess.Popen(cmd,cwd=c.ROOT,env=c.ENV,stdout=stdout,stderr=stderr,preexec_fn=setup)
        _,status,usage=os.wait4(proc.pid,0); proc.returncode=os.waitstatus_to_exitcode(status)
        wall=(time.perf_counter_ns()-clock)/1e6
    row={'variant':name,'fixture':fixture.stem,'label':label,'utc_start':started,'utc_end':c.utc(),'command':cmd,'wall_ms':wall,'user_ms':usage.ru_utime*1000,'system_ms':usage.ru_stime*1000,'maxrss_kib':usage.ru_maxrss,'exit_code':proc.returncode,'executable_sha256':c.sha(exe),'fixture_sha256':c.sha(fixture),'output':str(pdf.relative_to(OUT)),'instrumented':traced,'rlimit_as_bytes':512*1024*1024 if limit else None}
    if pdf.exists():row['output_sha256']=c.sha(pdf)
    return row


def block(round_number, fixture, traced):
    mode='traced' if traced else 'plain'; label=f'{mode}-r{round_number}-{fixture.stem}'
    directory=OUT/'samples'/label
    if directory.exists():
        receipt_path=OUT/'receipts'/(label+'.json')
        old=json.loads(receipt_path.read_text()) if receipt_path.exists() else {}
        expected=json.loads((OUT/'build-state.json').read_text())
        if old.get('valid') and old.get('before')==expected['sha256'] and old.get('after')==expected['sha256'] and old.get('head')==expected['head']:
            identities();print(label,'retained',flush=True);return
        dest=OUT/'rejected'/(label+'-'+str(time.time_ns()));dest.parent.mkdir(exist_ok=True);shutil.move(directory,dest)
    directory.mkdir(parents=True)
    identities();before=c.snapshot(label+'-before');c.require_built_state(before)
    stop=threading.Event();contamination=[];telemetry=[]
    def monitor():
        while not stop.wait(.1):
            hit=c.blockers(c.processes(),None)
            telemetry.append({'utc':c.utc(),'loadavg':pathlib.Path('/proc/loadavg').read_text().strip(),'blockers':hit})
            if hit:contamination.append({'utc':c.utc(),'processes':hit})
    assert not c.blockers(c.processes(),None), 'Workload appeared before block'
    worker=threading.Thread(target=monitor);worker.start(); rows=[];warmups=[]
    names=['before','lazy','interval','vanilla']
    try:
        for name in names:
            for warm in range(2):warmups.append(one(name,fixture,directory,f'warm-{name}-{warm}',traced))
        for iteration in range(6 if traced else 12):
            shift=(iteration+round_number)%len(names);order=names[shift:]+names[:shift]
            if iteration%2:order=list(reversed(order))
            for name in order:
                row=one(name,fixture,directory,f'{iteration:02}-{name}',traced);row.update(round=round_number,iteration=iteration,order=order);rows.append(row)
    finally:
        stop.set();worker.join()
    after=c.snapshot(label+'-after');identities()
    stable=(before['sha256'],before['head'])==(after['sha256'],after['head'])
    valid=stable and not contamination and all(row['exit_code']==0 for row in rows+warmups)
    c.save(pathlib.Path('samples')/label/'samples.json',{'warmups':warmups,'samples':rows})
    c.save(pathlib.Path('receipts')/(label+'.json'),{'valid':valid,'before':before['sha256'],'after':after['sha256'],'head':before['head'],'contamination':contamination,'telemetry':telemetry,'round':round_number,'fixture':fixture.name,'instrumented':traced})
    print(label,'VALID' if valid else 'DISCARDED',flush=True)
    if not valid:raise RuntimeError('Rejected complete block '+label)


def measure(traced):
    identities()
    for round_number in range(1,4):
        c.quiet()
        for fixture in sorted((OUT/'fixtures').glob('*.typ')):
            if fixture.stem.startswith('memory-'):continue
            block(round_number,fixture,traced)


def catalog():
    probes=json.loads((OUT/'probes.json').read_text())
    for name,row in probes.items():assert c.sha(row['path'])==row['sha256']
    for round_number in range(1,4):
        c.quiet(); label=f'catalog-r{round_number}'; directory=OUT/'catalog'/label;directory.mkdir(parents=True,exist_ok=False)
        before=c.snapshot(label+'-before');c.require_built_state(before)
        stop=threading.Event();contamination=[]
        def monitor():
            while not stop.wait(.1):
                hit=c.blockers(c.processes(),None)
                if hit:contamination.append({'utc':c.utc(),'processes':hit})
        worker=threading.Thread(target=monitor);worker.start(); rows=[];warmups=[]
        def execute(name,label):
            exe=probes[name]['path'];cmd=[exe];clock=time.perf_counter_ns();started=c.utc()
            result=subprocess.run(cmd,cwd=c.ROOT,env=c.ENV,stdout=subprocess.PIPE,stderr=subprocess.PIPE,preexec_fn=lambda:os.sched_setaffinity(0,{c.CPU}))
            wall=(time.perf_counter_ns()-clock)/1e6
            (directory/(label+'.stdout')).write_bytes(result.stdout);(directory/(label+'.stderr')).write_bytes(result.stderr)
            assert result.returncode==0
            row=json.loads(result.stdout);row.update(variant=name,round=round_number,label=label,wall_ms=wall,utc_start=started,utc_end=c.utc(),command=cmd,executable_sha256=c.sha(exe));return row
        try:
            for name in ['lazy','interval']:
                for warm in range(2):warmups.append(execute(name,f'warm-{name}-{warm}'))
            for iteration in range(12):
                for name in (['lazy','interval'] if (iteration+round_number)%2 else ['interval','lazy']):
                    row=execute(name,f'{iteration:02}-{name}');row['iteration']=iteration;rows.append(row)
        finally:stop.set();worker.join()
        after=c.snapshot(label+'-after')
        for name,row in probes.items():assert c.sha(row['path'])==row['sha256']
        controls={(row['faces'],row['metadata_control']) for row in rows+warmups}
        valid=(before['sha256'],before['head'])==(after['sha256'],after['head']) and not contamination and len(controls)==1
        c.save(pathlib.Path('catalog')/label/'samples.json',{'samples':rows,'warmups':warmups})
        c.save(pathlib.Path('receipts')/(label+'.json'),{'valid':valid,'before':before['sha256'],'after':after['sha256'],'head':before['head'],'contamination':contamination,'metadata_controls':sorted(controls)})
        print(label,'VALID' if valid else 'DISCARDED',flush=True)
        assert valid, 'Catalog block rejected'


def controls():
    directory=OUT/'control-results';directory.mkdir(exist_ok=True);rows=[]
    for fixture in sorted((OUT/'controls').glob('*.typ')):
        for name in ['before','lazy','interval','vanilla']:
            row=one(name,fixture,directory,fixture.stem+'-'+name)
            row['note']='Untimed semantic validation; elapsed values are not admitted performance samples.'
            rows.append(row)
    c.save(pathlib.Path('control-results.json'),{'rows':rows})
    assert all(row['exit_code']==0 for row in rows), 'Control compilation failed'
    print('Multiscript and computed fallback compilations passed',flush=True)


def memory():
    directory=OUT/'memory';directory.mkdir(exist_ok=True);rows=[]
    for fixture in sorted((OUT/'fixtures').glob('memory-*.typ')):
        for name in ['before','lazy','interval','vanilla']:
            rows.append(one(name,fixture,directory,fixture.stem+'-'+name,limit=True))
    c.save(pathlib.Path('memory-results.json'),{'note':'Functional/memory validation, not admitted timing samples; address-space limit is the unchanged P1410 512 MiB. wait4 RSS has the parent floor limitation.','rows':rows})
    assert all(row['exit_code']==0 for row in rows), 'memory probe failed'
    print('P1410 probes passed under 512 MiB',flush=True)

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('action',choices=['build','build-probe','plain','traced','memory','catalog','controls']);parser.add_argument('--name');args=parser.parse_args()
    if args.action=='build':build(args.name)
    elif args.action=='build-probe':build_probe(args.name)
    elif args.action=='memory':memory()
    elif args.action=='catalog':catalog()
    elif args.action=='controls':controls()
    else:measure(args.action=='traced')
