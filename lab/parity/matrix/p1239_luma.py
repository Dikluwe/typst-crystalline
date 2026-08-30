#!/usr/bin/env python3
"""P1239: verify public Luma witnesses and bind the current P1236 corpus."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import subprocess
from pathlib import Path


WITNESSES = {
    "public-luma-red": "repr(luma(red).components().at(0))",
    "linear-luma-red-endpoint": "repr(gradient.linear(red, blue, space: luma).stops().at(0).at(0).components().at(0))",
    "radial-luma-red-endpoint": "repr(gradient.radial(red, blue, space: luma).stops().at(0).at(0).components().at(0))",
}


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def digest(data: str) -> str:
    return hashlib.sha256(data.encode()).hexdigest()


def write(path: Path, rows: list[dict[str, object]]) -> None:
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, list(rows[0]), delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def evaluate(binary: Path, expression: str) -> dict[str, object]:
    argv = [str(binary), "eval", "--format", "json", expression]
    cp = subprocess.run(argv, text=True, capture_output=True)
    value = ""
    status = "Unknown"
    if cp.returncode == 0:
        try:
            parsed = json.loads(cp.stdout)
            if isinstance(parsed, str):
                value, status = parsed, "Available"
        except Exception:
            pass
    return {
        "argv": json.dumps(argv, separators=(",", ":")),
        "exit_status": cp.returncode,
        "value": value,
        "status": status,
        "stdout_sha256": digest(cp.stdout),
        "stderr_sha256": digest(cp.stderr),
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--vanilla", type=Path, required=True)
    parser.add_argument("--crystalline", type=Path, required=True)
    parser.add_argument("--p1236-summary", type=Path, required=True)
    parser.add_argument("--p1236-metrics", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)

    p1236 = json.loads(args.p1236_summary.read_text())
    if p1236["pairs"] != 42 or p1236["violated_luma"] != 0 or p1236["unknown"] != 0:
        raise ValueError("current P1236 corpus does not close Luma")

    binaries = {"vanilla": args.vanilla, "crystalline": args.crystalline}
    observations: list[dict[str, object]] = []
    commands: list[dict[str, object]] = []
    for witness, expression in WITNESSES.items():
        for system, binary in binaries.items():
            result = evaluate(binary, expression)
            observations.append({"witness": witness, "system": system,
                "public_value": result["value"], "status": result["status"],
                "verdict": "Preserved" if result["value"] == "54.02%" else
                           ("Unknown" if result["status"] == "Unknown" else "Violated")})
            commands.append({"witness": witness, "system": system, "argv": result["argv"],
                "binary_sha256": sha(binary), "exit_status": result["exit_status"],
                "stdout_sha256": result["stdout_sha256"], "stderr_sha256": result["stderr_sha256"]})

    observations.sort(key=lambda row: (row["witness"], row["system"]))
    commands.sort(key=lambda row: (row["witness"], row["system"]))
    write(args.out / "observations.tsv", observations)
    write(args.out / "commands.tsv", commands)
    summary = {
        "public_witnesses": len(WITNESSES),
        "public_comparisons": len(WITNESSES),
        "public_preserved": sum(
            all(row["verdict"] == "Preserved" for row in observations if row["witness"] == witness)
            for witness in WITNESSES
        ),
        "p1236_pairs": p1236["pairs"],
        "p1236_violated_luma": p1236["violated_luma"],
        "p1236_preserved": p1236["preserved"],
        "p1236_alpha_divergent": p1236["alpha_divergent"],
        "p1236_unknown": p1236["unknown"],
        "p1236_summary_sha256": sha(args.p1236_summary),
        "p1236_metrics_sha256": sha(args.p1236_metrics),
        "mutation_score": None,
        "attack_regime": "not executed; no mutation claim",
        "isolation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
    }
    (args.out / "summary.json").write_text(json.dumps(summary, sort_keys=True, indent=2) + "\n")


if __name__ == "__main__":
    main()
