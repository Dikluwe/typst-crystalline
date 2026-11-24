"""Separate producer removal from bypassing the remaining initial realization."""
from collections import Counter
from itertools import permutations
import json
import os
from pathlib import Path
from random import Random
import re
from shutil import copy2
from statistics import mean, median
import sys
import tarfile
import importlib.util


D = Path(__file__).resolve().parent


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


heading = load("reference_proof", D.parent / "references-500-heading-reference-proof-2026-11-21" / "bench.py")
initial, base = heading.initial, heading.base
WORK, OUT, STATES = heading.WORK, heading.OUT, heading.STATES
decoder, native = initial.decoder, initial.native
direct = initial.load("initial_direct", Path(initial.D) / "direct.py")
ARMS = ("candidate", "bridge", "proof", "vanilla")
TEKT_PHASES = ("initial_realization", "eval_rest", "module_index", "context_ref_callback",
               "context_index", "context_execution", "context_substitution", "context_control",
               "physical", "shape", "page_other", "pdf", "watch_other")
VANILLA_PHASES = ("eval", "realization", "physical_index", "layout_rest", "pdf", "watch_other")
PAIRS = (("candidate", "bridge"), ("bridge", "proof"),
         ("candidate", "proof"), ("proof", "vanilla"))


def dump(name, data):
    (OUT / name).write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n")


original_env = base.environment


def environment(tmp, arm, trace):
    env = original_env(tmp, arm, trace)
    if trace and arm == "bridge":
        env["TEKT_LAB_HEADING_REF_TRACE"] = "1"
    return env


base.environment = environment
heading.environment = environment


def run_watch(folder, arm, gold, *, trace=False, period=0, keep=True):
    rows = base.watch(folder, arm, STATES, trace=trace, period=period, keep=True)
    assert all(row["pdf_hash"] == gold[arm][row["state"]] for row in rows), (arm, folder, rows)
    if not keep:
        for row in rows:
            (folder / (row["state"] + ".pdf")).unlink()
    return rows


def validate():
    gold = {arm: {state: base.golden("vanilla" if arm == "vanilla" else "candidate", state)
                  for state, *_ in STATES} for arm in ARMS}
    receipts = {}
    for arm in ARMS:
        case = WORK / "validation" / arm
        m = run_watch(case, arm, gold, trace=True)
        receipts[arm] = dict(rows=m)
        for state, *_ in STATES:
            if state != "visible":
                continue
            pdf = case / f"{state}.pdf"
            detail = heading.pdf_details(pdf)
            rasters = base.render(pdf, case, "edition")
            receipts[arm]["visible"] = {k: detail[k] for k in ("pages","links","names","outlines","positions")}
            receipts[arm]["visible"]["rasters"] = rasters
        if arm in ("bridge","proof"):
            flags = heading.watch_flags(case)
            assert flags == ([True, None, None, True] if arm == "bridge" else [False, None, None, False])
            receipts[arm]["initial"] = flags
        if arm == "bridge":
            counts = re.findall(r"^lab-heading-cost context-roots=(\d+)$",
                                (case / "watch.log").read_text(), re.M)
            assert counts == ["0","0"], counts
            receipts[arm]["roots"] = list(map(int, counts))
    assert receipts["candidate"]["visible"] == receipts["bridge"]["visible"] == receipts["proof"]["visible"]
    fig = (b'#figure([FIGURE], kind: image, supplement: it => [CALLBACK], caption: [CAPTION]) <fig>\n'
           b'Figure @fig.\n')
    cases = {arm: heading.compile_case(WORK / "validation" / ("figure-"+arm), arm, fig)
             for arm in ARMS}
    assert all(c["code"] == 0 and "CALLBACK" in c["content"]["text"] for c in cases.values())
    assert len({cases[arm]["qdf"] for arm in ARMS if arm != "vanilla"}) == 1
    assert cases["bridge"]["initial"] == ["true"]
    dump("validation.json", dict(original=receipts, figure=cases))
    print("validated original cycle and Figure callback", flush=True)
    return gold


def orders(n):
    arc = ARMS
    rotations = [arc[i:]+arc[:i] for i in range(len(arc))]
    balanced = [*rotations, *(tuple(reversed(order)) for order in rotations)]
    return balanced * n


def bench_cycles(gold):
    schedule = [(0, order) for order in orders(2)]
    schedule += [(period, order) for period in (97000, 233000) for order in orders(2)]
    Random(17265).shuffle(schedule)
    rows = []
    with (WORK / "measurements.jsonl").open("w") as stream:
        for block, (period, order) in enumerate(schedule):
            for arm in order:
                folder = WORK / "cases" / f"{block:02d}-{arm}"
                for row in run_watch(folder, arm, gold, period=period, keep=False):
                    rows.append(dict(row, block=block))
                    print(json.dumps(rows[-1]), file=stream, flush=True)
            print(f"block {block+1}/{len(schedule)} period={period}", flush=True)
    return rows


def category(arm, stack):
    def has(*names):
        return any(token in frame for token in names for frame in stack)

    if arm == "vanilla":
        if has("typst_layout::introspect::PagedIntrospector::new"):
            return "physical_index"
        if has("typst_realize::realize"):
            return "realization"
        if has("typst_eval::eval"):
            return "eval"
        if has("typst_layout::pages::layout_document", "typst_layout::pages::layout_document_common"):
            return "layout_rest"
        if has("typst_pdf::pdf","krilla::"):
            return "pdf"
        return "watch_other"
    if has("::eval_to_module_in_session"):
        return "initial_realization" if has("::compiler::realize::realize_document") else "eval_rest"
    if has("::reference_supplements::produce"):
        return "context_ref_callback"
    if has("::context_stabilization::Session::realize", "::context_stabilization::Session::execute"):
        if has("::introspect_with_introspector", "::compiler::introspect::walk::"):
            return "context_index"
        return "context_execution"
    if has("::substitute_context_blocks", "::context_stabilization::RenderPath", "::running_topology"):
        return "context_substitution"
    if has("::introspect_with_introspector", "::introspect::bibliography::convert_bib_refs_to_cites",
           "::prepare_heading_supplements") and not has("::context_stabilization"):
        return "module_index"
    if has("::physical_memo::lab_retain", "::convergence::run_planned", "::layout_planned_root",
           "::compiler::layout::physical_memo::layout_with", "::prepare_outline_candidate",
           "::index_physical_steps", "::prepare_running_body"):
        return "physical"
    if has("::context_stabilization::"):
        return "context_control"
    if has("::shaper::traversal::shape_document_with_faces", "::layout_bidi::", "::pre_bidi::",
           "::shaper::reconciliation::"):
        return "shape"
    if has("::compile_paged_in_session"):
        return "page_other"
    if has("::compile_to_pdf_bytes_impl"):
        return "pdf"
    return "watch_other"


def reduce(rows):
    nominal = {arm: {} for arm in ARMS}
    samples = {arm: {} for arm in ARMS}
    hits = Counter()
    tables = {arm: decoder.symbols(WORK / f"{arm}.symbols") for arm in ARMS}
    for line in rows:
        if line["state"] != "visible":
            continue
        arm, block = line["arm"], line["block"]
        if not line["period_ns"]:
            nominal[arm][block] = line["cpu_ns"] / 1e6
            continue
        scopes = TEKT_PHASES if arm != "vanilla" else VANILLA_PHASES
        stacks, detail = decoder.stacks(initial.Scratch(), line, tables)
        count = Counter()
        for stack, n in stacks.items():
            kind = category(arm, stack)
            count[kind] += n
            hits[arm, kind, stack] += n
        assert set(count).issubset(scopes)
        weighted = {k: count[k] * detail["main_ns"] / detail["samples"] / 1e6 for k in scopes}
        samples[arm][block] = dict(**detail, count=dict(count), ms=weighted,
                                   period_ns=line["period_ns"], process_cpu_ns=line["cpu_ns"])
    summaries = {arm: {key: dict(**initial.prior.residual.mean_ci([row["ms"][key] for row in cases.values()]),
                                ticks=sum(row["count"].get(key,0) for row in cases.values()))
                       for key in (TEKT_PHASES if arm != "vanilla" else VANILLA_PHASES)}
                 for arm, cases in samples.items()}
    values = {f"{a}-{b}": [v - nominal[b][block] for block,v in nominal[a].items()]
              for a,b in PAIRS}
    pairs = {key: decoder.stats(delta) for key,delta in values.items()}
    paired_means = {key: initial.prior.residual.mean_ci(delta) for key,delta in values.items()}
    assert abs(paired_means["candidate-bridge"]["mean"] + paired_means["bridge-proof"]["mean"] -
               paired_means["candidate-proof"]["mean"]) < 1e-9
    with (OUT / "phase-stacks.txt").open("w") as stream:
        for (arm, scope, stack), n in hits.most_common():
            print(arm, scope, n, " | ".join(stack), file=stream)
    dump("attribution.json", dict(nominal={arm: decoder.stats(list(v.values())) for arm,v in nominal.items()},
                                  paired=pairs, paired_means=paired_means, paired_values=values,
                                  sample=summaries, sample_cases=samples))
    print("nominal differences", pairs, flush=True)


def archive_cases():
    with tarfile.open(OUT / "sample-cases.tar.gz", "w:gz") as archive:
        for folder in ("cases", "validation"):
            for path in sorted((WORK / folder).rglob("*")):
                if path.name in ("maps","samples.bin","watch.log","compile.log"):
                    archive.add(path, arcname=str(path.relative_to(WORK)))
        for arm in ARMS:
            archive.add(WORK / f"{arm}.symbols", arcname=f"{arm}.symbols")


def native_vanilla():
    gold = json.loads(Path(os.environ["TEKT_VISIBLE_GOLD"]).read_text())["hashes"]["vanilla"]
    records = []
    for i in range(12):
        measured = native.run(i, gold)
        records.append(dict(**initial.vanilla_scopes(WORK / "vanilla-traces" / f"{i:02d}"),
                            original_phases=measured["phase_ms"]))
    dump("native.json", dict(rows=records, median_ms={k: median(r["ms"][k] for r in records)
                                                     for k in records[0]["ms"]},
                              phase_medians={k: median(r["original_phases"][k] for r in records)
                                             for k in records[0]["original_phases"]}))
    with tarfile.open(OUT / "native-traces.tar.gz", "w:gz") as archive:
        for path in sorted((WORK / "vanilla-traces").rglob("*")):
            if path.name.startswith("timings-") or path.name in ("watch.log", "clock.bin"):
                archive.add(path, arcname=str(path.relative_to(WORK)))


def main():
    os.sched_setaffinity(0, {2})
    copy2(initial.prior.residual.TABLES["vanilla"], WORK / "vanilla.symbols")
    gold = validate()
    if "--validate-only" in sys.argv:
        return
    if "--clock-only" not in sys.argv:
        rows = bench_cycles(gold)
        reduce(rows)
        archive_cases()
        native_vanilla()
        direct.run_vanilla(sys.modules[__name__])
    phase_clock = load("phase_clock", D / "phase_clock.py")
    phase_clock.run(sys.modules[__name__], direct, gold)


if __name__ == "__main__":
    main()
