#!/usr/bin/env python3
"""P1397 black-box oracle for canonical math class/style repr morphology."""

from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
import tempfile
import time
from dataclasses import dataclass
from pathlib import Path


PROCESS_TIMEOUT_SECONDS = 5
MAX_STREAM_BYTES = 65_536


@dataclass(frozen=True)
class Case:
    case_id: str
    expression: str
    expected: str
    family: str


CASES = (
    Case(
        "dif-canonical-sequence",
        "repr(math.dif)",
        "sequence(\n"
        "  h(amount: 0.17em, weak: true),\n"
        "  class(class: \"unary\", body: styled(child: [d], ..)),\n"
        ")",
        "differential",
    ),
    Case(
        "Dif-canonical-sequence",
        "repr(math.Dif)",
        "sequence(\n"
        "  h(amount: 0.17em, weak: true),\n"
        "  class(class: \"unary\", body: styled(child: [D], ..)),\n"
        ")",
        "differential",
    ),
    Case(
        "class-relation-named-fields",
        'repr(math.class("relation", [x]))',
        'class(class: "relation", body: [x])',
        "class-control",
    ),
    Case(
        "class-unary-named-fields",
        'repr(math.class("unary", [x]))',
        'class(class: "unary", body: [x])',
        "class-control",
    ),
    Case(
        "styled-bb-preserved",
        "repr(math.bb([x]))",
        "styled(child: [x], ..)",
        "style-control",
    ),
    Case(
        "styled-frak-preserved",
        "repr(math.frak([x]))",
        "styled(child: [x], ..)",
        "style-control",
    ),
    Case(
        "styled-serif-preserved",
        "repr(math.serif([x]))",
        "styled(child: [x], ..)",
        "style-control",
    ),
    Case(
        "styled-inline-preserved",
        "repr(math.inline([x]))",
        "styled(child: [x], ..)",
        "style-control",
    ),
    Case(
        "styled-mono-preserved",
        "repr(math.mono([x]))",
        "styled(child: [x], ..)",
        "style-control",
    ),
    Case(
        "styled-script-preserved",
        "repr(math.script([x]))",
        "styled(child: [x], ..)",
        "style-control",
    ),
    Case(
        "math-op-unchanged",
        "repr(math.lim)",
        "op(text: [lim], limits: true)",
        "unrelated-control",
    ),
)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def bytes_view(value: bytes) -> dict[str, object]:
    return {"hex": value.hex(), "decimal": list(value)}


def codepoints(value: str) -> list[str]:
    return [f"U+{ord(character):04X}" for character in value]


def command(binary: Path, profile: str, expression: str) -> list[str]:
    argv = [str(binary), "eval", "--format", "raw"]
    if profile == "crystalline":
        argv.extend(["--color", "never"])
    argv.append(expression)
    return argv


def unknown_case(case: Case, reason_code: str, detail: str) -> dict[str, object]:
    expected_bytes = case.expected.encode("utf-8")
    return {
        "id": case.case_id,
        "family": case.family,
        "expression": case.expression,
        "expected": case.expected,
        "expected_bytes": bytes_view(expected_bytes),
        "expected_codepoints": codepoints(case.expected),
        "actual": None,
        "actual_bytes": None,
        "actual_codepoints": None,
        "process_exit": None,
        "stderr": detail,
        "stderr_bytes": None,
        "elapsed_seconds": 0.0,
        "classification": "Unknown",
        "reason_code": reason_code,
    }


def read_bounded(handle: object) -> tuple[bytes, bool]:
    handle.seek(0)
    value = handle.read(MAX_STREAM_BYTES + 1)
    return value[:MAX_STREAM_BYTES], len(value) > MAX_STREAM_BYTES


def run_case(binary: Path, profile: str, case: Case) -> dict[str, object]:
    argv = command(binary, profile, case.expression)
    started = time.monotonic()
    expected_bytes = case.expected.encode("utf-8")
    try:
        with tempfile.TemporaryFile() as stdout_file, tempfile.TemporaryFile() as stderr_file:
            process = subprocess.Popen(argv, stdout=stdout_file, stderr=stderr_file)
            try:
                process.wait(timeout=PROCESS_TIMEOUT_SECONDS)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()
                stdout, stdout_truncated = read_bounded(stdout_file)
                stderr, stderr_truncated = read_bounded(stderr_file)
                return {
                    **unknown_case(
                        case,
                        "TIMEOUT",
                        stderr.decode("utf-8", errors="replace"),
                    ),
                    "argv": argv,
                    "actual_bytes": bytes_view(stdout),
                    "stderr_bytes": bytes_view(stderr),
                    "stdout_truncated": stdout_truncated,
                    "stderr_truncated": stderr_truncated,
                    "elapsed_seconds": round(time.monotonic() - started, 6),
                }
            stdout, stdout_truncated = read_bounded(stdout_file)
            stderr, stderr_truncated = read_bounded(stderr_file)
    except OSError as error:
        return {
            **unknown_case(case, "TOOL_EXECUTION_FAILED", str(error)),
            "argv": argv,
            "elapsed_seconds": round(time.monotonic() - started, 6),
        }

    elapsed = round(time.monotonic() - started, 6)
    try:
        actual = stdout.decode("utf-8", errors="strict")
        stderr_text = stderr.decode("utf-8", errors="strict")
    except UnicodeDecodeError as error:
        return {
            **unknown_case(case, "INVALID_UTF8", str(error)),
            "argv": argv,
            "actual_bytes": bytes_view(stdout),
            "stderr_bytes": bytes_view(stderr),
            "process_exit": process.returncode,
            "stdout_truncated": stdout_truncated,
            "stderr_truncated": stderr_truncated,
            "elapsed_seconds": elapsed,
        }

    if stdout_truncated or stderr_truncated:
        classification = "Unknown"
        reason_code = "OUTPUT_LIMIT_EXCEEDED"
    elif process.returncode != 0:
        classification = "Unknown"
        reason_code = "TOOL_NONZERO_EXIT"
    elif not stdout:
        classification = "Unknown"
        reason_code = "MISSING_OUTPUT"
    elif stdout == expected_bytes:
        classification = "Preserved"
        reason_code = None
    else:
        classification = "Difference"
        reason_code = "EXACT_REPR_MISMATCH"

    return {
        "id": case.case_id,
        "family": case.family,
        "expression": case.expression,
        "argv": argv,
        "expected": case.expected,
        "expected_bytes": bytes_view(expected_bytes),
        "expected_codepoints": codepoints(case.expected),
        "actual": actual,
        "actual_bytes": bytes_view(stdout),
        "actual_codepoints": codepoints(actual),
        "process_exit": process.returncode,
        "stderr": stderr_text,
        "stderr_bytes": bytes_view(stderr),
        "stdout_truncated": stdout_truncated,
        "stderr_truncated": stderr_truncated,
        "elapsed_seconds": elapsed,
        "classification": classification,
        "reason_code": reason_code,
    }


def signature(row: dict[str, object]) -> tuple[object, ...]:
    return (
        row["id"],
        row["classification"],
        row["reason_code"],
        row["process_exit"],
        json.dumps(row["actual_bytes"], sort_keys=True),
        json.dumps(row["stderr_bytes"], sort_keys=True),
    )


def write_receipt(path: Path, receipt: dict[str, object]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        json.dumps(receipt, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--expected-binary-sha256", required=True)
    parser.add_argument(
        "--cli-profile",
        choices=("crystalline", "vanila"),
        default="crystalline",
    )
    args = parser.parse_args()

    binary = args.binary.resolve()
    expected_identity = args.expected_binary_sha256.lower()
    identity_reason = None
    actual_identity = None
    binary_bytes = None
    try:
        if not binary.is_file():
            identity_reason = "BINARY_NOT_FOUND"
        else:
            actual_identity = sha256(binary)
            binary_bytes = binary.stat().st_size
            if actual_identity != expected_identity:
                identity_reason = "BINARY_IDENTITY_MISMATCH"
    except OSError as error:
        identity_reason = f"BINARY_IDENTITY_ERROR: {error}"

    if identity_reason is not None:
        direct = [unknown_case(case, identity_reason, "binary identity not accepted") for case in CASES]
        repeated = [dict(row) for row in direct]
        reverse = [
            unknown_case(case, identity_reason, "binary identity not accepted")
            for case in reversed(CASES)
        ]
        order_agreement = True
    else:
        direct = [run_case(binary, args.cli_profile, case) for case in CASES]
        repeated = [run_case(binary, args.cli_profile, case) for case in CASES]
        reverse = [run_case(binary, args.cli_profile, case) for case in reversed(CASES)]
        reverse_by_id = {row["id"]: row for row in reverse}
        order_agreement = all(
            signature(first)
            == signature(second)
            == signature(reverse_by_id[first["id"]])
            for first, second in zip(direct, repeated)
        )
        if not order_agreement:
            for row in direct:
                row["classification"] = "Unknown"
                row["reason_code"] = "ORDER_DISAGREEMENT"

    counts = {
        state: sum(row["classification"] == state for row in direct)
        for state in ("Preserved", "Difference", "Unknown")
    }
    overall = (
        "Unknown"
        if counts["Unknown"] or not order_agreement
        else "Difference"
        if counts["Difference"]
        else "Preserved"
    )
    receipt = {
        "schema": "p1397-math-differential-repr/v1",
        "binary": {
            "path": str(binary),
            "expected_sha256": expected_identity,
            "actual_sha256": actual_identity,
            "bytes": binary_bytes,
            "identity_accepted": identity_reason is None,
            "identity_reason": identity_reason,
        },
        "cli_profile": args.cli_profile,
        "process_timeout_seconds": PROCESS_TIMEOUT_SECONDS,
        "max_stream_bytes": MAX_STREAM_BYTES,
        "execution": "serial",
        "comparison": "exact UTF-8 bytes and Unicode code points; no normalization",
        "external_resource_measurement": {
            "max_rss_kib": None,
            "swap_kib": None,
            "reason": "external wrapper measurement is recorded separately",
        },
        "cases": direct,
        "repeat_cases": repeated,
        "reverse_cases": reverse,
        "order_agreement": order_agreement,
        "counts": counts,
        "overall": overall,
    }
    write_receipt(args.output, receipt)
    return 0 if overall == "Preserved" else 1 if overall == "Difference" else 2


if __name__ == "__main__":
    sys.exit(main())
