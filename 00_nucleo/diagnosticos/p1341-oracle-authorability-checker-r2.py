#!/usr/bin/env python3
"""Validate P1341 R3 oracle authorability without candidate or product access."""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path
from typing import Any


def load(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def file_sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def canonical_sha(value: Any) -> str:
    raw = json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(raw).hexdigest()


def walk_keys(value: Any):
    if isinstance(value, dict):
        for key, child in value.items():
            yield key
            yield from walk_keys(child)
    elif isinstance(value, list):
        for child in value:
            yield from walk_keys(child)


def walk_strings(value: Any):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for child in value.values():
            yield from walk_strings(child)
    elif isinstance(value, list):
        for child in value:
            yield from walk_strings(child)


def main() -> int:
    if len(sys.argv) != 5:
        print("usage: CHECKER CONTRACT FOCAL BINDINGS EXPECTATIONS", file=sys.stderr)
        return 2
    contract_path, focal_path, bindings_path, expectations_path = map(Path, sys.argv[1:])
    contract = load(contract_path)
    focal = load(focal_path)
    bindings = load(bindings_path)
    expectations = load(expectations_path)
    errors: list[str] = []

    contract_sha = file_sha(contract_path)
    focal_sha = file_sha(focal_path)
    for name, artifact in (("bindings", bindings), ("expectations", expectations)):
        if artifact.get("status") != "AUTHORED_NOT_SEALED":
            errors.append(name + "-status")
        if artifact.get("contract", {}).get("sha256") != contract_sha:
            errors.append(name + "-contract-pin")
        if artifact.get("focal", {}).get("sha256") != focal_sha:
            errors.append(name + "-focal-pin")

    contract_manifest = contract.get("static_binding_manifest", {})
    authored_rows = bindings.get("bindings", [])
    authored_digest = canonical_sha(authored_rows)
    if bindings.get("schema") != contract_manifest.get("schema"):
        errors.append("binding-schema")
    if authored_rows != contract_manifest.get("bindings"):
        errors.append("binding-content")
    if bindings.get("event_cardinalities") != contract_manifest.get("event_cardinalities"):
        errors.append("binding-event-cardinalities")
    if bindings.get("content_addressing", {}).get("content_sha256") != authored_digest:
        errors.append("binding-authored-pin")
    if contract_manifest.get("content_sha256") != authored_digest:
        errors.append("binding-contract-pin")
    roles = [row.get("stable_role") for row in authored_rows]
    cardinalities = {row.get("stable_role"): row.get("object_cardinality") for row in authored_rows}
    obligations = contract.get("semantic_obligations", {})
    if roles != obligations.get("required_roles"):
        errors.append("binding-role-order-and-completeness")
    if cardinalities != obligations.get("role_cardinalities"):
        errors.append("binding-role-cardinalities")

    external_contract = contract.get("external_expectation", {})
    required_sections = set(external_contract.get("required_exact_sections", []))
    forbidden = set(external_contract.get("forbidden_runtime_keys", []))
    external_cases = expectations.get("external_cases", [])
    if [row.get("expected_classification") for row in external_cases] != ["Preserved", "Unknown"]:
        errors.append("external-class-vector")
    contract_owned = (
        "source_coordinates",
        "required_roles",
        "role_cardinalities",
        "feature_witnesses",
        "callbacks",
        "counter_occurrences",
        "dict_expectations",
        "required_causal_role_edges",
    )
    for index, row in enumerate(external_cases):
        expectation = row.get("expectation", {})
        if set(expectation) != required_sections:
            errors.append(f"external-exact-sections-{index}")
        if forbidden.intersection(walk_keys(expectation)):
            errors.append(f"external-forbidden-key-leak-{index}")
        for section in contract_owned:
            if expectation.get(section) != obligations.get(section):
                errors.append(f"external-contract-owned-{section}-{index}")
        dynamic_markers = ("dynamic-", "r-exec", "r-default", "r-selected", "r-with", "r-callback", "r-body", "r-counter", "r-content", "r-dict")
        if any(value.startswith(dynamic_markers) for value in walk_strings(expectation)):
            errors.append(f"external-runtime-value-leak-{index}")
    if len(external_cases) == 2:
        left = external_cases[0].get("expectation", {})
        right = external_cases[1].get("expectation", {})
        for section in contract_owned:
            if left.get(section) != right.get(section):
                errors.append("opaque-semantic-drift:" + section)
        allowed = contract.get("unknown_policy", {}).get("allowed_reason_codes", [])
        if external_cases[1].get("allowed_reason_code") not in allowed:
            errors.append("opaque-reason")

    authored_gate = expectations.get("negative_gate_expectation", {})
    contract_gate = contract.get("materialized_gate", {})
    focal_gate = focal.get("required_gate", {})
    expected_gate = {
        "protected": {"valid": contract_gate.get("protected", {}).get("valid"), "required_classification": focal_gate.get("negative_classification")},
        "additional": {"valid": contract_gate.get("additional", {}).get("valid"), "required_classification": focal_gate.get("negative_classification")},
        "combined": {"valid": contract_gate.get("protected", {}).get("valid", 0) + contract_gate.get("additional", {}).get("valid", 0), "required_classification": focal_gate.get("negative_classification")},
        "orders": contract_gate.get("orders"),
        "required_score": contract_gate.get("required_score"),
        "survivor_policy": "Any classification other than Violated blocks a future seal.",
    }
    if authored_gate != expected_gate:
        errors.append("negative-gate-expectation")

    result = {
        "schema": "p1341-oracle-authorability-check-r2",
        "status": "AUTHORABLE_NOT_SEALED" if not errors else "NOT_AUTHORABLE",
        "pins": {"contract": contract_sha, "focal": focal_sha, "bindings_content": authored_digest},
        "bindings": {"roles": len(roles), "exact_cardinalities": cardinalities == obligations.get("role_cardinalities"), "event_cardinalities": bindings.get("event_cardinalities")},
        "expectations": {"external_cases": len(external_cases), "exact_r3_sections": len(required_sections), "forbidden_key_leaks": sum("forbidden-key-leak" in error for error in errors), "runtime_value_leaks": sum("runtime-value-leak" in error for error in errors)},
        "negative_gate": {"protected": 41, "additional": 10, "combined": 51, "classification": "Violated"},
        "errors": errors,
        "limitations": ["Authorability only; not a seal, candidate check, product verdict or reachability proof."],
    }
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0 if not errors else 1


if __name__ == "__main__":
    raise SystemExit(main())
