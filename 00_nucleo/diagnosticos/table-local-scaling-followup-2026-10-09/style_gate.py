"""Allocation RED/GREEN gate for selecting fonts without a candidate vector."""
import gzip
import hashlib
import json
import os
import statistics
import subprocess
import sys
import tempfile
import time
from datetime import datetime, timezone
from pathlib import Path

label = sys.argv[1]
assert label in ("red", "green")
D = Path(__file__).resolve().parent
R = D.parents[2]
F = D / "fixtures"
B = R / "target/release/typst"
SO = D / "alloc_probe.so"
FONT = R / "00_nucleo/diagnosticos/performance-baseline/fonts"
CPU = min(os.sched_getaffinity(0))
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
utc = lambda: datetime.now(timezone.utc).isoformat()
env = os.environ.copy()
for key in list(env):
    if key.startswith(("TYPST_", "CRYSTALLINE_", "CARGO_PROFILE_")):
        env.pop(key)
env.update(RAYON_NUM_THREADS="1", SOURCE_DATE_EPOCH="0", NO_COLOR="1", TZ="UTC", LC_ALL="C", LD_PRELOAD=str(SO))
state = {
    "utc_start": utc(),
    "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=R, text=True).strip(),
    "working_tree": "uncommitted",
    "git_status": subprocess.check_output(["git", "status", "--short", "--untracked-files=all"], cwd=R, text=True),
    "git_diff_stat": subprocess.check_output(["git", "diff", "HEAD", "--stat"], cwd=R, text=True),
    "binary_sha256": sha(B), "interposer_sha256": sha(SO), "driver_sha256": sha(D / "style_gate.py"),
    "cpu": CPU, "threshold_calls": 1015000,
    "fixtures": {f.name: {"text": f.read_text(), "sha256": sha(f)} for f in (F / "block-no-set-1600.typ", F / "local-set-1600.typ")},
    "samples": [],
}
with tempfile.TemporaryDirectory(prefix="table-empty-order-") as tmp:
    out = Path(tmp)
    for variant in ("block-no-set", "local-set"):
        for rep in range(3):
            f = F / f"{variant}-1600.typ"
            pdf = out / f"{variant}-{rep}.pdf"
            profile = out / f"{variant}-{rep}.json"
            run_env = env.copy()
            run_env["ALLOC_PROFILE_OUT"] = str(profile)
            cmd = [str(B), "compile", str(f), str(pdf), "--root", str(F), "--pdf-tagged=false", "--pretty=false", "--font-path", str(FONT)]
            start = time.perf_counter_ns()
            proc = subprocess.run(cmd, cwd=R, env=run_env, capture_output=True, text=True, check=True, timeout=120,
                                  preexec_fn=lambda: os.sched_setaffinity(0, {CPU}))
            wall_ms = (time.perf_counter_ns() - start) / 1e6
            counts = json.loads(profile.read_text())
            state["samples"].append({"variant": variant, "rep": rep, "utc": utc(), "command": cmd,
                                     "wall_ms": wall_ms, "profile": counts, "pdf_sha256": sha(pdf), "stderr": proc.stderr})
state["utc_end"] = utc()
state["summary"] = {}
for variant in ("block-no-set", "local-set"):
    rows = [x for x in state["samples"] if x["variant"] == variant]
    state["summary"][variant] = {
        "allocation_calls_median": statistics.median(sum(x["profile"][key] for key in ("malloc_calls", "calloc_calls", "realloc_calls", "memalign_calls")) for x in rows),
        "wall_ms_median": statistics.median(x["wall_ms"] for x in rows),
        "pdf_sha256": rows[0]["pdf_sha256"],
    }
state["passed"] = state["summary"]["local-set"]["allocation_calls_median"] <= state["threshold_calls"]
assert sha(B) == state["binary_sha256"] and sha(SO) == state["interposer_sha256"]
with gzip.open(D / f"style-{label}-gate.json.gz", "wt") as out:
    json.dump(state, out, ensure_ascii=False)
print(state["summary"], "passed", state["passed"])
if not state["passed"]:
    sys.exit(1)
