#!/usr/bin/env python3
"""Gate estatico e pre-implementacao do contrato observavel P1362.

Le somente os artefatos documentais e fontes vanilla congelados. Nao le codigo
produtivo cristalino, testes, diffs ou binarios; nao executa Cargo, produto,
testes, watch ou w; e nao cria subprocessos. O resultado mede exclusivamente a
discriminacao binding-free do contrato selado, nao uma implementacao candidata.
"""

import hashlib
import json
import sys


ROOT = "/repos/Antigravity/typst-crystalline/"
MANIFEST = "00_nucleo/diagnosticos/p1362-manifest.json"
CONTRACT = "00_nucleo/diagnosticos/p1362-contract.json"
ORACLES = "00_nucleo/diagnosticos/p1362-oracles.json"
MUTANTS = "00_nucleo/diagnosticos/p1362-mutants.json"
STEP = "00_nucleo/materialization/typst-passo-1362.md"
PREFLIGHT = "00_nucleo/diagnosticos/p1362-watch-status-preflight.md"
PROMPTS = [
    "00_nucleo/prompts/shell/duration.md",
    "00_nucleo/prompts/shell/tests/p1361-duration.md",
    "00_nucleo/prompts/wiring.md",
]
VANILLA_SOURCES = [
    "lab/typst-original/crates/typst-cli/src/watch.rs",
    "lab/typst-original/crates/typst-cli/src/compile.rs",
    "lab/typst-original/crates/typst-cli/src/terminal.rs",
]
READ_ALLOWLIST = [
    MANIFEST,
    CONTRACT,
    ORACLES,
    MUTANTS,
    STEP,
    PREFLIGHT,
    *PROMPTS,
    *VANILLA_SOURCES,
]
MAX_INPUT_BYTES = 4 * 1024 * 1024
MAX_TOTAL_BYTES = 16 * 1024 * 1024
EXPECTED_RAW_HASHES = {
    MANIFEST: "3216260f0b29fcd3b8dbc2cf483f9749573f4e89975de2b5a20ac809aefbf39e",
    CONTRACT: "17ae4ff69097473817b0f3e2d135c85f5aeee854de980286b74cabaf98799bb2",
    ORACLES: "4dea845b6f0beb0ed47da423075863d52cba435360658527560cecdc48255663",
    MUTANTS: "e346091c23e47c4f37671329293a21ef40fd41073e9724e97e9172818655ab85",
    STEP: "ffeec61dc1fc9d95838eb6dee3e3f68233b1099900c53e07c9215ddc2c757792",
    PREFLIGHT: "08df5c612b38d99599ee60e2ddb58f23efcc9bc81efd69036fc3b801e3295c60",
    PROMPTS[0]: "e2e71db117dff24bcd3b44950b6a852bbb8de94eccf69c735387da0845462468",
    PROMPTS[1]: "ccae170c7c936cf71673bad190929a4eb7e84552ad63c93df2d795fb4a67e3e9",
    PROMPTS[2]: "e5d89873da3dc03d8429f014762f35316197c18eb3ce21704964c14f15351944",
    VANILLA_SOURCES[0]: "3d5ee88a01513902055881b8cd4245540327ef33a4d10a6788bcabce17cdb585",
    VANILLA_SOURCES[1]: "a943a87637dd1f85a95f39c29175225c378cbf6d7a0e8d30645eb5cca7470703",
    VANILLA_SOURCES[2]: "0fbde0a28d5f7842d2c0b692b6710d422784f8df1b61ee7c77d74bf40d2f04a5",
}
EXPECTED_NORMATIVE_HASHES = {
    PROMPTS[0]: "02bb6aa40c3fd936d786ee20ecdbd3d078c41dc23bbbf547a7608db0c3c3499b",
    PROMPTS[1]: "14216054705c8f2b0cee0e3f8b17677d37065c7dd706a49086d1ed9206ba1ded",
    PROMPTS[2]: "e9907dedd503cbc2e82829632b0a6996342ea67cffe5545fffb23a6cda76ab1e",
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


def load_json(path, blobs):
    return json.loads(blobs[path].decode("utf-8"))


def normative_digest(data, path):
    prefix = "Hash do Código:".encode("utf-8")
    lines = data.splitlines(keepends=True)
    matches = [index for index, line in enumerate(lines) if line.startswith(prefix)]
    require(len(matches) == 1, "prompt must contain exactly one Hash do Codigo line: " + path)
    canonical = b"".join(line for index, line in enumerate(lines) if index != matches[0])
    return sha256(canonical)


def classify_positive(case):
    decidable = (
        case.get("expected_classification") == "Preserved"
        and bool(case.get("obligation"))
        and "expected_observation" in case
    )
    return {
        "id": case.get("id"),
        "classification": "Preserved" if decidable else "Unknown",
        "witness": case.get("expected_observation") if decidable else None,
    }


def classify_mutant(mutant, positive_ids):
    witnesses = mutant.get("witnesses", [])
    known = all(
        witness in positive_ids
        or witness.startswith("source-guard:")
        or witness.startswith("execution-guard:")
        or witness.startswith("manifest:")
        or witness.startswith("contract:")
        for witness in witnesses
    )
    decidable = (
        mutant.get("validity") == "valid"
        and mutant.get("expected_classification") == "Violated"
        and bool(witnesses)
        and known
        and bool(mutant.get("reason_code"))
    )
    return {
        "id": mutant.get("id"),
        "classification": "Violated" if decidable else "Unknown",
        "witnesses": witnesses if decidable else [],
        "reason_code": mutant.get("reason_code") if decidable else "UNKNOWN",
    }


def classify_opaque(case):
    decidable = (
        case.get("expected_classification") == "Unknown"
        and bool(case.get("question"))
        and bool(case.get("reason_code"))
    )
    return {
        "id": case.get("id"),
        "classification": "Unknown" if decidable else "Violated",
        "reason_code": case.get("reason_code") if decidable else "OPAQUE_CASE_MALFORMED",
        "success_credit": 0,
    }


def classify_excluded(attack, opaque_ids):
    decidable = (
        attack.get("validity") == "invalid_for_P1362"
        and attack.get("expected_classification") == "Unknown"
        and attack.get("witness") in opaque_ids
        and bool(attack.get("reason_code"))
    )
    return {
        "id": attack.get("id"),
        "classification": "Unknown" if decidable else "Violated",
        "included_in_mutation_denominator": False,
        "success_credit": 0,
    }


def repeated_vectors(rows, classifier):
    canonical = [[classifier(item) for item in rows] for _ in range(3)]
    reverse = [[classifier(item) for item in reversed(rows)] for _ in range(3)]
    require(canonical[0] == canonical[1] == canonical[2], "canonical repetition changed")
    require(reverse[0] == reverse[1] == reverse[2], "reverse repetition changed")
    require(canonical[0] == list(reversed(reverse[0])), "reordering changed result")
    return canonical[0]


def main():
    blobs = {path: read_bytes(path) for path in READ_ALLOWLIST}
    observed_total = sum(len(data) for data in blobs.values())
    require(observed_total <= MAX_TOTAL_BYTES, "protected inputs exceed 16 MiB")
    raw_hashes = {path: sha256(data) for path, data in blobs.items()}
    normative_hashes = {path: normative_digest(blobs[path], path) for path in PROMPTS}
    require(normative_hashes == EXPECTED_NORMATIVE_HASHES, "prompt normative digest mismatch")
    strict_paths = [path for path in READ_ALLOWLIST if path not in PROMPTS]
    require(
        all(raw_hashes[path] == EXPECTED_RAW_HASHES[path] for path in strict_paths),
        "non-prompt protected raw hash mismatch",
    )
    prompt_reseal = {
        path: {
            "raw_before": EXPECTED_RAW_HASHES[path],
            "raw_observed": raw_hashes[path],
            "raw_changed": raw_hashes[path] != EXPECTED_RAW_HASHES[path],
            "normative_expected": EXPECTED_NORMATIVE_HASHES[path],
            "normative_observed": normative_hashes[path],
            "classification": (
                "mechanical_hash_line_reseal"
                if raw_hashes[path] != EXPECTED_RAW_HASHES[path]
                else "raw_unchanged"
            ),
        }
        for path in PROMPTS
    }

    manifest = load_json(MANIFEST, blobs)
    contract = load_json(CONTRACT, blobs)
    oracles = load_json(ORACLES, blobs)
    mutants = load_json(MUTANTS, blobs)

    require(manifest["step"] == oracles["step"] == mutants["step"] == "P1362", "step mismatch")
    require(contract["contract_id"] == "P1362-watch-status-bodies-v1", "contract id mismatch")
    require(contract["manifest"]["path"] == MANIFEST, "contract manifest path mismatch")
    require(contract["manifest"]["sha256"] == raw_hashes[MANIFEST], "contract manifest pin mismatch")
    require(oracles["frozen_inputs"]["manifest"]["sha256"] == raw_hashes[MANIFEST], "oracle manifest pin mismatch")
    require(oracles["frozen_inputs"]["contract"]["sha256"] == raw_hashes[CONTRACT], "oracle contract pin mismatch")
    require(mutants["frozen_inputs"]["manifest_sha256"] == raw_hashes[MANIFEST], "mutant manifest pin mismatch")
    require(mutants["frozen_inputs"]["contract_sha256"] == raw_hashes[CONTRACT], "mutant contract pin mismatch")

    protected = {item["path"]: item for item in manifest["protected_inputs"]}
    require(set(protected) == set([STEP, PREFLIGHT, *PROMPTS, *VANILLA_SOURCES]), "manifest protected path set mismatch")
    for path in [STEP, PREFLIGHT, *VANILLA_SOURCES]:
        require(protected[path]["sha256_raw"] == raw_hashes[path], "manifest raw pin mismatch: " + path)
    for path in PROMPTS:
        require(protected[path]["sha256_raw"] == EXPECTED_RAW_HASHES[path], "manifest pre-reseal raw pin mismatch: " + path)
        require(protected[path]["sha256_normative"] == normative_hashes[path], "manifest normative pin mismatch: " + path)

    oracle_prompt_pins = {
        item["path"]: item["sha256_normative"]
        for item in oracles["frozen_inputs"]["prompt_normative_digests"]
    }
    mutant_prompt_pins = mutants["frozen_inputs"]["prompt_normative_sha256"]
    require(oracle_prompt_pins == EXPECTED_NORMATIVE_HASHES, "oracle normative pins mismatch")
    require(mutant_prompt_pins == EXPECTED_NORMATIVE_HASHES, "mutant normative pins mismatch")
    oracle_vanilla_pins = {
        item["path"]: item["sha256"]
        for item in oracles["frozen_inputs"]["vanilla_source_digests"]
    }
    require(oracle_vanilla_pins == {path: raw_hashes[path] for path in VANILLA_SOURCES}, "oracle vanilla pins mismatch")

    positive_ids = {f"P{index:02d}" for index in range(1, 23)}
    opaque_ids = {f"O{index:02d}" for index in range(1, 9)}
    mutant_ids = {f"M{index:02d}" for index in range(1, 23)}
    excluded_ids = {f"X{index:02d}" for index in range(1, 4)}
    negative_ids = {f"N{index:02d}" for index in range(1, 23)}
    require({item["id"] for item in contract["positive_cases"]} == positive_ids, "contract positive ids mismatch")
    require({item["id"] for item in contract["negative_cases"]} == negative_ids, "contract negative ids mismatch")
    require({item["id"] for item in contract["opaque_cases"]} == opaque_ids, "contract opaque ids mismatch")
    require({item["id"] for item in oracles["positive_cases"]} == positive_ids, "oracle positive ids mismatch")
    require({item["id"] for item in oracles["opaque_cases"]} == opaque_ids, "oracle opaque ids mismatch")
    require({item["id"] for item in mutants["valid_mutants"]} == mutant_ids, "valid mutant ids mismatch")
    require({item["id"] for item in mutants["excluded_attacks"]} == excluded_ids, "excluded attack ids mismatch")
    require(all(item.get("expected") == "Violated" and item.get("witness") for item in contract["negative_cases"]), "contract negative without witness")

    positive = repeated_vectors(oracles["positive_cases"], classify_positive)
    mutation = repeated_vectors(
        mutants["valid_mutants"],
        lambda item: classify_mutant(item, positive_ids),
    )
    opaque = repeated_vectors(oracles["opaque_cases"], classify_opaque)
    excluded = repeated_vectors(
        mutants["excluded_attacks"],
        lambda item: classify_excluded(item, opaque_ids),
    )
    require(all(item["classification"] == "Preserved" for item in positive), "positive not Preserved")
    require(all(item["classification"] == "Violated" and item["witnesses"] for item in mutation), "mutant survived or lacks witness")
    require(all(item["classification"] == "Unknown" and item["success_credit"] == 0 for item in opaque), "opaque classification/credit mismatch")
    require(all(item["classification"] == "Unknown" and not item["included_in_mutation_denominator"] for item in excluded), "excluded attack mismatch")

    require(oracles["counts"] == {
        "positive": 22,
        "explicit_opaque": 8,
        "expected_preserved": 22,
        "expected_unknown": 8,
        "unexpected_unknown_budget": 0,
    }, "oracle declared counts mismatch")
    require(mutants["counts"] == {
        "valid_mutants": 22,
        "expected_violated": 22,
        "excluded_out_of_scope_attacks": 3,
        "required_mutation_score": 1.0,
    }, "mutant declared counts mismatch")
    require(manifest["unknown_policy"]["default"] == "fail_closed", "Unknown policy is not fail closed")
    require(manifest["mutation_policy"]["required_mutation_score"] == 1.0, "required score changed")
    require(contract["mutation_gate"]["unexpected_unknown"].startswith("Zero credit"), "Unknown received success credit")
    require(contract["determinism_gate"]["repetitions_per_order"] == 3, "determinism repetitions changed")

    safety = manifest["execution_safety"]
    require(safety["forbidden_processes"] == ["typst watch", "typst w", "watch", "w"], "watch/w prohibition changed")
    require(safety["cargo_build_jobs"] == 1, "Cargo jobs must be one")
    require(safety["timeout_seconds_max"] <= 120, "timeout exceeds 120 seconds")
    require(safety["max_bytes_per_stream"] <= MAX_INPUT_BYTES, "stream limit exceeds 4 MiB")
    require(safety["max_bytes_all_receipt_streams"] <= MAX_TOTAL_BYTES, "total stream limit exceeds 16 MiB")
    require(oracles["execution_safety"]["processes_spawned"] == 0, "oracle suite reports processes")
    require(oracles["execution_safety"]["watch_or_w_processes_spawned"] == 0, "oracle suite reports watch/w")
    require(oracles["execution_safety"]["cargo_commands_run"] == 0, "oracle suite reports Cargo")
    require(oracles["execution_safety"]["product_commands_run"] == 0, "oracle suite reports product")

    violated = sum(item["classification"] == "Violated" for item in mutation)
    score = violated / len(mutation)
    require(score == 1.0, "mutation score below 1.0")
    report = {
        "schema": "tekt-static-contract-gate-result/v1",
        "step": "P1362",
        "contract_id": contract["contract_id"],
        "verdict": "seal-static-contract",
        "scope": "pre-implementation binding-free discrimination of the frozen fragment only",
        "raw_hashes": raw_hashes,
        "normative_hashes": normative_hashes,
        "prompt_reseal": prompt_reseal,
        "classification": {
            "positive_preserved": len(positive),
            "valid_mutants_violated": violated,
            "valid_mutants_survived": 0,
            "unexpected_unknown": 0,
            "opaque_unknown": len(opaque),
            "opaque_success_credit": 0,
            "excluded_attacks_unknown": len(excluded),
            "excluded_attacks_in_denominator": 0,
            "mutation_score": score,
        },
        "mutation_witnesses": {item["id"]: item["witnesses"] for item in mutation},
        "determinism": {
            "canonical_repetitions": 3,
            "reverse_repetitions": 3,
            "classification_witness_vectors_identical": True,
        },
        "safety": {
            "subprocesses_created_by_gate": 0,
            "cargo_product_tests_executed": False,
            "watch_or_w_executed": False,
            "input_bytes_observed": observed_total,
            "input_bytes_each_limit": MAX_INPUT_BYTES,
            "input_bytes_total_limit": MAX_TOTAL_BYTES,
        },
        "attestation": "executado sem atestacao de isolamento",
    }
    print(json.dumps(report, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (AssertionError, KeyError, TypeError, ValueError, UnicodeError) as error:
        print(json.dumps({"verdict": "do-not-seal", "error": str(error)}, sort_keys=True, separators=(",", ":")))
        sys.exit(1)
