#!/usr/bin/env python3
"""P1341 R3 checker: applies the independently materialized mutation suite."""

from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import sys
from collections import Counter, deque
from pathlib import Path
from typing import Any


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def file_sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def canonical_sha(value: Any) -> str:
    raw = json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(raw).hexdigest()


def import_module(path: Path):
    spec = importlib.util.spec_from_file_location("p1341_materialized_suite_r1", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot import materialized suite")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def failure(code: str) -> tuple[str, str]:
    return "Violated", code


def walk_keys(value: Any):
    if isinstance(value, dict):
        for key, child in value.items():
            yield key
            yield from walk_keys(child)
    elif isinstance(value, list):
        for child in value:
            yield from walk_keys(child)


def typed_value_ok(value: Any) -> bool:
    if not isinstance(value, dict) or not isinstance(value.get("tag"), str):
        return False
    if value["tag"] == "dict":
        pairs = value.get("pairs")
        if not isinstance(pairs, list):
            return False
        keys = []
        for pair in pairs:
            if not isinstance(pair, list) or len(pair) != 2 or not isinstance(pair[0], str):
                return False
            keys.append(pair[0])
            if not typed_value_ok(pair[1]):
                return False
        return len(keys) == len(set(keys))
    if value["tag"] == "int":
        return isinstance(value.get("value"), int) and not isinstance(value.get("value"), bool)
    if value["tag"] == "str":
        return isinstance(value.get("value"), str)
    if value["tag"] == "bool":
        return isinstance(value.get("value"), bool)
    return False


def resolve_typed_path(value: Any, path: Any) -> Any:
    if not isinstance(path, list) or any(not isinstance(key, str) for key in path):
        raise ValueError("typed-path-schema")
    current = value
    for key in path:
        if not isinstance(current, dict) or current.get("tag") != "dict":
            raise ValueError("typed-path-through-nondict")
        pairs = current.get("pairs", [])
        matches = [child for candidate, child in pairs if candidate == key]
        if len(matches) != 1:
            raise ValueError("typed-path-key-cardinality")
        current = matches[0]
    return current


def role_graph_ok(roles: set[str], actual: set[tuple[str, str]], required: set[tuple[str, str]]) -> bool:
    if actual != required:
        return False
    graph = {role: [] for role in roles}
    indegree = {role: 0 for role in roles}
    for left, right in actual:
        if left not in roles or right not in roles:
            return False
        graph[left].append(right)
        indegree[right] += 1
    queue = deque([role for role, degree in indegree.items() if degree == 0])
    seen = []
    while queue:
        role = queue.popleft()
        seen.append(role)
        for child in graph[role]:
            indegree[child] -= 1
            if indegree[child] == 0:
                queue.append(child)
    if len(seen) != len(roles):
        return False
    reachable = {"execution"}
    queue = deque(["execution"])
    while queue:
        for child in graph[queue.popleft()]:
            if child not in reachable:
                reachable.add(child)
                queue.append(child)
    return reachable == roles


def validate_contract(contract: Any) -> list[str]:
    errors: list[str] = []
    if not isinstance(contract, dict) or contract.get("schema") != "p1341-contract-r3-v1":
        return ["contract-schema"]
    if contract.get("status") != "NOT_SEALED":
        errors.append("initial-status-not-NOT_SEALED")
    manifest = contract.get("static_binding_manifest", {})
    bindings = manifest.get("bindings", [])
    if manifest.get("content_sha256") != canonical_sha(bindings):
        errors.append("binding-manifest-pin")
    roles = [row.get("stable_role") for row in bindings if isinstance(row, dict)]
    cardinalities = {row.get("stable_role"): row.get("object_cardinality") for row in bindings if isinstance(row, dict)}
    obligations = contract.get("semantic_obligations", {})
    if roles != obligations.get("required_roles") or cardinalities != obligations.get("role_cardinalities"):
        errors.append("manifest-role-cardinality-drift")
    if any(value != 1 for value in cardinalities.values()) or len(roles) != len(set(roles)):
        errors.append("manifest-cardinality-not-exact")
    gate = contract.get("materialized_gate", {})
    if gate.get("protected") != {"valid": 41, "required_killed": 41} or gate.get("additional") != {"valid": 10, "required_killed": 10} or gate.get("required_score") != 1.0:
        errors.append("gate-threshold")
    order = contract.get("unknown_policy", {}).get("mandatory_order", [])
    if not order or order[-1] != "opacity":
        errors.append("unknown-not-last")
    return errors


def build_fixture(root: Path, contract: dict[str, Any], focal: dict[str, Any]) -> tuple[dict[str, Any], dict[str, Any], Any, list[str]]:
    errors: list[str] = []
    expectation_path = root / focal["base_expectations"][0]
    ledger_path = root / focal["base_positive_ledger"][0]
    suite_path = root / focal["materialized_suite"][0]
    for path, expected in ((expectation_path, focal["base_expectations"][1]), (ledger_path, focal["base_positive_ledger"][1]), (suite_path, focal["materialized_suite"][1])):
        if file_sha(path) != expected:
            errors.append("protected-input-hash:" + str(path.relative_to(root)))
    authored = load_json(expectation_path)
    base = load_json(ledger_path)
    expectation = copy.deepcopy(authored["external_cases"][0]["expectation"])
    obligations = contract["semantic_obligations"]
    for section in focal["contract_owned_expectation_sections"]:
        expectation[section] = copy.deepcopy(obligations[section])
    ledger = copy.deepcopy(base["positive"])
    ledger["binding_manifest_sha256"] = focal["ledger_overlay"]["binding_manifest_sha256"]
    ledger["causal_edges"] = copy.deepcopy(focal["ledger_overlay"]["causal_edges"])
    overlay = focal["ledger_overlay"]["events_by_kind"]
    feature_index = 0
    for event in ledger["events"]:
        patch = overlay.get(event.get("kind"))
        if patch:
            event.update(copy.deepcopy(patch))
        if event.get("kind") == "feature-origin":
            event["witness_ordinal"] = feature_index
            feature_index += 1
    suite = import_module(suite_path)
    return expectation, ledger, suite, errors


def classify(contract: dict[str, Any], expectation: Any, ledger: Any) -> tuple[str, str]:
    # Unknown is deliberately unreachable until the final block.
    if not isinstance(expectation, dict) or not isinstance(ledger, dict):
        return failure("schema")
    external = contract["external_expectation"]
    if set(external["required_exact_sections"]) - set(expectation):
        return failure("external-schema-completeness")
    if set(external["forbidden_runtime_keys"]).intersection(walk_keys(expectation)):
        return failure("external-runtime-leak")
    obligations = contract["semantic_obligations"]
    for section in ("source_coordinates", "required_roles", "role_cardinalities", "feature_witnesses", "callbacks", "counter_occurrences", "dict_expectations", "required_causal_role_edges"):
        if expectation.get(section) != obligations[section]:
            return failure("contract-owned-external-completeness:" + section)

    required_ledger = contract["runtime_ledger"]["required"]
    if any(field not in ledger for field in required_ledger):
        return failure("ledger-schema")
    manifest = contract["static_binding_manifest"]
    if ledger.get("binding_manifest_sha256") != manifest["content_sha256"]:
        return failure("binding-manifest-pin")
    objects = ledger.get("objects")
    events = ledger.get("events")
    edges = ledger.get("causal_edges")
    if not isinstance(objects, list) or not isinstance(events, list) or not isinstance(edges, list):
        return failure("ledger-collections")
    runtime_ids = [row.get("runtime_id") for row in objects if isinstance(row, dict)]
    object_roles = [row.get("stable_role") for row in objects if isinstance(row, dict)]
    if len(runtime_ids) != len(objects) or len(runtime_ids) != len(set(runtime_ids)) or any(not isinstance(item, str) for item in runtime_ids):
        return failure("runtime-id-bijection")
    expected_cardinality = obligations["role_cardinalities"]
    if Counter(object_roles) != Counter(expected_cardinality):
        return failure("exact-role-cardinality")
    role_of = dict(zip(runtime_ids, object_roles))
    idset = set(runtime_ids)
    if any(not isinstance(event, dict) or event.get("subject") not in idset for event in events):
        return failure("event-subject-reference")
    for event in events:
        if any(key.endswith("_ref") and value not in idset for key, value in event.items()):
            return failure("event-runtime-reference")
    if [event.get("seq") for event in events] != list(range(len(events))):
        return failure("event-sequence")
    kinds = Counter(event.get("kind") for event in events)
    if kinds != Counter(manifest["event_cardinalities"]):
        return failure("exact-event-cardinality")
    if events[0].get("kind") != "execution-start" or events[-1].get("kind") != "execution-end":
        return failure("lifecycle-boundary")
    by_kind: dict[str, list[dict[str, Any]]] = {}
    for event in events:
        by_kind.setdefault(event["kind"], []).append(event)

    feature_actual = sorted(
        (role_of[event["subject"]], event.get("origin"), event.get("features"), event.get("witness_ordinal"))
        for event in by_kind["feature-origin"]
    )
    feature_expected = sorted(
        (row["stable_role"], row["origin"], row["features"], index)
        for index, row in enumerate(obligations["feature_witnesses"])
    )
    if feature_actual != feature_expected or len({event["subject"] for event in by_kind["feature-origin"]}) != 2:
        return failure("feature-unique-witness-origin")

    callback = obligations["callbacks"][0]
    dispatch = by_kind["callback-dispatch"][0]
    actual_dispatch = (role_of[dispatch["subject"]], role_of.get(dispatch.get("func_ref")), role_of.get(dispatch.get("body_func_ref")), dispatch.get("phase"))
    expected_dispatch = (callback["callback_role"], callback["func_role"], callback["body_func_role"], callback["phase"])
    if actual_dispatch != expected_dispatch:
        return failure("callback-dispatch-binding")
    invoked = by_kind["function-invoked"][0]
    invoked_actual = (role_of[invoked["subject"]], role_of.get(invoked.get("carrier_ref")), invoked.get("producer_role"), invoked.get("origin"))
    invoked_expected = (callback["func_role"], callback["callback_role"], callback["func_producer_role"], callback["func_origin"])
    if invoked_actual != invoked_expected:
        return failure("func-producer-origin")
    body = by_kind["body-started"][0]
    body_actual = (role_of[body["subject"]], role_of.get(body.get("carrier_ref")), body.get("producer_role"), body.get("origin"))
    body_expected = (callback["body_func_role"], callback["callback_role"], callback["body_producer_role"], callback["body_origin"])
    if body_actual != body_expected:
        return failure("body-producer-origin")

    counter = obligations["counter_occurrences"][0]
    occurrence = by_kind["counter-occurrence"][0]
    counter_actual = (role_of[occurrence["subject"]], occurrence.get("key"), occurrence.get("action"), occurrence.get("occurrence_ordinal"), occurrence.get("span_role"), role_of.get(occurrence.get("walked_content_ref")))
    counter_expected = (counter["stable_role"], counter["key"], counter["action"], counter["occurrence_ordinal"], counter["span_role"], counter["walked_content_role"])
    if counter_actual != counter_expected:
        return failure("counter-semantic-anchor")
    span = occurrence.get("raw_span")
    coordinate = counter["source_coordinate"]
    if not occurrence.get("location") or not occurrence.get("snapshot_id") or not isinstance(span, dict) or not span.get("source"):
        return failure("counter-runtime-witness")
    if span.get("start") != coordinate["source_byte_offset"] or span.get("end") - span.get("start", 0) != coordinate["byte_length"] or span.get("lexical_role") != coordinate["lexical_role"]:
        return failure("counter-source-coordinate-role")

    expected_dict = obligations["dict_expectations"][0]
    dict_event = by_kind["dict-value"][0]
    value = dict_event.get("typed_value")
    path = dict_event.get("typed_path")
    if role_of[dict_event["subject"]] != expected_dict["stable_role"] or not typed_value_ok(value) or path != expected_dict["typed_path"]:
        return failure("dict-schema-role-path")
    try:
        selected = resolve_typed_path(value, path)
    except (ValueError, TypeError):
        return failure("dict-typed-path-resolution")
    if selected != expected_dict["ordered_typed_value"]:
        return failure("dict-typed-path-payload")

    projected_edges = set()
    for edge in edges:
        if not isinstance(edge, dict) or set(edge) != {"from", "to"} or edge["from"] not in idset or edge["to"] not in idset:
            return failure("causal-edge-reference")
        projected_edges.add((role_of[edge["from"]], role_of[edge["to"]]))
    required_edges = {tuple(edge) for edge in obligations["required_causal_role_edges"]}
    if len(projected_edges) != len(edges) or not role_graph_ok(set(obligations["required_roles"]), projected_edges, required_edges):
        return failure("directed-connected-causal-topology")

    payload = ledger.get("payload_state")
    if not isinstance(payload, dict):
        return failure("payload-state-schema")
    if payload.get("kind") == "opaque":
        if payload.get("reason_code") not in contract["unknown_policy"]["allowed_reason_codes"]:
            return failure("opacity-reason")
        return "Unknown", payload["reason_code"]
    if payload == {"kind": "inspectable"}:
        return "Preserved", "all-r3-predicates-passed"
    return failure("payload-state")


def run(contract_path: Path, focal_path: Path) -> dict[str, Any]:
    root = contract_path.resolve().parents[2]
    contract = load_json(contract_path)
    focal = load_json(focal_path)
    errors = validate_contract(contract)
    if focal.get("schema") != "p1341-contract-r3-focal-v1":
        errors.append("focal-schema")
    expectation, positive, suite, fixture_errors = build_fixture(root, contract, focal)
    errors.extend(fixture_errors)
    control = classify(contract, copy.deepcopy(expectation), copy.deepcopy(positive))
    opaque = copy.deepcopy(positive)
    opaque["payload_state"] = {"kind":"opaque", "reason_code":focal["controls"]["opaque_reason_code"]}
    opaque_control = classify(contract, copy.deepcopy(expectation), opaque)
    if control[0] != "Preserved":
        errors.append("positive-control:" + control[1])
    if opaque_control[0] != "Unknown":
        errors.append("opaque-control:" + opaque_control[1])

    protected = load_json(root / focal["base_positive_ledger"][0])["mutations"]
    protected_ids = [row["id"] for row in protected]
    additional_ids = list(suite.ADDITIONAL_CLASS)
    all_ids = protected_ids + additional_ids
    if len(protected_ids) != 41 or len(set(protected_ids)) != 41 or len(additional_ids) != 10 or len(set(additional_ids)) != 10:
        errors.append("suite-cardinality")
    if set(suite.MUTATIONS) != set(all_ids):
        errors.append("suite-materializer-bijection")

    runs: dict[str, dict[str, tuple[str, str]]] = {}
    orders = {"normal":all_ids, "repeat":all_ids, "reverse":list(reversed(all_ids))}
    for order_name, identifiers in orders.items():
        rows = {}
        for identifier in identifiers:
            mutated_expectation = copy.deepcopy(expectation)
            mutated_ledger = copy.deepcopy(positive)
            suite.MUTATIONS[identifier](mutated_expectation, mutated_ledger)
            rows[identifier] = classify(contract, mutated_expectation, mutated_ledger)
        runs[order_name] = rows
    deterministic = runs["normal"] == runs["repeat"] == runs["reverse"]
    if not deterministic:
        errors.append("order-nondeterminism")
    protected_killed = sum(runs["normal"][identifier][0] == "Violated" for identifier in protected_ids)
    additional_killed = sum(runs["normal"][identifier][0] == "Violated" for identifier in additional_ids)
    survivors = [identifier for identifier in all_ids if runs["normal"][identifier][0] != "Violated"]
    if protected_killed != 41 or additional_killed != 10 or survivors:
        errors.append("materialized-survivor")
    verdict = "FOCAL_MATERIALIZED_PASS_NOT_SEALED" if not errors else "BLOCK_SEAL"
    return {
        "schema":"p1341-contract-r3-check-result-v1",
        "contract_status":contract.get("status"),
        "controls":{"positive":control[0],"opaque":opaque_control[0]},
        "orders":["normal","repeat","reverse"],
        "deterministic":deterministic,
        "protected_41":{"valid":41,"killed":protected_killed,"score":protected_killed/41},
        "additional_10":{"valid":10,"killed":additional_killed,"score":additional_killed/10},
        "combined":{"valid":51,"killed":protected_killed+additional_killed,"score":(protected_killed+additional_killed)/51},
        "survivors":survivors,
        "errors":errors,
        "verdict":verdict,
        "details":{identifier:{"classification":runs["normal"][identifier][0],"reason_code":runs["normal"][identifier][1]} for identifier in all_ids}
    }


def main() -> int:
    if len(sys.argv) != 3:
        print(f"usage: {Path(sys.argv[0]).name} CONTRACT.json FOCAL.json", file=sys.stderr)
        return 2
    result = run(Path(sys.argv[1]), Path(sys.argv[2]))
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0 if result["verdict"] == "FOCAL_MATERIALIZED_PASS_NOT_SEALED" else 1


if __name__ == "__main__":
    raise SystemExit(main())
