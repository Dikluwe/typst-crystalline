#!/usr/bin/env python3
"""Executa o contrato público P1301 contra um binário candidato.

Este runner pertence à integração P7, não ao selo discriminatório pré-candidato.
Emite um recibo JSON em stdout e não escreve no checkout.
"""

from __future__ import annotations

import hashlib
import json
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CONTRACT = ROOT / "00_nucleo/diagnosticos/p1301-contract.json"
SEALED = {
    "manifest": (
        ROOT / "00_nucleo/diagnosticos/p1301-manifest.json",
        "074fabeda17f741f064fe1d561ad6a78ad81e45654861825586664a9646f05a8",
    ),
    "field_access_prompt": (
        ROOT / "00_nucleo/prompts/compiler/eval/bindings/field_access.md",
        "38d6f5cdee302491ec059577174d4f6dc6d3c28e3d2da2748f61c05853cc0c16",
    ),
    "tests_prompt": (
        ROOT / "00_nucleo/prompts/compiler/eval/tests.md",
        "a4ade7eda600891465c62c320d80b7c2182c30dbb5f456de210c4a823b3e609a",
    ),
    "contract": (
        CONTRACT,
        "2d8d7a141d63c13473681abdefa24ed9370e03925824a9cae74ca26305ea85f5",
    ),
    "oracle": (
        ROOT / "00_nucleo/diagnosticos/p1301-oracle-suite.json",
        "4182575a0c11e39aed01f05c827d3cb59522f69ab1795f017523586f4c19d012",
    ),
    "mutants": (
        ROOT / "00_nucleo/diagnosticos/p1301-mutants.json",
        "ee3c6c4d19e410850e56e26a984a8d53662056f8a4cba39da746fdb56c2ed18e",
    ),
    "seal": (
        ROOT / "00_nucleo/diagnosticos/p1301-contract-seal.json",
        "658fe879eaf7dce5b0a6372f686e376e7cc652f621d298c817628b03e16f82fb",
    ),
}


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def now() -> str:
    return datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")


def hash_gate() -> dict:
    actual = {name: sha256(path) for name, (path, _) in SEALED.items()}
    mismatches = {
        name: {"expected": expected, "actual": actual[name]}
        for name, (_, expected) in SEALED.items()
        if actual[name] != expected
    }
    return {"actual": actual, "mismatches": mismatches, "pass": not mismatches}


def expected_streams(case: dict) -> tuple[int, str, str]:
    expected = case["expected"]
    stderr = expected.get("rendered_stderr", expected.get("stderr", ""))
    return expected["exit_code"], expected["stdout"], stderr


def run_case(binary: Path, case: dict, profile: dict) -> dict:
    command = [
        str(binary),
        "--color",
        "never",
        "eval",
        *profile["argv"],
        "--diagnostic-format",
        "short",
        case["expression"],
    ]
    started = time.monotonic_ns()
    result = subprocess.run(command, capture_output=True, text=True, timeout=20)
    duration = time.monotonic_ns() - started
    expected_exit, expected_stdout, expected_stderr = expected_streams(case)
    differences = {}
    for name, expected, actual in (
        ("exit_code", expected_exit, result.returncode),
        ("stdout", expected_stdout, result.stdout),
        ("rendered_stderr", expected_stderr, result.stderr),
    ):
        if actual != expected:
            differences[name] = {"expected": expected, "actual": actual}
    return {
        "id": case["id"],
        "command": command,
        "exit_code": result.returncode,
        "stdout": result.stdout,
        "rendered_stderr": result.stderr,
        "duration_ns": duration,
        "verdict": "Preserved" if not differences else "Violated",
        "differences": differences,
    }


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: p1301-candidate-runner.py /absolute/path/to/typst", file=sys.stderr)
        return 2
    binary = Path(sys.argv[1]).resolve()
    if not binary.is_file():
        print(f"candidate binary not found: {binary}", file=sys.stderr)
        return 2

    before = hash_gate()
    if not before["pass"]:
        print(json.dumps({"status": "BLOCKED_FROZEN_INPUT_DRIFT", "hash_gate": before}, indent=2))
        return 1

    contract = json.loads(CONTRACT.read_text(encoding="utf-8"))
    cases = {case["id"]: case for case in contract["required_cases"]}
    if any(case["expression"] == "repr(std)" for case in cases.values()):
        print(json.dumps({"status": "BLOCKED_SCOPE_OUT_VIOLATION"}, indent=2))
        return 1

    trials = {}
    started_at = now()
    start_ns = time.monotonic_ns()
    for order_name in ("normal", "inverted"):
        order = contract["order_invariance"][order_name]
        results = [
            run_case(binary, cases[case_id], contract["profiles"][cases[case_id]["profile"]])
            for case_id in order
        ]
        trials[order_name] = {
            "order": order,
            "results": results,
            "counts": {
                verdict: sum(result["verdict"] == verdict for result in results)
                for verdict in ("Preserved", "Violated")
            },
        }

    normal_map = {item["id"]: item["verdict"] for item in trials["normal"]["results"]}
    inverted_map = {item["id"]: item["verdict"] for item in trials["inverted"]["results"]}
    after = hash_gate()
    pass_gate = (
        after["pass"]
        and normal_map == inverted_map
        and all(verdict == "Preserved" for verdict in normal_map.values())
    )
    receipt = {
        "schema_version": "p1301-candidate-receipt-v1",
        "step": "P1301",
        "status": "PASS" if pass_gate else "FAIL",
        "candidate": {"path": str(binary), "sha256": sha256(binary)},
        "runner": {"path": str(Path(__file__).relative_to(ROOT)), "sha256": sha256(Path(__file__))},
        "started_at": started_at,
        "finished_at": now(),
        "duration_ns": time.monotonic_ns() - start_ns,
        "processes": 46,
        "hash_gate_before": before,
        "hash_gate_after": after,
        "order_invariant": normal_map == inverted_map,
        "unknown": 0,
        "repr_std_executed": False,
        "trials": trials,
    }
    print(json.dumps(receipt, indent=2, ensure_ascii=False, sort_keys=True))
    return 0 if pass_gate else 1


if __name__ == "__main__":
    raise SystemExit(main())
