#!/usr/bin/env python3
"""Extrai os 15 presets P1284 da fonte vanilla pinada para dados L1."""

from __future__ import annotations

import hashlib
import pathlib
import re


ROOT = pathlib.Path(__file__).resolve().parents[2]
SOURCE = ROOT / "lab/typst-original/crates/typst-library/src/visualize/color.rs"
OUTPUT = ROOT / "01_core/src/compiler/stdlib/color_maps.hex"
EXPECTED_SOURCE_SHA256 = (
    "80473eba7460cb0f398e7937946e6412c1a8cb1cadffe00580aea9b48f6c0713"
)
NAMES = (
    "turbo", "cividis", "rainbow", "spectral", "viridis", "inferno",
    "magma", "plasma", "rocket", "mako", "coolwarm", "vlag", "icefire",
    "flare", "crest",
)


def main() -> None:
    raw = SOURCE.read_bytes()
    actual = hashlib.sha256(raw).hexdigest()
    if actual != EXPECTED_SOURCE_SHA256:
        raise SystemExit(f"fonte vanilla inesperada: {actual}")
    text = raw.decode()
    rows = []
    for name in NAMES:
        match = re.search(rf"preset!\({name};(.*?)\);", text, re.S)
        if match is None:
            raise SystemExit(f"preset ausente: {name}")
        values = []
        for token in re.findall(r"0x([0-9a-fA-F]+)", match.group(1)):
            values.append(token.lower().zfill(8))
        if not values:
            raise SystemExit(f"preset vazio: {name}")
        rows.append(f"{name} {' '.join(values)}")
    OUTPUT.write_text("\n".join(rows) + "\n")


if __name__ == "__main__":
    main()
