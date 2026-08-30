#!/usr/bin/env python3
"""Deterministic preseal gate for P1246's frozen clip contract.

This verifier deliberately consumes only the pre-code contract corpus, the owner
decision, the L0 prompt, the tactical step, the capability declaration, and the
ratified vanilla baseline.  It never consumes a candidate implementation.
"""

from __future__ import annotations

import csv
import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
INPUTS = {
    "contract": ROOT / "00_nucleo/diagnosticos/p1246-clip-contract.tsv",
    "oracles": ROOT / "00_nucleo/diagnosticos/p1246-clip-oracles.tsv",
    "attacks": ROOT / "00_nucleo/diagnosticos/p1246-clip-attacks.tsv",
    "owner_decision": ROOT / "00_nucleo/diagnosticos/p1246-owner-decision.tsv",
    "role_capabilities": ROOT / "00_nucleo/diagnosticos/p1246-role-capabilities.tsv",
    "step": ROOT / "typst-passo-1246.md",
    "prompt": ROOT / "00_nucleo/prompts/infra/export/svg.md",
    "baseline": ROOT / "lab/typst-original/crates/typst-svg/src/lib.rs",
}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def rows(name: str) -> list[dict[str, str]]:
    with INPUTS[name].open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def require(condition: bool, message: str, failures: list[str]) -> None:
    if not condition:
        failures.append(message)


def evaluate(order: str) -> dict[str, object]:
    failures: list[str] = []
    contract = rows("contract")
    oracles = rows("oracles")
    attacks = rows("attacks")
    owner = rows("owner_decision")
    capabilities = rows("role_capabilities")
    if order == "reverse":
        contract.reverse()
        oracles.reverse()
        attacks.reverse()

    contract_by_id = {row["id"]: row for row in contract}
    require(len(contract) == 12 and len(contract_by_id) == 12, "contract IDs/count", failures)
    require(set(contract_by_id) == {f"C{i:02d}" for i in range(1, 13)}, "contract ID domain", failures)
    for row in contract:
        require(all(row.get(k, "").strip() for k in ("input", "observable", "Preserved", "Violated", "Unknown")), f"{row['id']} complete trichotomy", failures)

    positive = [row for row in oracles if row["kind"] == "positive"]
    opaque = [row for row in oracles if row["kind"] == "opaque"]
    require(len(oracles) == 10 and len(positive) == 7 and len(opaque) == 3, "oracle partition 7 positive/3 opaque", failures)
    require({row["id"] for row in oracles} == {f"O{i:02d}" for i in range(1, 11)}, "oracle ID domain", failures)
    covered: set[str] = set()
    oracle_results: dict[str, str] = {}
    for row in oracles:
        refs = row["contract_rows"].split(",")
        require(all(ref in contract_by_id for ref in refs), f"{row['id']} contract references", failures)
        covered.update(refs)
        expected = row["expected_result"].split(":", 1)[0]
        wanted = "Preserved" if row["kind"] == "positive" else "Unknown"
        require(expected == wanted or expected.startswith(wanted + " "), f"{row['id']} must classify {wanted}", failures)
        require(bool(row["witness"].strip()), f"{row['id']} witness", failures)
        oracle_results[row["id"]] = expected
    require(covered == set(contract_by_id), "oracles cover every contract row", failures)

    attack_results: dict[str, str] = {}
    valid_attacks: list[dict[str, str]] = []
    require(len(attacks) == 12 and {row["id"] for row in attacks} == {f"A{i:02d}" for i in range(1, 13)}, "attack IDs/count", failures)
    for row in attacks:
        valid = (
            row["valid_for_mutation_score"] == "YES_IF_EXECUTED"
            and row["contract_id"] in contract_by_id
            and bool(row["semantic_negative_mutation"].strip())
            and bool(row["witness"].strip())
        )
        if valid:
            valid_attacks.append(row)
            result = row["expected_result"].strip()
            require(result == "Violated", f"{row['id']} negative mutation rejected", failures)
            attack_results[row["id"]] = result
    require({row["contract_id"] for row in valid_attacks} == set(contract_by_id), "one scored negative mutation per contract row", failures)
    rejected = sum(attack_results.get(row["id"]) == "Violated" for row in valid_attacks)
    score = rejected / len(valid_attacks) if valid_attacks else 0.0
    require(score == 1.0, "mutation score must equal 1.0", failures)

    require(len(owner) == 1 and owner[0]["owner_decision"] == "APPROVED", "owner approval", failures)
    require(owner and owner[0]["implementation_gate"] == "segregated preseal before code", "owner preseal gate", failures)
    prompt_text = INPUTS["prompt"].read_text(encoding="utf-8")
    step_text = INPUTS["step"].read_text(encoding="utf-8")
    baseline_text = INPUTS["baseline"].read_text(encoding="utf-8")
    require("## P1246 — clip geométrico local proposto" in prompt_text, "P1246 L0 section", failures)
    require("PRESEAL SEGREGADO PENDENTE" in step_text, "step preseal state", failures)
    require(all(token in baseline_text for token in ("clip_paths", 'attr("clip-path"', 'elem("clipPath")')), "ratified baseline clip graph evidence", failures)
    require(any(row.get("role") == "verifier_publisher" for row in capabilities), "verifier capability declaration", failures)

    return {
        "attack_results": dict(sorted(attack_results.items())),
        "failures": sorted(set(failures)),
        "mutation_denominator": len(valid_attacks),
        "mutation_rejected": rejected,
        "mutation_score": score,
        "oracle_results": dict(sorted(oracle_results.items())),
    }


def main() -> int:
    normal = evaluate("normal")
    reverse = evaluate("reverse")
    failures = list(normal["failures"])
    require(normal == reverse, "reordering changes gate result", failures)
    result = {
        "gate": "P1246_PRESEAL_V2",
        "input_sha256": {name: sha256(path) for name, path in sorted(INPUTS.items())},
        "mutation_denominator": normal["mutation_denominator"],
        "mutation_rejected": normal["mutation_rejected"],
        "mutation_score": normal["mutation_score"],
        "oracle_results": normal["oracle_results"],
        "reordering_invariant": normal == reverse,
        "verdict": "PRESEAL" if not failures and normal["mutation_score"] == 1.0 else "BLOCKED",
        "violations": sorted(set(failures)),
    }
    print(json.dumps(result, ensure_ascii=False, sort_keys=True, separators=(",", ":")))
    return 0 if result["verdict"] == "PRESEAL" else 1


if __name__ == "__main__":
    raise SystemExit(main())
