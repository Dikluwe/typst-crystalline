#!/usr/bin/env python3
"""P1461 focal public-language probe runner (stdlib only, fail closed)."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys
import time
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
PROTECTED_INPUTS = {
    "manifest": (
        ROOT / "00_nucleo/diagnosticos/p1461-manifest.json",
        "13c554edcff8afeca6e51381f73015906b68de189d79de989e30e769bbf9f825",
    ),
    "contract": (
        ROOT / "00_nucleo/diagnosticos/p1461-contract.json",
        "81da1964df13221a1b9a70666cfb1e035b40b1215344d5ec8934febc4073aaa2",
    ),
}
PROFILE_ORDER = ["default", "a11y-extras", "html", "html+a11y"]
PROFILE_ARGS = {
    "crystalline": {
        "default": [],
        "a11y-extras": ["--features", "a11y-extras"],
        "html": ["--features", "html"],
        "html+a11y": ["--features", "html,a11y-extras"],
    },
    "vanilla": {
        "default": [],
        "a11y-extras": ["--features", "a11y-extras"],
        "html": ["--target", "html", "--features", "html"],
        "html+a11y": ["--target", "html", "--features", "html,a11y-extras"],
    },
}
DIAGNOSTIC_PATTERNS = [
    ("missing-module-field", re.compile(r"module `([^`]+)` does not contain `([^`]+)`")),
    ("unknown-variable", re.compile(r"unknown variable `([^`]+)`")),
    (
        "field-access-on-none",
        re.compile(
            r"(?:cannot access fields? on (?:a value of type |type )?none"
            r"|cannot access fields? of (?:type )?none)"
        ),
    ),
]
LOCATION_PATTERN = re.compile(r"<input-expression>:(\d+):(\d+)")


def canonical(value: Any) -> bytes:
    return json.dumps(
        value, ensure_ascii=False, sort_keys=True, separators=(",", ":")
    ).encode("utf-8")


def digest_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def digest_file(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def load_json(path: Path) -> Any:
    with path.open(encoding="utf-8") as stream:
        return json.load(stream)


def emit(payload: Any, output: str | None) -> None:
    data = json.dumps(payload, ensure_ascii=False, sort_keys=True, indent=2) + "\n"
    if output:
        Path(output).write_text(data, encoding="utf-8")
    else:
        sys.stdout.write(data)


def unknown_receipt(reason: str, detail: str, args: argparse.Namespace) -> dict[str, Any]:
    return {
        "artifact": "p1461-focal-run",
        "schema_version": 1,
        "scope": "P1461 focal probes only; C07/C09 inventory is an external gate",
        "verdict": "Unknown",
        "reason_code": reason,
        "detail": detail,
        "requested": {
            "binary": getattr(args, "binary", None),
            "expected_sha256": getattr(args, "expected_sha", None),
            "oracles": getattr(args, "oracles", None),
            "oracles_sha256": getattr(args, "oracles_sha", None),
            "product": getattr(args, "product", None),
            "suite": getattr(args, "suite", None),
            "order": getattr(args, "order", None),
        },
    }


def verify_file(path_text: str, expected: str, label: str) -> dict[str, str]:
    path = Path(path_text).resolve()
    if not path.is_file():
        raise RuntimeError(f"MISSING_{label.upper()}: {path}")
    actual = digest_file(path)
    if actual != expected:
        raise RuntimeError(
            f"{label.upper()}_SHA_MISMATCH: expected {expected}, observed {actual}"
        )
    return {"path": str(path), "sha256": actual}


def argv_for(binary: str, product: str, expression: str, profile: str) -> list[str]:
    profile_args = PROFILE_ARGS[product][profile]
    if product == "vanilla":
        return [
            binary,
            "--color",
            "never",
            "eval",
            expression,
            "--format",
            "json",
            "--diagnostic-format",
            "short",
            *profile_args,
        ]
    return [
        binary,
        "eval",
        expression,
        "--format",
        "json",
        "--color",
        "never",
        *profile_args,
    ]


def execute(argv: list[str], timeout: float) -> dict[str, Any]:
    started_ns = time.monotonic_ns()
    try:
        proc = subprocess.run(
            argv,
            cwd=ROOT,
            capture_output=True,
            timeout=timeout,
            check=False,
        )
        return {
            "argv": argv,
            "exit_code": proc.returncode,
            "stdout": proc.stdout.decode("utf-8", errors="replace"),
            "stderr": proc.stderr.decode("utf-8", errors="replace"),
            "duration_ns": time.monotonic_ns() - started_ns,
            "reason_code": None,
        }
    except subprocess.TimeoutExpired as exc:
        return {
            "argv": argv,
            "exit_code": None,
            "stdout": (exc.stdout or b"").decode("utf-8", errors="replace"),
            "stderr": (exc.stderr or b"").decode("utf-8", errors="replace"),
            "duration_ns": time.monotonic_ns() - started_ns,
            "reason_code": "TIMEOUT",
        }
    except OSError as exc:
        return {
            "argv": argv,
            "exit_code": None,
            "stdout": "",
            "stderr": str(exc),
            "duration_ns": time.monotonic_ns() - started_ns,
            "reason_code": "ENVIRONMENT_INVALID",
        }


def parse_diagnostic(stderr: str, expected_token: str | None) -> dict[str, Any] | None:
    location = LOCATION_PATTERN.search(stderr)
    if not location:
        return None
    line = int(location.group(1))
    start = int(location.group(2))
    for class_name, pattern in DIAGNOSTIC_PATTERNS:
        match = pattern.search(stderr)
        if not match:
            continue
        if class_name == "missing-module-field":
            token = expected_token or match.group(2)
        elif class_name == "unknown-variable":
            token = match.group(1)
        else:
            token = expected_token or "none"
        parsed: dict[str, Any] = {
            "class": class_name,
            "message_core": match.group(0),
            "primary_span": {
                "line": line,
                "start_column": start,
                "end_column_exclusive": start + len(token),
                "token": token,
            },
        }
        if class_name == "missing-module-field":
            parsed["module"] = match.group(1)
            parsed["requested_field"] = match.group(2)
        elif class_name == "unknown-variable":
            parsed["variable"] = match.group(1)
        return parsed
    return None


def compare_case(case: dict[str, Any], receipt: dict[str, Any]) -> dict[str, Any]:
    base = {
        "id": case["id"],
        "category": case["category"],
        "profile": receipt["profile"],
        "mandatory": case.get("mandatory", True),
        "argv": receipt["argv"],
        "exit_code": receipt["exit_code"],
        "stdout": receipt["stdout"],
        "stderr": receipt["stderr"],
        "stdout_sha256": digest_bytes(receipt["stdout"].encode("utf-8")),
        "stderr_sha256": digest_bytes(receipt["stderr"].encode("utf-8")),
        "duration_ns": receipt["duration_ns"],
    }
    if receipt["reason_code"]:
        return {
            **base,
            "classification": "Unknown",
            "reason_code": receipt["reason_code"],
            "observed": None,
        }
    if receipt["exit_code"] is not None and receipt["exit_code"] < 0:
        return {
            **base,
            "classification": "Unknown",
            "reason_code": "PROCESS_SIGNAL",
            "observed": None,
        }

    expected = case["expected"]
    if expected["kind"] == "VALUE":
        if receipt["exit_code"] != expected["exit_code"]:
            observed = parse_diagnostic(receipt["stderr"], case.get("diagnostic_token"))
            if receipt["exit_code"] == 0:
                try:
                    observed = json.loads(receipt["stdout"])
                except json.JSONDecodeError:
                    observed = None
            if observed is None:
                return {
                    **base,
                    "classification": "Unknown",
                    "reason_code": "PARSER_UNSUPPORTED",
                    "observed": None,
                }
            return {
                **base,
                "classification": "Violated",
                "reason_code": "EXPECTED_VALUE",
                "observed": observed,
            }
        try:
            observed = json.loads(receipt["stdout"])
        except json.JSONDecodeError:
            return {
                **base,
                "classification": "Unknown",
                "reason_code": "JSON_PARSE_FAILURE",
                "observed": None,
            }
        classification = "Preserved" if observed == expected["value"] else "Violated"
        return {
            **base,
            "classification": classification,
            "reason_code": None if classification == "Preserved" else "VALUE_MISMATCH",
            "observed": observed,
        }

    if receipt["exit_code"] == 0:
        try:
            observed_value = json.loads(receipt["stdout"])
        except json.JSONDecodeError:
            return {
                **base,
                "classification": "Unknown",
                "reason_code": "JSON_PARSE_FAILURE",
                "observed": None,
            }
        return {
            **base,
            "classification": "Violated",
            "reason_code": "EXPECTED_DIAGNOSTIC_RECEIVED_VALUE",
            "observed": {"kind": "VALUE", "value": observed_value},
        }

    parsed = parse_diagnostic(receipt["stderr"], expected["primary_span"]["token"])
    if parsed is None:
        return {
            **base,
            "classification": "Unknown",
            "reason_code": "DIAGNOSTIC_PARSE_FAILURE",
            "observed": None,
        }
    expected_public = {key: value for key, value in expected.items() if key not in {"kind", "exit_code"}}
    matches = receipt["exit_code"] == expected["exit_code"] and parsed == expected_public
    return {
        **base,
        "classification": "Preserved" if matches else "Violated",
        "reason_code": None if matches else "DIAGNOSTIC_MISMATCH",
        "observed": parsed,
    }


def parser_self_tests() -> dict[str, Any]:
    fixtures = [
        (
            "matching-missing-module-field",
            "<input-expression>:1:7: error: module `format` does not contain `absent`\n",
            "absent",
            "missing-module-field",
        ),
        (
            "recognized-unknown-variable",
            "<input-expression>:1:0: error: unknown variable `format`\n",
            "absent",
            "unknown-variable",
        ),
        (
            "recognized-field-access-on-none",
            "<input-expression>:1:7: error: cannot access field on none\n",
            "absent",
            "field-access-on-none",
        ),
    ]
    observed = []
    for ident, stderr, token, expected_class in fixtures:
        parsed = parse_diagnostic(stderr, token)
        if parsed is None or parsed["class"] != expected_class:
            raise RuntimeError("PARSER_SELF_TEST_FAILED: " + ident)
        observed.append({"id": ident, "class": parsed["class"]})
    if parse_diagnostic(
        "<input-expression>:1:7: error: arbitrary unallowlisted stderr\n", "absent"
    ) is not None:
        raise RuntimeError("PARSER_SELF_TEST_FAILED: unallowlisted-stderr")

    case = {
        "id": "SELF_EXPECTED_DIAGNOSTIC",
        "category": "self-test",
        "expected": {
            "kind": "DIAGNOSTIC",
            "exit_code": 1,
            "class": "missing-module-field",
            "message_core": "module `format` does not contain `absent`",
            "module": "format",
            "requested_field": "absent",
            "primary_span": {
                "line": 1,
                "start_column": 7,
                "end_column_exclusive": 13,
                "token": "absent",
            },
        },
    }
    value_receipt = {
        "profile": "default",
        "argv": ["self-test"],
        "exit_code": 0,
        "stdout": "null\n",
        "stderr": "",
        "duration_ns": 0,
        "reason_code": None,
    }
    malformed_receipt = {**value_receipt, "stdout": "not-json\n"}
    if compare_case(case, value_receipt)["classification"] != "Violated":
        raise RuntimeError("PARSER_SELF_TEST_FAILED: parseable-success")
    if compare_case(case, malformed_receipt)["classification"] != "Unknown":
        raise RuntimeError("PARSER_SELF_TEST_FAILED: malformed-json")
    recipe = {
        "recognized": observed,
        "unallowlisted_stderr": "Unknown",
        "parseable_exit_0_when_diagnostic_expected": "Violated",
        "malformed_json_exit_0": "Unknown",
    }
    return {
        "status": "PASS",
        "case_count": 6,
        "recipe_sha256": digest_bytes(canonical(recipe)),
        "recipe": recipe,
    }


def normalized_row(row: dict[str, Any]) -> dict[str, Any]:
    return {
        "id": row["id"],
        "profile": row["profile"],
        "category": row["category"],
        "mandatory": row["mandatory"],
        "classification": row["classification"],
        "reason_code": row["reason_code"],
        "exit_code": row["exit_code"],
        "observed": row["observed"],
        "stdout_sha256": row["stdout_sha256"],
        "stderr_sha256": row["stderr_sha256"],
    }


def classify(rows: list[dict[str, Any]]) -> str:
    mandatory = [row for row in rows if row["mandatory"]]
    if any(row["classification"] == "Unknown" for row in mandatory):
        return "Unknown"
    if any(row["classification"] == "Violated" for row in mandatory):
        return "Violated"
    return "Preserved"


def execute_order(
    order: str,
    jobs: list[tuple[dict[str, Any], str]],
    binary: str,
    product: str,
    timeout: float,
) -> dict[str, Any]:
    physical = list(reversed(jobs)) if order == "reverse" else jobs
    rows = []
    for case, profile in physical:
        receipt = execute(argv_for(binary, product, case["expression"], profile), timeout)
        receipt["profile"] = profile
        rows.append(compare_case(case, receipt))
    rows.sort(key=lambda row: (row["id"], PROFILE_ORDER.index(row["profile"])))
    normalized = [normalized_row(row) for row in rows]
    return {
        "order": order,
        "verdict": classify(rows),
        "normalized_sha256": digest_bytes(canonical(normalized)),
        "classification_vector": [
            f'{row["id"]}@{row["profile"]}:{row["classification"]}' for row in rows
        ],
        "results": rows,
    }


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(description=__doc__)
    result.add_argument("--binary", required=True)
    result.add_argument("--expected-sha", required=True)
    result.add_argument("--product", choices=["crystalline", "vanilla"], required=True)
    result.add_argument("--oracles", required=True)
    result.add_argument("--oracles-sha", required=True)
    result.add_argument("--suite", choices=["candidate", "reference", "red", "opaque"], required=True)
    result.add_argument(
        "--order",
        choices=["direct", "direct_repeat", "reverse", "all"],
        default="all",
    )
    result.add_argument("--timeout", type=float, default=30.0)
    result.add_argument("--output")
    return result


def main() -> None:
    args = parser().parse_args()
    try:
        binary = verify_file(args.binary, args.expected_sha, "binary")
        oracles_identity = verify_file(args.oracles, args.oracles_sha, "oracles")
        protected_inputs = {
            name: verify_file(str(path), expected, name)
            for name, (path, expected) in PROTECTED_INPUTS.items()
        }
        oracle = load_json(Path(oracles_identity["path"]))
        if oracle.get("artifact") != "p1461-oracles" or oracle.get("schema_version") != 1:
            raise RuntimeError("ORACLE_SCHEMA_UNSUPPORTED")
        if oracle.get("profiles") != PROFILE_ORDER:
            raise RuntimeError("ORACLE_PROFILE_ORDER_MISMATCH")
        cases_by_id = {case["id"]: case for case in oracle["cases"]}
        parser_tests = parser_self_tests()

        if args.suite == "opaque":
            opaque = [
                {
                    "id": item["id"],
                    "category": "opaque",
                    "classification": "Unknown",
                    "reason_code": item["reason_code"],
                    "mandatory": False,
                    "observed": None,
                }
                for item in oracle["opaque_cases"]
            ]
            orders = (
                ["direct", "direct_repeat", "reverse"]
                if args.order == "all"
                else [args.order]
            )
            runs = []
            for order in orders:
                physical = list(reversed(opaque)) if order == "reverse" else opaque
                normalized = sorted(physical, key=lambda row: row["id"])
                runs.append(
                    {
                        "order": order,
                        "verdict": "Unknown",
                        "normalized_sha256": digest_bytes(canonical(normalized)),
                        "classification_vector": [
                            f'{row["id"]}:Unknown' for row in physical
                        ],
                        "results": physical,
                    }
                )
            consistent = len({run["normalized_sha256"] for run in runs}) == 1
            emit(
                {
                    "artifact": "p1461-focal-run",
                    "schema_version": 1,
                    "scope": "deliberately opaque protocol fixtures",
                    "verdict": "Unknown",
                    "binary": binary,
                    "oracles": oracles_identity,
                    "protected_inputs": protected_inputs,
                    "parser_self_tests": parser_tests,
                    "suite": args.suite,
                    "requested_order": args.order,
                    "orders_consistent": consistent,
                    "runs": runs,
                },
                args.output,
            )
            raise SystemExit(2)

        selected_ids = oracle["suites"][args.suite]
        missing = [case_id for case_id in selected_ids if case_id not in cases_by_id]
        if missing:
            raise RuntimeError("ORACLE_CASE_MISSING: " + ",".join(missing))
        jobs = [
            (cases_by_id[case_id], profile)
            for case_id in selected_ids
            for profile in cases_by_id[case_id].get("profiles", PROFILE_ORDER)
        ]
        orders = (
            ["direct", "direct_repeat", "reverse"]
            if args.order == "all"
            else [args.order]
        )
        runs = [
            execute_order(order, jobs, binary["path"], args.product, args.timeout)
            for order in orders
        ]
        consistent = len({run["normalized_sha256"] for run in runs}) == 1
        verdict = runs[0]["verdict"]
        if any(run["verdict"] != verdict for run in runs) or not consistent:
            verdict = "Unknown"
        payload = {
            "artifact": "p1461-focal-run",
            "schema_version": 1,
            "scope": "P1461 focal probes only; C07/C09 inventory is an external gate",
            "verdict": verdict,
            "reason_code": None if consistent else "ORDER_DEPENDENCE",
            "binary": binary,
            "oracles": oracles_identity,
            "protected_inputs": protected_inputs,
            "parser_self_tests": parser_tests,
            "runner": {
                "path": str(Path(__file__).resolve()),
                "sha256": digest_file(Path(__file__).resolve()),
            },
            "product": args.product,
            "suite": args.suite,
            "requested_order": args.order,
            "orders_consistent": consistent,
            "runs": runs,
        }
        emit(payload, args.output)
        if verdict == "Unknown":
            raise SystemExit(2)
        if verdict == "Violated":
            raise SystemExit(1)
    except (RuntimeError, KeyError, json.JSONDecodeError, OSError) as exc:
        text = str(exc)
        reason = "INPUT_IDENTITY_MISMATCH" if "SHA_MISMATCH" in text else "OBSERVER_INCOMPLETE"
        emit(unknown_receipt(reason, text, args), args.output)
        raise SystemExit(2)


if __name__ == "__main__":
    main()
