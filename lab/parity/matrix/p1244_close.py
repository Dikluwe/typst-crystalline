#!/usr/bin/env python3
"""Validate that P1244 closes without productive changes."""

import argparse
import csv
import hashlib
import json
from pathlib import Path


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def fields(path: Path) -> dict[str, str]:
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

    predecessor = fields(diag / "p1243-tekt-preseal-receipt.tsv")
    assert predecessor["verdict"] == "PRESEAL_INVALIDATED_NEGATIVE_BOUNDARY_RETAINED"
    assert predecessor["additional_svg_preserved"] == "0"
    assert predecessor["mutation_score"].startswith("NOT_MEASURED")

    svg_path = root / "03_infra/src/export/svg.rs"
    svg = svg_path.read_text()
    assert 'Paint::Tiling(_) => "tiling-content-or-size"' in svg
    assert "fn p1227_tiling_de_cor_emite_pattern_com_tamanho_e_spacing" in svg

    result = {
        "verdict": "CLOSED_NO_CODE_NEGATIVE_BOUNDARY_RETAINED_WITH_EXTERNAL_V5",
        "head": args.head,
        "measured_at": args.measured_at,
        "working_tree": args.working_tree,
        "predecessor_receipt_sha256": sha(diag / "p1243-tekt-preseal-receipt.tsv"),
        "predecessor_summary_sha256": sha(diag / "p1243-revalidation-summary.json"),
        "entity_l0_sha256": sha(root / "00_nucleo/prompts/entities/tiling.md"),
        "constructor_l0_sha256": sha(root / "00_nucleo/prompts/compiler/stdlib/tiling-stdlib.md"),
        "svg_l0_sha256": sha(root / "00_nucleo/prompts/infra/export/svg.md"),
        "entity_consumer_sha256": sha(root / "01_core/src/entities/tiling.rs"),
        "svg_consumer_sha256": sha(svg_path),
        "additional_svg_preserved": 0,
        "mutation_score": None,
        "production_change": False,
        "fallback": "tiling-content-or-size",
        "segregation": "not applicable; diagnostic no-code closure",
    }
    (out / "summary.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
