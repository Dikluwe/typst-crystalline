"""Close P1323 evidence without staging, committing, or changing product inputs."""
import importlib.util
import json
from pathlib import Path
import sys

sys.dont_write_bytecode = True
D = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("record", D / "p1323-record.py")
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
s = r.state()
r.verify(s)
previous = json.loads((D / "p1322-closure.json").read_text())
historical = {**previous["artifacts"], **previous["historical_preserved"]}
assert all(r.sha(path) == value for path, value in historical.items())
ab = json.loads((D / "p1323-ab-process-receipt-r2.json").read_text())
rows = [row for order in ab["rounds"] for row in order["rows"]]
assert len(rows) == 45
assert all(row["checks"][key] == "Preserved" for row in rows for key in ("warning", "preservation"))
assert len(ab["order_checks"]) == 60 and all(row["stable"] for row in ab["order_checks"])
gates = {}
for name in ("candidate-build", "unit-green", "workspace-tests", "fmt", "final-lint", "final-lineage-r2", "final-diff-check-r2"):
    path = D / ("p1323-" + name + ".json")
    receipt = json.loads(path.read_text())
    assert receipt["exit"] == 0
    gates[name] = {"path": str(path), "sha256": r.sha(path), "exit": receipt["exit"]}
report = D / "p1323-final-report.md"
assert "pendente da conclusão" not in report.read_text().lower()
review = D / "p1323-review-final.md"
assert review.is_file()
artifacts = {}
for path in sorted(D.glob("p1323-*")):
    if path.is_file():
        artifacts[str(path)] = r.sha(path)
    elif path.is_dir():
        for child in sorted(path.rglob("*")):
            if child.is_file():
                artifacts[str(child)] = r.sha(child)
# This is the exact plan written in this turn, never a historical-directory scan.
step = r.ROOT / "00_nucleo/materialization/typst-passo-1323.md"
r.save("closure", dict(
    at=r.now(), state=s, manifest_sha256=r.sha(D / "p1323-manifest.json"),
    baseline_sha256=r.sha(D / "p1323-baseline.json"),
    historical_preserved=historical, step_sha256=r.sha(step), gates=gates,
    artifacts=artifacts, report_sha256=r.sha(report), review_sha256=r.sha(review),
    ab={"warning_preserved": 45, "preservation_preserved": 45, "stable_order_checks": 60},
    limits=["A/B without technical isolation attestation", "No global HTML, ANSI or PDF parity claim", "No commit, stage or push", "R1 failure retained; its ephemeral PDFs are unavailable"],
))
