#!/usr/bin/env python3
"""Deterministic validator for P1290 candidate repr observations.

The runner reads only the frozen contract, the canonical oracle suite, and a
candidate JSON file. It never executes either implementation. Exit status is
0 only when every evaluated outcome is Preserved; Unknown never passes.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
from pathlib import Path
from typing import Any


MANIFEST_SHA256 = "235d9ed5015580c8f2c32cf16c6eb3478089f038f10774578ecc896e36f03266"
CONTRACT_SHA256 = "0ae42afcbc54ffe62daaec925be5e02ff3218f51ab5864fd64d6be9f7d1e6a26"
PROMPT_SHA256 = "e584d5b159f04f3cf8d9a485e2c3c64b3df344520403c2407e6955f2c6b07739"
CANDIDATE_SCHEMA = "p1290-repr-candidate-output/v1"
SUITE_SCHEMA = "p1290-repr-oracle-suite/v1"
REQUIRED_RUNS = ("canonical-1", "canonical-2", "reverse-1")


class OracleError(Exception):
    """Invalid oracle input or candidate envelope."""


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_json(path: Path) -> Any:
    try:
        with path.open("r", encoding="utf-8") as handle:
            return json.load(handle)
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise OracleError(f"cannot load JSON {path}: {error}") from error


def require_object(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise OracleError(f"{label} must be a JSON object")
    return value


def require_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise OracleError(f"{label} must be a JSON array")
    return value


def json_pointer(document: Any, pointer: str) -> Any:
    if not isinstance(pointer, str) or not pointer.startswith("/"):
        raise OracleError(f"invalid JSON pointer: {pointer!r}")
    current = document
    for raw_part in pointer[1:].split("/"):
        part = raw_part.replace("~1", "/").replace("~0", "~")
        if isinstance(current, dict) and part in current:
            current = current[part]
        elif isinstance(current, list) and part.isdecimal():
            index = int(part)
            if index >= len(current):
                raise OracleError(f"JSON pointer index out of bounds: {pointer}")
            current = current[index]
        else:
            raise OracleError(f"JSON pointer not found: {pointer}")
    return current


def validate_frozen_inputs(suite: dict[str, Any], contract: dict[str, Any], contract_path: Path) -> None:
    if sha256(contract_path) != CONTRACT_SHA256:
        raise OracleError("contract file hash differs from frozen CONTRACT_SHA256")
    if suite.get("schema") != SUITE_SCHEMA:
        raise OracleError("unsupported oracle suite schema")
    if suite.get("manifest_sha256") != MANIFEST_SHA256:
        raise OracleError("suite manifest hash differs from frozen manifest")
    if suite.get("contract_sha256") != CONTRACT_SHA256:
        raise OracleError("suite contract hash differs from frozen contract")
    if suite.get("prompt_sha256") != PROMPT_SHA256:
        raise OracleError("suite prompt hash differs from frozen prompt")
    if contract.get("manifest_sha256") != MANIFEST_SHA256:
        raise OracleError("contract manifest hash differs from frozen manifest")
    if contract.get("prompt_sha256") != PROMPT_SHA256:
        raise OracleError("contract prompt hash differs from frozen prompt")

    positive_cases = require_list(suite.get("positive_cases"), "suite.positive_cases")
    opaque_cases = require_list(suite.get("opaque_cases"), "suite.opaque_cases")
    seen: set[str] = set()
    for label, cases, expected_result in (
        ("positive", positive_cases, "Preserved"),
        ("opaque", opaque_cases, "Unknown"),
    ):
        for index, raw_case in enumerate(cases):
            case = require_object(raw_case, f"suite.{label}_cases[{index}]")
            case_id = case.get("id")
            if not isinstance(case_id, str) or not case_id:
                raise OracleError(f"suite.{label}_cases[{index}].id must be a non-empty string")
            if case_id in seen:
                raise OracleError(f"duplicate suite case id: {case_id}")
            seen.add(case_id)
            if case.get("expected_result") != expected_result:
                raise OracleError(f"suite case {case_id} has wrong expected_result")
            pointer = case.get("contract_pointer")
            contract_value = json_pointer(contract, pointer)
            if label == "positive" and pointer.startswith(("/arrays/exact/", "/math_op/exact/", "/math_op/non_mathop_control/exact")):
                if case.get("expected") != contract_value:
                    raise OracleError(f"suite expected value differs from contract at {pointer}")
            if label == "opaque" and not isinstance(contract_value, str):
                raise OracleError(f"opaque contract pointer must resolve to a string: {pointer}")


def parse_observations(raw: Any, label: str) -> list[dict[str, str]]:
    observations = require_list(raw, label)
    parsed: list[dict[str, str]] = []
    seen: set[str] = set()
    for index, raw_observation in enumerate(observations):
        observation = require_object(raw_observation, f"{label}[{index}]")
        case_id = observation.get("case_id")
        actual = observation.get("actual")
        if not isinstance(case_id, str) or not case_id:
            raise OracleError(f"{label}[{index}].case_id must be a non-empty string")
        if case_id in seen:
            raise OracleError(f"duplicate case_id {case_id!r} in {label}")
        if not isinstance(actual, str):
            raise OracleError(f"{label}[{index}].actual must be a string")
        seen.add(case_id)
        parsed.append({"case_id": case_id, "actual": actual})
    return parsed


def evaluate_candidate(suite: dict[str, Any], candidate: dict[str, Any]) -> tuple[dict[str, Any], bool]:
    if candidate.get("schema") != CANDIDATE_SCHEMA:
        raise OracleError("unsupported candidate schema")
    if candidate.get("manifest_sha256") != MANIFEST_SHA256:
        raise OracleError("candidate manifest hash differs from frozen manifest")
    if candidate.get("contract_sha256") != CONTRACT_SHA256:
        raise OracleError("candidate contract hash differs from frozen contract")

    positive_cases = [require_object(case, "positive case") for case in suite["positive_cases"]]
    positive_by_id = {case["id"]: case for case in positive_cases}
    canonical_ids = [case["id"] for case in positive_cases]
    required_orders = {
        "canonical-1": canonical_ids,
        "canonical-2": canonical_ids,
        "reverse-1": list(reversed(canonical_ids)),
    }

    raw_runs = require_list(candidate.get("runs"), "candidate.runs")
    run_by_name: dict[str, dict[str, Any]] = {}
    for index, raw_run in enumerate(raw_runs):
        run = require_object(raw_run, f"candidate.runs[{index}]")
        name = run.get("name")
        if not isinstance(name, str) or not name:
            raise OracleError(f"candidate.runs[{index}].name must be a non-empty string")
        if name in run_by_name:
            raise OracleError(f"duplicate candidate run name: {name}")
        run_by_name[name] = run
    if set(run_by_name) != set(REQUIRED_RUNS):
        raise OracleError(f"candidate runs must be exactly {list(REQUIRED_RUNS)!r}")

    outcomes: list[dict[str, Any]] = []
    actual_by_run: dict[str, dict[str, str]] = {}
    preserved = 0
    violated = 0
    unknown = 0

    for run_name in REQUIRED_RUNS:
        observations = parse_observations(run_by_name[run_name].get("observations"), f"candidate run {run_name}.observations")
        observed_ids = [observation["case_id"] for observation in observations]
        expected_ids = required_orders[run_name]
        if observed_ids != expected_ids:
            outcomes.append({
                "scope": "run-order",
                "run": run_name,
                "result": "Violated",
                "expected_case_ids": expected_ids,
                "actual_case_ids": observed_ids,
            })
            violated += 1
        actual_by_run[run_name] = {observation["case_id"]: observation["actual"] for observation in observations}
        for case_id in expected_ids:
            if case_id not in actual_by_run[run_name]:
                outcomes.append({
                    "scope": "positive",
                    "run": run_name,
                    "case_id": case_id,
                    "result": "Violated",
                    "reason": "missing required observation",
                })
                violated += 1
                continue
            expected = positive_by_id[case_id]["expected"]
            actual = actual_by_run[run_name][case_id]
            result = "Preserved" if actual == expected else "Violated"
            outcomes.append({
                "scope": "positive",
                "run": run_name,
                "case_id": case_id,
                "result": result,
                "expected": expected,
                "actual": actual,
            })
            if result == "Preserved":
                preserved += 1
            else:
                violated += 1

    for case_id in canonical_ids:
        actuals = [actual_by_run[name].get(case_id) for name in REQUIRED_RUNS]
        result = "Preserved" if actuals[0] == actuals[1] == actuals[2] else "Violated"
        outcomes.append({
            "scope": "determinism",
            "case_id": case_id,
            "result": result,
            "actual_by_run": dict(zip(REQUIRED_RUNS, actuals)),
        })
        if result == "Preserved":
            preserved += 1
        else:
            violated += 1

    opaque_by_id = {
        require_object(case, "opaque case")["id"]: require_object(case, "opaque case")
        for case in suite["opaque_cases"]
    }
    opaque_observations = parse_observations(candidate.get("opaque_observations", []), "candidate.opaque_observations")
    for observation in opaque_observations:
        case_id = observation["case_id"]
        if case_id not in opaque_by_id:
            raise OracleError(f"unknown opaque case id: {case_id}")
        outcomes.append({
            "scope": "opaque",
            "case_id": case_id,
            "result": "Unknown",
            "actual": observation["actual"],
            "reason": opaque_by_id[case_id]["reason"],
        })
        unknown += 1

    passed = violated == 0 and unknown == 0
    report = {
        "schema": "p1290-repr-oracle-report/v1",
        "manifest_sha256": MANIFEST_SHA256,
        "contract_sha256": CONTRACT_SHA256,
        "result": "Preserved" if passed else ("Violated" if violated else "Unknown"),
        "pass": passed,
        "counts": {
            "Preserved": preserved,
            "Unknown": unknown,
            "Violated": violated,
        },
        "opaque_catalog": [
            {
                "case_id": case_id,
                "result": "Unknown",
                "evaluated": case_id in {item["case_id"] for item in opaque_observations},
            }
            for case_id in opaque_by_id
        ],
        "outcomes": outcomes,
    }
    return report, passed


def make_self_test_candidate(suite: dict[str, Any]) -> dict[str, Any]:
    cases = [require_object(case, "positive case") for case in suite["positive_cases"]]
    canonical = [{"case_id": case["id"], "actual": case["expected"]} for case in cases]
    return {
        "schema": CANDIDATE_SCHEMA,
        "manifest_sha256": MANIFEST_SHA256,
        "contract_sha256": CONTRACT_SHA256,
        "runs": [
            {"name": "canonical-1", "observations": canonical},
            {"name": "canonical-2", "observations": [dict(item) for item in canonical]},
            {"name": "reverse-1", "observations": [dict(item) for item in reversed(canonical)]},
        ],
        "opaque_observations": [],
    }


def run_self_test(suite: dict[str, Any]) -> dict[str, Any]:
    valid = make_self_test_candidate(suite)
    valid_report, valid_pass = evaluate_candidate(suite, valid)
    if not valid_pass or valid_report["result"] != "Preserved":
        raise OracleError("self-test failed to preserve the canonical candidate")

    violated_candidate = json.loads(json.dumps(valid))
    violated_candidate["runs"][0]["observations"][0]["actual"] += "!"
    violated_report, violated_pass = evaluate_candidate(suite, violated_candidate)
    if violated_pass or violated_report["counts"]["Violated"] == 0:
        raise OracleError("self-test failed to reject a mismatching candidate")

    unknown_candidate = json.loads(json.dumps(valid))
    opaque_id = suite["opaque_cases"][0]["id"]
    unknown_candidate["opaque_observations"] = [{"case_id": opaque_id, "actual": "opaque"}]
    unknown_report, unknown_pass = evaluate_candidate(suite, unknown_candidate)
    if unknown_pass or unknown_report["counts"]["Unknown"] != 1:
        raise OracleError("self-test allowed Unknown to pass")

    return {
        "schema": "p1290-repr-oracle-self-test/v1",
        "result": "Preserved",
        "checks": {
            "canonical_candidate": "Preserved",
            "mismatching_candidate": "Violated",
            "opaque_candidate": "Unknown and rejected",
            "repetition": "canonical-1 equals canonical-2",
            "reverse_order": "reverse-1 equals canonical runs by case id",
        },
    }


def emit_json(value: Any) -> None:
    json.dump(value, sys.stdout, ensure_ascii=False, indent=2, sort_keys=True)
    sys.stdout.write("\n")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--suite", required=True, type=Path)
    parser.add_argument("--contract", required=True, type=Path)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--candidate", type=Path)
    group.add_argument("--self-test", action="store_true")
    args = parser.parse_args()

    try:
        suite = require_object(load_json(args.suite), "suite")
        contract = require_object(load_json(args.contract), "contract")
        validate_frozen_inputs(suite, contract, args.contract)
        if args.self_test:
            emit_json(run_self_test(suite))
            return 0
        candidate = require_object(load_json(args.candidate), "candidate")
        report, passed = evaluate_candidate(suite, candidate)
        emit_json(report)
        return 0 if passed else 1
    except OracleError as error:
        emit_json({
            "schema": "p1290-repr-oracle-report/v1",
            "result": "Violated",
            "pass": False,
            "error": str(error),
        })
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
