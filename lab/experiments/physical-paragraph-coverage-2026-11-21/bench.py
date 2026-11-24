from pathlib import Path
import sys,os,json,importlib.util
from itertools import permutations
from random import Random
D=Path(__file__).resolve().parent
sys.path.insert(0,str(D.parent/'references-500-visible-eval-residual-2026-11-21'))
from watch import WORK,EXES,run,cycle
PROTECT={'TEKT_LAB_MEMO_PROTECT':'1','TEKT_LAB_MEMO_BYTES':str(8*1024*1024)}
def main():
 os.sched_setaffinity(0,{2});rng=Random(51121)
 periods=[0]*24+[577000]*12+[937000]*12
 groups=[periods[i:i+6] for i in range(0,len(periods),6)];rng.shuffle(groups)
 with (WORK/'measurements.jsonl').open('w') as f:
  for group,g in enumerate(groups):
   for i,order in enumerate(rng.sample(list(permutations(['candidate','control','vanilla'])),6)):
    block=group*6+i
    for j,arm in enumerate(order):
     engine='vanilla' if arm=='vanilla' else 'candidate';EXES[engine]=WORK/arm
     row=run(3*block+j,engine,g[0],overrides=PROTECT if engine!='vanilla' else None)
     print(json.dumps(row|dict(block=block,arm=arm)),file=f,flush=True)
    print(f'{block+1}/48 period={g[0]}',flush=True)
if __name__=='__main__':main()
