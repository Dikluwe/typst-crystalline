#!/usr/bin/env python3
"""P1398 black-box oracle for symbol repr string escaping and path parity."""

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


def multiline(*lines: str) -> str:
    return "\n".join(lines)


QUOTE = multiline(
    "symbol(",
    r'  ("double", "\""),',
    "  (\"single\", \"'\"),",
    '  ("l.double", "“"),',
    '  ("l.single", "‘"),',
    '  ("r.double", "”"),',
    '  ("r.single", "’"),',
    '  ("chevron.l.double", "«"),',
    '  ("chevron.l.single", "‹"),',
    '  ("chevron.r.double", "»"),',
    '  ("chevron.r.single", "›"),',
    '  ("high.double", "‟"),',
    '  ("high.single", "‛"),',
    '  ("low.double", "„"),',
    '  ("low.single", "‚"),',
    ")",
)

QUOTE_DOUBLE = multiline(
    "symbol(",
    r'  "\"",',
    '  ("l", "“"),',
    '  ("r", "”"),',
    '  ("chevron.l", "«"),',
    '  ("chevron.r", "»"),',
    '  ("high", "‟"),',
    '  ("low", "„"),',
    ")",
)

QUOTE_SINGLE = multiline(
    "symbol(",
    "  \"'\",",
    '  ("l", "‘"),',
    '  ("r", "’"),',
    '  ("chevron.l", "‹"),',
    '  ("chevron.r", "›"),',
    '  ("high", "‛"),',
    '  ("low", "‚"),',
    ")",
)

EMOJI_HEART = multiline(
    "symbol(",
    r'  "❤\u{fe0f}",',
    '  ("arrow", "💘"),',
    '  ("beat", "💓"),',
    '  ("black", "🖤"),',
    '  ("blue", "💙"),',
    '  ("box", "💟"),',
    '  ("broken", "💔"),',
    '  ("brown", "🤎"),',
    '  ("double", "💕"),',
    r'  ("excl", "❣\u{fe0f}"),',
    '  ("gray", "🩶"),',
    '  ("green", "💚"),',
    '  ("grow", "💗"),',
    '  ("lightblue", "🩵"),',
    '  ("orange", "🧡"),',
    '  ("pink", "🩷"),',
    '  ("purple", "💜"),',
    '  ("real", "🫀"),',
    '  ("revolve", "💞"),',
    '  ("ribbon", "💝"),',
    '  ("spark", "💖"),',
    '  ("white", "🤍"),',
    '  ("yellow", "💛"),',
    ")",
)

ARROW_R = multiline(
    "symbol(",
    '  "→",',
    '  ("long.bar", "⟼"),',
    '  ("bar", "↦"),',
    '  ("curve", "⤷"),',
    '  ("turn", "⮎"),',
    '  ("dashed", "⇢"),',
    '  ("dotted", "⤑"),',
    '  ("double", "⇒"),',
    '  ("double.bar", "⤇"),',
    '  ("double.long", "⟹"),',
    '  ("double.long.bar", "⟾"),',
    '  ("double.not", "⇏"),',
    '  ("double.struck", "⤃"),',
    r'  ("filled", "➡\u{fe0e}"),',
    r'  ("hook", "↪\u{fe0e}"),',
    '  ("long", "⟶"),',
    '  ("long.squiggly", "⟿"),',
    '  ("loop", "↬"),',
    '  ("not", "↛"),',
    '  ("quad", "⭆"),',
    '  ("squiggly", "⇝"),',
    '  ("stop", "⇥"),',
    '  ("stroked", "⇨"),',
    '  ("struck", "⇸"),',
    '  ("dstruck", "⇻"),',
    '  ("tail", "↣"),',
    '  ("tail.struck", "⤔"),',
    '  ("tail.dstruck", "⤕"),',
    '  ("tilde", "⥲"),',
    '  ("triple", "⇛"),',
    '  ("twohead", "↠"),',
    '  ("twohead.bar", "⤅"),',
    '  ("twohead.struck", "⤀"),',
    '  ("twohead.dstruck", "⤁"),',
    '  ("twohead.tail", "⤖"),',
    '  ("twohead.tail.struck", "⤗"),',
    '  ("twohead.tail.dstruck", "⤘"),',
    '  ("open", "⇾"),',
    '  ("wave", "↝"),',
    r'  ("l", "↔\u{fe0e}"),',
    '  ("l.double", "⇔"),',
    '  ("l.double.long", "⟺"),',
    '  ("l.double.not", "⇎"),',
    '  ("l.double.struck", "⤄"),',
    '  ("l.filled", "⬌"),',
    '  ("l.long", "⟷"),',
    '  ("l.not", "↮"),',
    '  ("l.stroked", "⬄"),',
    '  ("l.struck", "⇹"),',
    '  ("l.dstruck", "⇼"),',
    '  ("l.open", "⇿"),',
    '  ("l.wave", "↭"),',
    ")",
)


CASES = (
    Case("sym-quote", "repr(sym.quote)", QUOTE, "quote-path"),
    Case("sym-quote-double", "repr(sym.quote.double)", QUOTE_DOUBLE, "quote-path"),
    Case("sym-quote-single", "repr(sym.quote.single)", QUOTE_SINGLE, "quote-path"),
    Case("math-quote", "repr(math.quote)", QUOTE, "quote-alias"),
    Case("math-quote-double", "repr(math.quote.double)", QUOTE_DOUBLE, "quote-alias"),
    Case("math-quote-single", "repr(math.quote.single)", QUOTE_SINGLE, "quote-alias"),
    Case("runtime-double-quote", r'repr(symbol("\""))', r'symbol("\"")', "runtime-string"),
    Case("runtime-apostrophe", "repr(symbol(\"'\"))", "symbol(\"'\")", "runtime-string"),
    Case("runtime-nul", r'repr(symbol("\u{0}"))', r'symbol("\u{0}")', "runtime-string"),
    Case("runtime-backslash-control", r'repr(symbol("\\"))', r'symbol("\\")', "escape-control"),
    Case("runtime-newline-control", r'repr(symbol("\n"))', r'symbol("\n")', "escape-control"),
    Case("runtime-tab-control", r'repr(symbol("\t"))', r'symbol("\t")', "escape-control"),
    Case("runtime-carriage-return-control", r'repr(symbol("\r"))', r'symbol("\r")', "escape-control"),
    Case(
        "runtime-zwj-control",
        'repr(symbol("👩‍💻"))',
        r'symbol("👩\u{200d}💻")',
        "escape-control",
    ),
    Case("emoji-heart-vs-control", "repr(emoji.heart)", EMOJI_HEART, "variation-selector-control"),
    Case("arrow-r-control", "repr(sym.arrow.r)", ARROW_R, "large-symbol-control"),
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
                    **unknown_case(case, "TIMEOUT", stderr.decode("utf-8", errors="replace")),
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
    parser.add_argument("--cli-profile", choices=("crystalline", "vanila"), default="crystalline")
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
            signature(first) == signature(second) == signature(reverse_by_id[first["id"]])
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
        "Unknown" if counts["Unknown"] or not order_agreement
        else "Difference" if counts["Difference"]
        else "Preserved"
    )
    receipt = {
        "schema": "p1398-symbol-string-repr/v1",
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
