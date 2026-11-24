from pathlib import Path
import json
import os
from random import Random
import sys

D=Path(__file__).resolve().parent
OUT=D.parents[1]/'results/construction/diagnostics'/D.name
sys.path.insert(0,str(D.parent/'references-500-visible-eval-residual-2026-11-21'))
from watch import EXES, WORK, run

combined='--combined' in sys.argv
arms=['candidate','control','vanilla'] if combined else ['control','vanilla']
label='post-profile' if combined else 'profile'
rng=Random(47253)
os.sched_setaffinity(0,{2})
periods=[347000]*10+[937000]*10
rng.shuffle(periods)
with (OUT/f'{label}-rows.jsonl').open('w') as file:
    for block, period in enumerate(periods):
        for slot, arm in enumerate(rng.sample(arms,len(arms))):
            engine='vanilla' if arm=='vanilla' else 'candidate'
            EXES[engine]=WORK/arm
            env={'TEKT_LAB_MEMO_PROTECT':'1','TEKT_LAB_MEMO_BYTES':str(8*1024*1024)}
            row=run(block*len(arms)+slot,engine,period,overrides=env if arm!='vanilla' else None,
                    directory=f'{label}-cases')
            print(json.dumps(row|dict(block=block,arm=arm)),file=file,flush=True)
        print(f'{block+1}/20',flush=True)
