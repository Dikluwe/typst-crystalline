#!/usr/bin/env python3
"""Final independent adversarial materialization against P1341 contract R4."""

from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
CONTRACT_PATH = DIAG / "p1341-contract-r4.json"
CHECKER_PATH = DIAG / "p1341-contract-r4-checker.py"
FOCAL_PATH = DIAG / "p1341-contract-r4-focal.json"
R1_PATH = DIAG / "p1341-adversary-materialized-mutations-r1.py"
R2_PATH = DIAG / "p1341-adversary-materialized-mutations-r2.py"


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


def event(ledger: dict[str, Any], kind: str) -> dict[str, Any]:
    return next(row for row in ledger["events"] if row.get("kind") == kind)


def obj(ledger: dict[str, Any], role: str) -> dict[str, Any]:
    return next(row for row in ledger["objects"] if row.get("stable_role") == role)


def reindex(ledger: dict[str, Any]) -> None:
    for index, row in enumerate(ledger["events"]):
        row["seq"] = index


Mutation = Callable[[dict[str, Any], dict[str, Any]], None]
NEW: dict[str, Mutation] = {}
META: dict[str, dict[str, str]] = {}


def mutation(identifier: str, attack_class: str, rationale: str):
    def register(function: Mutation) -> Mutation:
        NEW[identifier] = function
        META[identifier] = {"class": attack_class, "rationale": rationale}
        return function
    return register


@mutation("R3-SCHEMA-EXTRA-EXPECTATION", "strict-schema", "unknown top-level expectation section must fail closed")
def schema_extra_expectation(expectation, _ledger):
    expectation["candidate_declared_complete"] = True


@mutation("R3-SCHEMA-EXTRA-LEDGER", "strict-schema", "unknown top-level ledger section must fail closed")
def schema_extra_ledger(_expectation, ledger):
    ledger["candidate_declared_complete"] = True


@mutation("R3-SCHEMA-EXTRA-OBJECT", "strict-schema", "unknown runtime-object field must fail closed")
def schema_extra_object(_expectation, ledger):
    obj(ledger, "execution")["candidate_claim"] = "complete"


@mutation("R3-SCHEMA-EXTRA-EVENT", "strict-schema", "unknown event field must fail closed")
def schema_extra_event(_expectation, ledger):
    event(ledger, "callback-dispatch")["candidate_claim"] = "complete"


@mutation("R3-SCHEMA-MISSING-OBJECT-KIND", "strict-schema", "mandatory object kind is removed")
def schema_missing_object_kind(_expectation, ledger):
    del obj(ledger, "func.callback")["kind"]


@mutation("R3-SCHEMA-MISSING-CAUSAL-EDGES", "strict-schema", "mandatory causal_edges collection is removed")
def schema_missing_causal_edges(_expectation, ledger):
    del ledger["causal_edges"]


@mutation("R3-BOOL-AS-EVENT-SEQ", "bool-vs-int", "JSON boolean must not satisfy an integer event sequence")
def bool_as_event_seq(_expectation, ledger):
    event(ledger, "execution-start")["seq"] = False


@mutation("R3-BOOL-AS-COUNTER-ORDINAL", "bool-vs-int", "False must not satisfy the contract-owned integer zero occurrence ordinal")
def bool_as_counter_ordinal(_expectation, ledger):
    event(ledger, "counter-occurrence")["occurrence_ordinal"] = False


@mutation("R3-BOOL-AS-ROLE-CARDINALITY", "bool-vs-int", "True must not satisfy the contract-owned integer one cardinality")
def bool_as_role_cardinality(expectation, _ledger):
    expectation["role_cardinalities"]["execution"] = True


@mutation("R3-IDENTITY-CROSS-DOMAIN-VALUE-FUNC", "identity-cross-domain", "value_id is replaced by a func-domain identity")
def identity_cross_domain_value_func(_expectation, ledger):
    obj(ledger, "dict.result")["value_id"] = obj(ledger, "func.callback")["func_id"]


@mutation("R3-IDENTITY-CROSS-OWNER-FUNC", "identity-cross-domain", "two Func owners share one valid func-domain identity")
def identity_cross_owner_func(_expectation, ledger):
    obj(ledger, "func.body")["func_id"] = obj(ledger, "func.callback")["func_id"]


@mutation("R3-TEMPORAL-FEATURES-AFTER-DICT", "temporal-adjacent", "feature-origin observations move after dict-value but before execution-end")
def temporal_features_after_dict(_expectation, ledger):
    features = [row for row in ledger["events"] if row["kind"] == "feature-origin"]
    ledger["events"] = [row for row in ledger["events"] if row["kind"] != "feature-origin"]
    insert_at = next(i for i, row in enumerate(ledger["events"]) if row["kind"] == "execution-end")
    ledger["events"][insert_at:insert_at] = features
    reindex(ledger)


@mutation("R3-TEMPORAL-SWAP-COUNTER-DICT", "temporal-adjacent", "adjacent counter-occurrence and dict-value are reversed")
def temporal_swap_counter_dict(_expectation, ledger):
    rows = ledger["events"]
    left = next(i for i, row in enumerate(rows) if row["kind"] == "counter-occurrence")
    right = next(i for i, row in enumerate(rows) if row["kind"] == "dict-value")
    rows[left], rows[right] = rows[right], rows[left]
    reindex(ledger)


@mutation("R3-CAUSAL-DUPLICATE-EDGE", "causality", "a required edge is duplicated")
def causal_duplicate_edge(_expectation, ledger):
    ledger["causal_edges"].append(copy.deepcopy(ledger["causal_edges"][0]))


@mutation("R3-CAUSAL-EXTRA-TRANSITIVE-EDGE", "causality", "an unrequired transitive edge is added")
def causal_extra_transitive_edge(_expectation, ledger):
    ledger["causal_edges"].append({"from": "r-exec", "to": "r-with"})


@mutation("R3-OPAQUE-HIDES-EXTRA-SCHEMA", "unknown-short-circuit", "opaque wraps an unrecognized ledger field")
def opaque_hides_extra_schema(expectation, ledger):
    schema_extra_ledger(expectation, ledger)
    ledger["payload_state"] = {"kind": "opaque", "reason_code": "payload-deliberately-opaque"}


@mutation("R3-OPAQUE-HIDES-BOOL-ORDINAL", "unknown-short-circuit", "opaque wraps False substituted for integer occurrence ordinal zero")
def opaque_hides_bool_ordinal(expectation, ledger):
    bool_as_counter_ordinal(expectation, ledger)
    ledger["payload_state"] = {"kind": "opaque", "reason_code": "payload-deliberately-opaque"}


def safely_classify(checker, contract, expectation, ledger, r3_contract, r3_checker):
    try:
        classification, reason = checker.classify(
            contract, expectation, ledger, r3_contract, r3_checker
        )
        return {"classification": classification, "reason_code": reason}
    except Exception as error:
        return {
            "classification": "CheckerError",
            "reason_code": f"{type(error).__name__}:{error}",
        }


def main() -> int:
    contract = load_json(CONTRACT_PATH)
    focal = load_json(FOCAL_PATH)
    checker = import_module(CHECKER_PATH, "p1341_r4_checker_adversary_r3")
    r1 = import_module(R1_PATH, "p1341_r1_adversary_suite_for_r3")
    r2 = import_module(R2_PATH, "p1341_r2_adversary_suite_for_r3")
    expectation, positive, r3_contract, r3_checker, built_r1, built_r2, fixture_errors = checker.build_fixture(ROOT, contract, focal)
    if set(built_r1.MUTATIONS) != set(r1.MUTATIONS) or set(built_r2.NEW_MUTATIONS) != set(r2.NEW_MUTATIONS):
        raise RuntimeError("independently loaded and R4-loaded suites differ")

    r3_focal = load_json(ROOT / focal["base_focal"][0])
    protected_ids = [row["id"] for row in load_json(ROOT / r3_focal["base_positive_ledger"][0])["mutations"]]
    r1_ids = list(r1.ADDITIONAL_CLASS)
    r2_ids = list(r2.NEW_MUTATIONS)
    r3_ids = list(NEW)
    inherited_ids = protected_ids + r1_ids + r2_ids
    all_ids = inherited_ids + r3_ids
    materializers = dict(r1.MUTATIONS)
    materializers.update(r2.NEW_MUTATIONS)
    materializers.update(NEW)
    if (len(protected_ids), len(r1_ids), len(r2_ids), len(inherited_ids), len(set(all_ids)), set(materializers)) != (41, 10, 16, 67, len(all_ids), set(all_ids)):
        raise RuntimeError("suite cardinality or materializer bijection failure")

    opaque = copy.deepcopy(positive)
    opaque["payload_state"] = {"kind": "opaque", "reason_code": focal["controls"]["opaque_reason_code"]}
    controls = {
        "positive": safely_classify(checker, contract, copy.deepcopy(expectation), copy.deepcopy(positive), r3_contract, r3_checker),
        "opaque": safely_classify(checker, contract, copy.deepcopy(expectation), opaque, r3_contract, r3_checker),
    }
    orders = {"normal": all_ids, "repeat": all_ids, "reverse": list(reversed(all_ids))}
    runs = {}
    for order_name, identifiers in orders.items():
        rows = {}
        for identifier in identifiers:
            mutated_expectation = copy.deepcopy(expectation)
            mutated_ledger = copy.deepcopy(positive)
            try:
                materializers[identifier](mutated_expectation, mutated_ledger)
                rows[identifier] = safely_classify(checker, contract, mutated_expectation, mutated_ledger, r3_contract, r3_checker)
            except Exception as error:
                rows[identifier] = {"classification": "MaterializerError", "reason_code": type(error).__name__}
        runs[order_name] = rows
    deterministic = runs["normal"] == runs["repeat"] == runs["reverse"]

    groups = {
        "protected_41": protected_ids,
        "r1_additional_10": r1_ids,
        "r2_new_16": r2_ids,
        "inherited_67": inherited_ids,
        "r3_boundary": r3_ids,
        "combined": all_ids,
    }
    summaries = {}
    for name, identifiers in groups.items():
        killed = sum(runs["normal"][identifier]["classification"] == "Violated" for identifier in identifiers)
        survivors = [identifier for identifier in identifiers if runs["normal"][identifier]["classification"] != "Violated"]
        summaries[name] = {
            "valid": len(identifiers),
            "killed": killed,
            "survived": len(survivors),
            "score": killed / len(identifiers),
            "survivors": survivors,
        }
    details = {
        identifier: {
            **META[identifier],
            "materializer": NEW[identifier].__name__,
            **runs["normal"][identifier],
            "killed": runs["normal"][identifier]["classification"] == "Violated",
        }
        for identifier in r3_ids
    }
    all_survivors = summaries["combined"]["survivors"]
    result = {
        "schema": "p1341-adversary-materialized-result-r3",
        "scope": "R4 synthetic discrimination only; no candidate/product access",
        "inputs": {
            str(path.relative_to(ROOT)): sha256(path)
            for path in (CONTRACT_PATH, CHECKER_PATH, FOCAL_PATH, R1_PATH, R2_PATH)
        },
        "fixture_errors": fixture_errors,
        "controls": controls,
        "orders": list(orders),
        "deterministic": deterministic,
        "summaries": summaries,
        "r3_boundary_details": details,
        "verdict": (
            "RECOMMEND_SEAL"
            if not fixture_errors
            and deterministic
            and controls["positive"]["classification"] == "Preserved"
            and controls["opaque"]["classification"] == "Unknown"
            and not all_survivors
            else "BLOCK_SEAL"
        ),
        "limitations": [
            "Only the pinned R4 synthetic fragment is evaluated.",
            "Unknown, CheckerError and MaterializerError are survivors for negatives.",
            "Extra-schema attacks interpret R4 strict_schema and fail-closed wording as rejecting unrecognized fields; if extension fields are intended, the contract needs an explicit extension policy.",
            "No candidate/product source or output was read or executed.",
        ],
    }
    print(json.dumps(result, sort_keys=True, indent=2))
    return 0 if result["verdict"] == "RECOMMEND_SEAL" else 1


if __name__ == "__main__":
    raise SystemExit(main())
