"""Measure the native vanilla timing scopes on the stripped, pinned ELF."""
from collections import Counter
import json
import os
from pathlib import Path
from statistics import median
import struct
import time

from watch import WORK, STATES, EXES, cycle, driver, measure


def scopes(case):
    pairs = dict(struct.iter_unpack("<QQ", (case / "clock.bin").read_bytes()))
    events = json.loads((case / "timings-3.json").read_text())
    offsets = [round(event["ts"] * 1000) for event in events]
    zero, = (word for word in pairs if all(word + delta in pairs for delta in offsets[:12]))
    times = [pairs[zero + offset] for offset in offsets]
    assert len({event["tid"] for event in events}) == 1 and sorted(times) == times
    totals, counts, active = Counter(), Counter(), []
    for event, measured in zip(events, times):
        name = event["name"]
        if event["ph"] == "B":
            active.append((name, measured))
        else:
            prior, start = active.pop()
            assert name == prior
            totals[name] += measured-start
            counts[name] += 1
    assert not active and counts["compile once"] == counts["compile"] == counts["eval"] == counts["pdf"] == 1
    return totals, counts


def run(block, golden):
    case = WORK / "vanilla-traces" / f"{block:02d}"
    case.mkdir(parents=True)
    tmp, out = case / "tmp", case / "out.pdf"
    tmp.mkdir()
    cycle.base.runner.setup_sources(case, *STATES[0][1:])
    env = cycle.environment(tmp, "vanilla", False)
    env.update(LD_PRELOAD=str(WORK/"trace-clock.so"), TEKT_TRACE_CLOCK=str(case/"clock.bin"))
    command = [str(EXES["vanilla"]), "watch", str(case/"main.typ"), str(out), "--root", str(case),
               "--pdf-tagged=true", "--timings", str(case/"timings-{n}.json")]
    with (case/"watch.log").open("wb") as log:
        child = driver.Child(command, case, env, 1, log)
        try:
            last = None
            for i, (_, content, leaf) in enumerate(STATES):
                before = child.cpu() if i else 0
                start = time.perf_counter_ns()
                if i:
                    cycle.base.runner.setup_sources(case, content, leaf)
                last, published, _ = driver.await_pdf(child, out, last)
                after = measure.settle(child)
                deadline = time.monotonic() + 5
                while not (case / f"timings-{i}.json").is_file():
                    if time.monotonic() > deadline:
                        raise TimeoutError(case)
                    time.sleep(0.001)
                measure.settle(child)
                assert cycle.cycle.key("vanilla", out) == golden[STATES[i][0]], (case,i)
        finally:
            child.stop()
            child.close()
    totals, counts = scopes(case)
    phase_ns = dict(source=totals["create source"], eval=totals["eval"], physical=totals["layout document"],
                    paged_other=totals["compile"]-totals["create source"]-totals["eval"]-totals["layout document"],
                    pdf=totals["pdf"])
    phase_ns["other"] = (after-before)-sum(phase_ns.values())
    assert counts["layout document"] > 0 and all(x >= 0 for x in phase_ns.values()), (case, phase_ns)
    return dict(block=block, cpu_ms=(after-before)/1e6, wall_ms=(published-start)/1e6, utc=measure.utc(),
                phase_ms={k:v/1e6 for k,v in phase_ns.items()}, counts=counts)


def main():
    os.sched_setaffinity(0, {2})
    gold = json.loads(Path(os.environ["TEKT_VISIBLE_GOLD"]).read_text())["hashes"]["vanilla"]
    rows = [run(i,gold) for i in range(12)]
    output = dict(rows=rows, cpu_ms=median(x["cpu_ms"] for x in rows),
                  phase_ms={k:median(row["phase_ms"][k] for row in rows) for k in rows[0]["phase_ms"]})
    (WORK/"vanilla-phases.json").write_text(json.dumps(output, indent=2)+"\n")
    print(json.dumps(output["phase_ms"], indent=2), flush=True)


if __name__ == "__main__":
    main()
