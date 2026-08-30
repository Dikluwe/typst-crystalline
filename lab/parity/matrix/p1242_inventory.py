#!/usr/bin/env python3
"""Rebuild the conservative P1242 SVG gap inventory from current receipts."""

import argparse
import csv
import hashlib
import json
from pathlib import Path


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def fields(path: Path) -> dict[str, str]:
    with path.open(newline="") as source:
        return {row[0]: row[1] for row in list(csv.reader(source, delimiter="\t"))[1:]}


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

    p1241_summary = root / "00_nucleo/diagnosticos/p1241-svg-multispace-summary.json"
    p1241_cert = root / "00_nucleo/diagnosticos/p1241-tekt-certificado.tsv"
    p1243_receipt = root / "00_nucleo/diagnosticos/p1243-tekt-preseal-receipt.tsv"
    p1246 = root / "00_nucleo/diagnosticos/typst-p1246-clip-preseal.md"
    p1247 = root / "00_nucleo/diagnosticos/typst-p1247-internal-links-gap.md"
    p1248 = root / "00_nucleo/diagnosticos/typst-p1248-image-preseal.md"
    p1249 = root / "00_nucleo/diagnosticos/typst-p1249-glyph-contract-gap.md"
    svg = root / "03_infra/src/export/svg.rs"

    summary = json.loads(p1241_summary.read_text())
    assert (summary["pairs"], summary["preserved"], summary["unknown"]) == (16, 2, 14)
    assert summary["mutation_score"] is None
    cert = fields(p1241_cert)
    assert cert["verdict"] == "ACCEPTED_CONSERVATIVE_MULTISPACE_CONSOLIDATION_WITH_EXTERNAL_V5"
    receipt = fields(p1243_receipt)
    assert receipt["verdict"] == "PRESEAL_SEALED"
    assert receipt["additional_svg_preserved"] == "0"
    assert receipt["shared_vanilla_constructor_parity"].startswith("Unknown")
    source = svg.read_text()
    for witness in (
        "Paint::Tiling(tiling)", "FrameItem::Glyph { .. }", "FrameItem::Image {",
        "_clip_mask:", "LinkTarget::Destination(_)",
    ):
        assert witness in source, witness

    rows = [
        ["1", "tiling-modelable", "P1243 v3 sealed a negative boundary: 0 additional SVG Preserved", "03_infra/src/export/svg.rs:185-186,241-265", "lab/typst-original/crates/typst-svg/src/paint.rs:340-383", "00_nucleo/prompts/infra/export/svg.md", "language-morphology", "no public change for existing color+size subset", "high", "high", "medium", "P1244 closes without code unless new bilateral evidence appears"],
        ["2", "clip-geometry", "P1246 PRESEAL REJECTED 3/9", "03_infra/src/export/svg.rs:1086-1117", "lab/typst-original/crates/typst-svg/src/lib.rs:326-359", "00_nucleo/prompts/infra/export/svg.md", "language-morphology", "no public change for represented ShapeKind subset", "high", "high", "medium", "repair transform/units/fill-rule/dedup attacks before implementation"],
        ["3", "image-visible-morphology", "P1248 PRESEMANTIC REJECTED 9/11", "03_infra/src/export/svg.rs:702-704,824-850", "lab/typst-original/crates/typst-svg/src/image.rs:18-41", "00_nucleo/prompts/infra/export/svg.md", "language-morphology", "no public change for decoded raster subset", "medium", "high", "medium", "add visible-content mutant and morphology oracle"],
        ["4", "paint-multispace", "P1241 conservative consolidation: 2 sRGB Preserved; 14 Unknown", "03_infra/src/export/svg.rs:171-187", "lab/typst-original/crates/typst-svg/src/paint.rs:340-383", "00_nucleo/prompts/infra/export/svg.md", "language-morphology", "continuous for pair-specific internal parity corrections", "high", "medium", "high", "preserve exact P1241 classifications; require pair-specific L3 evidence"],
        ["5", "internal-links", "P1247 CONTRACT-GAP; executable universe zero", "03_infra/src/export/svg.rs:1120-1142", "lab/typst-original/crates/typst-svg/src/lib.rs:362-375", "00_nucleo/prompts/infra/export/svg.md", "language-semantics", "yes: identity/resolver pipeline expansion", "medium", "low", "high", "stop for ADR0127 before implementation"],
        ["6", "math-direct-glyph", "P1249 CONTRACT-GAP; executable universe zero", "03_infra/src/export/svg.rs:699-701", "lab/typst-original/crates/typst-svg/src/text.rs:55-92", "00_nucleo/prompts/infra/export/svg.md", "language-morphology", "yes: font identity/bytes/fill across pipeline", "medium", "low", "high", "stop for ADR0127 before implementation"],
        ["excluded", "external-differences", "P1240 is auxiliary and does not adjudicate IDs/defs order/external translation", "lab/parity/matrix/p1240_raster.py", "mechanical SVG serialization differences", "-", "mechanics", "none", "low", "high", "low", "do not promote to language gap"],
    ]
    header = ["priority", "gap", "current_evidence", "crystalline_witness", "vanilla_witness", "owner_l0", "adr0107_class", "adr0127_gate", "impact", "independence", "risk", "next_action"]
    inventory = out / "inventory.tsv"
    with inventory.open("w", newline="") as target:
        writer = csv.writer(target, delimiter="\t", lineterminator="\n")
        writer.writerow(header)
        writer.writerows(rows)

    dependencies = {p.name: sha(p) for p in (p1241_summary, p1241_cert, p1243_receipt, p1246, p1247, p1248, p1249)}
    result = {
        "verdict": "INVENTORY_CLOSED_TILING_FIRST_DIAGNOSTIC",
        "head": args.head,
        "measured_at": args.measured_at,
        "working_tree": args.working_tree,
        "pairs": 16,
        "preserved": 2,
        "unknown": 14,
        "p1243_additional_svg_preserved": 0,
        "inventory_sha256": sha(inventory),
        "dependencies": dependencies,
    }
    (out / "summary.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
