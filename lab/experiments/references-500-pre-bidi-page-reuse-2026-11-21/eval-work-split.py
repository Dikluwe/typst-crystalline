"""Separate AST evaluation, discovery and realization in archived evaluation scopes."""
from collections import Counter
from importlib.util import module_from_spec, spec_from_file_location
import json
from pathlib import Path
from statistics import mean
import sys
import tarfile

D = Path(__file__).resolve().parent
prior_experiment = D.parent / "references-500-visible-eval-residual-2026-11-21"
sys.path.insert(0, str(prior_experiment))
spec = spec_from_file_location("residual_eval", prior_experiment / "analyze.py")
residual = module_from_spec(spec)
spec.loader.exec_module(residual)
base = residual.base

MARKUP = ("::markup_stream::eval_markup", "::markup_stream::eval_markup_in_scope")
CONTENT_MAP = ("::map_content", "::map_children_if_changed")
PREPARATION = ("::needs_preparation", "::needs_capture")


def has(stack, *tokens):
    return any(token in frame for token in tokens for frame in stack)


def classify(stack, old_label):
    if not has(stack, "::eval_to_module_in_session"):
        return None
    if has(stack, "::compiler::realize::realize_document"):
        if has(stack, "::compiler::introspect::"):
            detail = "introspection"
        elif has(stack, "::recipes::"):
            detail = "recipes"
        elif has(stack, "::finish_occurrences", "::observe_occurrences", "::normalize_insertions", "::wrap_page_columns"):
            detail = "occurrences_and_normalization"
        else:
            detail = "other"
        return "early_realization", detail
    if has(stack, *MARKUP, "::expression_dispatch::eval_expr"):
        if has(stack, "::capture_table_insertion", "::table_cells::needs_preparation"):
            detail = "inspect_inserted_table"
        elif old_label == "find_memo" or has(stack, "::subtree_session::body_reads", "::subtree_session::spans",
                                            "::subtree_session::Memo::", "::subtree_session::Key::"):
            detail = "loop_memo"
        elif has(stack, "::LinkedNode::find"):
            detail = "find_span"
        elif has(stack, "::operators::join::"):
            detail = "join"
        elif has(stack, "::append_markup_sequence"):
            detail = "append_markup"
        elif has(stack, "::StyleChain::collapse", "::TextualSnapshot", "::close_style_tail"):
            detail = "snapshot_and_styles"
        elif has(stack, "::control_flow::run_for_loop", "::control_flow::eval_for"):
            detail = "other_under_loop"
        else:
            detail = "other"
        return "markup", detail
    if has(stack, *PREPARATION):
        return "discover_preparation", "bare_math" if has(stack, "::bare_math::") else "other"
    if has(stack, *CONTENT_MAP) and not has(stack, "::SubtreeSession::evaluate", "::eval_with_subtree_session"):
        return "asset_check", "map_content"
    return "other", "unclassified"


def analyze(root, samples, symbols, expected, annotated=False):
    rows = [r for line in (root / "measurements.jsonl").read_text().splitlines()
            if (r := json.loads(line))["arm"] == "candidate" and r["period_ns"]
            and r.get("state","visible") == "visible"]
    table = {"candidate":base.symbols(symbols)}
    orig_resolver = base.resolver
    if annotated:
        base.resolver = residual.annotated
    records, hot = [], Counter()
    try:
        with tarfile.open(root / samples) as archive:
            for row in rows:
                stacks, meta = base.stacks(archive, row, table)
                scale = meta["main_ns"]/meta["samples"]/1e6
                counts = Counter()
                for stack, n in stacks.items():
                    group = classify(stack, residual.category("candidate",stack) if annotated else None)
                    if group is None:
                        continue
                    counts[group] += n
                    interesting = tuple(name for name in stack if "typst_" in name)[:10]
                    hot[(group,interesting)] += n
                assert abs(counts.total()*scale-expected[str(row["block"])]) < 1e-9
                records.append(dict(block=row["block"], period_ns=row["period_ns"],
                                    counts={"/".join(k):n for k,n in counts.items()},
                                    ms={"/".join(k):n*scale for k,n in counts.items()},
                                    eval_ticks=counts.total(), **meta))
    finally:
        base.resolver = orig_resolver
    groups = ("markup", "discover_preparation", "early_realization", "asset_check", "other")
    totals = {}
    for group in groups:
        values = [sum(value for key,value in r["ms"].items() if key.startswith(group+"/"))
                  for r in records]
        totals[group] = residual.mean_ci(values)
        totals[group]["ticks"] = sum(sum(n for k,n in r["counts"].items() if k.startswith(group+"/"))
                                      for r in records)
    keys = sorted({key for r in records for key in r["ms"]})
    subgroups = {k:dict(mean=mean(r["ms"].get(k,0) for r in records),
                       ticks=sum(r["counts"].get(k,0) for r in records)) for k in keys}
    return dict(n=len(records), groups=totals, subgroups=subgroups, records=records), hot


def main(out, symbols):
    prior = out.parent / "references-500-visible-eval-residual-2026-11-21"
    current = json.loads((out / "flow-profile.json").read_text())
    older = json.loads((prior / "timing.json").read_text())
    expected_current = {b:r["ms"]["eval"] for b,r in current["cases"]["candidate"].items()}
    expected_older = {str(r["block"]):r["ms"]["eval"] for r in older["timings"] if r["arm"] == "candidate"}
    result = {}
    with (out / "eval-work-split-stacks.txt").open("w") as output:
        for name, args in (
            ("current", (out, "sample-cases.tar.gz", symbols, expected_current, False)),
            ("older", (prior, "cases.tar.gz", prior / "candidate.symbols", expected_older, True)),
        ):
            result[name], hot = analyze(*args)
            print(name,file=output)
            for (group,stack), n in hot.most_common(120):
                print(n,group,stack,file=output)
    assert result["current"]["n"] == 12 and result["older"]["n"] == 36
    result["vanilla_native"] = older["eval_anchor"]
    result["vanilla_current_sampled"] = current["means"]["vanilla"]["eval"]
    (out / "eval-work-split.json").write_text(json.dumps(result, indent=2) + "\n")
    for name in ("current","older"):
        keys = ("groups","subgroups")
        print(name,json.dumps({k:result[name][k] for k in keys},indent=2))


if __name__ == "__main__":
    main(*(Path(arg) for arg in sys.argv[1:]))
