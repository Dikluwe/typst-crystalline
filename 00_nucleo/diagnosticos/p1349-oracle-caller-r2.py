#!/usr/bin/env python3
"""External P1349 oracle R2 caller; pins checker, receipt and authoring root."""

from __future__ import annotations

import hashlib
import importlib.util
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
CHECKER_REL = "00_nucleo/diagnosticos/p1349-oracle-checker-r2.py"
RECEIPT_REL = "00_nucleo/diagnosticos/p1349-oracle-authorship-receipt-r2.json"
CHECKER_SHA256 = "999971b4e25c6a44ae76542d0115d8304c3a8f28afbaf26bc1c9347db6edc267"
RECEIPT_SHA256 = "d95113d55015300108e5571fe29d6b83b17a34bc37aa7dfa01be04e934fb6ab8"
AUTHORING_ROOT_SHA256 = "10cb199eea90852052d606937008eca8f5afe5bf5c289566cb14d064e4aa4117"


def sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def validate_chain() -> Any:
    checker_path, receipt_path = ROOT / CHECKER_REL, ROOT / RECEIPT_REL
    for path, expected in ((checker_path, CHECKER_SHA256), (receipt_path, RECEIPT_SHA256)):
        if path.is_symlink() or not path.is_file() or sha256(path.read_bytes()) != expected:
            raise RuntimeError(f"AUTHORITY_ROOT: P1349 R2 drift {path.name}")
    spec = importlib.util.spec_from_file_location("p1349_checker_r2_loaded_by_caller", checker_path)
    if spec is None or spec.loader is None:
        raise RuntimeError("AUTHORITY_ROOT: P1349 R2 checker loader")
    checker = importlib.util.module_from_spec(spec); sys.modules[spec.name] = checker; spec.loader.exec_module(checker)
    checker.validate_protected_inputs(); checker.load_corpus()
    checker.validate_authorship_receipt(RECEIPT_SHA256, CHECKER_SHA256, AUTHORING_ROOT_SHA256)
    if checker.oracle_authoring_root(CHECKER_SHA256) != AUTHORING_ROOT_SHA256:
        raise RuntimeError("AUTHORITY_ROOT: P1349 R2 authoring root")
    return checker


def run_focal() -> dict[str, Any]:
    checker = validate_chain()
    return checker.run_focal(CHECKER_SHA256, RECEIPT_SHA256, AUTHORING_ROOT_SHA256)


def main() -> int:
    if sys.argv[1:] != ["--focus"]:
        sys.stderr.write("AUTHORITY_ROOT: P1349 R2 caller accepts exactly --focus\n"); return 2
    try:
        checker = validate_chain(); report = checker.run_focal(CHECKER_SHA256, RECEIPT_SHA256, AUTHORING_ROOT_SHA256); sys.stdout.buffer.write(checker.canonical(report)); return 0 if report["verdict"] == "ORACLE_R2_NONPTRACE_FOCAL_NOT_SEALED" else 1
    except Exception as exc:
        detail = str(exc); sys.stderr.write((detail if detail.startswith(("AUTHORITY_ROOT:", "PROTECTED_INPUT:")) else f"AUTHORITY_ROOT: P1349 R2 caller {type(exc).__name__}") + "\n"); return 2


if __name__ == "__main__":
    raise SystemExit(main())
