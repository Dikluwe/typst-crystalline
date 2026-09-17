#!/usr/bin/env python3
"""Independent executable oracle for P1430 certification-policy V01-V05."""

from __future__ import annotations

import argparse
import importlib.util
import json
from pathlib import Path
from types import ModuleType
from typing import Any


OPAQUE_REASONS = {
    "U01": "UNKNOWN_IDENTITY_MISMATCH",
    "U02": "UNKNOWN_MATHCASES_CARDINALITY",
    "U03": "UNKNOWN_STYLE_CASCADE",
    "U04": "UNKNOWN_EVALUATION_OBSERVATION",
    "U05": "UNKNOWN_LAYOUT_OBSERVATION",
    "U06": "UNKNOWN_EXECUTION_OR_BUDGET",
}


def load_policy(path: Path) -> ModuleType:
    spec = importlib.util.spec_from_file_location("p1430_certification_policy", path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load policy: {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def expected_fields(actual: dict[str, Any], expected: dict[str, Any]) -> list[str]:
    return [
        f"{field}: expected {value!r}, got {actual.get(field)!r}"
        for field, value in expected.items()
        if actual.get(field) != value
    ]


def run(policy: ModuleType) -> dict[str, Any]:
    vectors: dict[str, Any] = {}
    failures: list[dict[str, Any]] = []

    opaque = []
    for case_id, reason in OPAQUE_REASONS.items():
        decision = policy.evaluate_case(
            {
                "case_id": case_id,
                "category": "opaque",
                "mandatory": False,
                "expected_classification": "Unknown",
                "actual_classification": "Unknown",
                "reason_code": reason,
            }
        )
        opaque.append(decision)
        mismatch = expected_fields(
            decision,
            {
                "classification_matches": True,
                "acceptance_credit": 0,
                "mutation_credit": 0,
                "blocks_gate": False,
                "reason_code": reason,
            },
        )
        if mismatch:
            failures.append({"vector": "V01", "case_id": case_id, "mismatch": mismatch})
    vectors["V01"] = opaque

    mandatory_unknown = policy.evaluate_case(
        {
            "case_id": "P-MANDATORY-OPAQUE",
            "category": "positive",
            "mandatory": True,
            "expected_classification": "Preserved",
            "actual_classification": "Unknown",
            "reason_code": "UNKNOWN_EXECUTION_OR_BUDGET",
        }
    )
    mismatch = expected_fields(
        mandatory_unknown,
        {
            "classification_matches": False,
            "acceptance_credit": 0,
            "mutation_credit": 0,
            "blocks_gate": True,
        },
    )
    if mismatch:
        failures.append({"vector": "V02", "mismatch": mismatch})
    vectors["V02"] = mandatory_unknown

    mutation_unknown_record = {
        "mutation_id": "M-UNKNOWN",
        "valid": True,
        "classification": "Unknown",
        "attributable": True,
        "reason_code": "UNKNOWN_EXECUTION_OR_BUDGET",
    }
    mutation_unknown = policy.evaluate_mutation(mutation_unknown_record)
    mismatch = expected_fields(
        mutation_unknown,
        {
            "classification_matches": False,
            "mutation_credit": 0,
            "score_numerator_delta": 0,
            "score_denominator_delta": 1,
            "blocks_gate": True,
            "reason_code": "UNKNOWN_EXECUTION_OR_BUDGET",
        },
    )
    if mismatch:
        failures.append({"vector": "V03", "mismatch": mismatch})
    vectors["V03"] = mutation_unknown

    violated_record = {
        "mutation_id": "M-VIOLATED",
        "valid": True,
        "classification": "Violated",
        "attributable": True,
        "reason_code": "ATTRIBUTABLE_POLICY_WITNESS",
    }
    mixed = policy.score_mutations([violated_record, mutation_unknown_record])
    mismatch = expected_fields(
        mixed,
        {
            "killed": 1,
            "valid": 2,
            "mutation_score": 0.5,
            "blocks_gate": True,
            "mutation_credit": 1,
        },
    )
    if mismatch:
        failures.append({"vector": "V04", "mismatch": mismatch})
    vectors["V04"] = mixed

    positive = policy.evaluate_case(
        {
            "case_id": "P-CONTROL",
            "category": "positive",
            "mandatory": True,
            "expected_classification": "Preserved",
            "actual_classification": "Preserved",
            "reason_code": "ATTRIBUTABLE_POSITIVE",
        }
    )
    negative = policy.evaluate_case(
        {
            "case_id": "N-CONTROL",
            "category": "negative",
            "mandatory": True,
            "expected_classification": "Violated",
            "actual_classification": "Violated",
            "reason_code": "ATTRIBUTABLE_NEGATIVE",
        }
    )
    constant_unknown_guard = policy.evaluate_case(
        {
            "case_id": "P-CONTROL",
            "category": "positive",
            "mandatory": True,
            "expected_classification": "Preserved",
            "actual_classification": "Unknown",
            "reason_code": "UNKNOWN_EXECUTION_OR_BUDGET",
        }
    )
    mismatch = []
    mismatch.extend(
        expected_fields(
            positive,
            {"classification_matches": True, "acceptance_credit": 1, "blocks_gate": False},
        )
    )
    mismatch.extend(
        expected_fields(
            negative,
            {"classification_matches": True, "acceptance_credit": 1, "blocks_gate": False},
        )
    )
    mismatch.extend(
        expected_fields(
            constant_unknown_guard,
            {"classification_matches": False, "acceptance_credit": 0, "blocks_gate": True},
        )
    )
    if mismatch:
        failures.append({"vector": "V05", "mismatch": mismatch})
    vectors["V05"] = {
        "positive": positive,
        "negative": negative,
        "constant_unknown_guard": constant_unknown_guard,
    }

    return {
        "schema": "tekt.materialization.certification-policy-oracle-result/v1",
        "status": "PASS" if not failures else "FAIL",
        "vectors": vectors,
        "failures": failures,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--policy", required=True, type=Path)
    args = parser.parse_args()
    try:
        result = run(load_policy(args.policy.resolve()))
    except Exception as error:  # infrastructure/load opacity is not a semantic kill
        result = {
            "schema": "tekt.materialization.certification-policy-oracle-result/v1",
            "status": "UNKNOWN",
            "vectors": {},
            "failures": [{"reason_code": "UNKNOWN_EXECUTION_OR_BUDGET", "error": str(error)}],
        }
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0 if result["status"] == "PASS" else (1 if result["status"] == "FAIL" else 2)


if __name__ == "__main__":
    raise SystemExit(main())

