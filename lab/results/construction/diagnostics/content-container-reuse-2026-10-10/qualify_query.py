"""Compare realized table queries before/after and against the pinned oracle."""
import resource
resource.setrlimit(resource.RLIMIT_STACK, (64 * 1024 * 1024, resource.getrlimit(resource.RLIMIT_STACK)[1]))
import datetime
import gzip
import hashlib
import json
import os
import subprocess
from pathlib import Path

D = Path(__file__).resolve().parent
R = D.parents[2]
F = R / "00_nucleo/diagnosticos/table-local-scaling-followup-2026-10-09/fixtures"
B = {
    "before": Path("/tmp/tekt-before-containers-2026-10-10"),
    "after": R / "target/release/typst",
    "vanilla": R / "lab/typst-original/target/release/typst",
}
sha = lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
env = os.environ.copy()
for key in list(env):
    if key.startswith(("TYPST_", "CRYSTALLINE_", "CARGO_PROFILE_")):
        env.pop(key)
env.update(SOURCE_DATE_EPOCH="0", NO_COLOR="1", TZ="UTC", LC_ALL="C")
cases = [F / f"{variant}-{n}.typ" for n in (100, 1600)
         for variant in ("local-set", "no-fill", "inline-set")]
state = {
    "utc_start": utc(),
    "head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=R, text=True).strip(),
    "working_tree": "uncommitted",
    "git_status": subprocess.check_output(["git", "status", "--short", "--untracked-files=all"], cwd=R, text=True),
    "git_diff_stat": subprocess.check_output(["git", "diff", "HEAD", "--stat"], cwd=R, text=True),
    "binaries": {name: sha(path) for name, path in B.items()},
    "script_sha256": sha(__file__),
    "fixtures": {p.name: sha(p) for p in cases},
    "samples": [],
}
for fixture in cases:
    outputs = {}
    for name, binary in B.items():
        cmd = [str(binary), "eval", "--in", str(fixture), "query(table).len()"]
        result = subprocess.run(cmd, cwd=R, env=env, capture_output=True, text=True, timeout=120)
        row = {"fixture": fixture.name, "engine": name, "utc": utc(),
               "command": cmd, "returncode": result.returncode,
               "stdout": result.stdout, "stderr": result.stderr}
        state["samples"].append(row)
        outputs[name] = row
    assert all(row["returncode"] == 0 for row in outputs.values()), outputs
    assert outputs["before"]["stdout"] == outputs["after"]["stdout"] == outputs["vanilla"]["stdout"] == "1\n", outputs
state["utc_end"] = utc()
assert state["binaries"] == {name: sha(path) for name, path in B.items()}
assert state["script_sha256"] == sha(__file__)
assert all(sha(F / name) == digest for name, digest in state["fixtures"].items())
with gzip.open(D / "qualification-query.json.gz", "wt") as out:
    json.dump(state, out, ensure_ascii=False)
print(f"query(table).len(): {len(cases)} fixtures, three engines, all 1")
