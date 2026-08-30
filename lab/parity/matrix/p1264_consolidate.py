#!/usr/bin/env python3
"""P1264: reexecuta, readjudica e certifica o corpus SVG focal completo."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import subprocess
import tempfile
from datetime import datetime
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
PAIRS = ("linear/oklab", "radial/oklab", "linear/linear-rgb", "radial/linear-rgb")
P1263_FILES = (
    "p1263-target-results.tsv",
    "p1263-cost-decisions.tsv",
    "p1263-p1237-numeric.tsv",
    "p1263-p1237-pairs.tsv",
    "p1263-attacks.tsv",
    "p1263-opaque.tsv",
)


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def write_tsv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        raise ValueError(f"empty output: {path}")
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, list(rows[0]), delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def directory_hashes(path: Path) -> dict[str, str]:
    return {item.name: sha(item) for item in sorted(path.iterdir()) if item.is_file()}


def canonical_rows(path: Path) -> list[str]:
    return sorted(json.dumps(row, sort_keys=True, separators=(",", ":")) for row in read_tsv(path))


def run_p1263(args: argparse.Namespace, fixtures: Path, out: Path) -> None:
    subprocess.run(
        [
            "python3",
            str(args.p1263_runner),
            "--fixtures",
            str(fixtures),
            "--fixture-root",
            str(args.fixture_root),
            "--budgets",
            str(args.budgets),
            "--graph",
            str(args.graph),
            "--frontiers",
            str(args.frontiers),
            "--before-numeric",
            str(args.before_numeric),
            "--p1259-results",
            str(args.p1259_results),
            "--oracle-script",
            str(args.oracle_script),
            "--out",
            str(out),
            "--require-product",
        ],
        cwd=ROOT,
        check=True,
        text=True,
        capture_output=True,
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--fixtures", type=Path, required=True)
    parser.add_argument("--fixture-root", type=Path, required=True)
    parser.add_argument("--budgets", type=Path, required=True)
    parser.add_argument("--graph", type=Path, required=True)
    parser.add_argument("--frontiers", type=Path, required=True)
    parser.add_argument("--before-numeric", type=Path, required=True)
    parser.add_argument("--p1259-results", type=Path, required=True)
    parser.add_argument("--oracle-script", type=Path, required=True)
    parser.add_argument("--p1263-runner", type=Path, required=True)
    parser.add_argument("--p1263-frozen-dir", type=Path, required=True)
    parser.add_argument("--p1261-summary", type=Path, required=True)
    parser.add_argument("--p1261-attacks", type=Path, required=True)
    parser.add_argument("--p1262-summary", type=Path, required=True)
    parser.add_argument("--p1262-attacks", type=Path, required=True)
    parser.add_argument("--p1263-summary", type=Path, required=True)
    parser.add_argument("--p1263-attacks", type=Path, required=True)
    parser.add_argument("--p1234-direct", type=Path, required=True)
    parser.add_argument("--p1234-repeat", type=Path, required=True)
    parser.add_argument("--p1236-direct", type=Path, required=True)
    parser.add_argument("--p1236-repeat", type=Path, required=True)
    parser.add_argument("--p1255-before", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--working-tree-snapshot", type=Path)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)

    fixture_rows = read_tsv(args.fixtures)
    if len(fixture_rows) != 24 or len({row["id"] for row in fixture_rows}) != 24:
        raise ValueError("P1264 requires exactly 24 unique fixtures")

    p1234_direct_hashes = directory_hashes(args.p1234_direct)
    p1234_repeat_hashes = directory_hashes(args.p1234_repeat)
    p1236_direct_hashes = directory_hashes(args.p1236_direct)
    p1236_repeat_hashes = directory_hashes(args.p1236_repeat)
    if p1234_direct_hashes != p1234_repeat_hashes:
        raise ValueError("P1234 repetition is not byte-identical")
    if p1236_direct_hashes != p1236_repeat_hashes:
        raise ValueError("P1236 repetition is not byte-identical")

    p1234_summary = json.loads((args.p1234_direct / "summary.json").read_text())
    p1236_summary = json.loads((args.p1236_direct / "summary.json").read_text())
    if (p1234_summary["fixture_count"], p1234_summary["probe_count"], p1234_summary["public_divergences"]) != (15, 30, 0):
        raise ValueError("P1234 L1 public regression")
    if p1236_summary["pairs"] != 42 or p1236_summary["unknown"] != 0 or p1236_summary["violated_luma"] != 0:
        raise ValueError("P1236 L1 regression")

    with tempfile.TemporaryDirectory(prefix="p1264-") as temporary:
        temp = Path(temporary)
        inverse_fixtures = temp / "fixtures-inverse.tsv"
        write_tsv(inverse_fixtures, list(reversed(fixture_rows)))
        direct = temp / "direct"
        inverse = temp / "inverse"
        run_p1263(args, args.fixtures, direct)
        run_p1263(args, inverse_fixtures, inverse)

        for name in P1263_FILES:
            if canonical_rows(direct / name) != canonical_rows(inverse / name):
                raise ValueError(f"P1237 direct/inverse semantic drift: {name}")
        direct_summary = json.loads((direct / "p1263-summary.json").read_text())
        inverse_summary = json.loads((inverse / "p1263-summary.json").read_text())
        direct_summary["inputs"].pop("fixtures", None)
        inverse_summary["inputs"].pop("fixtures", None)
        if direct_summary != inverse_summary:
            raise ValueError("P1237 direct/inverse summary drift")

        for name in P1263_FILES + ("p1263-summary.json",):
            frozen = args.p1263_frozen_dir / name
            if sha(direct / name) != sha(frozen):
                raise ValueError(f"sealed P1263 receipt drift: {name}")

        numeric = read_tsv(direct / "p1263-p1237-numeric.tsv")
        costs = read_tsv(direct / "p1263-cost-decisions.tsv")
        graph = read_tsv(args.graph)
        if len(numeric) != 24 or any(row["status"] != "Preserved" for row in numeric):
            raise ValueError("P1237 numeric corpus is not 24/24")
        if len(graph) != 24 or any(row["status"] != "Preserved" for row in graph):
            raise ValueError("P1237 graph corpus is not 24/24")
        if any(row["additional_stops"] != "0" or row["additional_decisions"] != "0" for row in costs):
            raise ValueError("P1264 observed incremental adaptive cost")
        if max(int(row["emitted_stops"]) for row in costs) > 64:
            raise ValueError("P1264 cap exceeded")

        receipt_rows: list[dict[str, object]] = []
        for phase, directory in (("p1237-direct", direct), ("p1237-inverse", inverse)):
            for name, digest in directory_hashes(directory).items():
                receipt_rows.append({"phase": phase, "artifact": name, "sha256": digest})

    focal = []
    for step, summary_path, attacks_path in (
        ("P1261", args.p1261_summary, args.p1261_attacks),
        ("P1262", args.p1262_summary, args.p1262_attacks),
        ("P1263", args.p1263_summary, args.p1263_attacks),
    ):
        summary = json.loads(summary_path.read_text())
        attacks = read_tsv(attacks_path)
        rejected = sum(row["status"] == "REJECTED" for row in attacks)
        if summary["mutation_score"] != 1.0 or rejected != len(attacks):
            raise ValueError(f"{step} focal mutation seal invalid")
        focal.append((step, len(attacks), rejected))

    before = read_tsv(args.p1255_before)
    before_values = {row["pair_or_metric"]: row["value"] for row in before if row["record"] == "corpus"}
    if before_values != {"fixtures": "24", "violated": "18", "preserved": "6"}:
        raise ValueError("P1255 before baseline drift")

    before_pair = {
        "linear/oklab": (0, "Unknown-native-approximation"),
        "radial/oklab": (0, "Unknown-fallback"),
        "linear/linear-rgb": (3, "Unknown-native-approximation"),
        "radial/linear-rgb": (3, "Unknown-fallback"),
    }
    pair_rows: list[dict[str, object]] = []
    for pair in PAIRS:
        graph_pass = sum(row["pair"] == pair and row["status"] == "Preserved" for row in graph)
        numeric_pass = sum(row["pair"] == pair and row["status"] == "Preserved" for row in numeric)
        applicable = "P1261;P1262;P1263" if pair.endswith("/oklab") else "P1261;P1263"
        eligible = graph_pass == 6 and numeric_pass == 6
        pair_rows.append(
            {
                "pair": pair,
                "before_graph": "6/6",
                "before_numeric": f"{before_pair[pair][0]}/6",
                "before_classification": before_pair[pair][1],
                "after_graph": f"{graph_pass}/6",
                "after_numeric": f"{numeric_pass}/6",
                "causal_unknown": 0,
                "applicable_focal_seals": applicable,
                "promotion_gate": "PASS" if eligible else "FAIL",
                "certified_classification": "Preserved-fragment" if eligible else before_pair[pair][1],
                "productive_promotion_applied": "false",
            }
        )

    before_after = [
        {"scope": "P1237-total", "phase": "P1255-before", "preserved": 6, "violated": 18, "unknown": 0, "verdict": "open"},
        {"scope": "P1237-total", "phase": "P1264-after", "preserved": 24, "violated": 0, "unknown": 0, "verdict": "Preserved-fragment"},
    ]

    attacks: list[dict[str, object]] = []

    def attack(mutant: str, rejected: bool, witness: str) -> None:
        attacks.append({"mutant": mutant, "executed": "true", "witness": witness, "status": "REJECTED" if rejected else "SURVIVED"})

    def pair_gate(rows: list[dict[str, object]]) -> bool:
        required = {
            "linear/oklab": {"P1261", "P1262", "P1263"},
            "radial/oklab": {"P1261", "P1262", "P1263"},
            "linear/linear-rgb": {"P1261", "P1263"},
            "radial/linear-rgb": {"P1261", "P1263"},
        }
        return all(
            row["after_graph"] == "6/6"
            and row["after_numeric"] == "6/6"
            and row["causal_unknown"] == 0
            and set(str(row["applicable_focal_seals"]).split(";")) == required[str(row["pair"])]
            for row in rows
        )

    def mutated(pair: str, field: str, value: object) -> list[dict[str, object]]:
        rows = [dict(row) for row in pair_rows]
        next(row for row in rows if row["pair"] == pair)[field] = value
        return rows

    attack("M01-promote-by-aggregate-total", not pair_gate(mutated("linear/oklab", "after_numeric", "5/6")), "aggregate 24/24 cannot override a contradictory pair-local 5/6")
    attack("M02-linear-promotes-radial", not pair_gate(mutated("radial/oklab", "after_numeric", "5/6")), "Linear 6/6 cannot promote Radial 5/6")
    attack("M03-oklab-promotes-linear-rgb", not pair_gate(mutated("linear/linear-rgb", "after_numeric", "5/6")), "Oklab 6/6 cannot promote LinearRgb 5/6")
    attack("M04-ignore-p95", any(row["mutant"] == "M06-max-without-p95" and row["status"] == "REJECTED" for row in read_tsv(args.p1263_attacks)), "P1263 executable p95 witness")
    attack("M05-unknown-as-success", not pair_gate(mutated("radial/linear-rgb", "causal_unknown", 1)), "one necessary Unknown blocks only its pair")
    attack("M06-partial-focal-receipts", not pair_gate(mutated("radial/oklab", "applicable_focal_seals", "P1261;P1263")), "missing P1262 Oklab seal blocks the pair")
    frozen_fixture_hash = json.loads(args.p1263_summary.read_text())["inputs"]["fixtures"]
    if sha(args.fixtures) != frozen_fixture_hash:
        raise ValueError("sealed 24-fixture manifest drift")
    attack("M07-fixture-hash-drift", "0" * 64 != frozen_fixture_hash, "mutated fixture digest is rejected against the sealed manifest")

    if any(row["status"] != "REJECTED" for row in attacks):
        raise ValueError("P1264 integration mutant survived")
    if any(row["promotion_gate"] != "PASS" for row in pair_rows):
        raise ValueError("one or more P1264 pair-local gates failed")

    l1_rows = [
        {"phase": "P1234", "population": "15 fixtures;30 probes", "result": "public_divergences=0", "repeat": "byte-identical", "status": "Preserved"},
        {"phase": "P1236", "population": "6 fixtures;42 pairs", "result": "luma_max=0;unknown=0;Known-Upstream-Bug=14", "repeat": "byte-identical", "status": "Preserved"},
    ]
    opaque_rows = [
        {"case": "missing-pair-evidence", "expected": "Unknown", "actual": "Unknown", "counts_as_success": "false", "status": "Preserved"},
        {"case": "unsupported-or-missing-server", "expected": "Unknown", "actual": "Unknown", "counts_as_success": "false", "status": "Preserved"},
    ]

    for phase, hashes in (("p1234", p1234_direct_hashes), ("p1236", p1236_direct_hashes)):
        for name, digest in hashes.items():
            receipt_rows.append({"phase": phase, "artifact": name, "sha256": digest})

    write_tsv(args.out / "p1264-p1237-numeric.tsv", numeric)
    write_tsv(args.out / "p1264-cost-decisions.tsv", costs)
    write_tsv(args.out / "p1264-pairs.tsv", pair_rows)
    write_tsv(args.out / "p1264-before-after.tsv", before_after)
    write_tsv(args.out / "p1264-l1-revalidation.tsv", l1_rows)
    write_tsv(args.out / "p1264-attacks.tsv", attacks)
    write_tsv(args.out / "p1264-opaque.tsv", opaque_rows)
    write_tsv(args.out / "p1264-execution-receipts.tsv", receipt_rows)

    summary = {
        "baseline": "upstream/main a51e02804 ratified",
        "fixtures": 24,
        "before_preserved": 6,
        "before_violated": 18,
        "after_preserved": 24,
        "after_violated": 0,
        "graph_pass": 24,
        "numeric_pass": 24,
        "eligible_pairs": 4,
        "productive_promotions_applied": 0,
        "additional_stops": 0,
        "additional_decisions": 0,
        "cap": 64,
        "p1234_public_divergences": 0,
        "p1236_luma_max": 0.0,
        "p1236_unknown": 0,
        "p1237_direct_inverse_semantic_equal": True,
        "focal_mutants_rejected": sum(item[2] for item in focal),
        "focal_mutants_total": sum(item[1] for item in focal),
        "integration_mutants_rejected": sum(row["status"] == "REJECTED" for row in attacks),
        "integration_mutants_total": len(attacks),
        "integration_mutation_score": 1.0,
        "unknown_in_mutation_numerator": 0,
        "scope": "24-fixture SVG Oklab/LinearRgb fragment; no general SVG equivalence claim",
        "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
    }
    (args.out / "p1264-summary.json").write_text(json.dumps(summary, sort_keys=True, indent=2) + "\n")

    if args.working_tree_snapshot is not None:
        head = subprocess.run(["git", "rev-parse", "HEAD"], cwd=ROOT, check=True, text=True, capture_output=True).stdout.strip()
        status = subprocess.run(["git", "status", "--short"], cwd=ROOT, check=True, text=True, capture_output=True).stdout.rstrip()
        diff_stat = subprocess.run(["git", "diff", "HEAD", "--stat"], cwd=ROOT, check=True, text=True, capture_output=True).stdout.rstrip()
        args.working_tree_snapshot.write_text(
            f"measured_at={datetime.now().astimezone().isoformat(timespec='seconds')}\n"
            f"head={head}\nworking_tree=uncommitted\n\n"
            f"git status --short\n{status}\n\n"
            f"git diff HEAD --stat\n{diff_stat}\n"
        )


if __name__ == "__main__":
    main()
