"""Paired watch timings and directed PDF checks for each substitution arm."""
from collections import Counter
from importlib.util import module_from_spec, spec_from_file_location
from itertools import permutations
import json
import os
from pathlib import Path
from random import Random
from shutil import copy2, copytree
from statistics import mean
import subprocess
import sys
import tarfile
import tempfile
import time

D = Path(__file__).resolve().parent
WORK = Path(os.environ["TEKT_LAB_WORK"])
ROOT = D.parents[2]
RESULTS = ROOT / "lab/results/construction/diagnostics"
OUT = Path(os.environ["TEKT_CONTEXT_OUT"])
sys.path.insert(0, str(D.parent / "references-500-visible-eval-residual-2026-11-21"))
sys.path.insert(0, str(D.parent / "references-500-visible-stable-cids-2026-11-21"))
from pdf_check import render, structure, text_layout
from watch import cycle, driver, measure, golden, toggle, STATES

spec = spec_from_file_location("phases", D.parent / "references-500-visible-pre-pdf-2026-11-21/analyze.py")
phases = module_from_spec(spec)
spec.loader.exec_module(phases)
ARMS = ("control", "atomized", "retained", "vanilla")


def environment(case, arm, trace):
    env = cycle.environment(case, "vanilla" if arm == "vanilla" else "candidate", trace)
    if arm != "vanilla":
        env.update(TEKT_LAB_MEMO_PROTECT="1", TEKT_LAB_MEMO_BYTES=str(8 * 1024 * 1024))
    return env


def watch(case, arm, states, *, trace=False, period=0, keep=False):
    case.mkdir(parents=True)
    tmp, log_path, pdf = case / "tmp", case / "watch.log", case / "out.pdf"
    tmp.mkdir()
    cycle.base.runner.setup_sources(case, *states[0][1:])
    exe = str(WORK / arm)
    env = environment(tmp, arm, trace)
    if period:
        env.update(LD_PRELOAD=str(WORK / "profiler.so"), TEKT_CPU_TRACE=str(case / "samples.bin"),
                   TEKT_CPU_ACK=str(case / "samples.ack"), TEKT_CPU_PERIOD=str(period))
    command = [exe, "watch", str(case / "main.typ"), str(pdf), "--root", str(case), "--pdf-tagged=true"]
    rows = []
    with log_path.open("wb") as log:
        started = time.perf_counter_ns()
        child = driver.Child(command, case, env, 1, log)
        try:
            publication = None
            for i, (state, data, leaf) in enumerate(states):
                if period and i == len(states) - 1:
                    (case / "maps").write_text(Path(f"/proc/{child.process.pid}/maps").read_text())
                    toggle(child, case, 1)
                before = child.cpu() if i else 0
                if i:
                    started = time.perf_counter_ns()
                    cycle.base.runner.setup_sources(case, data, leaf)
                publication, published, _ = driver.await_pdf(child, pdf, publication)
                after = measure.settle(child)
                if period and i == len(states) - 1:
                    toggle(child, case, 2)
                engine = "vanilla" if arm == "vanilla" else "candidate"
                digest = cycle.cycle.key(engine, pdf)
                if keep:
                    (case / f"{state}.pdf").write_bytes(pdf.read_bytes())
                else:
                    assert digest == golden(engine, state), (case, state, digest)
                rows.append(dict(arm=arm, state=state, period_ns=period, cpu_ns=after-before,
                                 wall_ns=published-started, utc=measure.utc(), exe=exe,
                                 directory=str(case.relative_to(WORK)), pdf_hash=digest))
        finally:
            child.stop()
            child.close()
    assert log_path.read_text().count("compiled successfully") == len(states), case
    return rows


def fresh(case, arm, states):
    case.mkdir(parents=True)
    env = environment(case, arm, False)
    for name, data, leaf in states:
        cycle.base.runner.setup_sources(case, data, leaf)
        with (case / f"{name}.log").open("wb") as log:
            command = [str(WORK / arm), "compile", str(case / "main.typ"),
                       str(case / f"{name}.pdf"), "--root", str(case), "--pdf-tagged=true"]
            child = driver.Child(command, case, env, 1, log)
            try:
                child.reap()
                assert child.process.returncode == 0, (case, name)
            finally:
                child.close()


def directed():
    visible = STATES[-1][1]
    changed_height = visible.replace(b"height:600pt", b"height:440pt")
    assert changed_height != visible
    suites = {
        "original": STATES,
        "links": [("open", visible, None), ("move", changed_height, None),
                  ("link", changed_height.replace(b"See @s0.", b"See @s13.", 1), None)],
        "dependency": [("open", b'#include "middle.typ"\n' + visible, cycle.opening.LEAF),
                       ("edit", b'#include "middle.typ"\n' + visible, cycle.opening.LEAF_EDIT),
                       ("restore", b'#include "middle.typ"\n' + visible, cycle.opening.LEAF)],
        "span": cycle.opening.DIRECTED["positions"],
        "counter-state": cycle.opening.DIRECTED["counter-state"],
    }
    evidence = {}
    for suite, states in suites.items():
        case = WORK / "directed" / suite
        rows = {arm: watch(case / arm, arm, states, trace=True, keep=True) for arm in ARMS}
        locations = {arm: cycle.positions(case / arm / "watch.log") for arm in ARMS[:-1]}
        assert locations["control"] == locations["atomized"] == locations["retained"], suite
        for arm in ("control", "retained", "vanilla"):
            fresh(case / ("fresh-" + arm), arm, states)
        traversals = {arm: cycle.opening.log_groups(case / arm / "watch.log") for arm in ARMS[:-1]}
        evidence[suite] = []
        for i, (name, _, _) in enumerate(states):
            paths = {arm: case / arm / f"{name}.pdf" for arm in ARMS}
            layout = text_layout(paths["control"])
            objects = structure(paths["control"])
            for arm in ("atomized", "retained"):
                assert rows[arm][i]["pdf_hash"] == rows["control"][i]["pdf_hash"], (suite, arm, name)
                assert text_layout(paths[arm]) == layout, (suite, arm, name, "word positions")
                assert structure(paths[arm]) == objects, (suite, arm, name, "destinations/tags")
            for arm in ("control", "retained"):
                new = case / ("fresh-" + arm) / f"{name}.pdf"
                assert text_layout(new) == layout, (suite, arm, name, "fresh positions")
                assert structure(new) == objects, (suite, arm, name, "fresh links")
            vanilla_fresh = case / "fresh-vanilla" / f"{name}.pdf"
            assert measure.sha(vanilla_fresh) == rows["vanilla"][i]["pdf_hash"], (suite, name)
            rasters = 0
            if suite == "original" and name == "visible":
                expected = render(paths["control"], case, "control-page")
                for arm in ("atomized", "retained"):
                    assert render(paths[arm], case, arm + "-page") == expected, arm
                rasters = len(expected)
            evidence[suite].append(dict(state=name, qdf=rows["control"][i]["pdf_hash"],
                vanilla_sha256=rows["vanilla"][i]["pdf_hash"], text_bbox=layout, structure=objects,
                locations=len(locations["control"][i]), rasters=rasters,
                memo={arm:{k:groups[i][k] for k in ("traversals","hits","skipped")}
                      for arm,groups in traversals.items()}))
        if suite == "links":
            for arm in ARMS[:-1]:
                before = cycle.destination_links(case / arm / "move.pdf")
                after = cycle.destination_links(case / arm / "link.pdf")
                assert before - after == {"u:s0": 1}, (suite, arm, before - after)
                assert after - before == {"u:s13": 1}, (suite, arm, after - before)
            evidence[suite][-1]["link_change"] = {"removed":"u:s0","added":"u:s13"}
        if suite == "span":
            assert locations["control"][0] != locations["control"][1]
        if suite == "dependency":
            before, edit, restore = evidence[suite]
            # Stable-CID history can keep a new glyph after source restoration.
            # Compare text and PDF structure across publications, plus the
            # full QDF against the paired Tekt control within each publication.
            for key in ("text_bbox", "structure"):
                assert before[key] != edit[key] and before[key] == restore[key], (suite, key)
            for arm in ARMS:
                pdf = case / arm / "edit.pdf"
                text = subprocess.check_output(["pdftotext", str(pdf), "-"], text=True)
                qdf = subprocess.check_output(["qpdf", "--qdf", str(pdf), "-"])
                assert "External omega." in text and "External alpha." not in text, arm
                assert b"https://example.test/omega" in qdf and b"https://example.test/alpha" not in qdf, arm
        (WORK / "directed.json").write_text(json.dumps(evidence, indent=2) + "\n")
        print("directed", suite, len(states), flush=True)
    with tarfile.open(OUT / "directed-logs.tar.gz", "w:gz") as archive:
        for log in sorted((WORK / "directed").rglob("*.log")):
            archive.add(log, arcname=str(log.relative_to(WORK)))


def summarize(rows):
    bootstrap = phases.base.stats
    nominal = {arm: {} for arm in ARMS}
    profile = {arm: {} for arm in ARMS}
    table = {arm: phases.base.symbols(WORK / f"{arm}.symbols") for arm in ARMS[:-1]}
    table["vanilla"] = phases.base.symbols(RESULTS / "references-500-visible-page-operator-cost-2026-11-21/vanilla-symbols-twin.txt")
    for row in rows:
        block, arm = row["block"], row["arm"]
        if not row["period_ns"]:
            nominal[arm].setdefault(block, {})[row["state"]] = row["cpu_ns"] / 1e6
        elif row["state"] == STATES[-1][0]:
            class Scratch:
                def extractfile(self, name):
                    return (WORK / name).open("rb")
            stacks, detail = phases.base.stacks(Scratch(), row, table)
            counts = Counter()
            for stack, n in stacks.items():
                if any("::substitute_context_blocks" in f or "::substitution::" in f for f in stack):
                    group = "substitution"
                else:
                    group = phases.category(arm, stack)
                counts[group] += n
            scale = detail["main_ns"] / detail["samples"] / 1e6
            profile[arm][block] = dict(ms={key:n*scale for key,n in counts.items()}, **detail)
    out = {"nominal": {}, "profile": profile}
    for key in [s[0] for s in STATES] + ["cycle"]:
        measurements = {arm: {b:sum(res.values()) if key == "cycle" else res[key]
                               for b,res in cases.items()} for arm,cases in nominal.items()}
        out["nominal"][key] = dict(absolute={arm:bootstrap(list(items.values())) for arm,items in measurements.items()},
            paired={a+"-"+b:bootstrap([v-measurements[b][block] for block,v in measurements[a].items()])
                    for a,b in (("atomized","control"),("retained","atomized"),("retained","control"),("retained","vanilla"))})
    out["substitution"] = {arm: dict(mean=mean(r["ms"].get("substitution",0) for r in cases.values()),
         **bootstrap([r["ms"].get("substitution",0) for r in cases.values()])) for arm,cases in profile.items()}
    out["substitution_paired"] = {a+"-"+b:bootstrap([r["ms"].get("substitution",0)-profile[b][k]["ms"].get("substitution",0)
        for k,r in profile[a].items()]) for a,b in (("atomized","control"),("retained","atomized"),("retained","control"))}
    (WORK / "phases.json").write_text(json.dumps(out, indent=2) + "\n")


def main():
    os.sched_setaffinity(0, {2})
    for fixtures in ("parity/corpus", "corpus/legacy-sections"):
        copytree(ROOT / "lab" / fixtures, WORK / "src/lab" / fixtures)
    directed()
    rng = Random(78103)
    orders = list(permutations(ARMS))
    # Every ordering occurs once. Spread each arm evenly across positions in
    # both the nominal and profiled groups.
    blocks = [(0, order) for order in orders]
    blocks += [(period, ARMS[i:] + ARMS[:i]) for period in (347000, 937000)
               for i in range(4) for _ in range(3)]
    rng.shuffle(blocks)
    rows = []
    with (WORK / "measurements.jsonl").open("w") as output:
        for block, (period, order) in enumerate(blocks):
            for arm in order:
                case = WORK / "cases" / f"{block:03d}-{arm}"
                for row in watch(case, arm, STATES, period=period):
                    record = dict(row, block=block)
                    rows.append(record)
                    print(json.dumps(record), file=output, flush=True)
            print(f"{block+1}/{len(blocks)} period={period}", flush=True)
    summarize(rows)


if __name__ == "__main__":
    try:
        main()
    except Exception:
        # Retain build products only for a follow-up run if validation stops.
        recovery = Path(tempfile.mkdtemp(prefix="tekt-substitution-recovery.", dir="/tmp"))
        for arm in ("atomized", "retained"):
            copy2(WORK / arm, recovery / arm)
        with tarfile.open(OUT / "failure-logs.tar.gz", "w:gz") as archive:
            for log in sorted((WORK / "directed").rglob("*.log")):
                archive.add(log, arcname=str(log.relative_to(WORK)))
        print(f"Recovery ELFs: {recovery}", file=sys.stderr, flush=True)
        raise
