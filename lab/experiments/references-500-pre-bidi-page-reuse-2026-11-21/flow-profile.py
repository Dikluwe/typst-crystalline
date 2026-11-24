"""Attribute exclusive CPU phases of the already measured original edit."""
from collections import Counter
from importlib.util import module_from_spec, spec_from_file_location
import json
from pathlib import Path
from statistics import mean
import sys
import tarfile

D = Path(__file__).resolve().parent
spec = spec_from_file_location(
    "phases", D.parent / "references-500-visible-pre-pdf-2026-11-21/analyze.py")
phases = module_from_spec(spec)
spec.loader.exec_module(phases)
ARMS = ("candidate", "vanilla")
FINAL = {
    "bidi": ("typst_infra::layout_bidi::", "typst_infra::bidi_context::Registry::"),
    "shape": ("::shape_document_with_faces",),
    "reconcile": ("::shaper::reconciliation::",),
    "proof_and_retention": ("::pre_bidi::",),
}
GROUPS = {
    "eval": ("eval",),
    "realization": ("realization", "context_execution", "substitution"),
    "physical": ("physical",),
    "other_paged": ("outline", "source_plan", "index", "memo", "paged_other"),
    "post_layout": tuple(FINAL),
    "pdf": ("pdf", "pdf_and_wrapper"),
    "other_main": ("other",),
}
KEYS = (*GROUPS, "other_threads", "pre_pdf", "total")


def has(stack, *tokens):
    return any(token in frame for token in tokens for frame in stack)


def classify(arm, stack):
    if arm == "candidate":
        for label, tokens in FINAL.items():
            if has(stack, *tokens):
                return label
        if has(stack, "::substitute_context_blocks", "::substitution::"):
            return "substitution"
    return phases.category(arm, stack)


def main(out, candidate_path):
    base = phases.base
    twin = out.parent / "references-500-visible-page-operator-cost-2026-11-21"
    assert (twin / "vanilla-symbols-verified").is_file()
    assert next(line.split()[0] for line in (twin/"binaries.sha256").read_text().splitlines()
                if line.split()[1] == "vanilla") == next(
        line.split()[0] for line in (out/"binaries.sha256").read_text().splitlines()
        if line.split()[1] == "vanilla")
    tables = {
        "candidate": base.symbols(candidate_path),
        "vanilla": base.symbols(twin / "vanilla-symbols-twin.txt"),
    }
    prior = json.loads((out / "timing.json").read_text())
    rows = (json.loads(line) for line in (out / "measurements.jsonl").read_text().splitlines())
    results = {arm:{} for arm in ARMS}
    observed_keys = {key for keys in GROUPS.values() for key in keys}
    with tarfile.open(out / "sample-cases.tar.gz") as archive:
        for row in rows:
            arm, block = row["arm"], str(row["block"])
            if arm not in ARMS or row["state"] != "visible" or not row["period_ns"]:
                continue
            stacks, detail = base.stacks(archive, row, tables)
            counts = Counter()
            inline = Counter()
            for stack, n in stacks.items():
                phase = classify(arm, stack)
                assert phase in observed_keys, phase
                counts[phase] += n
                if arm == "vanilla" and phase == "physical":
                    if has(stack, "typst_layout::inline::prepare::", "typst_layout::inline::shaping::"):
                        inline["prepare_or_shape"] += n
                    elif has(stack, "typst_layout::inline::"):
                        inline["other_inline"] += n
            scale = detail["main_ns"]/detail["samples"]/1e6
            raw_ms = {key:count*scale for key,count in counts.items()}
            groups = {
                group:sum(raw_ms.get(key,0) for key in keys)
                for group,keys in GROUPS.items()
            }
            groups["other_threads"] = (detail["process_ns"]-detail["main_ns"])/1e6
            groups["pre_pdf"] = sum(groups[k] for k in (
                "eval", "realization", "physical", "other_paged", "post_layout"))
            groups["total"] = detail["process_ns"]/1e6
            assert abs(sum(groups[k] for k in (*GROUPS,"other_threads")) - groups["total"]) < 1e-8
            if arm == "candidate":
                prior_row = prior["profile_cases"][arm][block]
                assert all(prior_row[k] == v for k,v in detail.items())
                assert all(prior_row[k] == raw_ms.get(k,0) for k in FINAL)
            results[arm][block] = dict(**detail, period_ns=row["period_ns"], ms=groups,
                                        scopes_ms=raw_ms, inline_ms={k:n*scale for k,n in inline.items()})
    assert set(results["candidate"]) == set(results["vanilla"]) and len(results["candidate"]) == 12
    means = {arm:{k:mean(r["ms"][k] for r in results[arm].values()) for k in KEYS} for arm in ARMS}
    paired = {k:base.stats([r["ms"][k]-results["vanilla"][b]["ms"][k]
                           for b,r in results["candidate"].items()]) for k in KEYS}
    scope_means = {arm:{k:mean(r["scopes_ms"].get(k,0) for r in results[arm].values())
                       for k in sorted({k for r in results[arm].values() for k in r["scopes_ms"]})}
                   for arm in ARMS}
    inline_means = {k:mean(r["inline_ms"].get(k,0) for r in results["vanilla"].values())
                    for k in ("prepare_or_shape","other_inline")}
    output = dict(means=means, paired=paired, scopes=scope_means, vanilla_inline=inline_means,
                  nominal=prior["nominal"]["visible"], cases=results)
    (out / "flow-profile.json").write_text(json.dumps(output,indent=2)+"\n")
    print(json.dumps({k:output[k] for k in ("means","scopes","vanilla_inline")},indent=2))


if __name__ == "__main__":
    main(*(Path(arg) for arg in sys.argv[1:]))
