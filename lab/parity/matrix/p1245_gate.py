#!/usr/bin/env python3
"""Validate the P1245 ADR-0127 stop gate without authorizing code."""

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

    p1243 = fields(diag / "p1243-tekt-preseal-receipt.tsv")
    p1244 = fields(diag / "p1244-closure-receipt.tsv")
    confirmation = fields(diag / "p1245-owner-confirmation.tsv")
    assert p1243["verdict"] == "PRESEAL_INVALIDATED_NEGATIVE_BOUNDARY_RETAINED"
    assert p1244["verdict"] == "CLOSED_NO_CODE_NEGATIVE_BOUNDARY_RETAINED_WITH_EXTERNAL_V5"
    entity_hash = sha(root / "00_nucleo/prompts/entities/tiling.md")
    constructor_hash = sha(root / "00_nucleo/prompts/compiler/stdlib/tiling-stdlib.md")
    assert confirmation["confirmed_by"] == "owner"
    assert confirmation["entities_tiling_l0_sha256"] == entity_hash
    assert confirmation["stdlib_tiling_l0_sha256"] == constructor_hash

    with (diag / "p1245-tiling-options.tsv").open(newline="") as source:
        options = list(csv.DictReader(source, delimiter="\t"))
    assert len(options) == 4
    assert sum(row["decision"].startswith("PROPOSED") for row in options) == 1

    with (diag / "p1245-tiling-consumers.tsv").open(newline="") as source:
        consumers = list(csv.DictReader(source, delimiter="\t"))
    productive = [row for row in consumers if row["layer"] != "excluded"]
    excluded = [row for row in consumers if row["layer"] == "excluded"]
    assert (len(productive), len(excluded)) == (17, 3)

    result = {
        "verdict": "L0_HASHES_CONFIRMED_AWAITING_SEGREGATED_PRESEAL",
        "head": args.head,
        "measured_at": args.measured_at,
        "working_tree": args.working_tree,
        "entity_l0_sha256": entity_hash,
        "constructor_l0_sha256": constructor_hash,
        "owner_confirmation_sha256": sha(diag / "p1245-owner-confirmation.tsv"),
        "p1243_receipt_sha256": sha(diag / "p1243-tekt-preseal-receipt.tsv"),
        "p1244_receipt_sha256": sha(diag / "p1244-closure-receipt.tsv"),
        "options": 4,
        "productive_consumers": 17,
        "excluded_textual_matches": 3,
        "adr0127_gate": "owner confirmed both exact L0 hashes",
        "productive_change": False,
        "hash_reseal": False,
    }
    (out / "summary.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
