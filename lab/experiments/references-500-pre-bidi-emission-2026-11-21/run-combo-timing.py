"""Paired CPU of integrated control, line probe, combined probe, and pinned vanilla."""
from importlib.util import module_from_spec, spec_from_file_location
import json
import os
from pathlib import Path
from random import Random
from statistics import median

here = Path(__file__).resolve().parent
root = here.parents[2]
out = root / "lab/results/construction/diagnostics/references-500-pre-bidi-emission-2026-11-21"
ram = Path((out / "ram-path.txt").read_text().strip())
assert str(ram).startswith("/dev/shm/")
os.environ.update(TEKT_LAB_WORK=str(ram), TEKT_CONTEXT_OUT=str(out),
                  TEKT_VISIBLE_GOLD=str(root / "lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"))
spec = spec_from_file_location("emission_combo_timing", here.parent/"references-500-context-substitution-2026-11-21/bench.py")
base = module_from_spec(spec)
spec.loader.exec_module(base)
base.WORK = ram
arms = ("census", "proof", "combo", "vanilla")
schedule = [tuple(arms[(i+j) % 4] for j in range(4))
            for i in range(4) for _ in range(3)]
Random(24719).shuffle(schedule)
rows = []
for block, order in enumerate(schedule):
    for arm in order:
        observed = base.watch(ram/"combo-timing"/f"{block:02d}"/arm, arm, base.STATES, keep=True)
        rows.append(dict(block=block, arm=arm,
                         visible_ms=observed[-1]["cpu_ns"]/1e6,
                         cycle_ms=sum(x["cpu_ns"] for x in observed)/1e6,
                         hashes=[x["pdf_hash"] for x in observed]))
    current = [x for x in rows if x["block"] == block]
    tekt = {x["arm"]:x["hashes"] for x in current if x["arm"] != "vanilla"}
    assert tekt["census"] == tekt["proof"] == tekt["combo"]
    (out/"combo-timing.json").write_text(json.dumps(rows, indent=2)+"\n")
summary = {arm:dict(visible_median_ms=median(x["visible_ms"] for x in rows if x["arm"]==arm),
                    cycle_median_ms=median(x["cycle_ms"] for x in rows if x["arm"]==arm))
           for arm in arms}
for arm in arms[1:]:
    summary[arm]["visible_paired_minus_control_ms"] = median(
        next(x["visible_ms"] for x in rows if x["block"]==block and x["arm"]==arm)
        - next(x["visible_ms"] for x in rows if x["block"]==block and x["arm"]=="census")
        for block in range(12))
(out/"combo-timing-summary.json").write_text(json.dumps(summary, indent=2)+"\n")
print(json.dumps(summary, indent=2))
