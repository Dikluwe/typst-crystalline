"""Sample only the visible watch publication with the established CPU timer."""
import importlib.util
from collections import Counter
import json
import os
from pathlib import Path
from random import Random
from statistics import median
import sys
import tarfile

D = Path(__file__).resolve().parent
prior = D.parent / "references-500-visible-page-operator-cost-2026-11-21"


def load(name):
    spec = importlib.util.spec_from_file_location(name, prior / f"{name}.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def measure():
    sample = load("sample")
    os.sched_setaffinity(0, {2})
    rng = Random(50077)
    periods = [0] * 8 + [577000] * 8 + [937000] * 8
    rng.shuffle(periods)
    with (sample.WORK / "measurements.jsonl").open("w") as output:
        for block, period in enumerate(periods):
            for arm in rng.sample(sample.ARMS, len(sample.ARMS)):
                row = sample.run(block, arm, period)
                assert row["pdf_hash"] == sample.cycle.gold(arm, "visible"), row
                print(json.dumps(row), file=output, flush=True)
            print(f"profile {block+1}/{len(periods)} {period}", flush=True)


def analyze(root):
    analysis = load("analyze")
    prior_result = root.parent / prior.name
    assert (prior_result / "vanilla-symbols-verified").is_file()
    assert (root / "binaries.sha256").read_text().splitlines()[2] == (
        prior_result / "binaries.sha256").read_text().splitlines()[2]
    tables = {arm:analysis.symbols(root / f"{arm}.symbols") for arm in ("candidate", "hybrid")}
    tables["vanilla"] = analysis.symbols(prior_result / "vanilla-symbols-twin.txt")

    def category(arm, stack):
        if arm == "candidate" and any("::" + function in frame for frame in stack for function in
                ("collect_page_fonts", "collect_document_fonts", "merge_page_fonts", "compact_clusters",
                 "collect_face_scan", "partition_pages")):
            return "fonts"
        return analysis.category(arm, stack)

    original_resolver = analysis.resolver
    analysis.resolver = lambda maps, exe, table, arm: original_resolver(
        maps, exe, table, "hybrid" if arm == "candidate" else arm
    )
    raw = [json.loads(line) for line in (root / "measurements.jsonl").read_text().splitlines()]
    assert len(raw) == 72 and len({row["block"] for row in raw}) == 24
    samples = []
    with tarfile.open(root / "profiles.tar.gz", "r:gz") as archive:
        for row in raw:
            if not row["period_ns"]:
                continue
            counts, details = analysis.stacks(archive, row, tables)
            phases = Counter()
            for stack, count in counts.items():
                phases[category(row["arm"], stack)] += count
            factor = details["main_ns"] / 1e6 / counts.total()
            estimates = {phase:count*factor for phase,count in phases.items()}
            estimates["pdf_total"] = sum(ms for phase,ms in estimates.items() if phase != "outside_pdf")
            selected = {phase: estimates.get(phase, 0)
                        for phase in ("fonts", "pdf_total", "outside_pdf")}
            samples.append(dict(row, **details, phase_ms=selected))
    classes = ("fonts", "pdf_total", "outside_pdf")
    out = {"samples": samples, "phase_medians_ms": {}, "paired_phase_ms": {}}
    groups = {arm:[row for row in samples if row["arm"] == arm] for arm in tables}
    for arm, rows in groups.items():
        assert len(rows) == 16
        out["phase_medians_ms"][arm] = {
            phase:median(r["phase_ms"].get(phase,0) for r in rows) for phase in classes}
    for phase in classes:
        values = {arm: {row["block"]:row["phase_ms"][phase] for row in rows}
                  for arm,rows in groups.items()}
        out["paired_phase_ms"][phase] = {}
        for arm in ("hybrid", "vanilla"):
            differences = [value - values[arm][block] for block,value in values["candidate"].items()]
            out["paired_phase_ms"][phase][arm] = analysis.stats(differences)
    (root / "font-phases.json").write_text(json.dumps(out, indent=2) + "\n")
    print(json.dumps({k:v for k,v in out.items() if k != "samples"}, indent=2))


if __name__ == "__main__":
    if len(sys.argv) == 1:
        measure()
    else:
        analyze(Path(sys.argv[1]))
