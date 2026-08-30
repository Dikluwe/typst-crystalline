#!/usr/bin/env python3
"""Reproduce the P1247 split between link edges and destination nodes."""

import argparse
import csv
import hashlib
import json
from pathlib import Path


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def effective_nucleus_sha(path: Path) -> str:
    digest = hashlib.sha256()
    digest.update(path.read_bytes())
    digest.update(b"\0TEKT-NUCLEUS-DEPS-V1\0")
    return digest.hexdigest()


def rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="") as source:
        return list(csv.DictReader(source, delimiter="\t"))


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

    boundaries = rows(diag / "p1247-link-boundaries.tsv")
    options = rows(diag / "p1247-link-options.tsv")
    decisions = rows(diag / "p1247-owner-decision.tsv")
    assert (len(boundaries), len(options)) == (8, 5)
    assert len(decisions) == 1
    assert sum(row["classification"] == "REPRESENTED" for row in boundaries) == 1
    assert sum(row["classification"] == "Preserved-current" for row in boundaries) == 1
    assert sum(row["classification"] == "CONTRACT-GAP" for row in boundaries) == 5
    assert sum(row["classification"] == "Unknown" for row in boundaries) == 1
    assert options[0]["decision"] == "APPROVED BY OWNER 2026-08-28; PRESEAL REQUIRED"
    assert decisions[0]["owner_decision"] == "APPROVED"

    layout = root / "01_core/src/entities/layout_types.rs"
    svg = root / "03_infra/src/export/svg.rs"
    nucleus = root / "00_nucleo/prompts/_nuclei/export/svg-destination-context.toml"
    svg_l0 = root / "00_nucleo/prompts/infra/export/svg.md"
    pipeline_l0 = root / "00_nucleo/prompts/infra/pipeline.md"
    layout_text = layout.read_text()
    svg_text = svg.read_text()
    assert "Destination(Label)" in layout_text
    assert "extracted_label_pages" in layout_text
    assert "extracted_label_positions" in layout_text
    assert "LinkTarget::Destination(_)" in svg_text
    assert "// Links internos: scope-out." in svg_text
    nucleus_hash = effective_nucleus_sha(nucleus)
    expected_pin = f"sha256:{nucleus_hash}"
    assert expected_pin in svg_l0.read_text()
    assert expected_pin in pipeline_l0.read_text()
    assert "arquitetura aprovada pelo dono em 2026-08-28" in svg_l0.read_text()
    assert "arquitetura aprovada pelo dono em 2026-08-28" in pipeline_l0.read_text()

    result = {
        "verdict": "L0_WRITTEN_ARCHITECTURE_APPROVED_AWAITING_SEGREGATED_PRESEAL",
        "head": args.head,
        "measured_at": args.measured_at,
        "working_tree": args.working_tree,
        "boundaries_sha256": sha(diag / "p1247-link-boundaries.tsv"),
        "options_sha256": sha(diag / "p1247-link-options.tsv"),
        "owner_decision_sha256": sha(diag / "p1247-owner-decision.tsv"),
        "nucleus_file_sha256": sha(nucleus),
        "nucleus_sha256": nucleus_hash,
        "svg_l0_sha256": sha(svg_l0),
        "pipeline_l0_sha256": sha(pipeline_l0),
        "layout_types_sha256": sha(layout),
        "svg_consumer_sha256": sha(svg),
        "represented_internal_edge": True,
        "closed_internal_svg_graphs": 0,
        "executed_oracles": 0,
        "executed_mutations": 0,
        "mutation_score": None,
        "productive_change": False,
        "adr0127_gate": "destination-context architecture approved; public link expansion remains a separate decision; preseal required before code",
    }
    (out / "summary.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
