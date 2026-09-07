"""Documentary baseline and read-only gates for authorized P1307-R3 L0 drafting."""
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


def read(name):
    return json.loads((D / name).read_text())


def state():
    return {name: subprocess.check_output(["git", *args], cwd=ROOT, text=True) for name, args in {"head": ["rev-parse", "HEAD"], "status": ["status", "--short"], "diff": ["diff", "HEAD", "--binary"], "diff_stat": ["diff", "HEAD", "--stat"]}.items()}


def stamp():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def save(name, value):
    with (D / name).open("x") as stream:
        json.dump(value, stream, ensure_ascii=False, indent=2)
        stream.write("\n")
    print(json.dumps({"path": str(D / name), "sha256": sha(D / name)}))


def freeze():
    pf = read("p1307-r3-preflight.json")
    b = read("p1307-r2-baseline.json")
    assert all(sha(ROOT / p) == h for p, h in b["source_inventory"].items())
    assert all(sha(r["path"]) == r["sha256"] for r in b["predecessor_artifacts"].values())
    prev = {p.name: sha(p) for p in D.glob("p1307-*") if not p.name.startswith("p1307-r3-")}
    paths = [ROOT / pf["L0_root"] / p for p in pf["L0_write_allowlist"]]
    save("p1307-r3-baseline.json", {"schema": "p1307-r3-before-L0-edit-v1", "at": stamp(), "preflight_sha256": sha(D / "p1307-r3-preflight.json"), "script_sha256": sha(__file__), "state": state(), "source_inventory": b["source_inventory"], "previous_artifacts": prev, "L0_before": {str(p.relative_to(ROOT)): {"sha256": sha(p), "text": p.read_text()} for p in paths}, "initial_capture_attempts": "Two read-only stdout snapshot attempts exceeded tool return limits; neither wrote a baseline or changed inputs. This script generates the canonical snapshot directly, preserving full text."})


def gate(argv):
    before = state()
    start = stamp()
    tick = time.monotonic()
    p = subprocess.run(argv, cwd=ROOT, capture_output=True, text=True)
    row = {"argv": argv, "cwd": str(ROOT), "started": start, "finished": stamp(), "seconds": time.monotonic()-tick, "exit": p.returncode, "stdout": p.stdout, "stderr": p.stderr, "before": before, "after": state()}
    path = D / "p1307-r3-gates.json"
    rows = read(path.name) if path.exists() else []
    rows.append(row)
    path.write_text(json.dumps(rows, ensure_ascii=False, indent=2)+"\n")
    print(json.dumps({"argv": argv, "exit": p.returncode, "seconds": row["seconds"]}))
    print(p.stdout[-1800:])
    raise SystemExit(p.returncode)


if __name__ == "__main__":
    if sys.argv[1:] == ["freeze"]:
        freeze()
    else:
        gate(sys.argv[2:])
