"""Measure four successive watch publications against the exact integrated control."""
from collections import Counter
import gzip
import hashlib
import importlib.util
from itertools import permutations
import json
import os
from pathlib import Path
from random import Random
import re
import time

D = Path(__file__).resolve().parent
WORK = Path(os.environ["TEKT_LAB_WORK"])
PRIOR = D.parent / "references-500-outline-hybrid-2026-11-21"
spec = importlib.util.spec_from_file_location("hybrid_opening_bench", PRIOR / "bench.py")
opening = importlib.util.module_from_spec(spec)
spec.loader.exec_module(opening)
base = opening.base
driver = base.driver
measure = base.measure
original = base.runner.prior
ARMS = ("hybrid", "tekt", "vanilla")
EXES = {arm: WORK / arm for arm in ARMS}
STATES = base.REGULAR
NAMES = [name for name, _, _ in STATES]
HISTORICAL = json.loads((original.D / "receipt.json").read_text())


def dump(file, value):
    (WORK / file).write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n")


def environment(tmp, arm, trace):
    env = driver.environment(tmp)
    if arm == "hybrid":
        env.update(TEKT_LAB_SKIP_PROVISIONAL_REPLAY="1", TEKT_LAB_SOURCE_ONLY_FIRST_INDEX="1",
                   TEKT_LAB_OUTLINE_ORDINALS="hybrid", TEKT_LAB_WATCH_RETAIN="1",
                   TEKT_LAB_WATCH_LAZY="1", TEKT_LAB_WATCH_INERT="1", TEKT_LAB_PDF_REUSE="both")
        if trace:
            env["TEKT_LAB_WATCH_TRACE"] = "1"
    return env


def watch(case, arm, trace=False):
    case.mkdir(parents=True)
    tmp = case / "tmp"
    tmp.mkdir()
    source, pdf = case / "main.typ", case / "out.pdf"
    base.runner.setup_sources(case, *STATES[0][1:])
    command = [str(EXES[arm]), "watch", str(source), str(pdf), "--root", str(case), "--pdf-tagged=true"]
    log_path = case / "watch.log"
    rows = []
    with log_path.open("wb") as log:
        start = time.perf_counter_ns()
        child = driver.Child(command, case, environment(tmp, arm, trace), 1, log)
        try:
            publication = None
            for number, (state, data, leaf) in enumerate(STATES):
                before = 0
                if number:
                    before = child.cpu()
                    start = time.perf_counter_ns()
                    base.runner.setup_sources(case, data, leaf)
                publication, published, _ = driver.await_pdf(child, pdf, publication)
                after = measure.settle(child)
                output = case / f"{state}.pdf"
                output.write_bytes(pdf.read_bytes())
                rows.append(dict(arm=arm, state=state, utc=measure.utc(), cpu_ns=after-before,
                                 wall_ns=published-start, pdf=str(output.relative_to(WORK))))
        finally:
            child.stop()
            child.close()
    log = log_path.read_text()
    assert log.count("compiled successfully") == len(STATES), (case, log[-3000:])
    return rows


def key(arm, pdf):
    return measure.sha(pdf) if arm == "vanilla" else base.qdf_hash(pdf)


def validate(rows):
    base.EXES.update(EXES)
    for arm in ARMS:
        base.fresh(WORK / "fresh" / arm, arm, STATES, True)
    refs = {(arm, name): key(arm, WORK / "fresh" / arm / f"{name}.pdf")
            for arm in ARMS for name in NAMES}
    for name in NAMES:
        assert refs["hybrid", name] == refs["tekt", name], name
    checked = Counter()
    for row in rows:
        arm, name = row["arm"], row["state"]
        assert key(arm, WORK / row["pdf"]) == refs[arm, name], (arm, name, row["pdf"])
        checked[arm, name] += 1
    navigation = {}
    for name in NAMES:
        navigation[name] = {}
        for arm in ARMS:
            path = WORK / "fresh" / arm / f"{name}.pdf"
            desc = original.compare.describe(path)
            entry = original.navigation(path, desc, name == "visible")
            bad = entry["wrong_pages"]
            assert len(bad["toc_links"]) == len(bad["bookmarks"]) == (0 if arm == "vanilla" else 19), (arm, name, bad)
            assert not bad["references"] and not bad["toc_printed"], (arm, name, bad)
            navigation[name][arm] = dict(pages=desc["page_count"], links=sum(map(len, desc["links"])), **entry)
        assert len({item["body_sha256"] for item in navigation[name].values()}) == 1, name
    dump("validation.json", dict(checked={arm:{name:checked[arm, name] for name in NAMES} for arm in ARMS},
                                hashes={arm:{name:refs[arm, name] for name in NAMES} for arm in ARMS},
                                navigation=navigation))


def summarize(blocks):
    outputs = {}
    for states, name in [((s,), s) for s in NAMES] + [(NAMES[:3], "through-comments"), (NAMES, "total")]:
        entry = {}
        for metric in ("cpu_ns", "wall_ns"):
            values = {arm: [sum(row[metric] for row in block[arm] if row["state"] in states) / 1e6
                            for block in blocks] for arm in ARMS}
            entry[metric] = dict(absolute={arm:base.bootstrap(ms) for arm, ms in values.items()},
                                 paired={arm:base.bootstrap([a-b for a,b in zip(values["hybrid"],values[arm])])
                                         for arm in ("tekt", "vanilla")})
        outputs[name] = entry
    dump("timing.json", outputs)


def trace():
    rows = watch(WORK / "trace" / "hybrid", "hybrid", trace=True)
    path = WORK / "trace" / "hybrid" / "watch.log"
    groups = opening.log_groups(path)
    pages = base.LINE.findall(path.read_text())
    publications = path.read_text().split("compiled successfully")[:-1]
    assert len(groups) == len(publications) == len(pages) == len(STATES)
    assert groups[0]["native_proved"] == 1
    assert all("watch-retain hit inert" in part for part in publications[1:3])
    result = {}
    for row, group, pdf, publication in zip(rows, groups, pages, publications):
        font_hit, operators, total, eligible = pdf
        result[row["state"]] = dict(native_proved=bool(group["native_proved"]),
            checkpoint_attempted=bool(group["checkpoint_attempted"]),
            physical_memo_builds=group["traversals"], physical_memo_hits=group["hits"],
            watch_inert="watch-retain hit inert" in publication, font_hit=font_hit=="true",
            operator_hits=int(operators), pages=int(total), pdf_reuse_eligible=eligible=="true")
    dump("trace.json", result)
    return rows


def main():
    os.sched_setaffinity(0, {2})
    hash_inputs = {name: hashlib.sha256(data).hexdigest() for name, data, _ in STATES}
    assert hash_inputs == HISTORICAL["input_sha256"]
    dump("inputs.json", hash_inputs)
    for arm in ARMS:
        watch(WORK / "warmup" / arm, arm)

    orders = list(permutations(ARMS)) * 4
    Random(50021).shuffle(orders)
    blocks = []
    with gzip.open(WORK / "nominal.json.gz", "wt") as samples:
        for block, order in enumerate(orders):
            current = {arm:watch(WORK / "nominal" / f"{block:02d}" / arm, arm) for arm in order}
            for state_rows in current.values():
                for row in state_rows:
                    samples.write(json.dumps(row | {"block": block}) + "\n")
            blocks.append(current)
            print(f"{block+1:02d}/24", " ".join(order), flush=True)
    summarize(blocks)
    rows = [row for block in blocks for case in block.values() for row in case]
    validate(rows + trace())


if __name__ == "__main__":
    main()
