"""Run the original four-publication watch and summarize emitted lines by page."""
from importlib.util import module_from_spec, spec_from_file_location
import ast
import json
import os
from pathlib import Path
import re
from shutil import copy2
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

base = load("emission_base", here.parent / "references-500-context-substitution-2026-11-21/bench.py")
original_env = base.environment

def environment(case, arm, trace):
    env = original_env(case, arm, trace)
    if arm == "census":
        env["TEKT_LAB_EMISSION_CENSUS"] = "1"
    return env

base.environment = environment
base.WORK = ram
states = base.STATES
rows = {}
for arm in ("census", "vanilla"):
    folder = ram / "watch" / arm
    if (folder / "watch.log").exists():
        engine = "vanilla" if arm == "vanilla" else "candidate"
        rows[arm] = [dict(state=state, pdf_hash=base.cycle.cycle.key(engine, folder / f"{state}.pdf"))
                     for state, *_ in states]
    else:
        rows[arm] = base.watch(folder, arm, states, trace=True, keep=True)

groups = (ram / "watch/census/watch.log").read_text().split("compiled successfully")[:-1]
assert len(groups) == len(states)
copy2(ram / "watch/census/watch.log", out / "census-watch.log")
copy2(ram / "watch/vanilla/watch.log", out / "vanilla-watch.log")
pattern = re.compile(r"^lab-emission-pages (\{.*\})$", re.M)
emission = []
for group in groups:
    records = [ast.literal_eval(x) for x in pattern.findall(group)]
    emission.append(records)

gold = load("emission_heading", here.parent / "references-500-heading-reference-proof-2026-11-21/bench.py")
pdf = {}
for state, *_ in states:
    pdf[state] = {arm: gold.pdf_details(ram / "watch" / arm / f"{state}.pdf")
                  for arm in ("census", "vanilla")}

summary = dict(head=(out / "head.txt").read_text().strip(),
               source=(out / "product-before.sha256").read_text().split()[0],
               states=[state for state, *_ in states], rows=rows,
               prior_gold={state:{"tekt":base.golden("candidate", state),
                                  "vanilla":base.golden("vanilla", state)} for state, *_ in states},
               emission=emission,
               pages={state:{arm:pdf[state][arm]["pages"] for arm in pdf[state]} for state in pdf},
               links={state:{arm:len(pdf[state][arm]["links"]) for arm in pdf[state]} for state in pdf},
               navigation={state:{arm:dict(names=pdf[state][arm]["names"], outlines=pdf[state][arm]["outlines"])
                                  for arm in pdf[state]} for state in pdf})
(out / "census.json").write_text(json.dumps(summary, indent=2, ensure_ascii=False) + "\n")
print(json.dumps({"emission": emission, "pages": summary["pages"], "links":summary["links"]}, ensure_ascii=False))
