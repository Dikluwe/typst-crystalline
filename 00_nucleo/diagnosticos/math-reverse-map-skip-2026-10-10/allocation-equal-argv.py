"""RED/GREEN allocation gate for change-aware container traversal."""
import resource
import datetime, gzip, hashlib, json, os, statistics, subprocess, sys, tempfile, time
from pathlib import Path

resource.setrlimit(resource.RLIMIT_STACK, (64 * 1024 * 1024, resource.getrlimit(resource.RLIMIT_STACK)[1]))
label = sys.argv[1]
assert label in ("red", "green")
D = Path(__file__).resolve().parent
R = D.parents[2]
F = R / "00_nucleo/diagnosticos/font-primary-reuse-2026-10-10/fixtures"
B = Path("/tmp/tekt-math-map/old/typst") if label == "red" else Path("/tmp/tekt-math-map/new/typst")
SO = R / "00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/alloc_probe.so"
FONT = R / "00_nucleo/diagnosticos/performance-baseline/fonts"
CPU = min(os.sched_getaffinity(0))
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
env = os.environ.copy()
for key in list(env):
    if key.startswith(("TYPST_", "CRYSTALLINE_", "CARGO_PROFILE_")):
        env.pop(key)
env.update(RAYON_NUM_THREADS="1", SOURCE_DATE_EPOCH="0", NO_COLOR="1", TZ="UTC", LC_ALL="C", LD_PRELOAD=str(SO))
fixtures = [F / name for name in ("no-fill-1600.typ", "families-1600.typ", "variants-1600.typ")]
state = {
    "stack_limit_bytes": resource.getrlimit(resource.RLIMIT_STACK)[0], "utc_start": utc(), "label": label,
    "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=R, text=True).strip(),
    "working_tree": "uncommitted",
    "git_status": subprocess.check_output(["git", "status", "--short", "--untracked-files=all"], cwd=R, text=True),
    "git_diff_stat": subprocess.check_output(["git", "diff", "HEAD", "--stat"], cwd=R, text=True),
    "binary_sha256": sha(B), "interposer_sha256": sha(SO), "driver_sha256": sha(__file__),
    "cpu": CPU, "criterion": "variants allocation calls <=1116479; other controls do not increase",
    "environment": {k: env[k] for k in ("RAYON_NUM_THREADS", "SOURCE_DATE_EPOCH", "NO_COLOR", "TZ", "LC_ALL")},
    "fixtures": {f.name: {"text": f.read_text(), "sha256": sha(f)} for f in fixtures}, "samples": [],
}
with tempfile.TemporaryDirectory(prefix="table-observe-") as tmp:
    out = Path(tmp)
    for rep in range(3):
        for fixture in (fixtures if rep % 2 == 0 else list(reversed(fixtures))):
            pdf = out / f"{fixture.stem}-{rep}.pdf"
            profile = out / f"{fixture.stem}-{rep}.json"
            run_env = env.copy(); run_env["ALLOC_PROFILE_OUT"] = str(profile)
            cmd = [str(B), "compile", str(fixture), str(pdf), "--root", str(F), "--pdf-tagged=false", "--pretty=false", "--font-path", str(FONT)]
            start = time.perf_counter_ns()
            proc = subprocess.run(cmd, cwd=R, env=run_env, capture_output=True, text=True, check=True,
                                  timeout=120, preexec_fn=lambda: os.sched_setaffinity(0, {CPU}))
            state["samples"].append({"fixture": fixture.stem, "rep": rep, "utc": utc(), "command": cmd,
                                     "wall_ms": (time.perf_counter_ns() - start) / 1e6,
                                     "profile": json.loads(profile.read_text()), "pdf_sha256": sha(pdf), "stderr": proc.stderr})
state["summary"] = {}
for fixture in fixtures:
    rows = [x for x in state["samples"] if x["fixture"] == fixture.stem]
    state["summary"][fixture.stem] = {
        "allocation_calls_median": statistics.median(sum(x["profile"][key] for key in ("malloc_calls", "calloc_calls", "realloc_calls", "memalign_calls")) for x in rows),
        "wall_ms_median": statistics.median(x["wall_ms"] for x in rows),
    }
state["passed"] = state["summary"]["variants-1600"]["allocation_calls_median"] <= 1116479
if label == "green":
    before = json.load(gzip.open(D / "allocation-equal-red.json.gz", "rt"))
    state["red_summary"] = before["summary"]
    state["passed"] = state["summary"]["variants-1600"]["allocation_calls_median"] <= 1116479 and all(state["summary"][name]["allocation_calls_median"] <= old["allocation_calls_median"] for name, old in before["summary"].items())
state["utc_end"] = utc()
assert sha(B) == state["binary_sha256"] and sha(SO) == state["interposer_sha256"] and sha(__file__) == state["driver_sha256"]
assert all(sha(F / name) == entry["sha256"] for name, entry in state["fixtures"].items())
with gzip.open(D / f"allocation-equal-{label}.json.gz", "wt") as out:
    json.dump(state, out, ensure_ascii=False)
print(state["summary"], "passed", state["passed"])
if not state["passed"]:
    sys.exit(1)
