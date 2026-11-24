"""Validate directed links and time the original edit without census probes."""
from collections import Counter, defaultdict
from importlib.util import module_from_spec, spec_from_file_location
from itertools import permutations
import json
import os
from pathlib import Path
from random import Random
from statistics import mean
from shutil import copy2
import subprocess
import sys
import tarfile

D = Path(__file__).resolve().parent
spec = spec_from_file_location("context_bench", D.parent / "references-500-context-substitution-2026-11-21/bench.py")
base = module_from_spec(spec)
spec.loader.exec_module(base)
WORK, OUT, STATES = base.WORK, base.OUT, base.STATES
ARMS = ("control", "candidate", "vanilla")
NESTED = b'''#set page(width: 230pt, height: 105pt, margin: 12pt)
#set text(size: 11pt)
#let outer = "https://example.test/outer"
#let inner = "https://example.test/inner"
Before.

#link(outer)[Prefix #link(outer)[same target].
  #link(inner)[different target].
  #for i in range(20) [ continuation #i;]
  tail.]

After.
'''


def focused():
    core = WORK / "target/release/deps"
    library = max(core.glob("libtypst_core-*.rlib"), key=lambda path: path.stat().st_mtime_ns)
    result = []
    for arm, file in (
        ("control", WORK / "link_capture.control.rs"),
        ("candidate", OUT / "link_capture.candidate.rs"),
    ):
        source = file.read_text()
        begin, end = source.index("fn same_target("), source.index("fn wrap<")
        tests = WORK / f"test_{arm}.rs"
        tests.write_text((D / "focused.rs").read_text() + "\n" + source[begin:end])
        exe = WORK / f"test-{arm}"
        command = ["rustc", "--test", "--edition", "2024", "-L", f"dependency={core}",
                   "--extern", f"typst_core={library}", str(tests), "-o", str(exe)]
        compiled = subprocess.run(command, text=True, capture_output=True)
        if compiled.returncode and ("file format" in compiled.stderr or "undefined reference" in compiled.stderr):
            subprocess.run([*command, "-C", "lto=thin"], check=True)
        elif compiled.returncode:
            raise RuntimeError(compiled.stderr)
        semantic = subprocess.run([exe, "keeps_order_and_never_opens_a_distinct_link"],
                                  capture_output=True, text=True, check=True)
        storage = subprocess.run([exe, "retains_parent_and_unchanged_child_buffers"],
                                 capture_output=True, text=True)
        assert storage.returncode == (101 if arm == "control" else 0), storage.stdout
        result.append(arm + "\n" + semantic.stdout + storage.stdout + storage.stderr)
    (WORK / "focused.log").write_text("\n".join(result))


def annotations(pdf):
    from hashlib import sha256
    data = json.loads(subprocess.check_output(["qpdf", "--json", str(pdf)], timeout=30))
    objects = data["qpdf"][-1]
    links = []
    uris = defaultdict(set)
    for page in data["pages"]:
        entry = objects["obj:" + page["object"]]["value"]
        for ref in entry.get("/Annots", []):
            item = objects["obj:" + ref]["value"]
            if item.get("/Subtype") == "/Link":
                assert len(item["/Rect"]) == 4, item
                target = item["/Dest"] if "/Dest" in item else item["/A"]
                ordinal = page["pageposfrom1"]
                links.append([ordinal, item["/Rect"], target])
                if isinstance(target, dict) and "/URI" in target:
                    uris[target["/URI"]].add(ordinal)
    return dict(count=len(links), sha256=sha256(json.dumps(links, sort_keys=True).encode()).hexdigest(),
                uris={uri:sorted(positions) for uri,positions in sorted(uris.items())})


def directed():
    visible = STATES[-1][1]
    moved = visible.replace(b"height:600pt", b"height:440pt")
    assert moved != visible
    retarget = moved.replace(b"See @s0.", b"See @s13.", 1)
    assert retarget != moved
    suites = {
        "original": STATES,
        "moves-links": [("open", visible, None), ("moved", moved, None), ("retarget", retarget, None)],
        "nested-page-spanning": [("nested", NESTED, None),
            ("new-target", NESTED.replace(b"example.test/inner", b"example.test/replacement"), None)],
    }
    evidence = {}
    for suite, states in suites.items():
        case = WORK / "directed" / suite
        rows = {arm: base.watch(case / arm, arm, states, trace=True, keep=True)
                for arm in ARMS[:2]}
        locations = [base.cycle.positions(case / arm / "watch.log") for arm in ARMS[:2]]
        assert locations[0] == locations[1], suite
        evidence[suite] = []
        for i, (name, _, _) in enumerate(states):
            control, candidate = (case / arm / f"{name}.pdf" for arm in ARMS[:2])
            assert rows["control"][i]["pdf_hash"] == rows["candidate"][i]["pdf_hash"], (suite, name, "QDF")
            assert base.text_layout(control) == base.text_layout(candidate), (suite, name, "word positions")
            assert base.structure(control) == base.structure(candidate), (suite, name, "PDF structure")
            info = annotations(control)
            assert info == annotations(candidate) and info["count"], (suite, name, "links")
            rasters = 0
            if (suite, name) in (("original", "visible"), ("nested-page-spanning", "new-target")):
                a = base.render(control, case, "control-page")
                b = base.render(candidate, case, "candidate-page")
                assert a == b, (suite, name, "rasters")
                rasters = len(a)
            evidence[suite].append(dict(state=name, qdf=rows["control"][i]["pdf_hash"],
                annotations=info, locations=len(locations[0][i]), rasters=rasters))
        if suite == "moves-links":
            for arm in ARMS[:2]:
                a, b = (base.cycle.destination_links(case / arm / f"{s}.pdf")
                        for s in ("moved", "retarget"))
                assert a - b == {"u:s0": 1} and b - a == {"u:s13": 1}, arm
        if suite == "nested-page-spanning":
            for entry in evidence[suite]:
                uri_pages = entry["annotations"]["uris"]
                assert len(uri_pages["u:https://example.test/outer"]) >= 2, entry
            for arm in ARMS[:2]:
                qdf = subprocess.check_output(["qpdf", "--qdf", str(case / arm / "new-target.pdf"), "-"])
                assert b"https://example.test/replacement" in qdf and b"https://example.test/inner" not in qdf
                assert b"https://example.test/outer" in qdf
        if suite == "original":
            for arm in ARMS[:2]:
                base.fresh(case / f"fresh-{arm}", arm, STATES)
            for name, _, _ in STATES:
                a, b = (case / f"fresh-{arm}" / f"{name}.pdf" for arm in ARMS[:2])
                assert base.cycle.cycle.key("candidate", a) == base.cycle.cycle.key("candidate", b)
        (WORK / "directed.json").write_text(json.dumps(evidence, indent=2) + "\n")
        print("validated", suite, flush=True)
    with tarfile.open(OUT / "directed-logs.tar.gz", "w:gz") as archive:
        for path in sorted((WORK / "directed").rglob("*.log")):
            archive.add(path, arcname=str(path.relative_to(WORK)))


def reduce(rows):
    stats = base.phases.base.stats
    nominal = {arm: {} for arm in ARMS}
    profiled = {arm: {} for arm in ARMS[:2]}
    tables = {}
    for arm in ARMS[:2]:
        path = WORK / f"{arm}.symbols"
        path.write_bytes(subprocess.check_output(["nm", "-n", "-S", "-C", "--defined-only", WORK / arm]))
        tables[arm] = base.phases.base.symbols(path)

    class Scratch:
        def extractfile(self, name):
            return (WORK / name).open("rb")

    for row in rows:
        if row["state"] != "visible":
            continue
        arm, block = row["arm"], row["block"]
        if not row["period_ns"]:
            nominal[arm][block] = row["cpu_ns"]/1e6
        elif arm != "vanilla":
            stacks, detail = base.phases.base.stacks(Scratch(), row, tables)
            counts = Counter()
            for stack, n in stacks.items():
                if any("::link_capture::" in frame for frame in stack):
                    counts["link_capture"] += n
                if any("::link_capture::remove_redundant" in frame
                       or "::link_capture::clean_link_child" in frame
                       or "::link_capture::promote_link" in frame for frame in stack):
                    counts["remove_redundant"] += n
            factor = detail["main_ns"]/detail["samples"]/1e6
            profiled[arm][block] = dict(**detail, **{k:counts[k]*factor
                for k in ("link_capture", "remove_redundant")})
    result = dict(
        nominal={a:stats(list(rs.values())) for a,rs in nominal.items()},
        paired={a:stats([v-nominal[a][b] for b,v in nominal["candidate"].items()])
                for a in ("control","vanilla")},
        profile_cases=profiled,
        profile={scope:{**{a:mean(r[scope] for r in rs.values()) for a,rs in profiled.items()},
                        "paired":stats([r[scope]-profiled["control"][b][scope]
                                        for b,r in profiled["candidate"].items()])}
                 for scope in ("link_capture", "remove_redundant")})
    (WORK / "timing.json").write_text(json.dumps(result, indent=2) + "\n")


def main():
    os.sched_setaffinity(0, {2})
    profiler = D.parent / "unicode-pdf-cost-2026-11-19/profiler.so"
    copy2(profiler, WORK / "profiler.so")
    focused()
    directed()
    orders = list(permutations(ARMS))
    blocks = [(0, order) for order in orders * 3]
    blocks += [(period, order) for period in (347000, 937000) for order in orders]
    Random(118409).shuffle(blocks)
    rows = []
    with (WORK / "measurements.jsonl").open("w") as output:
        for block, (period, order) in enumerate(blocks):
            for arm in order:
                case = WORK / "cases" / f"{block:03d}-{arm}"
                for row in base.watch(case, arm, STATES, period=period):
                    record = dict(row, block=block)
                    rows.append(record)
                    print(json.dumps(record), file=output, flush=True)
            print(f"block {block + 1}/{len(blocks)} period={period}", flush=True)
    reduce(rows)
    with tarfile.open(OUT / "sample-cases.tar.gz", "w:gz") as archive:
        for p in sorted((WORK / "cases").rglob("*")):
            if p.name in ("maps", "samples.bin", "watch.log"):
                archive.add(p, arcname=str(p.relative_to(WORK)))


if __name__ == "__main__":
    main()
