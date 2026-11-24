"""Matched watch runs and fresh comparisons for the lab-only retention probe."""
from collections import Counter
import gzip
import hashlib
import importlib.util
import json
import os
from pathlib import Path
from random import Random
from statistics import median
import subprocess
import sys
import time

D = Path(__file__).resolve().parent
ROOT = D.parents[2]
DIAG = ROOT / "lab/results/construction/diagnostics"
os.environ.setdefault("TEKT_EXE", str(D / "control.elf"))
sys.path.insert(0, str(DIAG / "references-500-vanilla-after-shared-replay-2026-11-20"))
spec = importlib.util.spec_from_file_location("prior", Path(sys.path[0]) / "bench.py")
prior = importlib.util.module_from_spec(spec)
spec.loader.exec_module(prior)
driver = prior.compare.DRIVER
measure = prior.measure
describe = prior.compare.describe
write_json = measure.write_json
EXES = {"control": D / "control.elf", "off": D / "candidate.elf", "retain": D / "candidate.elf"}
BASE = (D / "references_500.typ").read_bytes()
STATES = prior.STATES
assert BASE == prior.BASE
INCLUDE = b'#include "middle.typ"\n'
LEAF_FIRST = b"= Original External <external>\nExternal alpha.\n"
LEAF_EDIT = b"#pagebreak()\n= Changed External <external>\nExternal omega.\n"
MIDDLE = b'#include "leaf.typ"\n'
DEPENDENCY = [
    ("dep-start", INCLUDE + STATES["startup"], LEAF_FIRST),
    ("dep-comment", INCLUDE + STATES["comment"], LEAF_FIRST),
    ("dep-leaf-edit", INCLUDE + STATES["comment"], LEAF_EDIT),
    ("dep-leaf-restore", INCLUDE + STATES["comment"], LEAF_FIRST),
]
REGULAR = [(name, main, None) for name, main in STATES.items()]


def setup_sources(case, main, leaf):
    source = case / "main.typ"
    if not source.exists() or source.read_bytes() != main:
        source.write_bytes(main)
    if leaf is not None:
        path = case / "leaf.typ"
        if not path.exists() or path.read_bytes() != leaf:
            path.write_bytes(leaf)
        middle = case / "middle.typ"
        if not middle.exists():
            middle.write_bytes(MIDDLE)


def command(case, arm, mode, dest, trace):
    temporary = case / "tmp"
    temporary.mkdir(exist_ok=True)
    env = driver.environment(temporary)
    if arm == "retain":
        env["TEKT_LAB_WATCH_RETAIN"] = "1"
    if trace:
        env["TEKT_LAB_WATCH_TRACE"] = "1"
    return [str(EXES[arm]), mode, str(case / "main.typ"), str(dest),
            "--root", str(case), "--pdf-tagged=true"], env


def run_watch(case, arm, states, trace=False):
    case.mkdir(parents=True)
    setup_sources(case, *states[0][1:])
    dest = case / "out.pdf"
    args, env = command(case, arm, "watch", dest, trace)
    rows = []
    with (case / "watch.log").open("wb") as log:
        started = time.perf_counter_ns()
        child = driver.Child(args, case, env, 1, log)
        try:
            previous = None
            for index, (name, main, leaf) in enumerate(states):
                before_cpu = 0
                if index:
                    before_cpu = child.cpu()
                    started = time.perf_counter_ns()
                    setup_sources(case, main, leaf)
                previous, published, publish_cpu = driver.await_pdf(child, dest, previous)
                settled = measure.settle(child)
                snapshot = case / (name + ".pdf")
                snapshot.write_bytes(dest.read_bytes())
                rows.append(dict(arm=arm, state=name, utc=measure.utc(),
                                 cpu_ns=settled - before_cpu, wall_ns=published - started,
                                 publish_cpu_ns=publish_cpu - before_cpu,
                                 path=str(snapshot.relative_to(D)),
                                 **driver.read_memory(child.process.pid)))
        finally:
            child.stop()
            child.close()
    return rows


def run_fresh(case, arm, states, trace=False):
    case.mkdir(parents=True)
    for name, main, leaf in states:
        setup_sources(case, main, leaf)
        dest = case / (name + "-fresh.pdf")
        args, env = command(case, arm, "compile", dest, trace)
        with (case / (name + "-fresh.log")).open("wb") as log:
            child = driver.Child(args, case, env, 1, log)
            try:
                child.reap()
                assert child.process.returncode == 0, log.name
            finally:
                child.close()


def positions(log):
    return [line.removeprefix("watch-positions ") for line in log.read_text().splitlines()
            if line.startswith("watch-positions ")]


def variants():
    return {arm: measure.sha(path) for arm, path in EXES.items()}


def verify(runs, nominal):
    checks = []
    fingerprint = {}
    described = {}

    def inspect(path):
        digest = measure.sha(path)
        if digest not in described:
            described[digest] = describe(path)
        return described[digest]

    for kind, states in (("regular", REGULAR), ("dependency", DEPENDENCY)):
        for arm in EXES:
            case = runs / "validation" / kind / arm
            rows = run_watch(case, arm, states, trace=arm != "control")
            run_fresh(case, arm, states, trace=arm != "control")
            trace_positions = positions(case / "watch.log")
            for index, row in enumerate(rows):
                state = row["state"]
                actual = inspect(D / row["path"])
                fresh_pdf = case / (state + "-fresh.pdf")
                fresh = inspect(fresh_pdf)
                key = kind, state
                if arm == "control":
                    fingerprint[key] = fresh
                assert actual == fresh == fingerprint[key], (kind, arm, state)
                pos = None
                if arm != "control":
                    expected_positions, = positions(case / (state + "-fresh.log"))
                    pos = trace_positions[index]
                    assert pos == expected_positions, (kind, arm, state, pos, expected_positions)
                navigation = None
                if arm == "retain" and kind == "regular":
                    navigation = prior.navigation(fresh_pdf, fresh, state == "visible")
                    wrong = navigation["wrong_pages"]
                    assert len(wrong["toc_links"]) == len(wrong["bookmarks"]) == 19
                    assert not wrong["references"] and not wrong["toc_printed"]
                checks.append(dict(kind=kind, arm=arm, state=state, observables=list(actual),
                                   positions=pos, navigation=navigation))
            log = (case / "watch.log").read_text()
            if arm == "retain" and kind == "regular":
                assert log.count("watch-retain hit") == 2, log
                assert "watch-retain miss sealed=true fonts=true warnings=0" in log, log
            if arm == "retain" and kind == "dependency":
                assert log.count("watch-retain miss sealed=false") == len(states), log
                text = subprocess.check_output(
                    ["pdftotext", str(case / "dep-leaf-edit.pdf"), "-"], text=True)
                assert "External omega." in text and "External alpha." not in text
                assert fingerprint[kind, "dep-comment"] != fingerprint[kind, "dep-leaf-edit"]
                assert fingerprint[kind, "dep-comment"] == fingerprint[kind, "dep-leaf-restore"]

    for row in nominal:
        expected = fingerprint["regular", row["state"]]
        actual = inspect(D / row["path"])
        assert actual == expected, row
    return dict(utc=measure.utc(), nominal_verified=len(nominal), checks=checks)


def timing(rows, n):
    summary = {}
    rng = Random(500)
    for state in STATES:
        per_arm = {arm: [r for r in rows if r["arm"] == arm and r["state"] == state]
                   for arm in EXES}
        assert all(len(entries) == n for entries in per_arm.values())
        out = summary[state] = {}
        for key in ("cpu_ns", "wall_ns"):
            result = out[key] = {"median_ms": {
                arm: median(row[key] for row in entries) / 1e6 for arm, entries in per_arm.items()
            }}
            for arm in ("off", "retain"):
                ratios = [a[key] / c[key] for a, c in zip(per_arm[arm], per_arm["control"])]
                intervals = sorted(median(rng.choices(ratios, k=n)) for _ in range(10000))
                result[arm + "_over_control"] = dict(median=median(ratios),
                    ci95=[intervals[250], intervals[9749]])
    return dict(by_state=summary, max_hwm_mib={
        arm: max(r["hwm_bytes"] for r in rows if r["arm"] == arm) / 2**20 for arm in EXES
    })


def run():
    os.sched_setaffinity(0, {2})
    blocks = int(os.environ.get("BLOCKS", "12"))
    assert (D / "control-before.sha256").read_bytes() == (D / "control-input-manifest.sha256").read_bytes()
    hashes = variants()
    for arm in ("control", "candidate"):
        exe_arm = "control" if arm == "control" else "retain"
        assert hashes[exe_arm] == (D / (arm + ".sha256")).read_text().split()[0]
    runs = D / "runs"
    runs.mkdir()
    write_json(D / "receipt.json", dict(utc=measure.utc(), head=(D / "head.txt").read_text().strip(),
        elves=hashes, blocks=blocks, cpus=dict(child=1, controller=2),
        provenance={name: measure.sha(D / name) for name in (
            "control-diffstat.txt", "control-input-manifest.sha256", "candidate-input-manifest.sha256",
            "prototype.py", "watch_probe.inc.rs", "build-control.sh", "build-candidate.sh", "bench.py")},
        tooling={name: subprocess.check_output(args, text=True, stderr=subprocess.STDOUT).splitlines()[0]
                 for name, args in {"qpdf": ["qpdf", "--version"], "mutool": ["mutool", "-v"],
                                    "poppler": ["pdftotext", "-v"]}.items()},
        inputs={name: dict(main=hashlib.sha256(main).hexdigest(),
                           middle=hashlib.sha256(MIDDLE).hexdigest() if leaf else None,
                           leaf=hashlib.sha256(leaf).hexdigest() if leaf else None)
                for name, main, leaf in REGULAR + DEPENDENCY}))
    rows = []
    orders = (("control", "off", "retain"), ("off", "retain", "control"), ("retain", "control", "off"),
              ("retain", "off", "control"), ("control", "retain", "off"), ("off", "control", "retain"))
    for block in range(blocks):
        for arm in orders[block % len(orders)]:
            rows.extend(dict(block=block, **row) for row in run_watch(runs / str(block) / arm, arm, REGULAR))
        print("block", block, flush=True)
    with gzip.open(D / "nominal.json.gz", "wt") as out:
        json.dump(rows, out, indent=2)
    write_json(D / "timing.json", timing(rows, blocks))
    write_json(D / "verification.json", verify(runs, rows))
    assert hashes == variants()


if __name__ == "__main__":
    run()
