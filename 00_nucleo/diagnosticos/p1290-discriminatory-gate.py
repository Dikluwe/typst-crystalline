#!/usr/bin/env python3
"""Verifier-owned adapter for the frozen P1290 oracle/adversarial artifacts."""

from __future__ import annotations

import hashlib
import importlib.util
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"

FROZEN = {
    ROOT / "00_nucleo/prompts/compiler/eval/repr.md": "e584d5b159f04f3cf8d9a485e2c3c64b3df344520403c2407e6955f2c6b07739",
    DIAG / "p1290-manifest.json": "235d9ed5015580c8f2c32cf16c6eb3478089f038f10774578ecc896e36f03266",
    DIAG / "p1290-contract.json": "0ae42afcbc54ffe62daaec925be5e02ff3218f51ab5864fd64d6be9f7d1e6a26",
    DIAG / "p1290-oracle-suite.json": "3b98ef7a51bbc18e845b75cd05d571811172d25c763093de14eac174f8a43b5e",
    DIAG / "p1290-oracle-runner.py": "d44bf520530ab71e51ab10cba36639ced90a1b3b978001674fa8c035d12372da",
    DIAG / "p1290-adversarial-candidates.json": "2f850b05f0a0c77c334227c8633b0082f4a6b0cfa5d08da74b34532f69c3eb4b",
    DIAG / "p1290-adversarial-opaque.json": "a070f248356bcfb0b66df03bd6600f5510f9b4895f682845198e25b25fc1da9e",
}

KEY_TO_CASE = {
    "array.empty": "array.empty",
    "array.singleton": "array.singleton",
    "array.short": "array.short",
    "array.inner_length_49": "array.inner-length-49",
    "array.inner_length_50": "array.inner-length-50",
    "array.inner_length_51": "array.inner-length-51",
    "array.zeros_body_49": "array.zeros-body-49",
    "array.zeros_body_52": "array.zeros-body-52",
    "array.internal_multiline": "array.internal-multiline",
    "math_op.csc": "mathop.csc",
    "math_op.dim": "mathop.dim",
    "math_op.lim_limits_true": "mathop.lim-limits-true",
    "math_op.compound_arcsin": "mathop.compound-arcsin",
    "math_op.user_false_string": "mathop.user-false-string",
    "math_op.user_true_string": "mathop.user-true-string",
    "math_op.user_false_content": "mathop.user-false-content",
    "math_op.user_markup": "mathop.user-markup",
    "math_op.user_escaping_string": "mathop.user-escaping-string",
    "math_op.user_escaping_content": "mathop.user-escaping-content",
    "control.non_mathop_sum": "control.math-sum-non-mathop",
}

OPAQUE_TO_CASE = {
    "U01-non-ascii-width-unit": "opaque.non-ascii-width-unit",
    "U02-unenumerated-math-member": "opaque.unmeasured-math-member",
    "U03-unenumerated-content-morphology": "opaque.unmeasured-content-morphology",
    "U04-parser-unsupported-construction": "opaque.parser-unsupported",
    "U05-outside-repr-fragment": "opaque.outside-repr-fragment",
    "U06-pre-resolved-smartquote-provenance": "opaque.preresolved-smartquote-provenance",
}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def load_runner():
    path = DIAG / "p1290-oracle-runner.py"
    spec = importlib.util.spec_from_file_location("p1290_oracle_runner", path)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def make_candidate(runner, suite, actual_by_case, opaque=()):
    ordered = [case["id"] for case in suite["positive_cases"]]
    canonical = [{"case_id": case_id, "actual": actual_by_case[case_id]} for case_id in ordered]
    return {
        "schema": runner.CANDIDATE_SCHEMA,
        "manifest_sha256": runner.MANIFEST_SHA256,
        "contract_sha256": runner.CONTRACT_SHA256,
        "runs": [
            {"name": "canonical-1", "observations": canonical},
            {"name": "canonical-2", "observations": [dict(item) for item in canonical]},
            {"name": "reverse-1", "observations": [dict(item) for item in reversed(canonical)]},
        ],
        "opaque_observations": list(opaque),
    }


def main() -> None:
    observed_hashes = {str(path.relative_to(ROOT)): sha256(path) for path in FROZEN}
    assert all(sha256(path) == expected for path, expected in FROZEN.items())

    contract = load(DIAG / "p1290-contract.json")
    suite = load(DIAG / "p1290-oracle-suite.json")
    adversarial = load(DIAG / "p1290-adversarial-candidates.json")
    opaque = load(DIAG / "p1290-adversarial-opaque.json")
    runner = load_runner()
    runner.validate_frozen_inputs(suite, contract, DIAG / "p1290-contract.json")

    expected = {case["id"]: case["expected"] for case in suite["positive_cases"]}
    assert set(KEY_TO_CASE) == set(adversarial["canonical_outputs"])
    for adversarial_key, case_id in KEY_TO_CASE.items():
        assert adversarial["canonical_outputs"][adversarial_key] == expected[case_id]

    canonical_report, canonical_pass = runner.evaluate_candidate(
        suite, make_candidate(runner, suite, expected)
    )
    assert canonical_pass and canonical_report["result"] == "Preserved"

    def campaign(mutations):
        results = {}
        for mutation in mutations:
            assert mutation["valid_negative"] is True
            actual = dict(expected)
            for adversarial_key, value in mutation["candidate_output_overlay"].items():
                actual[KEY_TO_CASE[adversarial_key]] = value
            report, passed = runner.evaluate_candidate(
                suite, make_candidate(runner, suite, actual)
            )
            result = report["result"]
            assert not passed and result == "Violated"
            results[mutation["id"]] = {
                "result": result,
                "violations": report["counts"]["Violated"],
                "unknown": report["counts"]["Unknown"],
            }
        return results

    forward = campaign(adversarial["mutations"])
    reverse = campaign(list(reversed(adversarial["mutations"])))
    assert forward == reverse

    opaque_results = {}
    assert opaque["denominator_membership"] is False
    for item in opaque["cases"]:
        case_id = OPAQUE_TO_CASE[item["id"]]
        observation = json.dumps(item["opaque_observation"], ensure_ascii=False, sort_keys=True)
        report, passed = runner.evaluate_candidate(
            suite,
            make_candidate(
                runner,
                suite,
                expected,
                opaque=({"case_id": case_id, "actual": observation},),
            ),
        )
        assert not passed and report["result"] == "Unknown"
        assert report["counts"]["Unknown"] == 1
        opaque_results[item["id"]] = "Unknown"

    killed = sum(item["result"] == "Violated" for item in forward.values())
    denominator = len(adversarial["mutations"])
    score = killed / denominator
    assert score == 1.0
    print(json.dumps({
        "schema": "p1290-discriminatory-gate-report/v1",
        "result": "Preserved",
        "frozen_hashes": observed_hashes,
        "canonical": canonical_report["result"],
        "mutations": forward,
        "reverse_order_equal": forward == reverse,
        "valid_negative_killed": killed,
        "valid_negative_total": denominator,
        "mutation_score": score,
        "opaque": opaque_results,
        "opaque_in_denominator": False,
    }, ensure_ascii=False, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
