#!/usr/bin/env python3
"""P1278: rerun the frozen P1277 public-language focal checks."""

from __future__ import annotations

import json
import sys
from pathlib import Path

import p1277_focal


def main() -> None:
    try:
        out = Path(sys.argv[sys.argv.index("--out") + 1])
    except (ValueError, IndexError) as error:
        raise SystemExit("--out is required") from error

    p1277_focal.main()
    for source in sorted(out.glob("p1277-*")):
        source.replace(out / source.name.replace("p1277-", "p1278-", 1))

    summary_path = out / "p1278-focal-summary.json"
    summary = json.loads(summary_path.read_text())
    summary.update(
        {
            "step": 1278,
            "scope": "frozen P1277 focal language checks rerun after L3-only refinement",
            "attestation": "EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO",
        }
    )
    summary_path.write_text(json.dumps(summary, sort_keys=True, indent=2) + "\n")


if __name__ == "__main__":
    main()
