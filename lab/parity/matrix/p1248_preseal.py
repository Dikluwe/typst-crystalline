#!/usr/bin/env python3
"""Deterministic presemantic gate for P1248; does not inspect candidate code."""

import argparse
import csv
import hashlib
import json
from pathlib import Path
from typing import Iterable


REQUIRED_OBSERVABLES = {
    "raster-content", "layout-box", "aspect-ratio", "orientation",
    "cover-clip", "embedded-svg", "media-type", "unknown-format",
}

# Each tuple is (canonical target, groups of witness terms). Every group must
# contribute at least one term. This makes rejection depend on the described
# semantic counterexample, not merely on a mutation label or expected column.
ATTACK_RULES = {
    "M01-mime-swapped": ("media-type", (("png",), ("image/jpeg",), ("declara", "mime"))),
    "M02-raster-alpha-lost": ("raster-content", (("alpha",), ("64",), ("255",))),
    "M03-raster-payload-replaced": ("raster-content", (("gif",), ("payload",), ("landmark",))),
    "M04-layout-box-rebased": ("layout-box", (("x=11",), ("y=17",), ("width=40",), ("height=24",))),
    "M05-fit-recomputed": ("aspect-ratio", (("stretch",), ("contain",), ("margens", "margin"))),
    "M06-cover-clip-removed": ("cover-clip", (("clip",), ("remove",), ("fora", "outside"))),
    "M07-cover-clip-wrong-base": ("cover-clip", (("clip_rect",), ("reaplica", "dupla"), ("x=210",))),
    "M08-orientation-ignored": ("orientation", (("exif 6",), ("ignora",), ("canto",))),
    "M09-orientation-double": ("orientation", (("exif 6",), ("novamente", "duas"), ("transform svg",))),
    "M10-embedded-svg-omitted": ("embedded-svg", (("svg",), ("nao emite", "omite"), ("landmark",))),
    "M11-embedded-svg-flattened-lossy": ("embedded-svg", (("svg",), ("achata",), ("alpha",), ("texto",))),
    "M12-unknown-promoted": ("unknown-format", (("bytes",), ("inventa",), ("preserved",), ("unknown",))),
    "M13-unknown-silent-omission": ("unknown-format", (("ambigu",), ("omite",), ("data-crystalline-image-fallback",))),
}

ORACLE_RULES = {
    "png-alpha-mime": ("raster", "semantic", "Preserved"),
    "jpeg-exif-6-asymmetric": ("raster", "semantic", "Preserved"),
    "gif-alpha-mime": ("raster", "semantic", "Preserved"),
    "webp-alpha-mime": ("raster", "semantic", "Preserved"),
    "resolved-stretch-box": ("raster", "semantic", "Preserved"),
    "cover-clip-local": ("raster", "semantic", "Preserved"),
    "nested-svg-red-blue-alpha": ("svg", "semantic", "Preserved"),
    "unknown-marker-no-image": ("unknown", "semantic", "Unknown"),
    "opaque-svg-external-dependency": ("svg", "semantic", "Unknown"),
    "opaque-svg-ambiguous-root": ("svg", "semantic", "Unknown"),
    "opaque-svg-compressed": ("svg", "semantic", "Unknown"),
    "determinism-repeat": ("meta", "meta", "Preserved"),
    "determinism-reorder": ("meta", "meta", "Preserved"),
}


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="") as source:
        return list(csv.DictReader(source, delimiter="\t"))


def require_fields(row: dict[str, str], fields: Iterable[str], identity: str) -> None:
    missing = [field for field in fields if not row.get(field, "").strip()]
    assert not missing, f"{identity}: empty fields {missing}"


def canonical_json(value: object) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def digest(value: object) -> str:
    return hashlib.sha256(canonical_json(value).encode()).hexdigest()


def validate_contract(contract: list[dict[str, str]]) -> dict[str, dict[str, str]]:
    assert len(contract) == len(REQUIRED_OBSERVABLES)
    by_observable = {row.get("observable", ""): row for row in contract}
    assert set(by_observable) == REQUIRED_OBSERVABLES
    assert len(by_observable) == len(contract), "duplicate observable"
    for observable, row in by_observable.items():
        require_fields(row, ("preserved", "violated", "unknown"), observable)
    assert "unknown" in by_observable["unknown-format"]["unknown"].casefold()
    assert "preserved" in by_observable["unknown-format"]["unknown"].casefold()
    return by_observable


def classify_oracle(row: dict[str, str]) -> dict[str, str]:
    require_fields(
        row,
        ("case", "kind", "expected", "witness", "fixture", "carrier", "executable_criterion"),
        row.get("case", "<oracle>"),
    )
    case = row["case"]
    assert case in ORACLE_RULES, f"unknown oracle {case}"
    kind, category, verdict = ORACLE_RULES[case]
    assert row["kind"] == kind, f"{case}: kind drift"
    assert row["expected"] == verdict, f"{case}: verdict drift"
    assert verdict in {"Preserved", "Unknown"}
    assert category == ("meta" if kind == "meta" else "semantic")
    if verdict == "Unknown":
        assert category == "semantic", f"{case}: meta case cannot consume Unknown"
    return {"case": case, "category": category, "verdict": verdict}


def classify_attack(
    row: dict[str, str], contract: dict[str, dict[str, str]]
) -> dict[str, str]:
    require_fields(row, ("mutation", "target", "expected", "witness"), row.get("mutation", "<attack>"))
    mutation = row["mutation"]
    assert mutation in ATTACK_RULES, f"unknown mutation {mutation}"
    target, term_groups = ATTACK_RULES[mutation]
    assert row["target"] in contract, f"{mutation}: target does not exist"
    assert row["target"] == target, f"{mutation}: target drift"
    assert row["expected"] == "Violated", f"{mutation}: expected must be Violated"
    witness = row["witness"].casefold()
    for terms in term_groups:
        assert any(term in witness for term in terms), f"{mutation}: witness lacks {terms}"
    assert contract[target]["violated"].strip(), f"{mutation}: target has no violation rule"
    return {
        "mutation": mutation,
        "target": target,
        "verdict": "Violated",
        "witness_digest": hashlib.sha256(row["witness"].encode()).hexdigest(),
    }


def evaluate(
    contract_rows: list[dict[str, str]],
    oracle_rows: list[dict[str, str]],
    attack_rows: list[dict[str, str]],
) -> dict[str, object]:
    contract = validate_contract(contract_rows)
    assert len(oracle_rows) == len(ORACLE_RULES)
    assert {row.get("case", "") for row in oracle_rows} == set(ORACLE_RULES)
    oracle_results = sorted((classify_oracle(row) for row in oracle_rows), key=lambda item: item["case"])
    assert len(attack_rows) == len(ATTACK_RULES)
    assert {row.get("mutation", "") for row in attack_rows} == set(ATTACK_RULES)
    attack_results = sorted(
        (classify_attack(row, contract) for row in attack_rows),
        key=lambda item: item["mutation"],
    )
    rejected = sum(item["verdict"] == "Violated" for item in attack_results)
    score = rejected / len(attack_results)
    assert score == 1.0
    return {
        "observables": sorted(contract),
        "oracles": oracle_results,
        "attacks": attack_results,
        "valid_mutations": len(attack_results),
        "rejected_mutations": rejected,
        "mutation_score": score,
    }


def orderings(items: list[dict[str, str]]) -> list[list[dict[str, str]]]:
    return [items, list(reversed(items)), items[1:] + items[:1], items[::2] + items[1::2]]


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--head", required=True)
    parser.add_argument("--measured-at", required=True)
    parser.add_argument("--working-tree", required=True)
    args = parser.parse_args()
    root = args.root.resolve()
    diag = root / "00_nucleo/diagnosticos"
    contract_path = diag / "p1248-image-contract.tsv"
    oracle_path = diag / "p1248-image-oracles.tsv"
    attack_path = diag / "p1248-image-attacks.tsv"
    roles_path = diag / "p1248-role-capabilities.tsv"
    contract = rows(contract_path)
    oracles = rows(oracle_path)
    attacks = rows(attack_path)
    roles = rows(roles_path)
    assert len(roles) == 6

    evaluations = []
    for index in range(4):
        evaluations.append(evaluate(
            orderings(contract)[index], orderings(oracles)[index], orderings(attacks)[index]
        ))
    representations = [canonical_json(value) for value in evaluations]
    digests = [digest(value) for value in evaluations]
    assert len(set(representations)) == 1, "canonical result changed under repeat/reorder"
    assert len(set(digests)) == 1, "canonical digest changed under repeat/reorder"
    evaluation = evaluations[0]
    oracle_results = evaluation["oracles"]

    result = {
        "verdict": "PRESEMANTIC_GATE_PASS_UNATTESTED_ISOLATION",
        "head": args.head,
        "measured_at": args.measured_at,
        "working_tree": args.working_tree,
        "contract_sha256": sha(contract_path),
        "oracles_sha256": sha(oracle_path),
        "attacks_sha256": sha(attack_path),
        "roles_sha256": sha(roles_path),
        "observable_count": len(evaluation["observables"]),
        "oracle_count": len(oracle_results),
        "semantic_cases": sum(item["category"] == "semantic" for item in oracle_results),
        "meta_cases": sum(item["category"] == "meta" for item in oracle_results),
        "preserved_cases": sum(item["verdict"] == "Preserved" for item in oracle_results),
        "unknown_cases": sum(item["verdict"] == "Unknown" for item in oracle_results),
        "valid_mutations": evaluation["valid_mutations"],
        "rejected_mutations": evaluation["rejected_mutations"],
        "mutation_score": evaluation["mutation_score"],
        "determinism_runs": len(evaluations),
        "canonical_gate_sha256": digests[0],
        "candidate_executed": False,
        "productive_change": False,
        "isolation_attested": False,
    }
    args.out.mkdir(parents=True, exist_ok=True)
    (args.out / "summary.json").write_text(
        json.dumps(result, indent=2, sort_keys=True) + "\n"
    )


if __name__ == "__main__":
    main()
