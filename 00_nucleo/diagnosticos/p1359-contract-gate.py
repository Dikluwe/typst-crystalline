#!/usr/bin/env python3
"""Gate abstrato pré-implementação P1359; não lê nem executa produto candidato."""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo" / "diagnosticos"
MANIFEST_SHA = "1f5221b735c280a64413502d8bbf87d8ba133cb8107a545b35a4a59aca50ddd2"
CONTRACT_SHA = "4f394981ad28be9179efa5f477bef6b9c37536df7791fcbda17e234a3acc5cb7"
ORACLES_SHA = "8818eb105975757a49e3d2ed86236ee6b6eb0a87b1a9720977815a7db9518259"
MUTANTS_SHA = "9b8dee6f929cb6a60437c64c7ef524122d4a0ca22ba57779b4fe9e8d4704aea5"
PROFILES = ["default", "html", "a11y-extras", "html+a11y"]
FAMILIES = [
    ("V", "parse_values", "parse_value_oracles", [f"V{i:02d}" for i in range(1, 8)]),
    ("R", "resolution_values", "resolution_oracles", [f"R{i:02d}" for i in range(1, 7)]),
    ("D", "diagnostics", "diagnostic_oracles", [f"D{i:02d}" for i in range(1, 4)]),
    ("M", "morphology", "morphology_oracles", [f"M{i:02d}" for i in range(1, 8)]),
    ("P", "preservation", "preservation_oracles", [f"P{i:02d}" for i in range(1, 5)]),
]


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def canonical(value: Any) -> bytes:
    return (json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False) + "\n").encode()


def load(name: str) -> dict[str, Any]:
    return json.loads((DIAG / name).read_text())


def contract_projection(prefix: str, item: dict[str, Any]) -> dict[str, Any]:
    if prefix in {"V", "R"}:
        result = {"public_value": item["expected"]}
    elif prefix == "D":
        result = {"exit_code": item["expected_exit"], "message_contains": item["message_contains"]}
    elif prefix == "M":
        result = {"morphology": item["expected"]}
    else:
        result = {"claim": item["claim"]}
    return result


def oracle_projection(prefix: str, item: dict[str, Any]) -> dict[str, Any]:
    if prefix in {"V", "R"}:
        result = {"public_value": item["expected_public_value"]}
    elif prefix == "D":
        result = {"exit_code": item["expected_exit"], "message_contains": item["message_contains"]}
    elif prefix == "M":
        result = {"morphology": item["expected_morphology"]}
    else:
        result = {"claim": item["claim"]}
    return result


def validate_inputs(manifest: dict[str, Any], contract: dict[str, Any], oracles: dict[str, Any], mutants: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    fixed = {
        DIAG / "p1359-manifest.json": MANIFEST_SHA,
        DIAG / "p1359-contract.json": CONTRACT_SHA,
        DIAG / "p1359-oracles.json": ORACLES_SHA,
        DIAG / "p1359-mutants.json": MUTANTS_SHA,
    }
    for path, expected in fixed.items():
        actual = digest(path)
        if actual != expected:
            errors.append(f"hash mismatch: {path.relative_to(ROOT)} expected {expected} got {actual}")
    for entry in manifest["protected_inputs"].values():
        path = ROOT / entry["path"]
        actual = digest(path)
        if actual != entry["sha256"]:
            errors.append(f"manifest protected hash mismatch: {entry['path']}")
    if manifest["unknown_policy"] != "fail_closed":
        errors.append("manifest Unknown policy is not fail_closed")
    if contract["profiles"] != PROFILES or oracles["execution_contract"]["profiles"] != PROFILES:
        errors.append("profile list/order differs from frozen four-profile order")
    if contract["unknown_policy"]["production_unknown_counts_as_success"]:
        errors.append("contract promotes production Unknown")
    if oracles["oracle_policy"]["production_unknown_counts_as_success"]:
        errors.append("oracles promote production Unknown")
    for prefix, contract_key, oracle_key, expected_ids in FAMILIES:
        contract_items = contract[contract_key]
        oracle_items = oracles[oracle_key]
        contract_ids = [item["id"] for item in contract_items]
        oracle_ids = [item["id"] for item in oracle_items]
        if contract_ids != expected_ids or oracle_ids != expected_ids:
            errors.append(f"{prefix} exact ID isomorphism failed")
            continue
        oracle_by_id = {item["id"]: item for item in oracle_items}
        for item in contract_items:
            oid = item["id"]
            oracle = oracle_by_id[oid]
            if oracle.get("expected") != "Preserved":
                errors.append(f"{oid} positive expectation is not Preserved")
            if contract_projection(prefix, item) != oracle_projection(prefix, oracle):
                errors.append(f"{oid} contract/oracle payload mismatch")
    iso = oracles["contract_id_isomorphism"]
    mutant_iso = mutants["contract_id_isomorphism"]
    for _, contract_key, _, expected_ids in FAMILIES:
        if iso[contract_key] != expected_ids or mutant_iso[contract_key] != expected_ids:
            errors.append(f"declared isomorphism mismatch for {contract_key}")
    opaque = oracles["opaque_fixtures"]
    if len(opaque) != 3:
        errors.append(f"expected 3 opaque fixtures, got {len(opaque)}")
    for item in opaque:
        if item.get("expected") != "Unknown" or item.get("input", {}).get("complete") is not False:
            errors.append(f"opaque fixture {item.get('id')} is not fail-closed Unknown")
        if item.get("reason_code") not in oracles["oracle_policy"]["allowed_unknown_reason_codes"]:
            errors.append(f"opaque fixture {item.get('id')} has unapproved reason code")
    mutant_rows = mutants["mutants"]
    expected_mutant_ids = [f"A{i:02d}_{suffix}" for i, suffix in enumerate([
        "COLLAPSE_TRI_STATE", "DISABLE_REQUIRE_EQUALS", "BARE_DOES_NOT_DEFAULT_TRUE",
        "COMPILE_ONLY_NO_WATCH", "ALIAS_W_DROPS_FLAG", "INVERT_EXPLICIT_FALSE",
        "LEGACY_OVERRIDES_MODERN", "L4_IGNORES_MODERN_FLAG", "COUPLE_COMPACT_TO_TAGGING",
        "PROPAGATE_TO_NON_PDF", "REMOVE_LEGACY_FLAG", "COMPARE_RAW_PDF_BYTES",
        "PROMOTE_UNKNOWN",
    ], 1)]
    if [m["id"] for m in mutant_rows] != expected_mutant_ids:
        errors.append("mutant ID/order mismatch")
    if any(m.get("valid") is not True or m.get("expected") != "Violated" for m in mutant_rows):
        errors.append("not every mutant is valid and expected Violated")
    if mutants["coverage"]["valid_mutant_count"] != 13:
        errors.append("declared valid mutant count is not 13")
    return errors


def baseline_cells(oracles: dict[str, Any]) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    cells: list[dict[str, Any]] = []
    for prefix, _, oracle_key, _ in FAMILIES:
        for item in oracles[oracle_key]:
            for profile in PROFILES:
                observable = oracle_projection(prefix, item)
                if prefix == "M":
                    observable["observer"] = "qpdf_structural"
                    observable["qpdf_receipt_complete"] = True
                cells.append({
                    "family": prefix,
                    "oracle_id": item["id"],
                    "profile": profile,
                    "classification": "Preserved",
                    "complete": True,
                    "reason_code": None,
                    "observable": observable,
                })
    opaque = [{
        "oracle_id": item["id"],
        "classification": "Unknown",
        "complete": False,
        "reason_code": item["reason_code"],
        "parity_credit": 0,
    } for item in oracles["opaque_fixtures"]]
    return cells, opaque


def mutate(mutant_id: str, cells: list[dict[str, Any]], opaque: list[dict[str, Any]], profile: str) -> None:
    by_id = {c["oracle_id"]: c for c in cells if c["profile"] == profile}
    if mutant_id == "A01_COLLAPSE_TRI_STATE":
        by_id["V03"]["observable"]["public_value"] = "false"
        by_id["V04"]["observable"]["public_value"] = "false"
    elif mutant_id == "A02_DISABLE_REQUIRE_EQUALS":
        by_id["D01"]["observable"] = {"exit_code": 0, "public_value": "Some(false)"}
    elif mutant_id == "A03_BARE_DOES_NOT_DEFAULT_TRUE":
        by_id["V01"]["observable"]["public_value"] = "None"
        by_id["V05"]["observable"]["public_value"] = "None"
    elif mutant_id == "A04_COMPILE_ONLY_NO_WATCH":
        by_id["V05"]["observable"]["public_value"] = "None"
        by_id["V06"]["observable"]["public_value"] = "None"
    elif mutant_id == "A05_ALIAS_W_DROPS_FLAG":
        by_id["V07"]["observable"]["public_value"] = "None"
    elif mutant_id == "A06_INVERT_EXPLICIT_FALSE":
        by_id["R04"]["observable"]["public_value"] = "Enabled"
        by_id["R06"]["observable"]["public_value"] = "Enabled"
    elif mutant_id == "A07_LEGACY_OVERRIDES_MODERN":
        by_id["R05"]["observable"]["public_value"] = "Disabled"
        by_id["M06"]["observable"]["morphology"] = "untagged"
    elif mutant_id == "A08_L4_IGNORES_MODERN_FLAG":
        by_id["R04"]["observable"]["public_value"] = "Enabled"
        by_id["M04"]["observable"]["morphology"] = "tagged"
    elif mutant_id == "A09_COUPLE_COMPACT_TO_TAGGING":
        by_id["M07"]["observable"]["morphology"] = "untagged_and_verbose"
    elif mutant_id == "A10_PROPAGATE_TO_NON_PDF":
        by_id["P01"]["observable"]["claim"] = "HTML changes under --pdf-tagged=false"
    elif mutant_id == "A11_REMOVE_LEGACY_FLAG":
        by_id["P04"]["observable"]["claim"] = "--no-pdf-tags is rejected"
        by_id["M05"]["observable"]["morphology"] = "parse_error"
    elif mutant_id == "A12_COMPARE_RAW_PDF_BYTES":
        by_id["M02"]["observable"]["observer"] = "raw_sha256"
        by_id["M02"]["observable"]["qpdf_receipt_complete"] = False
    elif mutant_id == "A13_PROMOTE_UNKNOWN":
        target = next(item for item in opaque if item["oracle_id"] == "O01_QPDF_UNAVAILABLE")
        target.update(classification="Preserved", complete=False, reason_code=None, parity_credit=1)
    else:
        raise AssertionError(f"unhandled mutant {mutant_id}")


def run_schedule(oracles: dict[str, Any], mutants: dict[str, Any], reverse: bool) -> dict[str, Any]:
    baseline, opaque_baseline = baseline_cells(oracles)
    physical = list(reversed(baseline)) if reverse else list(baseline)
    positives = [{**cell, "classification": "Preserved"} for cell in physical]
    opaque_physical = list(reversed(opaque_baseline)) if reverse else list(opaque_baseline)
    mutant_rows = list(reversed(mutants["mutants"])) if reverse else list(mutants["mutants"])
    mutation_results: list[dict[str, Any]] = []
    baseline_canonical = canonical(sorted(baseline, key=lambda c: (c["family"], c["oracle_id"], PROFILES.index(c["profile"]))))
    opaque_canonical = canonical(sorted(opaque_baseline, key=lambda c: c["oracle_id"]))
    for mutant in mutant_rows:
        for profile in (list(reversed(PROFILES)) if reverse else PROFILES):
            cells = copy.deepcopy(baseline)
            opaque = copy.deepcopy(opaque_baseline)
            mutate(mutant["id"], cells, opaque, profile)
            violated = canonical(sorted(cells, key=lambda c: (c["family"], c["oracle_id"], PROFILES.index(c["profile"])))) != baseline_canonical or canonical(sorted(opaque, key=lambda c: c["oracle_id"])) != opaque_canonical
            mutation_results.append({
                "mutant_id": mutant["id"],
                "profile": profile,
                "classification": "Violated" if violated else "Preserved",
            })
    return {
        "positives": sorted(positives, key=lambda c: (c["family"], c["oracle_id"], PROFILES.index(c["profile"]))),
        "opaque": sorted(opaque_physical, key=lambda c: c["oracle_id"]),
        "mutations": sorted(mutation_results, key=lambda c: (c["mutant_id"], PROFILES.index(c["profile"]))),
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    manifest = load("p1359-manifest.json")
    contract = load("p1359-contract.json")
    oracles = load("p1359-oracles.json")
    mutants = load("p1359-mutants.json")
    errors = validate_inputs(manifest, contract, oracles, mutants)
    schedules = [run_schedule(oracles, mutants, reverse) for reverse in (False, False, True, True)]
    hashes = [hashlib.sha256(canonical(schedule)).hexdigest() for schedule in schedules]
    reference = schedules[0]
    positive_ok = sum(c["classification"] == "Preserved" and c["complete"] and c["reason_code"] is None for c in reference["positives"])
    opaque_ok = sum(c["classification"] == "Unknown" and not c["complete"] and c["parity_credit"] == 0 for c in reference["opaque"])
    mutant_profiles = {}
    for row in reference["mutations"]:
        mutant_profiles.setdefault(row["mutant_id"], []).append(row["classification"])
    killed = sum(all(value == "Violated" for value in values) and len(values) == 4 for values in mutant_profiles.values())
    mutation_score = killed / 13
    if positive_ok != 108:
        errors.append(f"positive controls: expected 108 Preserved, got {positive_ok}")
    if opaque_ok != 3:
        errors.append(f"opaque controls: expected 3 Unknown without credit, got {opaque_ok}")
    if killed != 13:
        errors.append(f"mutants: expected 13 Violated in all profiles, got {killed}")
    if len(set(hashes)) != 1:
        errors.append("canonical/reverse repetitions are not deterministic")
    result = {
        "schema": "p1359-contract-gate-result-v1",
        "created_utc": datetime.now(timezone.utc).isoformat().replace("+00:00", "Z"),
        "regime": "full_segregated_materialization_protocol",
        "attestation": "executado sem atestação de isolamento",
        "executor": "/root/verificador_p1356",
        "candidate_product_read": False,
        "candidate_product_executed": False,
        "inputs": {
            "manifest_sha256": digest(DIAG / "p1359-manifest.json"),
            "contract_sha256": digest(DIAG / "p1359-contract.json"),
            "oracles_sha256": digest(DIAG / "p1359-oracles.json"),
            "mutants_sha256": digest(DIAG / "p1359-mutants.json"),
        },
        "isomorphism": {"V01-V07": 7, "R01-R06": 6, "D01-D03": 3, "M01-M07": 7, "P01-P04": 4, "exact": not any("isomorphism" in e or "payload mismatch" in e for e in errors)},
        "positive_controls": {"preserved": positive_ok, "total": 108},
        "opaque_controls": {"unknown": opaque_ok, "total": 3, "parity_credit": 0},
        "mutants": {"violated": killed, "valid": 13, "profile_executions": 52, "mutation_score": mutation_score},
        "determinism": {"physical_orders": ["canonical", "canonical", "reverse", "reverse"], "hashes": hashes, "stable": len(set(hashes)) == 1},
        "cost": {"abstract_schedules": 4, "positive_profile_cells_per_schedule": 108, "opaque_cells_per_schedule": 3, "mutant_profile_injections_per_schedule": 52},
        "errors": errors,
        "verdict": "PASS" if not errors else "FAIL",
    }
    encoded = json.dumps(result, indent=2, ensure_ascii=False) + "\n"
    if args.output:
        args.output.write_text(encoded)
    print(encoded, end="")
    return 0 if not errors else 1


if __name__ == "__main__":
    raise SystemExit(main())
