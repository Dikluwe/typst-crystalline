"""P1307 authorized reopening: provenance and read-only repr probes."""
import base64
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / "00_nucleo/diagnosticos"


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def pin(path):
    path = Path(path)
    return {"path": str(path), "sha256": sha(path)}


def read(name):
    return json.loads((D / name).read_text())


def save(name, obj):
    with (D / name).open("x") as stream:
        json.dump(obj, stream, ensure_ascii=False, indent=2)
        stream.write("\n")
    print(json.dumps(pin(D / name)), flush=True)


def git(*args):
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True)


def state():
    return {"at": now(), "head": git("rev-parse", "HEAD"), "diff": git("diff", "HEAD", "--binary"), "diff_stat": git("diff", "HEAD", "--stat"), "status": git("status", "--short")}


def unchanged():
    old = read("p1307-baseline.json")
    assert git("rev-parse", "HEAD") == old["source_before"]["head"]
    assert git("diff", "HEAD", "--binary") == old["source_before"]["diff"]
    assert all(sha(ROOT / path) == digest for path, digest in old["source_hashes"].items())
    assert all(sha(record["path"]) == record["sha256"] for record in old["binaries"].values())
    return old


def freeze():
    old = unchanged()
    previous = {p.name: pin(p) for p in sorted(D.glob("p1307-*")) if p.is_file() and not p.name.startswith("p1307-r2-")}
    for record in old["p1306_closure"].values():
        assert sha(record["path"]) == record["sha256"]
    save("p1307-r2-baseline.json", {"schema": "p1307-r2-unchanged-baseline-v1", "state": state(), "preflight": pin(D / "p1307-r2-preflight.json"), "script": pin(__file__), "predecessor_artifacts": previous, "p1306_baseline": pin(D / "p1307-baseline.json"), "binaries": old["binaries"], "source_inventory": old["source_hashes"], "decision": "Reuse identical fresh P1307 binary after complete source+binary validation; no product/L0 change occurred since its build."})


def run(argv):
    env = os.environ.copy()
    env.pop("TYPST_FEATURES", None)
    env.update(NO_COLOR="1", TERM="dumb")
    started = now()
    tick = time.monotonic()
    proc = subprocess.run(argv, cwd=ROOT, env=env, capture_output=True, timeout=60)
    row = {"argv": argv, "cwd": str(ROOT), "started": started, "finished": now(), "seconds": time.monotonic()-tick, "exit": proc.returncode, "env": {"TYPST_FEATURES": "unset", "NO_COLOR": "1", "TERM": "dumb"}}
    for label, data in (("stdout", proc.stdout), ("stderr", proc.stderr)):
        row[label] = data.decode(errors="replace")
        row[label+"_base64"] = base64.b64encode(data).decode()
    return row


def append(row):
    path = D / "p1307-r2-gates.json"
    previous = json.loads(path.read_text()) if path.exists() else []
    previous.append(row)
    path.write_text(json.dumps(previous, ensure_ascii=False, indent=2)+"\n")


def repr_probes():
    old = unchanged()
    cases = [
        'state("probe")', 'state("probe", 0)', 'state("", none)',
        'state("a\\\"b\\nc", "é\\n\\\"\\\\")',
        'state("long", range(20))', 'state("nested", (z: (1, 2), a: true))',
        'state("nested", state("inner", counter("manual")))',
        'counter(page)', 'counter("page")', 'counter(heading)', 'counter("heading")',
        'counter(<a>)', 'counter(heading.where(level: 2))',
        'counter("a\\\"b\\nc")', 'counter(heading.or(figure))',
        'selector(heading).or(figure)', 'heading.where(level: 2)',
    ]
    profiles = {"default": [], "html": ["--features", "html"], "a11y": ["--features", "a11y-extras"], "html+a11y": ["--features", "html,a11y-extras"]}
    rows = []
    before = state()
    for i, expr in enumerate(cases):
        for profile, flags in profiles.items():
            for side, binary in old["binaries"].items():
                row = run([binary["path"], "eval", f"repr({expr})", "--format", "json", *flags])
                row.update(case=i, expression=expr, profile=profile, side=side)
                rows.append(row)
    unchanged()
    append({"kind": "repr_audit_measurement", "baseline": pin(D / "p1307-r2-baseline.json"), "script": pin(__file__), "before": before, "after": state(), "cases": cases, "profiles": profiles, "runs": rows})
    print(json.dumps({"cases": len(cases), "runs": len(rows), "errors": sum(row["exit"] != 0 for row in rows)}))


def gate(argv):
    before = state()
    row = run(argv)
    row.update(kind="read_only_gate", before=before, after=state())
    append(row)
    print(json.dumps({"argv": argv, "exit": row["exit"], "seconds": row["seconds"]}))
    print(row["stdout"][-1200:])
    raise SystemExit(row["exit"])


if __name__ == "__main__":
    if sys.argv[1] == "freeze":
        freeze()
    elif sys.argv[1] == "repr":
        repr_probes()
    else:
        gate(sys.argv[2:])
