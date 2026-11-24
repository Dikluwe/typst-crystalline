"""Check the complete saved receipts independently of the timing process."""
from collections import Counter
import json
from pathlib import Path
import re
import sys
import tarfile


out = Path(sys.argv[1])
load = lambda name: json.loads((out / name).read_text())
original = load("original.json")
directed = load("directed.json")
target = load("target-edit.json")
assert original["initial"] == [False, None, None, False]
assert original["fresh"]["candidate"]["qdf"] == original["fresh"]["proof"]["qdf"]
assert original["evidence"][-1]["rasters_equal"]
assert all(e["digests"]["candidate"] == e["digests"]["proof"] and all(e["same"].values())
           for e in original["evidence"])
assert all(len(e["links"]["proof"]) == 40 for e in original["evidence"])

def user_diagnostics(log):
    return "\n".join(line for line in log.splitlines()
                     if not line.startswith(("watch-positions ", "watch-memo ", "pdf-")))


for name, case in directed.items():
    assert case["initial"] == [str(case["expected_initial"]).lower()], name
    assert case["status_equal"] and case["qdf_equal"], name
    assert user_diagnostics(case["results"]["candidate"]["diag"]) == \
           user_diagnostics(case["results"]["proof"]["diag"]), name
    assert case["same"] is None or all(case["same"].values()), name
    assert case["token_seen"] is None or all(case["token_seen"].values()), name

assert target["initial"] == [False, False, True, False]
assert all(all(case["same"].values()) for case in target["states"].values())
assert target["ref_change"]["proof"] == target["ref_change"]["candidate"]
for arm, changes in target["ref_change"].items():
    assert sum(changes["removed"].values()) == sum(changes["added"].values()) == 1, arm
    assert "u:s0" in next(iter(changes["removed"])) and "u:s13" in next(iter(changes["added"]))

rows = [json.loads(line) for line in (out / "measurements.jsonl").read_text().splitlines()]
refs = {e["state"]: e["digests"] for e in original["evidence"]}
assert len(rows) == 216 and len({row["block"] for row in rows}) == 18
assert all(row["pdf_hash"] == refs[row["state"]][row["arm"]] for row in rows)

sealed = re.compile(r"Location\((\d+)\): Position \{ page: (\d+), point: Point \{ x: Pt\(([^)]+)\), y: Pt\(([^)]+)\)")
traces = {}
with tarfile.open(out / "validation-logs.tar.gz") as archive:
    for arm in ("candidate", "proof"):
        entries = archive.extractfile(f"original/{arm}/watch.log").read().decode().split("compiled successfully")[:-1]
        data = []
        for entry in entries:
            named, = (line.split(" ", 2)[2] for line in entry.splitlines() if line.startswith("watch-positions "))
            raw, = (line for line in entry.splitlines() if line.startswith("lab-sealed-positions "))
            locs = sealed.findall(raw)
            data.append((named, Counter((int(page), float(x).hex(), float(y).hex()) for _, page, x, y in locs)))
        traces[arm] = data

positions = {}
for e, control, proof in zip(original["evidence"], traces["candidate"], traces["proof"]):
    positions[e["state"]] = dict(named_trace_equal=control[0] == proof[0],
                                 control=control[1].total(), proof=proof[1].total(),
                                 coordinates_removed=(control[1]-proof[1]).total(),
                                 coordinates_added=(proof[1]-control[1]).total())
controls_with_vanilla_difference = [name for name, case in directed.items()
                                  if case["results"]["candidate"]["code"] != case["results"]["vanilla"]["code"]]
result = dict(nominal_pdfs=len(rows), directed_cases=len(directed), position_trace=positions,
              controls_with_vanilla_difference=controls_with_vanilla_difference)
(out / "verified.json").write_text(json.dumps(result, indent=2) + "\n")
print(result)
