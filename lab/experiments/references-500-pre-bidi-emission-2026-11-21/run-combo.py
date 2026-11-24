"""Compare line replay plus existing pre-BiDi page probe with line replay and vanilla."""
from importlib.util import module_from_spec, spec_from_file_location
import json
import os
from pathlib import Path
from statistics import median

here = Path(__file__).resolve().parent
root = here.parents[2]
out = root / "lab/results/construction/diagnostics/references-500-pre-bidi-emission-2026-11-21"
ram = Path((out / "ram-path.txt").read_text().strip())
assert str(ram).startswith("/dev/shm/")
os.environ.update(TEKT_LAB_WORK=str(ram), TEKT_CONTEXT_OUT=str(out),
                  TEKT_VISIBLE_GOLD=str(root / "lab/results/construction/diagnostics/references-500-hybrid-watch-cycle-2026-11-21/validation.json"))

def load(name, path):
    spec = spec_from_file_location(name, path)
    module = module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

base = load("emission_combo_base", here.parent / "references-500-context-substitution-2026-11-21/bench.py")
heading = load("emission_combo_heading", here.parent / "references-500-heading-reference-proof-2026-11-21/bench.py")
base.WORK = ram
normal_env = base.environment

def environment(case, arm, trace):
    env = normal_env(case, arm, trace)
    if trace and arm in ("proof", "combo"):
        env["TEKT_LAB_WHOLE_TRACE"] = "1"
    if trace and arm == "combo":
        env["TEKT_LAB_WATCH_TRACE"] = "1"
    return env

base.environment = environment
states = base.STATES
visible = states[-1][1]
height = visible.replace(b"height:600pt", b"height:440pt")
retarget = height.replace(b"See @s0.", b"See @s13.", 1)
assert height != visible and retarget != height
suites = {
    "original": states,
    "height-link": [("open", visible, None), ("height", height, None), ("link", retarget, None)],
    "indirect": [("open", b'#include "middle.typ"\n'+visible, base.cycle.opening.LEAF),
                 ("edit", b'#include "middle.typ"\n'+visible, base.cycle.opening.LEAF_EDIT),
                 ("restore", b'#include "middle.typ"\n'+visible, base.cycle.opening.LEAF)],
}
evidence = {}
for suite, inputs in suites.items():
    paths = {arm: ram/"combo-validation"/suite/arm for arm in ("proof", "combo", "vanilla")}
    runs = {arm:base.watch(paths[arm], arm, inputs, trace=True, keep=True) for arm in paths}
    cases = []
    for i, (name, *_rest) in enumerate(inputs):
        a, b, v = [heading.pdf_details(paths[arm]/f"{name}.pdf") for arm in paths]
        same = heading.compare(a, b)
        assert all(same.values()), (suite,name,same)
        assert runs["proof"][i]["pdf_hash"] == runs["combo"][i]["pdf_hash"]
        cases.append(dict(name=name, same=same, pages=a["pages"], links=len(a["links"]),
                          vanilla_pages=v["pages"], vanilla_links=len(v["links"])))
    log = (paths["combo"]/"watch.log").read_text().split("compiled successfully")[:-1]
    reuse = [[line for line in group.splitlines() if line.startswith("pre-bidi-pages")]
             for group in log]
    whole = [dict(hits=group.count("lab-whole-line hit"), published=group.count("lab-whole-line published"))
             for group in log]
    evidence[suite] = dict(cases=cases, pre_bidi=reuse, whole=whole)
    (out/"combo-validation.json").write_text(json.dumps(evidence, indent=2, ensure_ascii=False)+"\n")
    print(suite, reuse, whole, flush=True)
