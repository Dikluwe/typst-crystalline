"""Independent CLI observations. Never reads candidate source or owner tests."""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
PROFILES = {"default": [], "html": ["--features", "html"], "a11y": ["--features", "a11y-extras"], "html+a11y": ["--features", "html,a11y-extras"]}
L0 = ROOT / "00_nucleo/prompts/compiler/stdlib/loading.md"
FIXTURES = [Path("/tmp/p1312-ab-fixtures") / n for n in ("data.txt", "data.csv", "paths.typ")]

def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def sha(path):
    with open(path, "rb") as handle:
        return hashlib.file_digest(handle, "sha256").hexdigest()

def state():
    def git(*args):
        return subprocess.check_output(["git", *args], cwd=ROOT, text=True)
    return {"utc": now(), "head": git("rev-parse", "HEAD").strip(), "diff_stat": git("diff", "HEAD", "--stat"), "status": git("status", "--short")}

def save(path, result):
    path = Path(path).resolve()
    assert not path.exists(), "Output must be new; historical evidence is immutable"
    raw = json.dumps(result, ensure_ascii=False, indent=2) + "\n"
    patch = "*** Begin Patch\n*** Add File: " + str(path) + "\n" + "".join("+" + line + "\n" for line in raw.splitlines()) + "*** End Patch\n"
    subprocess.run(["apply_patch"], input=patch, text=True, check=True, capture_output=True)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--candidate")
    parser.add_argument("--candidate-sha256")
    parser.add_argument("--orders", default="normal")
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    cases_path = BASE / "p1312-ab-cases.json"
    cases = json.loads(cases_path.read_text())["cases"]
    binaries = {"baseline": ("/dev/shm/p1311-target.JE8Cyy/release/typst", "4bbced9ec793fb84daa560e0d965958b33e1eb2f5ecd4bfd3bc30b5900bb09fb"), "vanilla": ("/usr/local/bin/typst", "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8")}
    if args.candidate:
        assert args.candidate_sha256
        binaries = {"candidate": (args.candidate, args.candidate_sha256)}
    for path, digest in binaries.values():
        assert sha(path) == digest, ("binary identity", path)
    result = {"schema": "p1312-ab-runs-v1", "start": state(), "inputs": {"cases_sha256": sha(cases_path), "runner_sha256": sha(__file__), "l0_sha256": sha(L0), "fixtures": {str(p): sha(p) for p in FIXTURES}}, "binaries": {k: {"path": p, "sha256": s} for k, (p, s) in binaries.items()}, "runs": []}
    env = dict(os.environ)
    for key in ("TYPST_FEATURES", "TYPST_DIAGNOSTIC_FORMAT", "TYPST_ROOT"):
        env.pop(key, None)
    env.update({"NO_COLOR": "1", "TERM": "dumb", "PYTHONDONTWRITEBYTECODE": "1"})
    for order in args.orders.split(","):
        for profile, flags in PROFILES.items():
            for case in list(reversed(cases)) if order == "reverse" else cases:
                for product, (binary, digest) in binaries.items():
                    argv = [binary, "eval", case["expr"], *flags]
                    started, tick = now(), time.monotonic()
                    proc = subprocess.run(argv, cwd=FIXTURES[0].parent, env=env, capture_output=True, timeout=30)
                    result["runs"].append({"id": case["id"], "kind": case["kind"], "profile": profile, "order": order, "product": product, "argv": argv, "cwd": str(FIXTURES[0].parent), "utc": started, "elapsed_seconds": time.monotonic() - tick, "exit": proc.returncode, "stdout": proc.stdout.decode(), "stderr": proc.stderr.decode()})
    result["end"] = state()
    for path, digest in binaries.values():
        assert sha(path) == digest, ("binary changed during run", path)
    save(args.output, result)
    print(json.dumps({"output": args.output, "sha256": sha(args.output), "runs": len(result["runs"])}))
    if not args.candidate:
        for run in result["runs"]:
            if run["profile"] == "default":
                print(run["id"], run["product"], run["exit"], repr(run["stdout"]), repr(run["stderr"].splitlines()[:1]))

if __name__ == "__main__":
    main()
