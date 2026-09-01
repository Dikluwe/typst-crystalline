#!/usr/bin/env python3
"""Independent black-box oracle for P1289 float.is-infinite.

The candidate is observed only through the Typst CLI.  The oracle deliberately
does not import or inspect any crystalline implementation module.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
import tempfile
from copy import deepcopy
from pathlib import Path
from typing import Any


SCHEMA = "tekt-ab-p1289-oracle-v3"
LABEL = "p1289-oracle"


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def normalized_diagnostic(stderr: str) -> str:
    return "\n".join(line.rstrip() for line in stderr.replace("\r\n", "\n").splitlines()).strip()


def diagnostic_signature(diagnostic: str) -> list[str]:
    """Keep language diagnostics, excluding temp paths and CLI deprecation hints."""
    return [
        line.strip()
        for line in diagnostic.splitlines()
        if line.lstrip().startswith("error:")
    ]


def query_source(expression: str) -> str:
    return f"#metadata({expression}) <{LABEL}>\n"


def observe(binary: Path, case: dict[str, Any], timeout: float) -> dict[str, Any]:
    try:
        with tempfile.TemporaryDirectory(prefix="p1289-oracle-") as tmp:
            source = Path(tmp) / "case.typ"
            source.write_text(query_source(case["expression"]), encoding="utf-8")
            command = [
                str(binary),
                "query",
                str(source),
                f"<{LABEL}>",
                "--field",
                "value",
                "--one",
            ]
            completed = subprocess.run(
                command,
                stdin=subprocess.DEVNULL,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                timeout=timeout,
                check=False,
            )
    except (OSError, subprocess.SubprocessError) as error:
        return {
            "availability": "unknown",
            "reason": f"execution unavailable: {type(error).__name__}: {error}",
        }

    stdout_raw = completed.stdout.strip()
    stdout_json: Any = None
    stdout_parse_error: str | None = None
    if stdout_raw:
        try:
            stdout_json = json.loads(stdout_raw)
        except json.JSONDecodeError as error:
            stdout_parse_error = str(error)

    return {
        "availability": "observed",
        "exit_code": completed.returncode,
        "stdout_raw": stdout_raw,
        "stdout_json": stdout_json,
        "stdout_parse_error": stdout_parse_error,
        "diagnostic": normalized_diagnostic(completed.stderr),
    }


def classify(case: dict[str, Any], actual: dict[str, Any]) -> tuple[str, str]:
    if actual.get("availability") != "observed":
        return "Unknown", actual.get("reason", "observation unavailable")

    expected_exit = case["exit_code"]
    if actual["exit_code"] != expected_exit:
        return (
            "Violated",
            f"exit code {actual['exit_code']} != expected {expected_exit}",
        )

    if expected_exit == 0:
        if actual.get("stdout_parse_error") is not None:
            return "Unknown", f"unsupported JSON output: {actual['stdout_parse_error']}"
        if actual.get("stdout_json") != case["stdout"]:
            return (
                "Violated",
                f"stdout JSON {actual.get('stdout_json')!r} != expected {case['stdout']!r}",
            )
        return "Preserved", "exit code and language value match"

    expected_diagnostic = case["diagnostic"]
    if expected_diagnostic not in actual.get("diagnostic", ""):
        return (
            "Violated",
            f"diagnostic does not contain {expected_diagnostic!r}",
        )
    return "Preserved", "exit code and diagnostic witness match"


def observation_fingerprint(actual: dict[str, Any]) -> str:
    stable = {
        "availability": actual.get("availability"),
        "exit_code": actual.get("exit_code"),
        "stdout_raw": actual.get("stdout_raw"),
        "diagnostic_signature": diagnostic_signature(actual.get("diagnostic", "")),
        "reason": actual.get("reason"),
    }
    return hashlib.sha256(
        json.dumps(stable, sort_keys=True, separators=(",", ":")).encode("utf-8")
    ).hexdigest()


def canonical_observations(cases: list[dict[str, Any]]) -> dict[str, dict[str, Any]]:
    observations: dict[str, dict[str, Any]] = {}
    for case in cases:
        if case["exit_code"] == 0:
            value = deepcopy(case["stdout"])
            observations[case["id"]] = {
                "availability": "observed",
                "exit_code": 0,
                "stdout_raw": json.dumps(value, separators=(",", ":")),
                "stdout_json": value,
                "stdout_parse_error": None,
                "diagnostic": "",
            }
        else:
            observations[case["id"]] = {
                "availability": "observed",
                "exit_code": case["exit_code"],
                "stdout_raw": "",
                "stdout_json": None,
                "stdout_parse_error": None,
                "diagnostic": case["diagnostic"],
            }
    return observations


def mutate(
    name: str, observations: dict[str, dict[str, Any]]
) -> dict[str, dict[str, Any]]:
    mutated = deepcopy(observations)
    if name == "always_false":
        for case_id in ("static-values", "bound-values"):
            mutated[case_id]["stdout_json"] = [False, False, False, False, False]
            mutated[case_id]["stdout_raw"] = "[false,false,false,false,false]"
    elif name == "nan_is_infinite":
        for case_id in ("static-values", "bound-values"):
            mutated[case_id]["stdout_json"] = [False, False, True, True, True]
            mutated[case_id]["stdout_raw"] = "[false,false,true,true,true]"
    elif name == "accept_extra_or_unknown_named":
        for case_id in ("static-extra", "static-named", "bound-extra", "bound-named"):
            mutated[case_id].update(
                exit_code=0,
                stdout_raw="false",
                stdout_json=False,
                diagnostic="",
            )
    elif name == "qualified_repr":
        value = '(function, "float.is-infinite")'
        mutated["presence-repr-static"].update(
            stdout_raw=json.dumps(value), stdout_json=value
        )
    elif name == "presence_without_call":
        for case_id in ("static-values", "bound-values", "integer-coercion"):
            mutated[case_id].update(
                exit_code=1,
                stdout_raw="",
                stdout_json=None,
                diagnostic='type float does not contain field "is-infinite"',
            )
    else:
        raise ValueError(f"unknown mutation: {name}")
    return mutated


def mutation_campaign(baseline: dict[str, Any]) -> dict[str, Any]:
    cases = baseline["observables"]
    expected = {case["id"]: case for case in cases}
    canonical = canonical_observations(cases)
    results = []
    for mutation in baseline["required_mutations"]:
        mutant = mutate(mutation, canonical)
        witnesses = []
        unknown = []
        for case_id, actual in mutant.items():
            state, reason = classify(expected[case_id], actual)
            if state == "Violated":
                witnesses.append({"case": case_id, "reason": reason})
            elif state == "Unknown":
                unknown.append({"case": case_id, "reason": reason})
        killed = bool(witnesses) and not unknown
        results.append(
            {
                "mutation": mutation,
                "killed": killed,
                "witnesses": witnesses,
                "unknown": unknown,
            }
        )
    killed_count = sum(1 for result in results if result["killed"])
    total = len(results)
    return {
        "mutations": results,
        "killed": killed_count,
        "total": total,
        "mutation_score": killed_count / total if total else 0.0,
        "verdict": "Preserved" if total and killed_count == total else "Violated",
    }


def execute(binary: Path, baseline: dict[str, Any], timeout: float) -> dict[str, Any]:
    cases = baseline["observables"]
    by_id = {case["id"]: case for case in cases}
    orders = {
        "forward": [case["id"] for case in cases],
        "reverse": [case["id"] for case in reversed(cases)],
    }
    observations: dict[str, dict[str, dict[str, Any]]] = {}
    for order_name, case_ids in orders.items():
        order_results: dict[str, dict[str, Any]] = {}
        for case_id in case_ids:
            actual = observe(binary, by_id[case_id], timeout)
            state, reason = classify(by_id[case_id], actual)
            order_results[case_id] = {
                "state": state,
                "reason": reason,
                "fingerprint": observation_fingerprint(actual),
                "actual": actual,
            }
        observations[order_name] = order_results

    nondeterministic = []
    for case in cases:
        case_id = case["id"]
        forward = observations["forward"][case_id]
        reverse = observations["reverse"][case_id]
        if forward["fingerprint"] != reverse["fingerprint"]:
            nondeterministic.append(case_id)
            for item in (forward, reverse):
                item["state"] = "Unknown"
                item["reason"] = "forward/reverse observations differ"

    counts = {"Preserved": 0, "Violated": 0, "Unknown": 0}
    final_cases = []
    for case in cases:
        case_id = case["id"]
        pair = [
            observations["forward"][case_id]["state"],
            observations["reverse"][case_id]["state"],
        ]
        if "Unknown" in pair:
            state = "Unknown"
        elif "Violated" in pair:
            state = "Violated"
        else:
            state = "Preserved"
        counts[state] += 1
        final_cases.append(
            {
                "id": case_id,
                "state": state,
                "forward": observations["forward"][case_id],
                "reverse": observations["reverse"][case_id],
            }
        )

    if counts["Unknown"]:
        verdict = "Unknown"
    elif counts["Violated"]:
        verdict = "Violated"
    else:
        verdict = "Preserved"

    campaign = mutation_campaign(baseline)
    if campaign["verdict"] != "Preserved" and verdict == "Preserved":
        verdict = "Violated"

    return {
        "schema": SCHEMA,
        "contract_id": baseline["contract_id"],
        "manifest_sha256": baseline["manifest_sha256"],
        "candidate": {"path": str(binary), "sha256": sha256(binary)},
        "orders": ["forward", "reverse"],
        "deterministic": not nondeterministic,
        "nondeterministic_cases": nondeterministic,
        "cases": final_cases,
        "summary": {"total": len(cases), **counts, "verdict": verdict},
        "mutation_campaign": campaign,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--candidate", type=Path)
    parser.add_argument(
        "--baseline",
        type=Path,
        default=Path(__file__).with_name("p1289-oracle-baseline.json"),
    )
    parser.add_argument("--timeout", type=float, default=30.0)
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--summary-only", action="store_true")
    parser.add_argument("--pretty", action="store_true")
    args = parser.parse_args()

    try:
        baseline = json.loads(args.baseline.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        print(json.dumps({"verdict": "Unknown", "reason": str(error)}))
        return 2

    if args.self_test:
        result: dict[str, Any] = {
            "schema": SCHEMA,
            "contract_id": baseline["contract_id"],
            "manifest_sha256": baseline["manifest_sha256"],
            "mutation_campaign": mutation_campaign(baseline),
        }
        ok = result["mutation_campaign"]["verdict"] == "Preserved"
    else:
        if args.candidate is None:
            parser.error("--candidate is required unless --self-test is used")
        if not args.candidate.is_file():
            result = {
                "schema": SCHEMA,
                "summary": {
                    "total": len(baseline.get("observables", [])),
                    "Preserved": 0,
                    "Violated": 0,
                    "Unknown": len(baseline.get("observables", [])),
                    "verdict": "Unknown",
                },
                "reason": f"candidate is not a file: {args.candidate}",
            }
            ok = False
        else:
            result = execute(args.candidate.resolve(), baseline, args.timeout)
            ok = (
                result["summary"]["verdict"] == "Preserved"
                and result["summary"]["Unknown"] == 0
                and result["summary"]["Violated"] == 0
                and result["mutation_campaign"]["mutation_score"] == 1.0
            )

    if args.summary_only and not args.self_test and "summary" in result:
        result = {
            "schema": result.get("schema"),
            "contract_id": result.get("contract_id"),
            "manifest_sha256": result.get("manifest_sha256"),
            "candidate": result.get("candidate"),
            "orders": result.get("orders"),
            "deterministic": result.get("deterministic"),
            "nondeterministic_cases": result.get("nondeterministic_cases"),
            "summary": result["summary"],
            "mutation_campaign": {
                "killed": result.get("mutation_campaign", {}).get("killed"),
                "total": result.get("mutation_campaign", {}).get("total"),
                "mutation_score": result.get("mutation_campaign", {}).get("mutation_score"),
                "verdict": result.get("mutation_campaign", {}).get("verdict"),
            },
        }
    print(json.dumps(result, indent=2 if args.pretty else None, sort_keys=True))
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
