#!/usr/bin/env python3
"""P1395 black-box oracle for lexical callees in math mode."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
import tempfile
import time
from dataclasses import dataclass
from pathlib import Path


PROCESS_TIMEOUT_SECONDS = 10
ANSI = re.compile(r"\x1b\[[0-9;]*m")


@dataclass(frozen=True)
class Case:
    case_id: str
    source: str
    mode: str
    expected: object


CASES = (
    Case("local-abs-closure", "#let abs(x) = [LOCAL-ABS]\n$abs(1)$\n", "text", "LOCAL-ABS"),
    Case("local-sqrt-closure", "#let sqrt(x) = [LOCAL-SQRT]\n$sqrt(1)$\n", "text", "LOCAL-SQRT"),
    Case("local-abs-non-func-positional", "#let abs = 7\n$abs(1, x)$\n", "text", "7(1, 𝑥)"),
    Case(
        "local-abs-non-func-named",
        "#let abs = 7\n$abs(foo: 1)$\n",
        "diagnostic",
        (
            "error: named-argument syntax can only be used with functions",
            "hint: to render the colon as text, escape it: \x60foo\\: 1\x60",
        ),
    ),
    Case(
        "local-abs-non-func-spread",
        "#let abs = 7\n#let xs = (1,)\n$abs(..xs)$\n",
        "diagnostic",
        (
            "error: spread-argument syntax can only be used with functions",
            "hint: to render the dots as text, add a space: \x60.. xs\x60",
        ),
    ),
    Case("native-abs-control", "$abs(1)$\n", "text", "|1|"),
    Case("native-sqrt-control", "$sqrt(1)$\n", "text", "√ 1"),
)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def fixture_sha256(source: str) -> str:
    return hashlib.sha256(source.encode("utf-8")).hexdigest()


def public_diagnostics(stderr: str) -> list[str]:
    lines = []
    for raw in ANSI.sub("", stderr).splitlines():
        line = raw.strip()
        if line.startswith("= hint:"):
            line = line[2:]
        if line.startswith(("error:", "warning:", "hint:")):
            lines.append(line)
    return lines


def normalized_text(text: str) -> str:
    return " ".join(text.replace("\f", " ").split())


def run_case(binary: Path, root: Path, case: Case, ordinal: int) -> dict[str, object]:
    source = root / f"{ordinal:02d}-{case.case_id}.typ"
    artifact = root / f"{ordinal:02d}-{case.case_id}.pdf"
    source.write_text(case.source, encoding="utf-8")
    argv = [str(binary), "--color", "never", "compile", str(source), str(artifact)]
    started = time.monotonic()
    try:
        process = subprocess.run(
            argv,
            capture_output=True,
            text=True,
            timeout=PROCESS_TIMEOUT_SECONDS,
            check=False,
        )
    except subprocess.TimeoutExpired as error:
        return {
            "id": case.case_id,
            "fixture_sha256": fixture_sha256(case.source),
            "expected": case.expected,
            "actual": None,
            "process_exit": None,
            "stderr": (error.stderr or "")[-4000:],
            "elapsed_seconds": round(time.monotonic() - started, 6),
            "classification": "Unknown",
            "reason_code": "TIMEOUT",
        }

    elapsed = round(time.monotonic() - started, 6)
    stderr = process.stderr[-4000:]
    actual: object
    reason_code: str | None
    if case.mode == "text":
        if process.returncode != 0 or not artifact.is_file():
            actual = None
            classification = "Difference"
            reason_code = "LANGUAGE_MORPHOLOGY"
        else:
            try:
                extracted = subprocess.run(
                    ["pdftotext", "-layout", str(artifact), "-"],
                    capture_output=True,
                    text=True,
                    timeout=PROCESS_TIMEOUT_SECONDS,
                    check=True,
                )
                actual = normalized_text(extracted.stdout)
                classification = "Preserved" if actual == case.expected else "Difference"
                reason_code = None if classification == "Preserved" else "LANGUAGE_MORPHOLOGY"
            except (subprocess.SubprocessError, OSError) as error:
                actual = None
                classification = "Unknown"
                reason_code = "OUTPUT_MISSING"
                stderr = f"{stderr}\npdftotext: {error}"[-4000:]
    else:
        actual = public_diagnostics(process.stderr)
        expected = list(case.expected)
        classification = (
            "Preserved" if process.returncode != 0 and actual == expected else "Difference"
        )
        reason_code = None if classification == "Preserved" else "PUBLIC_DIAGNOSTIC"

    return {
        "id": case.case_id,
        "fixture_sha256": fixture_sha256(case.source),
        "expected": case.expected,
        "actual": actual,
        "process_exit": process.returncode,
        "stderr": stderr,
        "elapsed_seconds": elapsed,
        "classification": classification,
        "reason_code": reason_code,
    }


def observable_signature(row: dict[str, object]) -> tuple[object, ...]:
    return (
        row["id"],
        row["classification"],
        row["reason_code"],
        row["process_exit"],
        json.dumps(row["actual"], ensure_ascii=False, sort_keys=True),
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    binary = args.binary.resolve()
    if not binary.is_file():
        parser.error(f"binary does not exist: {binary}")

    with tempfile.TemporaryDirectory(prefix="p1395-") as directory:
        root = Path(directory)
        direct = [run_case(binary, root, case, index) for index, case in enumerate(CASES)]
        repeated = [
            run_case(binary, root, case, index + len(CASES))
            for index, case in enumerate(CASES)
        ]
        reverse = [
            run_case(binary, root, case, index + 2 * len(CASES))
            for index, case in enumerate(reversed(CASES))
        ]

    reverse_by_id = {row["id"]: row for row in reverse}
    order_agreement = all(
        observable_signature(first)
        == observable_signature(second)
        == observable_signature(reverse_by_id[first["id"]])
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
    single_evaluation = {
        "classification": "NotObserved",
        "reason_code": "INSUFFICIENT_PUBLIC_EVIDENCE",
        "required_for_exit": False,
        "explanation": (
            "The bounded CLI/PDF surface has no safe deterministic counter for "
            "evaluations of a discarded argument. R3 requires this only when observable by CLI."
        ),
    }
    overall = (
        "Unknown"
        if counts["Unknown"] or not order_agreement
        else "Difference"
        if counts["Difference"]
        else "Preserved"
    )
    receipt = {
        "schema": "p1395-math-callee/v1",
        "binary": {
            "path": str(binary),
            "sha256": sha256(binary),
            "bytes": binary.stat().st_size,
        },
        "process_timeout_seconds": PROCESS_TIMEOUT_SECONDS,
        "execution": "serial",
        "extractor": "pdftotext -layout",
        "cases": direct,
        "repeat_cases": repeated,
        "reverse_cases": reverse,
        "order_agreement": order_agreement,
        "single_evaluation": single_evaluation,
        "counts": counts,
        "overall": overall,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(
        json.dumps(receipt, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )

    return 0 if overall == "Preserved" else 1 if overall == "Difference" else 2


if __name__ == "__main__":
    sys.exit(main())
