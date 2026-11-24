"""Compare the widened outline guard with the unmodified hybrid and vanilla."""
from collections import Counter
import gzip
import importlib.util
from itertools import permutations
import json
import os
from pathlib import Path
from random import Random
import subprocess

D = Path(__file__).resolve().parent
WORK = Path(os.environ["TEKT_LAB_WORK"])
PRIOR = D.parent / "references-500-hybrid-watch-cycle-2026-11-21" / "bench.py"
spec = importlib.util.spec_from_file_location("watch_cycle", PRIOR)
cycle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(cycle)
opening, base = cycle.opening, cycle.base
ARMS = ("candidate", "hybrid", "vanilla")
EXES = {arm: WORK / arm for arm in ARMS}
cycle.EXES = EXES
STATES = cycle.STATES
VISIBLE = STATES[-1][1]
GOLD = json.loads(Path(os.environ["TEKT_VISIBLE_GOLD"]).read_text())

original_environment = cycle.environment
def environment(tmp, arm, trace):
    return original_environment(tmp, "hybrid" if arm == "candidate" else arm, trace)

cycle.environment = environment


def watch(directory, arm, states=STATES, trace=False):
    cycle.STATES = states
    try:
        return cycle.watch(directory, arm, trace=trace)
    finally:
        cycle.STATES = STATES


def gold(arm, name):
    return GOLD["hashes"]["hybrid" if arm == "candidate" else arm][name]


def positions(log):
    result = []
    for part in log.read_text().split("compiled successfully")[:-1]:
        lines = part.splitlines()
        announced, = (int(line.split()[1]) for line in lines if line.startswith("watch-positions "))
        dumped, = (line for line in lines if line.startswith("lab-sealed-positions "))
        entries = {key: (int(page), float(x).hex(), float(y).hex())
                   for key, page, x, y in opening.POSITION.findall(dumped)}
        assert len(entries) == announced
        result.append(entries)
    return result


def destination_links(pdf):
    info = base.runner.describe(pdf)
    return Counter(dest for page in info["links"] for link in page
                   if isinstance((dest := link.get("/Dest") or link.get("/A", {}).get("/D")), str))


def directed():
    move = VISIBLE.replace(b"height:600pt", b"height:440pt")
    suite = {
        "original": STATES,
        "links": [("open", VISIBLE, None), ("move", move, None),
                  ("reference", move.replace(b"See @s0.", b"See @s13.", 1), None)],
        "dependency": [
            ("open", b'#include "middle.typ"\n' + VISIBLE, opening.LEAF),
            ("leaf-edit", b'#include "middle.typ"\n' + VISIBLE, opening.LEAF_EDIT),
            ("leaf-restore", b'#include "middle.typ"\n' + VISIBLE, opening.LEAF)],
        "span-query": opening.DIRECTED["positions"],
        "extra-material": [("visible", VISIBLE, None), ("page-break", VISIBLE + b"#pagebreak()\n", None)],
    }
    evidence = {}
    for kind, states in suite.items():
        case = WORK / "directed" / kind
        files = {arm:watch(case / arm, arm, states, trace=True) for arm in ("hybrid", "candidate")}
        maps = {arm:positions(case / arm / "watch.log") for arm in files}
        assert maps["candidate"] == maps["hybrid"], kind
        traces = {arm:opening.log_groups(case / arm / "watch.log") for arm in files}
        base.EXES["hybrid"] = EXES["hybrid"]
        base.fresh(case / "fresh", "hybrid", states, True)
        reports = []
        for number,(name,_,_) in enumerate(states):
            pdfs = {arm:WORK / data[number]["pdf"] for arm, data in files.items()}
            h = base.qdf_hash(pdfs["candidate"])
            assert h == base.qdf_hash(pdfs["hybrid"]) == base.qdf_hash(case / "fresh" / f"{name}.pdf"), (kind,name)
            reports.append(dict(state=name, qdf_sha256=h, positions=maps["candidate"][number],
                                traces={arm:traces[arm][number] for arm in files}))
        if kind == "original":
            assert traces["candidate"][-1]["native_proved"] == 1 and not traces["hybrid"][-1]["skipped"]
            for i in (1,2):
                assert all("watch-retain hit inert" in (case / arm / "watch.log").read_text().split("compiled successfully")[i]
                           for arm in files)
        if kind == "links":
            assert traces["candidate"][1]["checkpoint_attempted"] == 1 and traces["candidate"][1]["skipped"] == 1
            assert maps["candidate"][0] != maps["candidate"][1]
            before, after = (destination_links(WORK / files["candidate"][i]["pdf"]) for i in (1,2))
            assert before - after == {"u:s0": 1} and after - before == {"u:s13": 1}
        if kind == "dependency":
            assert all(not x["skipped"] for x in traces["candidate"])
            assert maps["candidate"][0] == maps["candidate"][2] != maps["candidate"][1]
            assert reports[0]["qdf_sha256"] == reports[2]["qdf_sha256"] != reports[1]["qdf_sha256"]
            leaf = WORK / files["candidate"][1]["pdf"]
            text = subprocess.check_output(["pdftotext", str(leaf), "-"], text=True)
            qdf = subprocess.check_output(["qpdf", "--qdf", str(leaf), "-"])
            assert "External omega." in text and "External alpha." not in text
            assert b"https://example.test/omega" in qdf and b"https://example.test/alpha" not in qdf
        if kind == "extra-material":
            assert traces["candidate"][0]["skipped"] == 1 and not traces["candidate"][1]["skipped"]
        evidence[kind] = reports
        print(f"directed {kind}: {len(reports)}", flush=True)
    cycle.dump("directed.json", evidence)


def timings(blocks):
    samples = {}
    for state in [name for name,_,_ in STATES] + ["total"]:
        entry = {}
        for metric in ("cpu_ns", "wall_ns"):
            values = {arm:[sum(row[metric] for row in block[arm] if state == "total" or row["state"] == state)/1e6
                           for block in blocks] for arm in ARMS}
            entry[metric] = dict(absolute={arm:base.bootstrap(data) for arm,data in values.items()},
                                 paired={arm:base.bootstrap([a-b for a,b in zip(values["candidate"],values[arm])])
                                         for arm in ("hybrid","vanilla")})
        samples[state] = entry
    cycle.dump("timing.json", samples)


def main():
    os.sched_setaffinity(0, {2})
    for arm in ARMS:
        watch(WORK / "warmup" / arm, arm)
    directed()
    orders = list(permutations(ARMS)) * 4
    Random(50021).shuffle(orders)
    blocks = []
    with gzip.open(WORK / "nominal.json.gz", "wt") as output:
        for n, order in enumerate(orders):
            block = {arm:watch(WORK / "nominal" / f"{n:02d}" / arm, arm) for arm in order}
            blocks.append(block)
            for rows in block.values():
                for row in rows:
                    output.write(json.dumps(row | {"block": n}) + "\n")
            print(f"{n+1:02d}/24 {' '.join(order)}", flush=True)
    timings(blocks)
    checks = Counter()
    for block in blocks:
        for arm, rows in block.items():
            for row in rows:
                name = row["state"]
                assert cycle.key(arm, WORK / row["pdf"]) == gold(arm, name), (arm, name, row)
                checks[arm,name] += 1
    cycle.dump("verified.json", {arm:{state:checks[arm,state] for state,_,_ in STATES} for arm in ARMS})


if __name__ == "__main__":
    main()
