"""Census the memo in a separate run, then compare four watch arms in RAM."""
import json
import os
from pathlib import Path
from random import Random
import subprocess

from variants import make_variants
from watch import D, EXES, STATES, WORK, cycle, golden, run

ARMS = ("cap16","cap32","memory","vanilla")
T = {"cap16": 16, "cap32": 32, "memory": 8*1024*1024, "pressure": 256*1024}


def census(arm, states=STATES, verify_reference=True):
    case = WORK / "census" / arm
    (case/"edits").mkdir(parents=True)
    for i, (_, source, leaf) in enumerate(states):
        assert leaf is None
        (case/("main.typ" if i == 0 else f"edits/{i}.typ")).write_bytes(source)
    args = ["taskset","-c","1","gdb","-q","-nx","-batch","-x",str(D/"census.gdb"),
            "--args",str(WORK/arm),"watch","main.typ","out.pdf","--root",str(case),"--pdf-tagged=true"]
    with (case/"gdb.log").open("wb") as log:
        subprocess.run(args,cwd=case,env=cycle.environment(case,"candidate",False),
                       stdout=log,stderr=subprocess.STDOUT,timeout=60,check=True)
    if verify_reference:
        for i,(name,_,_) in enumerate(states):
            assert cycle.cycle.key("candidate",case/f"pub-{i}.pdf") == golden("candidate",name)
    stats = json.loads((case/"census.json").read_text())
    totals = []
    for rows in stats:
        totals.append(dict(loops=len(rows),body=sum(r["body"] for r in rows),
                           hit=sum(r["hit"] for r in rows), miss=sum(r["miss"] for r in rows),
                           stores=sum(r["store"] for r in rows), evictions=sum(len(r["evictions"]) for r in rows),
                           max_bytes=max((r["retained_bytes"] for r in rows),default=0),
                           max_entries=max((r["retained_entries"] for r in rows),default=0)))
    limit = T[arm] if arm in ("memory","pressure") else T["memory"]
    assert all(t["max_bytes"]<=limit for t in totals)
    return totals


def main():
    os.sched_setaffinity(0,{2})
    make_variants(WORK)
    counts = {arm:census(arm) for arm in ("cap16","cap32","memory","pressure")}
    for arm,t in counts.items():
        print(arm,t[0],t[-1],flush=True)
    assert counts["cap16"][-1]["body"]==500
    for arm in ("cap32","memory"):
        assert counts[arm][-1]["hit"]==19 and counts[arm][-1]["body"]==25 and counts[arm][-1]["evictions"]==0
    assert counts["pressure"][0]["evictions"]>0 and counts["pressure"][-1]["evictions"]>0
    (WORK/"census-summary.json").write_text(json.dumps(counts,indent=2)+"\n")
    # A balanced order in every group of four blocks.
    rng = Random(50128)
    base_orders = [(0,1,3,2),(1,2,0,3),(2,3,1,0),(3,0,2,1)]
    periods = [0,0,0,0,577000,577000,937000,937000]
    rng.shuffle(periods)
    with (WORK/"measurements.jsonl").open("w") as output:
        for group,period in enumerate(periods):
            labels = rng.sample(ARMS,4)
            for index,order in enumerate(rng.sample(base_orders,4)):
                block = 4*group+index
                for j,index_arm in enumerate(order):
                    arm = labels[index_arm]
                    exe_arm = "vanilla" if arm=="vanilla" else "candidate"
                    EXES[exe_arm] = WORK/arm
                    row = run(4*block+j,exe_arm,period)
                    print(json.dumps(row|dict(arm=arm,block=block)),file=output,flush=True)
                print(f"{block+1}/32 {period}",flush=True)


if __name__=="__main__":
    main()
