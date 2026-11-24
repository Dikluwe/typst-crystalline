"""Separate the native estimator's errors from the final layout's named pages."""
import json
import math
from pathlib import Path
import re
import sys

src, dst = map(Path, sys.argv[1:])
cases = json.loads(src.read_text())
data = {}
for key, rec in sorted(cases.items()):
    trace = rec["trace"]["omit"]
    estimate, = (json.loads(row.split("mode=metrics ", 1)[1]) for row in trace["seed_events"]
                 if "mode=metrics" in row)
    actual = dict((int(section), dict(page=int(page), y_pt=float(y))) for section,page,y in
                  re.findall(r'\("s(\d+)", (\d+), Some\(Point \{ x: Pt\([^)]*\), y: Pt\(([^)]+)\)',
                             trace["positions"][0]))
    assert sorted(actual) == list(range(len(estimate))) == list(range(20)), key
    mismatches = [dict(section=i, predicted=page, actual=actual[i]["page"],
                       actual_y_pt=actual[i]["y_pt"], delta=page-actual[i]["page"])
                  for i,page in enumerate(estimate) if page != actual[i]["page"]]
    validated, = (int(re.search(r"validation=(\d+)/20", row)[1])
                   for row in trace["seed_events"] if "validation=" in row)
    assert validated == 20 - len(mismatches), (key, validated, mismatches)
    # The fixture width is part of the grid. The emitted native steps are
    # parsed for this diagnosis, rather than chosen from an earlier PDF.
    metrics, = trace["native_steps"]
    steps = {key: float(value) for key,value in re.findall(r'(body|outline|title)=([0-9.]+)',metrics)}
    slots = math.floor((rec["height"] - 2 * min(400,rec["height"]) * 2.5 / 21.0) / steps["body"])
    data[key] = dict(height=rec["height"], paragraphs_per_section=rec["count"], slots=slots,
                     native_steps_pt=steps, valid=validated, errors=mismatches,
                     predicted=estimate, actual=[actual[i]["page"] for i in range(len(estimate))])
assert len(data)==16 and sum(bool(x["errors"]) for x in data.values())==14
dst.write_text(json.dumps(data,ensure_ascii=False,indent=2)+"\n")
