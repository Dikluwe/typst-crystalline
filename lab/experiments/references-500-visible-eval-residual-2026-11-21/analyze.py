"""Measure residual evaluation CPU from disjoint stack categories."""
from collections import Counter
import importlib.util
import json
from pathlib import Path
from random import Random
from statistics import mean
import sys
import tarfile

from symbols import COLLAPSES, MARKUP_FIND, TABLES

D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "decoder",D.parent/"references-500-visible-edit-phases-2026-11-21/analyze.py")
base = importlib.util.module_from_spec(spec)
spec.loader.exec_module(base)
ARMS = ("candidate","control","vanilla")
resolver = base.resolver
LABELS = ("collapse_boundary", "collapse_entry", "collapse_other", "collapse_group",
          "collapse_residual", "find_origin", "find_memo", "find_other", "find",
          "concat", "join_other", "join", "target_total", "outside_targets", "eval")


def annotated(maps,exe,table):
    lookup = resolver(maps,exe,table)
    arm = Path(exe).name
    start = next(int(row.split()[0].split("-")[0],16) for row in maps.splitlines()
                 if row.split()[2]=="00000000" and row.endswith(exe))
    tags = dict(COLLAPSES.get(arm,{}))
    if arm in MARKUP_FIND:
        tags[MARKUP_FIND[arm]] = "find_origin"
    def resolve(pc):
        name = lookup(pc)
        label = tags.get(pc-start+1)
        return f"{name} [{label}]" if label else name
    return resolve


def category(arm,stack):
    def has(*tokens):
        return any(token in f for token in tokens for f in stack)
    if arm=="vanilla":
        if "typst_eval::eval" not in stack:
            return None
        if has("LinkedNode::find_number","LinkedNode::find_range"):
            return "find_other"
        if has("foundations::content::Content as core::ops::arith::Add"):
            return "concat"
        if has("foundations::ops::join"):
            return "join_other"
        return "outside_targets"
    if not has("::eval_to_module_in_session"):
        return None
    if has("::style_chain::StyleChain::collapse"):
        if has("[group]","::TextualSnapshot::same_group"):
            return "collapse_group"
        if has("[markup_entry]"):
            return "collapse_entry"
        if has("[markup_boundary]"):
            return "collapse_boundary"
        return "collapse_other"
    if has("::LinkedNode::find"):
        if has("[find_origin]"):
            return "find_origin"
        if has("::control_flow::run_for_loop") and not has("::markup_stream::eval_markup_impl"):
            return "find_memo"
        # A higher-level markup frame can call the loop's memo probes.
        find_index = next(i for i,f in enumerate(stack) if "::LinkedNode::find" in f)
        caller = next((f for f in stack[find_index+1:] if "::LinkedNode::find" not in f),"")
        if "::control_flow::run_for_loop" in caller:
            return "find_memo"
        return "find_other"
    if has("::eval::operators::join::concat_content"):
        return "concat"
    if has("::eval::operators::join::join"):
        return "join_other"
    return "outside_targets"


def mean_ci(values):
    rng = Random(50171)
    draws = sorted(mean(rng.choices(values,k=len(values))) for _ in range(10000))
    return dict(mean=mean(values),ci95=[draws[250],draws[9749]])


def main(root):
    base.resolver = annotated
    rows = [json.loads(l) for l in (root/"measurements.jsonl").read_text().splitlines()]
    assert len(rows)==144 and all(len(r["all_pdfs"])==4 for r in rows)
    assert len({r["pdf_hash"] for r in rows if r["arm"]!="vanilla"})==1
    tables = {arm:base.symbols(path) for arm,path in TABLES.items()}
    profile = []
    with tarfile.open(root/"cases.tar.gz","r:gz") as archive:
        for row in rows:
            if not row["period_ns"]:
                continue
            stacks, detail = base.stacks(archive,row,tables)
            counts = Counter()
            for stack,n in stacks.items():
                label = category(row["arm"],stack)
                if label is not None:
                    counts[label] += n
            counts["eval"] = counts.total()
            counts["collapse_residual"] = sum(counts[k] for k in
                ("collapse_boundary","collapse_entry","collapse_other"))
            counts["find"] = sum(counts[k] for k in ("find_origin","find_memo","find_other"))
            counts["join"] = counts["concat"]+counts["join_other"]
            counts["target_total"] = counts["collapse_residual"] + counts["find"] + counts["join"]
            scale = detail["main_ns"]/stacks.total()/1e6
            profile.append(dict(row, **detail, counts=counts, ms={k:counts[k]*scale for k in LABELS}))
    native = json.loads((root/"vanilla-phases.json").read_text())
    nominal = {arm:{r["block"]:r["cpu_ns"]/1e6 for r in rows if not r["period_ns"] and r["arm"]==arm}
               for arm in ARMS}
    assert all(len(x)==12 for x in nominal.values())
    result = dict(nominal={arm:base.stats(list(vals.values())) for arm,vals in nominal.items()},
                  nominal_paired={arm:base.stats([v-nominal[arm][b] for b,v in nominal["candidate"].items()])
                                  for arm in ARMS[1:]},
                  eval_anchor=base.stats([r["phase_ms"]["eval"] for r in native["rows"]]),
                  estimates={}, timings=profile)
    for arm in ARMS:
        subset = [r for r in profile if r["arm"]==arm]
        assert len(subset)==36
        result["estimates"][arm] = {
            label:{**mean_ci([r["ms"][label] for r in subset]),
                   "median":base.stats([r["ms"][label] for r in subset])["median"],
                   "ticks":sum(r["counts"][label] for r in subset)}
            for label in LABELS}
    candidate = {r["block"]:r["ms"]["eval"] for r in profile if r["arm"]=="candidate"}
    result["tekt_eval_paired"] = base.stats([candidate[r["block"]]-r["ms"]["eval"]
                             for r in profile if r["arm"]=="control"])
    (root/"timing.json").write_text(json.dumps(result,indent=2)+"\n")
    for a,data in result["estimates"].items():
        print(a,{k:(round(v["mean"],3),v["ticks"]) for k,v in data.items()})
    print(result["nominal"],result["nominal_paired"],result["eval_anchor"])


if __name__=="__main__":
    main(Path(sys.argv[1]))
