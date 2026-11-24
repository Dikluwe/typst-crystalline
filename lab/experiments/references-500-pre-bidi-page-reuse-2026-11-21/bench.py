"""Run the previous watch protocol and observe admission before BiDi."""
from collections import Counter
from importlib.util import module_from_spec, spec_from_file_location
from itertools import permutations
import json
import os
from pathlib import Path
from random import Random
import re
from shutil import copy2
from statistics import mean
import subprocess
import tarfile

D = Path(__file__).resolve().parent
spec = spec_from_file_location("link_bench", D.parent / "references-500-link-redundant-vectors-2026-11-21/bench.py")
previous = module_from_spec(spec)
spec.loader.exec_module(previous)
base = previous.base
WORK, OUT, ARMS, STATES = previous.WORK, previous.OUT, previous.ARMS, previous.STATES
REUSE = re.compile(r"^pre-bidi-pages total=(\d+) checked=(\d+) reused=(\d+) "
                   r"(?:ordinals=(\[.*\])|reason=(\w+))$", re.M)


def focused():
    lib = WORK / "target/release/deps"
    core = max(lib.glob("libtypst_core-*.rlib"), key=lambda p: p.stat().st_mtime_ns)
    source = WORK / "proof_test.rs"
    snapshot = WORK / "src/03_infra/src/pipeline/paged/pre_bidi.rs"
    source.write_text(f'mod pipeline {{ mod paged {{ mod pre_bidi {{ include!("{snapshot}"); }} }} }}\n')
    exe = WORK / "proof-test"
    command = ["rustc", "--test", "--edition", "2024", "-A", "dead_code", "-L", f"dependency={lib}",
               "--extern", f"typst_core={core}", str(source), "-o", str(exe)]
    compiled = subprocess.run(command, text=True, capture_output=True)
    if compiled.returncode and ("file format" in compiled.stderr or "undefined reference" in compiled.stderr):
        subprocess.run([*command, "-C", "lto=thin"], check=True)
    elif compiled.returncode:
        raise RuntimeError(compiled.stderr)
    output = subprocess.check_output([exe], text=True)
    (OUT / "proof-test.log").write_text(output)


def counts(log):
    publications = log.read_text().split("compiled successfully")[:-1]
    results = []
    for text in publications:
        observations = []
        for total, checked, reused, ordinals, reason in REUSE.findall(text):
            total, checked, reused = map(int, (total, checked, reused))
            pages = json.loads(ordinals) if ordinals else []
            assert len(pages) == reused, (pages, reused)
            observations.append(dict(total=total, compared=checked, reused=reused, processed=total-reused,
                                     ordinals=pages, reason=reason or None))
        results.append(observations)
    return results


def directed():
    visible = STATES[-1][1]
    moved = visible.replace(b"height:600pt", b"height:440pt")
    retarget = moved.replace(b"See @s0.", b"See @s13.", 1)
    assert visible != moved != retarget
    rtl = ('#set page(width: 235pt, height: 130pt, margin: 12pt)\n'
           '#set text(font: "DejaVu Sans", size: 11pt, lang: "he", dir: rtl)\n'
           + ('עברית ABC 123 (טקסט) ' * 100) + 'END\n').encode()
    opening = base.cycle.opening
    suites = {
        "original": STATES,
        "links": [("open", visible, None), ("move", moved, None), ("retarget", retarget, None)],
        "dependency": [("open", b'#include "middle.typ"\n' + visible, opening.LEAF),
                       ("edit", b'#include "middle.typ"\n' + visible, opening.LEAF_EDIT),
                       ("restore", b'#include "middle.typ"\n' + visible, opening.LEAF)],
        "span": opening.DIRECTED["positions"],
        "font": [("old-font", visible, None), ("new-font", b'#set text(font: "DejaVu Sans", size: 12pt)\n' + visible, None)],
        "resource": [("plain", visible, None),
                     ("shape-page", visible + b"\n#pagebreak()\n#rect(width: 30pt, height: 10pt)\n", None)],
        "multpage-bidi": [("rtl-open", rtl, None), ("rtl-tail", rtl.replace(b"END", "סוף".encode()), None)],
    }
    evidence, trace = {}, {}
    for suite, states in suites.items():
        case = WORK / "directed" / suite
        rows = {arm: base.watch(case / arm, arm, states, trace=True, keep=True) for arm in ARMS[:2]}
        locations = [base.cycle.positions(case / arm / "watch.log") for arm in ARMS[:2]]
        assert locations[0] == locations[1], suite
        trace[suite] = counts(case / "candidate/watch.log")
        assert len(trace[suite]) == len(states)
        evidence[suite] = []
        for i, (name, _, _) in enumerate(states):
            control, candidate = (case / arm / f"{name}.pdf" for arm in ARMS[:2])
            assert rows["control"][i]["pdf_hash"] == rows["candidate"][i]["pdf_hash"], (suite, name,"QDF")
            text, structure = base.text_layout(control), base.structure(control)
            assert base.text_layout(candidate) == text, (suite, name, "positions")
            assert base.structure(candidate) == structure, (suite, name, "structure")
            links = previous.annotations(control)
            assert links == previous.annotations(candidate), (suite, name, "links")
            rasters = 0
            if (suite, name) in (("original","visible"), ("multpage-bidi","rtl-tail")):
                a = base.render(control, case, "control-page")
                b = base.render(candidate, case, "candidate-page")
                assert a == b, (suite, name, "raster")
                assert suite != "multpage-bidi" or len(a) > 1
                rasters = len(a)
            evidence[suite].append(dict(state=name, qdf=rows["control"][i]["pdf_hash"], text=text,
                structure=structure, links=links, locations=len(locations[0][i]), rasters=rasters,
                page_reuse=trace[suite][i]))
        if suite == "original":
            for arm in ARMS:
                base.fresh(case / f"fresh-{arm}", arm, states)
            for name, _, _ in states:
                a, b = (case / f"fresh-{arm}" / f"{name}.pdf" for arm in ARMS[:2])
                assert base.cycle.cycle.key("candidate", a) == base.cycle.cycle.key("candidate", b)
                assert base.measure.sha(case / "fresh-vanilla" / f"{name}.pdf") == base.golden("vanilla", name)
        if suite == "links":
            for arm in ARMS[:2]:
                a, b = (base.cycle.destination_links(case / arm / f"{s}.pdf") for s in ("move","retarget"))
                assert a-b == {"u:s0":1} and b-a == {"u:s13":1}
        if suite == "dependency":
            a, b, c = evidence[suite]
            assert all(a[k] == c[k] != b[k] for k in ("text","structure","links"))
            for arm in ARMS[:2]:
                pdf = case / arm / "edit.pdf"
                text = subprocess.check_output(["pdftotext", pdf, "-"])
                assert b"External omega." in text and b"External alpha." not in text
                assert "u:https://example.test/omega" in previous.annotations(pdf)["uris"]
        (WORK / "directed.json").write_text(json.dumps(evidence, indent=2) + "\n")
        (WORK / "reuse-counts.json").write_text(json.dumps(trace, indent=2) + "\n")
        print("validated", suite, trace[suite], flush=True)
    with tarfile.open(OUT / "directed-logs.tar.gz", "w:gz") as archive:
        for path in sorted((WORK / "directed").rglob("*.log")):
            archive.add(path, arcname=str(path.relative_to(WORK)))


def reduce(rows):
    stats = base.phases.base.stats
    nominal = {arm:{} for arm in ARMS}
    profile = {arm:{} for arm in ARMS[:2]}
    tables = {}
    for arm in ARMS[:2]:
        path = WORK / f"{arm}.symbols"
        path.write_bytes(subprocess.check_output(["nm","-n","-S","-C","--defined-only",WORK/arm]))
        tables[arm] = base.phases.base.symbols(path)
    labels = {
        "bidi": ("typst_infra::layout_bidi::", "typst_infra::bidi_context::Registry::"),
        "shape": ("::shape_document_with_faces",),
        "reconcile": ("::shaper::reconciliation::",),
        "proof_and_retention": ("::pre_bidi::",),
    }

    class Scratch:
        def extractfile(self, name):
            return (WORK/name).open("rb")

    for row in rows:
        arm, block = row["arm"], row["block"]
        if not row["period_ns"]:
            nominal[arm].setdefault(block,{})[row["state"]] = row["cpu_ns"]/1e6
        if row["period_ns"] and row["state"] == "visible" and arm != "vanilla":
            stacks, detail = base.phases.base.stacks(Scratch(), row, tables)
            counts = Counter()
            for stack, n in stacks.items():
                for label, tokens in labels.items():
                    if any(t in frame for t in tokens for frame in stack):
                        counts[label] += n
                        break
            scale = detail["main_ns"]/detail["samples"]/1e6
            ms = {label:counts[label]*scale for label in labels}
            profile[arm][block] = dict(**detail, **ms, final=sum(ms.values()))
    result = dict(nominal={}, profile={}, profile_cases=profile)
    for state in [s[0] for s in STATES] + ["cycle"]:
        values = {arm:{b:sum(v.values()) if state=="cycle" else v[state] for b,v in rs.items()}
                  for arm,rs in nominal.items()}
        result["nominal"][state] = dict(
            absolute={arm:stats(list(rs.values())) for arm,rs in values.items()},
            paired={arm:stats([v-values[arm][b] for b,v in values["candidate"].items()])
                    for arm in ("control","vanilla")})
    for label in [*labels, "final"]:
        result["profile"][label] = dict(
            absolute={arm:mean(v[label] for v in rs.values()) for arm,rs in profile.items()},
            paired=stats([v[label]-profile["control"][b][label] for b,v in profile["candidate"].items()]))
    (WORK / "timing.json").write_text(json.dumps(result, indent=2)+"\n")


def main():
    os.sched_setaffinity(0,{2})
    copy2(D.parent / "unicode-pdf-cost-2026-11-19/profiler.so", WORK / "profiler.so")
    focused()
    directed()
    orders = list(permutations(ARMS))
    blocks = [(0,order) for order in orders*3]
    blocks += [(period,order) for period in (347000,937000) for order in orders]
    Random(50923).shuffle(blocks)
    rows = []
    with (WORK/"measurements.jsonl").open("w") as output:
        for block,(period,order) in enumerate(blocks):
            for arm in order:
                for row in base.watch(WORK/"cases"/f"{block:03d}-{arm}", arm, STATES, period=period):
                    record = dict(row, block=block)
                    rows.append(record)
                    print(json.dumps(record), file=output, flush=True)
            print(f"block {block+1}/{len(blocks)} period={period}", flush=True)
    reduce(rows)
    with tarfile.open(OUT/"sample-cases.tar.gz","w:gz") as archive:
        for p in sorted((WORK/"cases").rglob("*")):
            if p.name in ("maps","samples.bin","watch.log"):
                archive.add(p,arcname=str(p.relative_to(WORK)))


if __name__ == "__main__":
    main()
