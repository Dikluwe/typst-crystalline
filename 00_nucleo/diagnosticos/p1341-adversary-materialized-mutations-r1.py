#!/usr/bin/env python3
"""Materialize P1341 mutations against the protected positive expectation/ledger.

Adversarial-only harness. It imports the protected R2 classifier but deliberately
does not call classify_mutation: every case below changes a fresh deep copy of
the externally authored expectation and/or the positive runtime ledger.
"""

from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import sys
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
CONTRACT_PATH = DIAG / "p1341-contract-r2.json"
CHECKER_PATH = DIAG / "p1341-contract-r2-checker.py"
FOCAL_PATH = DIAG / "p1341-contract-r2-focal.json"
EXPECTATIONS_PATH = DIAG / "p1341-oracle-expectations-r1.json"


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_checker():
    spec = importlib.util.spec_from_file_location("p1341_contract_r2_checker", CHECKER_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load protected checker")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def event(ledger: dict[str, Any], kind: str, occurrence: int = 0) -> dict[str, Any]:
    matches = [row for row in ledger["events"] if row.get("kind") == kind]
    return matches[occurrence]


def reindex(ledger: dict[str, Any]) -> None:
    for index, row in enumerate(ledger["events"]):
        row["seq"] = index


def remove_roles(
    expectation: dict[str, Any],
    ledger: dict[str, Any],
    roles: set[str],
) -> None:
    ids = {
        row["runtime_id"]
        for row in ledger["objects"]
        if row.get("stable_role") in roles
    }
    expectation["required_roles"] = [
        role for role in expectation["required_roles"] if role not in roles
    ]
    ledger["objects"] = [row for row in ledger["objects"] if row["runtime_id"] not in ids]
    ledger["events"] = [
        row
        for row in ledger["events"]
        if row.get("subject") not in ids
        and not any(key.endswith("_ref") and value in ids for key, value in row.items())
    ]
    ledger["causal_edges"] = [
        row
        for row in ledger["causal_edges"]
        if row["from"] not in ids and row["to"] not in ids
    ]
    reindex(ledger)


Mutation = Callable[[dict[str, Any], dict[str, Any]], None]
MUTATIONS: dict[str, Mutation] = {}


def mutation(identifier: str):
    def register(function: Mutation) -> Mutation:
        MUTATIONS[identifier] = function
        return function
    return register


@mutation("R6-INHERIT-01")
def missing_execution_start(_expectation, ledger):
    ledger["events"] = [row for row in ledger["events"] if row["kind"] != "execution-start"]
    reindex(ledger)


@mutation("R6-INHERIT-02")
def missing_execution_end(_expectation, ledger):
    ledger["events"] = [row for row in ledger["events"] if row["kind"] != "execution-end"]
    reindex(ledger)


@mutation("R6-INHERIT-03")
def event_outside_lifecycle(_expectation, ledger):
    start = ledger["events"].pop(0)
    ledger["events"].insert(1, start)
    reindex(ledger)


@mutation("R6-INHERIT-04")
def cross_cell_reference(_expectation, ledger):
    event(ledger, "callback-dispatch")["func_ref"] = "foreign-cell:r-callback"


@mutation("R6-OMISSION-01")
def omit_callback_role(expectation, ledger):
    expectation["callbacks"] = []
    remove_roles(expectation, ledger, {"callback.with", "func.callback", "func.body"})
    ledger["causal_edges"].append({"from": "r-exec", "to": "r-counter"})


@mutation("R6-OMISSION-02")
def omit_counter_role(expectation, ledger):
    expectation["counter_occurrences"] = []
    remove_roles(expectation, ledger, {"counter.step.current"})
    ledger["causal_edges"].append({"from": "r-body", "to": "r-content"})


@mutation("R6-OMISSION-03")
def omit_dict_role(expectation, ledger):
    expectation["dict_expectations"] = []
    remove_roles(expectation, ledger, {"dict.result"})


@mutation("R6-MISSING-OPAQUE-01")
def missing_objects_then_opaque(_expectation, ledger):
    del ledger["objects"]
    ledger["payload_state"] = {"kind": "opaque", "reason_code": "payload-deliberately-opaque"}


@mutation("R6-MISSING-OPAQUE-02")
def missing_events_then_opaque(_expectation, ledger):
    del ledger["events"]
    ledger["payload_state"] = {"kind": "opaque", "reason_code": "payload-deliberately-opaque"}


@mutation("R6-MISSING-OPAQUE-03")
def missing_edges_then_opaque(_expectation, ledger):
    del ledger["causal_edges"]
    ledger["payload_state"] = {"kind": "opaque", "reason_code": "payload-deliberately-opaque"}


@mutation("R6-MISSING-OPAQUE-04")
def missing_binding_pin_then_opaque(_expectation, ledger):
    del ledger["binding_manifest_sha256"]
    ledger["payload_state"] = {"kind": "opaque", "reason_code": "payload-deliberately-opaque"}


@mutation("R6-MISSING-OPAQUE-05")
def unknown_opacity_reason(_expectation, ledger):
    ledger["payload_state"] = {"kind": "opaque", "reason_code": "not-allowlisted"}


@mutation("B-WITH-CALLSITE")
def callback_wrong_stable_role(_expectation, ledger):
    event(ledger, "callback-dispatch")["subject"] = "r-default"


@mutation("B-CALLBACK-PHASE")
def callback_wrong_phase(_expectation, ledger):
    event(ledger, "callback-dispatch")["phase"] = "layout"


@mutation("B-EXTRA-CALLBACK")
def callback_extra(_expectation, ledger):
    duplicate = copy.deepcopy(event(ledger, "callback-dispatch"))
    ledger["events"].insert(-1, duplicate)
    reindex(ledger)


@mutation("B-COUNTER-WRONG-LOCATION")
def counter_location_missing(_expectation, ledger):
    del event(ledger, "counter-occurrence")["location"]


@mutation("B-COUNTER-READ-SPAN")
def counter_wrong_span_role(_expectation, ledger):
    event(ledger, "counter-occurrence")["span_role"] = "counter-read"


@mutation("B-LINEAGE-BREAK")
def causal_reference_missing(_expectation, ledger):
    ledger["causal_edges"][2]["to"] = "missing-runtime-object"


@mutation("B-DICT-TOP-ORDER")
def dict_top_order(_expectation, ledger):
    pairs = event(ledger, "dict-value")["typed_value"]["pairs"]
    pairs[:] = list(reversed(pairs))


@mutation("N-FEATURE-COORDINATED")
def feature_coordinated_replacement(_expectation, ledger):
    rows = [row for row in ledger["events"] if row["kind"] == "feature-origin"]
    rows[0]["origin"], rows[1]["origin"] = rows[1]["origin"], rows[0]["origin"]
    rows[0]["features"], rows[1]["features"] = rows[1]["features"], rows[0]["features"]


@mutation("N-FEATURE-ORIGIN-RELABEL")
def feature_origin_relabel(_expectation, ledger):
    event(ledger, "feature-origin", 1)["origin"] = "inherited"


@mutation("N-FEATURE-FICTITIOUS-INHERIT")
def feature_fictitious_inherit(_expectation, ledger):
    event(ledger, "feature-origin", 0)["features"] = ["html"]


@mutation("N-FEATURE-WITNESS-ALIAS")
def feature_witness_alias(_expectation, ledger):
    event(ledger, "feature-origin", 0)["subject"] = "r-selected"


@mutation("N-BODY-CALLBACK-FUNC")
def body_callback_func_alias(_expectation, ledger):
    event(ledger, "callback-dispatch")["body_func_ref"] = "r-callback"


@mutation("N-CALLBACK-JOINT-FUNC")
def callback_joint_func_replacement(_expectation, ledger):
    dispatch = event(ledger, "callback-dispatch")
    dispatch["func_ref"] = "r-body"
    dispatch["body_func_ref"] = "r-callback"


@mutation("N-FUNC-CARRIER-ALIAS")
def func_carrier_alias(_expectation, ledger):
    event(ledger, "function-invoked")["carrier_ref"] = "r-body"


@mutation("N-FUNC-WRONG-PRODUCER")
def func_wrong_producer(_expectation, ledger):
    event(ledger, "function-invoked")["subject"] = "r-body"


@mutation("N-COUNTER-WRONG-KEY")
def counter_wrong_key(_expectation, ledger):
    event(ledger, "counter-occurrence")["key"] = "figure"


@mutation("N-COUNTER-WRONG-ACTION")
def counter_wrong_action(_expectation, ledger):
    event(ledger, "counter-occurrence")["action"] = "update"


@mutation("N-COUNTER-WRONG-PRODUCER")
def counter_wrong_producer(_expectation, ledger):
    event(ledger, "counter-occurrence")["subject"] = "r-content"


@mutation("N-COUNTER-WRONG-CONTENT")
def counter_wrong_content(_expectation, ledger):
    event(ledger, "counter-occurrence")["walked_content_ref"] = "r-dict"


@mutation("N-COUNTER-ROOT-ROLE")
def counter_root_span_role(_expectation, ledger):
    event(ledger, "counter-occurrence")["span_role"] = "document-root"


@mutation("N-LINEAGE-CYCLE")
def lineage_content_cycle(_expectation, ledger):
    ledger["causal_edges"].append({"from": "r-dict", "to": "r-exec"})


@mutation("N-DICT-NESTED-RETYPE")
def dict_nested_retype(_expectation, ledger):
    nested = event(ledger, "dict-value")["typed_value"]["pairs"][0][1]["pairs"]
    nested[0][1] = {"tag": "str", "value": "1"}


@mutation("N-DICT-NESTED-REORDER")
def dict_nested_reorder(_expectation, ledger):
    nested = event(ledger, "dict-value")["typed_value"]["pairs"][0][1]["pairs"]
    nested[:] = list(reversed(nested))


@mutation("N-DICT-PAYLOAD")
def dict_payload_change(_expectation, ledger):
    nested = event(ledger, "dict-value")["typed_value"]["pairs"][0][1]["pairs"]
    nested[0][1]["value"] = 2


@mutation("N-DICT-DUPLICATE-NESTED-KEY")
def dict_duplicate_key(_expectation, ledger):
    nested = event(ledger, "dict-value")["typed_value"]["pairs"][0][1]["pairs"]
    nested[1][0] = nested[0][0]


def make_opaque(ledger: dict[str, Any]) -> None:
    ledger["payload_state"] = {"kind": "opaque", "reason_code": "payload-deliberately-opaque"}


@mutation("N-OPAQUE-HIDES-COVERAGE")
def opaque_hides_callback_omission(expectation, ledger):
    omit_callback_role(expectation, ledger)
    make_opaque(ledger)


@mutation("N-OPAQUE-HIDES-SPAN-RANGE")
def opaque_hides_invalid_span(_expectation, ledger):
    event(ledger, "counter-occurrence")["raw_span"] = {"source": "dynamic-source", "start": 20, "end": 10}
    make_opaque(ledger)


@mutation("N-OPAQUE-HIDES-FEATURE")
def opaque_hides_feature_mismatch(expectation, ledger):
    feature_origin_relabel(expectation, ledger)
    make_opaque(ledger)


@mutation("N-OPAQUE-HIDES-DICT-ORDER")
def opaque_hides_dict_order(expectation, ledger):
    dict_top_order(expectation, ledger)
    make_opaque(ledger)


# Additional coherent attacks against the eight formerly surviving classes.
@mutation("X-FEATURE-DUPLICATE-WITNESS")
def feature_duplicate_witness(_expectation, ledger):
    ledger["events"].insert(-1, copy.deepcopy(event(ledger, "feature-origin", 1)))
    reindex(ledger)


@mutation("X-FUNC-BODY-PRODUCER-ALIAS")
def func_body_producer_alias(_expectation, ledger):
    event(ledger, "body-started")["subject"] = "r-callback"


@mutation("X-COUNTER-ACTION-CASE-SUBSTITUTION")
def counter_action_case_substitution(_expectation, ledger):
    event(ledger, "counter-occurrence")["action"] = "Step"


@mutation("X-COUNTER-OCCURRENCE-ORDINAL")
def counter_occurrence_ordinal(_expectation, ledger):
    event(ledger, "counter-occurrence")["occurrence_ordinal"] = 1


@mutation("X-COUNTER-SPAN-COORDINATE-SUBSTITUTION")
def counter_span_coordinate_substitution(_expectation, ledger):
    event(ledger, "counter-occurrence")["raw_span"] = {
        "source": "unrelated-source",
        "start": 1000,
        "end": 1005,
    }


@mutation("X-LINEAGE-DISCONNECTED-DAG")
def lineage_disconnected_dag(_expectation, ledger):
    ledger["causal_edges"] = [
        row for row in ledger["causal_edges"]
        if row != {"from": "r-content", "to": "r-dict"}
    ]


@mutation("X-LINEAGE-REVERSED-ACYCLIC-EDGE")
def lineage_reversed_acyclic_edge(_expectation, ledger):
    edge = next(row for row in ledger["causal_edges"] if row == {"from": "r-body", "to": "r-counter"})
    edge["from"], edge["to"] = edge["to"], edge["from"]


@mutation("X-DICT-CONTRADICTORY-TYPED-PATH")
def dict_contradictory_typed_path(_expectation, ledger):
    event(ledger, "dict-value")["typed_path"] = ["outer", "missing"]


@mutation("X-OPAQUE-HIDES-FUNC-PRODUCER")
def opaque_hides_func_producer(expectation, ledger):
    func_wrong_producer(expectation, ledger)
    make_opaque(ledger)


@mutation("X-OPAQUE-HIDES-DISCONNECTED-LINEAGE")
def opaque_hides_disconnected_lineage(expectation, ledger):
    lineage_disconnected_dag(expectation, ledger)
    make_opaque(ledger)


ADDITIONAL_CLASS = {
    "X-FEATURE-DUPLICATE-WITNESS": "feature-actual-anchor",
    "X-FUNC-BODY-PRODUCER-ALIAS": "func-carrier-origin",
    "X-COUNTER-ACTION-CASE-SUBSTITUTION": "counter-key-action",
    "X-COUNTER-OCCURRENCE-ORDINAL": "counter-occurrence-anchor",
    "X-COUNTER-SPAN-COORDINATE-SUBSTITUTION": "counter-span-role",
    "X-LINEAGE-DISCONNECTED-DAG": "lineage-global-acyclicity",
    "X-LINEAGE-REVERSED-ACYCLIC-EDGE": "lineage-global-acyclicity",
    "X-DICT-CONTRADICTORY-TYPED-PATH": "dict-payload-anchor",
    "X-OPAQUE-HIDES-FUNC-PRODUCER": "unknown-semantic-short-circuit",
    "X-OPAQUE-HIDES-DISCONNECTED-LINEAGE": "unknown-semantic-short-circuit",
}


def apply_and_classify(checker, contract, expectation, positive, identifier):
    mutated_expectation = copy.deepcopy(expectation)
    mutated_ledger = copy.deepcopy(positive)
    MUTATIONS[identifier](mutated_expectation, mutated_ledger)
    classification, reason = checker.classify(contract, mutated_expectation, mutated_ledger)
    return {"classification": classification, "reason_code": reason}


def main() -> int:
    contract = load_json(CONTRACT_PATH)
    focal = load_json(FOCAL_PATH)
    authored = load_json(EXPECTATIONS_PATH)
    checker = load_checker()
    expectation = authored["external_cases"][0]["expectation"]
    positive = focal["positive"]
    protected_rows = focal["mutations"]
    protected_ids = [row["id"] for row in protected_rows]
    expected_ids = set(protected_ids) | set(ADDITIONAL_CLASS)
    if set(MUTATIONS) != expected_ids:
        raise RuntimeError({
            "missing_materializers": sorted(expected_ids - set(MUTATIONS)),
            "unexpected_materializers": sorted(set(MUTATIONS) - expected_ids),
        })

    control = checker.classify(contract, copy.deepcopy(expectation), copy.deepcopy(positive))
    opaque = copy.deepcopy(positive)
    make_opaque(opaque)
    opaque_control = checker.classify(contract, copy.deepcopy(expectation), opaque)

    orders = {
        "normal": protected_ids + list(ADDITIONAL_CLASS),
        "repeat": protected_ids + list(ADDITIONAL_CLASS),
        "reverse": list(reversed(protected_ids + list(ADDITIONAL_CLASS))),
    }
    runs = {
        name: {
            identifier: apply_and_classify(
                checker, contract, expectation, positive, identifier
            )
            for identifier in identifiers
        }
        for name, identifiers in orders.items()
    }
    deterministic = runs["normal"] == runs["repeat"] == runs["reverse"]

    row_class = {row["id"]: row["class"] for row in protected_rows}
    row_class.update(ADDITIONAL_CLASS)
    details = []
    for identifier in orders["normal"]:
        result = runs["normal"][identifier]
        details.append({
            "id": identifier,
            "origin": "protected-41" if identifier in protected_ids else "additional-adversarial",
            "class": row_class[identifier],
            "materializer": MUTATIONS[identifier].__name__,
            **result,
            "killed": result["classification"] == "Violated",
        })

    protected_results = [row for row in details if row["origin"] == "protected-41"]
    additional_results = [row for row in details if row["origin"] == "additional-adversarial"]
    all_killed = sum(row["killed"] for row in details)
    protected_killed = sum(row["killed"] for row in protected_results)
    additional_killed = sum(row["killed"] for row in additional_results)
    survivors = [row["id"] for row in details if not row["killed"]]
    result = {
        "schema": "p1341-adversary-materialized-result-r1",
        "scope": "synthetic contract discrimination only; no product or candidate access",
        "inputs": {
            str(path.relative_to(ROOT)): sha256(path)
            for path in (CONTRACT_PATH, CHECKER_PATH, FOCAL_PATH, EXPECTATIONS_PATH)
        },
        "controls": {
            "positive": {"classification": control[0], "reason_code": control[1]},
            "opaque": {"classification": opaque_control[0], "reason_code": opaque_control[1]},
        },
        "execution": {
            "orders": list(orders),
            "deterministic": deterministic,
            "protected_41": {
                "valid": len(protected_results),
                "killed": protected_killed,
                "survived": len(protected_results) - protected_killed,
                "score": protected_killed / len(protected_results),
            },
            "additional": {
                "valid": len(additional_results),
                "killed": additional_killed,
                "survived": len(additional_results) - additional_killed,
                "score": additional_killed / len(additional_results),
            },
            "combined": {
                "valid": len(details),
                "killed": all_killed,
                "survived": len(details) - all_killed,
                "score": all_killed / len(details),
            },
            "survivors": survivors,
        },
        "details": details,
        "verdict": "RECOMMEND_SEAL" if deterministic and not survivors and control[0] == "Preserved" and opaque_control[0] == "Unknown" else "BLOCK_SEAL",
        "limitations": [
            "The harness evaluates only the protected synthetic expectation and positive ledger fragment.",
            "It does not read candidate/product code, candidate outputs, or P1340 implementation/outputs.",
            "A materialized mutation is killed only by classify returning Violated; Unknown is never counted as a kill.",
        ],
    }
    print(json.dumps(result, sort_keys=True, indent=2))
    return 0 if result["verdict"] == "RECOMMEND_SEAL" else 1


if __name__ == "__main__":
    raise SystemExit(main())
