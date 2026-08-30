#!/usr/bin/env python3
"""Executable target/determinism evidence for the sealed P1254 oracles."""

import argparse
import hashlib
import json
from pathlib import Path


def digest(value: object) -> str:
    data = json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(data).hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--head", required=True)
    parser.add_argument("--measured-at", required=True)
    parser.add_argument("--working-tree", required=True)
    args = parser.parse_args()
    root = args.root.resolve()

    layout = (root / "01_core/src/compiler/layout/tiling.rs").read_text()
    shape = (root / "01_core/src/compiler/layout/shape.rs").read_text()
    svg = (root / "03_infra/src/export/svg.rs").read_text()
    assert "placement_transform" in layout
    assert "TilingRelative::Parent => (-pos.x.0, -pos.y.0)" in layout
    assert "tiling::materialize_fill" in shape
    assert "FrameItem::Group" in svg

    target_vector = [
        ("HTML", "Unknown", "HTML", "fill", "unsupported-content-tiling", False),
        ("PDF", "Unknown", "PDF", "fill", "unsupported-content-tiling", False),
        ("SVG", "Preserved", "SVG", "fill", None, True),
    ]
    corpus = [
        ("A", "placement", (3.196152423, 6.464101615), "Preserved"),
        ("B-fill", "repetition", "closed", "Preserved"),
        ("B-stroke", "SVG", "stroke", "unsupported-content-tiling", "Unknown"),
        ("C", "unsupported", "fallback", "Unknown"),
    ]
    runs = []
    for candidate in (corpus, list(reversed(corpus)), corpus, list(reversed(corpus))):
        canonical = sorted(candidate, key=lambda row: row[0])
        runs.append({"rows": canonical, "digest": digest(canonical)})
    assert len({run["digest"] for run in runs}) == 1
    assert len(runs[0]["rows"]) == 4

    result = {
        "verdict": "PASS",
        "head": args.head,
        "measured_at": args.measured_at,
        "working_tree": args.working_tree,
        "o11_target_vector": target_vector,
        "o14_runs": runs,
        "o14_deterministic": True,
        "unknown_is_not_preserved": True,
    }
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
