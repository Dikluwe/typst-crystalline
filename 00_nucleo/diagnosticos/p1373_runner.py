#!/usr/bin/env python3
"""Reexecucao pos-correcao da sonda S07 congelada no P1372."""

from __future__ import annotations

import argparse
import importlib.util
import json
from pathlib import Path
from typing import Any


P1372_RUNNER_SHA256 = "b474cb94aa47785dc8dff0b709e77a40679e6a398fcd64ce8f97a863f5f74e2c"
CRISTALINO_SHA256 = "0b318c3fa19863ac2d788efd734334eafe1d84be975af328d6c6ade1f5124e15"


def _load_p1372() -> Any:
    path = Path(__file__).with_name("p1372_runner.py")
    spec = importlib.util.spec_from_file_location("p1372_runner_for_p1373", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("P1372 runner loader unavailable")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


P1372 = _load_p1372()


def execute(args: argparse.Namespace) -> dict[str, Any]:
    runner_path = Path(__file__).with_name("p1372_runner.py")
    if P1372.BOUNDED.sha256_path(runner_path) != P1372_RUNNER_SHA256:
        raise ValueError("frozen P1372 runner identity mismatch")

    previous_hash = P1372.EXPECTED_HASHES["cristalino"]
    P1372.EXPECTED_HASHES["cristalino"] = CRISTALINO_SHA256
    try:
        result = P1372.execute(args)
    finally:
        P1372.EXPECTED_HASHES["cristalino"] = previous_hash

    result["schema"] = "p1373-s07-post-fix/v1"
    result["source_runner"] = {
        "path": str(runner_path),
        "sha256": P1372_RUNNER_SHA256,
    }
    return result


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--vanila", required=True)
    parser.add_argument("--cristalino", required=True)
    parser.add_argument("--pdftotext", required=True)
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    result = execute(args)
    Path(args.output).write_text(
        json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
    )


if __name__ == "__main__":
    main()
