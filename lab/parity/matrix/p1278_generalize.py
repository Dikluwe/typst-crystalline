#!/usr/bin/env python3
"""P1278: rerun P1277's sealed matrix with the refined polar adapter.

The frozen P1277 harness remains byte-preserved. This wrapper only selects the
P1278 probe and renames the generated receipts after the complete run, keeping
Luma as a negative control outside the implemented polar refinement.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

import p1277_generalize


def main() -> None:
    try:
        out = Path(sys.argv[sys.argv.index("--out") + 1])
    except (ValueError, IndexError) as error:
        raise SystemExit("--out is required") from error

    p1277_generalize.main()

    for source in sorted(out.glob("p1277-*")):
        source.replace(out / source.name.replace("p1277-", "p1278-", 1))

    summary_path = out / "p1278-summary.json"
    summary = json.loads(summary_path.read_text())
    summary.update(
        {
            "step": 1278,
            "scope": (
                "P1277 sealed 192-fixture matrix rerun after internal polar "
                "sampler/gamut/Ratio::repr refinement; Luma is a control"
            ),
            "polar_pairs": [
                "linear/oklch",
                "radial/oklch",
                "linear/hsl",
                "radial/hsl",
                "linear/hsv",
                "radial/hsv",
            ],
            "excluded_from_refinement": ["linear/luma", "radial/luma"],
            "productive_promotions_applied": 0,
            "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
        }
    )
    summary_path.write_text(json.dumps(summary, sort_keys=True, indent=2) + "\n")


if __name__ == "__main__":
    main()
