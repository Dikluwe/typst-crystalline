#!/usr/bin/env python3
"""External P1347 R2 caller; pins checker, authorship receipt and root only."""

from __future__ import annotations

import hashlib
import importlib.util
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
CHECKER_REL = "00_nucleo/diagnosticos/p1347-oracle-checker-r2.py"
RECEIPT_REL = "00_nucleo/diagnosticos/p1347-oracle-authorship-receipt-r2.json"
CHECKER_SHA256 = "9aa0e93591195eac7ddb67069abecbcf0722abeddd9e955fc062804a89d16ce5"
RECEIPT_SHA256 = "fb66120dd74f102139e2db839516c716793c5d852dfce1f6fb414d15b67a681c"
AUTHORING_ROOT_SHA256 = "b78fca0360ce823d1fbc9c996609e1278f131c6ce318c89067d56e5c346e0787"


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def validate_chain() -> Any:
    checker_path = ROOT / CHECKER_REL
    receipt_path = ROOT / RECEIPT_REL
    for path, expected in ((checker_path, CHECKER_SHA256), (receipt_path, RECEIPT_SHA256)):
        if path.is_symlink() or not path.is_file() or sha256(path.read_bytes()) != expected:
            raise RuntimeError(f"AUTHORITY_ROOT: R2 drift {path.name}")
    spec = importlib.util.spec_from_file_location("p1347_checker_r2_loaded_by_caller", checker_path)
    if spec is None or spec.loader is None:
        raise RuntimeError("AUTHORITY_ROOT: R2 checker loader")
    checker = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = checker
    spec.loader.exec_module(checker)
    checker.validate_protected_inputs()
    checker.load_corpus()
    checker.validate_authorship_receipt(RECEIPT_SHA256, CHECKER_SHA256, AUTHORING_ROOT_SHA256)
    if checker.authoring_root(CHECKER_SHA256) != AUTHORING_ROOT_SHA256:
        raise RuntimeError("AUTHORITY_ROOT: R2 authoring root")
    return checker


def run_focal() -> dict[str, Any]:
    checker = validate_chain()
    return checker.run_focal(CHECKER_SHA256, RECEIPT_SHA256, AUTHORING_ROOT_SHA256)


def main() -> int:
    if sys.argv[1:] != ["--focus"]:
        sys.stderr.write("AUTHORITY_ROOT: R2 caller accepts exactly --focus\n")
        return 2
    try:
        checker = validate_chain()
        report = checker.run_focal(CHECKER_SHA256, RECEIPT_SHA256, AUTHORING_ROOT_SHA256)
        sys.stdout.buffer.write(checker.canonical(report))
        return 0 if report["verdict"] == "FOCAL_AUTHORED_R2_NOT_VERIFIED_NOT_SEALED" else 1
    except Exception as exc:
        detail = str(exc)
        sys.stderr.write((detail if detail.startswith(("AUTHORITY_ROOT:", "PROTECTED_INPUT:")) else f"AUTHORITY_ROOT: R2 caller {type(exc).__name__}") + "\n")
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
