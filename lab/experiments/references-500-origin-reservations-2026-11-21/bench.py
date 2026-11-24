from pathlib import Path
from itertools import permutations
from statistics import median
from random import Random
import json
import os
import sys

D=Path(__file__).resolve().parent
OUT=D.parents[1]/'results/construction/diagnostics'/D.name
sys.path.insert(0,str(D.parent/'references-500-visible-eval-residual-2026-11-21'))
from watch import WORK, EXES, driver, run
OUT=Path(os.environ.get("TEKT_LAB_TIMING_OUT", OUT))
CASE_DIRECTORY=os.environ.get("TEKT_LAB_CASE_DIRECTORY", "cases")
OPTIONS={'TEKT_LAB_MEMO_PROTECT':'1','TEKT_LAB_MEMO_BYTES':str(8*1024*1024)}
ARMS=('candidate','control','vanilla')
def main():
    controller_cpu = int(os.environ.get("TEKT_LAB_CONTROLLER_CPU", "2"))
    compiler_cpu = int(os.environ.get("TEKT_LAB_COMPILER_CPU", "1"))
    control_elf = Path(os.environ.get("TEKT_LAB_CONTROL_ELF", WORK / "control"))
    os.sched_setaffinity(0, {controller_cpu})
    spawn = driver.Child

    def start_pinned(command, directory, env, cpu, log):
        assert cpu == 1
        return spawn(command, directory, env, compiler_cpu, log)

    driver.Child = start_pinned
    rng=Random(int(os.environ.get("TEKT_LAB_ORDER_SEED", "67231")))
    pairs={}
    with (OUT/'measurements.jsonl').open('w') as file:
        for group in range(4):
            for i,order in enumerate(rng.sample(list(permutations(ARMS)),6)):
                block=group*6+i
                pairs[block]={}
                for pos,arm in enumerate(order):
                    engine='vanilla' if arm=='vanilla' else 'candidate'
                    EXES[engine]=control_elf if arm == "control" else WORK/arm
                    row=run(3*block+pos,engine,0,overrides=OPTIONS if arm!='vanilla' else None,
                              directory=CASE_DIRECTORY)
                    pairs[block][arm]=row['cpu_ns']/1e6
                    print(json.dumps(row|dict(block=block,arm=arm)),file=file,flush=True)
                print(f'{block+1}/24',flush=True)

    def stats(values):
        samples=sorted(median(rng.choices(values,k=len(values))) for _ in range(10000))
        return {'median':median(values),'ci95':[samples[249],samples[9749]]}
    arms={a:stats([v[a] for v in pairs.values()]) for a in ARMS}
    ratio=arms['candidate']['median']/arms['vanilla']['median']
    result={
        'cpu_ms':arms,'blocks':len(pairs),'ratio_of_medians':ratio,
        'controller_cpu': controller_cpu, 'compiler_cpu': compiler_cpu,
        'paired_ratio_candidate_over_vanilla':stats([v['candidate']/v['vanilla'] for v in pairs.values()]),
        'paired_reduction_vs_control_percent':stats([100*(1-v['candidate']/v['control']) for v in pairs.values()]),
        'reduction_to_match_percent':100*(1-1/ratio),
        'candidate_minus_control_ms':stats([v['candidate']-v['control'] for v in pairs.values()]),
        'candidate_minus_vanilla_ms':stats([v['candidate']-v['vanilla'] for v in pairs.values()]),
    }
    (OUT/'timing.json').write_text(json.dumps(result,indent=2)+'\n')
    print(result)
if __name__=='__main__':
    main()
