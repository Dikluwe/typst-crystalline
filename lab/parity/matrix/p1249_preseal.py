#!/usr/bin/env python3
"""Deterministic presemantic discriminator for sealed P1249 glyph artifacts."""

import argparse
import csv
import hashlib
import json
from pathlib import Path


OBSERVABLES = {
    "resolved-font-identity", "request-key-stability", "outline-morphology",
    "position-and-scale", "y-axis-orientation", "fill-and-alpha",
    "assembly-multiple-pieces", "transform-composition",
    "advance-noninterference", "collection-reordering", "fontless-wrapper",
    "determinism",
}

ORACLES = {
    "P01-resolved-font-collision": ("positive", "resolved-font-identity", "Preserved"),
    "P02-equal-request-stability": ("positive", "request-key-stability", "Preserved"),
    "P03-variant-distinguishes-face": ("positive", "request-key-stability", "Preserved"),
    "P04-outline-hole-and-curves": ("positive", "outline-morphology", "Preserved"),
    "P05-position-scale-once": ("positive", "position-and-scale", "Preserved"),
    "P06-upright-y-inversion": ("positive", "y-axis-orientation", "Preserved"),
    "P07-fill-alpha-under-ancestor": ("positive", "fill-and-alpha", "Preserved"),
    "P08-assembly-three-pieces": ("positive", "assembly-multiple-pieces", "Preserved"),
    "P09-transform-composes-once": ("positive", "transform-composition", "Preserved"),
    "P10-advance-noninterference": ("positive", "advance-noninterference", "Preserved"),
    "P11-collection-reorder": ("positive", "collection-reordering", "Preserved"),
    "P12-repeat-semantic-digest": ("positive", "determinism", "Preserved"),
    "N01-incidental-font-probe": ("negative", "resolved-font-identity", "Violated"),
    "N02-assembly-piece-lost": ("negative", "assembly-multiple-pieces", "Violated"),
    "N03-fill-alpha-made-opaque": ("negative", "fill-and-alpha", "Violated"),
    "N04-double-local-transform": ("negative", "transform-composition", "Violated"),
    "U01-request-association-absent": ("opaque", "resolved-font-identity", "Unknown"),
    "U02-request-association-conflict": ("opaque", "request-key-stability", "Unknown"),
    "U03-mapped-font-missing": ("opaque", "collection-reordering", "Unknown"),
    "U04-invalid-scale-carrier": ("opaque", "position-and-scale", "Unknown"),
    "U05-fontless-wrapper": ("opaque", "fontless-wrapper", "Unknown"),
}

ATTACKS = {
    "M01-incidental-first-font": ("resolved-font-identity", ("alpha", "beta", "glyph_id 37")),
    "M02-request-key-drops-variant": ("request-key-stability", ("variant", "triangle", "diamond")),
    "M03-request-key-uses-occurrence": ("request-key-stability", ("occurrence", "address", "equal requests")),
    "M04-wrong-glyph-or-tofu": ("outline-morphology", ("tofu", "hole", "shoulder")),
    "M05-scale-applied-twice": ("position-and-scale", ("twice", "20/1000", "bounds")),
    "M06-position-translated-twice": ("position-and-scale", ("31 47", "twice", "anchor")),
    "M07-y-inversion-omitted": ("y-axis-orientation", ("omit", "notch", "baseline")),
    "M08-fill-alpha-forced-opaque": ("fill-and-alpha", ("0.25", "opaque", "blue")),
    "M09-assembly-extender-dropped": ("assembly-multiple-pieces", ("middle extender", "two landmarks", "gap")),
    "M10-ancestor-transform-reordered": ("transform-composition", ("reverse", "90-degree", "quadrant")),
    "M11-advance-applied-again": ("advance-noninterference", ("x_advance 17", "x=80", "x=97")),
    "M12-collection-index-identity": ("collection-reordering", ("index", "revers", "outline")),
    "M13-fontless-fallback-invented": ("fontless-wrapper", ("fontless", "system/network", "unknown")),
    "M14-unknown-promoted": ("resolved-font-identity", ("absent", "missing", "unknown", "preserved")),
    "M15-order-dependent-cache": ("determinism", ("traversal order", "digest", "unknown")),
}


def read_tsv(path):
    with path.open(newline="") as f:
        return list(csv.DictReader(f, delimiter="\t"))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def required(row, fields, identity):
    missing = [f for f in fields if not row.get(f, "").strip()]
    assert not missing, f"{identity}: missing {missing}"


def evaluate(contract_rows, oracle_rows, attack_rows):
    assert len(contract_rows) == len(OBSERVABLES)
    contract = {row["observable"]: row for row in contract_rows}
    assert set(contract) == OBSERVABLES and len(contract) == len(contract_rows)
    for name, row in contract.items():
        required(row, ("preserved", "violated", "unknown", "mechanics_outside_gate"), name)
    assert "unknown" in contract["fontless-wrapper"]["unknown"].casefold()
    assert "preserved" in contract["fontless-wrapper"]["unknown"].casefold()

    assert len(oracle_rows) == len(ORACLES)
    assert {r["case"] for r in oracle_rows} == set(ORACLES)
    oracle_results = []
    for row in oracle_rows:
        required(row, ("case", "category", "observable", "expected", "witness", "fixture", "carrier", "executable_criterion"), row.get("case", "oracle"))
        expected = ORACLES[row["case"]]
        assert (row["category"], row["observable"], row["expected"]) == expected
        if row["category"] == "opaque":
            assert row["expected"] == "Unknown"
            assert "unknown" in row["executable_criterion"].casefold()
        oracle_results.append({"case": row["case"], "category": row["category"], "observable": row["observable"], "verdict": row["expected"]})

    assert len(attack_rows) == len(ATTACKS)
    assert {r["mutation"] for r in attack_rows} == set(ATTACKS)
    attack_results = []
    for row in attack_rows:
        required(row, ("mutation", "target", "expected", "witness"), row.get("mutation", "attack"))
        target, terms = ATTACKS[row["mutation"]]
        assert row["target"] == target and target in contract
        assert row["expected"] == "Violated"
        witness = row["witness"].casefold()
        assert all(term in witness for term in terms), f"{row['mutation']}: weak witness"
        assert contract[target]["violated"].strip()
        attack_results.append({"mutation": row["mutation"], "target": target, "verdict": "Violated", "witness_sha256": hashlib.sha256(row["witness"].encode()).hexdigest()})

    rejected = sum(r["verdict"] == "Violated" for r in attack_results)
    score = rejected / len(attack_results)
    assert score == 1.0
    return {
        "observables": sorted(contract),
        "oracles": sorted(oracle_results, key=lambda r: r["case"]),
        "attacks": sorted(attack_results, key=lambda r: r["mutation"]),
        "mutation_score": score,
        "valid_mutations": len(attack_results),
        "rejected_mutations": rejected,
    }


def orders(rows):
    return [rows, list(reversed(rows)), rows[1:] + rows[:1], rows[::2] + rows[1::2]]


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--root", type=Path, required=True)
    p.add_argument("--out", type=Path, required=True)
    p.add_argument("--head", required=True)
    p.add_argument("--measured-at", required=True)
    p.add_argument("--working-tree", required=True)
    args = p.parse_args()
    root = args.root.resolve()
    diag = root / "00_nucleo/diagnosticos"
    paths = {
        "contract": diag / "p1249-glyph-contract.tsv",
        "oracles": diag / "p1249-glyph-oracles.tsv",
        "attacks": diag / "p1249-glyph-attacks.tsv",
    }
    source = {name: read_tsv(path) for name, path in paths.items()}
    evaluations = [evaluate(orders(source["contract"])[i], orders(source["oracles"])[i], orders(source["attacks"])[i]) for i in range(4)]
    representations = [canonical(v) for v in evaluations]
    digests = [hashlib.sha256(v.encode()).hexdigest() for v in representations]
    assert len(set(representations)) == 1
    assert len(set(digests)) == 1
    evaluation = evaluations[0]
    result = {
        "verdict": "PRESEMANTIC_GATE_PASS_UNATTESTED_ISOLATION",
        "head": args.head,
        "measured_at": args.measured_at,
        "working_tree": args.working_tree,
        "contract_sha256": sha(paths["contract"]),
        "oracles_sha256": sha(paths["oracles"]),
        "attacks_sha256": sha(paths["attacks"]),
        "observable_count": len(evaluation["observables"]),
        "oracle_count": len(evaluation["oracles"]),
        "positive_cases": sum(r["category"] == "positive" for r in evaluation["oracles"]),
        "negative_cases": sum(r["category"] == "negative" for r in evaluation["oracles"]),
        "unknown_cases": sum(r["verdict"] == "Unknown" for r in evaluation["oracles"]),
        "valid_mutations": evaluation["valid_mutations"],
        "rejected_mutations": evaluation["rejected_mutations"],
        "mutation_score": evaluation["mutation_score"],
        "determinism_runs": 4,
        "canonical_gate_sha256": digests[0],
        "candidate_executed": False,
        "productive_change": False,
        "isolation_attested": False,
    }
    args.out.mkdir(parents=True, exist_ok=True)
    (args.out / "summary.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
