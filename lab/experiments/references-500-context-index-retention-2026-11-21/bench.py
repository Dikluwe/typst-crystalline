"""Benchmark and verify one-batch context location memoization."""
from collections import Counter
import importlib.util
from itertools import permutations
import json
import os
from pathlib import Path
from random import Random
import re
from shutil import copy2
import subprocess
import sys
import tarfile
import traceback


D = Path(__file__).resolve().parent


def load(name, file):
    spec = importlib.util.spec_from_file_location(name, file)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


heading = load("heading_proof", D.parent / "references-500-heading-reference-proof-2026-11-21" / "bench.py")
initial, base = heading.initial, heading.base
WORK, OUT, STATES = heading.WORK, heading.OUT, heading.STATES
previous = load("reference_cost", D.parent / "references-500-heading-reference-cost-2026-11-21" / "bench.py")
ARMS = ("candidate","reuse","vanilla")
decoder = initial.decoder
debug_env = base.environment
LOG = re.compile(
    r"^lab-topology-index attempt=(\d+) roots=(\d+) builds=(\d+) hits=(\d+) physical=(\d+) "
    r"invalidations=(\d+) checked=(\d+)$", re.M)
FIELDS = ("attempt","roots","builds","hits","physical","invalidations","checked")


def dump(name, data):
    (OUT / name).write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n")


def environment(tmp, arm, trace):
    env = debug_env(tmp, arm, trace)
    if trace and arm == "reuse":
        env["TEKT_LAB_TOPOLOGY_INDEX_TRACE"] = "verify"
    return env


base.environment = environment
heading.environment = environment


def attempts(log):
    return [dict(zip(FIELDS, map(int, fields))) for fields in LOG.findall(log)]


def watch(case, arm, states, *, trace=False, period=0, gold=None):
    rows = base.watch(case, arm, states, trace=trace, period=period, keep=True)
    if gold:
        assert all(row["pdf_hash"] == gold[arm][row["state"]] for row in rows), (arm, case, rows)
    if not trace:
        for row in rows:
            (case / (row["state"] + ".pdf")).unlink()
    return rows


def visible_validation(gold):
    current = WORK / "validation" / "original"
    rows = {a:watch(current / a, a, STATES, trace=True, gold=gold) for a in ARMS}
    batches = [attempts(part) for part in (current/"reuse/watch.log").read_text().split("compiled successfully")[:-1]]
    assert len(batches) == 4
    assert all(row["checked"] == row["hits"] for batch in batches for row in batch)
    assert not batches[1] and not batches[2], batches
    assert sum(x["hits"] for x in batches[-1]) > 0, batches
    checks = {}
    for state, *_ in STATES:
        control, reuse, vanilla = [heading.pdf_details(current / a / f"{state}.pdf") for a in ARMS]
        same = heading.compare(control, reuse)
        assert all(same.values()), (state, same)
        checks[state] = dict(same=same, pages=control["pages"], links=len(control["links"]),
                             vanilla_links=len(vanilla["links"]), destinations=len(control["names"]))
    candidate, reuse = [base.render(current / a / "visible.pdf", current / a, "visible-raster") for a in ARMS[:2]]
    assert candidate == reuse
    positions = [base.cycle.positions(current / a / "watch.log") for a in ARMS[:2]]
    assert positions[0] == positions[1], (len(positions[0]), len(positions[1]))
    data = dict(rows=rows, phases=batches, pdf=checks, raster_pages=len(candidate),
                sealed=[len(p) for p in positions[0]])
    dump("original.json", data)
    print("original batches", batches, flush=True)


def compile_checked(directory, name, source, expected_tokens=(), bib=None):
    base_results = {a:heading.compile_case(directory / name / a, a, source, None, bib) for a in ARMS}
    control, reuse = [base_results[arm] for arm in ARMS[:2]]
    assert control["code"] == reuse["code"] and control["qdf"] == reuse["qdf"], (name, base_results)
    technical = ("watch-memo ", "watch-positions ", "pdf-")
    user_output = lambda row: [line for line in row["diag"].splitlines() if not line.startswith(technical)]
    assert user_output(control) == user_output(reuse), (name, control["diag"], reuse["diag"])
    same = None
    if control["content"] is not None:
        same = heading.compare(control["content"], reuse["content"])
        assert all(same.values()), (name, same)
        combined = reuse["content"]["text"]
    else:
        combined = reuse["diag"]
    assert all(value in combined for value in expected_tokens), (name, combined, expected_tokens)
    counts = attempts((directory / name / "reuse/compile.log").read_text())
    assert all(row["hits"] == row["checked"] for row in counts)
    return dict(results=base_results, checked=same, attempts=counts)


def directed():
    cases_path = WORK / "validation" / "directed"
    figures = heading.ARCHIVE
    heading_line = b'#set heading(numbering: "1.")\n'
    fixture = b'#figure([ART], kind: image, caption: [CAPTION], supplement: [FIG-SUPPLEMENT]) <fig>\n'
    callback = fixture.replace(b"[FIG-SUPPLEMENT]", b"it => [CALLBACK-SUPPLEMENT]")
    changes = heading_line + b"""= Alpha <a>

@a

#context {
  let n = counter(heading.where(level: 1)).get()
  [#metadata("changed-topology") <changed> #context [Nested result]]
}

= Beta <b>

@b
"""
    matrix = {
        "figure-explicit": (fixture + b"@fig", ("FIG-SUPPLEMENT",)),
        "figure-callback": (callback + b"@fig", ("CALLBACK-SUPPLEMENT",)),
        "figure-override": (callback + b'#ref(<fig>, supplement: [OVERRIDE])', ("OVERRIDE",)),
        "nested-reference-supplement": ((figures/"nested-references.typ").read_bytes(), ()),
        "error-in-supplement": ((figures/"inner-panic-reference.typ").read_bytes(), ("REF_INNER",)),
        "unnumbered-figure": ((figures/"unnumbered-target.typ").read_bytes(), ()),
        "changing-topology": (changes, ("Nested result",)),
    }
    output = {}
    for name, (source, tokens) in matrix.items():
        output[name] = compile_checked(cases_path, name, source, tokens)
        dump("directed.json", output)
        print("directed", name, [row["code"] for row in output[name]["results"].values()],
              output[name]["attempts"], flush=True)
    change_attempts = output["changing-topology"]["attempts"]
    assert any(row["invalidations"] and row["builds"] > 1 for row in change_attempts), change_attempts

    visible = STATES[-1][1]
    retarget = visible.replace(b"See @s0.", b"See @s13.", 1)
    figure = retarget.replace(b"= Section 13 <s13>", callback.rstrip().replace(b"<fig>", b"<s13>"), 1)
    assert visible != retarget != figure
    points = [("open",visible,None),("retarget",retarget,None),("figure",figure,None),("restore",retarget,None)]
    case = cases_path / "retarget"
    rows = {arm:watch(case/arm, arm, points, trace=True) for arm in ARMS}
    results = {}
    for i, (state,*_) in enumerate(points):
        assert rows["candidate"][i]["pdf_hash"] == rows["reuse"][i]["pdf_hash"], state
        pdf = [heading.pdf_details(case / arm / f"{state}.pdf") for arm in ARMS]
        assert all(heading.compare(*pdf[:2]).values()), state
        results[state] = {a: Counter(json.dumps(link[-1], sort_keys=True) for link in p["links"])
                          for a,p in zip(ARMS,pdf)}
    for arm in ARMS:
        first, modified = (results[s][arm] for s in ("open","retarget"))
        removed, added = first-modified, modified-first
        assert sum(removed.values()) == sum(added.values()) == 1
        assert "u:s0" in next(iter(removed)) and "u:s13" in next(iter(added))
        assert results["retarget"][arm] == results["restore"][arm]
    dump("retarget.json", dict(links=results, attempts=attempts((case / "reuse/watch.log").read_text())))
    print("directed retarget and restore", flush=True)


def measure(gold, stage=None):
    orders = list(permutations(ARMS))
    queue = [(0, order) for order in orders*3]
    queue += [(period, order) for period in (97000, 233000) for order in orders*2]
    Random(19920).shuffle(queue)
    rows = []
    filename = "measurements.jsonl" if stage is None else f"measurements-{stage}.jsonl"
    selected = [(block, period, order) for block, (period, order) in enumerate(queue)
                if stage is None or (period == 0) == (stage == "nominal")]
    with (WORK / filename).open("w") as output:
        for done, (block, period, order) in enumerate(selected, 1):
            for arm in order:
                path = WORK / "cases" / f"{block:02d}-{arm}"
                for row in watch(path, arm, STATES, period=period, gold=gold):
                    rows.append(dict(row, block=block))
                    print(json.dumps(rows[-1]), file=output, flush=True)
            copy2(WORK / filename, OUT / filename)
            print(f"{stage or 'all'} block {done}/{len(selected)}", flush=True)
    return rows


def reduce(rows):
    nominal, profiles, evidence = ({a:{} for a in ARMS} for _ in range(3))
    tables = {a: decoder.symbols(WORK / f"{a}.symbols") for a in ARMS}
    for row in rows:
        if row["state"] != "visible":
            continue
        arm, block = row["arm"], row["block"]
        if not row["period_ns"]:
            nominal[arm][block] = row["cpu_ns"] / 1e6
            continue
        stacks, detail = decoder.stacks(initial.Scratch(), row, tables)
        counts = Counter()
        for stack, n in stacks.items():
            k = previous.category(arm, stack)
            counts[k] += n
        scopes = previous.VANILLA_PHASES if arm == "vanilla" else previous.TEKT_PHASES
        profiles[arm][block] = {k: counts[k] * detail["main_ns"] / detail["samples"] / 1e6 for k in scopes}
        evidence[arm][block] = dict(**detail, counts=dict(counts), period_ns=row["period_ns"])
    phases = {a: {k:initial.prior.residual.mean_ci([v[k] for v in runs.values()])
                  for k in next(iter(runs.values()))}
              for a,runs in profiles.items()}
    differences = {other: decoder.stats([v-nominal[other][b] for b,v in nominal["reuse"].items()])
                   for other in ("candidate","vanilla")}
    result = dict(cpu={a:decoder.stats(list(v.values())) for a,v in nominal.items()},
                  paired=differences, phase=phases, profile=profiles, samples=evidence)
    dump("timing.json", result)
    print("visible cpu", result["cpu"], "paired", differences, flush=True)


def archive(stage):
    filename = "cases.tar.gz" if stage is None else f"{stage}-cases.tar.gz"
    with tarfile.open(OUT / filename, "w:gz") as archive:
        for group in ("validation", "cases"):
            for path in sorted((WORK/group).rglob("*")):
                if path.name in ("watch.log","compile.log","maps","samples.bin"):
                    archive.add(path, arcname=str(path.relative_to(WORK)))
        for arm in ARMS:
            path = WORK / f"{arm}.symbols"
            if path.exists():
                archive.add(path, arcname=path.name)


def main():
    os.sched_setaffinity(0,{2})
    copy2(D.parent/"unicode-pdf-cost-2026-11-19/profiler.so", WORK/"profiler.so")
    for arm in ARMS[:2]:
        with (WORK / f"{arm}.symbols").open("wb") as file:
            subprocess.run(["nm","-n","-S","-C","--defined-only",WORK/arm],stdout=file,check=True)
    copy2(initial.prior.residual.TABLES["vanilla"], WORK/"vanilla.symbols")
    gold = {arm:{s:base.golden("vanilla" if arm=="vanilla" else "candidate",s)
                 for s,*_ in STATES} for arm in ARMS}
    stage = next((mode for mode in ("nominal", "profile") if f"--{mode}" in sys.argv), None)
    checks_error = None
    try:
        if stage != "profile" and "--timing-only" not in sys.argv:
            visible_validation(gold)
            try:
                directed()
            except AssertionError as error:
                checks_error = error
                traceback.print_exc()
        if "--validation-only" not in sys.argv:
            rows = measure(gold, stage)
            if stage == "profile":
                data = (OUT / "measurements-nominal.jsonl").read_text().splitlines()
                rows += [json.loads(line) for line in data]
                (OUT / "measurements.jsonl").write_text(
                    "".join(json.dumps(row) + "\n" for row in sorted(rows, key=lambda row: row["block"])))
            if stage != "nominal":
                reduce(rows)
        if checks_error is not None:
            raise checks_error
    finally:
        archive(stage)


if __name__ == "__main__":
    main()
