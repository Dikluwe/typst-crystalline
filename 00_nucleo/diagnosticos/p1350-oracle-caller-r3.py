#!/usr/bin/env python3
"""External P1350 Oracle R3 authorship caller; no successor edge."""

from __future__ import annotations

import hashlib
import importlib.util
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
CHECKER_REL = "00_nucleo/diagnosticos/p1350-oracle-checker-r3.py"
RECEIPT_REL = "00_nucleo/diagnosticos/p1350-oracle-authorship-receipt-r3.json"
CHECKER_SHA256 = "06ba8291746116d36809a85773f8b4945fb479fbdfa7c13aa3e10e5bea9e153d"
RECEIPT_SHA256 = "e92c88ecc467e3ce6323bf13cf72fe6716264953ea0b0576249618879ddecd5d"
AUTHORING_ROOT_SHA256 = "79ee6301cf22a9d5da725606c1859bffadd2e21c5299e3f7e71761a708fcd7ec"


def sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def validate_chain() -> Any:
    checker_path, receipt_path = ROOT / CHECKER_REL, ROOT / RECEIPT_REL
    for path, expected in ((checker_path, CHECKER_SHA256), (receipt_path, RECEIPT_SHA256)):
        if path.is_symlink() or not path.is_file() or sha256(path.read_bytes()) != expected:
            raise RuntimeError(f"AUTHORITY_ROOT: P1350 R3 drift {path.name}")
    spec = importlib.util.spec_from_file_location("p1350_checker_r3_loaded_by_caller", checker_path)
    if spec is None or spec.loader is None: raise RuntimeError("AUTHORITY_ROOT: P1350 R3 checker loader")
    checker = importlib.util.module_from_spec(spec); sys.modules[spec.name] = checker; spec.loader.exec_module(checker)
    checker.validate_protected_inputs(); checker.load_corpus(); checker.validate_authorship_receipt(RECEIPT_SHA256, CHECKER_SHA256, AUTHORING_ROOT_SHA256)
    if checker.oracle_authoring_root(CHECKER_SHA256) != AUTHORING_ROOT_SHA256: raise RuntimeError("AUTHORITY_ROOT: P1350 R3 authoring root")
    return checker


def run_focal() -> dict[str, Any]:
    return validate_chain().run_focal(CHECKER_SHA256, RECEIPT_SHA256, AUTHORING_ROOT_SHA256)


def main() -> int:
    if sys.argv[1:] != ["--focus"]:
        sys.stderr.write("AUTHORITY_ROOT: P1350 R3 caller accepts exactly --focus\n"); return 2
    try:
        checker = validate_chain(); report = checker.run_focal(CHECKER_SHA256, RECEIPT_SHA256, AUTHORING_ROOT_SHA256); sys.stdout.buffer.write(checker.canonical(report)); return 0 if report["verdict"] == "ORACLE_R3_NONPTRACE_FOCAL_NOT_SEALED" else 1
    except Exception as exc:
        detail = str(exc); sys.stderr.write((detail if detail.startswith(("AUTHORITY_ROOT:", "PROTECTED_INPUT:")) else f"AUTHORITY_ROOT: P1350 R3 caller {type(exc).__name__}") + "\n"); return 2


if __name__ == "__main__":
    raise SystemExit(main())
