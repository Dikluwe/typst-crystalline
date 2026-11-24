"""Reduce watch profiles and bracketed phase intervals into disjoint buckets."""
from collections import Counter, defaultdict
import importlib.util
import json
from pathlib import Path
from statistics import median
import sys
import tarfile

D = Path(__file__).resolve().parent
source = D.parent / "references-500-visible-edit-phases-2026-11-21" / "analyze.py"
spec = importlib.util.spec_from_file_location("previous_analysis", source)
base = importlib.util.module_from_spec(spec)
spec.loader.exec_module(base)
ARMS = ("candidate", "hybrid", "vanilla")
TEKT_PRIORITY = {
    "eval": "eval",
    "realize_context": "realization",
    "substitute_context": "realization",
    "physical_root": "physical",
    "outline": "outline",
    "source_plan": "source_plan",
    "index": "index",
    "shape_document": "shaping",
    "memo_overhead": "memo",
    "paged": "paged_other",
}
VANILLA_PRIORITY = {
    "eval": "eval",
    "realize": "realization",
    "layout_document": "physical",
    "pdf": "pdf",
    "compile_pdf": "compile_other",
}
PAGED = {"eval", "realization", "physical", "outline", "source_plan", "index",
         "shaping", "memo", "paged_other"}


def category(arm, stack):
    def has(*names):
        return any(name in frame for name in names for frame in stack)
    if arm == "vanilla":
        if has("typst_eval::eval", "typst_eval::eval_string"):
            # When evaluating a show recipe this stack still belongs to realization.
            if not has("typst_realize::realize"):
                return "eval"
        if has("typst_realize::realize"):
            return "realization"
        if has("typst_layout::pages::layout_document"):
            return "physical"
        if has("typst_pdf::pdf", "krilla::"):
            return "pdf"
        return "other"
    if has("::eval_to_module_in_session"):
        return "eval"
    if has("::substitute_context_blocks"):
        return "context_substitution"
    if has("::context_stabilization::Session::realize"):
        return "context_execution"
    if has("::physical_memo::lab_retain", "::convergence::run_planned", "::layout_planned_root"):
        return "physical"
    if has("::prepare_outline_candidate"):
        return "outline"
    if has("::prepare_running_body", "::prepare_page_running_body"):
        return "source_plan"
    if has("::index_physical_steps"):
        return "index"
    if has("::shape_document_with_faces"):
        return "shaping"
    if has("::physical_memo::layout_with"):
        return "memo"
    if has("::compile_paged_in_session"):
        return "paged_other"
    if has("::compile_to_pdf_bytes_impl"):
        return "pdf_and_wrapper"
    return "other"


def summarize(rows, places="phase_ms"):
    keys = sorted({key for row in rows for key in row[places]})
    return {k:base.stats([row[places].get(k,0) for row in rows]) for k in keys}


def paired(rows, key, other, place="phase_ms"):
    amounts = {arm:{row["block"]:row[place].get(key,0) for row in rows if row["arm"] == arm}
               for arm in ("candidate", other)}
    return base.stats([ms - amounts[other][block] for block,ms in amounts["candidate"].items()])


def attributed(root):
    rows = [json.loads(line) for line in (root / "measurements.jsonl").read_text().splitlines()]
    assert len(rows) == 96
    twin = root.parent / "references-500-visible-page-operator-cost-2026-11-21"
    assert (twin / "vanilla-symbols-verified").is_file()
    assert (root / "binaries.sha256").read_text().splitlines()[2] == (
        twin / "binaries.sha256").read_text().splitlines()[2]
    tables = {arm:base.symbols(root / f"{arm}.symbols") for arm in ARMS[:2]}
    tables["vanilla"] = base.symbols(twin / "vanilla-symbols-twin.txt")
    profiles = []
    all_stacks = {arm:Counter() for arm in ARMS}
    with tarfile.open(root / "profiles.tar.gz", "r:gz") as archive:
        for row in rows:
            if not row["period_ns"]:
                continue
            stacks, detail = base.stacks(archive, row, tables)
            histogram = Counter()
            for stack, n in stacks.items():
                kind = category(row["arm"], stack)
                histogram[kind] += n
                all_stacks[row["arm"]][(kind, tuple(f for f in stack if "typst" in f)[:12])] += n
            factor = detail["main_ns"] / 1e6 / stacks.total()
            phase = {key:n * factor for key,n in histogram.items()}
            if row["arm"] != "vanilla":
                phase["realization"] = phase.get("context_substitution", 0) + phase.get("context_execution", 0)
            phase["paged_and_eval"] = sum(ms for key,ms in phase.items() if key in PAGED)
            phase["other_paged_combined"] = sum(phase.get(k, 0) for k in (
                "outline", "source_plan", "index", "shaping", "memo", "paged_other"
            ))
            profiles.append(dict(row, **detail, phase_ms=phase))
    with (root / "phase-stacks.txt").open("w") as output:
        for arm, counts in all_stacks.items():
            print(arm, file=output)
            for (kind, stack), n in counts.most_common(70):
                print(n, kind, stack, file=output)
    return dict(
        profiles=profiles,
        nominal={arm:base.stats([r["cpu_ns"]/1e6 for r in rows if r["arm"]==arm and not r["period_ns"]])
                 for arm in ARMS},
        nominal_paired={arm:paired([dict(row, phase_ms={"cpu":row["cpu_ns"]/1e6})
                                   for row in rows if not row["period_ns"]], "cpu", arm)
                        for arm in ARMS[1:]},
        profiled_paired={arm:{label:paired(profiles,label,arm) for label in (
                               ("paged_and_eval","eval","realization","physical","pdf_and_wrapper")
                               if arm == "hybrid" else ("paged_and_eval","realization","physical"))}
                         for arm in ARMS[1:]},
        pooled={arm:summarize([r for r in profiles if r["arm"]==arm]) for arm in ARMS},
        by_period={str(p):{arm:summarize([r for r in profiles if r["arm"]==arm and r["period_ns"]==p])
                           for arm in ARMS} for p in (577000,937000)},
    )


def exclusive(spans, priority):
    """Use each thread clock separately and assign overlaps to the inner phase."""
    per_thread = defaultdict(list)
    for name, intervals in spans.items():
        if name in priority:
            for tid, begin, end in intervals:
                assert begin <= end,(name,begin,end)
                per_thread[tid].extend(((begin, name, 1), (end, name, -1)))
    sums = Counter()
    for entries in per_thread.values():
        active = Counter()
        prior = 0
        for now, name, change in sorted(entries):
            phase = next((name for name in priority if active[name] > 0), None)
            if phase:
                sums[priority[phase]] += now-prior
            active[name] += change
            prior = now
        assert not any(active.values()), active
    return {key:v/1e6 for key,v in sums.items()}


def bracketed(root):
    rows = [json.loads(line) for line in (root / "scopes.jsonl").read_text().splitlines()]
    assert len(rows) == 24
    for row in rows:
        spans = row["spans"]
        vanilla = row["arm"] == "vanilla"
        row["phase_ms"] = phase = exclusive(spans, VANILLA_PRIORITY if vanilla else TEKT_PRIORITY)
        row["counts"] = {key:len(v) for key,v in spans.items()}
        row["threads"] = {key:len({interval[0] for interval in v}) for key,v in spans.items()}
        if vanilla:
            phase["layout_document_inclusive"] = sum(b-a for _,a,b in spans["layout_document"])/1e6
        phase["pre_pdf_scoped"] = sum(phase.get(k,0) for k in PAGED)
        if not vanilla:
            phase["compile_envelope"] = sum(b-a for _,a,b in spans["compile_pdf"])/1e6
            phase["pdf_and_wrapper"] = phase["compile_envelope"] - phase["pre_pdf_scoped"]
    return dict(rows=rows,
                medians={arm:summarize([r for r in rows if r["arm"]==arm]) for arm in ARMS},
                paired_control={key:paired(rows,key,"hybrid")
                                for key in ("eval","realization","physical","pre_pdf_scoped")})


if __name__ == "__main__":
    root = Path(sys.argv[1])
    for name, result in (("sample-phases", attributed(root)),):
        (root / f"{name}.json").write_text(json.dumps(result, indent=2) + "\n")
    if (root / "scopes-end.txt").exists():
        (root / "bracketed-phases.json").write_text(json.dumps(bracketed(root), indent=2) + "\n")
