#!/usr/bin/env python3
"""Verifier-owned bilateral P1290 candidate gate against the sealed oracle v4."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
PROBE = DIAG / "p1290-contract-probes.typ"
VANILLA = Path("/usr/local/bin/typst")

FROZEN = {
    ROOT / "00_nucleo/prompts/compiler/eval/repr.md": "e584d5b159f04f3cf8d9a485e2c3c64b3df344520403c2407e6955f2c6b07739",
    DIAG / "p1290-manifest.json": "235d9ed5015580c8f2c32cf16c6eb3478089f038f10774578ecc896e36f03266",
    DIAG / "p1290-contract.json": "0ae42afcbc54ffe62daaec925be5e02ff3218f51ab5864fd64d6be9f7d1e6a26",
    PROBE: "6ac1fed77e50b0ed7b7962048427be1c87ca9917d55c181a35d6ea4e01a6229b",
    DIAG / "p1290-oracle-suite.json": "3b98ef7a51bbc18e845b75cd05d571811172d25c763093de14eac174f8a43b5e",
    DIAG / "p1290-oracle-runner.py": "d44bf520530ab71e51ab10cba36639ced90a1b3b978001674fa8c035d12372da",
    DIAG / "p1290-seal-v4.json": "03ff5d26678b01d9078e0ea43aa2ac4838ab3120622575aa1766b6f36340b63c",
    VANILLA: "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8",
}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def load_runner():
    path = DIAG / "p1290-oracle-runner.py"
    spec = importlib.util.spec_from_file_location("p1290_oracle_runner_v4", path)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def query(binary: Path) -> tuple[dict, dict]:
    argv = [
        str(binary), "query", str(PROBE), "<p1290-contract>",
        "--field", "value", "--one", "--pretty",
    ]
    run = subprocess.run(argv, cwd=ROOT, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    assert run.returncode == 0, (argv, run.returncode, run.stderr)
    return json.loads(run.stdout), {
        "argv": argv,
        "exit": run.returncode,
        "stderr": run.stderr.strip(),
    }


def flatten(value: dict) -> dict[str, str]:
    arrays = value["arrays"]
    math = value["math-ops"]
    return {
        "array.empty": arrays["empty"],
        "array.singleton": arrays["singleton"],
        "array.short": arrays["short"],
        "array.inner-length-49": arrays["inner-length-49"],
        "array.inner-length-50": arrays["inner-length-50"],
        "array.inner-length-51": arrays["inner-length-51"],
        "array.zeros-body-49": arrays["zeros-body-49"],
        "array.zeros-body-52": arrays["zeros-body-52"],
        "array.internal-multiline": arrays["internal-multiline"],
        "array.nested-long": arrays["nested-long"],
        "mathop.csc": math["csc"],
        "mathop.dim": math["dim"],
        "mathop.lim-limits-true": math["lim"],
        "mathop.compound-arcsin": math["arcsin"],
        "mathop.user-false-string": math["user-false-string"],
        "mathop.user-true-string": math["user-true-string"],
        "mathop.user-false-content": math["user-false-content"],
        "mathop.user-markup": math["user-markup"],
        "mathop.user-escaping-string": math["user-escaping-string"],
        "mathop.user-escaping-content": math["user-escaping-content"],
        "control.math-sum-non-mathop": math["sum-non-op-control"],
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--expected-binary-sha256", required=True)
    args = parser.parse_args()

    observed_hashes = {str(path): sha256(path) for path in FROZEN}
    assert all(sha256(path) == expected for path, expected in FROZEN.items())
    assert sha256(args.binary) == args.expected_binary_sha256

    vanilla_1, vanilla_command_1 = query(VANILLA)
    candidate_1, candidate_command_1 = query(args.binary)
    candidate_2, candidate_command_2 = query(args.binary)
    assert candidate_1 == candidate_2
    assert candidate_1 == vanilla_1

    runner = load_runner()
    suite_path = DIAG / "p1290-oracle-suite.json"
    contract_path = DIAG / "p1290-contract.json"
    suite, contract = load(suite_path), load(contract_path)
    runner.validate_frozen_inputs(suite, contract, contract_path)

    actual = flatten(candidate_1)
    ordered = [case["id"] for case in suite["positive_cases"]]
    assert set(actual) == set(ordered)
    canonical = [{"case_id": case_id, "actual": actual[case_id]} for case_id in ordered]
    envelope = {
        "schema": runner.CANDIDATE_SCHEMA,
        "manifest_sha256": runner.MANIFEST_SHA256,
        "contract_sha256": runner.CONTRACT_SHA256,
        "runs": [
            {"name": "canonical-1", "observations": canonical},
            {"name": "canonical-2", "observations": [dict(item) for item in canonical]},
            {"name": "reverse-1", "observations": [dict(item) for item in reversed(canonical)]},
        ],
        "opaque_observations": [],
    }
    report, passed = runner.evaluate_candidate(suite, envelope)
    assert passed and report["result"] == "Preserved"
    print(json.dumps({
        "schema": "p1290-candidate-gate-report/v1",
        "result": "Preserved",
        "binary_sha256": sha256(args.binary),
        "vanilla_sha256": sha256(VANILLA),
        "frozen_hashes": observed_hashes,
        "bilateral_equal": True,
        "repeat_equal": True,
        "positive_cases": len(ordered),
        "oracle_counts": report["counts"],
        "commands": [vanilla_command_1, candidate_command_1, candidate_command_2],
    }, ensure_ascii=False, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
