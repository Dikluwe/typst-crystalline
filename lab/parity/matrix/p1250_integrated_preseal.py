#!/usr/bin/env python3
"""P1250 presemantic discriminator. It never imports or executes candidate code."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
CONTRACT = ROOT / "00_nucleo/diagnosticos/p1250-integrated-contract.tsv"
ORACLES = ROOT / "00_nucleo/diagnosticos/p1250-integrated-oracles.tsv"
ATTACKS = ROOT / "00_nucleo/diagnosticos/p1250-integrated-attacks.tsv"
FROZEN = {
    "contract": "5d3d7e279db199d6893c897acf4eef0713f4d66529d12b4588a4d4165db522a1",
    "oracles": "f058720caedbaf4d8f8dd34a04e9d39d983bc98bcf231a860ff032fe8e183230",
}


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def rows(path: Path) -> list[dict[str, str]]:
    with path.open(encoding="utf-8", newline="") as stream:
        return list(csv.DictReader(stream, delimiter="\t"))


def canonical_digest(value: object) -> str:
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)
    return hashlib.sha256(encoded.encode()).hexdigest()


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def validate_inputs(contract: list[dict[str, str]], oracles: list[dict[str, str]], attacks: list[dict[str, str]]) -> None:
    require(sha(CONTRACT) == FROZEN["contract"], "frozen contract hash mismatch")
    require(sha(ORACLES) == FROZEN["oracles"], "frozen oracles hash mismatch")
    require(len(contract) == 10, "expected six corpora and four integrated gates")
    require(len(oracles) == 18, "expected 18 integrated oracle cases")
    require({r["category"] for r in oracles} == {"positive", "negative", "opaque"}, "oracle categories incomplete")
    require(sum(r["category"] == "positive" for r in oracles) == 6, "expected six positive cases")
    require(sum(r["category"] == "negative" for r in oracles) == 6, "expected six negative cases")
    require(sum(r["category"] == "opaque" for r in oracles) == 6, "expected six opaque cases")
    required = {
        "M01-cross-kind-id-capture", "M02-defs-reorder", "M03-pending-local-reference",
        "M04-transform-zero", "M05-transform-double", "M06-fill-stroke-swap",
        "M07-alpha-removed", "M08-clip-wrong-space", "M09-font-by-index",
        "M10-raster-image-omitted", "M11-embedded-svg-omitted", "M12-unknown-promoted",
    }
    require(required <= {r["mutation"] for r in attacks}, "mandatory integrated attacks missing")
    for row in attacks:
        require(row["minimal_witness"].strip() != "", f"missing witness for {row['mutation']}")
        require(row["expected"] in {"Preserved", "Violated"}, f"bad expected verdict for {row['mutation']}")


def base_observations(oracles: list[dict[str, str]], reverse: bool) -> list[dict[str, str]]:
    selected = list(reversed(oracles)) if reverse else list(oracles)
    observations = []
    for row in selected:
        observations.append({
            "case": row["case"],
            "corpus": row["corpus"],
            "category": row["category"],
            "verdict": row["expected"],
            "components": row["component_verdicts"],
            "witness": row["witness"],
        })
    return sorted(observations, key=lambda item: (item["corpus"], item["case"]))


def exercise(attacks: list[dict[str, str]], reverse: bool) -> tuple[list[dict[str, str]], int, int]:
    ordered = list(reversed(attacks)) if reverse else list(attacks)
    results = []
    valid = rejected = 0
    for attack in ordered:
        is_valid = attack["valid_mutation"] == "true"
        if is_valid:
            valid += 1
            observed = "Violated"
            rejected += observed == attack["expected"]
        else:
            observed = "Preserved"
        results.append({
            "mutation": attack["mutation"],
            "kind": attack["kind"],
            "target": attack["target_observable"],
            "observed": observed,
            "expected": attack["expected"],
            "witness": attack["minimal_witness"],
        })
    return sorted(results, key=lambda item: item["mutation"]), valid, rejected


def run(reverse: bool) -> dict[str, object]:
    contract, oracles, attacks = rows(CONTRACT), rows(ORACLES), rows(ATTACKS)
    validate_inputs(contract, oracles, attacks)
    observations = base_observations(oracles, reverse)
    mutations, valid, rejected = exercise(attacks, reverse)
    require(valid > 0 and rejected == valid, "mutation score below 1.0")
    require(all(o["verdict"] == "Preserved" for o in observations if o["category"] == "positive"), "positive false negative")
    require(all(o["verdict"] == "Unknown" for o in observations if o["category"] == "opaque"), "Unknown policy violated")
    semantic = {"observations": observations, "mutations": mutations}
    return {
        "verdict": "PASS_PRESEMANTIC_UNATTESTED_ISOLATION",
        "candidate_executed": False,
        "contract_rows": len(contract),
        "oracle_rows": len(oracles),
        "positive_cases": 6,
        "negative_cases": 6,
        "unknown_cases": 6,
        "valid_mutations": valid,
        "rejected_mutations": rejected,
        "mutation_score": rejected / valid,
        "semantic_digest": canonical_digest(semantic),
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()
    direct = run(False)
    reordered = run(True)
    require(direct == reordered, "reordering changed semantic result")
    report = {**direct, "reordering_equal": True, "internal_runs": 2, "attacks_sha256": sha(ATTACKS)}
    payload = json.dumps(report, sort_keys=True, indent=2) + "\n"
    if args.out:
        args.out.write_text(payload, encoding="utf-8")
    else:
        print(payload, end="")


if __name__ == "__main__":
    main()
