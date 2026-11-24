"""Paired startup timings plus directed physical and PDF witnesses."""
from collections import Counter
import gzip
import importlib.util
from itertools import permutations
import json
import os
from pathlib import Path
from random import Random
import re
from statistics import median
import subprocess

D = Path(__file__).resolve().parent
WORK = Path(os.environ["TEKT_LAB_WORK"])
PRIOR = D.parent / "references-500-pdf-export-reuse-2026-11-20"
spec = importlib.util.spec_from_file_location("control_bench", PRIOR / "bench.py")
base = importlib.util.module_from_spec(spec)
spec.loader.exec_module(base)
measure = base.measure
BASE = base.REGULAR[0][1]
ARMS = ("both", "omit", "same-off", "vanilla")
PATHS = {arm: WORK / exe for arm, exe in zip(ARMS, ("control", "variant", "variant", "vanilla"))}
ORIGINAL_ENV = base.driver.environment
MEMO = re.compile(r"^watch-memo gen=(\d+) replay=(true|false) hits=(\d+) builds=(\d+) refusals=(\d+)", re.M)

NOMINAL = [base.REGULAR[0]]
DYNAMIC = b'#include "middle.typ"\n' + BASE
LEAF = b'= Original External <external>\n#link("https://example.test/alpha")[External alpha.]\n'
LEAF_EDIT = b'#pagebreak()\n= Changed External <external>\n#link("https://example.test/omega")[External omega.]\n'
NUMBERED = BASE.replace(b"#outline()", b'#set page(numbering: "1 / 1")\n#counter(page).update(7)\n#outline()', 1)
NUMBERED = NUMBERED.replace(b"See @s0.", b'See #ref(<s0>, form: "page").', 1)
FUNCTIONAL = BASE.replace(b"#outline()", b'#set page(numbering: (n, ..rest) => [NUM #n])\n#outline()', 1)
DIRECTED = {
    "regular": list(base.REGULAR),
    "moves-links": [
        ("opening", BASE, None),
        ("move", BASE.replace(b"height:600pt", b"height:440pt"), None),
        ("link", BASE.replace(b"height:600pt", b"height:440pt").replace(b"See @s0.", b"See @s13.", 1), None),
    ],
    "indirect": [("opening", DYNAMIC, LEAF), ("leaf-edit", DYNAMIC, LEAF_EDIT),
                 ("leaf-restore", DYNAMIC, LEAF)],
    "positions": list(base.previous.SPANS[:1]) + [base.previous.SPANS[4]],
    "page-reference": [("opening", NUMBERED, None),
                       ("counter-edit", NUMBERED.replace(b"update(7)", b"update(19)"), None)],
    "functional-numbering": [("opening", FUNCTIONAL, None)],
    "plain": [("opening", BASE.replace(b"#outline()\n", b""), None)],
    "counter-state": [("opening", BASE.replace(
        b"See @s0.", b'#counter("lab").update(7) #context [Counter: #counter("lab").get().first()].', 1), None)],
}


def run(directory, arm, states, trace=False):
    source_arm = "vanilla" if arm == "vanilla" else "both"
    base.EXES[source_arm] = PATHS[arm]
    if arm == "omit":
        def enabled(tmp):
            env = ORIGINAL_ENV(tmp)
            env["TEKT_LAB_SKIP_PROVISIONAL_REPLAY"] = "1"
            return env
        base.driver.environment = enabled
    try:
        return [dict(row, arm=arm) for row in base.run_watch(directory, source_arm, states, trace=trace)]
    finally:
        base.driver.environment = ORIGINAL_ENV


def digest(path):
    return measure.sha(path)


def write_json(path, payload):
    path.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n")


def log_groups(path):
    events = path.read_text().split("compiled successfully")[:-1]
    return [dict(
        hits=sum(int(m[2]) for m in MEMO.findall(part)),
        traversals=sum(int(m[3]) for m in MEMO.findall(part)),
        refusals=sum(int(m[4]) for m in MEMO.findall(part)),
        skipped=part.count("lab-provisional-replay skipped"),
        positions=[line for line in part.splitlines() if line.startswith("watch-positions ")],
        memo=MEMO.findall(part),
    ) for part in events]


def compare_pdfs(left, right):
    a, b = base.qdf_hash(left), base.qdf_hash(right)
    assert a == b, (left, right, a, b)
    return a


def raster(case, pdf):
    case.mkdir(parents=True)
    subprocess.run(["pdftoppm", "-r", "72", "-png", str(pdf), str(case / "page")],
                   check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return [digest(file) for file in sorted(case.glob("page-*.png"))]


def check_directed():
    evidence = {}
    for kind, states in DIRECTED.items():
        case = WORK / "directed" / kind
        paths = {arm:case / arm for arm in ("both", "omit")}
        data = {arm:run(paths[arm], arm, states, trace=True) for arm in paths}
        logs = {arm:log_groups(path / "watch.log") for arm, path in paths.items()}
        base.EXES["both"] = PATHS["both"]
        base.fresh(case / "fresh", "both", states, True)
        entries = []
        for i, (name, _, _) in enumerate(states):
            control, change = (logs[arm][i] for arm in ("both", "omit"))
            expected, actual = (WORK / data[arm][i]["path"] for arm in ("both", "omit"))
            fresh = case / "fresh" / f"{name}.pdf"
            output = compare_pdfs(actual, expected)
            assert output == base.qdf_hash(fresh), (kind, name, "fresh")
            assert control["positions"] == change["positions"] and len(change["positions"]) == 1, (kind, name)
            assert not control["skipped"], (kind, name)
            if kind in ("indirect", "positions", "page-reference", "functional-numbering", "plain", "counter-state"):
                assert not change["skipped"], (kind, name, change)
            entries.append(dict(state=name, qdf_sha256=output, control=control, change=change))
        if kind == "regular":
            control, change = logs["both"][0], logs["omit"][0]
            assert control["traversals"] == 4, control
            hashes = {arm: raster(WORK / "raster" / arm, WORK / data[arm][0]["path"]) for arm in paths}
            assert hashes["both"] == hashes["omit"]
            entries[0]["raster_72dpi_sha256"] = hashes["omit"]
        if kind == "moves-links":
            assert logs["omit"][0]["positions"] != logs["omit"][1]["positions"]
            assert entries[1]["qdf_sha256"] != entries[2]["qdf_sha256"]
            def destinations(row):
                info = base.runner.describe(WORK / row["path"])
                return Counter(dest for page in info["links"] for link in page
                               if isinstance((dest := link.get("/Dest") or
                                              link.get("/A", {}).get("/D")), str))
            before, after = (destinations(row) for row in data["omit"][1:])
            assert before - after == {"u:s0": 1}
            assert after - before == {"u:s13": 1}
            entries[2]["link_change"] = {"removed": "u:s0", "added": "u:s13"}
        if kind == "indirect":
            leaf = WORK / data["omit"][1]["path"]
            text = subprocess.check_output(["pdftotext", leaf, "-"], text=True)
            assert "External omega." in text and "External alpha." not in text
            structure = subprocess.check_output(["qpdf", "--qdf", str(leaf), "-"])
            assert b"https://example.test/omega" in structure and b"https://example.test/alpha" not in structure
            assert entries[0]["qdf_sha256"] == entries[2]["qdf_sha256"] != entries[1]["qdf_sha256"]
            assert logs["omit"][0]["positions"] == logs["omit"][2]["positions"] != logs["omit"][1]["positions"]
        evidence[kind] = entries
        print("directed", kind, [(entry["state"], entry["control"]["traversals"],
                                  entry["change"]["traversals"], entry["change"]["skipped"])
                                 for entry in entries], flush=True)
    return evidence


def timing(rows):
    by_arm = {arm:{row["block"]:row for row in rows if row["arm"] == arm} for arm in ARMS}
    return dict(
        arms={arm:{key:base.bootstrap([r[key] / 1e6 for r in data.values()])
                   for key in ("cpu_ns", "wall_ns")} for arm, data in by_arm.items()},
        paired_cpu_ms={
            f"{a}-{b}":base.bootstrap([(by_arm[a][i]["cpu_ns"]-by_arm[b][i]["cpu_ns"])/1e6
                                        for i in by_arm[a]])
            for a,b in (("omit","both"), ("omit","vanilla"), ("omit","same-off"),
                        ("same-off","both"), ("both","vanilla"))
        },
    )


def main():
    os.sched_setaffinity(0, {2})
    for arm in ARMS:
        run(WORK / "warmup" / arm, arm, NOMINAL)
    census = {arm:run(WORK / "census" / arm, arm, NOMINAL, trace=True)
              for arm in ARMS if arm != "vanilla"}
    for arm in census:
        print("census", arm, log_groups(WORK / "census" / arm / "watch.log")[0]["memo"], flush=True)
    directed = check_directed()
    write_json(WORK / "validation.json", directed)
    orders = list(permutations(ARMS))
    Random(50500).shuffle(orders)
    rows = []
    with gzip.open(WORK / "nominal.json.gz", "wt") as output:
        for block, order in enumerate(orders):
            for arm in order:
                row, = run(WORK / "nominal" / f"{block:02d}" / arm, arm, NOMINAL)
                row["block"] = block
                rows.append(row)
                output.write(json.dumps(row) + "\n")
            print("nominal", block, list(order), flush=True)
    expected = PRIOR / "fresh" / "regular"
    hashes = Counter()
    for row in rows:
        pdf = WORK / row["path"]
        if row["arm"] == "vanilla":
            assert digest(pdf) == digest(expected / "vanilla" / "startup.pdf")
        else:
            assert compare_pdfs(pdf, expected / "tekt" / "startup.pdf")
        hashes[row["arm"]] += 1
    write_json(WORK / "timing.json", timing(rows) | {"verified_publications": hashes})


if __name__ == "__main__":
    main()
