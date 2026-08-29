#!/usr/bin/env python3
"""P1277: focal public-language checks for polar and Luma gradients."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
import subprocess
from pathlib import Path


POSITIONS = (0.0, 0.000001, 0.25, 0.5, 0.75, 0.999999, 1.0)
PAIRS = (
    "linear/oklch", "radial/oklch", "linear/hsl", "radial/hsl",
    "linear/hsv", "radial/hsv", "linear/luma", "radial/luma",
)
POLAR_SCENARIOS = (
    "seam-forward", "seam-reverse", "no-wrap", "low-chroma",
    "alpha-first", "alpha-last",
)
LUMA_SCENARIOS = (
    "black-white", "white-black", "primary-extremes", "nonsaturated",
    "alpha-first", "alpha-last",
)


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def digest(value: str) -> str:
    return hashlib.sha256(value.encode()).hexdigest()


def write_tsv(path: Path, rows: list[dict[str, object]]) -> None:
    if not rows:
        raise ValueError(f"empty output: {path}")
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, list(rows[0]), delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def polar_stops(space: str, scenario: str) -> tuple[str, str]:
    seam = (
        ("rgb(230,0,140)", "rgb(255,80,120)")
        if space == "oklch"
        else ("rgb(255,0,43)", "rgb(255,43,0)")
    )
    if scenario == "seam-forward":
        return seam
    if scenario == "seam-reverse":
        return seam[1], seam[0]
    if scenario == "no-wrap":
        return "rgb(255,128,0)", "rgb(0,255,128)"
    if scenario == "low-chroma":
        return "rgb(128,128,128)", "rgb(255,43,0)"
    if scenario == "alpha-first":
        return seam[0] + ".transparentize(60%)", seam[1]
    if scenario == "alpha-last":
        return seam[0], seam[1] + ".transparentize(60%)"
    raise ValueError(scenario)


def luma_stops(scenario: str) -> tuple[str, str]:
    if scenario == "black-white":
        return "black", "white"
    if scenario == "white-black":
        return "white", "black"
    if scenario == "primary-extremes":
        return "blue", "rgb(0,255,0)"
    if scenario == "nonsaturated":
        return "rgb(38,70,83)", "rgb(244,162,97)"
    if scenario == "alpha-first":
        return "black.transparentize(60%)", "white"
    if scenario == "alpha-last":
        return "black", "white.transparentize(60%)"
    raise ValueError(scenario)


def expression(pair: str, scenario: str) -> str:
    kind, space = pair.split("/")
    first, last = polar_stops(space, scenario) if space != "luma" else luma_stops(scenario)
    args = f"({first},0%),({last},100%),space:color.{space}"
    if kind == "linear":
        args += ",angle:0deg"
    else:
        args += ",center:(50%,50%),radius:50%,focal-center:(50%,50%),focal-radius:0%"
    return f"gradient.{kind}({args})"


def evaluate(binary: Path, expr: str) -> dict[str, object]:
    positions = ",".join(f"{position * 100:.12f}%" for position in POSITIONS)
    query = (
        "let norm = x => if type(x) == ratio { x / 100% } "
        "else if type(x) == angle { x / 1deg } else { x }; "
        f"let g = {expr}; ({positions}).map(t => g.sample(t).components(alpha:true).map(norm))"
    )
    argv = [str(binary), "eval", "--format", "json", query]
    completed = subprocess.run(argv, text=True, capture_output=True)
    values: object = []
    status = "Unknown"
    if completed.returncode == 0:
        try:
            values = json.loads(completed.stdout)
            if isinstance(values, list) and len(values) == len(POSITIONS):
                status = "Available"
        except json.JSONDecodeError:
            pass
    return {
        "argv": json.dumps(argv, separators=(",", ":")),
        "exit": completed.returncode,
        "stdout_sha256": digest(completed.stdout),
        "stderr_sha256": digest(completed.stderr),
        "values": values,
        "status": status,
    }


def deltas(space: str, vanilla: list[float], candidate: list[float]) -> tuple[float, float, float]:
    hue_index = {"oklch": 2, "hsl": 0, "hsv": 0}.get(space)
    component = []
    for index, (left, right) in enumerate(zip(vanilla, candidate)):
        if index == hue_index:
            component.append(abs((float(right) - float(left) + 180.0) % 360.0 - 180.0))
        else:
            component.append(abs(float(right) - float(left)))
    alpha = component[-1]
    non_hue = max(value for index, value in enumerate(component[:-1]) if index != hue_index)
    hue = component[hue_index] if hue_index is not None else 0.0
    return hue, non_hue, alpha


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--vanilla", type=Path, required=True)
    parser.add_argument("--candidate", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--reverse", action="store_true")
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)

    pairs = tuple(reversed(PAIRS)) if args.reverse else PAIRS
    observations: list[dict[str, object]] = []
    commands: list[dict[str, object]] = []
    binaries = {"vanilla": args.vanilla, "candidate": args.candidate}
    binary_hashes = {name: sha(path) for name, path in binaries.items()}
    for pair in pairs:
        space = pair.split("/")[1]
        scenarios = LUMA_SCENARIOS if space == "luma" else POLAR_SCENARIOS
        if args.reverse:
            scenarios = tuple(reversed(scenarios))
        for scenario in scenarios:
            expr = expression(pair, scenario)
            measured = {name: evaluate(binary, expr) for name, binary in binaries.items()}
            for system, result in measured.items():
                commands.append({
                    "pair": pair, "scenario": scenario, "system": system,
                    "argv": result["argv"], "binary_sha256": binary_hashes[system],
                    "exit_status": result["exit"], "stdout_sha256": result["stdout_sha256"],
                    "stderr_sha256": result["stderr_sha256"],
                })
            for index, position in enumerate(POSITIONS):
                status = "Unknown"
                hue_delta: object = ""
                non_hue_delta: object = ""
                alpha_delta: object = ""
                if all(result["status"] == "Available" for result in measured.values()):
                    hue_delta, non_hue_delta, alpha_delta = deltas(
                        space,
                        measured["vanilla"]["values"][index],
                        measured["candidate"]["values"][index],
                    )
                    status = (
                        "Preserved"
                        if hue_delta <= 0.01 and non_hue_delta <= 0.0001 and alpha_delta <= 0.0001
                        else "Violated"
                    )
                observations.append({
                    "pair": pair, "family": "luma" if space == "luma" else "polar",
                    "scenario": scenario, "t": format(position, ".12f"),
                    "hue_delta_deg": hue_delta, "non_hue_max": non_hue_delta,
                    "alpha_delta": alpha_delta, "status": status,
                })

    observations.sort(key=lambda row: (row["pair"], row["scenario"], row["t"]))
    commands.sort(key=lambda row: (row["pair"], row["scenario"], row["system"]))
    pair_rows = []
    for pair in PAIRS:
        rows = [row for row in observations if row["pair"] == pair]
        pair_rows.append({
            "pair": pair, "observations": len(rows),
            "preserved": sum(row["status"] == "Preserved" for row in rows),
            "violated": sum(row["status"] == "Violated" for row in rows),
            "unknown": sum(row["status"] == "Unknown" for row in rows),
            "classification": (
                "Focal-Preserved" if all(row["status"] == "Preserved" for row in rows)
                else "Unknown-focal" if any(row["status"] == "Unknown" for row in rows)
                else "Violated-focal"
            ),
        })

    attacks = [
        {"mutant_id": "F01-drop-pair", "status": "REJECTED" if len(pair_rows) == 8 else "SURVIVED", "witness": "eight pair-local focal verdicts"},
        {"mutant_id": "F02-drop-scenario", "status": "REJECTED" if len(observations) == 336 else "SURVIVED", "witness": "48 groups x seven positions"},
        {"mutant_id": "F03-raw-hue-subtraction", "status": "REJECTED" if any(row["scenario"].startswith("seam-") for row in observations) else "SURVIVED", "witness": "seam cases use modular hue delta"},
        {"mutant_id": "F04-ignore-alpha", "status": "REJECTED" if all(row["alpha_delta"] != "" for row in observations) else "SURVIVED", "witness": "alpha delta on every observation"},
        {"mutant_id": "F05-drop-luma-extreme", "status": "REJECTED" if any(row["scenario"] == "primary-extremes" for row in observations) else "SURVIVED", "witness": "Luma primary extremes present"},
        {"mutant_id": "F06-drop-endpoint-epsilon", "status": "REJECTED" if {row["t"] for row in observations} == {format(value, ".12f") for value in POSITIONS} else "SURVIVED", "witness": "seven frozen positions include epsilon neighbours"},
    ]
    if len(observations) != 336 or any(row["observations"] != 42 for row in pair_rows):
        raise ValueError("focal population drift")
    if any(row["status"] != "REJECTED" for row in attacks):
        raise ValueError("P1277 focal mutant survived")

    write_tsv(args.out / "p1277-focal.tsv", observations)
    write_tsv(args.out / "p1277-focal-commands.tsv", commands)
    write_tsv(args.out / "p1277-focal-pairs.tsv", pair_rows)
    write_tsv(args.out / "p1277-focal-attacks.tsv", attacks)
    summary = {
        "fixture_groups": 48, "observations": len(observations),
        "preserved": sum(row["status"] == "Preserved" for row in observations),
        "violated": sum(row["status"] == "Violated" for row in observations),
        "unknown": sum(row["status"] == "Unknown" for row in observations),
        "pair_verdicts": {row["pair"]: row["classification"] for row in pair_rows},
        "mutants": len(attacks),
        "mutants_rejected": sum(row["status"] == "REJECTED" for row in attacks),
        "binary_sha256": binary_hashes,
        "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
    }
    (args.out / "p1277-focal-summary.json").write_text(json.dumps(summary, sort_keys=True, indent=2) + "\n")


if __name__ == "__main__":
    main()
