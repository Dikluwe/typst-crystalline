#!/usr/bin/env python3
"""P1279: executable witnesses for lossless diagnostic offset transport."""

from __future__ import annotations

import argparse
import csv
import json
import struct
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import p1277_generalize as harness


FIXTURES = (
    "S06-linear-oklch",
    "S11-linear-hsv",
    "S16-linear-oklch",
    "S16-linear-hsl",
    "S16-linear-hsv",
)


def write_tsv(path: Path, rows: list[dict[str, object]]) -> None:
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, list(rows[0]), delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def f32(value: float) -> float:
    return struct.unpack("!f", struct.pack("!f", value))[0]


def probe(
    binary: Path,
    fixture: harness.Fixture,
    meta: list[object],
    mode: str,
) -> list[tuple[float, tuple[int, int, int, int]]]:
    rows = []
    for index, (components, evaluated_offset) in enumerate(meta[0]):
        if mode == "p15":
            offset = format(float(evaluated_offset), ".15g")
        elif mode == "pre-eval" and fixture.seed["operation"] == "constructor":
            offset = format(float(fixture.source_offsets[index]), ".17g")
        elif mode == "f32":
            offset = format(f32(float(evaluated_offset)), ".17g")
        else:
            offset = format(float(evaluated_offset), ".17g")
        rows.append(",".join([*(harness.fmt(float(value)) for value in components), offset]))

    completed = harness.run(
        [
            str(binary),
            fixture.pair.split("/")[0],
            fixture.pair.split("/")[1],
            str(fixture.anti_alias).lower(),
            ";".join(rows),
        ]
    )
    observed = list(csv.DictReader(completed.stdout.splitlines(), delimiter="\t"))
    output = []
    for row in observed:
        text = row["offset_xml"]
        offset = float(text[:-1]) / 100.0 if text.endswith("%") else float(text)
        output.append(
            (
                offset,
                (int(row["r"]), int(row["g"]), int(row["b"]), int(row["a"])),
            )
        )
    return output


def vanilla_stops(path: Path, kind: str) -> list[tuple[float, tuple[int, int, int, int]]]:
    return [
        (
            offset,
            tuple(round(component * 255.0) for component in color),
        )
        for offset, color in harness.server_stops(path, kind)
    ]


def deltas(
    vanilla: list[tuple[float, tuple[int, int, int, int]]],
    candidate: list[tuple[float, tuple[int, int, int, int]]],
) -> tuple[int, int, bool]:
    offsets = sum(left[0] != right[0] for left, right in zip(vanilla, candidate))
    colors = sum(left[1] != right[1] for left, right in zip(vanilla, candidate))
    return offsets, colors, len(vanilla) == len(candidate)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--matrix", type=Path, required=True)
    parser.add_argument("--candidate", type=Path, required=True)
    parser.add_argument("--probe", type=Path, required=True)
    parser.add_argument("--oracle-run", type=Path, required=True)
    parser.add_argument("--product-boundary", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--reverse", action="store_true")
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)

    seeds = {
        row["seed"]: row
        for row in harness.read_tsv(args.matrix)
        if row["validity"] == "valid"
    }
    fixture_ids = tuple(reversed(FIXTURES)) if args.reverse else FIXTURES
    witnesses = []
    for fixture_id in fixture_ids:
        seed, kind, space = fixture_id.split("-")
        fixture = harness.make_fixture(seeds[seed], f"{kind}/{space}")
        meta = harness.public_observation(args.candidate, fixture.expression, mesh=False)["meta"]
        vanilla = vanilla_stops(args.oracle_run / f"oracle-{fixture_id}.svg", kind)
        measurements = {
            mode: deltas(vanilla, probe(args.probe, fixture, meta, mode))
            for mode in ("lossless", "p15", "pre-eval", "f32")
        }
        witnesses.append(
            {
                "fixture_id": fixture_id,
                "pair": fixture.pair,
                "vanilla_stops": len(vanilla),
                "lossless_offset_diffs": measurements["lossless"][0],
                "lossless_color_diffs": measurements["lossless"][1],
                "lossless_count_equal": str(measurements["lossless"][2]).lower(),
                "p15_offset_diffs": measurements["p15"][0],
                "pre_eval_offset_diffs": measurements["pre-eval"][0],
                "f32_offset_diffs": measurements["f32"][0],
                "status": (
                    "Preserved"
                    if measurements["lossless"] == (0, 0, True)
                    else "Violated"
                ),
            }
        )
    witnesses.sort(key=lambda row: row["fixture_id"])

    p15_diffs = sum(int(row["p15_offset_diffs"]) for row in witnesses)
    pre_eval_diffs = sum(int(row["pre_eval_offset_diffs"]) for row in witnesses)
    f32_diffs = sum(int(row["f32_offset_diffs"]) for row in witnesses)
    color_mutant = [item for item in vanilla]
    offset, color = color_mutant[0]
    color_mutant[0] = (offset, ((color[0] + 1) % 256, *color[1:]))
    color_mutant_detected = deltas(vanilla, color_mutant)[1] == 1
    product = harness.read_tsv(args.product_boundary)
    fallback_preserved = all(row["productive_promotion"] == "false" for row in product)

    attacks = [
        {
            "mutant_id": "O01-f64-as-15-digits",
            "witness": f"{p15_diffs} offset mismatches across five stopsets",
            "status": "REJECTED" if p15_diffs > 0 else "SURVIVED",
        },
        {
            "mutant_id": "O02-pre-evaluation-offsets",
            "witness": f"{pre_eval_diffs} mismatches; S06 constructor is decisive",
            "status": "REJECTED" if pre_eval_diffs > 0 else "SURVIVED",
        },
        {
            "mutant_id": "O03-offsets-through-f32",
            "witness": f"{f32_diffs} offset mismatches across five stopsets",
            "status": "REJECTED" if f32_diffs > 0 else "SURVIVED",
        },
        {
            "mutant_id": "O04-cardinality-only",
            "witness": "all cardinalities equal while p15 still changes offsets",
            "status": (
                "REJECTED"
                if all(row["lossless_count_equal"] == "true" for row in witnesses)
                and p15_diffs > 0
                else "SURVIVED"
            ),
        },
        {
            "mutant_id": "O05-ignore-color",
            "witness": "synthetic one-byte color mutation detected",
            "status": "REJECTED" if color_mutant_detected else "SURVIVED",
        },
        {
            "mutant_id": "O06-promote-product",
            "witness": "192/192 product groups retain fallback",
            "status": "REJECTED" if fallback_preserved else "SURVIVED",
        },
    ]
    if any(row["status"] != "Preserved" for row in witnesses):
        raise ValueError("lossless witness failed")
    if any(row["status"] != "REJECTED" for row in attacks):
        raise ValueError("P1279 offset mutant survived")

    write_tsv(args.out / "p1279-offset-witnesses.tsv", witnesses)
    write_tsv(args.out / "p1279-offset-attacks.tsv", attacks)
    (args.out / "p1279-offset-summary.json").write_text(
        json.dumps(
            {
                "fixtures": len(witnesses),
                "lossless_exact": sum(row["status"] == "Preserved" for row in witnesses),
                "p15_offset_diffs": p15_diffs,
                "pre_eval_offset_diffs": pre_eval_diffs,
                "f32_offset_diffs": f32_diffs,
                "mutants": len(attacks),
                "mutants_rejected": sum(row["status"] == "REJECTED" for row in attacks),
                "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
            },
            sort_keys=True,
            indent=2,
        )
        + "\n"
    )


if __name__ == "__main__":
    main()
