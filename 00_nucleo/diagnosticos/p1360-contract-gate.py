#!/usr/bin/env python3
"""Gate estatico, deterministico e pre-implementacao do contrato P1360.

Este programa le somente os artefatos documentais protegidos de P1360. Ele nao
le codigo produtivo, testes, diffs, binarios ou artefatos candidatos, e nao cria
processos. O resultado mede discriminacao binding-free do contrato; nao mede
uma implementacao.
"""

import hashlib
import json
import sys


ROOT = "/repos/Antigravity/typst-crystalline/"
CONTRACT = "00_nucleo/diagnosticos/p1360-contract.json"
MANIFEST = "00_nucleo/diagnosticos/p1360-manifest.json"
ORACLES = "00_nucleo/diagnosticos/p1360-oracles.json"
MUTANTS = "00_nucleo/diagnosticos/p1360-mutants.json"
PREFLIGHT = "00_nucleo/diagnosticos/p1360-preflight.md"
STEP = "00_nucleo/materialization/typst-passo-1360.md"
L0_PATHS = [
    "00_nucleo/prompts/shell/cli.md",
    "00_nucleo/prompts/wiring.md",
    "00_nucleo/prompts/infra/pipeline.md",
    "00_nucleo/prompts/infra/export/mod.md",
    "00_nucleo/prompts/infra/export/builder.md",
    "00_nucleo/prompts/infra/export/html.md",
    "00_nucleo/prompts/infra/export/svg.md",
]
READ_ALLOWLIST = [CONTRACT, MANIFEST, ORACLES, MUTANTS, PREFLIGHT, STEP, *L0_PATHS]
MAX_PROTECTED_INPUT_BYTES = 4 * 1024 * 1024
MAX_TOTAL_PROTECTED_BYTES = 16 * 1024 * 1024
EXPECTED_HASHES = {
    CONTRACT: "f4bd6f61c773106fdaf7f239ec7c4a21102b91e8d22505d02bda30b99107f3bd",
    MANIFEST: "5af38e0142b5d6453568c64c2da091d8961e4eff1eeb08154354891cc4677cbb",
    ORACLES: "040a66e1fa2457db06450b74d7ac9cbb05ab852c670e24a588f784fac78a460c",
    MUTANTS: "edc5c917520b106867d9fecb870263c988872295eaa4dc1af96c03439601473d",
    PREFLIGHT: "ade428efee0bc965eff01f670b755c87f4b8a0bbc2bdbc034214da15dfd72f85",
    STEP: "8f86022e615227e759cb5e61334965829b348824d41dfaa7c74ea6c2d30e4207",
    L0_PATHS[0]: "698bd37b02542ce470af0c9b645c33f1015a97a77cc822681c4ea7a39a4fc661",
    L0_PATHS[1]: "058e6053e98a77f5570e78ba858259617bc40d7d09e234b6025c33a32a2a278e",
    L0_PATHS[2]: "00602f723df6d1570337461a37c2505ebe5bbbd2358c9092291d0192bf6696ed",
    L0_PATHS[3]: "b63daea9a069c860ac6e33daba8bb37a4a3fde85199a334bbe4b44600fcc7696",
    L0_PATHS[4]: "29f9f4b546b107a308c0f07e158e8da6d6cb86416ed8e180d9b0c68ba4d9643a",
    L0_PATHS[5]: "890dbf55d8aa757aaee30816e69876215c25843e0aa2f4497cad599021ed66d1",
    L0_PATHS[6]: "58f64423d35533e41ab7187462ff17e00769be60412d7732789312b3baf4d145",
}


def read_bytes(path):
    if path not in READ_ALLOWLIST:
        raise AssertionError("read outside allowlist: " + path)
    with open(ROOT + path, "rb") as handle:
        data = handle.read(MAX_PROTECTED_INPUT_BYTES + 1)
    require(len(data) <= MAX_PROTECTED_INPUT_BYTES, "protected input exceeds 4 MiB: " + path)
    return data


def load_json(path):
    return json.loads(read_bytes(path).decode("utf-8"))


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def classify_mutant(mutant, oracle_families):
    # A mutacao e rejeitada apenas quando e valida, decidivel, tem testemunha
    # publica e aponta para ao menos um oraculo/safety gate congelado.
    killed_by = mutant.get("killed_by", [])
    known_killer = any(k in oracle_families or k == "safety-policy" for k in killed_by)
    decidable = (
        mutant.get("valid") is True
        and mutant.get("expected") == "Violated"
        and bool(mutant.get("witness"))
        and bool(mutant.get("reason_code"))
        and known_killer
    )
    return {
        "id": mutant.get("id"),
        "classification": "Violated" if decidable else "Unknown",
        "reason_code": mutant.get("reason_code") if decidable else "UNKNOWN_STATIC_DISCRIMINATION",
    }


def main():
    blobs = {path: read_bytes(path) for path in READ_ALLOWLIST}
    require(sum(map(len, blobs.values())) <= MAX_TOTAL_PROTECTED_BYTES, "protected input set exceeds 16 MiB")
    observed_hashes = {path: sha256(data) for path, data in blobs.items()}
    require(observed_hashes == EXPECTED_HASHES, "protected input hash mismatch")

    contract = load_json(CONTRACT)
    manifest = load_json(MANIFEST)
    oracles = load_json(ORACLES)
    mutants_doc = load_json(MUTANTS)

    contract_id = "P1360-compile-pretty"
    require(contract["contract_id"] == contract_id, "contract id mismatch")
    require(manifest["manifest_id"] == contract_id, "manifest id mismatch")
    require(oracles["contract_id"] == contract_id, "oracle contract id mismatch")
    require(mutants_doc["contract_id"] == contract_id, "mutant contract id mismatch")
    require(oracles["protected_inputs"]["manifest"]["sha256"] == observed_hashes[MANIFEST], "oracle manifest pin mismatch")
    require(oracles["protected_inputs"]["contract"]["sha256"] == observed_hashes[CONTRACT], "oracle contract pin mismatch")
    require(mutants_doc["protected_inputs"]["manifest_sha256"] == observed_hashes[MANIFEST], "mutant manifest pin mismatch")
    require(mutants_doc["protected_inputs"]["contract_sha256"] == observed_hashes[CONTRACT], "mutant contract pin mismatch")
    require(mutants_doc["oracle_suite"] == ORACLES, "mutant oracle path mismatch")

    manifest_l0 = [(item["path"], item["sha256"]) for item in manifest["prompt_roots"]]
    oracle_l0 = [tuple(item) for item in oracles["protected_inputs"]["l0_hashes"]]
    expected_l0 = [(path, EXPECTED_HASHES[path]) for path in L0_PATHS]
    require(manifest_l0 == expected_l0 == oracle_l0, "L0 order/hash isomorphism mismatch")
    contract_inputs = {item["path"]: item["sha256"] for item in contract["frozen_intent"]["input_hashes"]}
    for path in [PREFLIGHT, STEP, *L0_PATHS]:
        require(contract_inputs.get(path) == EXPECTED_HASHES[path], "contract input pin mismatch: " + path)

    required = [item["id"] for item in contract["required_mutations"]]
    require(required == [f"M{i:02d}" for i in range(1, 20)], "contract must require M01-M19 in order")
    coverage = mutants_doc["coverage"]["contract_required_mutations"]
    require(list(coverage) == required, "coverage keys are not isomorphic with M01-M19")

    mutant_rows = mutants_doc["mutants"]
    mutant_ids = [item["id"] for item in mutant_rows]
    require(len(mutant_rows) == len(set(mutant_ids)) == 36, "mutation suite must contain 36 unique mutants")
    require(coverage["M03"] == ["M03A", "M03B"], "M03 must split deterministically into M03A and M03B")
    covered_ids = [item for key in required for item in coverage[key]]
    require(len(covered_ids) == len(set(covered_ids)) == 34, "M01-M19 expansion must contain 34 unique mutants")
    require(set(covered_ids).issubset(set(mutant_ids)), "coverage names a missing mutant")
    require(set(mutant_ids) - set(covered_ids) == {"M20", "M21"}, "unexpected supplemental mutants")
    require(mutants_doc["coverage"]["valid_mutant_count"] == 36, "declared mutant count mismatch")
    require(mutants_doc["coverage"]["required_mutation_classes_covered"] == 19, "declared class count mismatch")
    require(mutants_doc["coverage"]["uncovered_required_mutation_classes"] == [], "required class uncovered")

    positive = oracles["positive_cases"]
    negative = oracles["negative_cases"]
    opaque = oracles["opaque_cases"]
    require(sum(item["count"] for item in positive) == 43, "positive count mismatch")
    require(sum(item["count"] for item in negative) == 6, "negative count mismatch")
    require(len(opaque) == 10, "opaque count mismatch")
    require(all(item["expected"] == "Preserved" for item in positive), "positive not Preserved")
    require(all(item["expected"].startswith("Preserved") for item in negative), "required rejection not Preserved")
    require(all(item["expected"] == "Unknown" and item.get("reason_code") for item in opaque), "opaque not Unknown")
    require(oracles["result_domain"]["unknown_policy"]["success_credit"] == 0, "Unknown received credit")

    oracle_families = {item["family"] for item in positive + negative}
    oracle_families.update(item["id"] for item in opaque)
    forward_1 = [classify_mutant(item, oracle_families) for item in mutant_rows]
    forward_2 = [classify_mutant(item, oracle_families) for item in mutant_rows]
    reverse = [classify_mutant(item, oracle_families) for item in reversed(mutant_rows)]
    require(forward_1 == forward_2, "same-order repetition changed classification vector")
    require(forward_1 == list(reversed(reverse)), "reverse order changed classification vector")
    require(all(item["classification"] == "Violated" for item in forward_1), "surviving or Unknown mutant")

    killed = sum(item["classification"] == "Violated" for item in forward_1)
    score = killed / len(forward_1)
    require(score == 1.0, "mutation score below 1.0")
    require(contract["execution_safety"]["watch_subprocess"] == "forbidden", "watch subprocess not forbidden")
    require(contract["execution_safety"]["w_subprocess"] == "forbidden", "w subprocess not forbidden")
    require(contract["execution_safety"]["allowed_product_subcommand"] == "compile only", "product scope not compile-only")
    require(contract["execution_safety"]["process_timeout_seconds_max"] <= 20, "timeout exceeds 20 seconds")
    require(contract["execution_safety"]["artifact_size_bytes_max"] <= 4194304, "artifact bound exceeds 4 MiB")
    require(oracles["execution_safety"]["watch"].startswith("parser-only"), "watch is not parser-only")
    require(oracles["execution_safety"]["w"].startswith("parser-only"), "w is not parser-only")

    l0_pairs = "\n".join(path + ":" + observed_hashes[path] for path in L0_PATHS).encode("utf-8")
    report = {
        "schema": "tekt-static-contract-gate-result/v1",
        "contract_id": contract_id,
        "verdict": "seal-static-contract",
        "scope": "pre-implementation binding-free contract discrimination only",
        "hashes": observed_hashes,
        "l0_set_sha256": sha256(l0_pairs),
        "isomorphism": {
            "contract_manifest_oracles_mutants": "Preserved",
            "required_classes": 19,
            "mutants": 36,
        },
        "oracles": {"positive_preserved": 43, "negative_rejections_preserved": 6, "opaque_unknown": 10},
        "mutations": {"valid": 36, "violated": killed, "survived": 0, "unknown": 0, "score": score},
        "determinism": {"same_order_runs": 2, "reverse_order_runs": 1, "result": "Preserved"},
        "safety": {
            "subprocesses_created_by_gate": 0,
            "product_or_tests_executed": False,
            "protected_input_bytes_each_max": MAX_PROTECTED_INPUT_BYTES,
            "protected_input_bytes_total_max": MAX_TOTAL_PROTECTED_BYTES,
            "protected_input_bytes_observed": sum(map(len, blobs.values())),
            "watch_and_w": "parser-only policy; not executed by this static gate",
            "future_product_scope": "compile-only, one process, timeout<=20s, artifacts<=4194304 bytes",
        },
    }
    print(json.dumps(report, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (AssertionError, KeyError, TypeError, ValueError, UnicodeError) as error:
        print(json.dumps({"verdict": "do-not-seal", "error": str(error)}, sort_keys=True, separators=(",", ":")))
        sys.exit(1)
