"""Independent CLI evidence; never reads implementation or owner tests."""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / "00_nucleo/diagnosticos"
PROFILES = {"default": [], "html": ["--features", "html"],
            "a11y": ["--features", "a11y-extras"],
            "html+a11y": ["--features", "html,a11y-extras"]}


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def sha(path):
    return hashlib.file_digest(open(path, "rb"), "sha256").hexdigest()


def state():
    def git(*args):
        return subprocess.check_output(["git", *args], cwd=ROOT, text=True)
    return {"utc": now(), "head": git("rev-parse", "HEAD").strip(),
            "diff_stat": git("diff", "HEAD", "--stat"),
            "status": git("status", "--short")}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--candidate")
    parser.add_argument("--output", required=True)
    parser.add_argument("--orders", default="normal")
    args = parser.parse_args()
    cases_path = BASE / "p1311-ab-cases.json"
    cases = json.loads(cases_path.read_text())["cases"]
    bins = {"vanilla": "/usr/local/bin/typst",
            "baseline": "/dev/shm/p1310-target.VeBP6Q/release/typst"}
    if args.candidate:
        bins = {"candidate": args.candidate}
    result = {"schema": "p1311-ab-runs-v1", "start": state(),
              "inputs": {"cases_sha256": sha(cases_path),
                         "runner_sha256": sha(__file__),
                         "l0_sha256": sha(ROOT / "00_nucleo/prompts/compiler/eval/bindings/field_access.md")},
              "binaries": {k: {"path": v, "sha256": sha(v)} for k, v in bins.items()},
              "runs": []}
    env = dict(os.environ)
    for key in ("TYPST_FEATURES", "TYPST_DIAGNOSTIC_FORMAT", "TYPST_ROOT"):
        env.pop(key, None)
    env.update({"NO_COLOR": "1", "TERM": "dumb"})
    for order in args.orders.split(","):
        ordered = list(reversed(cases)) if order == "reverse" else cases
        for profile, flags in PROFILES.items():
            for case in ordered:
                for product, binary in bins.items():
                    argv = [binary, "eval", case["expr"], *flags]
                    started, tick = now(), time.monotonic()
                    proc = subprocess.run(argv, cwd=ROOT, env=env, capture_output=True, timeout=30)
                    result["runs"].append({"id": case["id"], "kind": case["kind"],
                        "profile": profile, "order": order, "product": product,
                        "argv": argv, "cwd": str(ROOT), "utc": started,
                        "elapsed_seconds": time.monotonic() - tick, "exit": proc.returncode,
                        "stdout": proc.stdout.decode(), "stderr": proc.stderr.decode()})
    result["end"] = state()
    Path(args.output).write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps({"output": args.output, "sha256": sha(args.output), "runs": len(result["runs"])}))
    if not args.candidate:
        normal = {(r["id"], r["product"]): r for r in result["runs"] if r["profile"] == "default" and r["order"] == "normal"}
        for case in cases:
            a, b = [normal[(case["id"], p)] for p in ("vanilla", "baseline")]
            print(case["id"], case["kind"], "SAME" if (a["exit"], a["stdout"], a["stderr"]) == (b["exit"], b["stdout"], b["stderr"]) else "DIFF", repr(a["stderr"].splitlines()[:1]), repr(b["stderr"].splitlines()[:1]))


if __name__ == "__main__":
    main()
