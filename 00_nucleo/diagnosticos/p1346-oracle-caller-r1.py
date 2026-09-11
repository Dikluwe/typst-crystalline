#!/usr/bin/env python3
"""External P1346 caller; validates the forward DAG before importing checker."""

from __future__ import annotations

import hashlib
import importlib.util
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
CHECKER_REL = "00_nucleo/diagnosticos/p1346-oracle-checker-r1.py"
RECEIPT_REL = "00_nucleo/diagnosticos/p1346-oracle-authorship-receipt-r1.json"
CORPUS_REL = "00_nucleo/diagnosticos/p1346-oracle-corpus-r1.json"
CHECKER_SHA256 = "aed47f690bc6fb1b24fc0711e7ccf5b6797a19055aa94478cb0b40fad133e13e"
RECEIPT_SHA256 = "79ee4e7a9e246393926cee0dbb29ed01478f7c5cff441c0de82dee9ae20e781c"
AUTHORING_ROOT_SHA256 = "c346067d7355418a5da0db985a49fbeb6ea28fffd3af6a6c348c3f75c1a51158"


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _fail(detail: str) -> None:
    raise RuntimeError(f"AUTHORITY_ROOT: {detail}")


def validate_chain() -> Any:
    checker_path = ROOT / CHECKER_REL
    receipt_path = ROOT / RECEIPT_REL
    corpus_path = ROOT / CORPUS_REL
    for path in (checker_path, receipt_path, corpus_path):
        if path.is_symlink() or not path.is_file():
            _fail(f"non-regular or symlink authority path {path.name}")
    if sha256(checker_path.read_bytes()) != CHECKER_SHA256:
        _fail("checker hash drift")
    if sha256(receipt_path.read_bytes()) != RECEIPT_SHA256:
        _fail("authorship receipt hash drift")
    spec = importlib.util.spec_from_file_location("p1346_checker_loaded_by_canonical_caller", checker_path)
    if spec is None or spec.loader is None:
        _fail("checker loader unavailable")
    checker = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = checker
    spec.loader.exec_module(checker)
    checker.validate_authorship_receipt(receipt_path, RECEIPT_SHA256, CHECKER_SHA256, AUTHORING_ROOT_SHA256)
    if checker.authoring_root(CHECKER_SHA256) != AUTHORING_ROOT_SHA256:
        _fail("authoring root drift")
    return checker


def run_focal() -> dict[str, Any]:
    checker = validate_chain()
    return checker.run_focal(CHECKER_SHA256, RECEIPT_SHA256, AUTHORING_ROOT_SHA256)


def main() -> int:
    if sys.argv[1:] != ["--focus"]:
        sys.stderr.write("SCHEMA: caller accepts exactly --focus\n")
        return 2
    try:
        checker = validate_chain()
        report = checker.run_focal(CHECKER_SHA256, RECEIPT_SHA256, AUTHORING_ROOT_SHA256)
        sys.stdout.buffer.write(checker.canonical_json(report))
        return 0 if report["verdict"] == "FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED" else 1
    except Exception as exc:
        detail = str(exc)
        if not detail.startswith("AUTHORITY_ROOT:"):
            detail = f"AUTHORITY_ROOT: closed caller failure {type(exc).__name__}"
        sys.stderr.write(detail + "\n")
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
