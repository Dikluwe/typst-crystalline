#!/usr/bin/env python3
"""P1301 binding-free contract and mutation discriminator.

Reads only the frozen public artifacts, L0 hashes, and vanilla binary hash. It
does not read or execute candidate code/tests and does not write repository
files. The JSON receipt is emitted on stdout; a failing gate returns 1.
"""

from __future__ import annotations

import hashlib
import json
import os
import sys
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
PATHS = {
    "manifest": ROOT / "00_nucleo/diagnosticos/p1301-manifest.json",
    "field_access_prompt": ROOT / "00_nucleo/prompts/compiler/eval/bindings/field_access.md",
    "tests_prompt": ROOT / "00_nucleo/prompts/compiler/eval/tests.md",
    "contract": ROOT / "00_nucleo/diagnosticos/p1301-contract.json",
    "oracle_suite": ROOT / "00_nucleo/diagnosticos/p1301-oracle-suite.json",
    "mutants": ROOT / "00_nucleo/diagnosticos/p1301-mutants.json",
    "vanilla": Path("/usr/local/bin/typst"),
}
EXPECTED_HASHES = {
    "manifest": "074fabeda17f741f064fe1d561ad6a78ad81e45654861825586664a9646f05a8",
    "field_access_prompt": "38d6f5cdee302491ec059577174d4f6dc6d3c28e3d2da2748f61c05853cc0c16",
    "tests_prompt": "a4ade7eda600891465c62c320d80b7c2182c30dbb5f456de210c4a823b3e609a",
    "contract": "2d8d7a141d63c13473681abdefa24ed9370e03925824a9cae74ca26305ea85f5",
    "oracle_suite": "4182575a0c11e39aed01f05c827d3cb59522f69ab1795f017523586f4c19d012",
    "mutants": "ee3c6c4d19e410850e56e26a984a8d53662056f8a4cba39da746fdb56c2ed18e",
    "vanilla": "7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8",
}
OBSERVABLES = (
    "exit_code",
    "stdout",
    "diagnostic_class",
    "message",
    "hints",
    "span_bytes",
    "rendered_stderr",
    "public_kind",
)
SENTINEL_ID = "S-NONMODULE-dictionary-whole-span"
OPAQUE_EXPECTED = {
    "O-AMBIGUOUS-PRIMARY": "AMBIGUOUS_PRIMARY_DIAGNOSTIC",
    "O-UNMAPPABLE-SPAN": "SPAN_CANNOT_BE_MAPPED_WITH_PUBLIC_INPUT",
}


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def hash_gate(label: str) -> dict[str, Any]:
    actual = {name: sha256(path) for name, path in PATHS.items()}
    mismatches = {
        name: {"expected": EXPECTED_HASHES[name], "actual": value}
        for name, value in actual.items()
        if value != EXPECTED_HASHES[name]
    }
    return {
        "label": label,
        "at": utc_now(),
        "actual_sha256": actual,
        "mismatches": mismatches,
        "pass": not mismatches,
    }


def load_json(name: str) -> dict[str, Any]:
    with PATHS[name].open("r", encoding="utf-8") as stream:
        return json.load(stream)


def envelope(value: dict[str, Any]) -> dict[str, Any]:
    normalized = dict(value)
    if "stderr" in normalized and "rendered_stderr" not in normalized:
        normalized["rendered_stderr"] = normalized.pop("stderr")
    return {key: normalized[key] for key in OBSERVABLES if key in normalized}


def classify(expected: dict[str, Any], actual: Any) -> dict[str, Any]:
    if not isinstance(actual, dict):
        return {"verdict": "Unknown", "reason_code": "PUBLIC_ENVELOPE_UNAVAILABLE"}
    wanted = envelope(expected)
    observed = envelope(actual)
    for key in OBSERVABLES:
        if key not in wanted:
            continue
        if key not in observed or wanted[key] != observed[key]:
            return {
                "verdict": "Violated",
                "first_difference": {
                    "observable": key,
                    "expected": wanted[key],
                    "actual": observed.get(key, "<missing>"),
                },
            }
    return {"verdict": "Preserved"}


def counts(results: dict[str, dict[str, Any]]) -> dict[str, int]:
    return {
        verdict: sum(item["verdict"] == verdict for item in results.values())
        for verdict in ("Preserved", "Violated", "Unknown")
    }


def main() -> int:
    started_at = utc_now()
    start_ns = time.monotonic_ns()
    initial_hash_gate = hash_gate("before-discrimination")
    if not initial_hash_gate["pass"]:
        receipt = {
            "schema_version": "tekt-discrimination-receipt-v1",
            "step": "P1301",
            "status": "BLOCKED_FROZEN_INPUT_DRIFT",
            "seal_eligible": False,
            "hash_gates": [initial_hash_gate],
            "attestation": {
                "technical_isolation_attested": False,
                "language": "executado sem atestação de isolamento técnico",
            },
        }
        print(json.dumps(receipt, indent=2, ensure_ascii=False, sort_keys=True))
        return 1

    contract = load_json("contract")
    oracle = load_json("oracle_suite")
    mutant_spec = load_json("mutants")

    failures: list[str] = []
    required = {case["id"]: case for case in contract["required_cases"]}
    oracle_cases = {case["id"]: case for case in oracle["required_cases"]}
    opaque = {case["id"]: case for case in contract["deliberately_opaque_cases"]}
    normal = contract["order_invariance"]["normal"]
    inverted = contract["order_invariance"]["inverted"]
    orders = {"normal": normal, "inverted": inverted}

    if len(required) != 23 or set(normal) != set(required) or inverted != list(reversed(normal)):
        failures.append("required case/order partition is not the frozen 23-case reversible vector")
    if set(oracle_cases) != set(required):
        failures.append("oracle and contract case-id sets differ")

    vanilla_ids = [case_id for case_id in normal if case_id != SENTINEL_ID]
    if len(vanilla_ids) != 22:
        failures.append("vanilla-oracle partition is not exactly 22 cases")

    required_trials: dict[str, Any] = {}
    oracle_trials: dict[str, Any] = {}
    opaque_trials: dict[str, Any] = {}
    required_maps: dict[str, Any] = {}
    oracle_maps: dict[str, Any] = {}
    opaque_maps: dict[str, Any] = {}

    for order_name, order in orders.items():
        required_results = {
            case_id: classify(required[case_id]["expected"], required[case_id]["expected"])
            for case_id in order
        }
        oracle_results = {
            case_id: classify(required[case_id]["expected"], oracle_cases[case_id]["actual"])
            for case_id in order
            if case_id != SENTINEL_ID
        }
        opaque_results = {
            case_id: {
                "verdict": "Unknown",
                "reason_code": opaque[case_id]["reason_code"],
            }
            for case_id in (OPAQUE_EXPECTED if order_name == "normal" else reversed(OPAQUE_EXPECTED))
        }
        required_maps[order_name] = required_results
        oracle_maps[order_name] = oracle_results
        opaque_maps[order_name] = opaque_results
        required_trials[order_name] = {"counts": counts(required_results)}
        oracle_trials[order_name] = {"counts": counts(oracle_results)}
        opaque_trials[order_name] = {"counts": counts(opaque_results)}

    sentinel_contract_span = required[SENTINEL_ID]["expected"].get("span_bytes")
    sentinel_oracle = oracle_cases[SENTINEL_ID]
    sentinel_partition = {
        "case_id": SENTINEL_ID,
        "oracle_source": sentinel_oracle.get("oracle_source"),
        "candidate_gate_expected_span_bytes": sentinel_contract_span,
        "vanilla_informative_span_bytes": sentinel_oracle["actual"].get("span_bytes"),
        "vanilla_informative_verdict": classify(
            required[SENTINEL_ID]["expected"], sentinel_oracle["actual"]
        )["verdict"],
        "included_in_candidate_gate": SENTINEL_ID in required,
        "included_in_vanilla_preserved_partition": SENTINEL_ID in vanilla_ids,
    }
    if sentinel_partition != {
        "case_id": SENTINEL_ID,
        "oracle_source": "contract_pre_candidate_invariant",
        "candidate_gate_expected_span_bytes": [10, 21],
        "vanilla_informative_span_bytes": [17, 21],
        "vanilla_informative_verdict": "Violated",
        "included_in_candidate_gate": True,
        "included_in_vanilla_preserved_partition": False,
    }:
        failures.append("non-Module sentinel partition or spans drifted")

    for order_name in orders:
        if required_trials[order_name]["counts"] != {"Preserved": 23, "Violated": 0, "Unknown": 0}:
            failures.append(f"candidate contract template failed in {order_name} order")
        if oracle_trials[order_name]["counts"] != {"Preserved": 22, "Violated": 0, "Unknown": 0}:
            failures.append(f"22-case vanilla oracle partition failed in {order_name} order")
        if opaque_trials[order_name]["counts"] != {"Preserved": 0, "Violated": 0, "Unknown": 2}:
            failures.append(f"opaque controls failed in {order_name} order")
        for case_id, reason in OPAQUE_EXPECTED.items():
            if opaque_maps[order_name].get(case_id) != {"verdict": "Unknown", "reason_code": reason}:
                failures.append(f"opaque reason mismatch for {case_id} in {order_name} order")

    mutant_results: dict[str, Any] = {}
    valid_mutants = [mutant for mutant in mutant_spec["mutants"] if mutant.get("valid") is True]
    if len(valid_mutants) != 12:
        failures.append("valid mutant count is not 12")

    for mutant in valid_mutants:
        witness_by_id = {witness["case_id"]: witness for witness in mutant["witnesses"]}
        per_order: dict[str, Any] = {}
        verdict_maps: dict[str, Any] = {}
        observable_maps: dict[str, Any] = {}
        for order_name, order in orders.items():
            verdicts: dict[str, Any] = {}
            observations: dict[str, Any] = {}
            for case_id in order:
                witness = witness_by_id.get(case_id)
                actual = witness["mutated_envelope"] if witness else required[case_id]["expected"]
                observations[case_id] = envelope(actual)
                verdicts[case_id] = classify(required[case_id]["expected"], actual)
            verdict_maps[order_name] = verdicts
            observable_maps[order_name] = observations
            killed_by = [case_id for case_id, result in verdicts.items() if result["verdict"] == "Violated"]
            unknown = [case_id for case_id, result in verdicts.items() if result["verdict"] == "Unknown"]
            per_order[order_name] = {
                "counts": counts(verdicts),
                "killed": bool(killed_by) and not unknown,
                "killed_by": killed_by,
                "unknown_case_ids": unknown,
            }
            if not per_order[order_name]["killed"]:
                failures.append(f"mutant {mutant['id']} survived or became Unknown in {order_name} order")
            if set(killed_by) != set(witness_by_id):
                failures.append(f"mutant {mutant['id']} witness classification mismatch in {order_name} order")
        invariant = (
            verdict_maps["normal"] == verdict_maps["inverted"]
            and observable_maps["normal"] == observable_maps["inverted"]
        )
        if not invariant:
            failures.append(f"mutant {mutant['id']} is order-dependent")
        mutant_results[mutant["id"]] = {
            "valid": True,
            "fault_class": mutant["fault_class"],
            "normal": per_order["normal"],
            "inverted": per_order["inverted"],
            "order_invariant": invariant,
        }

    killed = sum(
        item["normal"]["killed"] and item["inverted"]["killed"]
        for item in mutant_results.values()
    )
    survivors = [
        mutant_id
        for mutant_id, item in mutant_results.items()
        if not (item["normal"]["killed"] and item["inverted"]["killed"])
    ]
    mutation_score = killed / len(valid_mutants) if valid_mutants else 0.0
    if mutation_score != 1.0 or survivors:
        failures.append("mutation score is below 1.0 or survivors are present")

    order_invariance = {
        "candidate_contract_verdict_maps_identical": required_maps["normal"] == required_maps["inverted"],
        "vanilla_oracle_verdict_maps_identical": oracle_maps["normal"] == oracle_maps["inverted"],
        "opaque_verdict_maps_identical": opaque_maps["normal"] == opaque_maps["inverted"],
        "all_mutants_identical": all(item["order_invariant"] for item in mutant_results.values()),
    }
    if not all(order_invariance.values()):
        failures.append("one or more normal/inverted maps differ")

    final_hash_gate = hash_gate("after-discrimination")
    if not final_hash_gate["pass"]:
        failures.append("frozen input drift after discrimination")

    end_ns = time.monotonic_ns()
    finished_at = utc_now()
    receipt = {
        "schema_version": "tekt-discrimination-receipt-v1",
        "step": "P1301",
        "status": "PASS" if not failures else "BLOCKED_GATE_FAILURE",
        "seal_eligible": not failures,
        "failures": failures,
        "attestation": {
            "technical_isolation_attested": False,
            "language": "executado sem atestação de isolamento técnico",
            "limitation": "logical role separation only; shared filesystem and process environment were not technically isolated",
        },
        "runner": {
            "path": str(Path(__file__).resolve().relative_to(ROOT)),
            "sha256": sha256(Path(__file__).resolve()),
            "pid": os.getpid(),
            "started_at": started_at,
            "finished_at": finished_at,
            "duration_ns": end_ns - start_ns,
            "process_model": "one Python runner process; zero child processes; frozen public envelopes only",
            "environment": {"python": sys.version.split()[0]},
        },
        "hash_gates": [initial_hash_gate, final_hash_gate],
        "partitions": {
            "vanilla_oracle": {
                "case_count": 22,
                "case_ids": vanilla_ids,
                "trials": oracle_trials,
            },
            "contract_candidate_gate_template": {
                "case_count": 23,
                "includes_non_module_sentinel": True,
                "trials": required_trials,
            },
            "non_module_sentinel": sentinel_partition,
            "opaque_controls": {
                "case_count": 2,
                "allowed_unknown_case_ids": list(OPAQUE_EXPECTED),
                "trials": opaque_trials,
            },
        },
        "mutation": {
            "valid_mutants": len(valid_mutants),
            "killed_mutants": killed,
            "survivors": survivors,
            "mutation_score": mutation_score,
            "required_score": 1.0,
            "unknown_mutant_case_count": sum(
                len(item[order]["unknown_case_ids"])
                for item in mutant_results.values()
                for order in ("normal", "inverted")
            ),
            "results": mutant_results,
        },
        "order_invariance": order_invariance,
        "unknown_accounting": {
            "required_or_oracle_or_mutant": 0,
            "opaque_expected": 4,
            "opaque_expected_basis": "two declared opaque cases in each of two orders",
            "unexpected": 0,
        },
        "cost": {
            "runner_processes": 1,
            "child_processes": 0,
            "vanilla_executions": 0,
            "candidate_executions": 0,
            "candidate_files_read": 0,
            "classification_evaluations": 646,
            "breakdown": {
                "vanilla_oracle": 44,
                "candidate_contract_template": 46,
                "opaque": 4,
                "mutants_full_corpus": 552,
            },
            "full_corpus_mutant_trials": 24,
        },
        "scope_guards": {
            "binding_free": True,
            "candidate_implementation_read": False,
            "candidate_tests_read": False,
            "repr_std_executed": False,
            "solution_implemented": False,
        },
    }
    print(json.dumps(receipt, indent=2, ensure_ascii=False, sort_keys=True))
    return 0 if not failures else 1


if __name__ == "__main__":
    raise SystemExit(main())
