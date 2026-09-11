#!/usr/bin/env python3
"""External P1348 R1 caller; pins checker, authorship receipt and root only."""

from __future__ import annotations

import hashlib
import importlib.util
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
CHECKER_REL = "00_nucleo/diagnosticos/p1348-oracle-checker-r1.py"
RECEIPT_REL = "00_nucleo/diagnosticos/p1348-oracle-authorship-receipt-r1.json"
CHECKER_SHA256 = "fd1c4a0e1601ea817ffb92ad6723367083ac9f5c7a0f9bcecb155e9148764249"
RECEIPT_SHA256 = "54f03388e58e5adb1acd55f27a19e8a0f0e28072c4e8ee3f9d23949c860bf24f"
AUTHORING_ROOT_SHA256 = "bfc5887f60d953ba14031b1df0327f51ce78c6a7cae7c773de32bce929defc39"


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def validate_chain() -> Any:
    checker_path = ROOT / CHECKER_REL
    receipt_path = ROOT / RECEIPT_REL
    for path, expected in ((checker_path, CHECKER_SHA256), (receipt_path, RECEIPT_SHA256)):
        if path.is_symlink() or not path.is_file() or sha256(path.read_bytes()) != expected:
            raise RuntimeError(f"AUTHORITY_ROOT: P1348 R1 drift {path.name}")
    spec = importlib.util.spec_from_file_location("p1348_checker_r1_loaded_by_caller", checker_path)
    if spec is None or spec.loader is None:
        raise RuntimeError("AUTHORITY_ROOT: P1348 R1 checker loader")
    checker = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = checker
    spec.loader.exec_module(checker)
    checker.validate_protected_inputs()
    checker.load_corpus()
    checker.validate_authorship_receipt(RECEIPT_SHA256, CHECKER_SHA256, AUTHORING_ROOT_SHA256)
    if checker.oracle_authoring_root(CHECKER_SHA256) != AUTHORING_ROOT_SHA256:
        raise RuntimeError("AUTHORITY_ROOT: P1348 R1 authoring root")
    return checker


def run_focal() -> dict[str, Any]:
    checker = validate_chain()
    return checker.run_focal(CHECKER_SHA256, RECEIPT_SHA256, AUTHORING_ROOT_SHA256)


def main() -> int:
    if sys.argv[1:] != ["--focus"]:
        sys.stderr.write("AUTHORITY_ROOT: P1348 R1 caller accepts exactly --focus\n")
        return 2
    try:
        checker = validate_chain()
        report = checker.run_focal(CHECKER_SHA256, RECEIPT_SHA256, AUTHORING_ROOT_SHA256)
        sys.stdout.buffer.write(checker.canonical(report))
        return 0 if report["verdict"] == "ORACLE_NONPTRACE_FOCAL_NOT_SEALED" else 1
    except Exception as exc:
        detail = str(exc)
        sys.stderr.write((detail if detail.startswith(("AUTHORITY_ROOT:", "PROTECTED_INPUT:")) else f"AUTHORITY_ROOT: P1348 R1 caller {type(exc).__name__}") + "\n")
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
