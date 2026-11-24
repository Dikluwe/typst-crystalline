"""Compare published Tekt and pinned Typst on the same edit, without lab switches."""
import hashlib
import json
import os
from pathlib import Path
from random import Random
import shutil
from statistics import median
import subprocess
import sys
import time

D = Path(__file__).resolve().parent
WORK = Path(os.environ["TEKT_LAB_WORK"])
OUT = Path(os.environ['TEKT_RESULT_DIR'])
OUT.mkdir(parents=True, exist_ok=True)
sys.path.insert(0, str(D.parent / 'references-500-visible-eval-residual-2026-11-21'))
from watch import STATES, cycle, driver, measure, toggle

ARMS = ('tekt', 'vanilla')
describe = cycle.cycle.original.compare.describe
EXES = {'tekt': WORK/'candidate', 'vanilla': WORK/'vanilla'}

class IncompleteWatch(Exception):
    pass

def dump(name, value):
    (OUT/name).write_text(json.dumps(value, indent=2) + '\n')

def env(case):
    return {k:v for k,v in driver.environment(case).items() if not k.startswith('TEKT_LAB_')}

def document_key(arm, pdf):
    return cycle.cycle.key(arm if arm == 'vanilla' else 'tekt', pdf)

def run(block, arm, period=0, *, area, expected=None, keep=False):
    case = WORK/area/f'{block:02d}-{arm}'
    case.mkdir(parents=True)
    pdf, source = case/'out.pdf', case/'main.typ'
    cycle.base.runner.setup_sources(case, *STATES[0][1:])
    environment = env(case)
    if period:
        environment.update(LD_PRELOAD=str(WORK/'profiler.so'),
            TEKT_CPU_TRACE=str(case/'samples.bin'), TEKT_CPU_ACK=str(case/'samples.ack'),
            TEKT_CPU_PERIOD=str(period))
    command = [str(EXES[arm]), 'watch', str(source), str(pdf), '--root', str(case),'--pdf-tagged=true']
    rows={}
    with (case/'watch.log').open('wb') as log:
        start=time.perf_counter_ns()
        child=driver.Child(command,case,environment,1,log)
        try:
            publication=None
            for i,(name,data,leaf) in enumerate(STATES):
                if period and i:
                    # Let the native watcher return to its wait loop before the
                    # next write, outside the sample and CPU windows.
                    time.sleep(0.12)
                if period and i == len(STATES)-1:
                    (case/'maps').write_text(Path(f'/proc/{child.process.pid}/maps').read_text())
                    toggle(child,case,1)
                before = child.cpu() if i else 0
                if i:
                    start=time.perf_counter_ns()
                    cycle.base.runner.setup_sources(case,data,leaf)
                publication, when, _ = driver.await_pdf(child,pdf,publication)
                after=measure.settle(child)
                if period and i == len(STATES)-1:
                    toggle(child,case,2)
                key = document_key(arm,pdf)
                if expected and key != expected[arm][name]['key']:
                    assert describe(pdf) == expected[arm][name]['description'], (arm, name, case)
                rows[name]=dict(cpu_ns=after-before,wall_ns=when-start,pdf_key=key)
                if keep:
                    shutil.copyfile(pdf,case/(name+'.pdf'))
        finally:
            child.stop()
            child.close()
    log=(case/'watch.log').read_text()
    if log.count('compiled successfully') != len(STATES):
        raise IncompleteWatch((case, log[-3000:]))
    return dict(block=block,arm=arm,period_ns=period,directory=str(case.relative_to(WORK)),
        exe=str(EXES[arm]),utc=measure.utc(),states=rows,cpu_ns=rows['visible']['cpu_ns'])

def validate():
    proven={}
    summary={}
    for arm in ARMS:
        row=run(0,arm,area='witness',keep=True)
        case=WORK/row['directory']
        proven[arm],summary[arm]={},{}
        for name,data,leaf in STATES:
            fresh=WORK/'fresh'/arm/name
            fresh.mkdir(parents=True)
            cycle.base.runner.setup_sources(fresh,data,leaf)
            with (fresh/'compile.log').open('wb') as log:
                subprocess.run(['taskset','-c','1',str(EXES[arm]),'compile','main.typ','out.pdf',
                    '--root',str(fresh),'--pdf-tagged=true'],cwd=fresh,env=env(fresh),
                    stdout=log,stderr=subprocess.STDOUT,timeout=120,check=True)
            hot=case/(name+'.pdf')
            info=describe(hot)
            assert info==describe(fresh/'out.pdf'), (arm,name)
            proven[arm][name]=dict(key=document_key(arm,hot),description=info)
            attribution=cycle.cycle.original.navigation(hot,info,name=='visible')
            summary[arm][name]=dict(pages=info['page_count'],links=sum(map(len,info['links'])),
                                    cold_equal=True,navigation=attribution)
    dump('validation.json',summary)
    dump('witness.json',proven)
    (OUT/'fixture.sha256').write_text(''.join(
        f'{hashlib.sha256(data).hexdigest()}  {name}\n' for name,data,_ in STATES))

def statistics(values):
    rng=Random(681999)
    samples=sorted(median(rng.choices(values,k=len(values))) for _ in range(10_000))
    return dict(median=median(values),ci95=[samples[249],samples[9749]])

def series(profile=False):
    expected=json.loads((OUT/'witness.json').read_text())
    label='profile' if profile else 'nominal'
    rng=Random(7291 if profile else 7287)
    driver.TIMEOUT=8
    periods=[347000,937000]*10 if profile else [0]*24
    rng.shuffle(periods)
    paired=[]
    with (OUT/(label+'.jsonl')).open('w') as log:
        for block,period in enumerate(periods):
            flip=((block//2)+block)%2 if profile else (block+block//4)%2
            order=ARMS[::-1] if flip else ARMS
            pair={}
            for arm in order:
                for attempt in range(3):
                    area=label+'-cases'
                    if attempt:
                        area+=f'/retry-{attempt}'
                    try:
                        row=run(block,arm,period,area=area,expected=expected)
                        break
                    except (TimeoutError, IncompleteWatch):
                        if attempt == 2:
                            raise
                        print('retry',arm,block,attempt+1,flush=True)
                print(json.dumps(row),file=log,flush=True)
                pair[arm]=row
            paired.append(pair)
            print(label,block+1,'/',len(periods),flush=True)
    if profile:
        return
    result={}
    for name,_,_ in STATES:
        ms={arm:[row[arm]['states'][name]['cpu_ns']/1e6 for row in paired] for arm in ARMS}
        ratios=[t/v for t,v in zip(ms['tekt'],ms['vanilla'])]
        result[name]=dict(cpu_ms={arm:statistics(values) for arm,values in ms.items()},
            paired_gap_ms=statistics([t-v for t,v in zip(ms['tekt'],ms['vanilla'])]),
            paired_ratio=statistics(ratios),
            cut_to_parity_percent=100*(1-1/median(ratios)),
            wall_ms={arm:median(row[arm]['states'][name]['wall_ns']/1e6 for row in paired)
                     for arm in ARMS})
    dump('timing.json',result)

if __name__=='__main__':
    os.sched_setaffinity(0,{2})
    {'validate':validate,'nominal':series,'profile':lambda:series(True)}[sys.argv[1]]()
