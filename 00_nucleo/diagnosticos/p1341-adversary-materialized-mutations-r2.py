#!/usr/bin/env python3
"""Independent materialized attack of the P1341 R3 synthetic contract.

The protected 51 transformations are imported from the adversary R1 suite and
applied to the R3 fixture. New R2 transformations are defined here. No nominal
mutation dispatcher is accepted as evidence, and Unknown/error never counts as
a kill.
"""

from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
CONTRACT_PATH = DIAG / "p1341-contract-r3.json"
CHECKER_PATH = DIAG / "p1341-contract-r3-checker.py"
FOCAL_PATH = DIAG / "p1341-contract-r3-focal.json"
R1_SUITE_PATH = DIAG / "p1341-adversary-materialized-mutations-r1.py"


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
NEW_MUTATIONS: dict[str, Mutation] = {}
NEW_CLASS: dict[str, str] = {}
NEW_RATIONALE: dict[str, str] = {}


def mutation(identifier: str, attack_class: str, rationale: str):
    def register(function: Mutation) -> Mutation:
        NEW_MUTATIONS[identifier] = function
        NEW_CLASS[identifier] = attack_class
        NEW_RATIONALE[identifier] = rationale
        return function
    return register


@mutation(
    "R2-LIFECYCLE-START-WRONG-SUBJECT",
    "lifecycle-role-binding",
    "execution-start remains first but is emitted by context.default, not execution",
)
def lifecycle_start_wrong_subject(_expectation, ledger):
    event(ledger, "execution-start")["subject"] = obj(ledger, "context.default")["runtime_id"]


@mutation(
    "R2-LIFECYCLE-END-WRONG-SUBJECT",
    "lifecycle-role-binding",
    "execution-end remains last but is emitted by dict.result, not execution",
)
def lifecycle_end_wrong_subject(_expectation, ledger):
    event(ledger, "execution-end")["subject"] = obj(ledger, "dict.result")["runtime_id"]


@mutation(
    "R2-TEMPORAL-INVOKE-BEFORE-DISPATCH",
    "event-causal-order",
    "function-invoked is sequenced before the callback-dispatch that produces it",
)
def temporal_invoke_before_dispatch(_expectation, ledger):
    rows = ledger["events"]
    left = next(i for i, row in enumerate(rows) if row["kind"] == "callback-dispatch")
    right = next(i for i, row in enumerate(rows) if row["kind"] == "function-invoked")
    rows[left], rows[right] = rows[right], rows[left]
    reindex(ledger)


@mutation(
    "R2-TEMPORAL-BODY-BEFORE-INVOKE",
    "event-causal-order",
    "body-started is sequenced before function-invoked despite the required role edge",
)
def temporal_body_before_invoke(_expectation, ledger):
    rows = ledger["events"]
    left = next(i for i, row in enumerate(rows) if row["kind"] == "function-invoked")
    right = next(i for i, row in enumerate(rows) if row["kind"] == "body-started")
    rows[left], rows[right] = rows[right], rows[left]
    reindex(ledger)


@mutation(
    "R2-CELL-KEY-EMPTY",
    "cell-identity",
    "the mandatory cell identity is present syntactically but empty",
)
def cell_key_empty(_expectation, ledger):
    ledger["cell_key"] = ""


@mutation(
    "R2-CELL-KEY-NONSTRING",
    "cell-identity",
    "the mandatory cell identity has no identity-bearing scalar type",
)
def cell_key_nonstring(_expectation, ledger):
    ledger["cell_key"] = {"not": "a-cell-key"}


@mutation(
    "R2-FUNC-DYNAMIC-ID-ALIAS",
    "func-identity-bijection",
    "func.callback and func.body expose the same runtime-only func_id",
)
def func_dynamic_id_alias(_expectation, ledger):
    obj(ledger, "func.callback")["func_id"] = "f-alias"
    obj(ledger, "func.body")["func_id"] = "f-alias"


@mutation(
    "R2-CARRIER-DYNAMIC-ID-ALIAS",
    "carrier-identity-bijection",
    "callback carrier and both Func objects expose one shared carrier_id",
)
def carrier_dynamic_id_alias(_expectation, ledger):
    for role in ("callback.with", "func.callback", "func.body"):
        obj(ledger, role)["carrier_id"] = "carrier-alias"


@mutation(
    "R2-COUNTER-SPAN-SOURCE-SUBSTITUTION",
    "counter-source-anchor",
    "offset, length and lexical role remain plausible but raw_span source is replaced",
)
def counter_span_source_substitution(_expectation, ledger):
    event(ledger, "counter-occurrence")["raw_span"]["source"] = "unrelated-source"


@mutation(
    "R2-COUNTER-SNAPSHOT-LOCATION-ALIAS",
    "counter-runtime-witness-bijection",
    "snapshot_id is coherently aliased to the location witness instead of its own identity domain",
)
def counter_snapshot_location_alias(_expectation, ledger):
    occurrence = event(ledger, "counter-occurrence")
    occurrence["snapshot_id"] = occurrence["location"]


@mutation(
    "R2-DICT-VALUE-ID-FUNC-ALIAS",
    "dict-runtime-identity",
    "dict.result reuses the Func runtime identity in its value_id field",
)
def dict_value_id_func_alias(_expectation, ledger):
    obj(ledger, "dict.result")["value_id"] = "f-alias"
    obj(ledger, "func.callback")["func_id"] = "f-alias"


@mutation(
    "R2-MALFORMED-COUNTER-END-TYPE",
    "total-classification",
    "malformed raw_span.end must classify Violated rather than abort classification",
)
def malformed_counter_end_type(_expectation, ledger):
    event(ledger, "counter-occurrence")["raw_span"]["end"] = "17"


@mutation(
    "R2-MALFORMED-RUNTIME-ID-CONTAINER",
    "total-classification",
    "unhashable runtime identity must classify Violated rather than abort classification",
)
def malformed_runtime_id_container(_expectation, ledger):
    obj(ledger, "dict.result")["runtime_id"] = ["r-dict"]


def make_opaque(ledger: dict[str, Any]) -> None:
    ledger["payload_state"] = {
        "kind": "opaque",
        "reason_code": "payload-deliberately-opaque",
    }


@mutation(
    "R2-OPAQUE-HIDES-LIFECYCLE-SUBJECT",
    "unknown-semantic-short-circuit",
    "opaque payload wraps a lifecycle event bound to the wrong runtime role",
)
def opaque_hides_lifecycle_subject(expectation, ledger):
    lifecycle_start_wrong_subject(expectation, ledger)
    make_opaque(ledger)


@mutation(
    "R2-OPAQUE-HIDES-TEMPORAL-REVERSAL",
    "unknown-semantic-short-circuit",
    "opaque payload wraps function invocation sequenced before dispatch",
)
def opaque_hides_temporal_reversal(expectation, ledger):
    temporal_invoke_before_dispatch(expectation, ledger)
    make_opaque(ledger)


@mutation(
    "R2-OPAQUE-HIDES-FUNC-ID-ALIAS",
    "unknown-semantic-short-circuit",
    "opaque payload wraps aliased Func runtime identities",
)
def opaque_hides_func_id_alias(expectation, ledger):
    func_dynamic_id_alias(expectation, ledger)
    make_opaque(ledger)


def safely_classify(checker, contract, expectation, ledger) -> dict[str, str]:
    try:
        classification, reason = checker.classify(contract, expectation, ledger)
        return {"classification": classification, "reason_code": reason}
    except Exception as error:  # Contract requires total negative classification.
        return {
            "classification": "CheckerError",
            "reason_code": f"{type(error).__name__}:{error}",
        }


def main() -> int:
    contract = load_json(CONTRACT_PATH)
    focal = load_json(FOCAL_PATH)
    checker = import_module(CHECKER_PATH, "p1341_contract_r3_checker_adversary")
    r1_suite = import_module(R1_SUITE_PATH, "p1341_adversary_r1_suite")
    expectation, positive, built_suite, fixture_errors = checker.build_fixture(ROOT, contract, focal)
    if built_suite.MUTATIONS.keys() != r1_suite.MUTATIONS.keys():
        raise RuntimeError("R3-loaded and independently loaded R1 suite differ")

    protected_rows = load_json(ROOT / focal["base_positive_ledger"][0])["mutations"]
    protected_ids = [row["id"] for row in protected_rows]
    r1_additional_ids = list(r1_suite.ADDITIONAL_CLASS)
    r2_ids = list(NEW_MUTATIONS)
    all_ids = protected_ids + r1_additional_ids + r2_ids
    materializers = dict(r1_suite.MUTATIONS)
    materializers.update(NEW_MUTATIONS)
    if len(protected_ids) != 41 or len(r1_additional_ids) != 10 or len(set(all_ids)) != len(all_ids):
        raise RuntimeError("mutation suite cardinality or identity failure")

    opaque = copy.deepcopy(positive)
    make_opaque(opaque)
    controls = {
        "positive": safely_classify(checker, contract, copy.deepcopy(expectation), copy.deepcopy(positive)),
        "opaque": safely_classify(checker, contract, copy.deepcopy(expectation), opaque),
    }
    orders = {
        "normal": all_ids,
        "repeat": all_ids,
        "reverse": list(reversed(all_ids)),
    }
    runs: dict[str, dict[str, dict[str, str]]] = {}
    for order_name, identifiers in orders.items():
        rows = {}
        for identifier in identifiers:
            mutated_expectation = copy.deepcopy(expectation)
            mutated_ledger = copy.deepcopy(positive)
            materializers[identifier](mutated_expectation, mutated_ledger)
            rows[identifier] = safely_classify(checker, contract, mutated_expectation, mutated_ledger)
        runs[order_name] = rows
    deterministic = runs["normal"] == runs["repeat"] == runs["reverse"]

    groups = {
        "protected_41": protected_ids,
        "r1_additional_10": r1_additional_ids,
        "r2_new": r2_ids,
        "combined": all_ids,
    }
    summaries = {}
    for name, identifiers in groups.items():
        killed = sum(runs["normal"][identifier]["classification"] == "Violated" for identifier in identifiers)
        summaries[name] = {
            "valid": len(identifiers),
            "killed": killed,
            "survived": len(identifiers) - killed,
            "score": killed / len(identifiers),
            "survivors": [
                identifier
                for identifier in identifiers
                if runs["normal"][identifier]["classification"] != "Violated"
            ],
        }
    r2_details = {
        identifier: {
            "class": NEW_CLASS[identifier],
            "rationale": NEW_RATIONALE[identifier],
            "materializer": NEW_MUTATIONS[identifier].__name__,
            **runs["normal"][identifier],
            "killed": runs["normal"][identifier]["classification"] == "Violated",
        }
        for identifier in r2_ids
    }
    survivors = summaries["combined"]["survivors"]
    result = {
        "schema": "p1341-adversary-materialized-result-r2",
        "scope": "R3 synthetic discrimination only; no candidate or product access",
        "inputs": {
            str(path.relative_to(ROOT)): sha256(path)
            for path in (CONTRACT_PATH, CHECKER_PATH, FOCAL_PATH, R1_SUITE_PATH)
        },
        "fixture_errors": fixture_errors,
        "controls": controls,
        "orders": list(orders),
        "deterministic": deterministic,
        "summaries": summaries,
        "r2_new_details": r2_details,
        "verdict": (
            "RECOMMEND_SEAL"
            if not fixture_errors
            and deterministic
            and controls["positive"]["classification"] == "Preserved"
            and controls["opaque"]["classification"] == "Unknown"
            and not survivors
            else "BLOCK_SEAL"
        ),
        "limitations": [
            "Only the synthetic R3 fixture and contract fragment are evaluated.",
            "Unknown and CheckerError are survivors for every negative attack.",
            "Dynamic identity alias attacks rely on the runtime-only identity vocabulary inherited by R3 from R2; if R3 deliberately excludes those fields, that scope reduction requires explicit authority rather than silent checker omission.",
            "No candidate/product source, P1340 implementation, or candidate/product output was read or executed.",
        ],
    }
    print(json.dumps(result, sort_keys=True, indent=2))
    return 0 if result["verdict"] == "RECOMMEND_SEAL" else 1


if __name__ == "__main__":
    raise SystemExit(main())
