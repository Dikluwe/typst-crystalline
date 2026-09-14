#!/usr/bin/env python3
"""Gate estatico e deterministico do contrato P1361.

Este programa le somente os artefatos congelados do contrato. Ele nao importa
codigo do produto, nao executa Cargo/testes/binarios e nao inicia processos.
"""

from __future__ import annotations

import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"

PATHS = {
    "manifest": DIAG / "p1361-manifest.json",
    "contract": DIAG / "p1361-contract.json",
    "oracles": DIAG / "p1361-oracles.json",
    "mutants": DIAG / "p1361-mutants.json",
}

EXPECTED = {
    "manifest": "10c0d40a0e8dc6fc4507dd5cb9675018a7f55bf100e559e72916d084e051ac40",
    "contract": "e0362460eceb7b74232463a694ee4c0dada9cfcd50ad2f724db4d46887e54ced",
    "oracles": "92ca8560fdb6714e23957ce0f0b74da1baeeb2adf13e383a663d0852b7de48e5",
    "mutants": "1153d68cff2cba129e1aed9f57276f52ff80f5430088617c1ca204230de82315",
}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def frozen_path(path: str) -> Path:
    candidate = (ROOT / path).resolve()
    require(candidate == ROOT or ROOT in candidate.parents, f"path fora do repositorio: {path}")
    return candidate


def main() -> None:
    actual = {name: sha256(path) for name, path in PATHS.items()}
    require(actual == EXPECTED, f"pins principais divergiram: {actual}")

    manifest = load(PATHS["manifest"])
    contract = load(PATHS["contract"])
    oracles = load(PATHS["oracles"])
    mutants = load(PATHS["mutants"])

    # Linhagem e congelamento.
    require(contract["manifest"]["sha256"] == EXPECTED["manifest"], "contrato nao pina manifesto")
    require(oracles["pins"]["manifest"]["sha256"] == EXPECTED["manifest"], "oraculos nao pinam manifesto")
    require(oracles["pins"]["contract"]["sha256"] == EXPECTED["contract"], "oraculos nao pinam contrato")
    require(mutants["manifest_sha256"] == EXPECTED["manifest"], "mutantes nao pinam manifesto")
    require(mutants["contract_sha256"] == EXPECTED["contract"], "mutantes nao pinam contrato")
    frozen = manifest["frozen_inputs"]
    require(len(frozen) == 6, "conjunto congelado inesperado")
    for item in frozen:
        require(sha256(frozen_path(item["path"])) == item["sha256"], f"input congelado divergiu: {item['path']}")
    require(oracles["pins"]["execution_step_sha256"] == frozen[0]["sha256"], "pin do passo divergiu")
    require(oracles["pins"]["preflight_sha256"] == frozen[1]["sha256"], "pin do preflight divergiu")
    require(oracles["pins"]["owner_prompt_sha256"] == frozen[2]["sha256"], "pin do owner divergiu")
    require(oracles["pins"]["crate_index_prompt_sha256"] == frozen[3]["sha256"], "pin do indice divergiu")
    require(oracles["pins"]["integration_test_owner_prompt_sha256"] == frozen[4]["sha256"], "pin do owner de teste divergiu")
    require(oracles["pins"]["vanilla_source_and_test_sha256"] == frozen[5]["sha256"], "pin vanila divergiu")
    require(manifest["baseline"]["commit"] == oracles["pins"]["vanilla_commit"], "commit vanila divergiu")
    require(manifest["phase"] == "lineage-reopening-after-first-green", "fase retrospectiva ausente")
    require(manifest["semantic_obligations_changed_by_revision_3"] is False, "reabertura semantica inesperada")
    lineage = contract["lineage_revision"]
    require(lineage["revision"] == 3 and lineage["prior_seal_invalidated"] is True, "linhagem anterior nao invalidada")
    require(lineage["productive_owner_metadata_corrected_after_first_candidate_and_first_green"] is True, "timing do owner produtivo ausente")
    require(lineage["integration_test_owner_created_after_first_candidate_and_first_green"] is True, "timing do owner de teste ausente")
    require(oracles["verdict"]["eligible_for_prospective_seal"] is False, "oraculos permitem selo prospectivo")
    require(oracles["verdict"]["eligible_for_retrospective_limited_verification"] is True, "oraculos bloqueiam verificacao retrospectiva")
    require(mutants["lineage_revision"]["prospective_seal_available"] is False, "mutantes permitem selo prospectivo")
    require(mutants["lineage_revision"]["retrospective_limited_verification_available"] is True, "mutantes bloqueiam verificacao retrospectiva")

    # Vetores observaveis e opacidade fail-closed.
    positives = contract["positive_cases"]
    oracle_positives = oracles["positive_cases"]
    require(len(positives) == 22 and len(oracle_positives) == 22, "sao exigidos 22 positivos")
    positive_ids = [case["id"] for case in positives]
    require(len(set(positive_ids)) == 22, "ids positivos duplicados")
    contract_vector = [(x["id"], x["secs"], x["nanos"], x["expected"]) for x in positives]
    oracle_vector = [(x["id"], x["secs"], x["nanos"], x["expected"]) for x in oracle_positives]
    require(contract_vector == oracle_vector, "vetor positivo diverge entre contrato e oraculos")
    require(contract["acceptance_vector"]["positives"] == {"count": 22, "required": "Preserved"}, "aceitacao positiva invalida")

    opaque = contract["opaque_cases"]
    oracle_opaque = oracles["opaque_cases"]
    require(len(opaque) == 7 and len(oracle_opaque) == 7, "sao exigidos 7 opacos")
    require(all(x["required_result"] == "Unknown" for x in opaque), "opaco sem Unknown")
    require(all(x["expected_classification"] == "Unknown" and x["acceptance_credit"] == 0 for x in oracle_opaque), "Unknown recebeu credito")
    require(manifest["unknown_policy"]["acceptance_credit"] == 0, "politica Unknown concede credito")
    require(manifest["unknown_policy"]["unexpected_unknown_blocks_pass"] is True, "Unknown inesperado nao bloqueia")

    # Mutacoes: 22 validas, todas Violated e com testemunha pertencente ao
    # vetor/estrutura; nenhuma classificacao Unknown entra no denominador.
    negative = contract["negative_cases"]
    valid = [x for x in mutants["mutants"] if x.get("valid") is True]
    require(len(negative) == 22 and len(valid) == 22, "sao exigidos 22 negativos/mutantes validos")
    require([x["id"] for x in valid] == [f"M{i:02d}" for i in range(1, 23)], "ids mutantes invalidos")
    require(all(x["required"] == "Violated" for x in valid), "mutante valido nao classificado Violated")
    witness_domain = set(positive_ids) | {x["id"] for x in contract["structural_requirements"]}
    require(all(x["witnesses"] and set(x["witnesses"]) <= witness_domain for x in valid), "mutante sem testemunha valida")
    negative_by_id = {x["id"]: x for x in negative}
    require(all(x["contract_negative"] in negative_by_id for x in valid), "mutante sem negativo contratual")
    require(all(negative_by_id[x["contract_negative"]]["required_result"] == "Violated" for x in valid), "negativo nao exige Violated")
    require(all(set(x["witnesses"]) & set(negative_by_id[x["contract_negative"]]["witnesses"]) for x in valid), "testemunha nao vinculada ao negativo")

    excluded = mutants["excluded_equivalent_mutants"]
    require(len(excluded) == 1, "exclusoes equivalentes inesperadas")
    e01 = excluded[0]
    require(e01["id"] == "E01" and e01["valid"] is False, "E01 deve estar excluida")
    require(e01["reason_code"] == "EQUIVALENT_AT_EXACT_THRESHOLD" and e01["acceptance_credit"] == 0, "E01 invalida")
    require(e01["witnesses"] == [], "mutacao equivalente nao pode ter testemunha")
    m07 = next(x for x in valid if x["id"] == "M07")
    b04 = next(x for x in positives if x["id"] == "B04")
    require(m07["witnesses"] == ["B04"], "M07 deve ser morto por B04")
    require((b04["secs"], b04["nanos"], b04["expected"]) == (300, 6_000_000, "5 min 0 s"), "B04 nao discrimina 300.010 s")
    require(mutants["witness_coverage"]["all_valid_mutants_have_at_least_one_witness"] is True, "cobertura de testemunhas falsa")
    require(mutants["witness_coverage"]["opaque_unknowns_never_enter_denominator"] is True, "opacos entraram no denominador")
    require(mutants["target_mutation_score"] == 1.0, "alvo de mutacao nao e 1.0")
    score = sum(x["required"] == "Violated" for x in valid) / len(valid)
    require(score == 1.0, "score estatico inferior a 1.0")

    # Determinismo e reordenacao sao obrigações explícitas e completas.
    det = contract["determinism"]
    require(det["repeat_count"] == 3 and len(det["orders"]) == 2, "plano de determinismo incompleto")
    canonical = det["orders"][0]["case_ids"]
    reversed_ids = det["orders"][1]["case_ids"]
    require(canonical == positive_ids, "ordem canonica incompleta")
    require(reversed_ids == list(reversed(positive_ids)), "reordenacao nao e reversao exata")
    require(oracles["determinism"]["repeat_count"] == 3, "oraculos nao repetem tres vezes")

    # Contenção de memoria e proibição de processo aberto.
    limits = manifest["execution_limits"]
    policy = contract["execution_policy"]
    require(limits["watch_and_w_processes"] == "prohibited", "watch/w nao proibidos no manifesto")
    require(policy["watch_or_w_process"] == "prohibited", "watch/w nao proibidos no contrato")
    require(oracles["execution_policy"]["watch_or_w_process"] == "prohibited", "watch/w nao proibidos nos oraculos")
    require(mutants["execution_policy"]["watch_or_w_process"] == "prohibited", "watch/w nao proibidos nos mutantes")
    require(limits["cargo_build_jobs"] == policy["cargo_build_jobs"] == 1, "jobs deve ser 1")
    require(limits["command_timeout_seconds_max"] == policy["timeout_seconds_max"] == 120, "timeout deve ser <= 120 s")
    require(limits["receipt_bytes_max_each"] == policy["receipt_bytes_max_each"] == 4_194_304, "limite por recibo invalido")
    require(limits["receipt_bytes_max_total"] == policy["receipt_bytes_max_total"] == 16_777_216, "limite total invalido")
    require(limits["workspace_or_broad_filters"] == "prohibited", "suite ampla nao proibida")
    require(policy["broad_or_workspace_test"] == "prohibited", "teste amplo nao proibido")
    require(mutants["execution_policy"]["execution_performed"] is False, "suite declara execucao indevida")

    result = {
        "schema": "tekt-static-contract-gate-result/v1",
        "step": "P1361",
        "verdict": "PASS_RETROSPECTIVE_LIMITED",
        "seal_kind": "retrospective-limited-seal",
        "prospective": False,
        "mutation_score": score,
        "positive_cases": len(positives),
        "valid_mutants": len(valid),
        "valid_mutants_required_violated": len(valid),
        "excluded_equivalent": [e01["id"]],
        "opaque_unknown": len(opaque),
        "opaque_acceptance_credit": 0,
        "blocking_findings": 0,
        "execution": "static-only; no Cargo, product, tests, watch or w",
        "isolation_attestation": "executado sem atestacao de isolamento",
    }
    print(json.dumps(result, ensure_ascii=False, sort_keys=True))


if __name__ == "__main__":
    main()
