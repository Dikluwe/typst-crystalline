"""P1307-R4 reproducible baseline and commands; not an independent verifier."""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"


def sha(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()


def stamp():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def state():
    return {k: subprocess.check_output(["git", *v], cwd=ROOT, text=True) for k, v in {
        "head": ["rev-parse", "HEAD"], "status": ["status", "--short"],
        "diff_stat": ["diff", "HEAD", "--stat"], "diff": ["diff", "HEAD", "--binary"],
        "staged": ["diff", "--cached", "--binary"],
    }.items()}


def read(n):
    return json.loads((D / n).read_text())


def freeze():
    old = read("p1307-r3-baseline.json")
    lineage = read("p1307-r3-lineage.json")
    for row in lineage["pairs"]:
        assert sha(ROOT / row["prompt"]) == row["after_sha256"]
        assert sha(ROOT / row["consumer"]) == row["consumer_baseline_sha256"]
    protected = {p.name: sha(p) for p in D.glob("p1307-*") if not p.name.startswith("p1307-r4-")}
    p1306 = read("p1307-baseline.json")["p1306_closure"]
    assert all(sha(r["path"]) == r["sha256"] for r in p1306.values())
    result = {"schema": "p1307-r4-approved-before-code-v1", "at": stamp(),
              "state": state(), "preflight_sha256": sha(D / "p1307-r4-preflight.json"),
              "script_sha256": sha(__file__), "approved_L0": lineage["pairs"],
              "approved_L0_texts": {r["prompt"]: (ROOT/r["prompt"]).read_text() for r in lineage["pairs"]},
              "source_inventory": {p: sha(ROOT/p) for p in old["source_inventory"]},
              "protected_predecessors": protected, "p1306_closure": p1306,
              "binaries": read("p1307-r2-baseline.json")["binaries"]}
    with (D / "p1307-r4-baseline.json").open("x") as f:
        json.dump(result, f, ensure_ascii=False, indent=2)
        f.write("\n")
    print(json.dumps({"path": str(D/"p1307-r4-baseline.json"), "sha256": sha(D/"p1307-r4-baseline.json")}))


def run(argv):
    before = state()
    start = stamp()
    tick = time.monotonic()
    p = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True)
    row = {"argv": argv, "cwd": str(ROOT), "start": start, "finish": stamp(),
           "seconds": time.monotonic()-tick, "exit": p.returncode,
           "stdout": p.stdout, "stderr": p.stderr, "before": before, "after": state(),
           "baseline_sha256": sha(D/"p1307-r4-baseline.json")}
    dest = D/"p1307-r4-commands.json"
    rows = read(dest.name) if dest.exists() else []
    rows.append(row)
    dest.write_text(json.dumps(rows, ensure_ascii=False, indent=2)+"\n")
    print(json.dumps({"argv": argv, "exit": p.returncode, "seconds": row["seconds"]}))
    print(p.stdout[-4000:])
    print(p.stderr[-4000:])
    raise SystemExit(p.returncode)


if __name__ == "__main__":
    if sys.argv[1:] == ["freeze"]:
        freeze()
    else:
        run(sys.argv[2:])
