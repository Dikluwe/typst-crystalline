#!/usr/bin/env python3
"""Audit the historical P1243 v3 preseal against current protected inputs."""

import argparse
import csv
import hashlib
import json
from collections import Counter
from pathlib import Path


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def tsv_fields(path: Path) -> dict[str, str]:
    with path.open(newline="") as source:
        rows = list(csv.reader(source, delimiter="\t"))
    return {row[0]: row[1] for row in rows[1:] if len(row) >= 2}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--head", required=True)
    parser.add_argument("--measured-at", required=True)
    parser.add_argument("--working-tree", required=True)
    args = parser.parse_args()
    root = args.root.resolve()
    out = args.out.resolve()
    out.mkdir(parents=True, exist_ok=True)

    diag = root / "00_nucleo/diagnosticos"
    manifest = tsv_fields(diag / "p1243-tekt-manifesto-v3.tsv")
    current = {
        "predecessor_step_sha256": sha(root / "typst-passo-1242.md"),
        "predecessor_inventory_sha256": sha(diag / "p1242-svg-gap-inventory.tsv"),
        "tiling_l0_sha256": sha(root / "00_nucleo/prompts/entities/tiling.md"),
        "tiling_stdlib_l0_sha256": sha(root / "00_nucleo/prompts/compiler/stdlib/tiling-stdlib.md"),
        "svg_l0_sha256": sha(root / "00_nucleo/prompts/infra/export/svg.md"),
    }
    drift = {key: {"sealed": manifest[key], "current": value} for key, value in current.items() if manifest[key] != value}

    results_path = diag / "p1243-verifier-results-v3.tsv"
    with results_path.open(newline="") as source:
        rows = list(csv.DictReader(source, delimiter="\t"))
    attacks = [row for row in rows if row["kind"] == "attack"]
    assert len(attacks) == 34
    assert all(row["correct"] == "true" for row in attacks)
    observed = Counter(row["observed"] for row in attacks)
    assert observed == {"Unknown": 19, "CONTRACT-GAP": 10, "PASS": 3, "FAIL": 2}

    summary = {
        "verdict": "PRESEAL_INVALIDATED_NEGATIVE_BOUNDARY_RETAINED",
        "head": args.head,
        "measured_at": args.measured_at,
        "working_tree": args.working_tree,
        "historical_classification_accuracy": {"correct": 34, "total": 34},
        "historical_observed_counts": dict(sorted(observed.items())),
        "mutation_score": None,
        "drifted_protected_inputs": drift,
        "additional_svg_preserved": 0,
        "isolation_attestation": False,
        "historical_results_sha256": sha(results_path),
    }
    (out / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
