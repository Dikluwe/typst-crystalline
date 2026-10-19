"""Capture callers of complete grid layout without changing L1."""

import datetime
import gzip
import hashlib
import json
import os
import subprocess
import tempfile
from pathlib import Path


D = Path(__file__).resolve().parent
R = D.parents[2]
B = R / "target/release/typst"
F = D / "fixtures"
FONT = R / "00_nucleo/diagnosticos/performance-baseline/fonts"


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


rows = subprocess.check_output(["nm", str(B)], text=True).splitlines()
names = {}
for row in rows:
    parts = row.split()
    if len(parts) == 3 and parts[1] == "t" and "resolve_grid_columns" in parts[2]:
        names.setdefault(parts[0], parts[2])
assert names
script = "set pagination off\nset confirm off\nset breakpoint pending off\n"
for name in names.values():
    script += f"break '{name}'\ncommands\nsilent\nprintf \"GRID_ENTRY\\n\"\nbt 40\ncontinue\nend\n"
script += "run\n"
env = os.environ.copy()
for key in list(env):
    if key.startswith(("TYPST_", "CRYSTALLINE_", "CARGO_PROFILE_")):
        env.pop(key)
env.update(RAYON_NUM_THREADS="1", SOURCE_DATE_EPOCH="0", NO_COLOR="1", TZ="UTC", LC_ALL="C")
state = {
    "utc_start": utc(),
    "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=R, text=True).strip(),
    "working_tree": "uncommitted",
    "git_status": subprocess.check_output(["git", "status", "--short", "--untracked-files=all"], cwd=R, text=True),
    "git_diff_stat": subprocess.check_output(["git", "diff", "HEAD", "--stat"], cwd=R, text=True),
    "binary_sha256": sha(B), "driver_sha256": sha(D / "trace_grid_invocations.py"),
    "gdb_script": script, "symbol_names": list(names.values()), "samples": [],
}
with tempfile.TemporaryDirectory(prefix="table-grid-trace-") as temp:
    out = Path(temp)
    commands = out / "trace.gdb"
    commands.write_text(script)
    for name in ("local-set-100", "no-fill-100"):
        fixture = F / f"{name}.typ"
        pdf = out / f"{name}.pdf"
        cmd = ["gdb", "--batch", "-x", str(commands), "--args", str(B), "compile", str(fixture), str(pdf), "--root", str(F), "--pdf-tagged=false", "--pretty=false", "--font-path", str(FONT)]
        proc = subprocess.run(cmd, cwd=R, env=env, capture_output=True, text=True, timeout=120)
        state["samples"].append({"case": name, "utc": utc(), "fixture_sha256": sha(fixture),
                                 "command": cmd, "exit_code": proc.returncode, "stdout": proc.stdout,
                                 "stderr": proc.stderr, "entry_count": proc.stdout.count("GRID_ENTRY")})
        print(name, "entries", proc.stdout.count("GRID_ENTRY"), "exit", proc.returncode, flush=True)
state["utc_end"] = utc()
with gzip.open(D / "grid-invocation-traces-deep.json.gz", "wt") as out:
    json.dump(state, out, ensure_ascii=False)
assert all(row["exit_code"] == 0 for row in state["samples"])
