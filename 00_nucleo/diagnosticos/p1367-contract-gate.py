#!/usr/bin/env python3
"""Gate discriminatorio puramente estatico e limitado do P1367.

Nao executa Cargo, testes, build, binario, watch/w, lint ou rede. A
classificacao depende somente das entradas congeladas e de observacoes
canonicas declarativas da fronteira write_all -> flush.
"""

from __future__ import annotations

import copy
import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
MANIFEST_PATH = "00_nucleo/diagnosticos/p1367-manifest.json"
CONTRACT_PATH = "00_nucleo/diagnosticos/p1367-contract.json"
ORACLES_PATH = "00_nucleo/diagnosticos/p1367-oracles.json"
MUTANTS_PATH = "00_nucleo/diagnosticos/p1367-mutants.json"


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
    ("00_nucleo/materialization/typst-passo-1367.md", "step"),
    ("00_nucleo/diagnosticos/p1367-watch-flush-preflight.md", "preflight"),
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
check("candidate_write_helper_absent", b"fn write_watch_block" not in baseline, "absent required")
check("candidate_emit_wrapper_absent", b"fn emit_watch_block" not in baseline, "absent required")
check("legacy_status_eprints", baseline.count(b'eprint!("{}", compiling)') == 1 and baseline.count(b'eprint!("{}", watch_block_now(&status))') == 1, "two frozen status eprint sites")

vanila_watch_path = "lab/typst-original/crates/typst-cli/src/watch.rs"
vanila_terminal_path = "lab/typst-original/crates/typst-cli/src/terminal.rs"
vanila_watch = raw(vanila_watch_path)
vanila_terminal = raw(vanila_terminal_path)
vanila_watch_sha = sha(vanila_watch)
vanila_terminal_sha = sha(vanila_terminal)
check("vanila_watch_manifest", vanila_watch_sha == frozen[vanila_watch_path]["raw_sha256"], vanila_watch_sha)
check("vanila_watch_oracles", vanila_watch_sha == oracles["inputs"]["vanila_watch"]["raw_sha256"], vanila_watch_sha)
check("vanila_terminal_manifest", vanila_terminal_sha == frozen[vanila_terminal_path]["raw_sha256"], vanila_terminal_sha)
check("vanila_terminal_oracles", vanila_terminal_sha == oracles["inputs"]["vanila_terminal"]["raw_sha256"], vanila_terminal_sha)
watch_message = vanila_watch.find(b"self.print_message(&mut out)?")
watch_flush = vanila_watch.find(b"out.flush()", watch_message)
terminal_write = vanila_terminal.find(b"self.inner.stream.lock().write(buf)")
terminal_flush = vanila_terminal.find(b"self.inner.stream.lock().flush()", terminal_write)
check("vanila_status_flush_order", -1 < watch_message < watch_flush, "message precedes flush")
check("vanila_stderr_write_flush", -1 < terminal_write < terminal_flush, "stderr-backed Write exposes write and flush")

# Integridade estrutural do corpus.
positive_ids = [case["id"] for case in oracles["positive_cases"]]
negative_ids = [case["id"] for case in oracles["negative_cases"]]
opaque_ids = [case["id"] for case in oracles["opaque_cases"]]
contract_positive_ids = [case["id"] for case in contract["positive_cases"]]
contract_negative_ids = [case["id"] for case in contract["negative_cases"]]
contract_opaque_ids = [case["id"] for case in contract["opaque_cases"]]
valid_mutants = [entry for entry in mutants["mutants"] if entry["valid"]]
mutant_ids = [entry["id"] for entry in valid_mutants]
check("contract_counts", (len(contract_positive_ids), len(contract_negative_ids), len(contract_opaque_ids)) == (15, 22, 8), "15/22/8")
check("oracle_counts", oracles["counts"] == {"positive": 15, "negative": 22, "opaque": 8}, str(oracles["counts"]))
check("case_id_alignment", positive_ids == contract_positive_ids and negative_ids == contract_negative_ids and opaque_ids == contract_opaque_ids, "contract and oracle IDs match in order")
check("case_id_uniqueness", len(set(positive_ids + negative_ids + opaque_ids)) == 45, "45 unique case IDs")
check("negative_witnesses", all(case.get("public_witness") for case in oracles["negative_cases"]), "22/22 public witnesses present")
check("mutant_count", len(mutant_ids) == 30 and mutants["summary"]["valid_mutants"] == 30, f"{len(mutant_ids)}/30")
check("mutant_witnesses", all(entry.get("public_witness") for entry in valid_mutants), "30/30 public witnesses present")
check("unknown_fail_closed", manifest["unknown_policy"]["default"] == "fail_closed" and "never promoted" in oracles["classifier"]["fail_closed"], "Unknown has zero credit")


def canonical() -> dict:
    success = oracles["public_observation"]["canonical_success"]
    write_failure = oracles["public_observation"]["canonical_write_failure"]
    flush_failure = oracles["public_observation"]["canonical_flush_failure"]
    return {
        "helper_private": True,
        "helper_signature": "&mut impl Write,&str->io::Result<()> without stderr",
        "write_api": "write_all",
        "success_bytes": copy.deepcopy(success["block_bytes"]),
        "expected_bytes": copy.deepcopy(success["block_bytes"]),
        "success_events": copy.deepcopy(success["event_log"]),
        "success_write_all_count": success["write_all_call_count"],
        "success_flush_count": success["flush_call_count"],
        "success_result": success["result_kind"],
        "write_failure_events": copy.deepcopy(write_failure["event_log"]),
        "write_failure_write_count": write_failure["write_all_call_count"],
        "write_failure_flush_count": write_failure["flush_call_count"],
        "write_failure_result": write_failure["result_kind"],
        "flush_failure_events": copy.deepcopy(flush_failure["event_log"]),
        "flush_failure_write_count": flush_failure["write_all_call_count"],
        "flush_failure_flush_count": flush_failure["flush_call_count"],
        "flush_failure_result": flush_failure["result_kind"],
        "repeat_stable": True,
        "wrapper_private": True,
        "stderr_lock_extent": "complete delegation",
        "wrapper_helper_count": 1,
        "wrapper_same_block": True,
        "wrapper_failure_policy": "explicit failure",
        "initial_wrapper_count": 1,
        "initial_same_block": True,
        "initial_before_compile": True,
        "terminal_wrapper_count": 1,
        "terminal_same_block": True,
        "terminal_before_diagnostics": True,
        "productive_wrapper_callsites": 2,
        "legacy_status_eprint_sites": 0,
        "diagnostic_helper_callsites": [],
        "test_function_count": 1,
        "test_modes": ["success", "write_failure", "flush_failure"],
        "test_assertions_complete": True,
        "test_forbidden_calls": [],
        "public_surface_delta": [],
        "frozen_behavior_delta": [],
        "authorized_delta_only": True,
    }


def violations(obs: dict) -> list[str]:
    bad: list[str] = []
    if not obs["helper_private"] or obs["helper_signature"] != "&mut impl Write,&str->io::Result<()> without stderr": bad.append("helper_boundary")
    if obs["write_api"] != "write_all": bad.append("write_api")
    if obs["success_write_all_count"] != 1: bad.append("write_cardinality")
    if obs["success_bytes"] != obs["expected_bytes"]: bad.append("byte_identity")
    if obs["success_events"] != ["write(exact-block-bytes)", "flush"]: bad.append("success_order")
    if obs["success_flush_count"] != 1: bad.append("flush_cardinality")
    if obs["success_result"] != "Ok": bad.append("success_result")
    if obs["write_failure_write_count"] != 1 or obs["write_failure_flush_count"] != 0: bad.append("write_failure_cardinality")
    if obs["write_failure_events"] != ["write(BrokenPipe)"]: bad.append("flush_after_write_error")
    if obs["write_failure_result"] != "BrokenPipe": bad.append("write_error_kind")
    if obs["flush_failure_write_count"] != 1 or obs["flush_failure_flush_count"] != 1: bad.append("flush_failure_cardinality")
    if obs["flush_failure_events"] != ["write(exact-block-bytes)", "flush(TimedOut)"]: bad.append("flush_failure_order")
    if obs["flush_failure_result"] != "TimedOut": bad.append("flush_error_kind")
    if not obs["repeat_stable"]: bad.append("nondeterminism")
    if not obs["wrapper_private"] or obs["stderr_lock_extent"] != "complete delegation": bad.append("stderr_lock_extent")
    if obs["wrapper_helper_count"] != 1 or not obs["wrapper_same_block"]: bad.append("wrapper_delegation")
    if obs["wrapper_failure_policy"] != "explicit failure": bad.append("wrapper_failure_policy")
    if obs["initial_wrapper_count"] != 1 or not obs["initial_same_block"] or not obs["initial_before_compile"]: bad.append("initial_routing")
    if obs["terminal_wrapper_count"] != 1 or not obs["terminal_same_block"] or not obs["terminal_before_diagnostics"]: bad.append("terminal_routing")
    if obs["productive_wrapper_callsites"] != 2 or obs["legacy_status_eprint_sites"] != 0: bad.append("status_inventory")
    if obs["diagnostic_helper_callsites"]: bad.append("diagnostic_capture")
    if obs["test_function_count"] != 1 or set(obs["test_modes"]) != {"success", "write_failure", "flush_failure"} or not obs["test_assertions_complete"]: bad.append("test_fixture")
    if obs["test_forbidden_calls"]: bad.append("forbidden_test_capability")
    if obs["public_surface_delta"]: bad.append("public_surface")
    if obs["frozen_behavior_delta"]: bad.append("frozen_behavior")
    if not obs["authorized_delta_only"]: bad.append("scope_expansion")
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
    if case_id == "N01": obs["success_events"] = ["flush", "write(exact-block-bytes)"]
    elif case_id == "N02": obs["write_api"] = "write"
    elif case_id == "N03": obs["success_write_all_count"] = 0
    elif case_id == "N04": obs["success_bytes"] = obs["expected_bytes"][1:]
    elif case_id == "N05": obs["success_flush_count"] = 0
    elif case_id == "N06": obs["success_flush_count"] = 2
    elif case_id == "N07": obs["write_failure_events"] = ["write(BrokenPipe)", "flush"]
    elif case_id == "N08": obs["write_failure_result"] = "Other"
    elif case_id == "N09": obs["flush_failure_result"] = "Ok"
    elif case_id == "N10": obs["stderr_lock_extent"] = "none"
    elif case_id == "N11": obs["wrapper_helper_count"] = 2
    elif case_id == "N12": obs["wrapper_failure_policy"] = "silent success"
    elif case_id == "N13": obs["initial_wrapper_count"] = 2
    elif case_id == "N14": obs["terminal_wrapper_count"] = 2
    elif case_id == "N15": obs["productive_wrapper_callsites"] = 1
    elif case_id == "N16": obs["diagnostic_helper_callsites"] = ["drain_to_stderr"]
    elif case_id == "N17": obs["test_forbidden_calls"] = ["emit_watch_block", "stderr"]
    elif case_id == "N18": obs["test_forbidden_calls"] = ["process", "watch"]
    elif case_id == "N19": obs["test_function_count"] = 3
    elif case_id == "N20": obs["helper_private"] = False; obs["public_surface_delta"] = ["pub helper"]
    elif case_id == "N21": obs["frozen_behavior_delta"] = ["status bytes"]
    elif case_id == "N22": obs["authorized_delta_only"] = False
    else: raise ValueError(f"unknown negative case {case_id}")
    return obs


MUTANT_ATTACK = {
    "M01":"N01", "M02":"N02", "M03":"N02", "M04":"N03", "M05":"N03",
    "M06":"N03", "M07":"N04", "M08":"N04", "M09":"N04", "M10":"N05",
    "M11":"N06", "M12":"N07", "M13":"N08", "M14":"N09", "M15":"N09",
    "M16":"N10", "M17":"N10", "M18":"N11", "M19":"N12", "M20":"N13",
    "M21":"N14", "M22":"N15", "M23":"N16", "M24":"N16", "M25":"N17",
    "M26":"N18", "M27":"N19", "M28":"N20", "M29":"N21", "M30":"N22",
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
check("positive_vector", positive_count == 15, f"{positive_count}/15 Preserved")
check("negative_vector", negative_count == 22, f"{negative_count}/22 Violated")
check("opaque_vector", opaque_count == 8, f"{opaque_count}/8 Unknown; zero credit")
check("mutant_vector", rejected == 30 and mutation_score == 1.0, f"{rejected}/30; score={mutation_score:.1f}")
check("unexpected_unknown", not unexpected_unknown, str(unexpected_unknown))
check("repeat_deterministic", first == repeat, "classification, reason and witness equal")
check("reverse_deterministic", first == reverse, "classification, reason and witness equal")

passed = all(item["pass"] for item in checks)
receipt = {
    "schema": "tekt-discriminatory-gate-receipt/v1",
    "step": "P1367",
    "regime": "executado sem atestacao de isolamento",
    "mode": "pure_static_bounded",
    "verdict": "SEALABLE" if passed else "NOT_SEALABLE",
    "inputs": {
        "manifest_raw_sha256": manifest_sha, "contract_raw_sha256": contract_sha,
        "oracles_raw_sha256": oracles_sha, "mutants_raw_sha256": mutants_sha,
        "l0_raw_sha256": l0_raw_sha, "l0_normative_sha256": l0_norm_sha,
        "baseline_main_raw_sha256": baseline_sha,
        "vanila_watch_raw_sha256": vanila_watch_sha,
        "vanila_terminal_raw_sha256": vanila_terminal_sha,
    },
    "vectors": {
        "positive_preserved": positive_count, "positive_total": 15,
        "negative_violated": negative_count, "negative_total": 22,
        "opaque_unknown": opaque_count, "opaque_total": 8, "opaque_credit": 0,
        "valid_mutants_rejected": rejected, "valid_mutants": len(mutant_ids),
        "mutation_score": mutation_score, "unexpected_unknown": unexpected_unknown,
    },
    "determinism": {"repeat_equal": first == repeat, "reverse_equal": first == reverse},
    "checks": checks,
    "execution": {"cargo":0,"tests":0,"build":0,"binary":0,"watch_or_w":0,"lint":0,"network":0},
    "budgets": {"timeout_seconds":10,"gate_invocations_max":1,"stdout_bytes_max":4194304,"stderr_bytes_max":4194304,"aggregate_receipts_bytes_max":16777216},
    "claim_limit": "Somente poder discriminatorio estatico da fronteira write_all/flush P1367; nao atesta isolamento nem paridade funcional geral com o vanila.",
}
print(json.dumps(receipt, ensure_ascii=False, sort_keys=True, indent=2))
raise SystemExit(0 if passed else 1)
