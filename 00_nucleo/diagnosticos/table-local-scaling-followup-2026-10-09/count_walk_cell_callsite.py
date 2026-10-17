"""Diagnostic-only callsite/stage counts for the pinned release."""
import datetime, gzip, hashlib, json, os, subprocess, tempfile
from pathlib import Path

D = Path(__file__).resolve().parent
R = D.parents[2]
B = R / "target/release/typst"
F = D / "fixtures"
FONT = R / "00_nucleo/diagnosticos/performance-baseline/fonts"
GDB = D / "walk_cell_callsite.gdb.py"
CASES = ("local-set-100", "no-fill-100", "local-set-1600", "no-fill-1600")
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
assert sha(B) == "702060185fdc0533de86e7bf00c107e26fd0d281170ccd9c76d12b91e4e46883"
env = os.environ.copy()
for key in list(env):
    if key.startswith(("TYPST_", "CRYSTALLINE_", "CARGO_PROFILE_")):
        env.pop(key)
env.update(RAYON_NUM_THREADS="1", SOURCE_DATE_EPOCH="0", NO_COLOR="1", TZ="UTC", LC_ALL="C")
state = {
    "utc_start": utc(), "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=R, text=True).strip(),
    "working_tree": "uncommitted", "git_status": subprocess.check_output(["git", "status", "--short", "--untracked-files=all"], cwd=R, text=True),
    "git_diff_stat": subprocess.check_output(["git", "diff", "HEAD", "--stat"], cwd=R, text=True),
    "binary_sha256": sha(B), "driver_sha256": sha(__file__), "gdb_script_sha256": sha(GDB),
    "fixtures": {name: {"text": (F / f"{name}.typ").read_text(), "sha256": sha(F / f"{name}.typ")} for name in CASES},
    "runs": [], "scope": "GDB call counts only, not runtime. Struct offsets and enum stage bytes pinned to binary SHA; verified against disassembly and GDB branch count.",
}
with tempfile.TemporaryDirectory(prefix="tekt-cell-callsite-") as tmp:
    out = Path(tmp)
    for name in CASES:
        fixture = F / f"{name}.typ"
        pdf = out / f"{name}.pdf"
        counts = out / f"{name}.json"
        run_env = env.copy(); run_env["CELL_CALLSITE_OUT"] = str(counts)
        cmd = ["gdb", "--batch", "-ex", "set pagination off", "-ex", f"source {GDB}", "--args", str(B),
               "compile", str(fixture), str(pdf), "--root", str(F), "--pdf-tagged=false", "--pretty=false", "--font-path", str(FONT)]
        proc = subprocess.run(cmd, cwd=R, env=run_env, capture_output=True, text=True, timeout=240)
        assert proc.returncode == 0 and counts.exists() and pdf.exists(), (proc.returncode, proc.stderr)
        rows = json.loads(counts.read_text())
        state["runs"].append({"case": name, "utc": utc(), "command": cmd, "counts": rows,
                              "stderr": proc.stderr, "stdout": proc.stdout, "pdf_sha256": sha(pdf)})
        print(name, rows, flush=True)
state["utc_end"] = utc()
assert sha(B) == state["binary_sha256"] and sha(GDB) == state["gdb_script_sha256"] and sha(__file__) == state["driver_sha256"]
assert all(sha(F / f"{name}.typ") == item["sha256"] for name, item in state["fixtures"].items())
with gzip.open(D / "walk-cell-callsite-before.json.gz", "wt") as out:
    json.dump(state, out, ensure_ascii=False)
