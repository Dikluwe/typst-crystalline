"""Freeze only the L0 and independently measured public observations."""
import argparse
import datetime
import hashlib
import json
from pathlib import Path
import re

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]


def sha(path):
    return hashlib.file_digest(open(path, "rb"), "sha256").hexdigest()


def observation(run):
    return {k: run[k] for k in ("exit", "stdout", "stderr")}


def l0_normative_sha(path):
    raw = Path(path).read_bytes()
    normalized, count = re.subn(rb"(?m)^Hash do C\xc3\xb3digo: [0-9a-f]+\r?\n", b"", raw)
    assert count == 1
    return hashlib.sha256(normalized).hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--measurement", required=True)
    parser.add_argument("--candidate-runs")
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    measurement_path = Path(args.measurement)
    measurement = json.loads(measurement_path.read_text())
    cases_path = BASE / "p1311-ab-cases.json"
    if not args.candidate_runs:
        cases = json.loads(cases_path.read_text())["cases"]
        for case in cases:
            runs = [r for r in measurement["runs"] if r["id"] == case["id"]]
            assert len(runs) == 8
            assert all(r["argv"][2] == case["expr"] for r in runs)
        expected = []
        for profile in ("default", "html", "a11y", "html+a11y"):
            for case in cases:
                source = "vanilla" if case["kind"] == "target" else "baseline"
                run, = [r for r in measurement["runs"] if r["profile"] == profile and
                        r["id"] == case["id"] and r["product"] == source and r["order"] == "normal"]
                if case["kind"] == "target":
                    assert run["exit"] == 1 and run["stdout"] == ""
                    assert run["stderr"].startswith("error: function `")
                    assert run["stderr"].count("error:") == 1
                    assert "hint:" not in run["stderr"]
                expected.append({"id": case["id"], "kind": case["kind"], "profile": profile,
                                 "oracle_product": source, "expected": observation(run)})
        result = {"schema": "p1311-ab-freeze-v1", "utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
                  "regime": "A/B sem atestação de isolamento técnico",
                  "capabilities": {"executor": "/root/p1311_tests", "context": "role-scoped task; no candidate source/diff/owner tests read",
                    "read": ["CLAUDE.md", "01_core/CLAUDE.md", "skill and references", "L0 field_access", "P1309 selection", "P1311 baseline", "vanilla source", "baseline binaries", "HEAD prepatch stdlib source"],
                    "write": ["00_nucleo/diagnosticos/p1311-ab-*"],
                    "limitation": "shared filesystem permits access beyond declared allowlist; no technical attestation"},
                  "limitations": ["CLI cannot construct arbitrary custom Rust Element/native/plugin variants; owner unit tests must cover those, without claiming independent A/B coverage.",
                    "Controls intentionally preserve baseline public output including existing vanilla divergences.",
                    "eval inner diagnostic span is propagated to outer source string; no forced field-only outer span."],
                  "pre_freeze_classification": "assert is namespaceSome control; text and text.With are real Native None targets per prepatch reviewer category audit. All expressions/argv unchanged from the measurement.",
                  "l0": {"path": "00_nucleo/prompts/compiler/eval/bindings/field_access.md",
                    "raw_sha256": sha(ROOT / "00_nucleo/prompts/compiler/eval/bindings/field_access.md"),
                    "normative_sha256": l0_normative_sha(ROOT / "00_nucleo/prompts/compiler/eval/bindings/field_access.md"),
                    "normalization": "Remove exactly one full line matching ^Hash do Código: [0-9a-f]+ newline; all other bytes protected. This metadata reseal only is authorized by the root before implementation."},
                  "inputs": {str(cases_path.relative_to(ROOT)): sha(cases_path),
                    "00_nucleo/diagnosticos/p1311-ab-runner.py": sha(BASE / "p1311-ab-runner.py"),
                    "00_nucleo/diagnosticos/p1311-ab-freeze.py": sha(__file__),
                    str(measurement_path): sha(measurement_path)},
                  "expected": expected}
    else:
        candidate = json.loads(Path(args.candidate_runs).read_text())
        for path, digest in measurement["inputs"].items():
            assert sha(ROOT / path) == digest, ("frozen input drift", path)
        assert l0_normative_sha(ROOT / measurement["l0"]["path"]) == measurement["l0"]["normative_sha256"]
        required_keys = {(e["id"], e["profile"], order) for e in measurement["expected"]
                         for order in ("normal", "repeat", "reverse")}
        actual_keys = [(r["id"], r["profile"], r["order"]) for r in candidate["runs"]]
        assert len(set(actual_keys)) == len(actual_keys)
        assert set(actual_keys) == required_keys
        assert candidate["inputs"]["cases_sha256"] == sha(cases_path)
        assert candidate["inputs"]["runner_sha256"] == sha(BASE / "p1311-ab-runner.py")
        failures = []
        for run in candidate["runs"]:
            expect, = [e for e in measurement["expected"] if e["id"] == run["id"] and e["profile"] == run["profile"]]
            if observation(run) != expect["expected"]:
                failures.append({"id":run["id"], "kind":run["kind"], "profile":run["profile"], "order":run["order"],
                                 "expected":expect["expected"], "actual":observation(run)})
        assert len(candidate["runs"]) == len(measurement["expected"]) * 3
        result = {"schema":"p1311-ab-comparison-v1", "utc":datetime.datetime.now(datetime.timezone.utc).isoformat(),
                  "freeze_sha256":sha(measurement_path), "candidate_runs_sha256":sha(args.candidate_runs),
                  "comparisons":len(candidate["runs"]), "failures":failures,
                  "status":"PASS" if not failures else "FAIL", "unknown":0}
    Path(args.output).write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps({"output":args.output,"sha256":sha(args.output),"items":len(result.get("expected", result.get("failures", [])))}))


if __name__ == "__main__":
    main()
