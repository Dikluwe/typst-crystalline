#!/usr/bin/env python3
"""Independent P1345 caller: the out-of-band trust anchor for Oracle R3."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
CANONICAL_CHECKER = "00_nucleo/diagnosticos/p1345-oracle-checker-r3.py"
CANONICAL_RECEIPT = "00_nucleo/diagnosticos/p1345-oracle-authorship-receipt-r3.json"
CANONICAL_CORPUS = "00_nucleo/diagnosticos/p1345-oracle-corpus-r3.json"
HEX = set("0123456789abcdef")


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def fail(detail: str) -> None:
    raise RuntimeError(f"AUTHORITY_ROOT: {detail}")


def exact_hex(value: str, label: str) -> None:
    if len(value) != 64 or any(char not in HEX for char in value):
        fail(f"{label} must be exact lowercase 32-byte hex")


def reject_duplicates(argv: list[str]) -> None:
    known = {"--focus", "--checker", "--corpus", "--authorship-receipt", "--expected-checker-sha256", "--expected-receipt-sha256", "--expected-authoring-root-sha256"}
    seen = set()
    for token in argv:
        flag = token.split("=", 1)[0]
        if flag in known:
            if flag in seen:
                fail(f"duplicate flag {flag}")
            seen.add(flag)


def main() -> int:
    reject_duplicates(sys.argv[1:])
    parser = argparse.ArgumentParser(allow_abbrev=False)
    parser.add_argument("--focus", action="store_true", required=True)
    parser.add_argument("--checker", required=True)
    parser.add_argument("--corpus", required=True)
    parser.add_argument("--authorship-receipt", required=True)
    parser.add_argument("--expected-checker-sha256", required=True)
    parser.add_argument("--expected-receipt-sha256", required=True)
    parser.add_argument("--expected-authoring-root-sha256", required=True)
    args = parser.parse_args()
    if args.checker != CANONICAL_CHECKER or args.corpus != CANONICAL_CORPUS or args.authorship_receipt != CANONICAL_RECEIPT:
        fail("only exact canonical textual paths are accepted")
    for value, label in [(args.expected_checker_sha256, "checker"), (args.expected_receipt_sha256, "receipt"), (args.expected_authoring_root_sha256, "root")]:
        exact_hex(value, label)
    checker_path = ROOT / args.checker
    receipt_path = ROOT / args.authorship_receipt
    corpus_path = ROOT / args.corpus
    if checker_path.is_symlink() or receipt_path.is_symlink() or corpus_path.is_symlink():
        fail("symlink authority input")
    if sha256(checker_path.read_bytes()) != args.expected_checker_sha256 or sha256(receipt_path.read_bytes()) != args.expected_receipt_sha256:
        fail("canonical checker/receipt bytes do not match caller pins")
    spec = importlib.util.spec_from_file_location("p1345_checker_r3_loaded_by_external_caller", checker_path)
    if spec is None or spec.loader is None:
        fail("checker cannot load")
    checker = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = checker
    spec.loader.exec_module(checker)
    checker.validate_authorship_receipt(receipt_path, args.expected_receipt_sha256, args.expected_checker_sha256, args.expected_authoring_root_sha256)
    output = checker.run_focal(args.expected_checker_sha256, args.expected_receipt_sha256, args.expected_authoring_root_sha256)
    print(json.dumps(output, ensure_ascii=False, indent=2))
    return 0 if output["verdict"] == "FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED" else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        print(json.dumps({"schema": "p1345-oracle-caller-fatal-r3", "classification": "Violated", "reason_code": "AUTHORITY_ROOT", "detail": str(exc)}, sort_keys=True), file=sys.stderr)
        raise SystemExit(2)
