from pathlib import Path
import json
import os
import sys
from random import Random
from statistics import median

D=Path(__file__).resolve().parent
OUT=D.parents[1]/'results/construction/diagnostics'/D.name
sys.path.insert(0,str(D.parent/'references-500-visible-eval-residual-2026-11-21'))
from watch import WORK, EXES, run
os.sched_setaffinity(0,{2})
opts={'TEKT_LAB_MEMO_PROTECT':'1','TEKT_LAB_MEMO_BYTES':str(8*1024*1024)}
rng=Random(3712)
orders=[('tekt','vanilla'),('vanilla','tekt')]*12
rng.shuffle(orders)
values={}
with (OUT/'measurements.jsonl').open('w') as stream:
    for block, order in enumerate(orders):
        values[block]={}
        for pos,arm in enumerate(order):
            engine='candidate' if arm=='tekt' else 'vanilla'
            EXES[engine]=WORK/('control' if arm=='tekt' else arm)
            row=run(2*block+pos,engine,0,overrides=opts if arm=='tekt' else None,directory='matched')
            values[block][arm]=row['cpu_ns']/1e6
            print(json.dumps(row|dict(block=block,arm=arm)),file=stream,flush=True)
        print(f'{block+1}/24',flush=True)
def stats(xs):
    samples=sorted(median(rng.choices(xs,k=len(xs))) for _ in range(10000))
    return {'median':median(xs),'ci95':[samples[249],samples[9749]]}
arms={a:stats([v[a] for v in values.values()]) for a in ('tekt','vanilla')}
ratio=arms['tekt']['median']/arms['vanilla']['median']
result={'cpu_ms':arms,'tekt_minus_vanilla_ms':stats([v['tekt']-v['vanilla'] for v in values.values()]),
        'ratio_of_medians':ratio,'reduction_to_match_percent':100*(1-1/ratio),'blocks':len(values)}
(OUT/'timing.json').write_text(json.dumps(result,indent=2)+'\n')
print(result)
