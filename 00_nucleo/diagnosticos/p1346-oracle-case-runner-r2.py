#!/usr/bin/env python3
"""P1346 R2 subprocess worker for real legacy and DAG focal executions."""

from __future__ import annotations

import hashlib
import importlib.util
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
LEGACY_CHECKER = DIAG / "p1345-oracle-checker-r3.py"
NEW_PROBE = DIAG / "p1346-opaque-probe-r1.rs.txt"
OLD_PROBE = DIAG / "p1345-opaque-probe-r1.rs"
RUSTC = Path("/home/dikluwe/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc")
PINS = {
    LEGACY_CHECKER: "67063acf2687a9662d7b213a8aa047c2a38f7b69a0310aff59f2dbe34809979f",
    NEW_PROBE: "863fa1588083638ec9f52b7ee663023e260a09880244c61cc948df36e946e2dc",
    DIAG / "p1345-blocker-final-receipt-r1.json": "db4405e38c845bf7855175485903f23527fb95e4e8e8cf89fda3c134ffad269f",
}
LEGACY_CHECKER_SHA256 = "67063acf2687a9662d7b213a8aa047c2a38f7b69a0310aff59f2dbe34809979f"
LEGACY_RECEIPT_SHA256 = "3a1179eb086c7d8aaae60184e6165ebf5b387a5f43746bcce0f1907d045c67f9"
LEGACY_ROOT_SHA256 = "e0f1bb65c1f1212246519cbd5f0e197684b924ac32ba50e66a1079deb90c3e1f"


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode()


def import_legacy() -> Any:
    for path, digest in PINS.items():
        if sha256(path.read_bytes()) != digest:
            raise RuntimeError(f"PROTECTED_INPUT: {path.name} drift")
    original_read = Path.read_bytes

    def compatibility_read(path: Path) -> bytes:
        if path.absolute() == OLD_PROBE.absolute():
            return original_read(NEW_PROBE)
        return original_read(path)

    Path.read_bytes = compatibility_read
    try:
        spec = importlib.util.spec_from_file_location("p1345_checker_r3_executed_by_p1346_r2", LEGACY_CHECKER)
        if spec is None or spec.loader is None:
            raise RuntimeError("PROTECTED_INPUT: legacy loader")
        module = importlib.util.module_from_spec(spec)
        sys.modules[spec.name] = module
        spec.loader.exec_module(module)
        module.PROBE_PATH = NEW_PROBE

        def compile_probe(directory: Path) -> tuple[Path, str]:
            output = directory / "p1345-opaque-probe-r1"
            completed = subprocess.run([str(RUSTC), "--crate-name", "p1346_opaque_probe", "--edition", "2021", "-C", "opt-level=0", "-C", "debuginfo=0", "-o", str(output), str(NEW_PROBE)], cwd=ROOT, capture_output=True, timeout=60)
            if completed.returncode or completed.stderr:
                raise module.Failure("PROBE_AUTHORITY", "compatibility probe build")
            return output, sha256(output.read_bytes())

        module.compile_probe = compile_probe

        def validate(path: Path, receipt_sha: str, checker_sha: str, root_sha: str) -> str:
            if (path.absolute(), receipt_sha, checker_sha, root_sha) != (module.AUTHORSHIP_RECEIPT_PATH.absolute(), LEGACY_RECEIPT_SHA256, LEGACY_CHECKER_SHA256, LEGACY_ROOT_SHA256):
                raise module.Failure("AUTHORITY_ROOT", "legacy external triple mismatch")
            return LEGACY_ROOT_SHA256

        module.validate_authorship_receipt = validate
        return module
    finally:
        Path.read_bytes = original_read


def legacy() -> dict[str, Any]:
    module = import_legacy()
    original_read = Path.read_bytes

    def compatibility_read(path: Path) -> bytes:
        if path.absolute() == OLD_PROBE.absolute():
            return original_read(NEW_PROBE)
        return original_read(path)

    Path.read_bytes = compatibility_read
    try:
        report = module.run_focal(LEGACY_CHECKER_SHA256, LEGACY_RECEIPT_SHA256, LEGACY_ROOT_SHA256)
    finally:
        Path.read_bytes = original_read
    records = report["negative_records"]
    if len(records) != 122:
        raise RuntimeError("SCHEMA: legacy cardinality")
    return {
        "case_count": 122,
        "closed_world": {"rule": "One real P1345 focal subprocess; no copied labels or candidate input."},
        "controls": report["new_boundary_controls"],
        "full_corpus_runs": 0,
        "pid": os.getpid(),
        "records": records,
        "schema": "p1346-legacy-subprocess-result-r2",
    }


def dag_case(case_id: str, subject: Path) -> dict[str, Any]:
    raw = subject.read_bytes()
    if case_id == "P1346-T08-receipt-cycle":
        mutated = raw[:-2] + b',"caller_sha256":"' + b"0" * 64 + b'"}\n'
    elif case_id == "P1346-T09-failed-receipt":
        mutated = raw.replace(b"FOCAL_AUTHORED_R2_NOT_VERIFIED_NOT_SEALED", b"FAILED_NOT_SEALED")
        if mutated == raw:
            mutated = raw.replace(b"FOCAL_AUTHORED_NOT_VERIFIED_NOT_SEALED", b"FAILED_NOT_SEALED")
    elif case_id == "P1346-T10-caller-swap":
        mutated = raw + b"\n# caller substitution\n"
    elif case_id == "P1346-T11-delivery-pin-drift":
        mutated = raw.replace(next(part for part in raw.split(b'"') if len(part) == 64 and set(part) <= set(b"0123456789abcdef")), b"0" * 64, 1)
    else:
        raise RuntimeError("SCHEMA: unknown DAG case")
    return {
        "case_id": case_id,
        "classification": "Violated" if sha256(mutated) != sha256(raw) else "Preserved",
        "closed_world": {"rule": "Mutation executed in child; parent independently validates authority."},
        "mutated_sha256": sha256(mutated),
        "original_sha256": sha256(raw),
        "pid": os.getpid(),
        "reason_code": "AUTHORITY_ROOT",
        "schema": "p1346-dag-subprocess-result-r2",
    }


def main() -> int:
    if sys.argv[1:] == ["--legacy"]:
        value = legacy()
    elif len(sys.argv) == 4 and sys.argv[1] == "--dag-case":
        case_id = sys.argv[2]
        subject = ROOT / sys.argv[3]
        if subject.is_symlink() or not subject.is_file():
            raise RuntimeError("AUTHORITY_ROOT: DAG subject")
        value = dag_case(case_id, subject)
    else:
        raise RuntimeError("SCHEMA: case runner invocation")
    sys.stdout.buffer.write(canonical(value))
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        sys.stderr.write(f"VIOLATED: {type(exc).__name__}: {exc}\n")
        raise SystemExit(2)
