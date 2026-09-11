#!/usr/bin/env python3
"""Validate P1341 R5 closed-schema oracle authorability."""

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
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def exact(value: Any, keys: list[str]) -> bool:
    return isinstance(value, dict) and set(value) == set(keys)


def strict_int(value: Any) -> bool:
    return isinstance(value, int) and not isinstance(value, bool)


def string_list(value: Any) -> bool:
    return isinstance(value, list) and all(isinstance(item, str) for item in value)


def typed_closed(value: Any, keys: dict[str, Any]) -> bool:
    if not isinstance(value, dict) or not isinstance(value.get("tag"), str):
        return False
    if value["tag"] == "dict":
        if not exact(value, keys["typed_dict"]) or not isinstance(value["pairs"], list):
            return False
        names = []
        for pair in value["pairs"]:
            if not isinstance(pair, list) or len(pair) != 2 or not isinstance(pair[0], str) or not typed_closed(pair[1], keys):
                return False
            names.append(pair[0])
        return len(names) == len(set(names))
    if value["tag"] not in {"int", "str", "bool"} or not exact(value, keys["typed_scalar"]):
        return False
    scalar = value["value"]
    return strict_int(scalar) if value["tag"] == "int" else isinstance(scalar, str) if value["tag"] == "str" else isinstance(scalar, bool)


def validate_expectation(value: Any, keys: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if not exact(value, keys["expectation"]):
        return ["closed-expectation-keys"]
    if not exact(value.get("cell_selector"), keys["cell_selector"]):
        errors.append("closed-cell-selector")
    coordinates = value.get("source_coordinates")
    if not isinstance(coordinates, list) or not coordinates:
        errors.append("source-coordinate-list")
    else:
        for row in coordinates:
            if not exact(row, keys["source_coordinate"]) or not strict_int(row.get("context_ordinal")) or not strict_int(row.get("source_byte_offset")) or not isinstance(row.get("lexical_role"), str):
                errors.append("closed-source-coordinate")
    roles, cardinalities = value.get("required_roles"), value.get("role_cardinalities")
    if not string_list(roles) or not isinstance(cardinalities, dict) or set(cardinalities) != set(roles) or any(not strict_int(item) for item in cardinalities.values()):
        errors.append("strict-role-cardinalities")
    for row in value.get("feature_witnesses", []):
        if not exact(row, keys["feature_witness"]) or not string_list(row.get("features")) or not strict_int(row.get("multiplicity_per_attempt")):
            errors.append("closed-feature-witness")
    for row in value.get("callbacks", []):
        if not exact(row, keys["callback"]) or not strict_int(row.get("multiplicity_per_attempt")):
            errors.append("closed-callback")
    for row in value.get("counter_occurrences", []):
        coordinate = row.get("source_coordinate", {}) if isinstance(row, dict) else {}
        if not exact(row, keys["counter_occurrence_expectation"]) or not strict_int(row.get("occurrence_ordinal")) or not strict_int(row.get("multiplicity_per_attempt")) or not exact(coordinate, keys["counter_source_coordinate"]) or not strict_int(coordinate.get("source_byte_offset")) or not strict_int(coordinate.get("byte_length")):
            errors.append("closed-counter-expectation")
    for row in value.get("dict_expectations", []):
        if not exact(row, keys["dict_expectation"]) or not string_list(row.get("typed_path")) or not typed_closed(row.get("ordered_typed_value"), keys):
            errors.append("closed-dict-expectation")
    edges = value.get("required_causal_role_edges")
    if not isinstance(edges, list) or any(not isinstance(edge, list) or len(edge) != 2 or not string_list(edge) for edge in edges):
        errors.append("closed-causal-edges")
    return errors


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
    contract_path, focal_path, manifest_path, expectations_path = map(Path, sys.argv[1:])
    contract, focal, manifest, expectations = map(load, (contract_path, focal_path, manifest_path, expectations_path))
    errors: list[str] = []
    contract_sha, focal_sha = file_sha(contract_path), file_sha(focal_path)
    for name, artifact in (("manifest", manifest), ("expectations", expectations)):
        if artifact.get("status") != "AUTHORED_NOT_SEALED": errors.append(name + "-status")
        if artifact.get("contract", {}).get("sha256") != contract_sha: errors.append(name + "-contract-pin")
        if artifact.get("focal", {}).get("sha256") != focal_sha: errors.append(name + "-focal-pin")

    addressing = manifest.get("content_addressing", {})
    sections = addressing.get("covered_sections", [])
    content_sha = canonical_sha({key: manifest.get(key) for key in sections})
    if addressing.get("content_sha256") != content_sha: errors.append("manifest-content-address")
    if expectations.get("binding_manifest", {}).get("content_sha256") != content_sha: errors.append("expectations-manifest-pin")
    if len(manifest.get("bindings", [])) != 9 or any(row.get("object_cardinality") != 1 for row in manifest.get("bindings", [])): errors.append("binding-cardinality")

    keys = contract.get("closed_key_sets", {})
    if manifest.get("closed_key_sets") != keys: errors.append("closed-key-set-drift")
    if manifest.get("object_role_extensions") != contract.get("object_role_extensions"): errors.append("object-extension-drift")
    if manifest.get("strict_integer_fields") != contract.get("strict_integer_fields"): errors.append("strict-integer-drift")
    if manifest.get("temporal_order") != contract.get("temporal_order"): errors.append("temporal-order-drift")
    if manifest.get("unknown_policy") != contract.get("unknown_policy"): errors.append("unknown-policy-drift")
    if not contract.get("extension_policy", "").startswith("NONE:"): errors.append("extension-policy-not-none")

    cases = expectations.get("external_cases", [])
    if [row.get("expected_classification") for row in cases] != ["Preserved", "Unknown"]: errors.append("external-class-vector")
    for index, row in enumerate(cases):
        for error in validate_expectation(row.get("expectation"), keys):
            errors.append(f"case-{index}:" + error)
    if len(cases) == 2:
        left, right = cases[0]["expectation"], cases[1]["expectation"]
        for section in set(keys["expectation"]) - {"case_key", "cell_selector"}:
            if left.get(section) != right.get(section): errors.append("opaque-semantic-drift:" + section)
        if cases[1].get("allowed_reason_code") != focal.get("controls", {}).get("opaque_reason_code"): errors.append("opaque-reason")

    closed = expectations.get("closed_schema_expectation", {})
    if set(closed) != {"extension_policy","closed_key_sets_source","object_role_extensions_source","strict_integer_fields_source","temporal_order","feature_cardinality","unknown_precedence","malformed_or_unknown_key_classification"}: errors.append("closed-schema-expectation-keys")
    if closed.get("extension_policy") != "NONE" or closed.get("feature_cardinality") != contract.get("temporal_order", {}).get("feature_cardinality") or closed.get("unknown_precedence") != contract.get("unknown_policy", {}).get("order"): errors.append("closed-schema-expectation-drift")

    corpus = expectations.get("frozen_negative_corpus", {})
    frozen = focal.get("frozen_corpus", {})
    expected_corpus = {"protected":frozen.get("protected"),"r1_additional":frozen.get("r1_additional"),"r2_new":frozen.get("r2_new"),"r3_boundary":frozen.get("r3_boundary"),"total":frozen.get("total"),"expected_classification":"Violated","orders":frozen.get("orders"),"required_score":frozen.get("score"),"corpus_policy":"FROZEN; no oracle-local additions or omissions"}
    if corpus != expected_corpus: errors.append("frozen-corpus-declaration")
    if corpus.get("total") != 84 or sum(corpus.get(key, 0) for key in ("protected","r1_additional","r2_new","r3_boundary")) != 84: errors.append("frozen-corpus-cardinality")

    domains = manifest.get("runtime_identity_domains", {})
    prefixes = [domains[key]["prefix"] for key in ("runtime_id","func_id","carrier_id","value_id","location","snapshot_id","raw_span.source")]
    concrete = [value for value in walk_strings(expectations) if any(value.startswith(prefix) and value != prefix for prefix in prefixes)]
    if concrete: errors.append("concrete-runtime-identity")

    result = {"schema":"p1341-oracle-authorability-check-r4","status":"AUTHORABLE_NOT_SEALED" if not errors else "NOT_AUTHORABLE","pins":{"contract":contract_sha,"focal":focal_sha,"manifest_content":content_sha},"coverage":{"bindings":len(manifest.get("bindings", [])),"closed_key_sets":len(keys),"strict_integer_fields":len(contract.get("strict_integer_fields", [])),"external_cases":len(cases),"negative_cases":corpus.get("total")},"schema_closed":not any("closed" in error or "extension" in error for error in errors),"concrete_runtime_identity_values":len(concrete),"errors":errors,"limitations":["Authorability only; not a seal, candidate check, product verdict or reachability proof."]}
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0 if not errors else 1


if __name__ == "__main__":
    raise SystemExit(main())
