"""Freeze public observations, then compare without source access."""
import argparse
import datetime
import hashlib
import json
from pathlib import Path
import re
import subprocess

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
L0 = ROOT / "00_nucleo/prompts/compiler/stdlib/loading.md"

def sha(path):
    with open(path, "rb") as handle:
        return hashlib.file_digest(handle, "sha256").hexdigest()

def normative():
    raw, count = re.subn(rb"(?m)^Hash do C\xc3\xb3digo: [0-9a-f]+\r?\n", b"", L0.read_bytes())
    assert count == 1
    return hashlib.sha256(raw).hexdigest()

def obs(row):
    return {k: row[k] for k in ("exit", "stdout", "stderr")}

def save(path, result):
    path = Path(path).resolve()
    assert not path.exists(), "Output must be new; frozen evidence is immutable"
    raw = json.dumps(result, ensure_ascii=False, indent=2) + "\n"
    patch = "*** Begin Patch\n*** Add File: " + str(path) + "\n" + "".join("+" + line + "\n" for line in raw.splitlines()) + "*** End Patch\n"
    subprocess.run(["apply_patch"], input=patch, text=True, check=True, capture_output=True)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--measurement", required=True)
    parser.add_argument("--candidate-runs")
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    path = Path(args.measurement)
    document = json.loads(path.read_text())
    cases_path = BASE / "p1312-ab-cases.json"
    cases = json.loads(cases_path.read_text())["cases"]
    stamp = datetime.datetime.now(datetime.timezone.utc).isoformat()
    if not args.candidate_runs:
        assert document["inputs"]["cases_sha256"] == sha(cases_path)
        assert document["inputs"]["runner_sha256"] == sha(BASE / "p1312-ab-runner.py")
        assert document["inputs"]["l0_sha256"] == sha(L0)
        expected = []
        for profile in ("default", "html", "a11y", "html+a11y"):
            for case in cases:
                source = "baseline" if case["kind"] == "control" else "vanilla"
                run, = [r for r in document["runs"] if r["id"] == case["id"] and r["profile"] == profile and r["product"] == source and r["order"] == "normal"]
                assert run["argv"][2] == case["expr"]
                observation = obs(run)
                if case["kind"] == "target":
                    assert observation["exit"] == 1 and observation["stdout"] == ""
                    assert observation["stderr"].startswith("error: expected path or string, found "), case["id"]
                    baseline, = [r for r in document["runs"] if r["id"] == case["id"] and r["profile"] == profile and r["product"] == "baseline"]
                    assert obs(baseline) != observation
                elif case["kind"] == "symbol":
                    assert observation["stderr"].startswith("error: file not found (searched at /tmp/p1312-ab-fixtures/α)\n")
                    observation["stderr"] = "error: expected path or string, found symbol\n" + observation["stderr"].split("\n", 1)[1]
                    source = "normative Symbol rejection; measured vanilla value-span/trace, L0 literal message"
                expected.append({"id": case["id"], "kind": case["kind"], "profile": profile, "oracle": source, "expected": observation})
        inputs = {str(p.relative_to(ROOT)): sha(p) for p in (cases_path, BASE / "p1312-ab-runner.py", Path(__file__), path.resolve(), BASE / "p1312-baseline.json", BASE / "p1312-measurement.json")}
        inputs.update(document["inputs"]["fixtures"])
        result = {"schema": "p1312-ab-freeze-v1", "utc": stamp, "regime": "A/B executado sem atestação de isolamento técnico", "capabilities": {"executor": "/root/p1312_tests", "context": "new role-scoped context; authorized P1312 baseline/measurement contain historical P1310/P1311 diffs; no P1312 candidate source/diff/owner tests read", "reads": ["skill and references", "CLAUDE.md", "01_core/CLAUDE.md", "loading L0", "P1312 baseline and measurement", "P1311 independent test infrastructure", "binary observations", "git HEAD/diff stat/status"], "writes": ["00_nucleo/diagnosticos/p1312-ab-*", "/tmp/p1312-ab-fixtures/*"], "limitation": "Shared filesystem capabilities are broader than declared role; no isolation attestation."}, "unknown_policy": "Any missing, malformed, ambiguous, timeout or unverified required observation blocks; never map Unknown to success.", "budget": "One initial CLI compatibility failure; one semantic corpus preflight and one syntax correction; final baseline once, candidate normal/repeat/reverse. Revisions with unchanged failing cause require review after two attempts.", "l0": {"path": str(L0.relative_to(ROOT)), "raw_sha256": sha(L0), "normative_sha256": normative(), "allowed_mutation": "Only exactly one canonical full Hash do Código line; every other byte protected."}, "baseline_binaries": document["binaries"], "provenance": document["start"], "inputs": inputs, "expected": expected, "limitations": ["Symbol is an explicit normative expectation, not vanilla equality.", "Controls preserve baseline divergences, including CSV Bytes, extra positional, encoding override, XML fields and rooted imported path sandbox error.", "Arbitrary synthetic Rust Args and unsupported Value variants require owner unit evidence; CLI map case covers genuine detached origin.", "No implementation isolation or general language equivalence attested."]}
    else:
        for p, digest in document["inputs"].items():
            assert sha(ROOT / p) == digest, ("frozen drift", p)
        assert normative() == document["l0"]["normative_sha256"]
        candidate = json.loads(Path(args.candidate_runs).read_text())
        assert candidate["inputs"]["cases_sha256"] == sha(cases_path)
        assert candidate["inputs"]["runner_sha256"] == sha(BASE / "p1312-ab-runner.py")
        required = {(e["id"], e["profile"], o) for e in document["expected"] for o in ("normal", "repeat", "reverse")}
        actual = [(r["id"], r["profile"], r["order"]) for r in candidate["runs"]]
        assert set(actual) == required and len(actual) == len(required)
        failures = []
        for run in candidate["runs"]:
            case, = [c for c in cases if c["id"] == run["id"]]
            assert run["argv"][2] == case["expr"]
            assert run["product"] == "candidate"
            expect, = [e for e in document["expected"] if e["id"] == run["id"] and e["profile"] == run["profile"]]
            if obs(run) != expect["expected"]:
                failures.append({"id": run["id"], "kind": run["kind"], "profile": run["profile"], "order": run["order"], "expected": expect["expected"], "actual": obs(run)})
        result = {"schema": "p1312-ab-comparison-v1", "utc": stamp, "freeze_sha256": sha(path), "candidate_runs_sha256": sha(args.candidate_runs), "candidate": candidate["binaries"], "comparisons": len(actual), "failures": failures, "unknown": 0, "status": "PASS" if not failures else "FAIL"}
    save(args.output, result)
    print(json.dumps({"output": args.output, "sha256": sha(args.output), "items": len(result.get("expected", result.get("failures", [])))}))

if __name__ == "__main__":
    main()
