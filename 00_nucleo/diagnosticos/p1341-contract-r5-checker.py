#!/usr/bin/env python3
"""P1341 R5 closed-schema checker for the frozen 84 materialized attacks."""

from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import sys
from pathlib import Path
from typing import Any


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def import_module(path: Path, name: str):
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def violated(code: str) -> tuple[str, str]:
    return "Violated", code


def exact_keys(value: Any, expected: list[str]) -> bool:
    return isinstance(value, dict) and set(value) == set(expected)


def strict_int(value: Any) -> bool:
    return isinstance(value, int) and not isinstance(value, bool)


def string_list(value: Any) -> bool:
    return isinstance(value, list) and all(isinstance(item, str) for item in value)


def typed_closed(value: Any, keys: dict[str, Any]) -> bool:
    if not isinstance(value, dict) or not isinstance(value.get("tag"), str):
        return False
    if value["tag"] == "dict":
        if not exact_keys(value, keys["typed_dict"]) or not isinstance(value["pairs"], list):
            return False
        names = []
        for pair in value["pairs"]:
            if not isinstance(pair, list) or len(pair) != 2 or not isinstance(pair[0], str) or not typed_closed(pair[1], keys):
                return False
            names.append(pair[0])
        return len(names) == len(set(names))
    if value["tag"] not in {"int", "str", "bool"} or not exact_keys(value, keys["typed_scalar"]):
        return False
    scalar = value["value"]
    if value["tag"] == "int":
        return strict_int(scalar)
    if value["tag"] == "str":
        return isinstance(scalar, str)
    return isinstance(scalar, bool)


def closed_schema_and_types(contract: dict[str, Any], expectation: Any, ledger: Any) -> tuple[str, str] | None:
    keys = contract["closed_key_sets"]
    if not exact_keys(expectation, keys["expectation"]):
        return violated("closed-expectation-keys")
    if not exact_keys(expectation.get("cell_selector"), keys["cell_selector"]):
        return violated("closed-cell-selector-keys")
    coordinates = expectation.get("source_coordinates")
    if not isinstance(coordinates, list) or not coordinates:
        return violated("source-coordinate-list")
    for row in coordinates:
        if not exact_keys(row, keys["source_coordinate"]) or not strict_int(row["context_ordinal"]) or not strict_int(row["source_byte_offset"]) or not isinstance(row["lexical_role"], str):
            return violated("closed-source-coordinate")
    roles = expectation.get("required_roles")
    cardinalities = expectation.get("role_cardinalities")
    if not string_list(roles) or not isinstance(cardinalities, dict) or set(cardinalities) != set(roles) or any(not strict_int(value) for value in cardinalities.values()):
        return violated("strict-role-cardinalities")
    features = expectation.get("feature_witnesses")
    if not isinstance(features, list):
        return violated("feature-witness-list")
    for row in features:
        if not exact_keys(row, keys["feature_witness"]) or not isinstance(row["stable_role"], str) or not isinstance(row["origin"], str) or not string_list(row["features"]) or not strict_int(row["multiplicity_per_attempt"]):
            return violated("closed-feature-witness")
    callbacks = expectation.get("callbacks")
    if not isinstance(callbacks, list):
        return violated("callback-list")
    for row in callbacks:
        if not exact_keys(row, keys["callback"]) or any(not isinstance(row[field], str) for field in set(keys["callback"]) - {"multiplicity_per_attempt"}) or not strict_int(row["multiplicity_per_attempt"]):
            return violated("closed-callback")
    counters = expectation.get("counter_occurrences")
    if not isinstance(counters, list):
        return violated("counter-expectation-list")
    for row in counters:
        if not exact_keys(row, keys["counter_occurrence_expectation"]):
            return violated("closed-counter-expectation")
        if not strict_int(row["occurrence_ordinal"]) or not strict_int(row["multiplicity_per_attempt"]):
            return violated("strict-counter-expectation-integer")
        coordinate = row["source_coordinate"]
        if not exact_keys(coordinate, keys["counter_source_coordinate"]) or not strict_int(coordinate["source_byte_offset"]) or not strict_int(coordinate["byte_length"]) or not isinstance(coordinate["lexical_role"], str):
            return violated("closed-counter-source-coordinate")
    dicts = expectation.get("dict_expectations")
    if not isinstance(dicts, list):
        return violated("dict-expectation-list")
    for row in dicts:
        if not exact_keys(row, keys["dict_expectation"]) or not isinstance(row["stable_role"], str) or not string_list(row["typed_path"]) or not typed_closed(row["ordered_typed_value"], keys):
            return violated("closed-dict-expectation")
    edges = expectation.get("required_causal_role_edges")
    if not isinstance(edges, list) or any(not isinstance(edge, list) or len(edge) != 2 or not string_list(edge) for edge in edges):
        return violated("closed-required-role-edges")

    if not exact_keys(ledger, keys["ledger"]):
        return violated("closed-ledger-keys")
    objects = ledger.get("objects")
    events = ledger.get("events")
    if not isinstance(objects, list) or not isinstance(events, list):
        return violated("ledger-list-types")
    extensions = contract["object_role_extensions"]
    for row in objects:
        if not isinstance(row, dict) or not isinstance(row.get("stable_role"), str):
            return violated("object-shape")
        expected = keys["object_base"] + extensions.get(row["stable_role"], [])
        if not exact_keys(row, expected):
            return violated("closed-object-keys")
    for event in events:
        if not isinstance(event, dict) or event.get("kind") not in keys["event"] or not exact_keys(event, keys["event"][event["kind"]]):
            return violated("closed-event-keys")
        if not strict_int(event["seq"]):
            return violated("strict-event-seq")
        if event["kind"] == "feature-origin" and not strict_int(event["witness_ordinal"]):
            return violated("strict-feature-witness-ordinal")
        if event["kind"] == "counter-occurrence":
            if not strict_int(event["occurrence_ordinal"]):
                return violated("strict-counter-event-ordinal")
            span = event["raw_span"]
            if not exact_keys(span, keys["raw_span"]) or not strict_int(span["start"]) or not strict_int(span["end"]):
                return violated("closed-raw-span")
        if event["kind"] == "dict-value" and (not string_list(event["typed_path"]) or not typed_closed(event["typed_value"], keys)):
            return violated("closed-dict-event")
    causal = ledger.get("causal_edges")
    if not isinstance(causal, list) or any(not exact_keys(edge, keys["causal_edge"]) for edge in causal):
        return violated("closed-causal-edge")
    payload = ledger.get("payload_state")
    if not isinstance(payload, dict) or payload.get("kind") not in {"inspectable", "opaque"}:
        return violated("payload-kind")
    payload_keys = keys["payload_opaque"] if payload["kind"] == "opaque" else keys["payload_inspectable"]
    if not exact_keys(payload, payload_keys):
        return violated("closed-payload-keys")
    return None


def _classify(contract: dict[str, Any], expectation: Any, ledger: Any, r4_contract: dict[str, Any], r4_checker: Any, r3_contract: dict[str, Any], r3_checker: Any) -> tuple[str, str]:
    problem = closed_schema_and_types(contract, expectation, ledger)
    if problem:
        return problem
    events = ledger["events"]
    feature_positions = [index for index, event in enumerate(events) if event["kind"] == "feature-origin"]
    position = {event["kind"]: index for index, event in enumerate(events) if event["kind"] != "feature-origin"}
    if len(feature_positions) != contract["temporal_order"]["feature_cardinality"]:
        return violated("feature-temporal-cardinality")
    if not (position["execution-start"] < min(feature_positions) <= max(feature_positions) < position["callback-dispatch"] < position["function-invoked"] < position["body-started"] < position["counter-occurrence"] < position["dict-value"] < position["execution-end"]):
        return violated("feature-and-lifecycle-temporal-order")
    return r4_checker.classify(r4_contract, expectation, ledger, r3_contract, r3_checker)


def classify(contract: dict[str, Any], expectation: Any, ledger: Any, r4_contract: dict[str, Any], r4_checker: Any, r3_contract: dict[str, Any], r3_checker: Any) -> tuple[str, str]:
    try:
        return _classify(contract, expectation, ledger, r4_contract, r4_checker, r3_contract, r3_checker)
    except Exception as error:
        return violated("malformed-fail-closed:" + type(error).__name__)


def build_fixture(root: Path, contract: dict[str, Any], focal: dict[str, Any]):
    errors = []
    for key in ("base_contract", "base_focal", "base_checker", "materializer_r3"):
        path = root / focal[key][0]
        if sha256(path) != focal[key][1]:
            errors.append("protected-input-hash:" + key)
    r4_contract = load_json(root / focal["base_contract"][0])
    r4_focal = load_json(root / focal["base_focal"][0])
    r4_checker = import_module(root / focal["base_checker"][0], "p1341_r4_checker_for_r5")
    expectation, ledger, r3_contract, r3_checker, r1, r2, inherited_errors = r4_checker.build_fixture(root, r4_contract, r4_focal)
    errors.extend(inherited_errors)
    r3_suite = import_module(root / focal["materializer_r3"][0], "p1341_r3_suite_for_r5")
    return expectation, ledger, r4_contract, r4_checker, r3_contract, r3_checker, r1, r2, r3_suite, errors


def run(contract_path: Path, focal_path: Path) -> dict[str, Any]:
    root = contract_path.resolve().parents[2]
    contract = load_json(contract_path)
    focal = load_json(focal_path)
    errors = []
    if contract.get("schema") != "p1341-contract-r5-v1" or contract.get("status") != "NOT_SEALED" or contract.get("extension_policy", "").split(":", 1)[0] != "NONE":
        errors.append("contract-header")
    if focal.get("schema") != "p1341-contract-r5-focal-v1":
        errors.append("focal-schema")
    base = root / contract["base_contract"][0]
    if sha256(base) != contract["base_contract"][1]:
        errors.append("base-contract-pin")
    if contract.get("unknown_policy", {}).get("order", [])[-1:] != ["opacity"]:
        errors.append("opacity-not-last")
    if contract.get("materialized_gate", {}).get("combined") != {"valid":84,"required_killed":84,"required_score":1.0}:
        errors.append("gate-threshold")
    built = build_fixture(root, contract, focal)
    expectation, positive, r4_contract, r4_checker, r3_contract, r3_checker, r1, r2, r3_suite, fixture_errors = built
    errors.extend(fixture_errors)
    control = classify(contract, copy.deepcopy(expectation), copy.deepcopy(positive), r4_contract, r4_checker, r3_contract, r3_checker)
    opaque = copy.deepcopy(positive)
    opaque["payload_state"] = {"kind":"opaque","reason_code":focal["controls"]["opaque_reason_code"]}
    opaque_control = classify(contract, copy.deepcopy(expectation), opaque, r4_contract, r4_checker, r3_contract, r3_checker)
    if control[0] != "Preserved": errors.append("positive-control:" + control[1])
    if opaque_control[0] != "Unknown": errors.append("opaque-control:" + opaque_control[1])

    r4_focal = load_json(root / focal["base_focal"][0])
    r3_focal = load_json(root / r4_focal["base_focal"][0])
    protected_ids = [row["id"] for row in load_json(root / r3_focal["base_positive_ledger"][0])["mutations"]]
    r1_ids = list(r1.ADDITIONAL_CLASS)
    r2_ids = list(r2.NEW_MUTATIONS)
    r3_ids = list(r3_suite.NEW)
    all_ids = protected_ids + r1_ids + r2_ids + r3_ids
    materializers = dict(r1.MUTATIONS)
    materializers.update(r2.NEW_MUTATIONS)
    materializers.update(r3_suite.NEW)
    if (len(protected_ids),len(r1_ids),len(r2_ids),len(r3_ids),len(set(all_ids)),set(materializers)) != (41,10,16,17,84,set(all_ids)):
        errors.append("frozen-corpus-bijection")
    orders = {"normal":all_ids,"repeat":all_ids,"reverse":list(reversed(all_ids))}
    runs = {}
    for order_name, identifiers in orders.items():
        rows = {}
        for identifier in identifiers:
            exp = copy.deepcopy(expectation)
            led = copy.deepcopy(positive)
            try:
                materializers[identifier](exp, led)
                rows[identifier] = classify(contract, exp, led, r4_contract, r4_checker, r3_contract, r3_checker)
            except Exception as error:
                rows[identifier] = ("MaterializerError", type(error).__name__)
        runs[order_name] = rows
    deterministic = runs["normal"] == runs["repeat"] == runs["reverse"]
    if not deterministic: errors.append("order-nondeterminism")
    groups = {"inherited_67":protected_ids+r1_ids+r2_ids,"r3_boundary_17":r3_ids,"combined":all_ids}
    summaries = {}
    for name, identifiers in groups.items():
        killed = sum(runs["normal"][identifier][0] == "Violated" for identifier in identifiers)
        survivors = [identifier for identifier in identifiers if runs["normal"][identifier][0] != "Violated"]
        summaries[name] = {"valid":len(identifiers),"killed":killed,"survived":len(survivors),"score":killed/len(identifiers),"survivors":survivors}
    if summaries["combined"]["killed"] != 84: errors.append("materialized-survivor")
    verdict = "FOCAL_MATERIALIZED_PASS_NOT_SEALED" if not errors else "BLOCK_SEAL"
    return {"schema":"p1341-contract-r5-check-result-v1","contract_status":contract.get("status"),"controls":{"positive":control[0],"opaque":opaque_control[0]},"orders":["normal","repeat","reverse"],"deterministic":deterministic,"summaries":summaries,"r3_boundary_details":{identifier:{"classification":runs["normal"][identifier][0],"reason_code":runs["normal"][identifier][1]} for identifier in r3_ids},"errors":errors,"verdict":verdict}


def main() -> int:
    if len(sys.argv) != 3:
        print(f"usage: {Path(sys.argv[0]).name} CONTRACT.json FOCAL.json", file=sys.stderr)
        return 2
    result = run(Path(sys.argv[1]), Path(sys.argv[2]))
    print(json.dumps(result,sort_keys=True,separators=(",",":")))
    return 0 if result["verdict"] == "FOCAL_MATERIALIZED_PASS_NOT_SEALED" else 1


if __name__ == "__main__":
    raise SystemExit(main())
