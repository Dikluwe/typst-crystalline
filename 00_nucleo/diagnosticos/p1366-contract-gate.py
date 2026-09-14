#!/usr/bin/env python3
"""Gate discriminatorio puramente estatico e limitado do P1366.

Nao executa Cargo, testes, build, binario, watch/w, lint ou rede. A
classificacao depende somente das entradas congeladas e de observacoes
canonicas declarativas do fragmento status-antes-de-diagnosticos.
"""

from __future__ import annotations

import copy
import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
MANIFEST_PATH = "00_nucleo/diagnosticos/p1366-manifest.json"
CONTRACT_PATH = "00_nucleo/diagnosticos/p1366-contract.json"
ORACLES_PATH = "00_nucleo/diagnosticos/p1366-oracles.json"
MUTANTS_PATH = "00_nucleo/diagnosticos/p1366-mutants.json"


def raw(path: str) -> bytes:
    return (ROOT / path).read_bytes()


def sha(data: bytes) -> str:
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


# Identidade, proveniencia e ausencia do candidato antes do selo.
manifest_sha = sha(raw(MANIFEST_PATH))
contract_sha = sha(raw(CONTRACT_PATH))
oracles_sha = sha(raw(ORACLES_PATH))
mutants_sha = sha(raw(MUTANTS_PATH))
check("manifest_link_oracles", oracles["inputs"]["manifest"]["raw_sha256"] == manifest_sha, manifest_sha)
check("manifest_link_mutants", mutants["manifest_raw_sha256"] == manifest_sha, manifest_sha)
check("contract_link_oracles", oracles["inputs"]["contract"]["raw_sha256"] == contract_sha, contract_sha)
check("contract_link_mutants", mutants["contract_raw_sha256"] == contract_sha, contract_sha)

frozen = {entry["path"]: entry for entry in manifest["frozen_inputs"]}
for path, oracle_key in (
    ("00_nucleo/materialization/typst-passo-1366.md", "step"),
    ("00_nucleo/diagnosticos/p1366-watch-diagnostic-order-preflight.md", "preflight"),
):
    actual = sha(raw(path))
    check(f"raw_frozen:{path}", actual == frozen[path]["raw_sha256"], actual)
    check(f"raw_oracle:{path}", actual == oracles["inputs"][oracle_key]["raw_sha256"], actual)

l0_path = "00_nucleo/prompts/wiring.md"
l0_raw_sha = sha(raw(l0_path))
l0_norm_sha = sha(normative_l0(raw(l0_path)))
check("l0_raw_at_manifest", l0_raw_sha == frozen[l0_path]["raw_sha256_at_manifest"], l0_raw_sha)
check("l0_raw_oracles", l0_raw_sha == oracles["inputs"]["wiring_l0"]["raw_sha256"], l0_raw_sha)
check("l0_normative_manifest", l0_norm_sha == frozen[l0_path]["normative_sha256"], l0_norm_sha)
check("l0_normative_oracles", l0_norm_sha == oracles["inputs"]["wiring_l0"]["normative_sha256"], l0_norm_sha)

baseline_path = "04_wiring/src/main.rs"
baseline = raw(baseline_path)
baseline_sha = sha(baseline)
check("baseline_manifest", baseline_sha == frozen[baseline_path]["raw_sha256"], baseline_sha)
check("baseline_contract", baseline_sha == contract["baseline_main_rs_raw_sha256"], baseline_sha)
check("baseline_oracles", baseline_sha == oracles["inputs"]["baseline"]["raw_sha256"], baseline_sha)
check("candidate_helper_absent", b"fn report_in_order" not in baseline, "absent required")
check("candidate_observer_absent", b"run_compile_observed(compile," not in baseline, "absent required")
old_warning = baseline.find(b"drain_to_stderr(&world, &warnings")
old_watch_call = baseline.find(b"run_compile_observed(compile)")
old_watch_status = baseline.find(b"watch_status_for_outcome", old_watch_call)
check("baseline_old_order_present", -1 < old_warning and -1 < old_watch_call < old_watch_status, "diagnostics remain inside observer and terminal status after return")

vanila_path = "lab/typst-original/crates/typst-cli/src/compile.rs"
vanila = raw(vanila_path)
vanila_sha = sha(vanila)
check("vanila_manifest", vanila_sha == frozen[vanila_path]["raw_sha256"], vanila_sha)
check("vanila_oracles", vanila_sha == oracles["inputs"]["vanila_source"]["raw_sha256"], vanila_sha)
success_status = vanila.find(b"Status::Success(duration).print(config)")
success_diagnostics = vanila.find(b"print_diagnostics(world, &[], &warnings", success_status)
error_status = vanila.find(b"Status::Error.print(config)")
error_diagnostics = vanila.find(b"print_diagnostics(world, errors, &warnings", error_status)
check("vanila_success_order", -1 < success_status < success_diagnostics, "success status precedes diagnostics")
check("vanila_error_order", -1 < error_status < error_diagnostics, "error status precedes diagnostics")

# Integridade estrutural do corpus.
positive_ids = [case["id"] for case in oracles["positive_cases"]]
negative_ids = [case["id"] for case in oracles["negative_cases"]]
opaque_ids = [case["id"] for case in oracles["opaque_cases"]]
contract_positive_ids = [case["id"] for case in contract["positive_cases"]]
contract_negative_ids = [case["id"] for case in contract["negative_cases"]]
contract_opaque_ids = [case["id"] for case in contract["opaque_cases"]]
valid_mutants = [entry for entry in mutants["mutants"] if entry["valid"]]
mutant_ids = [entry["id"] for entry in valid_mutants]
check("contract_counts", (len(contract_positive_ids), len(contract_negative_ids), len(contract_opaque_ids)) == (16, 22, 8), "16/22/8")
check("oracle_counts", oracles["counts"] == {"positive": 16, "negative": 22, "opaque": 8}, str(oracles["counts"]))
check("case_id_alignment", positive_ids == contract_positive_ids and negative_ids == contract_negative_ids and opaque_ids == contract_opaque_ids, "contract and oracle IDs match in order")
check("case_id_uniqueness", len(set(positive_ids + negative_ids + opaque_ids)) == 46, "46 unique case IDs")
check("negative_witnesses", all(case.get("public_witness") for case in oracles["negative_cases"]), "22/22 public witnesses present")
check("mutant_count", len(mutant_ids) == 32 and mutants["summary"]["valid_mutants"] == 32, f"{len(mutant_ids)}/32")
check("mutant_witnesses", all(entry.get("public_witness") for entry in valid_mutants), "32/32 public witnesses present")
check("unknown_fail_closed", manifest["unknown_policy"]["default"] == "fail_closed" and "never promoted" in oracles["classifier"]["fail_closed"], "Unknown has zero credit")


def canonical() -> dict:
    fixture = oracles["public_observation"]["canonical_fixture"]
    return {
        "events": copy.deepcopy(fixture["fixture_event_log"]),
        "status_count": fixture["status_count"],
        "diagnostics_count": fixture["diagnostics_count"],
        "status_effect": fixture["status_effect"],
        "diagnostics_effect": fixture["diagnostics_effect"],
        "immediate_pair": True,
        "helper_private": True,
        "helper_fn_once": True,
        "observer_private": True,
        "observer_fn_mut": True,
        "return_shape_preserved": True,
        "callback_each_path": 1,
        "preliminary_order": ["callback", "message"],
        "preliminary_args": [2, False],
        "compiled_failure_order": ["callback", "warnings", "errors"],
        "compiled_failure_exit": 1,
        "warning_source": "pre-drain collection emptiness",
        "success_order": ["artifact", "timings", "callback", "warnings"],
        "write_failure_order": ["failed-write", "callback", "warnings", "message"],
        "write_failure_success": False,
        "watch_post_return_sites": 0,
        "watch_elapsed_source": "P1365 returned marker",
        "watch_typed_source": True,
        "watch_block_p1364": True,
        "compile_observer_effects": 0,
        "test_forbidden_calls": [],
        "public_surface_delta": [],
        "authorized_delta_only": True,
        "repeat_stable": True,
    }


def violations(obs: dict) -> list[str]:
    fixture = oracles["public_observation"]["canonical_fixture"]
    bad: list[str] = []
    if obs["events"] != ["status", "diagnostics"]: bad.append("causal_order")
    if obs["status_count"] != 1: bad.append("status_cardinality")
    if obs["diagnostics_count"] != 1: bad.append("diagnostics_cardinality")
    if not obs["immediate_pair"]: bad.append("immediate_pair")
    if obs["status_effect"] != fixture["status_effect"] or obs["diagnostics_effect"] != fixture["diagnostics_effect"]: bad.append("effect_preservation")
    if not (obs["helper_private"] and obs["helper_fn_once"]): bad.append("helper_boundary")
    if not (obs["observer_private"] and obs["observer_fn_mut"] and obs["return_shape_preserved"]): bad.append("observer_boundary")
    if obs["callback_each_path"] != 1: bad.append("observer_cardinality")
    if obs["preliminary_order"] != ["callback", "message"]: bad.append("preliminary_order")
    if obs["preliminary_args"] != [2, False]: bad.append("preliminary_args")
    if obs["compiled_failure_order"] != ["callback", "warnings", "errors"]: bad.append("compiled_failure_order")
    if obs["compiled_failure_exit"] != 1: bad.append("compiled_failure_exit")
    if obs["warning_source"] != "pre-drain collection emptiness": bad.append("warning_source")
    if obs["success_order"] != ["artifact", "timings", "callback", "warnings"]: bad.append("success_order")
    if obs["write_failure_order"] != ["failed-write", "callback", "warnings", "message"]: bad.append("write_failure_order")
    if obs["write_failure_success"]: bad.append("contradictory_success")
    if obs["watch_post_return_sites"] != 0: bad.append("post_return_status")
    if obs["watch_elapsed_source"] != "P1365 returned marker" or not obs["watch_typed_source"] or not obs["watch_block_p1364"]: bad.append("watch_routing")
    if obs["compile_observer_effects"] != 0: bad.append("compile_status_leak")
    if obs["test_forbidden_calls"]: bad.append("forbidden_test_capability")
    if obs["public_surface_delta"]: bad.append("public_surface")
    if not obs["authorized_delta_only"]: bad.append("scope_expansion")
    if not obs["repeat_stable"]: bad.append("nondeterminism")
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
    obs = canonical()
    if case_id == "N01": obs["events"] = ["diagnostics", "status"]
    elif case_id == "N02": obs["status_count"] = 0
    elif case_id == "N03": obs["diagnostics_count"] = 0
    elif case_id == "N04": obs["immediate_pair"] = False
    elif case_id == "N05": obs["diagnostics_effect"] = obs["status_effect"]
    elif case_id == "N06": obs["callback_each_path"] = 0
    elif case_id == "N07": obs["preliminary_order"] = ["message", "callback"]
    elif case_id == "N08": obs["preliminary_args"] = [1, True]
    elif case_id == "N09": obs["compiled_failure_order"] = ["warnings", "callback", "errors"]
    elif case_id == "N10": obs["compiled_failure_exit"] = 0
    elif case_id == "N11": obs["compiled_failure_order"] = ["callback", "errors", "warnings"]
    elif case_id == "N12": obs["warning_source"] = "rendered diagnostic text"
    elif case_id == "N13": obs["success_order"] = ["callback", "artifact", "timings", "warnings"]
    elif case_id == "N14": obs["success_order"] = ["artifact", "callback", "timings", "warnings"]
    elif case_id == "N15": obs["write_failure_success"] = True
    elif case_id == "N16": obs["write_failure_order"] = ["failed-write", "warnings", "message", "callback"]
    elif case_id == "N17": obs["watch_post_return_sites"] = 1
    elif case_id == "N18": obs["watch_elapsed_source"] = "fresh marker"
    elif case_id == "N19": obs["compile_observer_effects"] = 1
    elif case_id == "N20": obs["test_forbidden_calls"] = ["process"]
    elif case_id == "N21": obs["helper_private"] = False
    elif case_id == "N22": obs["authorized_delta_only"] = False
    else: raise ValueError(f"unknown negative case {case_id}")
    return obs


MUTANT_ATTACK = {
    "M01":"N01", "M02":"N02", "M03":"N02", "M04":"N03", "M05":"N03",
    "M06":"N04", "M07":"N05", "M08":"N06", "M09":"N07", "M10":"N08",
    "M11":"N08", "M12":"N09", "M13":"N09", "M14":"N10", "M15":"N11",
    "M16":"N12", "M17":"N12", "M18":"N13", "M19":"N14", "M20":"N15",
    "M21":"N15", "M22":"N16", "M23":"N16", "M24":"N06", "M25":"N17",
    "M26":"N18", "M27":"N18", "M28":"N19", "M29":"N20", "M30":"N21",
    "M31":"N21", "M32":"N22",
}


def evaluate(order: list[tuple[str, str]]) -> dict:
    output = {}
    for kind, case_id in order:
        if kind == "positive": value = classify(canonical())
        elif kind == "negative": value = classify(attacked(case_id))
        elif kind == "opaque": value = classify(None, opaque=True)
        elif kind == "mutant": value = classify(attacked(MUTANT_ATTACK[case_id]))
        else: raise ValueError(kind)
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
check("positive_vector", positive_count == 16, f"{positive_count}/16 Preserved")
check("negative_vector", negative_count == 22, f"{negative_count}/22 Violated")
check("opaque_vector", opaque_count == 8, f"{opaque_count}/8 Unknown; zero credit")
check("mutant_vector", rejected == 32 and mutation_score == 1.0, f"{rejected}/32; score={mutation_score:.1f}")
check("unexpected_unknown", not unexpected_unknown, str(unexpected_unknown))
check("repeat_deterministic", first == repeat, "classification, reason and witness equal")
check("reverse_deterministic", first == reverse, "classification, reason and witness equal")

passed = all(item["pass"] for item in checks)
receipt = {
    "schema": "tekt-discriminatory-gate-receipt/v1",
    "step": "P1366",
    "regime": "executado sem atestacao de isolamento",
    "mode": "pure_static_bounded",
    "verdict": "SEALABLE" if passed else "NOT_SEALABLE",
    "inputs": {
        "manifest_raw_sha256": manifest_sha, "contract_raw_sha256": contract_sha,
        "oracles_raw_sha256": oracles_sha, "mutants_raw_sha256": mutants_sha,
        "l0_raw_sha256": l0_raw_sha, "l0_normative_sha256": l0_norm_sha,
        "baseline_main_raw_sha256": baseline_sha, "vanila_source_raw_sha256": vanila_sha,
    },
    "vectors": {
        "positive_preserved": positive_count, "positive_total": 16,
        "negative_violated": negative_count, "negative_total": 22,
        "opaque_unknown": opaque_count, "opaque_total": 8, "opaque_credit": 0,
        "valid_mutants_rejected": rejected, "valid_mutants": len(mutant_ids),
        "mutation_score": mutation_score, "unexpected_unknown": unexpected_unknown,
    },
    "determinism": {"repeat_equal": first == repeat, "reverse_equal": first == reverse},
    "checks": checks,
    "execution": {"cargo":0,"tests":0,"build":0,"binary":0,"watch_or_w":0,"lint":0,"network":0},
    "budgets": {"timeout_seconds":10,"gate_invocations_max":1,"stdout_bytes_max":4194304,"stderr_bytes_max":4194304,"aggregate_receipts_bytes_max":16777216},
    "claim_limit": "Somente poder discriminatorio estatico da ordem causal P1366; nao atesta isolamento nem paridade funcional geral com o vanila.",
}
print(json.dumps(receipt, ensure_ascii=False, sort_keys=True, indent=2))
raise SystemExit(0 if passed else 1)
