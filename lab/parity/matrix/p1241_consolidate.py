#!/usr/bin/env python3
"""P1241: consolidate the current conservative SVG multispace verdict."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
from pathlib import Path


KINDS = ("Linear", "Radial")
SPACES = ("Srgb", "Oklab", "Oklch", "LinearRgb", "Luma", "Hsl", "Hsv", "Cmyk")


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def tsv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def write(path: Path, rows: list[dict[str, object]]) -> None:
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, list(rows[0]), delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--p1237", type=Path, required=True)
    parser.add_argument("--p1238", type=Path, required=True)
    parser.add_argument("--p1239", type=Path, required=True)
    parser.add_argument("--p1240", type=Path, required=True)
    parser.add_argument("--svg-l0", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--reverse", action="store_true")
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)

    p1237 = {row["pair"]: row for row in tsv(args.p1237)}
    p1238 = {row["pair"]: row for row in tsv(args.p1238)}
    p1239 = json.loads(args.p1239.read_text())
    p1240 = json.loads(args.p1240.read_text())
    svg_l0 = args.svg_l0.read_text()

    if len(p1237) != 4 or any(not row["classification"].startswith("Unknown") for row in p1237.values()):
        raise ValueError("P1237 must retain four Unknown pairs")
    if len(p1238) != 6 or any(row["classification"] != "Preserved" for row in p1238.values()):
        raise ValueError("P1238 L1 polar fragment is incomplete")
    if p1239["p1236_violated_luma"] != 0 or p1239["p1236_unknown"] != 0:
        raise ValueError("P1239 Luma fragment is incomplete")
    if not p1240["reverse_stable"] or any(row["verdict"] != "Preserved" for row in p1240["rows"]):
        raise ValueError("P1240 auxiliary raster harness is not stable")
    required_l0 = "Somente\nos controles Linear/Radial sRGB reutilizam a geometria nativa"
    if required_l0 not in svg_l0:
        raise ValueError("current SVG L0 does not retain the conservative sRGB boundary")

    kinds = tuple(reversed(KINDS)) if args.reverse else KINDS
    spaces = tuple(reversed(SPACES)) if args.reverse else SPACES
    results: list[dict[str, object]] = []
    for kind in kinds:
        for space in spaces:
            pair = f"{kind}/{space}"
            if space == "Srgb":
                classification, evidence = "Preserved", "current SVG L0 native sRGB boundary; P1240 auxiliary raster stable"
            elif space == "Cmyk":
                classification, evidence = "Unknown-ADR0097", "ICC/profile absent"
            elif space in {"Oklab", "LinearRgb"}:
                source = p1237[f"{kind.lower()}/{'linear-rgb' if space == 'LinearRgb' else 'oklab'}"]
                classification, evidence = source["classification"], f"P1237 numeric {source['numeric_pass']}/{source['numeric_total']}"
            elif space in {"Oklch", "Hsl", "Hsv"}:
                source = p1238[pair]
                classification, evidence = "Unknown", f"P1238 L1 {source['preserved']}/{source['total']}; SVG budget absent"
            else:
                classification, evidence = "Unknown", "P1239 Luma public fragment preserved; SVG budget absent"
            results.append({"pair": pair, "classification": classification, "evidence": evidence})
    results.sort(key=lambda row: row["pair"])
    write(args.out / "results.tsv", results)
    inputs = [{"input": name, "path": str(path), "sha256": sha(path)} for name, path in (
        ("p1237", args.p1237), ("p1238", args.p1238), ("p1239", args.p1239),
        ("p1240", args.p1240), ("svg_l0", args.svg_l0))]
    write(args.out / "inputs.tsv", inputs)
    summary = {
        "pairs": len(results),
        "preserved": sum(row["classification"] == "Preserved" for row in results),
        "unknown": sum(row["classification"] != "Preserved" for row in results),
        "conic_scope": "Unknown; P1240 has no path/wedge Conic coverage",
        "mutation_score": None,
        "attack_regime": "not executed; logical policy checks only",
        "isolation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
    }
    (args.out / "summary.json").write_text(json.dumps(summary, sort_keys=True, indent=2) + "\n")


if __name__ == "__main__":
    main()
