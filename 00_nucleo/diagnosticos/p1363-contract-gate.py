#!/usr/bin/env python3
"""Gate estatico, deterministico e pre-implementacao do contrato P1363.

Le somente os artefatos documentais e fontes de baseline explicitamente
congelados. Nao le codigo/testes candidatos, nao executa Cargo ou produto e
nao cria subprocessos. O resultado mede apenas a discriminacao binding-free
do fragmento P1363; nao certifica uma implementacao.
"""

import hashlib
import json
import sys


ROOT = "/repos/Antigravity/typst-crystalline/"
MANIFEST = "00_nucleo/diagnosticos/p1363-manifest.json"
CONTRACT = "00_nucleo/diagnosticos/p1363-contract.json"
ORACLES = "00_nucleo/diagnosticos/p1363-oracles.json"
MUTANTS = "00_nucleo/diagnosticos/p1363-mutants.json"
PREFLIGHT = "00_nucleo/diagnosticos/p1363-watch-timestamp-preflight.md"
STEP = "00_nucleo/materialization/typst-passo-1363.md"
L0_PATHS = [
    "00_nucleo/prompts/shell/duration.md",
    "00_nucleo/prompts/shell/tests/p1361-duration.md",
    "00_nucleo/prompts/wiring.md",
]
VANILLA_PATHS = [
    "lab/typst-original/crates/typst-cli/src/watch.rs",
    "lab/typst-original/crates/typst-cli/src/terminal.rs",
]
CARGO_PATHS = ["Cargo.toml", "04_wiring/Cargo.toml"]
READ_ALLOWLIST = [
    MANIFEST,
    CONTRACT,
    ORACLES,
    MUTANTS,
    PREFLIGHT,
    STEP,
    *L0_PATHS,
    *VANILLA_PATHS,
    *CARGO_PATHS,
]
MAX_INPUT_BYTES = 4 * 1024 * 1024
MAX_TOTAL_BYTES = 16 * 1024 * 1024
EXPECTED_RAW = {
    MANIFEST: "b8060f28aa39e917e1edd070b298a68ec2b2bc2508f45c3c2f721ef05cbf392e",
    CONTRACT: "00afeec9bf66593eb6b038c69336f96915b5b02e41eefd25d8dc589f485af76a",
    ORACLES: "f48e4d38d979df83b1258ab1e48af5addcbb298d3a75dd6114a7bb8220d560ee",
    MUTANTS: "27916f9159f44605c983608a83270406f5f97e90bfb13b8b2cb0ceeaf099beeb",
    PREFLIGHT: "536080636448ec007b79953c4342efb408041847db753b1d23f9b4e2f548d755",
    STEP: "7b2afb581df789843ef8952f0ccb321ce9e8c4e24f7aa6e75036e3e7e5c82c1c",
    L0_PATHS[0]: "ae02d8b69e1b4aa4f4c9ed08c8e6ecfcf62cf1c63b1e2042ac4576f117cac8ee",
    L0_PATHS[1]: "4c4d432ede32f8631cb310c150bb01fcb46133eb12e59901246c3d493d32504e",
    L0_PATHS[2]: "1f629828286a1c634fcf2cfb8535f73fa75c9dba9ae8404f1f03b7173fc4e152",
    VANILLA_PATHS[0]: "3d5ee88a01513902055881b8cd4245540327ef33a4d10a6788bcabce17cdb585",
    VANILLA_PATHS[1]: "0fbde0a28d5f7842d2c0b692b6710d422784f8df1b61ee7c77d74bf40d2f04a5",
    CARGO_PATHS[0]: "e594f6eb560ba88be6e59abfcb0a8cde363e24dbe383c4b2bfbeb9d75c6238ed",
    CARGO_PATHS[1]: "dbc8a208f247b5ca46767892d3cb974071a444e673ba6a06c7a83d29ec3e94ef",
}
EXPECTED_NORMATIVE = {
    L0_PATHS[0]: "558e6cf3e01c9f6a175adc1f1d0caec5d8cc5597fa739b4d65cd77a2c52e8d3a",
    L0_PATHS[1]: "c14ad8fe99c930a8c075b4519a012a31ea79249afb0073896a61ca534df3764f",
    L0_PATHS[2]: "1db54618efd9b38368b9adb3d5fe80cbdbb8966a2fd00434963bc3ef991d8573",
}


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def read_bytes(path):
    require(path in READ_ALLOWLIST, "read outside allowlist: " + path)
    with open(ROOT + path, "rb") as handle:
        data = handle.read(MAX_INPUT_BYTES + 1)
    require(len(data) <= MAX_INPUT_BYTES, "input exceeds 4 MiB: " + path)
    return data


def load_json(blobs, path):
    return json.loads(blobs[path].decode("utf-8"))


def normative_bytes(data):
    marker = "Hash do Código:".encode("utf-8")
    lines = data.splitlines(keepends=True)
    selected = [line for line in lines if not line.startswith(marker)]
    require(len(selected) + 1 == len(lines), "expected exactly one Hash do Codigo line")
    return b"".join(selected)


def classify_mutant(mutant, witness_ids):
    witnesses = mutant.get("witnesses", [])
    decidable = (
        mutant.get("valid") is True
        and mutant.get("equivalent") is False
        and mutant.get("must_be") == "Violated"
        and bool(mutant.get("counterexample"))
        and bool(witnesses)
        and all(witness in witness_ids for witness in witnesses)
    )
    return {
        "id": mutant.get("id"),
        "classification": "Violated" if decidable else "Unknown",
        "witnesses": witnesses if decidable else [],
    }


def main():
    blobs = {path: read_bytes(path) for path in READ_ALLOWLIST}
    observed_bytes = sum(len(data) for data in blobs.values())
    require(observed_bytes <= MAX_TOTAL_BYTES, "protected inputs exceed 16 MiB")
    observed_raw = {path: sha256(data) for path, data in blobs.items()}
    require(observed_raw == EXPECTED_RAW, "protected raw hash mismatch")

    observed_normative = {
        path: sha256(normative_bytes(blobs[path])) for path in L0_PATHS
    }
    require(observed_normative == EXPECTED_NORMATIVE, "normative digest mismatch")

    manifest = load_json(blobs, MANIFEST)
    contract = load_json(blobs, CONTRACT)
    oracles = load_json(blobs, ORACLES)
    mutants_doc = load_json(blobs, MUTANTS)

    require(manifest["step"] == contract["step"] == oracles["step"] == mutants_doc["step"] == "P1363", "step mismatch")
    require(contract["manifest_path"] == MANIFEST, "contract manifest path mismatch")
    require(oracles["frozen_inputs"]["manifest"]["raw_sha256"] == observed_raw[MANIFEST], "oracle manifest pin mismatch")
    require(oracles["frozen_inputs"]["contract"]["raw_sha256"] == observed_raw[CONTRACT], "oracle contract pin mismatch")
    require(mutants_doc["manifest_raw_sha256"] == observed_raw[MANIFEST], "mutant manifest pin mismatch")
    require(mutants_doc["contract_raw_sha256"] == observed_raw[CONTRACT], "mutant contract pin mismatch")
    require(mutants_doc["oracle_path"] == ORACLES, "mutant oracle path mismatch")

    frozen_by_path = {item["path"]: item for item in manifest["frozen_inputs"]}
    for path in [STEP, PREFLIGHT, *L0_PATHS, *VANILLA_PATHS, *CARGO_PATHS]:
        require(frozen_by_path[path]["raw_sha256"] == observed_raw[path], "manifest raw pin mismatch: " + path)
    for path in L0_PATHS:
        require(frozen_by_path[path]["normative_sha256"] == observed_normative[path], "manifest normative pin mismatch: " + path)

    oracle_prompts = {item["path"]: item for item in oracles["frozen_inputs"]["normative_prompts"]}
    for path in L0_PATHS:
        require(oracle_prompts[path]["raw_sha256"] == observed_raw[path], "oracle L0 raw pin mismatch: " + path)
        require(oracle_prompts[path]["normative_sha256"] == observed_normative[path], "oracle L0 normative pin mismatch: " + path)
    oracle_vanilla = {item["path"]: item for item in oracles["frozen_inputs"]["vanilla"]}
    for path in VANILLA_PATHS:
        require(oracle_vanilla[path]["raw_sha256"] == observed_raw[path], "oracle vanilla pin mismatch: " + path)

    root_cargo = blobs[CARGO_PATHS[0]].decode("utf-8")
    wiring_cargo = blobs[CARGO_PATHS[1]].decode("utf-8")
    require('time        = { version = "0.3", features = ["macros", "formatting", "parsing", "local-offset"] }  # ADR-0021 — Datetime' in root_cargo, "workspace time identity missing")
    require(observed_raw[CARGO_PATHS[0]] == EXPECTED_RAW[CARGO_PATHS[0]], "root Cargo raw drift")
    require(observed_raw[CARGO_PATHS[1]] == EXPECTED_RAW[CARGO_PATHS[1]], "wiring Cargo baseline drift")
    require("time        = { workspace = true }" not in wiring_cargo, "authorized Cargo delta already present before seal")
    require(oracles["frozen_inputs"]["cargo"]["workspace"]["raw_sha256"] == observed_raw[CARGO_PATHS[0]], "oracle root Cargo pin mismatch")
    require(oracles["frozen_inputs"]["cargo"]["wiring_baseline"]["raw_sha256"] == observed_raw[CARGO_PATHS[1]], "oracle wiring Cargo pin mismatch")

    positive = oracles["positive_cases"]
    negative = oracles["negative_cases"]
    opaque = oracles["opaque_cases"]
    require([item["id"] for item in positive] == [f"P{i:02d}" for i in range(1, 15)], "positive IDs/count mismatch")
    require([item["id"] for item in negative] == [f"N{i:02d}" for i in range(1, 17)], "negative IDs/count mismatch")
    require([item["id"] for item in opaque] == [f"O{i:02d}" for i in range(1, 9)], "opaque IDs/count mismatch")
    require(all(item.get("classification") == "Preserved" and item.get("witnesses") for item in positive), "positive lacks Preserved classification or witness")
    require(all(item.get("expected") == "Violated" and item.get("witness") and item.get("reason_code") for item in negative), "negative lacks Violated witness")
    require(all(item.get("expected") == "Unknown" and item.get("reason_code") for item in opaque), "opaque classification mismatch")
    require(oracles["classification_policy"]["unexpected_unknown"] == "fail_closed", "Unknown is not fail-closed")
    require(oracles["eligibility"]["unexpected_unknown_blocks_seal"] is True, "unexpected Unknown does not block seal")

    require([item["id"] for item in contract["positive_cases"]] == [item["id"] for item in positive], "contract/oracle positive ID mismatch")
    require([item["id"] for item in contract["negative_cases"]] == [item["id"] for item in negative], "contract/oracle negative ID mismatch")
    require([item["id"] for item in contract["opaque_cases"]] == [item["id"] for item in opaque], "contract/oracle opaque ID mismatch")
    require([item["id"] for item in contract["mutations"]] == [f"M{i:02d}" for i in range(1, 29)], "contract mutation IDs/count mismatch")

    mutants = mutants_doc["mutations"]
    require([item["id"] for item in mutants] == [f"M{i:02d}" for i in range(1, 29)], "mutant IDs/count mismatch")
    witness_ids = {item["id"] for item in positive + negative + opaque}
    forward_1 = [classify_mutant(item, witness_ids) for item in mutants]
    forward_2 = [classify_mutant(item, witness_ids) for item in mutants]
    reverse = [classify_mutant(item, witness_ids) for item in reversed(mutants)]
    require(forward_1 == forward_2, "same-order repetition changed vector")
    require(forward_1 == list(reversed(reverse)), "reverse order changed vector")
    require(all(item["classification"] == "Violated" for item in forward_1), "surviving or Unknown mutant")
    killed = len(forward_1)
    score = killed / len(mutants)
    require(score == mutants_doc["eligibility"]["required_mutation_score"] == contract["gate"]["required_mutation_score"] == 1.0, "mutation score below 1.0")

    safety = manifest["memory_safety"]
    require(set(["typst watch", "typst w", "external wiring suite", "broad test suite"]).issubset(set(safety["forbidden_processes"])), "memory forbidden-process set incomplete")
    require(safety["cargo_build_jobs"] == 1, "Cargo job bound mismatch")
    require(safety["timeout_seconds_per_command_max"] <= 120, "timeout exceeds 120 seconds")
    require(safety["stdout_limit_bytes_per_stream"] <= MAX_INPUT_BYTES, "stdout bound exceeds 4 MiB")
    require(safety["stderr_limit_bytes_per_stream"] <= MAX_INPUT_BYTES, "stderr bound exceeds 4 MiB")
    require(safety["receipt_total_limit_bytes"] <= MAX_TOTAL_BYTES, "receipt bound exceeds 16 MiB")
    require(oracles["classification_policy"]["process_safety"].startswith("no watch, w, product process"), "oracle process prohibition mismatch")

    norm_pairs = "\n".join(path + ":" + observed_normative[path] for path in L0_PATHS).encode("utf-8")
    result = {
        "schema": "tekt-static-contract-gate-result/v1",
        "step": "P1363",
        "verdict": "seal-static-contract",
        "scope": "pre-implementation binding-free timestamp-envelope discrimination only",
        "raw_hashes": observed_raw,
        "normative_hashes": observed_normative,
        "normative_set_sha256": sha256(norm_pairs),
        "cargo_raw": {"workspace": "Preserved", "wiring_baseline": "Preserved", "authorized_delta_present": False},
        "oracles": {"positive_preserved": 14, "negative_violated": 16, "opaque_unknown": 8, "unknown_success_credit": 0},
        "mutations": {"valid": 28, "violated": killed, "survived": 0, "unexpected_unknown": 0, "score": score},
        "classification_vector": forward_1,
        "determinism": {"same_order_runs": 2, "reverse_order_runs": 1, "result": "Preserved"},
        "safety": {
            "subprocesses_created_by_gate": 0,
            "cargo_product_tests_watch_or_w_executed": False,
            "protected_input_bytes_observed": observed_bytes,
            "protected_input_bytes_each_max": MAX_INPUT_BYTES,
            "protected_input_bytes_total_max": MAX_TOTAL_BYTES,
        },
    }
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (AssertionError, KeyError, TypeError, ValueError, UnicodeError) as error:
        print(json.dumps({"verdict": "do-not-seal", "error": str(error)}, sort_keys=True, separators=(",", ":")))
        sys.exit(1)
