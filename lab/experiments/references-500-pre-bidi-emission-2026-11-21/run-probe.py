"""Validate the RAM-only whole-line probe against integrated Tekt and vanilla."""
from importlib.util import module_from_spec, spec_from_file_location
import json
import os
from pathlib import Path
from random import Random
from statistics import median
import sys

here = Path(__file__).resolve().parent
root = here.parents[2]
out = root / "lab/results/construction/diagnostics/references-500-pre-bidi-emission-2026-11-21"
ram = Path((out / "ram-path.txt").read_text().strip())
assert str(ram).startswith("/dev/shm/")
os.environ.update(
    TEKT_LAB_WORK=str(ram), TEKT_CONTEXT_OUT=str(out),
    TEKT_VISIBLE_GOLD=str(root / "lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"),
)

def load(name, path):
    spec = spec_from_file_location(name, path)
    module = module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

base = load("emission_base_probe", here.parent / "references-500-context-substitution-2026-11-21/bench.py")
heading = load("emission_heading_probe", here.parent / "references-500-heading-reference-proof-2026-11-21/bench.py")
base.WORK = ram
normal_env = base.environment

def environment(case, arm, trace):
    env = normal_env(case, arm, trace)
    if arm == "proof" and trace:
        env["TEKT_LAB_WHOLE_TRACE"] = "1"
    return env

base.environment = environment
arms = ("census", "proof", "vanilla")
states = base.STATES
visible = states[-1][1]
shorter = visible.replace(b"height:600pt", b"height:440pt")
assert shorter != visible
retarget = shorter.replace(b"See @s0.", b"See @s13.", 1)
assert retarget != shorter
dependency = b'#include "middle.typ"\n' + visible
suites = {
    "original": states,
    "height-and-link": [("open", visible, None), ("shorter", shorter, None), ("retarget", retarget, None)],
    "indirect": [("open", dependency, base.cycle.opening.LEAF),
                 ("edit", dependency, base.cycle.opening.LEAF_EDIT),
                 ("restore", dependency, base.cycle.opening.LEAF)],
}
results = {}
for suite, inputs in ({} if "--timing-only" in sys.argv else suites).items():
    paths = {arm: ram / "probe" / suite / arm for arm in arms}
    runs = {arm: base.watch(paths[arm], arm, inputs, trace=True, keep=True) for arm in arms}
    comparisons = []
    for i, (state, _, _) in enumerate(inputs):
        a, b, vanilla = [heading.pdf_details(paths[arm] / f"{state}.pdf") for arm in arms]
        same = heading.compare(a, b)
        assert all(same.values()), (suite, state, same)
        assert runs["census"][i]["pdf_hash"] == runs["proof"][i]["pdf_hash"]
        if suite == "original":
            assert runs["vanilla"][i]["pdf_hash"] == base.golden("vanilla", state)
        comparisons.append(dict(state=state, same=same, pages=a["pages"], links=len(a["links"]),
                                vanilla_pages=vanilla["pages"], vanilla_links=len(vanilla["links"])))
    log = (paths["proof"] / "watch.log").read_text().split("compiled successfully")[:-1]
    stats = [{"hits":group.count("lab-whole-line hit"),
              "published":group.count("lab-whole-line published")}
             for group in log]
    results[suite] = dict(rows=runs, comparisons=comparisons,
                          positions="PDF bbox-layout comparison",
                          whole=stats)
    (out / "validation.json").write_text(json.dumps(results, indent=2, ensure_ascii=False) + "\n")
    print(suite, stats, comparisons, flush=True)

if "--validation-only" in sys.argv:
    raise SystemExit(0)

# Time the same four publications in interleaved paired blocks. Both binaries
# carry the same census clock with its trace disabled here.
order = [("census", "proof") if i % 2 else ("proof", "census") for i in range(12)]
Random(57217).shuffle(order)
timing = []
for block, arms_order in enumerate(order):
    for arm in arms_order:
        rows = base.watch(ram / "timing" / f"{block:02d}" / arm, arm, states, keep=True)
        timing.append(dict(block=block, arm=arm,
                           cpu_ns=rows[-1]["cpu_ns"], cycle_ns=sum(x["cpu_ns"] for x in rows)))
    (out / "timing.json").write_text(json.dumps(timing, indent=2) + "\n")
paired = []
for block in range(12):
    one = next(x for x in timing if x["block"] == block and x["arm"] == "census")
    two = next(x for x in timing if x["block"] == block and x["arm"] == "proof")
    paired.append((two["cpu_ns"]-one["cpu_ns"])/1e6)
print("visible paired ms", median(paired), "raw", paired, flush=True)
