#!/usr/bin/env python3
"""Total fail-closed P1341 R4 checker and 67-case materialized gate."""

from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import sys
from collections import Counter
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


def strict_string(value: Any, prefix: str | None = None) -> bool:
    return isinstance(value, str) and bool(value) and (prefix is None or value.startswith(prefix))


def strict_int(value: Any) -> bool:
    return isinstance(value, int) and not isinstance(value, bool)


def remap_runtime_ids(ledger: dict[str, Any], role_to_new: dict[str, str]) -> None:
    old_to_new = {row["runtime_id"]: role_to_new[row["stable_role"]] for row in ledger["objects"]}
    for row in ledger["objects"]:
        row["runtime_id"] = old_to_new[row["runtime_id"]]
    for event in ledger["events"]:
        event["subject"] = old_to_new[event["subject"]]
        for key, value in list(event.items()):
            if key.endswith("_ref"):
                event[key] = old_to_new[value]
    for edge in ledger["causal_edges"]:
        edge["from"] = old_to_new[edge["from"]]
        edge["to"] = old_to_new[edge["to"]]


def build_fixture(root: Path, contract: dict[str, Any], focal: dict[str, Any]):
    errors: list[str] = []
    pins = ("base_contract", "base_focal", "base_checker", "r1_materializer", "r2_materializer")
    for key in pins:
        path = root / focal[key][0]
        if sha256(path) != focal[key][1]:
            errors.append("protected-input-hash:" + key)
    r3_contract = load_json(root / focal["base_contract"][0])
    r3_focal = load_json(root / focal["base_focal"][0])
    r3_checker = import_module(root / focal["base_checker"][0], "p1341_r3_checker_for_r4")
    expectation, ledger, _suite, inherited_errors = r3_checker.build_fixture(root, r3_contract, r3_focal)
    errors.extend(inherited_errors)
    overlay = focal["runtime_overlay"]
    remap_runtime_ids(ledger, overlay["runtime_id_by_role"])
    objects = {row["stable_role"]: row for row in ledger["objects"]}
    for role, value in overlay["func_id_by_role"].items():
        objects[role]["func_id"] = value
    for role, value in overlay["carrier_id_by_role"].items():
        objects[role]["carrier_id"] = value
    for role, value in overlay["value_id_by_role"].items():
        objects[role]["value_id"] = value
    occurrence = next(row for row in ledger["events"] if row["kind"] == "counter-occurrence")
    occurrence.update(copy.deepcopy(overlay["counter_event"]))
    r1 = import_module(root / focal["r1_materializer"][0], "p1341_r1_suite_for_r4")
    r2 = import_module(root / focal["r2_materializer"][0], "p1341_r2_suite_for_r4")
    return expectation, ledger, r3_contract, r3_checker, r1, r2, errors


def _classify(contract: dict[str, Any], expectation: Any, ledger: Any, r3_contract: dict[str, Any], r3_checker: Any) -> tuple[str, str]:
    # Every R4 predicate is evaluated before R3 can reach opacity.
    if not isinstance(expectation, dict) or not isinstance(ledger, dict):
        return violated("strict-top-level-schema")
    if not strict_string(ledger.get("cell_key")):
        return violated("cell-key-type-or-empty")
    objects = ledger.get("objects")
    events = ledger.get("events")
    if not isinstance(objects, list) or not isinstance(events, list):
        return violated("strict-ledger-collections")

    domains = contract["runtime_identity_domains"]
    roles: dict[str, dict[str, Any]] = {}
    runtime_ids: list[str] = []
    for row in objects:
        if not isinstance(row, dict) or not strict_string(row.get("runtime_id"), domains["runtime_id"]["prefix"]) or not strict_string(row.get("stable_role")) or not strict_string(row.get("kind")):
            return violated("strict-object-schema")
        if row["stable_role"] in roles:
            return violated("duplicate-stable-role")
        roles[row["stable_role"]] = row
        runtime_ids.append(row["runtime_id"])
    if len(runtime_ids) != len(set(runtime_ids)):
        return violated("runtime-id-alias")

    slots: list[tuple[str, str, str]] = [("runtime_id", role, row["runtime_id"]) for role, row in roles.items()]
    for domain in ("func_id", "carrier_id", "value_id"):
        spec = domains[domain]
        for role in spec["roles"]:
            if role not in roles or not strict_string(roles[role].get(domain), spec["prefix"]):
                return violated("identity-domain-type:" + domain)
            slots.append((domain, role, roles[role][domain]))
    values = [value for _, _, value in slots]
    if len(values) != len(set(values)):
        return violated("cross-domain-or-owner-identity-alias")

    for event in events:
        if not isinstance(event, dict) or not strict_int(event.get("seq")) or not strict_string(event.get("kind")) or not strict_string(event.get("subject"), domains["runtime_id"]["prefix"]):
            return violated("strict-event-schema")
    by_kind: dict[str, list[dict[str, Any]]] = {}
    for event in events:
        by_kind.setdefault(event["kind"], []).append(event)
    precedence = contract["lifecycle"]["strict_precedence"]
    if any(len(by_kind.get(kind, [])) != 1 for kind in precedence):
        return violated("temporal-event-cardinality")
    role_of = {row["runtime_id"]: row["stable_role"] for row in objects}
    start = by_kind["execution-start"][0]
    end = by_kind["execution-end"][0]
    if role_of.get(start["subject"]) != "execution" or role_of.get(end["subject"]) != "execution":
        return violated("lifecycle-subject-role")
    positions = [events.index(by_kind[kind][0]) for kind in precedence]
    if positions != sorted(positions) or len(positions) != len(set(positions)):
        return violated("strict-temporal-precedence")

    occurrence = by_kind["counter-occurrence"][0]
    location = occurrence.get("location")
    snapshot = occurrence.get("snapshot_id")
    if not strict_string(location, domains["location"]["prefix"]) or not strict_string(snapshot, domains["snapshot_id"]["prefix"]) or location == snapshot:
        return violated("counter-location-snapshot-domain")
    slots.extend([("location", "counter-occurrence", location), ("snapshot_id", "counter-occurrence", snapshot)])
    span = occurrence.get("raw_span")
    coordinate = contract["counter_span"]["source_coordinate"]
    if not isinstance(span, dict) or set(span) != set(contract["counter_span"]["strict_keys"]):
        return violated("raw-span-strict-schema")
    if not strict_string(span.get("source"), domains["raw_span.source"]["prefix"]) or not strict_int(span.get("start")) or not strict_int(span.get("end")) or not strict_string(span.get("lexical_role")):
        return violated("raw-span-strict-types")
    if span["source"] != "source:" + coordinate["source_anchor"] or span["start"] != coordinate["source_byte_offset"] or span["end"] - span["start"] != coordinate["byte_length"] or span["lexical_role"] != coordinate["lexical_role"]:
        return violated("raw-span-source-coordinate-domain")
    slots.append(("raw_span.source", "counter-occurrence", span["source"]))
    all_identity_values = [value for _, _, value in slots]
    if len(all_identity_values) != len(set(all_identity_values)):
        return violated("global-dynamic-identity-alias")

    # R3 owns external completeness, semantic bindings, Dict and causal topology.
    return r3_checker.classify(r3_contract, expectation, ledger)


def classify(contract: dict[str, Any], expectation: Any, ledger: Any, r3_contract: dict[str, Any] | None = None, r3_checker: Any | None = None) -> tuple[str, str]:
    try:
        if r3_contract is None or r3_checker is None:
            root = Path(__file__).resolve().parents[2]
            base_path = root / contract["base_contract"][0]
            checker_path = root / "00_nucleo/diagnosticos/p1341-contract-r3-checker.py"
            r3_contract = load_json(base_path)
            r3_checker = import_module(checker_path, "p1341_r3_checker_direct_r4")
        return _classify(contract, expectation, ledger, r3_contract, r3_checker)
    except Exception as error:
        return violated("malformed-fail-closed:" + type(error).__name__)


def validate_contract(root: Path, contract: Any) -> list[str]:
    if not isinstance(contract, dict) or contract.get("schema") != "p1341-contract-r4-v1":
        return ["contract-schema"]
    errors = []
    if contract.get("status") != "NOT_SEALED":
        errors.append("contract-status")
    base = root / contract["base_contract"][0]
    if sha256(base) != contract["base_contract"][1]:
        errors.append("base-contract-pin")
    if contract.get("total_validation", {}).get("unknown_order", [])[-1:] != ["opacity"]:
        errors.append("opacity-not-last")
    gate = contract.get("materialized_gate", {})
    if gate.get("combined") != {"valid":67,"required_killed":67,"required_score":1.0}:
        errors.append("gate-threshold")
    return errors


def run(contract_path: Path, focal_path: Path) -> dict[str, Any]:
    root = contract_path.resolve().parents[2]
    contract = load_json(contract_path)
    focal = load_json(focal_path)
    errors = validate_contract(root, contract)
    if focal.get("schema") != "p1341-contract-r4-focal-v1":
        errors.append("focal-schema")
    expectation, positive, r3_contract, r3_checker, r1, r2, fixture_errors = build_fixture(root, contract, focal)
    errors.extend(fixture_errors)
    control = classify(contract, copy.deepcopy(expectation), copy.deepcopy(positive), r3_contract, r3_checker)
    opaque = copy.deepcopy(positive)
    opaque["payload_state"] = {"kind":"opaque", "reason_code":focal["controls"]["opaque_reason_code"]}
    opaque_control = classify(contract, copy.deepcopy(expectation), opaque, r3_contract, r3_checker)
    if control[0] != "Preserved":
        errors.append("positive-control:" + control[1])
    if opaque_control[0] != "Unknown":
        errors.append("opaque-control:" + opaque_control[1])

    r3_focal = load_json(root / focal["base_focal"][0])
    protected_ids = [row["id"] for row in load_json(root / r3_focal["base_positive_ledger"][0])["mutations"]]
    r1_ids = list(r1.ADDITIONAL_CLASS)
    r2_ids = list(r2.NEW_MUTATIONS)
    all_ids = protected_ids + r1_ids + r2_ids
    materializers = dict(r1.MUTATIONS)
    materializers.update(r2.NEW_MUTATIONS)
    if (len(protected_ids), len(r1_ids), len(r2_ids), len(set(all_ids)), set(materializers)) != (41, 10, 16, 67, set(all_ids)):
        errors.append("materializer-bijection")
    orders = {"normal":all_ids, "repeat":all_ids, "reverse":list(reversed(all_ids))}
    runs: dict[str, dict[str, tuple[str, str]]] = {}
    for order_name, identifiers in orders.items():
        rows = {}
        for identifier in identifiers:
            mutated_expectation = copy.deepcopy(expectation)
            mutated_ledger = copy.deepcopy(positive)
            try:
                materializers[identifier](mutated_expectation, mutated_ledger)
                rows[identifier] = classify(contract, mutated_expectation, mutated_ledger, r3_contract, r3_checker)
            except Exception as error:
                rows[identifier] = ("MaterializerError", type(error).__name__)
        runs[order_name] = rows
    deterministic = runs["normal"] == runs["repeat"] == runs["reverse"]
    if not deterministic:
        errors.append("order-nondeterminism")
    groups = {"protected_41":protected_ids, "r1_additional_10":r1_ids, "r2_new_16":r2_ids, "combined":all_ids}
    summaries = {}
    for name, identifiers in groups.items():
        killed = sum(runs["normal"][identifier][0] == "Violated" for identifier in identifiers)
        survivors = [identifier for identifier in identifiers if runs["normal"][identifier][0] != "Violated"]
        summaries[name] = {"valid":len(identifiers),"killed":killed,"survived":len(survivors),"score":killed/len(identifiers),"survivors":survivors}
    if summaries["combined"]["killed"] != 67:
        errors.append("materialized-survivor")
    verdict = "FOCAL_MATERIALIZED_PASS_NOT_SEALED" if not errors else "BLOCK_SEAL"
    return {
        "schema":"p1341-contract-r4-check-result-v1",
        "contract_status":contract.get("status"),
        "controls":{"positive":control[0],"opaque":opaque_control[0]},
        "orders":["normal","repeat","reverse"],
        "deterministic":deterministic,
        "summaries":summaries,
        "r2_new_details":{identifier:{"classification":runs["normal"][identifier][0],"reason_code":runs["normal"][identifier][1]} for identifier in r2_ids},
        "errors":errors,
        "verdict":verdict
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
