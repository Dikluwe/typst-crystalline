#!/usr/bin/env python3
"""Independent pre-implementation gate for the frozen P1358 abstract contract."""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
from pathlib import Path
import sys
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "00_nucleo/diagnosticos/p1358-manifest.json"
EXPECTED_MANIFEST_SHA256 = "8286a48131b94810623dd53787325dff748888b95d69916ea0fd281f7dc1ea0c"
PROFILES = ["default", "html", "a11y-extras", "html+a11y"]
MUTANT_IDS = [
    "M01_EMPTY_RETURNS_WHITE",
    "M02_TYPE_ERROR_RETURNS_WHITE",
    "M03_RANGE_ERROR_RETURNS_WHITE",
    "M04_ALPHA_NAMED_ONLY",
    "M05_ALPHA_NAMED_ALSO_ACCEPTED",
    "M06_REJECT_INTEGER_ALPHA",
    "M07_ACCEPT_OUT_OF_RANGE_INTEGER_ALPHA",
    "M08_OMIT_EMPTY_HINT",
    "M09_CHANGE_LUMA_COLOR_ALPHA",
    "M10_MECHANICAL_EQUALITY_ORACLE",
]


def canonical(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def file_digest(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def load(path: Path) -> Any:
    with path.open(encoding="utf-8") as stream:
        return json.load(stream)


def protected(manifest: dict[str, Any]) -> tuple[dict[str, Any], dict[str, str]]:
    loaded: dict[str, Any] = {}
    hashes: dict[str, str] = {"manifest": file_digest(MANIFEST)}
    if hashes["manifest"] != EXPECTED_MANIFEST_SHA256:
        raise ValueError("manifest SHA-256 drift")
    for name, entry in manifest["protected_inputs"].items():
        path = ROOT / entry["path"]
        actual = file_digest(path)
        if actual != entry["sha256"]:
            raise ValueError(f"protected input SHA-256 drift: {name}")
        hashes[name] = actual
        if path.suffix == ".json":
            loaded[name] = load(path)
    return loaded, hashes


def artifact_consistency(contract: dict[str, Any], oracles: dict[str, Any],
                         mutants: dict[str, Any], manifest: dict[str, Any]) -> list[dict[str, Any]]:
    issues: list[dict[str, Any]] = []
    contract_values = {row["id"]: row for row in contract["values"]}
    oracle_values = {row["id"]: row for row in oracles["value_oracles"]}
    contract_diags = {row["id"]: row for row in contract["diagnostics"]}
    oracle_diags = {row["id"]: row for row in oracles["diagnostic_oracles"]}
    contract_preservation = {row["id"] for row in contract["preservation"]}
    oracle_preservation = {row["id"] for row in oracles["preservation_oracles"]}

    if set(contract_values) != set(oracle_values):
        issues.append({"code": "VALUE_ID_SET_MISMATCH",
                       "contract_ids": sorted(contract_values),
                       "oracle_ids": sorted(oracle_values)})
    value_rows = []
    for ident in sorted(set(contract_values) & set(oracle_values)):
        left, right = contract_values[ident], oracle_values[ident]
        if (left["expression"], left["expected"]) != (right["expression"], right["expected_public_value"]):
            value_rows.append({"id": ident, "contract": [left["expression"], left["expected"]],
                               "oracle": [right["expression"], right["expected_public_value"]]})
    if value_rows:
        issues.append({"code": "VALUE_ROW_MISMATCH", "rows": value_rows})
    if set(contract_diags) != set(oracle_diags):
        issues.append({"code": "DIAGNOSTIC_ID_SET_MISMATCH",
                       "contract_ids": sorted(contract_diags),
                       "oracle_ids": sorted(oracle_diags)})
    rebound = []
    for ident in sorted(set(contract_diags) & set(oracle_diags)):
        left, right = contract_diags[ident], oracle_diags[ident]
        if left["expression"] != right["expression"]:
            rebound.append({"id": ident, "contract_expression": left["expression"],
                            "oracle_expression": right["expression"]})
    if rebound:
        issues.append({"code": "DIAGNOSTIC_ID_REBOUND", "rows": rebound})
    diagnostic_rows = []
    for ident in sorted(set(contract_diags) & set(oracle_diags)):
        left, right = contract_diags[ident], oracle_diags[ident]
        contract_match = left.get("message_match")
        oracle_match = right.get("message_match")
        normalized_oracle_match = "prefix_or_exact" if oracle_match == "public diagnostic begins with or equals the frozen phrase" else oracle_match
        if (left["message"], left.get("hint"), contract_match) != (right["expected_message"], right.get("expected_hint"), normalized_oracle_match):
            diagnostic_rows.append({"id": ident,
                                    "contract": [left["message"], left.get("hint"), contract_match],
                                    "oracle": [right["expected_message"], right.get("expected_hint"), normalized_oracle_match]})
    if diagnostic_rows:
        issues.append({"code": "DIAGNOSTIC_ROW_MISMATCH", "rows": diagnostic_rows})
    if contract_preservation != oracle_preservation:
        issues.append({"code": "PRESERVATION_ID_SET_MISMATCH",
                       "contract_ids": sorted(contract_preservation),
                       "oracle_ids": sorted(oracle_preservation)})
    preservation_rows = []
    contract_preservation_rows = {row["id"]: row for row in contract["preservation"]}
    oracle_preservation_rows = {row["id"]: row for row in oracles["preservation_oracles"]}
    for ident in sorted(contract_preservation & oracle_preservation):
        left, right = contract_preservation_rows[ident], oracle_preservation_rows[ident]
        left_expected = left.get("expected_crystalline", left.get("expected"))
        if (left.get("expression"), left_expected) != (right.get("expression"), right.get("expected_public_value")):
            preservation_rows.append({"id": ident,
                                      "contract": [left.get("expression"), left_expected],
                                      "oracle": [right.get("expression"), right.get("expected_public_value")]})
    if preservation_rows:
        issues.append({"code": "PRESERVATION_ROW_MISMATCH", "rows": preservation_rows})

    expected_hashes = {name: entry["sha256"] for name, entry in manifest["protected_inputs"].items()}
    nested_hash_issues = []
    contract_nested = {"l0": contract["frozen_inputs"]["l0_sha256"],
                       "step": contract["frozen_inputs"]["step_sha256"],
                       "preflight": contract["frozen_inputs"]["preflight_sha256"]}
    for name, actual in contract_nested.items():
        if actual != expected_hashes[name]:
            nested_hash_issues.append({"artifact": "contract", "input": name, "actual": actual,
                                       "expected": expected_hashes[name]})
    for artifact_name, artifact in [("oracles", oracles), ("mutants", mutants)]:
        nested = {Path(row["path"]).name: row["sha256"] for row in artifact["frozen_inputs"]}
        if nested.get("p1358-contract.json") != expected_hashes["contract"]:
            nested_hash_issues.append({"artifact": artifact_name, "input": "contract",
                                       "actual": nested.get("p1358-contract.json"),
                                       "expected": expected_hashes["contract"]})
    if nested_hash_issues:
        issues.append({"code": "NESTED_FROZEN_HASH_MISMATCH", "rows": nested_hash_issues})

    expected_ids = {
        "values": [f"V{i:02d}" for i in range(1, 10)],
        "diagnostics": [f"D{i:02d}" for i in range(1, 17)],
        "preservation": ["P01_LUMA_COLOR_ALPHA_SCOPE_OUT", "P02_LUMA_PUBLIC_KIND"],
    }
    actual_ids = {"values": sorted(contract_values), "diagnostics": sorted(contract_diags),
                  "preservation": sorted(contract_preservation)}
    if actual_ids != expected_ids:
        issues.append({"code": "REQUIRED_EXACT_ID_SURFACE_MISMATCH",
                       "actual": actual_ids, "expected": expected_ids})

    requirements = oracles["gate_requirements"]
    reported = {
        "values": len(oracles["value_oracles"]),
        "diagnostics": len(oracles["diagnostic_oracles"]),
        "preservation": len(oracles["preservation_oracles"]),
        "opaque": len(oracles["opaque_fixtures"]),
    }
    required = {
        "values": requirements["value_oracle_count"],
        "diagnostics": requirements["diagnostic_oracle_count"],
        "preservation": requirements["preservation_oracle_count"],
        "opaque": requirements["opaque_fixture_count"],
    }
    if reported != required:
        issues.append({"code": "ORACLE_COUNT_REQUIREMENT_MISMATCH",
                       "reported": reported, "required": required})
    if contract["profiles"] != PROFILES or oracles["execution_contract"]["profiles"] != PROFILES:
        issues.append({"code": "PROFILE_ORDER_MISMATCH"})
    if manifest["unknown_policy"] != "fail_closed" or contract["unknown_policy"]["mode"] != "fail_closed" or oracles["oracle_policy"]["unknown_mode"] != "fail_closed":
        issues.append({"code": "UNKNOWN_POLICY_MISMATCH"})
    actual_mutants = [row["id"] for row in sorted(mutants["mutants"], key=lambda row: row["ordinal"])]
    if actual_mutants != MUTANT_IDS or any(not row["valid"] or row["expected"] != "Violated" for row in mutants["mutants"]):
        issues.append({"code": "MUTANT_CATALOG_MISMATCH", "actual_ids": actual_mutants})
    return issues


def base_model(oracles: dict[str, Any]) -> dict[str, dict[str, Any]]:
    model: dict[str, dict[str, Any]] = {}
    for row in oracles["value_oracles"]:
        for profile in PROFILES:
            model[f'{row["id"]}@{profile}'] = {
                "status": "Preserved", "kind": "VALUE",
                "observable": row["expected_public_value"],
                "comparison_basis": "canonical_public_language_value",
                "language_observable_compared": True, "reason_code": None,
            }
    for row in oracles["diagnostic_oracles"]:
        for profile in PROFILES:
            model[f'{row["id"]}@{profile}'] = {
                "status": "Preserved", "kind": "DIAGNOSTIC",
                "message": row["expected_message"], "hint": row.get("expected_hint"),
                "span_rule": row["span_rule"], "reason_code": None,
            }
    for row in oracles["preservation_oracles"]:
        for profile in PROFILES:
            model[f'{row["id"]}@{profile}'] = {
                "status": "Preserved", "kind": "PRESERVATION",
                "observable": row["expected_public_value"], "reason_code": None,
            }
    for row in oracles["opaque_fixtures"]:
        model[row["id"]] = {"status": "Unknown", "complete": False,
                            "reason_code": row["reason_code"]}
    return model


def for_profiles(model: dict[str, dict[str, Any]], ident: str):
    return [model[f"{ident}@{profile}"] for profile in PROFILES]


def inject(model: dict[str, dict[str, Any]], mutant_id: str) -> None:
    if mutant_id == "M01_EMPTY_RETURNS_WHITE":
        for row in for_profiles(model, "D01"): row.update(kind="VALUE", observable="luma(100%)")
    elif mutant_id == "M02_TYPE_ERROR_RETURNS_WHITE":
        for row in for_profiles(model, "D02"): row.update(kind="VALUE", observable="luma(100%)")
    elif mutant_id == "M03_RANGE_ERROR_RETURNS_WHITE":
        for row in for_profiles(model, "D04"): row.update(kind="VALUE", observable="luma(100%)")
    elif mutant_id == "M04_ALPHA_NAMED_ONLY":
        for row in for_profiles(model, "V04"): row.update(kind="DIAGNOSTIC", observable="unexpected positional")
        for row in for_profiles(model, "D13"): row.update(kind="VALUE", observable="luma(50.2%, 40%)")
    elif mutant_id == "M05_ALPHA_NAMED_ALSO_ACCEPTED":
        for row in for_profiles(model, "D13"): row.update(kind="VALUE", observable="luma(50.2%, 40%)")
    elif mutant_id == "M06_REJECT_INTEGER_ALPHA":
        for row in for_profiles(model, "V05"): row.update(kind="DIAGNOSTIC", observable="type diagnostic")
    elif mutant_id == "M07_ACCEPT_OUT_OF_RANGE_INTEGER_ALPHA":
        for row in for_profiles(model, "D07"): row.update(kind="VALUE", observable="luma(3.92%)")
    elif mutant_id == "M08_OMIT_EMPTY_HINT":
        for row in for_profiles(model, "D01"): row["hint"] = None
    elif mutant_id == "M09_CHANGE_LUMA_COLOR_ALPHA":
        for row in for_profiles(model, "P01_LUMA_COLOR_ALPHA_SCOPE_OUT"): row["observable"] = "(49.85%, 100%)"
    elif mutant_id == "M10_MECHANICAL_EQUALITY_ORACLE":
        for row in for_profiles(model, "V02"):
            row["comparison_basis"] = "rendered_bytes_sha256"
            row["language_observable_compared"] = False
    else:
        raise ValueError(f"unsupported mutant: {mutant_id}")


def evaluate(model: dict[str, dict[str, Any]], reference: dict[str, dict[str, Any]]) -> list[str]:
    return sorted(key for key in reference if model.get(key) != reference[key])


def run_gate() -> dict[str, Any]:
    manifest = load(MANIFEST)
    loaded, hashes = protected(manifest)
    contract, oracles, mutants = loaded["contract"], loaded["oracles"], loaded["mutants"]
    issues = artifact_consistency(contract, oracles, mutants, manifest)
    reference = base_model(oracles)
    positive_keys = [key for key in reference if not key.startswith("O")]
    opaque_keys = [key for key in reference if key.startswith("O")]
    positives_preserved = sum(reference[key]["status"] == "Preserved" and reference[key]["reason_code"] is None for key in positive_keys)
    opaque_unknown = sum(
        reference[key]["status"] == "Unknown" and bool(reference[key]["reason_code"])
        for key in opaque_keys
    )

    rows = []
    by_id = {row["id"]: row for row in mutants["mutants"]}
    for ident in MUTANT_IDS:
        candidate = copy.deepcopy(reference)
        inject(candidate, ident)
        witnesses = evaluate(candidate, reference)
        verdict = "Violated" if witnesses else "Preserved"
        rows.append({"id": ident, "verdict": verdict, "witnesses": witnesses,
                     "catalog_witness": by_id[ident]["witness"]})
    rejected = sum(row["verdict"] == "Violated" for row in rows)
    valid = sum(row["valid"] for row in mutants["mutants"])
    score = rejected / valid if valid else 0.0

    hashes_by_order = []
    positive_hashes = []
    opaque_hashes = []
    mutant_hashes = []
    positive_rows = [{"key": key, "observation": reference[key]} for key in positive_keys]
    opaque_rows = [{"key": key, "observation": reference[key]} for key in opaque_keys]
    for physical in [rows, list(reversed(rows))]:
        for _ in range(2):
            mutation_payload = sorted(physical, key=lambda row: row["id"])
            positive_payload = sorted(positive_rows, key=lambda row: row["key"])
            opaque_payload = sorted(opaque_rows, key=lambda row: row["key"])
            positive_hashes.append(digest(canonical(positive_payload)))
            opaque_hashes.append(digest(canonical(opaque_payload)))
            mutant_hashes.append(digest(canonical(mutation_payload)))
            hashes_by_order.append(digest(canonical({"positive": positive_payload,
                                                      "opaque": opaque_payload,
                                                      "mutants": mutation_payload})))
    deterministic = all(len(set(group)) == 1 for group in
                        [positive_hashes, opaque_hashes, mutant_hashes, hashes_by_order])
    discriminating = rejected == valid == 10 and score == 1.0 and deterministic
    eligible = discriminating and not issues and positives_preserved == len(positive_keys) and opaque_unknown == len(opaque_keys)
    return {
        "schema": "p1358-contract-gate-receipt-v1",
        "regime": "full_segregated_materialization_protocol",
        "attestation": "executado sem atestação de isolamento",
        "verdict": "PASS" if eligible else "REJECTED",
        "seal_eligible": eligible,
        "input_hashes": hashes,
        "consistency": {"passed": not issues, "issues": issues},
        "positive_control": {"observations": len(positive_keys), "preserved": positives_preserved},
        "opaque_control": {"fixtures": len(opaque_keys), "unknown": opaque_unknown,
                           "parity_credit": 0, "fail_closed": True},
        "mutants": rows,
        "valid_mutants": valid,
        "rejected_mutants": rejected,
        "mutation_score": score,
        "required_mutation_score": 1.0,
        "positive_determinism_hashes": positive_hashes,
        "opaque_determinism_hashes": opaque_hashes,
        "mutant_determinism_hashes": mutant_hashes,
        "determinism_hashes": hashes_by_order,
        "deterministic": deterministic,
        "candidate_patch_read": False,
        "candidate_executed": False,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output")
    args = parser.parse_args()
    try:
        receipt = run_gate()
    except (ValueError, KeyError, FileNotFoundError, json.JSONDecodeError) as error:
        print(json.dumps({"schema": "p1358-contract-gate-error-v1", "verdict": "INVALID", "error": str(error)}, sort_keys=True), file=sys.stderr)
        raise SystemExit(2)
    payload = canonical(receipt) + b"\n"
    if args.output:
        Path(args.output).write_bytes(payload)
    else:
        sys.stdout.buffer.write(payload)
    raise SystemExit(0 if receipt["seal_eligible"] else 1)


if __name__ == "__main__":
    main()
