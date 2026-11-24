"""Compare the verified native/checkpoint hybrid, checkpoint, native and pinned vanilla in RAM."""
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
import time

D = Path(__file__).resolve().parent
WORK = Path(os.environ["TEKT_LAB_WORK"])
PRIOR = D.parent / "references-500-pdf-export-reuse-2026-11-20"
spec = importlib.util.spec_from_file_location("control_bench", PRIOR / "bench.py")
base = importlib.util.module_from_spec(spec)
spec.loader.exec_module(base)
measure = base.measure
BASE = base.REGULAR[0][1]
ARMS = ("both", "hybrid", "checkpoint", "vanilla")
PATHS = {arm: WORK / exe for arm, exe in zip(ARMS, ("control", "variant", "checkpoint", "vanilla"))}
ORIGINAL_ENV = base.driver.environment
MEMO = re.compile(r"^watch-memo gen=(\d+) replay=(true|false) hits=(\d+) builds=(\d+) refusals=(\d+)", re.M)

NOMINAL = [base.REGULAR[0]]
DYNAMIC = b'#include "middle.typ"\n' + BASE
LEAF = b'= Original External <external>\n#link("https://example.test/alpha")[External alpha.]\n'
LEAF_EDIT = b'#pagebreak()\n= Changed External <external>\n#link("https://example.test/omega")[External omega.]\n'
NUMBERED = BASE.replace(b"#outline()", b'#set page(numbering: "1 / 1")\n#counter(page).update(7)\n#outline()', 1)
NUMBERED = NUMBERED.replace(b"See @s0.", b'See #ref(<s0>, form: "page").', 1)
FUNCTIONAL = BASE.replace(b"#outline()", b'#set page(numbering: (n, ..rest) => [NUM #n])\n#outline()', 1)
ALTERNATE = b"""#set page(width:240pt,height:280pt)
#set heading(numbering:"1.1")
#outline(title: none)
= Earlier <first>
See @later.
#for n in range(35) [Earlier paragraph #n on a narrow sheet.

]
== Later <later>
See @first.
#for n in range(30) [Later paragraph #n on a narrow sheet.

]
"""
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
    "alternate": [("opening", ALTERNATE, None),
                  ("move", ALTERNATE.replace(b"height:280pt", b"height:320pt"), None)],
    "long-outline": [("opening", BASE.replace(b"height:600pt", b"height:240pt"), None)],
    "renamed": [("opening", BASE.replace(b"= Section ", b"= Chapter "), None)],
    "regular-count": [("opening", BASE.replace(b"range(25)", b"range(26)"), None)],
    "counter-state": [("opening", BASE.replace(
        b"See @s0.", b'#counter("lab").update(7) #context [Counter: #counter("lab").get().first()].', 1), None)],
}


def environment(arm, tmp):
    env = ORIGINAL_ENV(tmp)
    if arm != "vanilla":
        env.update(TEKT_LAB_SKIP_PROVISIONAL_REPLAY="1",
                   TEKT_LAB_SOURCE_ONLY_FIRST_INDEX="1")
    if arm != "vanilla":
        env["TEKT_LAB_OUTLINE_ORDINALS"] = {"hybrid": "hybrid", "checkpoint": "retain", "both": "metrics"}[arm]
    return env


def run(directory, arm, states, trace=False):
    source_arm = "vanilla" if arm == "vanilla" else "both"
    base.EXES[source_arm] = PATHS[arm]
    base.driver.environment = lambda tmp: environment(arm, tmp)
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
        skipped=part.count("lab-retain accepted") + part.count("lab-hybrid native"),
        native_proved=part.count("lab-hybrid native"),
        checkpoint_attempted=part.count("lab-hybrid checkpoint attempt"),
        retained_failures=[line for line in part.splitlines() if "lab-retain refused:" in line],
        native_steps=[line for line in part.splitlines() if line.startswith("lab-outline-native-step ")],
        seed_events=[line for line in part.splitlines() if line.startswith("lab-outline-predict ")],
        source_only=part.count("lab-source-only-first-index"),
        replay_skipped=part.count("lab-provisional-replay skipped"),
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
        paths = {arm:case / arm for arm in ("both", "hybrid")}
        data = {arm:run(paths[arm], arm, states, trace=True) for arm in paths}
        logs = {arm:log_groups(path / "watch.log") for arm, path in paths.items()}
        base.EXES["both"] = PATHS["both"]
        base.fresh(case / "fresh", "both", states, True)
        entries = []
        for i, (name, _, _) in enumerate(states):
            control, change = (logs[arm][i] for arm in ("both", "hybrid"))
            expected, actual = (WORK / data[arm][i]["path"] for arm in ("both", "hybrid"))
            fresh = case / "fresh" / f"{name}.pdf"
            output = compare_pdfs(actual, expected)
            assert output == base.qdf_hash(fresh), (kind, name, "fresh")
            assert control["positions"] == change["positions"] and len(change["positions"]) == 1, (kind, name)
            if kind in ("indirect", "positions", "page-reference", "functional-numbering", "plain", "counter-state"):
                assert not change["skipped"], (kind, name, change)
            if kind == "alternate":
                assert not change["seed_events"] and change["checkpoint_attempted"] == 1, (name, change)
                assert change["skipped"] == 1 and not change["retained_failures"], (name, change)
            entries.append(dict(state=name, qdf_sha256=output, control=control, change=change))
        if kind == "regular":
            control, change = logs["both"][0], logs["hybrid"][0]
            assert control["traversals"] == 1, control
            hashes = {arm: raster(WORK / "raster" / arm, WORK / data[arm][0]["path"]) for arm in paths}
            assert hashes["both"] == hashes["hybrid"]
            entries[0]["raster_72dpi_sha256"] = hashes["hybrid"]
        if kind == "moves-links":
            assert logs["hybrid"][0]["positions"] != logs["hybrid"][1]["positions"]
            assert entries[1]["qdf_sha256"] != entries[2]["qdf_sha256"]
            def destinations(row):
                info = base.runner.describe(WORK / row["path"])
                return Counter(dest for page in info["links"] for link in page
                               if isinstance((dest := link.get("/Dest") or
                                              link.get("/A", {}).get("/D")), str))
            before, after = (destinations(row) for row in data["hybrid"][1:])
            assert before - after == {"u:s0": 1}
            assert after - before == {"u:s13": 1}
            entries[2]["link_change"] = {"removed": "u:s0", "added": "u:s13"}
        if kind == "indirect":
            leaf = WORK / data["hybrid"][1]["path"]
            text = subprocess.check_output(["pdftotext", leaf, "-"], text=True)
            assert "External omega." in text and "External alpha." not in text
            structure = subprocess.check_output(["qpdf", "--qdf", str(leaf), "-"])
            assert b"https://example.test/omega" in structure and b"https://example.test/alpha" not in structure
            assert entries[0]["qdf_sha256"] == entries[2]["qdf_sha256"] != entries[1]["qdf_sha256"]
            assert logs["hybrid"][0]["positions"] == logs["hybrid"][2]["positions"] != logs["hybrid"][1]["positions"]
        evidence[kind] = entries
        print("directed", kind, [(entry["state"], entry["control"]["traversals"],
                                  entry["change"]["traversals"], entry["change"]["skipped"])
                                 for entry in entries], flush=True)
    return evidence




def missing_reference():
    broken = BASE.replace(b"See @s0.", b"See @missing.", 1)
    result = {}
    for arm in ("both", "hybrid"):
        case = WORK / "directed" / "missing-ref" / arm
        case.mkdir(parents=True)
        source, pdf, log = [case / name for name in ("main.typ", "out.pdf", "watch.log")]
        source.write_bytes(BASE)
        (case / "broken.typ").write_bytes(broken)
        env = environment(arm, case)
        env.update(TEKT_LAB_WATCH_RETAIN="1", TEKT_LAB_WATCH_LAZY="1",
                   TEKT_LAB_WATCH_INERT="1", TEKT_LAB_PDF_REUSE="both")
        cmd = [str(PATHS[arm]), "watch", "main.typ", "out.pdf",
               "--root", str(case), "--pdf-tagged=true"]
        with log.open("wb") as output:
            child = base.driver.Child(cmd, case, env, 1, output)
            try:
                publication, _, _ = base.driver.await_pdf(child, pdf, None)
                measure.settle(child)
                previous_pdf = (digest(pdf), base.qdf_hash(pdf))
                start = len(log.read_text())
                base.runner.write_source(source, broken)
                deadline = time.monotonic() + 10
                while True:
                    text = log.read_text()[start:]
                    if "compiled with errors" in text and "error:" in text:
                        measure.settle(child)
                        text = log.read_text()[start:]
                        break
                    if child.ready() or time.monotonic() >= deadline:
                        raise RuntimeError((arm, text))
                    time.sleep(0.005)
                assert "label `<missing>` does not exist" in text
                assert digest(pdf) == previous_pdf[0]
                base.runner.write_source(source, BASE)
                base.driver.await_pdf(child, pdf, publication)
                measure.settle(child)
                assert base.qdf_hash(pdf) == previous_pdf[1]
                result[arm] = re.sub(r"^\[\d\d:\d\d:\d\d\] .*\n", "", text, flags=re.M)
            finally:
                child.stop()
                child.close()
    assert result["both"] == result["hybrid"], result
    write_json(WORK / "missing-reference.json", result)

POSITION = re.compile(r"Location\((\d+)\): Position \{ page: (\d+), "
                      r"point: Point \{ x: Pt\(([^)]*)\), y: Pt\(([^)]*)\) \} \}")


def positions_from_log(path):
    lines = path.read_text().splitlines()
    announced, = (int(line.split()[1]) for line in lines if line.startswith("watch-positions "))
    dumped, = (line for line in lines if line.startswith("lab-sealed-positions "))
    positions = {key: (int(page), float(x).hex(), float(y).hex())
                 for key, page, x, y in POSITION.findall(dumped)}
    assert len(positions) == announced, (path, announced, len(positions))
    return positions


def verify_all_positions():
    baseline, candidate = (positions_from_log(WORK / "census" / arm / "watch.log")
                           for arm in ("checkpoint", "hybrid"))
    assert baseline == candidate
    write_json(WORK / "sealed-positions.json", dict(
        count=len(candidate), same_location_keys_and_coordinates=baseline == candidate,
        baseline=baseline, candidate=candidate))

def timing(rows):
    by_arm = {arm:{row["block"]:row for row in rows if row["arm"] == arm} for arm in ARMS}
    return dict(
        arms={arm:{key:base.bootstrap([r[key] / 1e6 for r in data.values()])
                   for key in ("cpu_ns", "wall_ns")} for arm, data in by_arm.items()},
        paired_cpu_ms={
            f"{a}-{b}":base.bootstrap([(by_arm[a][i]["cpu_ns"]-by_arm[b][i]["cpu_ns"])/1e6
                                        for i in by_arm[a]])
            for a,b in (("hybrid","both"), ("hybrid","vanilla"), ("hybrid","checkpoint"),
                        ("checkpoint","both"), ("both","vanilla"))
        },
    )


HEIGHTS = (440, 520, 600, 680)
COUNTS = (20, 24, 25, 26)
GRID_ARMS = ("both", "hybrid", "checkpoint", "vanilla")


def grid_states(height, count):
    source = BASE.replace(b"height:600pt", f"height:{height}pt".encode())
    source = source.replace(b"range(25)", f"range({count})".encode())
    return [("startup", source, None)]


def grid():
    checked = {}
    rng = Random(42142)
    shapes = [(h, p) for h in HEIGHTS for p in COUNTS]
    rng.shuffle(shapes)
    with gzip.open(WORK / "grid-nominal.json.gz", "wt") as output:
        for height, count in shapes:
            key = f"h{height}-p{count}"
            case = WORK / "grid" / key
            states = grid_states(height, count)
            census = {arm:run(case / "trace" / arm, arm, states, trace=arm != "vanilla")[0]
                      for arm in GRID_ARMS}
            tekt = WORK / census["both"]["path"]
            variant = WORK / census["hybrid"]["path"]
            checkpoint = WORK / census["checkpoint"]["path"]
            vanilla = WORK / census["vanilla"]["path"]
            base.EXES.update(both=PATHS["both"], vanilla=PATHS["vanilla"])
            base.fresh(case / "tekt-fresh", "both", states, True)
            base.fresh(case / "vanilla-fresh", "vanilla", states, True)
            expected_qdf = base.qdf_hash(case / "tekt-fresh" / "startup.pdf")
            assert compare_pdfs(tekt, variant) == expected_qdf, key
            assert base.qdf_hash(checkpoint) == expected_qdf, key
            assert digest(vanilla) == digest(case / "vanilla-fresh" / "startup.pdf"), key
            trace = {arm:log_groups(case / "trace" / arm / "watch.log")[0]
                     for arm in ("both", "hybrid", "checkpoint")}
            assert trace["both"]["positions"] == trace["hybrid"]["positions"] == trace["checkpoint"]["positions"], key
            assert len(trace["hybrid"]["positions"]) == 1, key
            assert trace["hybrid"]["skipped"] == trace["checkpoint"]["skipped"] == 1, (key, trace)
            native_won = trace["both"]["traversals"] == 1
            assert trace["hybrid"]["native_proved"] == int(native_won), (key, trace)
            assert trace["hybrid"]["checkpoint_attempted"] == int(not native_won), (key, trace)

            # Each order is paired with its reverse to balance pairwise precedence.
            orders = rng.sample(list(permutations(GRID_ARMS)), 3)
            orders += [order[::-1] for order in orders]
            rng.shuffle(orders)
            rows = {arm:[] for arm in GRID_ARMS}
            for block, order in enumerate(orders):
                for arm in order:
                    row, = run(case / "nominal" / f"block-{block:02d}" / arm, arm, states)
                    row.update(block=block, height=height, paragraphs_per_heading=count)
                    rows[arm].append(row["cpu_ns"] / 1e6)
                    output.write(json.dumps(row) + "\n")
            output.flush()
            checked[key] = dict(height=height, count=count, trace=trace, tekt_qdf_sha256=expected_qdf,
                                cpu_ms={arm:median(values) for arm, values in rows.items()},
                                paired_cpu_ms={arm:base.bootstrap([a-b for a,b in zip(rows["hybrid"],rows[arm])])
                                               for arm in ("both","checkpoint","vanilla")})
            write_json(WORK / "grid.json", checked)
            print("grid", key, "native" if native_won else "checkpoint",
                  {arm:trace[arm]["traversals"] for arm in trace}, checked[key]["cpu_ms"], flush=True)


def main():
    os.sched_setaffinity(0, {2})
    if not os.environ.get("TEKT_CHECKPOINT_SMOKE"):
        for arm in ARMS:
            run(WORK / "warmup" / arm, arm, NOMINAL)
    old_fresh = PRIOR / "fresh" / "regular" / "tekt" / "startup.pdf"
    census = {arm:run(WORK / "census" / arm, arm, NOMINAL, trace=True)
              for arm in ("both", "hybrid", "checkpoint")}
    for arm in census:
        observed = log_groups(WORK / "census" / arm / "watch.log")[0]
        print("census", arm, observed["memo"], observed["skipped"], observed["retained_failures"],
              observed["seed_events"], flush=True)
    for arm, rows in census.items():
        row, = rows
        compare_pdfs(WORK / row["path"], old_fresh)
    verify_all_positions()
    assert log_groups(WORK / "census/hybrid/watch.log")[0]["native_proved"] == 1
    if os.environ.get("TEKT_CHECKPOINT_SMOKE"):
        return
    directed = check_directed()
    write_json(WORK / "validation.json", directed)
    missing_reference()
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
    grid()


if __name__ == "__main__":
    main()
