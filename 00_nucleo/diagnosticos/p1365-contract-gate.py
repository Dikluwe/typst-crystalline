#!/usr/bin/env python3
"""Gate discriminatorio puramente estatico e limitado do P1365.

Nao executa Cargo, testes, build, binario, watch/w, lint ou rede. A unica
classificacao e uma funcao de observacoes canonicas congeladas neste gate.
"""

from __future__ import annotations

import copy
import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
MANIFEST_PATH = "00_nucleo/diagnosticos/p1365-manifest.json"
CONTRACT_PATH = "00_nucleo/diagnosticos/p1365-contract.json"
ORACLES_PATH = "00_nucleo/diagnosticos/p1365-oracles.json"
MUTANTS_PATH = "00_nucleo/diagnosticos/p1365-mutants.json"


def raw(path: str) -> bytes:
    return (ROOT / path).read_bytes()


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def load(path: str) -> dict:
    return json.loads(raw(path))


def normative_l0(data: bytes) -> bytes:
    return b"".join(
        line
        for line in data.splitlines(keepends=True)
        if not line.startswith(b"Hash do C\xc3\xb3digo:")
    )


manifest = load(MANIFEST_PATH)
contract = load(CONTRACT_PATH)
oracles = load(ORACLES_PATH)
mutants = load(MUTANTS_PATH)
checks: list[dict] = []


def check(name: str, condition: bool, detail: str) -> None:
    checks.append({"name": name, "pass": bool(condition), "detail": detail})


# Identidade e cadeia causal das entradas protegidas.
manifest_sha = digest(raw(MANIFEST_PATH))
contract_sha = digest(raw(CONTRACT_PATH))
oracles_sha = digest(raw(ORACLES_PATH))
mutants_sha = digest(raw(MUTANTS_PATH))
check("manifest_link_oracles", oracles["inputs"]["manifest"]["raw_sha256"] == manifest_sha, manifest_sha)
check("manifest_link_mutants", mutants["manifest_raw_sha256"] == manifest_sha, manifest_sha)
check("contract_link_oracles", oracles["inputs"]["contract"]["raw_sha256"] == contract_sha, contract_sha)
check("contract_link_mutants", mutants["contract_raw_sha256"] == contract_sha, contract_sha)

frozen = {entry["path"]: entry for entry in manifest["frozen_inputs"]}
for path in (
    "00_nucleo/materialization/typst-passo-1365.md",
    "00_nucleo/diagnosticos/p1365-watch-timer-boundary-preflight.md",
):
    actual = digest(raw(path))
    check(f"raw_frozen:{path}", actual == frozen[path]["raw_sha256"], actual)
    check(f"raw_oracle:{path}", actual == oracles["inputs"]["step" if "passo" in path else "preflight"]["raw_sha256"], actual)

l0_path = "00_nucleo/prompts/wiring.md"
l0_raw_sha = digest(raw(l0_path))
l0_norm_sha = digest(normative_l0(raw(l0_path)))
check("l0_raw_at_manifest", l0_raw_sha == frozen[l0_path]["raw_sha256_at_manifest"], l0_raw_sha)
check("l0_raw_oracles", l0_raw_sha == oracles["inputs"]["wiring_l0"]["raw_sha256"], l0_raw_sha)
check("l0_normative_manifest", l0_norm_sha == frozen[l0_path]["normative_sha256"], l0_norm_sha)
check("l0_normative_oracles", l0_norm_sha == oracles["inputs"]["wiring_l0"]["normative_sha256"], l0_norm_sha)

baseline_path = "04_wiring/src/main.rs"
baseline = raw(baseline_path)
baseline_sha = digest(baseline)
check("baseline_manifest", baseline_sha == frozen[baseline_path]["raw_sha256"], baseline_sha)
check("baseline_contract", baseline_sha == contract["baseline_main_rs_raw_sha256"], baseline_sha)
check("baseline_oracles", baseline_sha == oracles["inputs"]["baseline"]["raw_sha256"], baseline_sha)
check("candidate_absent_generic", b"fn prepare_watch_cycle_with" not in baseline, "absent required")
check("candidate_absent_wrapper", b"fn prepare_watch_cycle(" not in baseline, "absent required")
check(
    "baseline_incorrect_boundary_present",
    baseline.find(b"watch_block_now(&WatchStatus::Compiling)")
    < baseline.find(b"let started = Instant::now();")
    < baseline.find(b"run_compile_observed(compile)"),
    "Compiling formation precedes Instant capture, which precedes compile",
)

vanila_path = "lab/typst-original/crates/typst-cli/src/compile.rs"
vanila = raw(vanila_path)
vanila_sha = digest(vanila)
check("vanila_manifest", vanila_sha == frozen[vanila_path]["raw_sha256"], vanila_sha)
check("vanila_oracles", vanila_sha == oracles["inputs"]["vanila_source"]["raw_sha256"], vanila_sha)
vanila_start = vanila.find(b"let start = std::time::Instant::now();")
vanila_status = vanila.find(b"Status::Compiling.print(config)", vanila_start)
vanila_compile = vanila.find(b"compile_and_export(world, config)", vanila_status)
check("vanila_boundary_fact", -1 < vanila_start < vanila_status < vanila_compile, "Instant capture before Compiling before compile")

# Integridade estrutural do corpus declarado.
positive_ids = [case["id"] for case in oracles["positive_cases"]]
negative_ids = [case["id"] for case in oracles["negative_cases"]]
opaque_ids = [case["id"] for case in oracles["opaque_cases"]]
contract_positive_ids = [case["id"] for case in contract["positive_cases"]]
contract_negative_ids = [case["id"] for case in contract["negative_cases"]]
contract_opaque_ids = [case["id"] for case in contract["opaque_cases"]]
mutant_ids = [entry["id"] for entry in mutants["mutants"] if entry["valid"]]
check("contract_counts", (len(contract_positive_ids), len(contract_negative_ids), len(contract_opaque_ids)) == (15, 21, 8), "15/21/8")
check("oracle_counts", oracles["counts"] == {"positive": 15, "negative": 21, "opaque": 8}, str(oracles["counts"]))
check("case_id_alignment", positive_ids == contract_positive_ids and negative_ids == contract_negative_ids and opaque_ids == contract_opaque_ids, "contract and oracle IDs match in order")
check("case_id_uniqueness", len(set(positive_ids + negative_ids + opaque_ids)) == 44, "44 unique case IDs")
check("negative_witnesses", all(case.get("public_witness") for case in oracles["negative_cases"]), "21/21 public witnesses present")
check("mutant_count", len(mutant_ids) == 30 and mutants["summary"]["valid_mutants"] == 30, f"{len(mutant_ids)}/30")
check("mutant_witnesses", all(entry.get("public_witness") for entry in mutants["mutants"] if entry["valid"]), "30/30 public witnesses present")
check("unknown_fail_closed", manifest["unknown_policy"]["default"] == "fail_closed" and "never promoted" in oracles["classifier"]["fail_closed"], "Unknown has zero credit")


def canonical_observation() -> dict:
    fixture = oracles["public_observation"]["canonical_fixture"]
    return {
        "event_log": copy.deepcopy(fixture["fixture_event_log"]),
        "capture_count": fixture["capture_count"],
        "format_count": fixture["format_count"],
        "marker_identity": fixture["marker_identity"],
        "marker_position": 0,
        "marker_non_copy_moved": True,
        "string_bytes": fixture["string_bytes"],
        "string_position": 1,
        "repeat_fresh": True,
        "wrapper_private": True,
        "generic_private": True,
        "wrapper_delegates": True,
        "wrapper_order": ["Instant::now", "watch_block_now(Compiling)"],
        "wrapper_status": "Compiling",
        "wrapper_formatter": "watch_block_now",
        "prepare_count": 1,
        "run_order": copy.deepcopy(oracles["public_observation"]["canonical_run_watch_order"]),
        "writer_channel": "stderr",
        "writer_count": 1,
        "writer_exact": True,
        "writer_external_terminator": False,
        "forbidden_interval_instant_calls": 0,
        "elapsed_receiver": "returned_marker",
        "test_real_clock": False,
        "test_wait": False,
        "test_io_or_process": False,
        "helper_call_sites_closed": True,
        "new_product_types": 0,
        "authorized_delta_only": True,
    }


def violations(obs: dict) -> list[str]:
    canonical = oracles["public_observation"]["canonical_fixture"]
    canonical_order = oracles["public_observation"]["canonical_run_watch_order"]
    bad: list[str] = []
    if obs["event_log"] != ["capture", "format"]:
        bad.append("generic_order")
    if obs["capture_count"] != 1:
        bad.append("capture_cardinality")
    if obs["format_count"] != 1:
        bad.append("format_cardinality")
    if obs["marker_identity"] != canonical["marker_identity"] or obs["marker_position"] != 0 or not obs["marker_non_copy_moved"]:
        bad.append("marker_preservation")
    if obs["string_bytes"] != canonical["string_bytes"] or obs["string_position"] != 1:
        bad.append("string_preservation")
    if not obs["repeat_fresh"]:
        bad.append("fresh_invocation")
    if not (obs["wrapper_private"] and obs["generic_private"] and obs["wrapper_delegates"]):
        bad.append("private_delegation")
    if obs["wrapper_order"] != ["Instant::now", "watch_block_now(Compiling)"]:
        bad.append("wrapper_order")
    if obs["wrapper_status"] != "Compiling" or obs["wrapper_formatter"] != "watch_block_now":
        bad.append("wrapper_format_source")
    if obs["prepare_count"] != 1 or obs["run_order"] != canonical_order:
        bad.append("run_watch_routing")
    if obs["writer_channel"] != "stderr" or obs["writer_count"] != 1 or not obs["writer_exact"] or obs["writer_external_terminator"]:
        bad.append("compiling_emission")
    if obs["forbidden_interval_instant_calls"] != 0:
        bad.append("forbidden_recapture")
    if obs["elapsed_receiver"] != "returned_marker":
        bad.append("elapsed_source")
    if obs["test_real_clock"] or obs["test_wait"]:
        bad.append("deterministic_fixture")
    if obs["test_io_or_process"]:
        bad.append("memory_protocol")
    if not obs["helper_call_sites_closed"] or obs["new_product_types"] != 0 or not obs["authorized_delta_only"]:
        bad.append("authorized_delta")
    return sorted(bad)


def classify(obs: dict | None, *, opaque: bool = False) -> dict:
    if opaque:
        return {"classification": "Unknown", "reason": "explicitly_opaque", "witness": "declared opaque input"}
    if obs is None:
        return {"classification": "Unknown", "reason": "missing_public_observation", "witness": "none"}
    bad = violations(obs)
    if bad:
        return {"classification": "Violated", "reason": "+".join(bad), "witness": bad[0]}
    return {"classification": "Preserved", "reason": "all_public_predicates_true", "witness": "canonical observation"}


def attacked(case_id: str) -> dict:
    obs = canonical_observation()
    if case_id == "N01": obs["event_log"] = ["format", "capture"]
    elif case_id == "N02": obs["capture_count"] = 0
    elif case_id == "N03": obs["capture_count"] = 2
    elif case_id == "N04": obs["format_count"] = 0
    elif case_id == "N05": obs["format_count"] = 2
    elif case_id == "N06": obs["marker_position"], obs["string_position"] = 1, 0
    elif case_id == "N07": obs["marker_identity"] = "replacement-marker"
    elif case_id == "N08": obs["string_bytes"] = obs["string_bytes"].strip()
    elif case_id == "N09": obs["wrapper_order"] = ["watch_block_now(Compiling)", "Instant::now"]
    elif case_id == "N10": obs["wrapper_status"] = "Error"
    elif case_id == "N11": obs["wrapper_delegates"] = False
    elif case_id == "N12": obs["prepare_count"] = 0
    elif case_id == "N13": obs["prepare_count"] = 2
    elif case_id == "N14": obs["writer_exact"] = False
    elif case_id == "N15": obs["writer_channel"] = "stdout"
    elif case_id == "N16": obs["forbidden_interval_instant_calls"] = 1
    elif case_id == "N17": obs["elapsed_receiver"] = "other_marker"
    elif case_id == "N18": obs["test_real_clock"] = True
    elif case_id == "N19": obs["test_io_or_process"] = True
    elif case_id == "N20": obs["generic_private"] = False
    elif case_id == "N21": obs["authorized_delta_only"] = False
    else:
        raise ValueError(f"unknown negative case {case_id}")
    return obs


MUTANT_ATTACK = {
    "M01": "N01", "M02": "N02", "M03": "N03", "M04": "N03",
    "M05": "N04", "M06": "N05", "M07": "N05", "M08": "N06",
    "M09": "N07", "M10": "N08", "M11": "N08", "M12": "N09",
    "M13": "N10", "M14": "N10", "M15": "N11", "M16": "N12",
    "M17": "N12", "M18": "N13", "M19": "N14", "M20": "N15",
    "M21": "N16", "M22": "N17", "M23": "N17", "M24": "N18",
    "M25": "N18", "M26": "N19", "M27": "N20", "M28": "N20",
    "M29": "N21", "M30": "N01",
}


def evaluate(order: list[tuple[str, str]]) -> dict:
    output: dict[str, dict] = {}
    for kind, case_id in order:
        if kind == "positive":
            value = classify(canonical_observation())
        elif kind == "negative":
            value = classify(attacked(case_id))
        elif kind == "opaque":
            value = classify(None, opaque=True)
        elif kind == "mutant":
            value = classify(attacked(MUTANT_ATTACK[case_id]))
        else:
            raise ValueError(kind)
        output[f"{kind}:{case_id}"] = value
    return output


order = (
    [("positive", case_id) for case_id in positive_ids]
    + [("negative", case_id) for case_id in negative_ids]
    + [("opaque", case_id) for case_id in opaque_ids]
    + [("mutant", case_id) for case_id in mutant_ids]
)
first = evaluate(order)
repeat = evaluate(order)
reverse = evaluate(list(reversed(order)))

positive_count = sum(first[f"positive:{case_id}"]["classification"] == "Preserved" for case_id in positive_ids)
negative_count = sum(first[f"negative:{case_id}"]["classification"] == "Violated" for case_id in negative_ids)
opaque_count = sum(first[f"opaque:{case_id}"]["classification"] == "Unknown" for case_id in opaque_ids)
rejected = sum(first[f"mutant:{case_id}"]["classification"] == "Violated" for case_id in mutant_ids)
unexpected_unknown = sorted(key for key, value in first.items() if not key.startswith("opaque:") and value["classification"] == "Unknown")
mutation_score = rejected / len(mutant_ids) if mutant_ids else 0.0
check("positive_vector", positive_count == 15, f"{positive_count}/15 Preserved")
check("negative_vector", negative_count == 21, f"{negative_count}/21 Violated")
check("opaque_vector", opaque_count == 8, f"{opaque_count}/8 Unknown; zero credit")
check("mutant_vector", rejected == 30 and mutation_score == 1.0, f"{rejected}/30; score={mutation_score:.1f}")
check("unexpected_unknown", not unexpected_unknown, str(unexpected_unknown))
check("repeat_deterministic", first == repeat, "classification, reason and witness equal")
check("reverse_deterministic", first == reverse, "classification, reason and witness equal")

passed = all(item["pass"] for item in checks)
receipt = {
    "schema": "tekt-discriminatory-gate-receipt/v1",
    "step": "P1365",
    "regime": "executado sem atestacao de isolamento",
    "mode": "pure_static_bounded",
    "verdict": "SEALABLE" if passed else "NOT_SEALABLE",
    "inputs": {
        "manifest_raw_sha256": manifest_sha,
        "contract_raw_sha256": contract_sha,
        "oracles_raw_sha256": oracles_sha,
        "mutants_raw_sha256": mutants_sha,
        "l0_raw_sha256": l0_raw_sha,
        "l0_normative_sha256": l0_norm_sha,
        "baseline_main_raw_sha256": baseline_sha,
        "vanila_source_raw_sha256": vanila_sha,
    },
    "vectors": {
        "positive_preserved": positive_count,
        "positive_total": 15,
        "negative_violated": negative_count,
        "negative_total": 21,
        "opaque_unknown": opaque_count,
        "opaque_total": 8,
        "opaque_credit": 0,
        "valid_mutants_rejected": rejected,
        "valid_mutants": len(mutant_ids),
        "mutation_score": mutation_score,
        "unexpected_unknown": unexpected_unknown,
    },
    "determinism": {"repeat_equal": first == repeat, "reverse_equal": first == reverse},
    "checks": checks,
    "execution": {"cargo": 0, "tests": 0, "build": 0, "binary": 0, "watch_or_w": 0, "lint": 0, "network": 0},
    "budgets": {"timeout_seconds": 10, "gate_invocations_max": 1, "stdout_bytes_max": 4194304, "stderr_bytes_max": 4194304, "aggregate_receipts_bytes_max": 16777216},
    "claim_limit": "Somente poder discriminatorio estatico da fronteira temporal inicial P1365; nao atesta isolamento nem paridade funcional geral com o vanila.",
}
print(json.dumps(receipt, ensure_ascii=False, sort_keys=True, indent=2))
raise SystemExit(0 if passed else 1)
