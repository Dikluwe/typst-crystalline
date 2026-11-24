"""Attribute CPU on the original watch publication without altering either ELF."""
from collections import Counter
from importlib.util import module_from_spec, spec_from_file_location
from itertools import permutations
import json
import os
from pathlib import Path
from random import Random
from shutil import copy2
import subprocess
import sys
import tarfile


D = Path(__file__).resolve().parent
RESIDUAL = D.parent / "references-500-visible-eval-residual-2026-11-21"


def load(name, path):
    spec = spec_from_file_location(name, path)
    module = module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


base = load("context_bench", D.parent / "references-500-context-substitution-2026-11-21/bench.py")
residual = load("residual", RESIDUAL / "analyze.py")
native = load("native", RESIDUAL / "vanilla.py")
decoder = residual.base
WORK, OUT, STATES = base.WORK, base.OUT, base.STATES
ARMS = ("candidate", "vanilla")
LABELS = ("capture", "group", "close", "scopes", "eval")
SET_WRAP_RETURN = 0x24DBB2C


def classify(arm, stack):
    def has(*tokens):
        return any(token in frame for token in tokens for frame in stack)

    if arm == "vanilla":
        if "typst_eval::eval" not in stack:
            return None
        if has("[set-wrap-pc]"):
            return "close"
        return "other"

    if not has("::eval_to_module_in_session"):
        return None
    if has("::TextualSnapshot::capture_following", "::TextualSnapshot::capture"):
        return "capture"
    if has("::TextualSnapshot::same_group"):
        return "group"
    if has("::markup_stream::close_style_tail"):
        return "close"
    return "other"


def annotate(maps, exe, table):
    resolve = original_resolver(maps, exe, table)
    if Path(exe).name != "vanilla":
        return resolve
    image = next(int(line.split()[0].split("-")[0], 16) for line in maps.splitlines()
                 if line.split()[2] == "00000000" and line.endswith(exe))

    def lookup(ip):
        name = resolve(ip)
        if ip - image == SET_WRAP_RETURN - 1:
            return f"{name} [set-wrap-pc]"
        return name
    return lookup


original_resolver = decoder.resolver


class Scratch:
    def extractfile(self, name):
        return (WORK / name).open("rb")


def reduce(rows):
    tables = {arm: decoder.symbols(WORK / f"{arm}.symbols") for arm in ARMS}
    nominal = {arm: {} for arm in ARMS}
    sampled = {arm: [] for arm in ARMS}
    hot = Counter()
    decoder.resolver = annotate
    for row in rows:
        if row["state"] != "visible":
            continue
        arm, block = row["arm"], row["block"]
        if not row["period_ns"]:
            nominal[arm][block] = row["cpu_ns"] / 1e6
            continue
        stacks, detail = decoder.stacks(Scratch(), row, tables)
        counts = Counter()
        for stack, ticks in stacks.items():
            label = classify(arm, stack)
            if label is None:
                continue
            counts[label] += ticks
            if label != "other":
                hot[arm, label, stack] += ticks
        counts["eval"] = counts.total()
        counts["scopes"] = sum(counts[k] for k in ("capture", "group", "close"))
        scale = detail["main_ns"] / detail["samples"] / 1e6
        sampled[arm].append(dict(block=block, period_ns=row["period_ns"], counts=dict(counts),
                                 ms={k: counts[k] * scale for k in LABELS},
                                 process_cpu_ms=row["cpu_ns"] / 1e6, **detail))
    results = {}
    for arm, cases in sampled.items():
        summaries = {}
        for label in LABELS:
            summaries[label] = dict(
                **residual.mean_ci([v["ms"][label] for v in cases]),
                ticks=sum(v["counts"].get(label, 0) for v in cases),
                period_means={period: residual.mean_ci([v["ms"][label] for v in cases
                                                       if v["period_ns"] == period])["mean"]
                              for period in sorted({v["period_ns"] for v in cases})},
            )
        results[arm] = summaries
    with (OUT / "scope-stacks.txt").open("w") as output:
        for (arm, label, stack), count in hot.most_common():
            print(arm, label, count, " | ".join(stack), file=output)
    return dict(
        nominal={arm: decoder.stats(list(v.values())) for arm, v in nominal.items()},
        nominal_samples=len(nominal["candidate"]),
        nominal_gap=decoder.stats([v - nominal["vanilla"][block]
                                   for block, v in nominal["candidate"].items()]),
        sampled=results, sample_cases=sampled,
    )


def validate_pdf():
    evidence = {}
    for arm in ARMS:
        case = WORK / "validation" / arm
        rows = base.watch(case, arm, STATES, keep=True)
        for row in rows:
            state = row["state"]
            pdf = case / f"{state}.pdf"
            assert row["pdf_hash"] == base.golden(arm, state)
            check = subprocess.check_output(["qpdf", "--check", pdf], text=True)
            evidence.setdefault(arm, {})[state] = dict(reference=row["pdf_hash"], qpdf=check,
                                                       positions=base.text_layout(pdf))
        pages = base.render(case / "visible.pdf", case, "visible-page")
        evidence[arm]["visible"]["rendered_page_hashes"] = pages
    (OUT / "validation.json").write_text(json.dumps(evidence, indent=2) + "\n")


def main():
    os.sched_setaffinity(0, {2})
    copy2(residual.TABLES["vanilla"], WORK / "vanilla.symbols")
    # Verify this is the call to Content::styled_with_map in eval_markup.
    relocations = subprocess.check_output(["readelf", "-rW", WORK / "vanilla"], text=True)
    wrap = next(line.split() for line in relocations.splitlines() if line.startswith("00000000031e1bf0 "))
    assert int(wrap[-1], 16) == 0x287F458, wrap
    orders = list(permutations(ARMS))
    blocks = [(0, order) for order in orders * 10]
    blocks += [(period, order) for period in (73000, 173000) for order in orders * 14]
    Random(698540).shuffle(blocks)
    rows = []
    with (WORK / "measurements.jsonl").open("w") as output:
        for block, (period, order) in enumerate(blocks):
            for arm in order:
                for row in base.watch(WORK / "cases" / f"{block:03d}-{arm}", arm, STATES, period=period):
                    record = dict(row, block=block)
                    rows.append(record)
                    print(json.dumps(record), file=output, flush=True)
            print(f"block {block+1}/{len(blocks)} period={period}", flush=True)
    result = reduce(rows)
    (WORK / "attribution.json").write_text(json.dumps(result, indent=2) + "\n")
    with tarfile.open(OUT / "sample-cases.tar.gz", "w:gz") as archive:
        for path in sorted((WORK / "cases").rglob("*")):
            if path.name in ("maps", "samples.bin", "watch.log"):
                archive.add(path, arcname=str(path.relative_to(WORK)))
        for arm in ARMS:
            archive.add(WORK / f"{arm}.symbols", arcname=f"{arm}.symbols")
    native.main()
    copy2(WORK / "vanilla-phases.json", WORK / "native.json")
    verify()


def verify():
    os.sched_setaffinity(0, {2})
    load("wrap", D / "wrap.py").main(WORK, OUT, native)
    validate_pdf()
    with tarfile.open(OUT / "native-traces.tar.gz", "w:gz") as archive:
        for path in sorted((WORK / "vanilla-traces").rglob("*")):
            if path.name.startswith("timings-") or path.name in ("clock.bin", "watch.log", "wrap.bin"):
                archive.add(path, arcname=str(path.relative_to(WORK)))


if __name__ == "__main__":
    (verify if sys.argv[1:] == ["--verify-only"] else main)()
