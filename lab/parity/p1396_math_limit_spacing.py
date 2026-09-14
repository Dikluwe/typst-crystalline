#!/usr/bin/env python3
"""P1396 black-box oracle for exact spacing in math limit operators."""

from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
import time
from dataclasses import dataclass
from pathlib import Path


PROCESS_TIMEOUT_SECONDS = 5


@dataclass(frozen=True)
class Case:
    case_id: str
    name: str
    expected: str
    kind: str

    @property
    def expression(self) -> str:
        return f"repr(math.{self.name})"


CASES = (
    Case(
        "liminf-thin-space",
        "liminf",
        "op(text: [lim\u2009inf], limits: true)",
        "changed",
    ),
    Case(
        "limsup-thin-space",
        "limsup",
        "op(text: [lim\u2009sup], limits: true)",
        "changed",
    ),
    Case("lim-adjacent-control", "lim", "op(text: [lim], limits: true)", "control"),
    Case("max-adjacent-control", "max", "op(text: [max], limits: true)", "control"),
)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def bytes_view(value: bytes) -> dict[str, object]:
    return {
        "hex": value.hex(),
        "decimal": list(value),
    }


def codepoints(value: str) -> list[str]:
    return [f"U+{ord(character):04X}" for character in value]


def operator_text(value: str) -> str | None:
    prefix = "op(text: ["
    suffix = "], limits: true)"
    if value.startswith(prefix) and value.endswith(suffix):
        return value[len(prefix) : -len(suffix)]
    return None


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
        "kind": case.kind,
        "name": case.name,
        "expression": case.expression,
        "expected": case.expected,
        "expected_bytes": bytes_view(expected_bytes),
        "expected_codepoints": codepoints(case.expected),
        "actual": None,
        "actual_bytes": None,
        "actual_codepoints": None,
        "expected_thin_space_count": case.expected.count("\u2009"),
        "actual_thin_space_count": None,
        "process_exit": None,
        "stderr": detail,
        "stderr_bytes": None,
        "elapsed_seconds": 0.0,
        "control_inner_name_contains_whitespace": None,
        "classification": "Unknown",
        "reason_code": reason_code,
    }


def run_case(binary: Path, profile: str, case: Case) -> dict[str, object]:
    argv = command(binary, profile, case.expression)
    started = time.monotonic()
    expected_bytes = case.expected.encode("utf-8")
    try:
        process = subprocess.run(
            argv,
            capture_output=True,
            timeout=PROCESS_TIMEOUT_SECONDS,
            check=False,
        )
    except subprocess.TimeoutExpired as error:
        stderr = error.stderr or b""
        return {
            **unknown_case(case, "TIMEOUT", stderr.decode("utf-8", errors="replace")),
            "argv": argv,
            "stderr_bytes": bytes_view(stderr),
            "elapsed_seconds": round(time.monotonic() - started, 6),
        }
    except OSError as error:
        return {
            **unknown_case(case, "TOOL_EXECUTION_FAILED", str(error)),
            "argv": argv,
            "elapsed_seconds": round(time.monotonic() - started, 6),
        }

    elapsed = round(time.monotonic() - started, 6)
    try:
        actual = process.stdout.decode("utf-8", errors="strict")
        stderr = process.stderr.decode("utf-8", errors="strict")
    except UnicodeDecodeError as error:
        return {
            **unknown_case(case, "INVALID_UTF8", str(error)),
            "argv": argv,
            "actual_bytes": bytes_view(process.stdout),
            "stderr_bytes": bytes_view(process.stderr),
            "process_exit": process.returncode,
            "elapsed_seconds": elapsed,
        }

    if process.returncode != 0:
        classification = "Unknown"
        reason_code = "TOOL_NONZERO_EXIT"
    elif not process.stdout:
        classification = "Unknown"
        reason_code = "MISSING_OUTPUT"
    elif process.stdout == expected_bytes:
        classification = "Preserved"
        reason_code = None
    else:
        classification = "Difference"
        reason_code = "EXACT_UNICODE_MISMATCH"

    control_has_whitespace = None
    if case.kind == "control":
        inner_text = operator_text(actual)
        if inner_text is not None:
            control_has_whitespace = any(character.isspace() for character in inner_text)

    return {
        "id": case.case_id,
        "kind": case.kind,
        "name": case.name,
        "expression": case.expression,
        "argv": argv,
        "expected": case.expected,
        "expected_bytes": bytes_view(expected_bytes),
        "expected_codepoints": codepoints(case.expected),
        "actual": actual,
        "actual_bytes": bytes_view(process.stdout),
        "actual_codepoints": codepoints(actual),
        "expected_thin_space_count": case.expected.count("\u2009"),
        "actual_thin_space_count": actual.count("\u2009"),
        "process_exit": process.returncode,
        "stderr": stderr,
        "stderr_bytes": bytes_view(process.stderr),
        "elapsed_seconds": elapsed,
        "control_inner_name_contains_whitespace": control_has_whitespace,
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
        reverse = [unknown_case(case, identity_reason, "binary identity not accepted") for case in reversed(CASES)]
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
        "schema": "p1396-math-limit-spacing/v1",
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
        "execution": "serial",
        "comparison": "exact UTF-8 bytes and Unicode code points; no whitespace normalization",
        "external_resource_measurement": {
            "max_rss_kib": None,
            "swap_kib": None,
            "reason": "external wrapper measurement is recorded by the execution receipt, not inferred here",
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
