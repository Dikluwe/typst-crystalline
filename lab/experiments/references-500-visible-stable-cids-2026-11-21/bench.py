"""Validate and time stable CIDs with and without compressed-page reuse."""
import importlib.util
import os
from pathlib import Path
import re

from pdf_check import compressed_pages, render, structure, text_layout

D = Path(__file__).resolve().parent
prior = D.parent / "references-500-visible-page-operators-2026-11-21" / "bench.py"
spec = importlib.util.spec_from_file_location("operator_bench", prior)
bench = importlib.util.module_from_spec(spec)
spec.loader.exec_module(bench)
cycle, base = bench.cycle, bench.base

COUNTS = re.compile(r"^pdf-stable-cids seeded_faces=(\d+) streams_reused=(\d+) serialized=(\d+)$", re.M)
environment = bench.environment
reference = False
gold = bench.gold
candidate_gold = {}


def variant_environment(tmp, arm, trace):
    env = environment(tmp, arm, trace)
    if arm == "candidate" and trace:
        env["TEKT_LAB_FONT_SCAN_CHECK"] = "1"
        if reference:
            env["TEKT_LAB_CID_REEMIT"] = "1"
    return env


def expected_pdf(arm, name):
    return candidate_gold[name] if arm == "candidate" else gold(arm, name)


def counters(log):
    values = COUNTS.findall(log.read_text())
    return [{key:int(value) for key,value in zip(("faces_seeded", "streams_reused", "serialized"), entry)}
            for entry in values]


def directed():
    global reference
    visible = bench.VISIBLE
    move = visible.replace(b"height:600pt", b"height:440pt")
    suite = {
        "original": bench.STATES,
        "round-trip": [bench.STATES[2], bench.STATES[3], ("restore", bench.STATES[2][1], None)],
        "links": [("open", visible, None), ("move", move, None),
                  ("reference", move.replace(b"See @s0.", b"See @s13.", 1), None)],
        "dependency": [("open", b'#include "middle.typ"\n' + visible, bench.opening.LEAF),
                       ("leaf-edit", b'#include "middle.typ"\n' + visible, bench.opening.LEAF_EDIT),
                       ("leaf-restore", b'#include "middle.typ"\n' + visible, bench.opening.LEAF)],
        "span-query": bench.opening.DIRECTED["positions"],
        "font-change": [("visible", visible, None),
                        ("font-change", b'#set text(font: "DejaVu Sans", size: 12pt)\n' + visible, None)],
        "resource": [("visible", visible, None),
                     ("rect", visible + b"#rect(width: 30pt, height: 10pt)\n", None)],
    }
    evidence = {}
    for kind, states in suite.items():
        case = bench.WORK / "directed" / kind
        candidate = bench.watch(case / "candidate", "candidate", states, trace=True)
        control = bench.watch(case / "hybrid", "hybrid", states, trace=True)
        reference = True
        try:
            reemitted = bench.watch(case / "reemitted", "candidate", states, trace=True)
        finally:
            reference = False
        positions = [bench.positions(case / arm / "watch.log") for arm in ("candidate", "hybrid", "reemitted")]
        assert positions[0] == positions[1] == positions[2], kind
        base.EXES["hybrid"] = bench.EXES["hybrid"]
        base.fresh(case / "fresh", "hybrid", states, True)
        evidence[kind] = []
        for i, (name, _, _) in enumerate(states):
            paths = [bench.WORK / run[i]["pdf"] for run in (candidate, control, reemitted)]
            paths.append(case / "fresh" / f"{name}.pdf")
            a, b, c, d = paths
            stable_hash, oracle = base.qdf_hash(a), base.qdf_hash(b)
            assert stable_hash == base.qdf_hash(c), (kind, name, "same-CID forced emission")
            assert oracle == base.qdf_hash(d), (kind, name, "fresh control")
            normalized = structure(a)
            assert normalized == structure(b), (kind, name, "navigation, tags or other PDF objects")
            text_bbox = text_layout(a)
            assert text_bbox == text_layout(b), (kind, name, "text or positions")
            evidence[kind].append(dict(state=name, stable_qdf=stable_hash, control_qdf=oracle,
                                       structure_sha256=normalized, text_bbox_sha256=text_bbox,
                                       positions=len(positions[0][i])))
            if kind == "original":
                candidate_gold[name] = stable_hash
        if kind == "original":
            check_original(case, states)
        if kind in ("font-change", "resource"):
            last = (case / "candidate" / "watch.log").read_text().split("compiled successfully")[-2]
            found = COUNTS.findall(last)
            assert not found or len(found) == 1 and found[0][1] == "0", (kind, found)
        print("directed", kind, len(states), flush=True)
    cycle.dump("directed.json", evidence)
    for arm in bench.ARMS:
        base.EXES[arm] = bench.EXES[arm]
        dest = bench.WORK / "fresh-verification" / arm
        base.fresh(dest, arm, bench.STATES, True)
        for name, _, _ in bench.STATES:
            assert cycle.key(arm, dest / f"{name}.pdf") == gold(arm, name), (arm, name, "fresh")
    print("fresh compilation verified for all engines", flush=True)


def check_original(case, states):
    a = case / "candidate"
    control = case / "hybrid"
    reference = case / "reemitted"
    names = [s[0] for s in states]
    start, previous, visible = names[0], names[2], names[3]
    normal, forced = counters(a / "watch.log"), counters(reference / "watch.log")
    trace = a / "watch.log"
    operators = bench.OPERATORS.findall(trace.read_text())
    layouts = [bench.opening.log_groups(case / arm / "watch.log")
               for arm in ("candidate", "hybrid", "reemitted")]
    assert all(groups[-1]["native_proved"] == 1 for groups in layouts)
    # The native layout runs before these memo probes. Compare the memo
    # results across arms without treating them as a global traversal census.
    memo = [groups[-1]["memo"] for groups in layouts]
    assert memo[0] == memo[1] == memo[2]
    assert len(normal) == len(forced) == len(operators) == 4
    print("candidate stream counts", normal, "operators", operators, flush=True)
    assert normal[-1]["faces_seeded"] > 0, normal
    assert normal[-1]["streams_reused"] == 21 and normal[-1]["serialized"] == 2, normal
    assert operators[-1] == ("21", "0", "[1, 2]"), operators
    assert forced[-1]["streams_reused"] == 0 and forced[-1]["serialized"] == 23
    assert all(record["faces_seeded"] == force["faces_seeded"] for record,force in zip(normal, forced))
    generations = {
        "opening": compressed_pages(a / f"{start}.pdf"),
        "previous": compressed_pages(a / f"{previous}.pdf"),
        "visible": compressed_pages(a / f"{visible}.pdf"),
        "control-previous": compressed_pages(control / f"{previous}.pdf"),
        "control-visible": compressed_pages(control / f"{visible}.pdf"),
    }
    assert len(generations["visible"]) == 23 and generations["opening"] == generations["previous"]
    reused = [i for i,(old,new) in enumerate(zip(generations["previous"], generations["visible"]), 1)
              if old == new]
    assert reused == list(range(3,24)), reused
    cycle.dump("streams.json", dict(trace=normal, forced_trace=forced, operator_trace=operators,
                                   reused_pages=reused, compressed_sha256=generations,
                                   visible_layout_memo=memo))
    directory = case / "rendered"
    directory.mkdir()
    pdfs = {"candidate":a / f"{visible}.pdf", "control":control / f"{visible}.pdf",
            "fresh":case / "fresh" / f"{visible}.pdf", "forced-reemission":reference / f"{visible}.pdf"}
    images = {name:render(pdf, directory, name) for name,pdf in pdfs.items()}
    assert len(images["candidate"]) == 23 and all(pages == images["candidate"] for pages in images.values())
    cycle.dump("renders.json", images)


if __name__ == "__main__":
    cycle.environment = variant_environment
    bench.gold = expected_pdf
    bench.directed = directed
    os.sched_setaffinity(0, {2})
    bench.main()
