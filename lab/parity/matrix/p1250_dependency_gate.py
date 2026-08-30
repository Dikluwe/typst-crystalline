#!/usr/bin/env python3
"""Deterministic dependency gate for P1250 integrated SVG campaign."""

import argparse
import csv
import hashlib
import json
from pathlib import Path


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--head", required=True)
    parser.add_argument("--measured-at", required=True)
    parser.add_argument("--working-tree", required=True)
    args = parser.parse_args()
    root = args.root.resolve()
    matrix = root / "00_nucleo/diagnosticos/p1250-dependency-gate.tsv"
    with matrix.open(newline="") as source:
        rows = list(csv.DictReader(source, delimiter="\t"))
    assert [row["dependency"] for row in rows] == [
        "P1245", "P1246", "P1247", "P1248", "P1249"
    ]
    blocked = [row["dependency"] for row in rows if row["gate"] == "BLOCKED"]
    ready = [row["dependency"] for row in rows if row["gate"] == "READY"]
    assert len(blocked) + len(ready) == len(rows)
    verdict = "DEPENDENCY_STOP" if blocked else "INTEGRATED_CAMPAIGN_READY"
    result = {
        "verdict": verdict,
        "head": args.head,
        "measured_at": args.measured_at,
        "working_tree": args.working_tree,
        "matrix_sha256": sha(matrix),
        "dependency_count": len(rows),
        "blocked_dependencies": blocked,
        "ready_dependencies": ready,
        "integrated_contract_started": False,
        "candidate_executed": False,
        "executed_mutations": 0,
        "mutation_score": None,
    }
    args.out.mkdir(parents=True, exist_ok=True)
    (args.out / "summary.json").write_text(
        json.dumps(result, indent=2, sort_keys=True) + "\n"
    )


if __name__ == "__main__":
    main()
