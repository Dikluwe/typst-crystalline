#!/usr/bin/env python3
"""Executa exatamente o adaptador S11 com contenção de recursos."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import resource
import subprocess
import tempfile
import time
from pathlib import Path
from typing import Any


EXACT_TEST = "p1361_duration_exact_vector_is_deterministic_and_boundary_safe"
TEST_BINARY_SHA256 = "06c91f0646fff5c4263935851e8d2899ea2cc3b8eb86af5beab6964daa75e1be"
TIMEOUT_SECONDS = 120
ADDRESS_SPACE_BYTES = 3 * 1024**3
FILE_SIZE_BYTES = 8 * 1024**2
CHANNEL_READ_BYTES = 2 * 1024**2
CPU_SECONDS = 120
MAX_PROCESSES = 1


def command(test_binary: Path) -> list[str]:
    return [
        str(test_binary),
        EXACT_TEST,
        "--exact",
    ]


def classify_receipt(receipt: dict[str, Any]) -> str:
    if receipt.get("reason_code") is not None:
        return "Unknown"
    passed = receipt.get("passed")
    failed = receipt.get("failed")
    exit_code = receipt.get("exit_code")
    if not isinstance(passed, int) or not isinstance(failed, int):
        return "Unknown"
    if exit_code != 0 or failed != 0:
        return "Violated"
    if passed != 1:
        return "Unknown"
    return "Preserved"


def _apply_limits() -> None:
    resource.setrlimit(resource.RLIMIT_AS, (ADDRESS_SPACE_BYTES, ADDRESS_SPACE_BYTES))
    resource.setrlimit(resource.RLIMIT_FSIZE, (FILE_SIZE_BYTES, FILE_SIZE_BYTES))
    resource.setrlimit(resource.RLIMIT_CPU, (CPU_SECONDS, CPU_SECONDS))


def _count_tests(stdout: bytes) -> tuple[int | None, int | None]:
    text = stdout.decode("utf-8", errors="replace")
    matches = re.findall(r"test result: \w+\. (\d+) passed; (\d+) failed;", text)
    if len(matches) != 1:
        return None, None
    passed, failed = matches[0]
    return int(passed), int(failed)


def _sha256_path(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def execute(test_binary: Path) -> dict[str, Any]:
    test_binary = test_binary.resolve()
    identity = _sha256_path(test_binary)
    if identity != TEST_BINARY_SHA256:
        return {
            "classification": "Unknown",
            "s11_status": "Unknown",
            "reason_code": "TEST_BINARY_IDENTITY",
            "test_binary_sha256": identity,
        }
    argv = command(test_binary)
    started = time.monotonic()
    reason_code = None
    with tempfile.TemporaryFile() as stdout, tempfile.TemporaryFile() as stderr:
        process = subprocess.Popen(
            argv,
            stdin=subprocess.DEVNULL,
            stdout=stdout,
            stderr=stderr,
            preexec_fn=_apply_limits,
        )
        try:
            exit_code = process.wait(timeout=TIMEOUT_SECONDS)
        except subprocess.TimeoutExpired:
            process.kill()
            exit_code = process.wait()
            reason_code = "TIMEOUT"

        elapsed_ms = round((time.monotonic() - started) * 1000, 3)
        stdout_size = stdout.tell()
        stderr_size = stderr.tell()
        stdout.seek(0)
        stderr.seek(0)
        stdout_bytes = stdout.read(CHANNEL_READ_BYTES + 1)
        stderr_bytes = stderr.read(CHANNEL_READ_BYTES + 1)

    if stdout_size > CHANNEL_READ_BYTES or stderr_size > CHANNEL_READ_BYTES:
        reason_code = "CHANNEL_LIMIT"
    elif exit_code < 0 and reason_code is None:
        reason_code = f"SIGNAL_{-exit_code}"
    passed, failed = _count_tests(stdout_bytes[:CHANNEL_READ_BYTES])
    receipt = {
        "command": argv,
        "test_binary_sha256": identity,
        "exit_code": exit_code,
        "reason_code": reason_code,
        "passed": passed,
        "failed": failed,
        "elapsed_ms": elapsed_ms,
        "stdout_size": stdout_size,
        "stderr_size": stderr_size,
        "stdout_sha256": hashlib.sha256(stdout_bytes[:CHANNEL_READ_BYTES]).hexdigest(),
        "stderr_sha256": hashlib.sha256(stderr_bytes[:CHANNEL_READ_BYTES]).hexdigest(),
        "stdout_tail": stdout_bytes[-8192:].decode("utf-8", errors="replace"),
        "stderr_preview": stderr_bytes[:8192].decode("utf-8", errors="replace"),
        "stderr_tail": stderr_bytes[-8192:].decode("utf-8", errors="replace"),
        "limits": {
            "processes": MAX_PROCESSES,
            "timeout_seconds": TIMEOUT_SECONDS,
            "address_space_bytes": ADDRESS_SPACE_BYTES,
            "file_size_bytes": FILE_SIZE_BYTES,
            "channel_read_bytes": CHANNEL_READ_BYTES,
            "cpu_seconds": CPU_SECONDS,
        },
    }
    receipt["classification"] = classify_receipt(receipt)
    receipt["s11_status"] = "MATCH" if receipt["classification"] == "Preserved" else receipt["classification"]
    return receipt


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--test-binary", required=True)
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    result = execute(Path(args.test_binary))
    Path(args.output).write_text(
        json.dumps(result, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()
