"""Run the four original watch publications with the existing fixture driver."""
from importlib.util import module_from_spec, spec_from_file_location
from pathlib import Path
from random import Random
from statistics import median
import ast
import json
import os
import re
import sys

here = Path(__file__).resolve().parent
ram = Path(os.environ["TEKT_LAB_WORK"])
out = Path(os.environ["TEKT_CONTEXT_OUT"])
assert str(ram).startswith("/dev/shm/")

def load(name, rel):
    spec = spec_from_file_location(name, here.parent / rel)
    mod = module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod

base = load("line_watch", "references-500-context-substitution-2026-11-21/bench.py")
details = load("line_pdf", "references-500-heading-reference-proof-2026-11-21/bench.py")
base.WORK = ram
normal_env = base.environment
states = base.STATES
(mode,) = sys.argv[1:]
def environment(case, arm, trace):
    env = normal_env(case, arm, trace)
    if arm == "proof":
        env["TEKT_LINE_REUSE"] = "1"
    if trace and arm in ("census", "proof", "cold"):
        env["TEKT_LINE_CENSUS"] = "1"
    return env

base.environment = environment

def dump(name, value):
    (out / name).write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n")

def watch(suite, arm, inputs=states, trace=False):
    folder = ram / suite / arm
    rows = base.watch(folder, arm, inputs, trace=trace, keep=True)
    return folder, rows

def census(folder, scope="root"):
    content = (folder / "watch.log").read_text()
    fields = ("page", "calls", "cpu_ns", "alloc", "realloc", "requested_bytes", "hits", "builds", "edge_queries_saved")
    groups = content.split("compiled successfully")[:-1]
    return [[[dict(zip(fields, row, strict=True)) for row in ast.literal_eval(payload)]
             for payload in re.findall(rf"^lab-line-commit-{scope} (.*)$", group, re.M)] for group in groups]

def validate(suite, inputs, arms):
    results = {arm: watch(suite, arm, inputs, trace=True) for arm in arms}
    if suite in ("control", "candidate"):
        dump(f"{suite}-watch-rows.json", {arm:rows for arm, (_, rows) in results.items()})
        for arm, (folder, _) in results.items():
            if arm in ("census", "proof", "cold"):
                dump(f"{suite}-{arm}-scopes.json",
                     {scope:census(folder, scope) for scope in ("root", "finish")})
    evidence = []
    for i, (state, *_) in enumerate(inputs):
        pdf = {arm: details.pdf_details(folder / f"{state}.pdf")
               for arm, (folder, _) in results.items()}
        expected = results["integrated"][1][i]["pdf_hash"]
        for arm, (_, rows) in results.items():
            if arm != "vanilla":
                assert rows[i]["pdf_hash"] == expected, (suite, state, arm)
                assert all(details.compare(pdf["integrated"], pdf[arm]).values())
        expected_navigation = None
        if suite in ("control", "candidate"):
            historical = base.cycle.cycle.original
            expected_navigation = {}
            for arm in ("integrated", "vanilla"):
                path = results[arm][0] / f"{state}.pdf"
                nav = historical.navigation(path, historical.compare.describe(path), state == "visible")
                counts = {key: len(entries) for key, entries in nav["wrong_pages"].items()}
                expected_navigation[arm] = counts
        evidence.append(dict(state=state, wrong_destination_pages=expected_navigation,
                             hashes={arm: rows[i]["pdf_hash"] for arm, (_, rows) in results.items()},
                             pages={arm: p["pages"] for arm,p in pdf.items()},
                             links={arm: len(p["links"]) for arm,p in pdf.items()},
                             navigation={arm: {k:p[k] for k in ("links", "names", "outlines")}
                                         for arm,p in pdf.items() if arm in ("vanilla","integrated")},
                             positions={arm:p["positions"] for arm,p in pdf.items()}))
    if suite in ("control", "candidate"):
        for state, record in zip(states, evidence):
            assert record["hashes"]["vanilla"] == base.golden("vanilla", state[0])
    return evidence, results

if mode == "census":
    evidence, results = validate("control", states, ("integrated", "census", "vanilla"))
    dump("control.json", evidence)
    c = census(results["census"][0])
    assert len(c) == 4 and all(c)
    dump("control-pages.json", c)
    dump("control-finish-pages.json", census(results["census"][0], "finish"))
    repeats = []
    for i in range(4):
        folder, rows = watch(f"census-repeat/{i}", "census", trace=True)
        assert [r["pdf_hash"] for r in rows] == [r["hashes"]["integrated"] for r in evidence]
        repeats.append(dict(root=census(folder), finish=census(folder, "finish"), rows=rows))
    dump("control-repeats.json", repeats)
    raise SystemExit

(ram/"cold").symlink_to("proof")
if mode == "resume":
    evidence = json.loads((out / "candidate.json").read_text())
else:
    evidence, result = validate("candidate", states, ("integrated", "census", "cold", "proof", "vanilla"))
    dump("candidate.json", evidence)
    cold, proof = (census(result[arm][0]) for arm in ("cold","proof"))
    dump("proof-pages.json", dict(cold=cold, proof=proof))
    dump("proof-finish-pages.json", {arm:census(result[arm][0], "finish") for arm in ("cold","proof")})
    hits = sum(row["hits"] for root in proof[-1] for row in root)
    print("visible hits", hits, flush=True)
    assert hits > 0, "probe did not exercise replay"

# Move page boundaries, retarget a reference, then exercise only a leaf include.
visible = states[-1][1]
shorter = visible.replace(b"height:600pt", b"height:440pt")
retarget = shorter.replace(b"See @s0.", b"See @s13.", 1)
assert shorter != visible and retarget != shorter
leaf = b'#include "middle.typ"\n' + visible
matrix = {
    "height-and-link": [("open", visible, None), ("height", shorter, None), ("link", retarget, None)],
    "indirect": [("open", leaf, base.cycle.opening.LEAF), ("edit", leaf, base.cycle.opening.LEAF_EDIT),
                 ("restore", leaf, base.cycle.opening.LEAF)],
}

arms = ("integrated", "cold", "proof", "vanilla")
schedule = [tuple(arms[(i+j)%4] for j in range(4)) for i in range(4) for _ in range(3)]
Random(40320).shuffle(schedule)
raw = []
load_by_block = []
for block, order in enumerate(schedule):
    load_by_block.append(dict(block=block, order=order,
                              load_before=Path("/proc/loadavg").read_text().strip()))
    for arm in order:
        folder, rows = watch(f"timing/{block:02}", arm)
        for i, row in enumerate(rows):
            comparator = "vanilla" if arm == "vanilla" else "integrated"
            assert row["pdf_hash"] == evidence[i]["hashes"][comparator], (block, arm, row)
            raw.append(dict(row, block=block))
        for path in folder.glob("*.pdf"):
            path.unlink()
    load_by_block[-1]["load_after"] = Path("/proc/loadavg").read_text().strip()
    dump("timing.json", raw)
    dump("timing-load.json", load_by_block)

visible_ms = {arm: {row["block"]:row["cpu_ns"]/1e6 for row in raw
                    if row["state"] == states[-1][0] and row["arm"] == arm} for arm in arms}
summary = {"median_ms":{arm:median(values.values()) for arm,values in visible_ms.items()}}
for arm, compare in (("proof","cold"),("proof","integrated"),("vanilla","integrated")):
    deltas = [visible_ms[arm][k]-visible_ms[compare][k] for k in range(len(schedule))]
    rng = Random(44142)
    bootstrap = sorted(median(rng.choices(deltas, k=len(deltas))) for _ in range(10000))
    summary[f"{arm}_minus_{compare}"] = dict(median_ms=median(deltas),
                                            ci95_ms=[bootstrap[250], bootstrap[9749]])
dump("timing-summary.json", summary)

for suite, inputs in matrix.items():
    records, _ = validate(suite, inputs, ("integrated", "proof", "vanilla"))
    if suite == "indirect":
        for arm in ("integrated", "proof", "vanilla"):
            original, edited, restored = (record["hashes"][arm] for record in records)
            assert original == restored and original != edited, arm
    dump(f"{suite}{'-resume' if mode == 'resume' else ''}.json", records)
