"""Compare the original edit and validate focused textual-style documents."""
from itertools import permutations
import json
import os
from random import Random

from watch import D, EXES, STATES, WORK, cycle, run

ARMS = ("candidate", "control", "vanilla")
PROTECT = {"TEKT_LAB_MEMO_PROTECT":"1", "TEKT_LAB_MEMO_BYTES":str(8*1024*1024)}
def main():
    os.sched_setaffinity(0,{2})
    rng = Random(50150)
    periods = [0]*12 + [347000]*18 + [937000]*18
    groups = [periods[i:i+6] for i in range(0,len(periods),6)]
    rng.shuffle(groups)
    with (WORK/"measurements.jsonl").open("w") as output:
        for group, (period,*_) in enumerate(groups):
            for i, order in enumerate(rng.sample(list(permutations(ARMS)),6)):
                block = 6*group+i
                for j, arm in enumerate(order):
                    engine = "vanilla" if arm=="vanilla" else "candidate"
                    EXES[engine] = WORK/arm
                    row = run(3*block+j,engine,period,overrides=PROTECT if engine!="vanilla" else None)
                    print(json.dumps(row|dict(block=block,arm=arm)),file=output,flush=True)
                print(f"{block+1}/48 {period}",flush=True)


if __name__=="__main__":
    main()
