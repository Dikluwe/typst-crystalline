#!/usr/bin/env python3
"""Validate the P1246 L0 proposal and mandatory ADR-0127 stop."""

import argparse
import csv
import hashlib
import json
from pathlib import Path


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


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

    contract = rows(diag / "p1246-clip-contract-draft.tsv")
    oracles = rows(diag / "p1246-clip-oracles-draft.tsv")
    attacks = rows(diag / "p1246-clip-attacks-draft.tsv")
    decisions = rows(diag / "p1246-owner-decision.tsv")
    assert (len(contract), len(oracles), len(attacks)) == (12, 10, 12)
    assert all(row["status"] == "NOT_EXECUTED" for row in oracles + attacks)
    assert len(decisions) == 1 and decisions[0]["owner_decision"] == "APPROVED"

    l0_path = root / "00_nucleo/prompts/infra/export/svg.md"
    l0 = l0_path.read_text()
    assert "## P1246 — clip geométrico local proposto" in l0
    assert "arquitetura aprovada pelo dono em 2026-08-28" in l0
    source_path = root / "03_infra/src/export/svg.rs"
    source = source_path.read_text()
    assert "_clip_mask:" in source
    assert "// clip_mask: scope-out neste passo." in source

    result = {
        "verdict": "L0_WRITTEN_ARCHITECTURE_APPROVED_AWAITING_SEGREGATED_PRESEAL",
        "head": args.head,
        "measured_at": args.measured_at,
        "working_tree": args.working_tree,
        "svg_l0_sha256": sha(l0_path),
        "svg_consumer_sha256": sha(source_path),
        "contract_draft_sha256": sha(diag / "p1246-clip-contract-draft.tsv"),
        "oracles_draft_sha256": sha(diag / "p1246-clip-oracles-draft.tsv"),
        "attacks_draft_sha256": sha(diag / "p1246-clip-attacks-draft.tsv"),
        "owner_decision_sha256": sha(diag / "p1246-owner-decision.tsv"),
        "contract_rows": 12,
        "oracle_rows": 10,
        "attack_rows": 12,
        "executed_oracles": 0,
        "executed_mutations": 0,
        "mutation_score": None,
        "productive_change": False,
        "adr0127_gate": "owner approved semantic clip contract; segregated preseal required before code",
    }
    (out / "summary.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
