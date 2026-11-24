"""Check TOC and bookmark destinations in the first complete benchmark block."""
import json
import os
from pathlib import Path
import sys

here = Path(__file__).resolve().parent
sys.path.insert(0, str(here.parent / "references-500-visible-eval-residual-2026-11-21"))
from watch import WORK, cycle

out = Path(os.environ["TEKT_LAB_TIMING_OUT"])
original = cycle.cycle.original
rows = [json.loads(line) for line in (out / "measurements.jsonl").read_text().splitlines()]
first = [row for row in rows if row["block"] == 0]
assert {row["arm"] for row in first} == {"candidate", "control", "vanilla"}
results = {}
for row in first:
    pdf = WORK / row["directory"] / "out.pdf"
    results[row["arm"]] = original.navigation(pdf, original.compare.describe(pdf), True)
(out / "navigation.json").write_text(json.dumps(results, indent=2) + "\n")
