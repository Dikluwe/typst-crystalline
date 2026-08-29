#!/usr/bin/env python3
"""P1279: rerun the sealed matrix with lossless f64 offset transport."""

from __future__ import annotations

import csv
import json
import sys
from pathlib import Path

import p1277_generalize as harness


def probe_stops_lossless(
    probe: Path,
    fixture: harness.Fixture,
    meta: list[object],
) -> list[dict[str, object]]:
    encoded_rows = []
    for components, evaluated_offset in meta[0]:
        component_fields = [harness.fmt(float(value)) for value in components]
        offset_field = format(float(evaluated_offset), ".17g")
        encoded_rows.append(",".join([*component_fields, offset_field]))

    completed = harness.run(
        [
            str(probe),
            fixture.pair.split("/")[0],
            fixture.pair.split("/")[1],
            str(fixture.anti_alias).lower(),
            ";".join(encoded_rows),
        ]
    )
    rows = list(csv.DictReader(completed.stdout.splitlines(), delimiter="\t"))
    return [
        {
            "offset": float(row["offset"]),
            "offset_xml": row["offset_xml"],
            "r": int(row["r"]),
            "g": int(row["g"]),
            "b": int(row["b"]),
            "a": int(row["a"]),
        }
        for row in rows
    ]


def main() -> None:
    try:
        out = Path(sys.argv[sys.argv.index("--out") + 1])
    except (ValueError, IndexError) as error:
        raise SystemExit("--out is required") from error

    harness.probe_stops = probe_stops_lossless
    harness.main()

    for source in sorted(out.glob("p1277-*")):
        source.replace(out / source.name.replace("p1277-", "p1279-", 1))

    summary_path = out / "p1279-summary.json"
    summary = json.loads(summary_path.read_text())
    summary.update(
        {
            "step": 1279,
            "scope": (
                "P1278 sealed matrix rerun with evaluated offsets and .17g "
                "lossless f64 transport to the diagnostic probe"
            ),
            "harness_change_only": True,
            "productive_promotions_applied": 0,
            "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
        }
    )
    summary_path.write_text(json.dumps(summary, sort_keys=True, indent=2) + "\n")


if __name__ == "__main__":
    main()
