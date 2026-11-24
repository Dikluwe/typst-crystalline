"""Run original watch and sample the latest paired Tekt probes and pinned vanilla."""
from collections import Counter
from itertools import permutations
import json
import os
from pathlib import Path
from random import Random
from shutil import copy2
import subprocess
import sys

from phases import PHYSICAL, TEKT, VANILLA, classify

D = Path(__file__).resolve().parent
sys.path.insert(0, str(D.parent/"references-500-retained-producer-cost-2026-11-21"))
import bench as previous

load, base, decoder = previous.load, previous.base, previous.decoder
WORK, OUT, STATES, ARMS = previous.WORK, previous.OUT, previous.STATES, previous.ARMS


def validate(gold):
    previous.validate(gold)
    evidence = {}
    expected = "pre-bidi-pages total=23 checked=23 reused=21 ordinals="+str(list(range(3,24)))
    for arm in ARMS[:2]:
        log = (WORK/"validation"/arm/"watch.log").read_text()
        groups = log.split("compiled successfully")[:-1]
        lines = [[line for line in group.splitlines() if line.startswith("pre-bidi-pages")]
                 for group in groups]
        assert len(lines)==4 and lines[-1]==[expected], (arm,lines)
        assert not lines[1] and not lines[2]
        evidence[arm]=lines
    previous.dump("page-reuse.json",evidence)


def capture(gold):
    rows = []
    order = list(permutations(ARMS))
    schedule = [(p,o) for p in (117000, 293000) for o in order*2]
    Random(80125).shuffle(schedule)
    profiler = D.parent/"unicode-pdf-cost-2026-11-19/profiler.so"
    copy2(profiler,WORK/"profiler.so")
    file = "measurements-profile.jsonl"
    with (WORK/file).open("w") as stream:
        for block,(period,arms) in enumerate(schedule):
            for arm in arms:
                for row in previous.watch(WORK/"profile"/f"{block:02d}-{arm}",arm,gold,period=period):
                    rows.append(dict(row,block=block))
                    print(json.dumps(rows[-1]),file=stream,flush=True)
            copy2(WORK/file,OUT/file)
            print(f"profile {block+1}/{len(schedule)}",flush=True)
    return rows


def profile(rows, tables):
    cases = {a:[] for a in ARMS}
    examples = Counter()
    for row in rows:
        if row["state"]!="visible":
            continue
        arm=row["arm"]
        stacks,detail=decoder.stacks(previous.Scratch(),row,tables)
        counts=Counter()
        for stack,n in stacks.items():
            phase=classify(arm,stack)
            counts[phase]+=n
            if phase!="other":
                examples[(arm,phase,stack)]+=n
        tick=detail["main_ns"]/detail["samples"]/1e6
        ms={k:counts[k]*tick for k in (VANILLA if arm=="vanilla" else TEKT)}
        assert abs(sum(ms.values())-detail["main_ns"]/1e6)<1e-8
        if arm=="vanilla":
            ms["layout_total"]=sum(ms[k] for k in VANILLA[:-3])
        else:
            ms["core_layout"]=sum(ms[k] for k in PHYSICAL)
            ms["root"]=sum(ms[k] for k in PHYSICAL if k.startswith("root_") or k.startswith("paragraph_"))
            ms["finish"]=sum(ms[k] for k in PHYSICAL if k.startswith("finish_"))
            ms["post_layout"]=sum(ms[k] for k in ("reuse_pages","bidi","shaping","line_reconciliation"))
            ms["physical_pipeline"]=ms["core_layout"]+sum(ms[k] for k in (
                "outline_preparation","source_plan","physical_indexes","physical_memo"))
        cases[arm].append(dict(**detail,block=row["block"],period_ns=row["period_ns"],counts=counts,ms=ms))
    ci=previous.initial.prior.residual.mean_ci
    stats={a:{k:dict(**ci([r["ms"][k] for r in cases[a]]),
                     ticks=sum(r["counts"].get(k,0) for r in cases[a]))
              for k in cases[a][0]["ms"]} for a in ARMS}
    left,right=cases["reuse"],cases["proof"]
    assert [r["block"] for r in left]==[r["block"] for r in right]
    paired={k:ci([b["ms"][k]-a["ms"][k] for a,b in zip(left,right)]) for k in left[0]["ms"]}
    previous.dump("profile.json",dict(phases=stats,paired_proof_minus_reuse=paired,cases=cases))
    with (OUT/"stacks.txt").open("w") as stream:
        for (arm,phase,stack),n in examples.most_common():
            print(arm,phase,n," | ".join(stack),file=stream)
    print("profile complete",flush=True)


def census(gold,tables):
    gdb=D.parent/"references-500-visible-pre-pdf-2026-11-21/scopes.gdb"
    evidence={}
    for arm in ARMS[:2]:
        case=WORK/"census"/arm
        (case/"edits").mkdir(parents=True)
        for i,(_,source,leaf) in enumerate(STATES):
            assert leaf is None
            (case/("main.typ" if not i else f"edits/{i}.typ")).write_bytes(source)
        symbols={kind:sorted(a for a,(_,name) in tables[arm][0].items() if name.endswith(suffix))
                 for kind,suffix in (("root","::layout_planned_root"),
                                     ("watch_wait","::watch::wait_for_change_since"))}
        assert len(symbols["watch_wait"])==1 and symbols["root"]
        (case/"symbols.json").write_text(json.dumps(symbols))
        exe=WORK/arm
        command=["gdb","-q","-nx","-batch","-x",str(gdb),"--args",str(exe),"watch","main.typ",
                 "out.pdf","--root",str(case),"--pdf-tagged=true"]
        log=case/"watch.log"
        with log.open("wb") as stream:
            run=subprocess.run(command,cwd=case,env=base.environment(case,arm,False),
                               stdout=stream,stderr=subprocess.STDOUT,timeout=60)
        copy2(log,OUT/f"census-{arm}.log")
        run.check_returncode()
        spans=json.loads((case/"scopes.json").read_text())
        assert len(spans["root"])==1 and "out_of_scope" not in spans, (arm,spans)
        hashes={s:base.cycle.cycle.key("candidate",case/f"publication-{i}.pdf")
                for i,(s,*_) in enumerate(STATES)}
        assert hashes==gold[arm],(arm,hashes)
        evidence[arm]=dict(count=len(spans["root"]),symbols=symbols,pdf=hashes)
    previous.dump("root-census.json",evidence)


def main():
    os.sched_setaffinity(0,{2})
    mode=sys.argv[1]
    gold={a:{s:base.golden("vanilla" if a=="vanilla" else "candidate",s) for s,*_ in STATES}
          for a in ARMS}
    tables=previous.configure_symbols()
    try:
        if mode=="nominal":
            validate(gold)
            previous.nominal(previous.measure(gold,mode))
        elif mode=="profile":
            profile(capture(gold),tables)
        elif mode=="direct":
            load("physical_direct",D/"direct.py").main(previous,gold,tables)
        elif mode=="census":
            census(gold,tables)
        else:
            raise ValueError(mode)
    finally:
        previous.archive(mode)


if __name__=="__main__":
    main()
