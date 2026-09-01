#!/usr/bin/env python3
"""Execute the sealed P1284 semantic oracle against a frozen candidate.

The candidate query serializer is deliberately assigned to P1285.  Therefore,
value probes are observed by replacing their final metadata node with an exact
Typst assertion and compiling the source.  The sealed source expression and
expected value are otherwise unchanged.  Error probes and blocked controls are
compiled verbatim; color maps are read through ``typst eval``.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import pathlib
import subprocess
import tempfile


ROOT = pathlib.Path(__file__).resolve().parents[2]
DEFAULT_ORACLE = pathlib.Path(__file__).with_name("p1284-probes.json")


def sha256(path: pathlib.Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def typst_literal(value: object) -> str:
    if value is True:
        return "true"
    if value is False:
        return "false"
    if value is None:
        return "none"
    if isinstance(value, str):
        return json.dumps(value, ensure_ascii=False)
    if isinstance(value, (int, float)):
        return repr(value)
    if isinstance(value, list):
        items = ",".join(typst_literal(item) for item in value)
        return f"({items}{',' if len(value) == 1 else ''})"
    raise TypeError(f"unsupported expected value: {value!r}")


def assertion_source(source: str, expected: object) -> str:
    marker = " <p1284>"
    start = source.rfind("#metadata(")
    end = source.rfind(marker)
    if start < 0 or end < start:
        raise ValueError("sealed value probe has no final P1284 metadata node")
    expression = source[start + len("#metadata(") : end]
    if not expression.endswith(")"):
        raise ValueError("sealed metadata node is not balanced at its boundary")
    expression = expression[:-1]
    replacement = f"#assert(({expression}) == {typst_literal(expected)})"
    return source[:start] + replacement + source[end + len(marker) :]


def compile_source(binary: pathlib.Path, source: str, work: pathlib.Path) -> dict:
    input_path = work / "probe.typ"
    output_path = work / "probe.pdf"
    input_path.write_text(source)
    run = subprocess.run(
        [str(binary), "compile", str(input_path), str(output_path)],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    return {
        "exit_code": run.returncode,
        "stdout": run.stdout,
        "stderr": run.stderr.replace(str(work), "<TMP>"),
    }


def eval_json(binary: pathlib.Path, expression: str) -> dict:
    run = subprocess.run(
        [str(binary), "eval", expression, "--format", "json"],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    value = json.loads(run.stdout) if run.returncode == 0 else None
    return {"exit_code": run.returncode, "value": value, "stderr": run.stderr}


def color_token(value: str) -> str:
    token = value.lower()
    if token.startswith("#"):
        token = "0x" + token[1:]
    if len(token) == 8:
        token += "ff"
    return token


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--candidate", type=pathlib.Path, required=True)
    parser.add_argument("--oracle", type=pathlib.Path, default=DEFAULT_ORACLE)
    parser.add_argument("--output", type=pathlib.Path, required=True)
    args = parser.parse_args()
    candidate = args.candidate.resolve()
    oracle = json.loads(args.oracle.read_text())

    value_groups = (
        oracle["six_sentinels"]
        + oracle["required_extra_sentinels"]
        + oracle["real_call_probes"]
        + [
            probe
            for probe in oracle["argument_and_error_probes"]
            if "expect_query_json" in probe
        ]
    )
    error_probes = [
        probe
        for probe in oracle["argument_and_error_probes"]
        if "expect_query_json" not in probe
    ]
    nested = next(
        probe["unbound_projection_control"]
        for probe in oracle["real_call_probes"]
        if "unbound_projection_control" in probe
    )

    receipts = []
    with tempfile.TemporaryDirectory(prefix="p1284-oracle-") as temp:
        work = pathlib.Path(temp)
        for index, probe in enumerate(value_groups):
            probe_work = work / f"value-{index}"
            probe_work.mkdir()
            run = compile_source(
                candidate,
                assertion_source(probe["source"], probe["expect_query_json"]),
                probe_work,
            )
            receipts.append(
                {
                    "id": probe["id"],
                    "class": "value",
                    "observation": "compile_exact_assertion",
                    "passed": run["exit_code"] == 0,
                    "run": run,
                }
            )

        for index, probe in enumerate(error_probes):
            probe_work = work / f"error-{index}"
            probe_work.mkdir()
            run = compile_source(candidate, probe["source"], probe_work)
            expected = probe.get("candidate_expect", probe.get("expect"))
            needle = probe.get("candidate_must_include", probe.get("must_include", ""))
            passed = (run["exit_code"] != 0 if expected == "compile_error" else run["exit_code"] == 0)
            if needle:
                passed = passed and needle in run["stderr"]
            receipts.append(
                {
                    "id": probe["id"],
                    "class": "argument_or_error",
                    "observation": "compile_verbatim",
                    "passed": passed,
                    "expected": expected,
                    "must_include": needle,
                    "run": run,
                }
            )

        nested_work = work / "nested-control"
        nested_work.mkdir()
        nested_run = compile_source(candidate, nested["source"], nested_work)
        nested_receipt = {
            "id": "array-mutators-unbound-projection-control",
            "class": "expected_language_limitation_not_success",
            "passed": nested_run["exit_code"] != 0
            and nested["must_include"] in nested_run["stderr"],
            "run": nested_run,
        }

    map_receipts = []
    for member in oracle["color_maps"]["members"]:
        run = eval_json(candidate, f"color.map.{member['name']}.map(c => c.to-hex())")
        tokens = [color_token(value) for value in (run["value"] or [])]
        digest = hashlib.sha256(",".join(tokens).encode()).hexdigest()
        passed = (
            run["exit_code"] == 0
            and len(tokens) == member["cardinality"]
            and tokens[0] == member["first"]
            and tokens[-1] == member["last"]
            and digest == member["sha256"]
        )
        map_receipts.append(
            {
                "name": member["name"],
                "passed": passed,
                "cardinality": len(tokens),
                "first": tokens[0] if tokens else None,
                "last": tokens[-1] if tokens else None,
                "sha256": digest,
                "run": {"exit_code": run["exit_code"], "stderr": run["stderr"]},
            }
        )

    blocked_receipts = []
    for path in ("color.spot", "outline.entry", "selector.before", "selector.after"):
        run = eval_json(candidate, path)
        blocked_receipts.append(
            {"path": path, "passed": run["exit_code"] != 0, "run": run}
        )

    passed = sum(item["passed"] for item in receipts)
    payload = {
        "schema": "tekt.p1284.candidate-oracle-receipt.v1",
        "oracle_id": oracle["oracle_id"],
        "oracle_sha256": sha256(args.oracle),
        "candidate": str(candidate),
        "candidate_sha256": sha256(candidate),
        "observation_note": "P1284 values use exact compile assertions because query serialization is assigned to P1285; sealed expressions and expected values are unchanged.",
        "functional": {"passed": passed, "total": len(receipts), "receipts": receipts},
        "nested_control": nested_receipt,
        "color_maps": {
            "passed": sum(item["passed"] for item in map_receipts),
            "total": len(map_receipts),
            "receipts": map_receipts,
        },
        "blocked_controls": blocked_receipts,
        "all_passed": (
            passed == len(receipts)
            and nested_receipt["passed"]
            and all(item["passed"] for item in map_receipts)
            and all(item["passed"] for item in blocked_receipts)
        ),
    }
    args.output.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n")
    print(
        json.dumps(
            {
                "functional": f"{passed}/{len(receipts)}",
                "nested_control": nested_receipt["passed"],
                "color_maps": f"{payload['color_maps']['passed']}/{len(map_receipts)}",
                "blocked_controls": sum(item["passed"] for item in blocked_receipts),
                "all_passed": payload["all_passed"],
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
