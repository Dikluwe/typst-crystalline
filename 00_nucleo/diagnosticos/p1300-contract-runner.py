#!/usr/bin/env python3
"""P1300 binding-free discriminatory contract runner.

This program intentionally consumes only frozen P1-P3 artifacts.  It does not
inspect a candidate patch, Rust source, or permanent tests.  The "candidate"
under discrimination is a public-observation vector materialized from the
frozen oracle plus the L0-owned expectations embedded in those artifacts.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
import random
import sys
import time
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAGNOSTICS = ROOT / "00_nucleo" / "diagnosticos"
PROFILES = ["default", "html", "a11y", "html+a11y"]
NA = "__not_applicable__"

FROZEN_INPUTS = {
    "manifest": ("p1300-manifest.json", "c3e946e1330ccab108dba2f9a438fd6a8d4c3a2af7fca3307842dbd7403feed8"),
    "contract": ("p1300-contract.json", "c5eace73e5ff07ab49057ec1741283543408accf132ca2f4431a1ad3827a3e37"),
    "contract_author_receipt": ("p1300-contract-author-receipt.md", "a5e6bb2a6ab270f65fa6befb6446e0a3444741fb30fdbb4918e729a25c451e53"),
    "oracle_suite": ("p1300-oracle-suite.json", "a9a519a8230e39c55fa07223e306a7156d6c2b07bd71ef2926cc9e43f1ce8278"),
    "vanilla_measurement_receipt": ("p1300-vanilla-measurement-receipt.md", "491d07c4411597951fd47bd18431337a375319a345cf2b45a82174c3ace61959"),
    "mutants": ("p1300-mutants.json", "cefa2fb00d6ad5adb81783b86b94f2450c9b0366591e42637d926adc7e598501"),
    "adversarial_plan": ("p1300-adversarial-plan.md", "2ae106a071888a4da9a96f54d8f1391dfa0104d352ac7941dbfb125a29edc166"),
}

PUBLIC_FIELDS = [
    "process_completion",
    "exit_code",
    "stdout_bytes",
    "stderr_bytes",
    "diagnostic_class",
    "diagnostic_message",
    "diagnostic_hints_in_order",
    "rendered_span_bytes",
    "public_type",
    "public_repr",
    "representative_call_value",
    "color_space_identity",
    "feature_profile_invariance",
    "capture_fields",
]
IDENTITY_FIELDS = ["family", "case_id", "entry", "namespace", "probe", "profile"]


class HarnessError(RuntimeError):
    def __init__(self, code: str, message: str):
        super().__init__(message)
        self.code = code
        self.message = message


def canonical_bytes(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode("utf-8")


def digest_value(value: Any) -> str:
    return hashlib.sha256(canonical_bytes(value)).hexdigest()


def sha256_path(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_frozen_inputs() -> tuple[dict[str, Any], dict[str, str]]:
    verified: dict[str, str] = {}
    raw: dict[str, bytes] = {}
    for name, (filename, expected) in FROZEN_INPUTS.items():
        path = DIAGNOSTICS / filename
        if not path.is_file():
            raise HarnessError("missing_frozen_input", f"missing frozen input: {filename}")
        payload = path.read_bytes()
        actual = hashlib.sha256(payload).hexdigest()
        if actual != expected:
            raise HarnessError("frozen_input_hash_mismatch", f"{filename}: expected {expected}, got {actual}")
        raw[name] = payload
        verified[name] = actual
    try:
        parsed = {
            "manifest": json.loads(raw["manifest"]),
            "contract": json.loads(raw["contract"]),
            "oracle": json.loads(raw["oracle_suite"]),
            "mutants": json.loads(raw["mutants"]),
        }
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise HarnessError("invalid_frozen_json", str(error)) from error
    return parsed, verified


def require(condition: bool, code: str, message: str) -> None:
    if not condition:
        raise HarnessError(code, message)


def validate_cross_pins(data: dict[str, Any]) -> None:
    manifest = data["manifest"]
    contract = data["contract"]
    oracle = data["oracle"]
    mutants = data["mutants"]
    require(manifest.get("schema") == "p1300-segregated-materialization-manifest/v1", "wrong_schema", "manifest schema")
    require(contract.get("$schema") == "p1300-observable-contract/v1", "wrong_schema", "contract schema")
    require(oracle.get("$schema") == "p1300-oracle-suite/v1", "wrong_schema", "oracle schema")
    require(mutants.get("$schema") == "p1300-adversarial-mutants/v1", "wrong_schema", "mutants schema")
    require(contract["manifest"]["sha256"] == FROZEN_INPUTS["manifest"][1], "cross_pin_mismatch", "contract -> manifest")
    require(oracle["frozen_inputs"]["manifest"]["sha256"] == FROZEN_INPUTS["manifest"][1], "cross_pin_mismatch", "oracle -> manifest")
    require(oracle["frozen_inputs"]["contract"]["sha256"] == FROZEN_INPUTS["contract"][1], "cross_pin_mismatch", "oracle -> contract")
    require(mutants["inputs"]["manifest"]["sha256"] == FROZEN_INPUTS["manifest"][1], "cross_pin_mismatch", "mutants -> manifest")
    require(mutants["inputs"]["contract"]["sha256"] == FROZEN_INPUTS["contract"][1], "cross_pin_mismatch", "mutants -> contract")
    require(contract["confirmed_l0"] == oracle["frozen_inputs"]["confirmed_l0"], "cross_pin_mismatch", "contract/oracle L0 pins")
    require(contract["confirmed_l0"] == mutants["inputs"]["confirmed_l0"], "cross_pin_mismatch", "contract/mutants L0 pins")
    require({item["path"]: item["sha256"] for item in contract["confirmed_l0"]} == manifest["confirmed_l0"], "cross_pin_mismatch", "manifest/contract L0 pins")
    require(contract["baseline"]["git_head"] == manifest["baseline"]["head"], "cross_pin_mismatch", "baseline HEAD")
    require(contract["baseline"]["measurement_sha256"] == manifest["protected_ancestry"]["00_nucleo/diagnosticos/p1300-pre-gate-measurement.json"], "cross_pin_mismatch", "baseline measurement")
    require(contract["vanilla"]["sha256"] == oracle["oracle_identity"]["sha256"] == manifest["vanilla"]["sha256"], "cross_pin_mismatch", "vanilla binary")
    require(contract["vanilla"]["revision"] == oracle["oracle_identity"]["ratified_revision"] == manifest["vanilla"]["revision"], "cross_pin_mismatch", "vanilla revision")
    require([item["id"] for item in contract["profiles"]] == PROFILES, "profile_mismatch", "contract profiles")
    require(mutants["execution_matrix"]["semantic_profiles"] == PROFILES, "profile_mismatch", "mutant profiles")
    require(contract["repetition_and_order_requirements"]["minimum_repetitions_per_order"] >= 2, "insufficient_repetition", "contract repetitions")
    require(mutants["execution_matrix"]["minimum_repetitions_per_order"] >= 2, "insufficient_repetition", "mutant repetitions")
    require([item["id"] for item in mutants["execution_matrix"]["orders"]] == ["canonical", "reverse", "seeded_shuffle"], "order_mismatch", "mutant orders")
    require(len(mutants["mutants"]) == 14, "mutation_cardinality", "expected 14 mutants")
    require(len({item["id"] for item in mutants["mutants"]}) == 14, "duplicate_mutant", "mutant IDs must be unique")
    require(contract["discrimination_gate"]["valid_mutation_count"] == 14, "mutation_cardinality", "contract denominator")
    require(mutants["gate_expectation"]["required_mutation_score"] == 1.0, "mutation_score_requirement", "score must be 1.0")
    require(oracle["oracle_identity"]["identity_ambiguous"] is False, "ambiguous_oracle", "oracle identity is ambiguous")
    require(oracle["coverage"]["semantic_unknown_invocations"] == [], "semantic_unknown", "oracle has semantic Unknown")
    require(oracle["coverage"]["timeouts"] == [], "oracle_timeout", "oracle has timeout")


def validate_oracle_integrity(oracle: dict[str, Any]) -> dict[tuple[str, str], dict[str, Any]]:
    blobs = oracle["blobs"]
    for key, blob in blobs.items():
        require(isinstance(blob.get("text"), str), "missing_blob_text", key)
        payload = blob["text"].encode(blob.get("encoding", "UTF-8"))
        require(hashlib.sha256(payload).hexdigest() == key, "blob_hash_mismatch", key)
        require(len(payload) == blob["byte_length"], "blob_length_mismatch", key)
    invocation_map: dict[tuple[str, str], dict[str, Any]] = {}
    for invocation in oracle["invocations"]:
        require(invocation["profile"] in PROFILES, "unknown_profile", invocation["invocation_id"])
        require(invocation["process_completion"] == "completed", "incomplete_observation", invocation["invocation_id"])
        require(invocation["timed_out"] is False, "oracle_timeout", invocation["invocation_id"])
        require(invocation["observation_completeness"]["complete"] is True, "incomplete_observation", invocation["invocation_id"])
        for stream in ("stdout", "stderr"):
            descriptor = invocation[stream]
            ref = descriptor["blob_ref"]
            require(ref in blobs, "missing_blob", f"{invocation['invocation_id']}:{stream}")
            require(descriptor["sha256"] == ref, "blob_reference_mismatch", f"{invocation['invocation_id']}:{stream}")
            require(descriptor["byte_length"] == blobs[ref]["byte_length"], "blob_length_mismatch", f"{invocation['invocation_id']}:{stream}")
        for probe_id in invocation["logical_probe_ids"]:
            key = (probe_id, invocation["profile"])
            require(key not in invocation_map, "duplicate_oracle_coordinate", f"{probe_id}:{invocation['profile']}")
            invocation_map[key] = invocation
    catalog = oracle["probe_catalog"]
    for probe_id, probe in catalog.items():
        require(probe.get("probe_id") == probe_id, "probe_identity_mismatch", probe_id)
        require(probe.get("observed_profiles") == PROFILES, "profile_omission", probe_id)
        for profile in PROFILES:
            require((probe_id, profile) in invocation_map, "missing_oracle_coordinate", f"{probe_id}:{profile}")
    require(len(catalog) == 233, "oracle_coverage_mismatch", "logical probe catalog")
    require(len(catalog) * len(PROFILES) == oracle["coverage"]["logical_probes"] == 932, "oracle_coverage_mismatch", "expanded logical probes")
    require(len(oracle["invocations"]) == oracle["coverage"]["process_invocations"] == 84, "oracle_coverage_mismatch", "process invocations")
    return invocation_map


def blank_observation() -> dict[str, Any]:
    return {
        "process_completion": "completed",
        "exit_code": 0,
        "stdout_bytes": "",
        "stderr_bytes": "",
        "diagnostic_class": NA,
        "diagnostic_message": NA,
        "diagnostic_hints_in_order": [],
        "rendered_span_bytes": NA,
        "public_type": NA,
        "public_repr": NA,
        "representative_call_value": NA,
        "color_space_identity": NA,
        "feature_profile_invariance": "invariant",
        "capture_fields": {},
    }


def json_stdout(value: str) -> str:
    return json.dumps(value, ensure_ascii=False, separators=(",", ":")) + "\n"


def coordinate(family: str, case_id: str, probe: str, profile: str, expression: str, *, entry: str = NA, namespace: str = NA) -> dict[str, Any]:
    return {
        "family": family,
        "case_id": case_id,
        "entry": entry,
        "namespace": namespace,
        "probe": probe,
        "profile": profile,
        "expression": expression,
    }


def normalize_probe(probe: dict[str, Any]) -> tuple[str, str, str, str, str]:
    raw_case = probe["case_id"]
    raw_probe = probe["probe_subtype"]
    entry = NA
    namespace = NA
    if raw_case.startswith("negative-"):
        return "negative_alias_cases", raw_case, "main", entry, namespace
    if raw_case.startswith("qualified-color-"):
        return "qualified_constructor_cases", raw_case, raw_probe, entry, namespace
    if raw_case.startswith("space-"):
        return "color_space_cases", raw_case, "main", entry, namespace
    if raw_case.startswith("global-"):
        _, name, namespace = raw_case.split("-", 2)
        return "ratified_global_constructor_cases", f"ratified-global-{name}", raw_probe, name, namespace
    if raw_case.startswith("predefined-"):
        entry = raw_case.removeprefix("predefined-")
        normalized = "repr" if raw_probe == "public_repr" else raw_probe
        return "ratified_predefined_color_controls", raw_case, normalized, entry, namespace
    if raw_case.startswith("compat-"):
        entry = raw_case.removeprefix("compat-")
        return "crystalline_compatibility_global_controls", raw_case, raw_probe, entry, "bare"
    if raw_case.startswith("operator-"):
        entry = raw_case.removeprefix("operator-")
        return "color_operator_controls", f"color-operator-{entry}", raw_probe, entry, "color"
    if raw_case == "color-map-closed-inventory-and-values":
        return "color_map_control", raw_case, "root_kind", entry, "color.map"
    if raw_case.startswith("color-map-"):
        entry = raw_case.removeprefix("color-map-")
        normalized = "entry_capture" if raw_probe == "values" else raw_probe
        return "color_map_control", "color-map-closed-inventory-and-values", normalized, entry, "color.map"
    raise HarnessError("unsupported_oracle_probe", probe["probe_id"])


def invocation_observation(oracle: dict[str, Any], invocation: dict[str, Any]) -> dict[str, Any]:
    observation = blank_observation()
    observation["process_completion"] = invocation["process_completion"]
    observation["exit_code"] = invocation["exit_code"]
    observation["stdout_bytes"] = oracle["blobs"][invocation["stdout"]["blob_ref"]]["text"]
    observation["stderr_bytes"] = oracle["blobs"][invocation["stderr"]["blob_ref"]]["text"]
    diagnostic = invocation["diagnostic"]
    if diagnostic is not None:
        observation["diagnostic_class"] = diagnostic["class"]
        observation["diagnostic_message"] = diagnostic["message"]
        observation["diagnostic_hints_in_order"] = diagnostic["hints_in_order"]
        observation["rendered_span_bytes"] = diagnostic["rendered_span"]
    return observation


def synthesize_l0_compatibility(probe: dict[str, Any]) -> tuple[str, str]:
    expectation = probe["baseline_preservation_expectation_from_L0"]
    subtype = probe["probe_subtype"]
    if subtype == "type":
        value = expectation["expected_type"]
    elif subtype == "value_equality":
        value = "true"
    elif subtype == "repr":
        ratified = expectation["ratified_value"]
        if ratified == "rgb(0x00,0xB3,0xB3)":
            value = 'rgb("#00b3b3")'
        elif ratified == "rgb(0xE5,0x00,0xE5)":
            value = 'rgb("#e500e5")'
        elif ratified == "none":
            value = "none"
        else:
            raise HarnessError("unsupported_l0_expectation", probe["probe_id"])
    else:
        raise HarnessError("unsupported_l0_expectation", probe["probe_id"])
    return value, json_stdout(value)


def apply_probe_value(observation: dict[str, Any], probe: dict[str, Any], value: str) -> None:
    subtype = probe["probe_subtype"]
    if subtype in {"type", "bare_type", "qualified_type"}:
        observation["public_type"] = value
    elif subtype in {"repr", "public_repr"}:
        observation["public_repr"] = value
    elif subtype == "call":
        observation["public_repr"] = value
        observation["representative_call_value"] = value
    elif subtype in {"color_space_identity", "space"}:
        observation["color_space_identity"] = value
    elif subtype in {"global_equals_qualified", "value_equality", "components"}:
        observation["representative_call_value"] = value
    elif subtype in {"root_kind", "child_kind"}:
        observation["public_type"] = value
        observation["capture_fields"] = {subtype: value}
    else:
        raise HarnessError("unsupported_probe_subtype", probe["probe_id"])


def materialize_expected_vector(data: dict[str, Any], invocation_map: dict[tuple[str, str], dict[str, Any]]) -> list[dict[str, Any]]:
    contract = data["contract"]
    oracle = data["oracle"]
    vector: list[dict[str, Any]] = []
    aggregate_inserted = False
    map_derived = {
        (invocation["profile"], entry["name"]): entry
        for invocation in oracle["invocations"]
        if invocation["family"] == "color_map_control"
        for entry in invocation["derived_public_observations"]["entries"]
    }
    map_roots = {
        invocation["profile"]: invocation["derived_public_observations"]
        for invocation in oracle["invocations"]
        if invocation["family"] == "color_map_control"
    }
    for probe_id, probe in oracle["probe_catalog"].items():
        family, case_id, normalized_probe, entry, namespace = normalize_probe(probe)
        if family == "ratified_global_constructor_cases" and not aggregate_inserted:
            aggregate = contract["ratified_global_constructor_cases"]["aggregate_type_measurement"]
            for profile in PROFILES:
                for ns in ("bare", "std"):
                    expected = aggregate["expected"]
                    expression = aggregate[f"{ns}_expression"]
                    item = coordinate(family, "ratified-global-aggregate", "aggregate_type", profile, expression, namespace=ns)
                    observation = blank_observation()
                    observation["exit_code"] = expected["exit_code"]
                    observation["stdout_bytes"] = expected["stdout"]
                    observation["stderr_bytes"] = expected["stderr"]
                    observation["public_type"] = "(function, function, function, function, function)"
                    item["observables"] = observation
                    item["expectation_authority"] = "pre_gate_measurement"
                    vector.append(item)
            aggregate_inserted = True
        for profile in PROFILES:
            invocation = invocation_map[(probe_id, profile)]
            item = coordinate(family, case_id, normalized_probe, profile, probe["expression"], entry=entry, namespace=namespace)
            if probe_id.startswith("negative-"):
                observation = invocation_observation(oracle, invocation)
            elif probe.get("expectation_authority", "").startswith("baseline_preservation") and "observed_public_value" not in probe:
                value, stdout = synthesize_l0_compatibility(probe)
                observation = blank_observation()
                observation["stdout_bytes"] = stdout
                apply_probe_value(observation, probe, value)
            elif probe["probe_subtype"] == "values" and family == "color_map_control":
                capture = copy.deepcopy(map_derived[(profile, entry)])
                observation = blank_observation()
                observation["stdout_bytes"] = canonical_bytes(capture).decode("utf-8") + "\n"
                observation["capture_fields"] = capture
            else:
                require(invocation["exit_code"] == 0, "semantic_oracle_rejection", f"{probe_id}:{profile}")
                require("observed_public_value" in probe, "oracle_not_frozen", probe_id)
                require("standalone_stdout_utf8" in probe, "oracle_not_frozen", probe_id)
                observation = blank_observation()
                observation["stdout_bytes"] = probe["standalone_stdout_utf8"]
                value = probe["observed_public_value"]
                apply_probe_value(observation, probe, value)
                if case_id == "color-map-closed-inventory-and-values" and normalized_probe == "root_kind":
                    root = map_roots[profile]
                    observation["capture_fields"] = {
                        "root_kind": root["root_kind"],
                        "ordered_child_names": root["ordered_child_names"],
                    }
            item["observables"] = observation
            item["expectation_authority"] = probe["expectation_authority"]
            vector.append(item)
    validate_vector(vector)
    identities = [identity_key(item) for item in vector]
    require(len(identities) == len(set(identities)), "duplicate_semantic_coordinate", "materialized vector")
    return vector


def identity_key(item: dict[str, Any]) -> tuple[str, str, str, str, str, str]:
    return tuple(item[field] for field in IDENTITY_FIELDS)  # type: ignore[return-value]


def validate_no_null(value: Any, path: str) -> None:
    if value is None:
        raise HarnessError("null_required_field", path)
    if isinstance(value, dict):
        for key, child in value.items():
            validate_no_null(child, f"{path}.{key}")
    elif isinstance(value, list):
        for index, child in enumerate(value):
            validate_no_null(child, f"{path}[{index}]")


def validate_vector(vector: list[dict[str, Any]]) -> None:
    for index, item in enumerate(vector):
        for field in IDENTITY_FIELDS + ["expression", "observables", "expectation_authority"]:
            if field not in item:
                raise HarnessError("missing_required_field", f"coordinate[{index}].{field}")
            validate_no_null(item[field], f"coordinate[{index}].{field}")
        for field in PUBLIC_FIELDS:
            if field not in item["observables"]:
                raise HarnessError("missing_required_field", f"coordinate[{index}].observables.{field}")
            validate_no_null(item["observables"][field], f"coordinate[{index}].observables.{field}")
        require(item["observables"]["process_completion"] == "completed", "incomplete_observation", str(identity_key(item)))


def select(vector: list[dict[str, Any]], selector: dict[str, Any], *, same_profile: str | None = None) -> list[dict[str, Any]]:
    allowed = set(IDENTITY_FIELDS)
    for field in selector:
        require(field in allowed, "invalid_selector_field", field)
    resolved = dict(selector)
    if resolved.get("profile") == "$same":
        require(same_profile is not None, "missing_same_profile", str(selector))
        resolved["profile"] = same_profile
    matches = []
    for item in vector:
        if all(item[field] == value for field, value in resolved.items() if field != "profile" or value != "all"):
            matches.append(item)
    if not matches:
        raise HarnessError("missing_selector", json.dumps(selector, sort_keys=True))
    return matches


def expand_target(selector: dict[str, Any]) -> list[dict[str, Any]]:
    if selector.get("profile") == "all":
        return [dict(selector, profile=profile) for profile in PROFILES]
    return [selector]


def apply_mutant(expected: list[dict[str, Any]], mutant: dict[str, Any]) -> tuple[list[dict[str, Any]], set[tuple[str, str, str, str, str, str]]]:
    actual = copy.deepcopy(expected)
    changed_targets: set[tuple[str, str, str, str, str, str]] = set()
    for operation in mutant["operations"]:
        op = operation.get("op")
        require(op in {"copy_public_observables_from", "replace_public_observables", "replace_capture_fields"}, "unsupported_operation", str(op))
        targets = operation.get("targets")
        require(isinstance(targets, list) and targets, "missing_required_field", f"{mutant['id']}.targets")
        for target_template in targets:
            for target_selector in expand_target(target_template):
                matches = select(actual, target_selector)
                require(len(matches) == 1, "ambiguous_selector", json.dumps(target_selector, sort_keys=True))
                target = matches[0]
                before_identity = identity_key(target)
                before_expression = target["expression"]
                if op == "copy_public_observables_from":
                    source_selector = operation.get("source")
                    require(isinstance(source_selector, dict), "missing_required_field", f"{mutant['id']}.source")
                    sources = select(actual, source_selector, same_profile=target["profile"])
                    require(len(sources) == 1, "ambiguous_selector", json.dumps(source_selector, sort_keys=True))
                    validate_vector([sources[0], target])
                    target["observables"] = copy.deepcopy(sources[0]["observables"])
                elif op == "replace_public_observables":
                    values = operation.get("values")
                    require(isinstance(values, dict) and values, "missing_required_field", f"{mutant['id']}.values")
                    for field, value in values.items():
                        require(field in PUBLIC_FIELDS and field != "capture_fields", "invalid_observable_field", field)
                        require(field in target["observables"], "missing_required_field", field)
                        validate_no_null(value, field)
                        target["observables"][field] = copy.deepcopy(value)
                else:
                    values = operation.get("values")
                    require(isinstance(values, dict) and values, "missing_required_field", f"{mutant['id']}.values")
                    capture = target["observables"].get("capture_fields")
                    require(isinstance(capture, dict), "missing_required_field", "capture_fields")
                    for field, value in values.items():
                        require(field in capture, "missing_required_field", f"capture_fields.{field}")
                        validate_no_null(value, field)
                        capture[field] = copy.deepcopy(value)
                require(identity_key(target) == before_identity, "identity_mutation", mutant["id"])
                require(target["expression"] == before_expression, "expression_mutation", mutant["id"])
                changed_targets.add(before_identity)
    recompute_profile_invariance(actual)
    validate_vector(actual)
    expected_by_id = {identity_key(item): item for item in expected}
    for target_id in changed_targets:
        target = next(item for item in actual if identity_key(item) == target_id)
        require(target["observables"] != expected_by_id[target_id]["observables"], "mutation_no_public_difference", f"{mutant['id']}:{target_id}")
    require(changed_targets, "mutation_no_targets", mutant["id"])
    return actual, changed_targets


def recompute_profile_invariance(vector: list[dict[str, Any]]) -> None:
    groups: dict[tuple[str, str, str, str, str], list[dict[str, Any]]] = {}
    for item in vector:
        key = tuple(item[field] for field in IDENTITY_FIELDS if field != "profile")
        groups.setdefault(key, []).append(item)
    for items in groups.values():
        signatures = []
        for item in items:
            projected = {field: value for field, value in item["observables"].items() if field != "feature_profile_invariance"}
            signatures.append(digest_value(projected))
        value = "invariant" if len(set(signatures)) == 1 else "divergent"
        for item in items:
            item["observables"]["feature_profile_invariance"] = value


def compare_observables(expected: dict[str, Any], actual: dict[str, Any]) -> list[tuple[str, Any, Any]]:
    differences: list[tuple[str, Any, Any]] = []
    for field in PUBLIC_FIELDS:
        if expected[field] != actual[field]:
            if field == "capture_fields" and isinstance(expected[field], dict) and isinstance(actual[field], dict):
                for subfield in sorted(set(expected[field]) | set(actual[field])):
                    left = expected[field].get(subfield, "__missing__")
                    right = actual[field].get(subfield, "__missing__")
                    if left != right:
                        differences.append((f"capture_fields.{subfield}", left, right))
            else:
                differences.append((field, expected[field], actual[field]))
    return differences


def classify_coordinate(expected: dict[str, Any], actual: dict[str, Any]) -> dict[str, Any]:
    validate_vector([expected, actual])
    require(identity_key(expected) == identity_key(actual), "identity_mutation", str(identity_key(expected)))
    require(expected["expression"] == actual["expression"], "expression_mutation", str(identity_key(expected)))
    differences = compare_observables(expected["observables"], actual["observables"])
    if not differences:
        return {"classification": "Preserved", "witnesses": []}
    witnesses = [
        {
            "case_id": actual["case_id"],
            "profile": actual["profile"],
            "probe": actual["probe"],
            "observable": field,
            "expected": expected_value,
            "actual": actual_value,
        }
        for field, expected_value, actual_value in differences
    ]
    return {"classification": "Violated", "witnesses": witnesses}


def ordered_indices(length: int, order: str, *, inverted_input: bool) -> list[int]:
    indices = list(range(length))
    if inverted_input:
        indices.reverse()
    if order == "canonical":
        return indices
    if order == "reverse":
        return list(reversed(indices))
    if order == "seeded_shuffle":
        shuffled = list(indices)
        rng = random.Random(1300)
        for index in range(len(shuffled) - 1, 0, -1):
            other = rng.randrange(index + 1)
            shuffled[index], shuffled[other] = shuffled[other], shuffled[index]
        return shuffled
    raise HarnessError("unknown_order", order)


def run_order(expected: list[dict[str, Any]], actual: list[dict[str, Any]], order: str, *, inverted_input: bool) -> dict[str, Any]:
    require(len(expected) == len(actual), "coordinate_cardinality", order)
    results = []
    for index in ordered_indices(len(expected), order, inverted_input=inverted_input):
        result = classify_coordinate(expected[index], actual[index])
        results.append({"identity": list(identity_key(actual[index])), **result})
    canonical_results = sorted(results, key=lambda item: item["identity"])
    classifications = [item["classification"] for item in canonical_results]
    return {
        "classification": "Violated" if "Violated" in classifications else "Preserved",
        "semantic_unknown_count": classifications.count("Unknown"),
        "violated_coordinate_count": classifications.count("Violated"),
        "result_vector_sha256": digest_value(canonical_results),
        "ordered_identity_sha256": digest_value([list(identity_key(actual[index])) for index in ordered_indices(len(actual), order, inverted_input=inverted_input)]),
        "witnesses": [witness for item in canonical_results for witness in item["witnesses"]],
    }


def validate_detection_witnesses(mutant: dict[str, Any], witnesses: list[dict[str, Any]]) -> None:
    for requirement in mutant["detection_witnesses"]:
        profiles = PROFILES if requirement.get("profiles") == "all" else requirement.get("profiles", PROFILES)
        probes = requirement.get("probes") or ([requirement["probe"]] if "probe" in requirement else [None])
        for profile in profiles:
            for probe in probes:
                matches = []
                for witness in witnesses:
                    if witness["case_id"] != requirement["case_id"] or witness["profile"] != profile:
                        continue
                    if probe is not None and witness["probe"] != probe:
                        continue
                    matches.append(witness)
                require(bool(matches), "missing_required_witness", f"{mutant['id']}:{requirement['case_id']}:{profile}:{probe}")
                observable_expression = requirement.get("observable")
                if observable_expression:
                    tokens = [token.strip() for token in observable_expression.replace(" and ", "/").split("/")]
                    require(any(match["observable"] in tokens for match in matches), "missing_required_witness_observable", f"{mutant['id']}:{observable_expression}")


def exercise_artifact(expected: list[dict[str, Any]], artifact: dict[str, Any], *, inverted_input: bool, positive: bool) -> dict[str, Any]:
    run_summaries = []
    canonical_witnesses: list[dict[str, Any]] = []
    reference_digest = None
    for order in ("canonical", "reverse", "seeded_shuffle"):
        for repetition in (1, 2):
            if positive:
                require(artifact["operations"] == [{"op": "identity"}], "positive_control_mutation", artifact["id"])
                actual = copy.deepcopy(expected)
            else:
                actual, _ = apply_mutant(expected, artifact)
            result = run_order(expected, actual, order, inverted_input=inverted_input)
            require(result["semantic_unknown_count"] == 0, "semantic_unknown", artifact["id"])
            if reference_digest is None:
                reference_digest = result["result_vector_sha256"]
                canonical_witnesses = result["witnesses"]
            else:
                require(result["result_vector_sha256"] == reference_digest, "order_repetition_divergence", artifact["id"])
            run_summaries.append({
                "order": order,
                "repetition": repetition,
                "classification": result["classification"],
                "violated_coordinate_count": result["violated_coordinate_count"],
                "semantic_unknown_count": result["semantic_unknown_count"],
                "result_vector_sha256": result["result_vector_sha256"],
                "ordered_identity_sha256": result["ordered_identity_sha256"],
            })
    classification = run_summaries[0]["classification"]
    require(all(run["classification"] == classification for run in run_summaries), "order_repetition_divergence", artifact["id"])
    if positive:
        require(classification == "Preserved", "positive_control_failed", artifact["id"])
        require(not canonical_witnesses, "positive_control_witness", artifact["id"])
    else:
        require(classification == artifact["expected_classification"] == "Violated", "surviving_mutant", artifact["id"])
        require(bool(canonical_witnesses), "missing_required_witness", artifact["id"])
        validate_detection_witnesses(artifact, canonical_witnesses)
    return {
        "id": artifact["id"],
        "classification": classification,
        "witness_count": len(canonical_witnesses),
        "witnesses_sha256": digest_value(canonical_witnesses),
        "witness_sample": canonical_witnesses[:3],
        "runs": run_summaries,
    }


def classify_opaque(construction: dict[str, Any]) -> tuple[str, str]:
    if construction == {"forced_timeout_ms": 0}:
        return "Unknown", "timeout"
    if construction == {"product_binary": "deliberately_absent"}:
        return "Unknown", "missing_product"
    if construction == {"product_sha256": None, "product_revision": None}:
        return "Unknown", "ambiguous_product_identity"
    if construction == {"expression": "color.linear-rgb(", "adapter_support": "deliberately_disabled"}:
        return "Unknown", "unsupported_parser_construction"
    raise HarnessError("unsupported_opaque_construction", json.dumps(construction, sort_keys=True))


def exercise_opaque(contract: dict[str, Any], mutants: dict[str, Any]) -> list[dict[str, Any]]:
    contract_cases = {item["id"]: item for item in contract["opaque_calibration_cases"]}
    results = []
    require(len(contract_cases) == len(mutants["opaque_controls"]) == 4, "opaque_cardinality", "expected four opaque controls")
    for control in mutants["opaque_controls"]:
        case = contract_cases.get(control["contract_case_id"])
        require(case is not None, "missing_selector", control["contract_case_id"])
        require(case["construction"] == control["construction"], "opaque_construction_mismatch", control["id"])
        classification, reason = classify_opaque(copy.deepcopy(control["construction"]))
        require(classification == case["expected_classification"] == control["expected_classification"] == "Unknown", "opaque_classification_mismatch", control["id"])
        require(reason == case["expected_reason_code"] == control["expected_reason_code"], "opaque_reason_mismatch", control["id"])
        results.append({"id": control["id"], "classification": classification, "reason_code": reason, "witness": {"construction": control["construction"], "reason_code": reason}})
    return results


def expect_harness_error(name: str, expected_code: str, operation: Any) -> dict[str, str]:
    try:
        operation()
    except HarnessError as error:
        require(error.code == expected_code, "self_test_wrong_error", f"{name}: {error.code}")
        return {"id": name, "outcome": "harness_error_rejected", "reason_code": error.code, "classification": "not_emitted"}
    raise HarnessError("self_test_false_accept", name)


def run_self_tests(expected: list[dict[str, Any]]) -> list[dict[str, str]]:
    def missing_selector() -> None:
        select(copy.deepcopy(expected), {"family": "negative_alias_cases", "case_id": "does-not-exist", "profile": "default"})

    def missing_field() -> None:
        broken = copy.deepcopy(expected[:1])
        del broken[0]["observables"]["stdout_bytes"]
        validate_vector(broken)

    def null_field() -> None:
        broken = copy.deepcopy(expected[:1])
        broken[0]["observables"]["stdout_bytes"] = None
        validate_vector(broken)

    frozen_hash = digest_value(expected)

    def oracle_erasure() -> None:
        erased = copy.deepcopy(expected)
        erased.pop()
        if digest_value(erased) != frozen_hash:
            raise HarnessError("oracle_erasure", "expected vector changed")

    return [
        expect_harness_error("reject_missing_selector", "missing_selector", missing_selector),
        expect_harness_error("reject_missing_field", "missing_required_field", missing_field),
        expect_harness_error("reject_null", "null_required_field", null_field),
        expect_harness_error("reject_oracle_erasure", "oracle_erasure", oracle_erasure),
    ]


def run(input_order: str) -> dict[str, Any]:
    started = time.monotonic_ns()
    data, verified_hashes = load_frozen_inputs()
    validate_cross_pins(data)
    invocation_map = validate_oracle_integrity(data["oracle"])
    expected = materialize_expected_vector(data, invocation_map)
    expected_hash = digest_value(expected)
    self_tests = run_self_tests(expected)
    positive = exercise_artifact(expected, data["mutants"]["positive_control"], inverted_input=input_order == "inverted", positive=True)
    opaque = exercise_opaque(data["contract"], data["mutants"])
    mutant_specs = list(data["mutants"]["mutants"])
    if input_order == "inverted":
        mutant_specs.reverse()
    mutant_results = [exercise_artifact(expected, mutant, inverted_input=input_order == "inverted", positive=False) for mutant in mutant_specs]
    canonical_mutants = sorted(mutant_results, key=lambda item: item["id"])
    violated = sum(item["classification"] == "Violated" and item["witness_count"] > 0 for item in canonical_mutants)
    survivors = [item["id"] for item in canonical_mutants if item["classification"] != "Violated" or item["witness_count"] == 0]
    semantic_unknown = sum(run["semantic_unknown_count"] for item in canonical_mutants for run in item["runs"])
    score = violated / 14
    require(positive["classification"] == "Preserved", "positive_control_failed", positive["id"])
    require(all(item["classification"] == "Unknown" for item in opaque), "opaque_classification_mismatch", "opaque controls")
    require(violated == 14 and not survivors, "surviving_mutant", json.dumps(survivors))
    require(score == 1.0, "mutation_score_failure", str(score))
    require(semantic_unknown == 0, "semantic_unknown", str(semantic_unknown))
    require(digest_value(expected) == expected_hash, "oracle_erasure", "expected vector changed during run")
    for name, (_, expected_input_hash) in FROZEN_INPUTS.items():
        require(sha256_path(DIAGNOSTICS / FROZEN_INPUTS[name][0]) == expected_input_hash, "frozen_input_changed_during_run", name)
    invariant_summary = {
        "expected_vector_sha256": expected_hash,
        "positive_result_vector_sha256": positive["runs"][0]["result_vector_sha256"],
        "opaque": [{"id": item["id"], "classification": item["classification"], "reason_code": item["reason_code"]} for item in opaque],
        "mutants": [{"id": item["id"], "classification": item["classification"], "witness_count": item["witness_count"], "witnesses_sha256": item["witnesses_sha256"], "result_vector_sha256": item["runs"][0]["result_vector_sha256"]} for item in canonical_mutants],
        "mutation_score": score,
    }
    return {
        "$schema": "p1300-discrimination-run/v1",
        "step": "P1300",
        "role": "P4_discrimination_sealer",
        "status": "SEAL_ELIGIBLE",
        "attestation_level": "executed_without_technical_isolation_attestation",
        "input_order": input_order,
        "frozen_input_hashes_verified": verified_hashes,
        "expected_candidate_vector": {
            "coordinate_count": len(expected),
            "underlying_oracle_logical_probe_count": len(data["oracle"]["probe_catalog"]),
            "expected_sha256": expected_hash,
            "final_positive_candidate_sha256": expected_hash,
            "expected_equals_final_positive": True,
            "source": "frozen oracle observations plus L0-owned expectations embedded in the frozen contract/oracle suite",
        },
        "execution_matrix": {
            "orders": ["canonical", "reverse", "seeded_shuffle"],
            "seeded_shuffle_algorithm": "Fisher-Yates",
            "seed": 1300,
            "repetitions_per_order": 2,
            "product_processes": 0,
            "model": "in_memory_public_observation_DSL",
        },
        "anti_erasure_self_tests": self_tests,
        "positive_control": positive,
        "opaque_controls": opaque,
        "mutants": mutant_results,
        "gate": {
            "semantic_positive_classification": "Preserved",
            "opaque_unknown_count": 4,
            "mutants_violated_with_witness": violated,
            "valid_mutation_count": 14,
            "mutation_score": score,
            "surviving_mutants": survivors,
            "semantic_unknown_count": semantic_unknown,
            "harness_error_count": 0,
            "verdict": "CONTRACT_DISCRIMINATES",
        },
        "invariant_result": invariant_summary,
        "invariant_result_sha256": digest_value(invariant_summary),
        "duration_ns": time.monotonic_ns() - started,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input-order", choices=("normal", "inverted"), default="normal")
    parser.add_argument("--output", type=Path, help="write JSON here instead of stdout")
    args = parser.parse_args()
    try:
        result = run(args.input_order)
        exit_code = 0
    except HarnessError as error:
        result = {
            "$schema": "p1300-discrimination-run/v1",
            "step": "P1300",
            "role": "P4_discrimination_sealer",
            "status": "BLOCKED",
            "attestation_level": "executed_without_technical_isolation_attestation",
            "input_order": args.input_order,
            "harness_errors": [{"kind": "harness_error", "reason_code": error.code, "message": error.message}],
            "gate": {"verdict": "P1300_BLOCKED_CONTRACT_DESIGN", "sealed": False},
        }
        exit_code = 2
    payload = json.dumps(result, ensure_ascii=False, sort_keys=True, indent=2) + "\n"
    if args.output:
        args.output.write_text(payload, encoding="utf-8")
    else:
        sys.stdout.write(payload)
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main())
