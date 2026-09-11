#!/usr/bin/env python3
"""Validate P1341 oracle authorability without reading product or candidate data."""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path
from typing import Any


def canonical_sha(value: Any) -> str:
    raw = json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(raw).hexdigest()


def file_sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


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
        print(
            "usage: CHECKER CONTRACT FOCAL BINDING_MANIFEST EXPECTATIONS",
            file=sys.stderr,
        )
        return 2

    contract_path, focal_path, manifest_path, expectations_path = map(Path, sys.argv[1:])
    contract = json.loads(contract_path.read_text(encoding="utf-8"))
    focal = json.loads(focal_path.read_text(encoding="utf-8"))
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    expectations = json.loads(expectations_path.read_text(encoding="utf-8"))
    errors: list[str] = []

    contract_binding = contract["planes"]["binding_manifest"]
    binding_digest = canonical_sha(manifest.get("bindings"))
    declared_digest = manifest.get("content_addressing", {}).get("content_sha256")
    if manifest.get("schema") != contract_binding.get("schema"):
        errors.append("binding-schema")
    if manifest.get("bindings") != contract_binding.get("bindings"):
        errors.append("binding-content")
    if binding_digest != declared_digest or binding_digest != contract_binding.get("content_sha256"):
        errors.append("binding-content-address")
    roles = [row.get("stable_role") for row in manifest.get("bindings", [])]
    if len(roles) != len(set(roles)) or any(not role for role in roles):
        errors.append("binding-role-bijection")

    authority = contract.get("authority_manifest", {})
    if manifest.get("authority", {}).get("sha256") != authority.get("sha256"):
        errors.append("binding-authority-pin")
    if expectations.get("authority", {}).get("sha256") != authority.get("sha256"):
        errors.append("expectations-authority-pin")
    contract_digest = file_sha(contract_path)
    if manifest.get("contract", {}).get("sha256") != contract_digest:
        errors.append("binding-contract-pin")
    if expectations.get("contract", {}).get("sha256") != contract_digest:
        errors.append("expectations-contract-pin")
    if expectations.get("binding_manifest", {}).get("content_sha256") != binding_digest:
        errors.append("expectations-binding-pin")

    external = contract["planes"]["external_expectation"]
    required = set(external["required"])
    forbidden = set(external["forbidden_runtime_keys"])
    external_cases = expectations.get("external_cases", [])
    if [row.get("expected_classification") for row in external_cases] != ["Preserved", "Unknown"]:
        errors.append("external-class-vector")
    for index, row in enumerate(external_cases):
        expectation = row.get("expectation", {})
        if not required.issubset(expectation):
            errors.append(f"external-required-fields-{index}")
        if forbidden.intersection(walk_keys(expectation)):
            errors.append(f"external-runtime-key-leak-{index}")
        if set(expectation.get("required_roles", [])) != set(roles):
            errors.append(f"external-role-completeness-{index}")
        dynamic_markers = ("dynamic-", "r-exec", "r-default", "r-selected", "r-with", "r-callback", "r-body", "r-counter", "r-content", "r-dict")
        if any(value.startswith(dynamic_markers) for value in walk_strings(expectation)):
            errors.append(f"external-runtime-value-leak-{index}")

    if len(external_cases) == 2:
        left = dict(external_cases[0].get("expectation", {}))
        right = dict(external_cases[1].get("expectation", {}))
        left.pop("case_key", None)
        right.pop("case_key", None)
        left.pop("cell_selector", None)
        right.pop("cell_selector", None)
        if left != right:
            errors.append("opaque-semantic-expectation-drift")
        allowed = set(contract.get("unknown_policy", {}).get("allowed_reason_codes", []))
        if external_cases[1].get("allowed_reason_code") not in allowed:
            errors.append("opaque-reason")

    focal_rows = focal.get("mutations", [])
    required_faults = {row.get("fault") for row in focal_rows}
    negative_cases = expectations.get("negative_cases", [])
    actual_faults = {row.get("fault") for row in negative_cases}
    case_keys = [row.get("case_key") for row in negative_cases]
    if len(negative_cases) != 41 or len(case_keys) != len(set(case_keys)):
        errors.append("negative-count-or-uniqueness")
    if actual_faults != required_faults:
        errors.append("negative-fault-completeness")
    if any(row.get("expected_classification") != "Violated" for row in negative_cases):
        errors.append("negative-class-vector")

    result = {
        "schema": "p1341-oracle-authorability-check-r1",
        "status": "AUTHORABLE_NOT_SEALED" if not errors else "NOT_AUTHORABLE",
        "binding_manifest": {
            "roles": len(roles),
            "content_sha256": binding_digest,
            "matches_contract": binding_digest == contract_binding.get("content_sha256"),
        },
        "expectations": {
            "external_cases": len(external_cases),
            "negative_cases": len(negative_cases),
            "forbidden_key_leaks": sum(error.startswith("external-runtime-key-leak") for error in errors),
            "runtime_value_leaks": sum(error.startswith("external-runtime-value-leak") for error in errors),
        },
        "errors": errors,
        "limitations": [
            "Authorability only; not a seal, product verdict, implementation check or reachability proof.",
            "The adversary and verifier remain independent authorities for mutation materialization and verdicts.",
        ],
    }
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0 if not errors else 1


if __name__ == "__main__":
    raise SystemExit(main())
