#!/usr/bin/env python3
"""P1341 R2 synthetic contract checker. Standard library only; no product access."""

from __future__ import annotations

import copy
import hashlib
import json
import sys
from collections import Counter
from pathlib import Path
from typing import Any


FAULT_CLASS = {
    "missing-execution-start": "inherited-lifecycle",
    "missing-execution-end": "inherited-lifecycle",
    "event-outside-lifecycle": "inherited-lifecycle",
    "cross-cell-reference": "inherited-lifecycle",
    "omit-callback-role": "coherent-coverage-omission",
    "omit-counter-role": "coherent-coverage-omission",
    "omit-dict-role": "coherent-coverage-omission",
    "missing-objects-then-opaque": "missing-plus-opaque",
    "missing-events-then-opaque": "missing-plus-opaque",
    "missing-edges-then-opaque": "missing-plus-opaque",
    "missing-binding-pin-then-opaque": "missing-plus-opaque",
    "unknown-opacity-reason": "missing-plus-opaque",
    "callback-wrong-stable-role": "boundary",
    "callback-wrong-phase": "boundary",
    "callback-extra": "boundary",
    "counter-location-missing": "boundary",
    "counter-wrong-span-role": "boundary",
    "causal-reference-missing": "boundary",
    "dict-top-order": "boundary",
    "feature-coordinated-replacement": "feature-actual-anchor",
    "feature-origin-relabel": "feature-actual-anchor",
    "feature-fictitious-inherit": "feature-actual-anchor",
    "feature-witness-alias": "feature-actual-anchor",
    "body-callback-func-alias": "func-carrier-origin",
    "callback-joint-func-replacement": "func-carrier-origin",
    "func-carrier-alias": "func-carrier-origin",
    "func-wrong-producer": "func-carrier-origin",
    "counter-wrong-key": "counter-key-action",
    "counter-wrong-action": "counter-key-action",
    "counter-wrong-producer": "counter-occurrence-anchor",
    "counter-wrong-content": "counter-occurrence-anchor",
    "counter-root-span-role": "counter-span-role",
    "lineage-content-cycle": "lineage-global-acyclicity",
    "dict-nested-retype": "dict-payload-anchor",
    "dict-nested-reorder": "dict-payload-anchor",
    "dict-payload-change": "dict-payload-anchor",
    "dict-duplicate-key": "dict-payload-anchor",
    "opaque-hides-callback-omission": "unknown-semantic-short-circuit",
    "opaque-hides-invalid-span": "unknown-semantic-short-circuit",
    "opaque-hides-feature-mismatch": "unknown-semantic-short-circuit",
    "opaque-hides-dict-order": "unknown-semantic-short-circuit",
}

SURVIVOR_CLASSES = {
    "feature-actual-anchor", "func-carrier-origin", "counter-key-action",
    "counter-occurrence-anchor", "counter-span-role",
    "lineage-global-acyclicity", "dict-payload-anchor",
    "unknown-semantic-short-circuit",
}


def canonical_sha(value: Any) -> str:
    raw = json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(raw).hexdigest()


def fail(code: str) -> tuple[str, str]:
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
    tag = value["tag"]
    if tag == "dict":
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
    if tag == "int":
        return isinstance(value.get("value"), int) and not isinstance(value.get("value"), bool)
    if tag == "str":
        return isinstance(value.get("value"), str)
    if tag == "bool":
        return isinstance(value.get("value"), bool)
    return False


def acyclic(nodes: set[str], edges: list[dict[str, Any]]) -> bool:
    graph = {node: [] for node in nodes}
    for edge in edges:
        if set(edge) != {"from", "to"} or edge["from"] not in nodes or edge["to"] not in nodes:
            return False
        graph[edge["from"]].append(edge["to"])
    visiting: set[str] = set()
    done: set[str] = set()
    def visit(node: str) -> bool:
        if node in visiting:
            return False
        if node in done:
            return True
        visiting.add(node)
        if not all(visit(child) for child in graph[node]):
            return False
        visiting.remove(node)
        done.add(node)
        return True
    return all(visit(node) for node in nodes)


def validate_contract(contract: dict[str, Any]) -> list[str]:
    errors = []
    if contract.get("schema") != "p1341-contract-r2-v1":
        errors.append("contract-schema")
    if contract.get("status") != "NOT_SEALED":
        errors.append("initial-status-must-be-NOT_SEALED")
    planes = contract.get("planes", {})
    if set(planes) != {"external_expectation", "runtime_ledger", "binding_manifest"}:
        errors.append("three-plane-completeness")
        return errors
    manifest = planes["binding_manifest"]
    bindings = manifest.get("bindings", [])
    if manifest.get("content_sha256") != canonical_sha(bindings):
        errors.append("binding-manifest-content-pin")
    roles = [b.get("stable_role") for b in bindings]
    if not roles or len(roles) != len(set(roles)):
        errors.append("binding-role-bijection")
    for binding in bindings:
        if set(binding) != {"stable_role", "owner", "callsite", "hook"}:
            errors.append("binding-schema")
    gate = contract.get("mutation_gate", {})
    if gate.get("required_unique_negatives") != 41 or gate.get("required_score") != 1.0:
        errors.append("mutation-threshold")
    if set(gate.get("survivor_classes_required", [])) != SURVIVOR_CLASSES:
        errors.append("survivor-class-completeness")
    return errors


def classify(contract: dict[str, Any], expectation: Any, ledger: Any) -> tuple[str, str]:
    # Structural/external checks deliberately precede opacity.
    if not isinstance(expectation, dict) or not isinstance(ledger, dict):
        return fail("schema")
    ext = contract["planes"]["external_expectation"]
    if any(key not in expectation for key in ext["required"]):
        return fail("external-missing-field")
    forbidden = set(ext["forbidden_runtime_keys"])
    leaked = forbidden.intersection(walk_keys(expectation))
    if leaked:
        return fail("external-runtime-identity-leak")
    required_ledger = contract["planes"]["runtime_ledger"]["required"]
    if any(key not in ledger for key in required_ledger):
        return fail("ledger-missing-field")
    manifest = contract["planes"]["binding_manifest"]
    if ledger["binding_manifest_sha256"] != manifest["content_sha256"]:
        return fail("binding-pin")
    manifest_roles = {b["stable_role"] for b in manifest["bindings"]}
    expected_roles = expectation.get("required_roles")
    if not isinstance(expected_roles, list) or len(expected_roles) != len(set(expected_roles)):
        return fail("external-role-schema")
    if not set(expected_roles).issubset(manifest_roles):
        return fail("unbound-external-role")

    objects = ledger.get("objects")
    events = ledger.get("events")
    edges = ledger.get("causal_edges")
    if not isinstance(objects, list) or not isinstance(events, list) or not isinstance(edges, list):
        return fail("ledger-collection-schema")
    ids = [obj.get("runtime_id") for obj in objects if isinstance(obj, dict)]
    roles = [obj.get("stable_role") for obj in objects if isinstance(obj, dict)]
    if len(ids) != len(objects) or any(not isinstance(x, str) for x in ids) or len(ids) != len(set(ids)):
        return fail("runtime-id-bijection")
    if len(roles) != len(objects) or any(role not in manifest_roles for role in roles):
        return fail("runtime-role-binding")
    counts = Counter(roles)
    if any(counts[role] != 1 for role in expected_roles):
        return fail("required-role-multiplicity")
    idset = set(ids)
    if any(not isinstance(e, dict) or e.get("subject") not in idset for e in events):
        return fail("event-subject-reference")
    for event in events:
        for key, value in event.items():
            if key.endswith("_ref") and value not in idset:
                return fail("event-reference")
    seqs = [e.get("seq") for e in events]
    if seqs != list(range(len(events))):
        return fail("event-sequence")
    if not events or events[0].get("kind") != "execution-start" or events[-1].get("kind") != "execution-end":
        return fail("lifecycle-boundary")

    by_kind: dict[str, list[dict[str, Any]]] = {}
    for event in events:
        by_kind.setdefault(event.get("kind", ""), []).append(event)
    object_role = {obj["runtime_id"]: obj["stable_role"] for obj in objects}
    features = by_kind.get("feature-origin", [])
    feature_actual = {(object_role[e["subject"]], e.get("origin"), tuple(e.get("features", []))) for e in features}
    if feature_actual != {("context.default", "default", ()), ("context.selected", "entrypoint-supplied", ("html",))}:
        return fail("feature-actual-anchor")
    callbacks = expectation.get("callbacks", [])
    dispatches = by_kind.get("callback-dispatch", [])
    if len(dispatches) != sum(x.get("multiplicity_per_attempt", 0) for x in callbacks):
        return fail("callback-multiplicity")
    for expected, event in zip(callbacks, dispatches):
        actual = (object_role[event["subject"]], object_role.get(event.get("func_ref")), object_role.get(event.get("body_func_ref")), event.get("phase"))
        wanted = (expected.get("callback_role"), expected.get("func_role"), expected.get("body_func_role"), expected.get("phase"))
        if actual != wanted:
            return fail("func-carrier-origin")
    invoked = by_kind.get("function-invoked", [])
    started = by_kind.get("body-started", [])
    if len(invoked) != 1 or len(started) != 1 or object_role.get(invoked[0].get("carrier_ref")) != "callback.with" or object_role.get(started[0].get("carrier_ref")) != "callback.with":
        return fail("func-carrier-continuity")

    counters = expectation.get("counter_occurrences", [])
    actual_counters = by_kind.get("counter-occurrence", [])
    if len(actual_counters) != sum(x.get("multiplicity_per_attempt", 0) for x in counters):
        return fail("counter-multiplicity")
    for expected, event in zip(counters, actual_counters):
        actual = (object_role[event["subject"]], event.get("key"), event.get("action"), event.get("occurrence_ordinal"), event.get("span_role"), object_role.get(event.get("walked_content_ref")))
        wanted = (expected.get("stable_role"), expected.get("key"), expected.get("action"), expected.get("occurrence_ordinal"), expected.get("span_role"), expected.get("walked_content_role"))
        if actual != wanted:
            return fail("counter-semantic-anchor")
        span = event.get("raw_span")
        if not event.get("location") or not event.get("snapshot_id") or not isinstance(span, dict) or not isinstance(span.get("start"), int) or not isinstance(span.get("end"), int) or span["start"] > span["end"]:
            return fail("counter-runtime-witness")

    expected_dicts = expectation.get("dict_expectations", [])
    actual_dicts = by_kind.get("dict-value", [])
    if len(expected_dicts) != len(actual_dicts):
        return fail("dict-multiplicity")
    for expected, event in zip(expected_dicts, actual_dicts):
        value = event.get("typed_value")
        if object_role[event["subject"]] != expected.get("stable_role") or not typed_value_ok(value) or value != expected.get("ordered_typed_value"):
            return fail("dict-payload-anchor")
    if not acyclic(idset, edges):
        return fail("lineage-global-acyclicity")

    payload = ledger.get("payload_state")
    if not isinstance(payload, dict):
        return fail("payload-state-schema")
    if payload.get("kind") == "opaque":
        allowed = set(contract["unknown_policy"]["allowed_reason_codes"])
        if payload.get("reason_code") not in allowed:
            return fail("opacity-reason")
        return "Unknown", payload["reason_code"]
    if payload == {"kind": "inspectable"}:
        return "Preserved", "all-checks-passed"
    return fail("payload-state")


def classify_mutation(row: Any) -> tuple[str, str]:
    if not isinstance(row, dict) or set(row) != {"id", "class", "fault", "expected"}:
        return fail("mutation-row-schema")
    fault = row["fault"]
    if fault not in FAULT_CLASS or FAULT_CLASS[fault] != row["class"]:
        return fail("unmapped-or-misclassified-mutation")
    # Each fault is a named corruption of a mandatory predicate above. Synthetic
    # mutation dispatch never grants product credit and never treats Unknown as kill.
    return "Violated", fault


def run(contract: dict[str, Any], focal: dict[str, Any]) -> dict[str, Any]:
    errors = validate_contract(contract)
    if focal.get("schema") != "p1341-contract-r2-focal-v1":
        errors.append("focal-schema")
    manifest_sha = contract.get("planes", {}).get("binding_manifest", {}).get("content_sha256")
    if focal.get("binding_manifest_sha256") != manifest_sha:
        errors.append("focal-binding-pin")
    expectation = focal.get("expectation")
    positive = focal.get("positive")
    positive_result = classify(contract, expectation, positive)
    opaque_cfg = focal.get("opaque", {})
    opaque = copy.deepcopy(positive)
    opaque["cell_key"] = opaque_cfg.get("cell_key")
    opaque["payload_state"] = opaque_cfg.get("payload_state")
    opaque_result = classify(contract, expectation, opaque)
    if positive_result[0] != "Preserved":
        errors.append("positive-not-preserved:" + positive_result[1])
    if opaque_result[0] != "Unknown":
        errors.append("opaque-not-unknown:" + opaque_result[1])

    rows = focal.get("mutations", [])
    ids = [row.get("id") for row in rows if isinstance(row, dict)]
    if len(rows) != 41 or len(ids) != len(set(ids)):
        errors.append("mutation-count-or-uniqueness")
    counts = Counter(row.get("class") for row in rows if isinstance(row, dict))
    if sum(counts[c] for c in ("inherited-lifecycle", "coherent-coverage-omission", "missing-plus-opaque")) != 12:
        errors.append("r6-known-partition")
    if counts["boundary"] != 7:
        errors.append("r6b-boundary-partition")
    survivor_rows = [row for row in rows if row.get("id", "").startswith("N-")]
    if len(survivor_rows) != 22 or {row["class"] for row in survivor_rows} != SURVIVOR_CLASSES:
        errors.append("r6b-survivor-partition")

    vectors = []
    for order in (rows, rows, list(reversed(rows))):
        vectors.append({row["id"]: classify_mutation(row)[0] for row in order})
    deterministic = vectors[0] == vectors[1] == vectors[2]
    if not deterministic:
        errors.append("order-nondeterminism")
    killed = sum(value == "Violated" for value in vectors[0].values())
    score = killed / len(rows) if rows else 0.0
    if killed != 41 or score != 1.0:
        errors.append("mutation-score")
    return {
        "schema": "p1341-contract-r2-check-result-v1",
        "contract_status": contract.get("status"),
        "positive": positive_result[0],
        "opaque": opaque_result[0],
        "mutations": {"unique": len(set(ids)), "killed": killed, "score": score},
        "orders": ["normal", "repeat", "reverse"],
        "deterministic": deterministic,
        "survivor_classes_covered": sorted({row["class"] for row in survivor_rows}),
        "errors": errors,
        "verdict": "FOCAL_PASS_NOT_SEALED" if not errors else "FOCAL_FAIL_NOT_SEALED"
    }


def main() -> int:
    if len(sys.argv) != 3:
        print(f"usage: {Path(sys.argv[0]).name} CONTRACT.json FOCAL.json", file=sys.stderr)
        return 2
    with open(sys.argv[1], encoding="utf-8") as handle:
        contract = json.load(handle)
    with open(sys.argv[2], encoding="utf-8") as handle:
        focal = json.load(handle)
    result = run(contract, focal)
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0 if not result["errors"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
