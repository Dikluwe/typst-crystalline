from pathlib import Path
import importlib.util
import itertools
import json
import os
from random import Random
import shutil
from statistics import median
import subprocess
import sys

D=Path(__file__).resolve().parent
ROOT=D.parents[2]
OUT=ROOT/'lab/results/construction/diagnostics'/D.name
sys.path.insert(0,str(D.parent/'references-500-visible-eval-residual-2026-11-21'))
from watch import WORK, EXES, run, cycle, STATES

ARMS=('candidate','control','vanilla')
OPTIONS={'TEKT_LAB_MEMO_PROTECT':'1','TEKT_LAB_MEMO_BYTES':str(8*1024*1024)}
CANDIDATE={'TEKT_LAB_PAGE_REPLAY':'1'}
HEADER=b'#set page(width: 400pt, height: 600pt, margin: 20pt)\n#set text(font: "Libertinus Serif")\n'
BREAK=b'#pagebreak()\n'
BODY=tuple(b''.join(
    f'Page {page} paragraph {index} has enough words to test page layout.\n\n'.encode()
    for index in range(20)) for page in range(23))
OPEN=HEADER+BREAK.join(BODY)
EDIT1=OPEN.replace(b'Page 0 paragraph 0', b'Visible page zero paragraph zero', 1)
EDIT2=EDIT1.replace(b'Page 0 paragraph 2', b'Another change in paragraph two', 1)
EDIT3=EDIT2.replace(b'Page 0 paragraph 4', b'Final change for paragraph four', 1)
EXPLICIT=[('startup',OPEN,None),('edit1',EDIT1,None),('edit2',EDIT2,None),('visible',EDIT3,None)]

def engine_for(arm):
    engine='vanilla' if arm=='vanilla' else 'candidate'
    EXES[engine]=WORK/arm
    return engine

def overrides(arm, trace=False):
    options={} if arm=='vanilla' else dict(OPTIONS)
    if arm=='candidate':
        options.update(CANDIDATE)
        if trace:
            options['TEKT_LAB_PAGE_TRACE']='1'
    return options

def fresh(arm, directory, data, leaf=None):
    engine=engine_for(arm)
    directory.mkdir(parents=True,exist_ok=True)
    main=directory/'main.typ'
    main.write_bytes(data)
    if leaf is not None:
        (directory/'middle.typ').write_bytes(b'#include "leaf.typ"\n')
        (directory/'leaf.typ').write_bytes(leaf)
    env=cycle.environment(directory, engine, False)
    pdf=directory/'fresh.pdf'
    with (directory/'fresh.log').open('wb') as log:
        subprocess.run(['taskset','-c','1',str(WORK/arm),'compile',str(main),str(pdf),'--root',str(directory),'--pdf-tagged=true'],
                       env=env,stdout=log,stderr=subprocess.STDOUT,check=True,timeout=60)
    return cycle.cycle.key(engine,pdf)

def trace_watch(index, label, states):
    row=run(index,engine_for('candidate'),0,overrides=overrides('candidate',True),states=states,check=False,directory=label)
    log=(WORK/row['directory']/'watch.log').read_text()
    trace=[line for line in log.splitlines() if 'lab-page-replay' in line]
    return row,trace

def smoke():
    row,trace=trace_watch(0,'explicit-smoke',EXPLICIT)
    expected={name:fresh('control',WORK/'explicit-fresh'/'control'/name,data,leaf) for name,data,leaf in EXPLICIT}
    result={'candidate':row,'trace':trace,'cold':expected}
    (OUT/'smoke.json').write_text(json.dumps(result,indent=2))
    print(trace,flush=True)
    assert row['all_pdfs']==expected, (row['all_pdfs'],expected)
    assert trace==['lab-page-replay native seeded=true']+['lab-page-replay hits=22/23']*3, trace
    expected_vanilla={name:fresh('vanilla',WORK/'explicit-fresh'/'vanilla'/name,data,leaf) for name,data,leaf in EXPLICIT}
    (OUT/'golden.json').write_text(json.dumps({'candidate':expected,'control':expected,'vanilla':expected_vanilla},indent=2))

def directed():
    middle=EDIT1.replace(b'Page 12 paragraph 2',b'A changed paragraph on the later page',1)
    long_body=(b'Wide words stretch the line width many times ' * 2 + b'with several more.\n\n') * 20
    overflow=OPEN.replace(BODY[0],long_body,1)
    dependency=b'#include "middle.typ"\n'+OPEN
    cases={
        'middle': ([('open',OPEN,None),('edit',middle,None),('restore',OPEN,None)],
                   ['native seeded=true','hits=21/23','hits=21/23']),
        'width': ([('open',OPEN,None),('edit',OPEN.replace(b'400pt',b'360pt',1),None),('restore',OPEN,None)],
                  ['native seeded=true']*3),
        'font': ([('open',OPEN,None),('edit',OPEN.replace(b'Libertinus Serif',b'New Computer Modern',1),None),('restore',OPEN,None)],
                 ['native seeded=true']*3),
        'overflow': ([('open',OPEN,None),('edit',overflow,None),('restore',OPEN,None)],
                      ['native seeded=true','native seeded=false','native seeded=true']),
        'dependency': ([('open',dependency,b'Original words.\n'),('edit',dependency,b'Changed external words.\n'),('restore',dependency,b'Original words.\n')],
                       ['refused: input or dependencies']*3),
        'context': ([('open',OPEN,None),('edit',OPEN+b'#context counter(page).display()\n',None),('restore',OPEN,None)],
                     ['native seeded=true','refused: input or dependencies','hits=23/23']),
    }
    results={}
    for index,(name,(states,wanted)) in enumerate(cases.items()):
        row, trace=trace_watch(index,'explicit-directed',states)
        expected={state:fresh('control',WORK/'explicit-directed-fresh'/name/state,data,leaf) for state,data,leaf in states}
        results[name]={'row':row,'trace':trace,'cold':expected}
        (OUT/'directed.json').write_text(json.dumps(results,indent=2))
        assert row['all_pdfs']==expected,(name,row['all_pdfs'],expected)
        assert trace==['lab-page-replay '+x for x in wanted],(name,trace,wanted)
        print(name, trace,flush=True)

def benchmark(kind):
    os.sched_setaffinity(0,{2})
    explicit=kind=='explicit'
    expected=json.loads((OUT/'golden.json').read_text()) if explicit else None
    states=EXPLICIT if explicit else STATES
    rng=Random(62121 if explicit else 21126)
    path=OUT/f'{kind}-measurements.jsonl'
    with path.open('w') as stream:
        for group in range(4):
            for i,order in enumerate(rng.sample(list(itertools.permutations(ARMS)),6)):
                block=group*6+i
                for j,arm in enumerate(order):
                    row=run(3*block+j,engine_for(arm),0,overrides=overrides(arm),
                            states=states,check=not explicit,directory=kind)
                    if explicit:
                        assert row['all_pdfs']==expected[arm],(block,arm)
                    print(json.dumps(row|dict(block=block,arm=arm)),file=stream,flush=True)
                print(f'{kind} {block+1}/24',flush=True)

if __name__=='__main__':
    mode=sys.argv[1]
    if mode=='smoke':
        smoke()
    elif mode=='directed':
        directed()
    else:
        assert mode in ['explicit','references']
        benchmark(mode)
