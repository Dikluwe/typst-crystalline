#!/usr/bin/env python3
"""Gate estatico e limitado para o contrato discriminatorio do P1364.

Nao executa Cargo, testes, binarios, watch, lint ou rede. O classificador opera
somente sobre observacoes canonicas e ataques declarativos congelados.
"""

from __future__ import annotations

import copy
import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"


def raw(path: str) -> bytes:
    return (ROOT / path).read_bytes()


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def load(path: str) -> dict:
    return json.loads(raw(path))


def normative_l0(data: bytes) -> bytes:
    lines = data.splitlines(keepends=True)
    return b"".join(line for line in lines if not line.startswith(b"Hash do C\xc3\xb3digo:"))


MANIFEST_PATH = "00_nucleo/diagnosticos/p1364-manifest.json"
CONTRACT_PATH = "00_nucleo/diagnosticos/p1364-contract.json"
ORACLES_PATH = "00_nucleo/diagnosticos/p1364-oracles.json"
MUTANTS_PATH = "00_nucleo/diagnosticos/p1364-mutants.json"

manifest = load(MANIFEST_PATH)
contract = load(CONTRACT_PATH)
oracles = load(ORACLES_PATH)
mutants = load(MUTANTS_PATH)

checks: list[dict] = []


def check(name: str, condition: bool, detail: str) -> None:
    checks.append({"name": name, "pass": bool(condition), "detail": detail})


# Identidade e encadeamento das entradas protegidas.
manifest_sha = sha(raw(MANIFEST_PATH))
contract_sha = sha(raw(CONTRACT_PATH))
oracles_sha = sha(raw(ORACLES_PATH))
mutants_sha = sha(raw(MUTANTS_PATH))
check("manifest_link_oracles", oracles["inputs"]["manifest"]["raw_sha256"] == manifest_sha, manifest_sha)
check("manifest_link_mutants", mutants["manifest_raw_sha256"] == manifest_sha, manifest_sha)
check("contract_link_oracles", oracles["inputs"]["contract"]["raw_sha256"] == contract_sha, contract_sha)
check("contract_link_mutants", mutants["contract_raw_sha256"] == contract_sha, contract_sha)

frozen = {entry["path"]: entry for entry in manifest["frozen_inputs"]}
for path in (
    "00_nucleo/materialization/typst-passo-1364.md",
    "00_nucleo/diagnosticos/p1364-watch-spacing-preflight.md",
):
    actual = sha(raw(path))
    check(f"raw_frozen:{path}", actual == frozen[path]["raw_sha256"], actual)

l0_path = "00_nucleo/prompts/wiring.md"
l0_raw = raw(l0_path)
l0_norm = sha(normative_l0(l0_raw))
check("l0_normative_digest", l0_norm == frozen[l0_path]["normative_sha256"], l0_norm)
check("l0_normative_digest_oracles", l0_norm == oracles["inputs"]["wiring_l0"]["normative_sha256"], l0_norm)

baseline_path = "04_wiring/src/main.rs"
baseline_sha = sha(raw(baseline_path))
check("baseline_identity_manifest", baseline_sha == frozen[baseline_path]["raw_sha256"], baseline_sha)
check("baseline_identity_contract", baseline_sha == contract["baseline_main_rs_raw_sha256"], baseline_sha)
check("candidate_absent_watch_block_at", b"fn watch_block_at" not in raw(baseline_path), "absent required")
check("candidate_absent_watch_block_now", b"fn watch_block_now" not in raw(baseline_path), "absent required")

vanila_path = "lab/typst-original/crates/typst-cli/src/watch.rs"
vanila = raw(vanila_path)
vanila_sha = sha(vanila)
check("vanila_source_identity", vanila_sha == oracles["inputs"]["vanila_source"]["raw_sha256"], vanila_sha)
check(
    "vanila_spacing_source_fact",
    b"self.print_message(&mut out)?;" in vanila
    and b"if !matches!(self, Status::Compiling)" in vanila
    and b"writeln!(out)?;" in vanila,
    "print_message followed by conditional writeln",
)

check("contract_counts", len(contract["positive_cases"]) == 14 and len(contract["negative_cases"]) == 18 and len(contract["opaque_cases"]) == 8, "14/18/8")
check("oracle_counts", oracles["counts"] == {"positive": 14, "negative": 18, "opaque": 8}, str(oracles["counts"]))
check("mutant_count", len(mutants["mutants"]) == 26 and mutants["summary"]["valid_mutants"] == 26, "26")
check("unknown_fail_closed", manifest["unknown_policy"]["default"] == "fail_closed" and oracles["classifier"]["fail_closed"].startswith("Unknown is never promoted"), "Unknown has zero credit")

CANON_LINES = oracles["public_observation"]["canonical_lines"]
CANON_BLOCKS = oracles["public_observation"]["canonical_blocks"]
STATES = ["Compiling", "Success", "PartialSuccess", "Error"]


def canonical_observation() -> dict:
    return {
        "lines": copy.deepcopy(CANON_LINES),
        "blocks": copy.deepcopy(CANON_BLOCKS),
        "pre_ready": True,
        "post_ready": True,
        "writer_channel": "stderr",
        "external_terminator": False,
        "block_at_clock": False,
        "now_local": True,
        "now_utc_lazy": True,
        "now_delegates": True,
        "helper_calls_only_run_watch": True,
        "test_clock": False,
        "forbidden_process": False,
        "helpers_private": True,
        "new_product_types": 0,
        "authorized_delta_only": True,
    }


def violations(obs: dict) -> list[str]:
    found = []
    if obs["lines"] != CANON_LINES or any("\r" in v or "\n" in v for v in obs["lines"].values()):
        found.append("line_bytes")
    if obs["blocks"] != CANON_BLOCKS or any("\r" in v for v in obs["blocks"].values()):
        found.append("block_bytes")
    if not obs["pre_ready"] or not obs["post_ready"]:
        found.append("ready_block_routing")
    if obs["writer_channel"] != "stderr" or obs["external_terminator"]:
        found.append("stderr_writer_shape")
    if obs["block_at_clock"] or not (obs["now_local"] and obs["now_utc_lazy"] and obs["now_delegates"]):
        found.append("clock_boundary")
    if not obs["helper_calls_only_run_watch"]:
        found.append("watch_only_routing")
    if obs["test_clock"]:
        found.append("literal_fixture")
    if obs["forbidden_process"]:
        found.append("memory_protocol")
    if not obs["helpers_private"] or obs["new_product_types"] or not obs["authorized_delta_only"]:
        found.append("authorized_delta")
    return found


def classify(obs: dict | None, opaque: bool = False) -> dict:
    if opaque:
        return {"classification": "Unknown", "reason": "explicitly_opaque"}
    if obs is None:
        return {"classification": "Unknown", "reason": "missing_public_observation"}
    bad = violations(obs)
    if bad:
        return {"classification": "Violated", "reason": "+".join(sorted(bad))}
    return {"classification": "Preserved", "reason": "all_public_predicates_true"}


def attacked(case_id: str) -> dict:
    o = canonical_observation()
    if case_id == "N01": o["blocks"]["Compiling"] = CANON_LINES["Compiling"]
    elif case_id == "N02": o["blocks"]["Compiling"] += "\n"
    elif case_id == "N03": o["blocks"]["Success"] = CANON_LINES["Success"] + "\n"
    elif case_id == "N04": o["blocks"]["PartialSuccess"] = CANON_LINES["PartialSuccess"] + "\n"
    elif case_id == "N05": o["blocks"]["Error"] = CANON_LINES["Error"] + "\n"
    elif case_id == "N06": o["blocks"]["Success"] += "\n"
    elif case_id == "N07": o["blocks"]["Error"] = CANON_LINES["Error"] + "\r\n\r\n"
    elif case_id == "N08": o["lines"]["Compiling"] += "\n"
    elif case_id == "N09": o["external_terminator"] = True
    elif case_id == "N10": o["writer_channel"] = "stdout"
    elif case_id == "N11": o["post_ready"] = False
    elif case_id == "N12": o["pre_ready"] = False
    elif case_id == "N13":
        o["blocks"]["Compiling"] += "\n"
        for state in STATES[1:]: o["blocks"][state] = CANON_LINES[state] + "\n"
    elif case_id == "N14": o["block_at_clock"] = True
    elif case_id == "N15": o["test_clock"] = True
    elif case_id == "N16": o["helper_calls_only_run_watch"] = False
    elif case_id == "N17": o["forbidden_process"] = True
    elif case_id == "N18": o["authorized_delta_only"] = False
    else: raise ValueError(f"unknown negative {case_id}")
    return o


MUTANT_TO_ATTACK = {
    "M01":"N01", "M02":"N02", "M03":"N03", "M04":"N04", "M05":"N05",
    "M06":"N06", "M07":"N07", "M08":"N08", "M09":"N09", "M10":"N10",
    "M11":"N11", "M12":"N12", "M13":"N03", "M14":"N04", "M15":"N05",
    "M16":"N02", "M17":"N14", "M20":"N16", "M21":"N18", "M22":"N18",
}


def mutant_observation(mutant_id: str) -> dict:
    if mutant_id in MUTANT_TO_ATTACK:
        o = attacked(MUTANT_TO_ATTACK[mutant_id])
        if mutant_id == "M21": o["helpers_private"] = False
        if mutant_id == "M22": o["new_product_types"] = 1
        return o
    o = canonical_observation()
    if mutant_id == "M18": o["now_local"] = False
    elif mutant_id == "M19": o["now_utc_lazy"] = False
    elif mutant_id == "M23":
        for state in STATES:
            o["lines"][state] = o["lines"][state].replace("[03:04:05]", "03:04:05")
            o["blocks"][state] = o["blocks"][state].replace("[03:04:05]", "03:04:05")
    elif mutant_id == "M24":
        o["lines"]["Error"] = "[03:04:05] compile failed"
        o["blocks"]["Error"] = "[03:04:05] compile failed\n\n"
    elif mutant_id == "M25": o["blocks"]["Success"] = "\n" + CANON_LINES["Success"] + "\n"
    elif mutant_id == "M26": o["blocks"]["Success"] = CANON_LINES["Success"] + "\n \n"
    else: raise ValueError(f"unknown mutant {mutant_id}")
    return o


positive_ids = [case["id"] for case in oracles["positive_cases"]]
negative_ids = [case["id"] for case in oracles["negative_cases"]]
opaque_ids = [case["id"] for case in oracles["opaque_cases"]]
mutant_ids = [mutant["id"] for mutant in mutants["mutants"] if mutant["valid"]]


def evaluate(order: list[tuple[str, str]]) -> dict:
    result = {}
    for kind, case_id in order:
        if kind == "positive": value = classify(canonical_observation())
        elif kind == "negative": value = classify(attacked(case_id))
        elif kind == "opaque": value = classify(None, opaque=True)
        elif kind == "mutant": value = classify(mutant_observation(case_id))
        else: raise ValueError(kind)
        result[f"{kind}:{case_id}"] = value
    return result


order = ([('positive', x) for x in positive_ids]
         + [('negative', x) for x in negative_ids]
         + [('opaque', x) for x in opaque_ids]
         + [('mutant', x) for x in mutant_ids])
first = evaluate(order)
repeat = evaluate(order)
reverse = evaluate(list(reversed(order)))

positive_ok = all(first[f"positive:{x}"]["classification"] == "Preserved" for x in positive_ids)
negative_ok = all(first[f"negative:{x}"]["classification"] == "Violated" for x in negative_ids)
opaque_ok = all(first[f"opaque:{x}"]["classification"] == "Unknown" for x in opaque_ids)
rejected = sum(first[f"mutant:{x}"]["classification"] == "Violated" for x in mutant_ids)
unexpected_unknown = [key for key, value in first.items() if not key.startswith("opaque:") and value["classification"] == "Unknown"]
mutation_score = rejected / len(mutant_ids) if mutant_ids else 0.0
deterministic = first == repeat == reverse

check("positive_vector", positive_ok, f"{sum(first[f'positive:{x}']['classification'] == 'Preserved' for x in positive_ids)}/14 Preserved")
check("negative_vector", negative_ok, f"{sum(first[f'negative:{x}']['classification'] == 'Violated' for x in negative_ids)}/18 Violated")
check("opaque_vector", opaque_ok, f"{sum(first[f'opaque:{x}']['classification'] == 'Unknown' for x in opaque_ids)}/8 Unknown, zero credit")
check("mutant_vector", rejected == 26 and mutation_score == 1.0, f"{rejected}/26; score={mutation_score:.1f}")
check("unexpected_unknown", not unexpected_unknown, str(unexpected_unknown))
check("repeat_reverse", deterministic, "classification and reason stable")

passed = all(item["pass"] for item in checks)
receipt = {
    "schema": "tekt-discriminatory-gate-receipt/v1",
    "step": "P1364",
    "regime": "executado sem atestacao de isolamento",
    "mode": "pure_static_bounded",
    "verdict": "SEALABLE" if passed else "NOT_SEALABLE",
    "inputs": {
        "manifest_raw_sha256": manifest_sha,
        "contract_raw_sha256": contract_sha,
        "oracles_raw_sha256": oracles_sha,
        "mutants_raw_sha256": mutants_sha,
        "l0_normative_sha256": l0_norm,
        "baseline_main_raw_sha256": baseline_sha,
        "vanila_source_raw_sha256": vanila_sha,
    },
    "vectors": {"positive_preserved": 14 if positive_ok else 0, "negative_violated": 18 if negative_ok else 0, "opaque_unknown": 8 if opaque_ok else 0, "valid_mutants_rejected": rejected, "valid_mutants": len(mutant_ids), "mutation_score": mutation_score, "unexpected_unknown": unexpected_unknown},
    "determinism": {"repeat_equal": first == repeat, "reverse_equal": first == reverse},
    "checks": checks,
    "execution": {"cargo": 0, "tests": 0, "binary": 0, "watch_or_w": 0, "lint": 0, "network": 0},
    "claim_limit": "Somente poder discriminatorio estatico do fragmento P1364; nao atesta isolamento nem paridade geral com o vanila.",
}
print(json.dumps(receipt, ensure_ascii=False, sort_keys=True, indent=2))
raise SystemExit(0 if passed else 1)
