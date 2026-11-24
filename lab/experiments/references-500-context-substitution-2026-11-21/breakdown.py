"""Subdivide the archived edit profile without timing another publication."""
from collections import Counter
from importlib.util import module_from_spec, spec_from_file_location
import json
from pathlib import Path
from random import Random
from statistics import mean, median
import sys
import tarfile

D = Path(__file__).resolve().parent
spec = spec_from_file_location("phases", D.parent / "references-500-visible-pre-pdf-2026-11-21/analyze.py")
phases = module_from_spec(spec)
spec.loader.exec_module(phases)

# Each arm/phase uses the first matching stack ancestor. The parent labels are
# unchanged from bench.py, so the earlier reduction can be checked exactly.
TEKT = {
    "physical": {
        "line_commit": ("::line_commit::",),
        "link_resolution": ("::link_capture::",),
        "position_projection": ("::position_marker::",),
        "paragraph_memo": ("::paragraph_session::", "::paragraph_identity"),
        "page_commit": ("::page_commit::",),
        "paragraph_work": ("::paragraph::", "::paragraph_plan::", "::paragraph_emit::", "::paragraph_paint::"),
    },
    "paged_other": {
        "bidi": ("::layout_bidi::", "::bidi_context::"),
        "line_reconciliation": ("::shaper::reconciliation::", "::fix_line_positions"),
        "charge_retained_product": ("::frame_charge", "::physical_memo::style_charge", "::physical_memo::lab_remember"),
        "content_mapping": ("::entities::content::traversal::",),
    },
    "context_execution": {
        "reserve_locations": ("::reserve_locations",),
        "introspection": ("::compiler::introspect::", "::running_topology"),
        "eval_realize": ("::compiler::realize::", "::compiler::eval::"),
    },
    "index": {
        "reserve_locations": ("::reserve_locations",),
        "page_running_index": ("::introspect_with_page_running",),
        "content_mapping": ("::entities::content::traversal::",),
    },
    "outline": {
        "reserve_locations": ("::reserve_locations",),
        "outline_realization": ("::outline_execution", "::rules::outline::"),
        "introspection": ("::compiler::introspect::",),
    },
    "source_plan": {
        "source_walk": ("::compiler::introspect::",),
        "content_mapping": ("::entities::content::traversal::",),
    },
    "shaping": {
        "lookup": ("::complete_session::Key::same", "::complete_session::Request::lookup"),
        "rustybuzz": ("rustybuzz::", "harfrust::"),
    },
    "pdf_and_wrapper": {
        "page_deflate": ("::serialize_page_stream",),
        "tagging": ("::tag_tree::", "::tag_structure::", "::tag_state::", "::export::tags::"),
        "font_preparation": ("::font_binding::", "::subset::", "::builder::fonts::prepare",
                             "::builder::fonts::Distributor", "subsetter::"),
        "operators": ("::export::stream::draw::", "::build_page_stream", "::emit_frame"),
    },
}
VANILLA = {
    "physical": {
        "finalize_pages": ("::pages::finalize::",),
        "paragraph": ("::inline::",),
        "flow": ("::flow::",),
        "introspection": ("::introspect::",),
    },
    "pdf": {
        "deflate": ("::deflate",),
        "fonts": ("::font::", "subsetter::"),
        "tagging": ("::tagging::", "::tag_tree::"),
    },
}


def detailed(arm, category, stack):
    groups = VANILLA if arm == "vanilla" else TEKT
    for label, tokens in groups.get(category, {}).items():
        if any(token in frame for token in tokens for frame in stack):
            return label
    return "other"


def fresh_profile(out):
    from bench import WORK, STATES, watch
    import os

    os.sched_setaffinity(0, {2})
    arms = ("retained", "vanilla")
    orderings = [(period, order) for period in (347000, 937000)
                 for order in (arms, arms[::-1]) for _ in range(3)]
    Random(78103).shuffle(orderings)
    rows = []
    for block, (period, order) in enumerate(orderings):
        for arm in order:
            directory = WORK / "profile-cases" / f"{block:03d}-{arm}"
            for row in watch(directory, arm, STATES, period=period):
                rows.append(dict(row, block=block))
        print(f"fresh profile {block + 1}/{len(orderings)}", flush=True)
    (out / "breakdown-fresh-measurements.jsonl").write_text(
        "".join(json.dumps(row) + "\n" for row in rows))
    raw = out / "breakdown-fresh-cases.tar.gz"
    with tarfile.open(raw, "w:gz") as archive:
        for path in sorted((WORK / "profile-cases").rglob("*")):
            if path.name in ("maps", "samples.bin", "watch.log"):
                archive.add(path, arcname=str(path.relative_to(WORK)))
    return rows, raw


def main(out, retained_symbols, mode=None):
    base = phases.base
    if mode == "fresh":
        rows, raw = fresh_profile(out)
        prior = None
    else:
        prior = json.loads((out / "phases.json").read_text())["profile"]
        rows = [json.loads(line) for line in (out / "measurements.jsonl").read_text().splitlines()]
        raw = out / "cases.tar.gz"
    vanilla_symbols = (out.parent / "references-500-visible-page-operator-cost-2026-11-21"
                        / "vanilla-symbols-twin.txt")
    tables = {"retained": base.symbols(retained_symbols), "vanilla": base.symbols(vanilla_symbols)}
    cases, hot = {arm: [] for arm in tables}, {arm: Counter() for arm in tables}
    with tarfile.open(raw) as archive:
        for row in rows:
            arm = row["arm"]
            if arm not in tables or not row["period_ns"] or row["state"] != "visible":
                continue
            stacks, detail = base.stacks(archive, row, tables)
            counts, breakdown = Counter(), Counter()
            for stack, n in stacks.items():
                if any("::substitute_context_blocks" in f or "::substitution::" in f for f in stack):
                    phase = "substitution"
                else:
                    phase = phases.category(arm, stack)
                label = detailed(arm, phase, stack)
                counts[phase] += n
                breakdown[(phase, label)] += n
                visible = tuple(f for f in stack if f.startswith(("typst", "krilla", "<typst", "subsetter")))[:5]
                hot[arm][phase, label, visible] += n
            scale = detail["main_ns"] / detail["samples"] / 1e6
            if prior is not None:
                observed = prior[arm][str(row["block"])]
                assert detail == {k: observed[k] for k in detail}
                assert {k:n*scale for k,n in counts.items()} == observed["ms"]
            groups = {}
            for (phase, label), n in breakdown.items():
                groups.setdefault(phase, {})[label] = n * scale
            cases[arm].append(dict(block=row["block"], period_ns=row["period_ns"],
                                   process_ms=detail["process_ns"]/1e6, **detail,
                                   groups=groups))
    summary = {}
    for arm, samples in cases.items():
        assert len(samples) == (12 if mode == "fresh" else 24)
        keys = sorted({(p,k) for r in samples for p,g in r["groups"].items() for k in g})
        aggregate = {}
        for phase, label in keys:
            values = [r["groups"].get(phase, {}).get(label, 0) for r in samples]
            aggregate.setdefault(phase, {})[label] = dict(mean=mean(values), median=median(values))
        summary[arm] = aggregate
    (out / "breakdown.json").write_text(json.dumps(dict(mode=mode or "archived", summary=summary, cases=cases), indent=2) + "\n")
    with (out / "breakdown-stacks.txt").open("w") as dest:
        for arm, counts in hot.items():
            print(arm, file=dest)
            for (phase, label, stack), n in counts.most_common(150):
                print(n, phase, label, stack, file=dest)


if __name__ == "__main__":
    main(Path(sys.argv[1]), Path(sys.argv[2]), *sys.argv[3:])
