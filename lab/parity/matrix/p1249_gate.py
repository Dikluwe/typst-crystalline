#!/usr/bin/env python3
"""Reproduce the P1249 split and ADR-0127 proposal gate."""

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
    diag = root / "00_nucleo/diagnosticos"
    boundaries = rows(diag / "p1249-glyph-boundaries.tsv")
    options = rows(diag / "p1249-glyph-options.tsv")
    decisions = rows(diag / "p1249-owner-decision.tsv")
    assert len(boundaries) == 8
    assert len(options) == 5
    assert sum(row["classification"] == "REPRESENTED" for row in boundaries) == 6
    assert sum(row["classification"] == "TRANSPORT-GAP" for row in boundaries) == 1
    assert sum(row["classification"] == "PRESERVED-SCOPE-OUT" for row in boundaries) == 1
    assert options[0]["decision"] == "APPROVED BY OWNER 2026-08-28; PRESEAL REQUIRED"
    assert len(decisions) == 1 and decisions[0]["owner_decision"] == "APPROVED"

    layout = root / "01_core/src/entities/layout_types.rs"
    pipeline = root / "03_infra/src/pipeline.rs"
    svg = root / "03_infra/src/export/svg.rs"
    layout_text = layout.read_text()
    assert "Glyph {" in layout_text and "base_char: char" in layout_text
    assert "resolve_font_combo(*base_char, style)" in pipeline.read_text()
    assert "FrameItem::Glyph { .. } =>" in svg.read_text()

    nucleus = root / "00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml"
    svg_l0 = root / "00_nucleo/prompts/infra/export/svg.md"
    pipeline_l0 = root / "00_nucleo/prompts/infra/pipeline.md"
    effective_hash = effective_nucleus_sha(nucleus)
    pin = f"sha256:{effective_hash}"
    assert pin in svg_l0.read_text()
    assert pin in pipeline_l0.read_text()

    result = {
        "verdict": "L0_WRITTEN_ARCHITECTURE_APPROVED_AWAITING_SEGREGATED_PRESEAL",
        "head": args.head,
        "measured_at": args.measured_at,
        "working_tree": args.working_tree,
        "boundaries_sha256": sha(diag / "p1249-glyph-boundaries.tsv"),
        "options_sha256": sha(diag / "p1249-glyph-options.tsv"),
        "owner_decision_sha256": sha(diag / "p1249-owner-decision.tsv"),
        "nucleus_file_sha256": sha(nucleus),
        "nucleus_effective_sha256": effective_hash,
        "svg_l0_sha256": sha(svg_l0),
        "pipeline_l0_sha256": sha(pipeline_l0),
        "represented_fields": 6,
        "transport_gaps": 1,
        "preserved_scope_outs": 1,
        "candidate_executed": False,
        "productive_change": False,
        "mutation_score": None,
    }
    args.out.mkdir(parents=True, exist_ok=True)
    (args.out / "summary.json").write_text(
        json.dumps(result, indent=2, sort_keys=True) + "\n"
    )


if __name__ == "__main__":
    main()
