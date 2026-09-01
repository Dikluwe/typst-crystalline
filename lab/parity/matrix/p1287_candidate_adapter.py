#!/usr/bin/env python3
"""Remove vanilla-only measurement flags without emulating product features."""

from __future__ import annotations

import os
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[3]
CANDIDATE = ROOT / "target/release/typst"

_REMOVABLE_PAIRS = {
    "--creation-timestamp": "0",
    "--jobs": "1",
    "--diagnostic-format": "short",
    "--ppi": "144",
}


def adapt(args: list[str]) -> list[str]:
    adapted: list[str] = []
    index = 0
    while index < len(args):
        option = args[index]
        expected = _REMOVABLE_PAIRS.get(option)
        if expected is not None and index + 1 < len(args) and args[index + 1] == expected:
            index += 2
            continue
        adapted.append(option)
        index += 1
    return adapted


def main() -> int:
    if not CANDIDATE.is_file():
        print(f"candidate binary missing: {CANDIDATE}", file=sys.stderr)
        return 127
    argv = [str(CANDIDATE), *adapt(sys.argv[1:])]
    os.execv(str(CANDIDATE), argv)
    return 127


if __name__ == "__main__":
    raise SystemExit(main())
