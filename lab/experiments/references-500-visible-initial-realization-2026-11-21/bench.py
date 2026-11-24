"""Attribute initial Tekt realization and the corresponding vanilla work."""
from collections import Counter
import importlib.util
from itertools import permutations
import json
import os
from pathlib import Path
from random import Random
from shutil import copy2
from statistics import mean, median
import struct
import subprocess
import sys
import tarfile

D = Path(__file__).resolve().parent


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


prior = load("style_scopes", D.parent / "references-500-visible-style-scopes-2026-11-21" / "bench.py")
base, decoder, native = prior.base, prior.decoder, prior.native
WORK, OUT, ARMS, STATES = prior.WORK, prior.OUT, prior.ARMS, prior.STATES
ELF_NORMALIZATION_LABEL = "normalize_insertions-callsite"
ELF_OTHER_TAGS = {0x220AA46: "observe_occurrences-callsite", 0x220ADFD: "page_running::needs_capture-callsite"}
ROOT = "::compiler::realize::realize_document"
NORMALIZATION = "::compiler::realize::normalize_insertions"
TEKT = ("observe", "finish", "occurrences", "normalization", "columns", "index", "recipes",
        "index_route", "other", "initial")
VANILLA = ("occurrences_and_show", "spaces", "other_realize", "document_realize",
           "nested_realize", "physical_index")


def has(stack, *tokens):
    return any(token in name for name in stack for token in tokens)


def classify(arm, stack):
    if arm == "candidate":
        if not (has(stack, ROOT) and has(stack, "::eval_to_module_in_session")):
            return None
        labels = {
            "observe": ("::realize::observe_occurrences", "observe_occurrences-callsite"),
            "finish": ("::realize::finish_occurrences",),
            "normalization": (NORMALIZATION, ELF_NORMALIZATION_LABEL),
            "columns": ("::wrap_page_columns",),
            "index": ("::introspect_with_introspector", "::provisional_body_index"),
            "recipes": ("::realize::recipes::realize",),
            "index_route": ("page_running::needs_capture",),
        }
        return next((k for k, tokens in labels.items() if has(stack, *tokens)), "other")

    if has(stack, "typst_layout::introspect::PagedIntrospector::new"):
        return "physical_index"
    if not has(stack, "typst_layout::pages::layout_document_common") or "typst_realize::realize" not in stack:
        return None
    if stack.count("typst_realize::realize") > 1 or any(
        has(stack, t) for t in ("typst_layout::flow::", "typst_layout::inline::",)
    ):
        return "nested_realize"
    if has(stack, "typst_realize::visit_show_rules"):
        return "occurrences_and_show"
    if has(stack, "typst_realize::spaces::collapse_spaces"):
        return "spaces"
    return "other_realize"


def instructions():
    entries, _ = decoder.symbols(WORK / "candidate.symbols")
    start, size = next((k, n) for k, (n, name) in entries.items() if name.endswith(ROOT))
    cmd = ["objdump", "-dC", f"--start-address={start}", f"--stop-address={start + size}", WORK / "candidate"]
    listing = subprocess.check_output(cmd, text=True)
    (OUT / "tekt-realization.asm").write_text(listing)
    return entries


def install_callsite_tags(addresses):
    resolver = decoder.resolver
    tags = {**ELF_OTHER_TAGS, **{pc: ELF_NORMALIZATION_LABEL for pc in addresses}}

    def annotate(maps, exe, table):
        lookup = resolver(maps, exe, table)
        if Path(exe).name != "candidate":
            return lookup
        instance = next(int(line.split()[0].split("-")[0], 16) for line in maps.splitlines()
                        if line.split()[2] == "00000000" and line.endswith(exe))

        def resolve(ip):
            name = lookup(ip)
            tag = tags.get(ip - instance + 1)
            return f"{name} [{tag}]" if tag else name

        return resolve

    decoder.resolver = annotate


class Scratch:
    def extractfile(self, name):
        return (WORK / name).open("rb")


def summarize(rows, norm_pc):
    tables = {arm: decoder.symbols(WORK / f"{arm}.symbols") for arm in ARMS}
    install_callsite_tags(norm_pc)
    nominal = {arm: {} for arm in ARMS}
    sampled = {arm: [] for arm in ARMS}
    hot = Counter()
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
            hot[arm, label, stack] += ticks
        if arm == "candidate":
            counts["initial"] = counts.total()
            counts["occurrences"] = counts["observe"] + counts["finish"]
        else:
            counts["document_realize"] = sum(counts[k] for k in ("occurrences_and_show", "spaces", "other_realize"))
        scale = detail["main_ns"] / detail["samples"] / 1e6
        sampled[arm].append(dict(block=block, period_ns=row["period_ns"], counts=dict(counts),
                                 ms={k: counts[k] * scale for k in (TEKT if arm == "candidate" else VANILLA)},
                                 process_cpu_ms=row["cpu_ns"] / 1e6, **detail))
    with (OUT / "scope-stacks.txt").open("w") as stream:
        for (arm, label, stack), count in hot.most_common():
            print(arm, label, count, " | ".join(stack), file=stream)
    return dict(
        nominal={arm: decoder.stats(list(v.values())) for arm, v in nominal.items()},
        nominal_gap=decoder.stats([v - nominal["vanilla"][b] for b, v in nominal["candidate"].items()]),
        sampled={arm: {label: dict(**prior.residual.mean_ci([v["ms"][label] for v in cases]),
                                   ticks=sum(v["counts"].get(label, 0) for v in cases))
                       for label in (TEKT if arm == "candidate" else VANILLA)}
                 for arm, cases in sampled.items()},
        sample_cases=sampled,
    )


def vanilla_scopes(case):
    clock = dict(struct.iter_unpack("<QQ", (case / "clock.bin").read_bytes()))
    events = json.loads((case / "timings-3.json").read_text())
    offsets = [round(event["ts"] * 1000) for event in events]
    zero, = (word for word in clock if all(word + delta in clock for delta in offsets[:12]))
    active, totals, calls = [], Counter(), Counter()
    first = {}
    for item, offset in zip(events, offsets):
        name, elapsed = item["name"], clock[zero + offset]
        if item["ph"] == "B":
            active.append((name, elapsed))
        else:
            label, start = active.pop()
            assert label == name and start <= elapsed
            duration = elapsed - start
            if name == "realize":
                key = ("document_realize" if active and active[-1][0] == "layout document"
                       else "nested_realize")
            elif name == "introspect pages":
                key = "physical_index"
            elif name in ("eval", "layout document", "page run"):
                key = name
            else:
                continue
            first.setdefault(key, duration)
            totals[key] += duration
            calls[key] += 1
    assert not active and calls["document_realize"] > 0 and calls["physical_index"] > 0
    return dict(ms={k: v / 1e6 for k, v in totals.items()}, calls=dict(calls),
                first_ms={k: v / 1e6 for k, v in first.items()})


def run_native():
    gold = json.loads(Path(os.environ["TEKT_VISIBLE_GOLD"]).read_text())["hashes"]["vanilla"]
    rows = []
    for block in range(20):
        recorded = native.run(block, gold)
        record = vanilla_scopes(WORK / "vanilla-traces" / f"{block:02d}")
        rows.append(dict(**record, publication_cpu_ms=recorded["cpu_ms"]))
    result = dict(rows=rows,
                  median_ms={k: median(r["ms"][k] for r in rows) for k in rows[0]["ms"]},
                  first_median_ms={k: median(r["first_ms"][k] for r in rows) for k in rows[0]["ms"]})
    (WORK / "native.json").write_text(json.dumps(result, indent=2) + "\n")
    return result


def archive_cases():
    with tarfile.open(OUT / "sample-cases.tar.gz", "w:gz") as archive:
        for path in sorted((WORK / "cases").rglob("*")):
            if path.name in ("maps", "samples.bin", "watch.log"):
                archive.add(path, arcname=str(path.relative_to(WORK)))
        for arm in ARMS:
            archive.add(WORK / f"{arm}.symbols", arcname=f"{arm}.symbols")


def reanalyze():
    with tarfile.open(OUT / "sample-cases.tar.gz") as archive:
        archive.extractall(WORK, filter="data")
    with (OUT / "measurements.jsonl").open() as stream:
        rows = [json.loads(line) for line in stream]
    norm_pc = {int(line, 16) for line in (D / "normalization-pcs.txt").read_text().splitlines()}
    result = summarize(rows, norm_pc)
    (OUT / "attribution-verified-pcs.json").write_text(json.dumps(result, indent=2) + "\n")


def main():
    os.sched_setaffinity(0, {2})
    copy2(prior.residual.TABLES["vanilla"], WORK / "vanilla.symbols")
    instructions()
    orders = list(permutations(ARMS))
    blocks = [(0, order) for order in orders * 10]
    blocks += [(period, order) for period in (93000, 227000) for order in orders * 9]
    Random(65730).shuffle(blocks)
    rows = []
    with (WORK / "measurements.jsonl").open("w") as output:
        for block, (period, order) in enumerate(blocks):
            for arm in order:
                for row in base.watch(WORK / "cases" / f"{block:03d}-{arm}", arm, STATES, period=period):
                    rows.append(dict(row, block=block))
                    print(json.dumps(rows[-1]), file=output, flush=True)
            print(f"block {block+1}/{len(blocks)} period={period}", flush=True)
    norm_pc = {int(line, 16) for line in (D / "normalization-pcs.txt").read_text().splitlines()}
    (WORK / "attribution.json").write_text(json.dumps(summarize(rows, norm_pc), indent=2) + "\n")
    prior.validate_pdf()
    archive_cases()
    result = run_native()
    print("timing vanilla", result["median_ms"], flush=True)
    with tarfile.open(OUT / "native-traces.tar.gz", "w:gz") as archive:
        for path in sorted((WORK / "vanilla-traces").rglob("*")):
            if path.name.startswith("timings-") or path.name in ("clock.bin", "watch.log"):
                archive.add(path, arcname=str(path.relative_to(WORK)))
    direct = load("direct", D / "direct.py")
    direct.run(prior)
    direct.run_vanilla(prior)


if __name__ == "__main__":
    (reanalyze if sys.argv[1:] == ["--reanalyze"] else main)()
