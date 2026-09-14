#!/usr/bin/env python3
"""Gate estatico de seguranca pre-execucao do P1359.

Este programa le somente as entradas protegidas explicitamente autorizadas,
nao inspeciona codigo/testes/binarios candidatos e nao cria subprocessos. A
avaliacao dos oraculos e mutantes ocorre integralmente em memoria.
"""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]

EXPECTED_HASHES = {
    "manifest": (
        "00_nucleo/diagnosticos/p1359-manifest.json",
        "acedfffe1bb426855d542dc9e0e36cdc99a61c20efd9ad2b6f7aa6802234e64d",
    ),
    "contract": (
        "00_nucleo/diagnosticos/p1359-contract.json",
        "a6004e2e759048a8651abe54608110b7b8812cb38b200df58abe5e58a299aedd",
    ),
    "oracles": (
        "00_nucleo/diagnosticos/p1359-oracles.json",
        "eb1846a8b9a612227af31a3a6c15d474d1f461ec4d5991f02952f7c69ea07e50",
    ),
    "mutants": (
        "00_nucleo/diagnosticos/p1359-mutants.json",
        "7a8eb96dfb45b3c55cc49f7449edef9c4a5a4a11aaabea7c12de41cb629eb10b",
    ),
    "memory_root_cause": (
        "00_nucleo/diagnosticos/p1359-memory-root-cause.md",
        "ff6175293ef32ad5bf921f828257c181815d46b3bc19b918f0d68219800f43b3",
    ),
    "step": (
        "00_nucleo/materialization/typst-passo-1359.md",
        "9b9904c638e14fe21d04820807e53396dbeb6b7b3dbba18a1eaaf52973d0fc4d",
    ),
    "shell_l0": (
        "00_nucleo/prompts/shell/cli.md",
        "37dbd1947ba6e1d89eb3598c548401ac922e3028a347e6153dbdc674725393ac",
    ),
    "wiring_l0": (
        "00_nucleo/prompts/wiring.md",
        "5d4dd2773b24f919ff45306bac6a4963eaf1b0683d92cadf2e8c15d136f16514",
    ),
}

FAMILIES = {
    "parse_values": [f"V{i:02d}" for i in range(1, 8)],
    "resolution_values": [f"R{i:02d}" for i in range(1, 7)],
    "diagnostics": [f"D{i:02d}" for i in range(1, 4)],
    "morphology": [f"M{i:02d}" for i in range(1, 8)],
    "preservation": [f"P{i:02d}" for i in range(1, 5)],
}

ORACLE_KEYS = {
    "parse_values": "parse_value_oracles",
    "resolution_values": "resolution_oracles",
    "diagnostics": "diagnostic_oracles",
    "morphology": "morphology_oracles",
    "preservation": "preservation_oracles",
}

EXPECTED_MUTANTS = [
    "A01_COLLAPSE_TRI_STATE",
    "A02_DISABLE_REQUIRE_EQUALS",
    "A03_BARE_DOES_NOT_DEFAULT_TRUE",
    "A04_COMPILE_ONLY_NO_WATCH",
    "A05_ALIAS_W_DROPS_FLAG",
    "A06_INVERT_EXPLICIT_FALSE",
    "A07_LEGACY_OVERRIDES_MODERN",
    "A08_L4_IGNORES_MODERN_FLAG",
    "A09_COUPLE_COMPACT_TO_TAGGING",
    "A10_PROPAGATE_TO_NON_PDF",
    "A11_REMOVE_LEGACY_FLAG",
    "A12_COMPARE_RAW_PDF_BYTES",
    "A13_PROMOTE_UNKNOWN",
    "A14_OPEN_WATCH_SUBPROCESS",
]

EXPECTED_OPAQUES = {
    "O01_QPDF_UNAVAILABLE": "PARSER_UNSUPPORTED",
    "O02_BINARY_IDENTITY_AMBIGUOUS": "AMBIGUOUS_PRODUCT_IDENTITY",
    "O03_IN_PROCESS_PARSER_UNAVAILABLE": "PARSER_UNSUPPORTED",
}

PROFILES = ["default", "html", "a11y-extras", "html+a11y"]


def canonical_bytes(value: Any) -> bytes:
    return (
        json.dumps(
            value,
            ensure_ascii=False,
            sort_keys=True,
            separators=(",", ":"),
        )
        + "\n"
    ).encode("utf-8")


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def check(condition: bool, code: str, failures: list[str]) -> None:
    if not condition:
        failures.append(code)


def ids(items: list[dict[str, Any]]) -> list[str]:
    return [item.get("id") for item in items]


def indexed(items: list[dict[str, Any]]) -> dict[str, dict[str, Any]]:
    return {item["id"]: item for item in items}


def main() -> int:
    failures: list[str] = []
    raw: dict[str, bytes] = {}
    actual_hashes: dict[str, dict[str, str]] = {}

    for name, (relative_path, expected_hash) in EXPECTED_HASHES.items():
        data = (ROOT / relative_path).read_bytes()
        actual_hash = digest(data)
        raw[name] = data
        actual_hashes[name] = {
            "path": relative_path,
            "expected": expected_hash,
            "actual": actual_hash,
        }
        check(actual_hash == expected_hash, f"HASH_MISMATCH:{name}", failures)

    parsed: dict[str, Any] = {}
    for name in ("manifest", "contract", "oracles", "mutants"):
        try:
            parsed[name] = json.loads(raw[name])
        except (UnicodeDecodeError, json.JSONDecodeError) as error:
            failures.append(f"INVALID_JSON:{name}:{type(error).__name__}")

    if failures:
        result = {
            "schema": "p1359-safety-gate-result-v1",
            "verdict": "FAIL",
            "phase": "pre_execution",
            "failures": sorted(failures),
            "hashes": actual_hashes,
            "product_processes_executed": 0,
        }
        sys.stdout.buffer.write(canonical_bytes(result))
        return 1

    manifest = parsed["manifest"]
    contract = parsed["contract"]
    oracles = parsed["oracles"]
    mutants = parsed["mutants"]

    # O manifesto deve pinhar exatamente as sete entradas protegidas auxiliares.
    protected = manifest.get("protected_inputs", {})
    for name in (
        "contract",
        "oracles",
        "mutants",
        "memory_root_cause",
        "step",
        "shell_l0",
        "wiring_l0",
    ):
        relative_path, expected_hash = EXPECTED_HASHES[name]
        entry = protected.get(name, {})
        check(entry.get("path") == relative_path, f"MANIFEST_PATH:{name}", failures)
        check(entry.get("sha256") == expected_hash, f"MANIFEST_HASH:{name}", failures)

    check(manifest.get("unknown_policy") == "fail_closed", "MANIFEST_UNKNOWN", failures)
    check(
        manifest.get("execution_safety")
        == "watch and w are parser-only; open-ended subprocess execution is forbidden",
        "MANIFEST_EXECUTION_SAFETY",
        failures,
    )
    check(
        manifest.get("baseline") == contract.get("baseline"),
        "BASELINE_CONTRACT",
        failures,
    )

    # Isomorfismo integral V/R/D/M/P entre contrato, oraculos e catalogo.
    positive_records: list[dict[str, Any]] = []
    for family, expected_ids in FAMILIES.items():
        contract_items = contract.get(family, [])
        oracle_items = oracles.get(ORACLE_KEYS[family], [])
        check(ids(contract_items) == expected_ids, f"CONTRACT_IDS:{family}", failures)
        check(ids(oracle_items) == expected_ids, f"ORACLE_IDS:{family}", failures)
        check(
            oracles.get("contract_id_isomorphism", {}).get(family) == expected_ids,
            f"ORACLE_ISOMORPHISM:{family}",
            failures,
        )
        check(
            mutants.get("contract_id_isomorphism", {}).get(family) == expected_ids,
            f"MUTANT_ISOMORPHISM:{family}",
            failures,
        )

        contract_by_id = indexed(contract_items)
        oracle_by_id = indexed(oracle_items)
        for oracle_id in expected_ids:
            contract_item = contract_by_id[oracle_id]
            oracle_item = oracle_by_id[oracle_id]
            check(oracle_item.get("expected") == "Preserved", f"POSITIVE:{oracle_id}", failures)
            if family in ("parse_values", "resolution_values"):
                check(
                    oracle_item.get("expected_public_value") == contract_item.get("expected"),
                    f"VALUE:{oracle_id}",
                    failures,
                )
            elif family == "diagnostics":
                check(
                    all(
                        oracle_item.get(field) == contract_item.get(field)
                        for field in ("command", "expected_exit", "message_contains")
                    ),
                    f"DIAGNOSTIC:{oracle_id}",
                    failures,
                )
            elif family == "morphology":
                check(
                    oracle_item.get("flags") == contract_item.get("flags")
                    and oracle_item.get("expected_morphology") == contract_item.get("expected"),
                    f"MORPHOLOGY:{oracle_id}",
                    failures,
                )
            else:
                check(
                    oracle_item.get("claim") == contract_item.get("claim"),
                    f"PRESERVATION:{oracle_id}",
                    failures,
                )
            positive_records.append({"id": oracle_id, "classification": "Preserved"})

    expected_flat_ids = [item for family in FAMILIES.values() for item in family]
    check(len(expected_flat_ids) == 27, "POSITIVE_COUNT_MODEL", failures)
    check(len(positive_records) == 27, "POSITIVE_COUNT", failures)
    check(oracles.get("gate_requirements", {}).get("positive_oracle_count") == 27, "GATE_POSITIVE_COUNT", failures)
    check(
        oracles.get("gate_requirements", {}).get("scheduled_positive_profile_cells") == 108,
        "POSITIVE_PROFILE_CELLS",
        failures,
    )

    contract_iso_hash = actual_hashes["contract"]["actual"]
    check(
        oracles.get("contract_id_isomorphism", {}).get("contract_sha256") == contract_iso_hash,
        "ORACLE_CONTRACT_HASH",
        failures,
    )
    check(
        mutants.get("contract_id_isomorphism", {}).get("contract_sha256") == contract_iso_hash,
        "MUTANT_CONTRACT_HASH",
        failures,
    )

    # V05-V07 sao exclusivamente parser in-process; watch/w nunca entram na matriz.
    parser_only = ["V05", "V06", "V07"]
    parse_by_id = indexed(oracles.get("parse_value_oracles", []))
    contract_parse_by_id = indexed(contract.get("parse_values", []))
    for oracle_id in parser_only:
        observed = parse_by_id[oracle_id]
        frozen = contract_parse_by_id[oracle_id]
        check(
            observed.get("observer_mode") == "in_process_parser_only"
            and observed.get("observer_api") == "Args::try_parse_from"
            and observed.get("subprocess_forbidden") is True,
            f"PARSER_ONLY:{oracle_id}",
            failures,
        )
        check(
            frozen.get("observer_mode") == "in_process_parser_only",
            f"CONTRACT_PARSER_ONLY:{oracle_id}",
            failures,
        )

    execution_contract = oracles.get("execution_contract", {})
    check(execution_contract.get("in_process_parser_only") == parser_only, "PARSER_ONLY_SET", failures)
    check(execution_contract.get("process_matrix_commands") == ["compile"], "PROCESS_MATRIX", failures)
    forbidden_plans = execution_contract.get("forbidden_execution_plans", [])
    check(len(forbidden_plans) == 3, "FORBIDDEN_PLAN_COUNT", failures)
    check(
        any("open-ended subprocess" in plan for plan in forbidden_plans)
        and any("blocking stdout or stderr capture" in plan for plan in forbidden_plans)
        and any("timeout or partial output" in plan for plan in forbidden_plans),
        "FORBIDDEN_PLANS",
        failures,
    )
    check(
        contract.get("execution_safety", {}).get("required")
        == [
            "observe V05-V07 with Args::try_parse_from",
            "use only compile for process morphology",
            "bound every external process output and lifetime",
        ],
        "CONTRACT_REQUIRED_SAFETY",
        failures,
    )

    root_cause_text = raw["memory_root_cause"].decode("utf-8")
    step_text = raw["step"].decode("utf-8")
    shell_l0_text = raw["shell_l0"].decode("utf-8")
    wiring_l0_text = raw["wiring_l0"].decode("utf-8")
    for label, text in (
        ("ROOT_CAUSE", root_cause_text),
        ("STEP", step_text),
    ):
        check("Args::try_parse_from" in text, f"{label}:PARSER_API", failures)
        check("somente `compile`" in text, f"{label}:COMPILE_ONLY", failures)
        check("subprocess" in text, f"{label}:SUBPROCESS_FORBIDDEN", failures)
    check("## P1359 — flag `--pdf-tagged` tri-state" in shell_l0_text, "SHELL_L0_P1359", failures)
    check("pdf_tagged: Option<bool>" in shell_l0_text, "SHELL_L0_TRI_STATE", failures)
    check("## P1359 — precedência de `--pdf-tagged`" in wiring_l0_text, "WIRING_L0_P1359", failures)
    check("Some(false) -> PdfTags::Disabled" in wiring_l0_text, "WIRING_L0_FALSE", failures)

    # Opacos permanecem Unknown, com reason_code predeclarado e sem credito.
    opaque_records: list[dict[str, Any]] = []
    opaque_items = oracles.get("opaque_fixtures", [])
    check(ids(opaque_items) == list(EXPECTED_OPAQUES), "OPAQUE_IDS", failures)
    allowed_reasons = set(oracles.get("oracle_policy", {}).get("allowed_unknown_reason_codes", []))
    for item in opaque_items:
        opaque_id = item.get("id")
        reason = item.get("reason_code")
        check(item.get("expected") == "Unknown", f"OPAQUE_EXPECTED:{opaque_id}", failures)
        check(reason == EXPECTED_OPAQUES.get(opaque_id), f"OPAQUE_REASON:{opaque_id}", failures)
        check(reason in allowed_reasons, f"OPAQUE_REASON_ALLOWED:{opaque_id}", failures)
        check(item.get("input", {}).get("complete") is False, f"OPAQUE_INCOMPLETE:{opaque_id}", failures)
        check("Preserved" in item.get("forbidden_promotions", []), f"OPAQUE_NO_CREDIT:{opaque_id}", failures)
        opaque_records.append(
            {"id": opaque_id, "classification": "Unknown", "reason_code": reason, "parity_credit": 0}
        )
    check(contract.get("unknown_policy", {}).get("production_unknown_counts_as_success") is False, "CONTRACT_UNKNOWN_CREDIT", failures)
    check(oracles.get("oracle_policy", {}).get("production_unknown_counts_as_success") is False, "ORACLE_UNKNOWN_CREDIT", failures)

    # Os 14 mutantes sao abstratos e decisivos. A14 e recusado no pre_execution.
    mutation_records: list[dict[str, Any]] = []
    mutant_items = mutants.get("mutants", [])
    check(ids(mutant_items) == EXPECTED_MUTANTS, "MUTANT_IDS", failures)
    check([item.get("ordinal") for item in mutant_items] == list(range(1, 15)), "MUTANT_ORDINALS", failures)
    for item in mutant_items:
        mutant_id = item.get("id")
        injection = item.get("deterministic_injection", {})
        witness = item.get("witness", {})
        decisive = (
            item.get("valid") is True
            and item.get("expected") == "Violated"
            and bool(item.get("validity_basis"))
            and bool(injection.get("target"))
            and bool(injection.get("operation"))
            and bool(witness.get("boundary"))
            and bool(witness.get("predicate"))
            and bool(witness.get("expected_observed"))
        )
        check(decisive, f"MUTANT_NOT_DECISIVE:{mutant_id}", failures)
        mutation_records.append(
            {
                "id": mutant_id,
                "classification": "Violated" if decisive else "Unknown",
                "phase": item.get("rejection_phase", "pre_execution_static_fixture"),
            }
        )

    a14 = indexed(mutant_items).get("A14_OPEN_WATCH_SUBPROCESS", {})
    check(a14.get("rejection_phase") == "pre_execution", "A14_PHASE", failures)
    check(a14.get("must_spawn_process") is False, "A14_NO_SPAWN", failures)
    check(
        a14.get("deterministic_injection", {}).get("operation")
        == "replace Args::try_parse_from with spawn_and_blocking_output_capture",
        "A14_OPERATION",
        failures,
    )
    check(
        a14.get("deterministic_injection", {}).get("argv", [None])[0] == "watch",
        "A14_WATCH_PLAN",
        failures,
    )
    check(
        indexed(mutation_records).get("A14_OPEN_WATCH_SUBPROCESS", {}).get("classification")
        == "Violated",
        "A14_NOT_VIOLATED",
        failures,
    )
    check(mutants.get("coverage", {}).get("valid_mutant_count") == 14, "MUTANT_COUNT", failures)
    check(mutants.get("coverage", {}).get("required_rejection_count_for_score_1_0") == 14, "REJECTION_COUNT", failures)

    rejected = sum(item["classification"] == "Violated" for item in mutation_records)
    mutation_score = rejected / 14
    check(rejected == 14, "MUTANT_SURVIVOR", failures)
    check(mutation_score == 1.0, "MUTATION_SCORE", failures)

    # Determinismo: expande perfis e compara ordem canonica vs reversa, 2x.
    frozen_profile_sets = [
        contract.get("profiles"),
        execution_contract.get("profiles"),
        mutants.get("mutation_protocol", {}).get("profiles_per_mutant"),
    ]
    check(all(profile_set == PROFILES for profile_set in frozen_profile_sets), "PROFILES", failures)

    cells: list[dict[str, Any]] = []
    for record in positive_records:
        for profile in PROFILES:
            cells.append({**record, "profile": profile, "kind": "positive"})
    for record in opaque_records:
        cells.append({**record, "profile": "opaque", "kind": "opaque"})
    for record in mutation_records:
        for profile in PROFILES:
            cells.append({**record, "profile": profile, "kind": "mutant"})

    profile_order = {profile: index for index, profile in enumerate(PROFILES)}
    profile_order["opaque"] = len(PROFILES)
    kind_order = {"positive": 0, "opaque": 1, "mutant": 2}

    def canonicalize(items: list[dict[str, Any]]) -> list[dict[str, Any]]:
        return sorted(
            items,
            key=lambda item: (
                kind_order[item["kind"]],
                item["id"],
                profile_order[item["profile"]],
            ),
        )

    order_inputs = {"canonical": list(cells), "reverse": list(reversed(cells))}
    determinism_hashes: dict[str, str] = {}
    for repetition in (1, 2):
        for order_name, order_input in order_inputs.items():
            key = f"repetition_{repetition}_{order_name}"
            determinism_hashes[key] = digest(canonical_bytes(canonicalize(order_input)))
    check(len(set(determinism_hashes.values())) == 1, "NONDETERMINISTIC_REORDER", failures)

    result = {
        "schema": "p1359-safety-gate-result-v1",
        "verdict": "PASS" if not failures else "FAIL",
        "phase": "pre_execution",
        "claim_boundary": "P1359 static safety/discrimination gate only",
        "attestation": "executado sem atestação de isolamento",
        "hashes": actual_hashes,
        "isomorphism": {
            "families": FAMILIES,
            "positive_oracles": len(positive_records),
            "positive_profile_cells": len(positive_records) * len(PROFILES),
            "status": "PASS" if not failures else "FAIL",
        },
        "execution_safety": {
            "v05_v07_observer": "Args::try_parse_from in_process_parser_only",
            "v05_v07_subprocess_forbidden": True,
            "process_matrix_commands": ["compile"],
            "a14": "Violated",
            "a14_rejection_phase": "pre_execution",
            "a14_process_spawned": False,
            "product_processes_executed": 0,
        },
        "opaque": {
            "count": len(opaque_records),
            "classifications": opaque_records,
            "parity_credit": 0,
        },
        "mutations": {
            "valid": 14,
            "rejected": rejected,
            "score": mutation_score,
            "classifications": mutation_records,
        },
        "determinism": {
            "method": "pure_in_memory_canonicalization_of_canonical_and_reverse_orders",
            "repetitions": 2,
            "orders": ["canonical", "reverse"],
            "cell_count": len(cells),
            "hashes": determinism_hashes,
            "status": "PASS" if len(set(determinism_hashes.values())) == 1 else "FAIL",
        },
        "failures": sorted(failures),
        "candidate_implementation_read": False,
        "candidate_product_code_read": False,
        "candidate_tests_read": False,
        "candidate_binary_read_or_executed": False,
        "product_processes_executed": 0,
    }
    sys.stdout.buffer.write(canonical_bytes(result))
    return 0 if not failures else 1


if __name__ == "__main__":
    raise SystemExit(main())
