#!/usr/bin/env python3
"""P1301r2 binding-free contract discriminator and public candidate runner.

This sealer-owned runner reads only the seven frozen P1301r2 inputs.  Its
candidate mode invokes a binary exclusively through the common CLI surface
frozen by the contract and performs syntactic extraction without translating
diagnostic content.
"""

from __future__ import annotations

import argparse
import base64
import copy
import datetime as dt
import hashlib
import json
import pathlib
import re
import subprocess
import sys
import time
from typing import Any


HERE = pathlib.Path(__file__).resolve().parent
ATTESTATION = "executado sem atestação de isolamento técnico"
FIELDS = [
    "exit_code",
    "stdout",
    "diagnostic_class",
    "message",
    "ordered_hints",
    "span_bytes",
]
FORBIDDEN_ARGUMENTS = {"--diagnostic-format", "--target"}
IMPLEMENTED_OPERATIONS = {
    "replace",
    "replace_template",
    "replace_from_case_parameter",
    "append",
    "promote_unknown",
    "carry_previous_span",
}
INPUT_HASHES = {
    "p1301r2-manifest.json": "1526dc81e08a36ce0153fe7639dd2558c220a85d44c4b1de7098a59c099a5712",
    "p1301r2-contract.json": "69e852d38636afd0ce996f10a64cbf0e6dc0be12dca453a65ee3e0d515c88fad",
    "p1301r2-contract-author-receipt.md": "34c777fa06897e6dffe0efa40abc4cdcae67bcff151722113abef08b40b68318",
    "p1301r2-oracle-suite.json": "4c1cbbafd5e81517061a2c369186a41d6de30a41abc29ff122ee9e77ed64b3ac",
    "p1301r2-oracle-author-receipt.md": "49811af01fbcca2e6faede7f094f5cf0d16ac70df9087d6f4e1099097ba97bf4",
    "p1301r2-mutants.json": "a9a99fcf1e714f0e54390ba87fb9afcd8054c2cb258e68e70bc14f65a2fb4c10",
    "p1301r2-mutation-plan.json": "c6d55f995788d25ed4e6b8b9ed881efa621a12f60daacd4d5ca6d134f0dde0b8",
}


class GateError(RuntimeError):
    def __init__(self, reason_code: str, detail: str):
        super().__init__(detail)
        self.reason_code = reason_code
        self.detail = detail


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: pathlib.Path) -> str:
    return sha256_bytes(path.read_bytes())


def canonical_bytes(value: Any) -> bytes:
    return json.dumps(
        value, ensure_ascii=False, sort_keys=True, separators=(",", ":")
    ).encode("utf-8")


def load_inputs() -> tuple[dict[str, bytes], dict[str, Any]]:
    raw: dict[str, bytes] = {}
    parsed: dict[str, Any] = {}
    for name, expected in INPUT_HASHES.items():
        path = HERE / name
        data = path.read_bytes()
        actual = sha256_bytes(data)
        if actual != expected:
            raise GateError(
                "INPUT_HASH_MISMATCH",
                f"{name}: expected {expected}, observed {actual}",
            )
        raw[name] = data
        if name.endswith(".json"):
            try:
                parsed[name] = json.loads(data)
            except (UnicodeDecodeError, json.JSONDecodeError) as exc:
                raise GateError("INPUT_JSON_INVALID", f"{name}: {exc}") from exc
    return raw, parsed


def require(condition: bool, reason: str, detail: str) -> None:
    if not condition:
        raise GateError(reason, detail)


def pointer_parts(pointer: str) -> list[str]:
    require(pointer.startswith("/"), "INVALID_JSON_POINTER", pointer)
    return [part.replace("~1", "/").replace("~0", "~") for part in pointer[1:].split("/")]


def pointer_get(record: dict[str, Any], pointer: str) -> Any:
    value: Any = record
    for part in pointer_parts(pointer):
        require(isinstance(value, dict) and part in value, "MISSING_POINTER", pointer)
        value = value[part]
    return value


def pointer_set(record: dict[str, Any], pointer: str, value: Any) -> None:
    parts = pointer_parts(pointer)
    target: Any = record
    for part in parts[:-1]:
        require(isinstance(target, dict) and part in target, "MISSING_POINTER", pointer)
        target = target[part]
    require(isinstance(target, dict) and parts[-1] in target, "MISSING_POINTER", pointer)
    target[parts[-1]] = copy.deepcopy(value)


def exact_envelope(value: dict[str, Any]) -> dict[str, Any]:
    require(list(value.keys()) == FIELDS, "ENVELOPE_FIELD_ORDER_MISMATCH", repr(list(value.keys())))
    return {field: copy.deepcopy(value[field]) for field in FIELDS}


def allowed_pointer(pointer: str) -> bool:
    return pointer in {f"/envelope/{field}" for field in FIELDS}


def build_argv(binary: str, case: dict[str, Any], contract: dict[str, Any]) -> list[str]:
    profile = case["profile"]
    profiles = contract["cli_surface"]["profiles"]
    require(profile in profiles, "UNKNOWN_PROFILE", profile)
    argv = [binary, "eval", case["expression"], "--format", "json"] + list(profiles[profile])
    validate_argv(argv, binary, case["expression"], list(profiles[profile]))
    return argv


def validate_argv(argv: list[str], binary: str, expression: str, suffix: list[str]) -> None:
    if any(arg in FORBIDDEN_ARGUMENTS for arg in argv):
        raise GateError("INVALID_CONTRACT_EXECUTION", repr(argv))
    expected = [binary, "eval", expression, "--format", "json"] + suffix
    if argv != expected:
        raise GateError("INVALID_CONTRACT_EXECUTION", f"expected {expected!r}, got {argv!r}")


def unknown(reason: str, exit_code: int, stdout: str | None, stderr: str | None) -> dict[str, Any]:
    result: dict[str, Any] = {
        "kind": "Unknown",
        "reason_code": reason,
        "exit_code": exit_code,
        "stdout": stdout,
        "raw_stderr": stderr,
    }
    return result


def parse_human(
    expression: str,
    exit_code: int,
    stdout_bytes: bytes,
    stderr_bytes: bytes,
) -> dict[str, Any]:
    try:
        stdout = stdout_bytes.decode("utf-8")
        stderr = stderr_bytes.decode("utf-8")
    except UnicodeDecodeError:
        return unknown("NON_UTF8_PROCESS_STREAM", exit_code, None, None)

    if exit_code == 0 and stderr == "":
        return {
            "kind": "Envelope",
            "envelope": {
                "exit_code": exit_code,
                "stdout": stdout,
                "diagnostic_class": None,
                "message": None,
                "ordered_hints": [],
                "span_bytes": None,
            },
        }

    if "\n" in expression or "\t" in expression or not all(0x20 <= ord(ch) <= 0x7E for ch in expression):
        return unknown("UNSUPPORTED_SOURCE_SHAPE", exit_code, stdout, stderr)

    headers = re.findall(r"^(error|warning): (.*)$", stderr, flags=re.MULTILINE)
    if len(headers) != 1:
        return unknown("UNSUPPORTED_DIAGNOSTIC_CARDINALITY", exit_code, stdout, stderr)

    coordinates = re.findall(r"<input-expression>:(\d+):(\d+)", stderr)
    if len(coordinates) != 1:
        return unknown("MISSING_OR_AMBIGUOUS_PRIMARY_SPAN", exit_code, stdout, stderr)
    line_number, coordinate_column = (int(part) for part in coordinates[0])

    source_echoes = re.findall(r"^1 │ (.*)$", stderr, flags=re.MULTILINE)
    if len(source_echoes) != 1 or source_echoes[0] != expression:
        return unknown("SOURCE_ECHO_MISMATCH", exit_code, stdout, stderr)

    carets = re.findall(r"^  │ (?P<spaces> *)(?P<carets>\^+)\s*$", stderr, flags=re.MULTILINE)
    if len(carets) != 1:
        return unknown("MISSING_OR_AMBIGUOUS_PRIMARY_SPAN", exit_code, stdout, stderr)
    spaces, caret_run = carets[0]
    if line_number != 1 or coordinate_column != len(spaces):
        return unknown("COORDINATE_CARET_MISMATCH", exit_code, stdout, stderr)

    diagnostic_class, message = headers[0]
    hints = re.findall(r"^  = hint: (.*)$", stderr, flags=re.MULTILINE)
    return {
        "kind": "Envelope",
        "envelope": {
            "exit_code": exit_code,
            "stdout": stdout,
            "diagnostic_class": diagnostic_class,
            "message": message,
            "ordered_hints": hints,
            "span_bytes": [coordinate_column, coordinate_column + len(caret_run)],
        },
    }


def case_index(contract: dict[str, Any]) -> dict[str, dict[str, Any]]:
    return {case["id"]: case for case in contract["cases"]}


def audit_oracle(contract: dict[str, Any], oracle: dict[str, Any]) -> dict[str, Any]:
    cases = case_index(contract)
    parser_samples = 0
    computed_vectors: dict[str, list[dict[str, Any]]] = {}
    for order in ("normal", "inverted"):
        expected_ids = [case_id for case_id in contract["execution_orders"][order] if case_id != "C23-dictionary-crystalline-span-sentinel"]
        runs = oracle["runs"][order]
        require([run["case_id"] for run in runs] == expected_ids, "ORACLE_ORDER_MISMATCH", order)
        vector: list[dict[str, Any]] = []
        for run in runs:
            case = cases[run["case_id"]]
            expected_argv = build_argv("/usr/local/bin/typst", case, contract)
            require(run["argv"] == expected_argv, "ORACLE_ARGV_MISMATCH", run["case_id"])
            require(not (FORBIDDEN_ARGUMENTS & set(run["argv"])), "FORBIDDEN_ORACLE_ARGUMENT", run["case_id"])
            raw_stderr = run["raw_stderr"].encode("utf-8")
            require(sha256_bytes(raw_stderr) == run["raw_stderr_sha256"], "ORACLE_STDERR_HASH_MISMATCH", run["case_id"])
            parsed = parse_human(
                case["expression"],
                run["exit_code_raw"],
                run["stdout_raw"].encode("utf-8"),
                raw_stderr,
            )
            parser_samples += 1
            if case["candidate_mode"] == "bilateral_declared_opaque":
                require(parsed["kind"] == "Unknown", "ORACLE_OPAQUE_NOT_UNKNOWN", run["case_id"])
                require(parsed["reason_code"] == case["expected"]["reason_code"], "ORACLE_OPAQUE_REASON_MISMATCH", run["case_id"])
                require(run["classification"] == "Unknown" and run["reason_code"] == parsed["reason_code"] and run["envelope"] is None, "ORACLE_OPAQUE_RECORD_MISMATCH", run["case_id"])
            else:
                require(parsed["kind"] == "Envelope", "ORACLE_REQUIRED_UNKNOWN", run["case_id"])
                require(parsed["envelope"] == case["expected"], "ORACLE_PARSED_ENVELOPE_MISMATCH", run["case_id"])
                require(run["classification"] == "Preserved" and run["reason_code"] is None, "ORACLE_CLASSIFICATION_MISMATCH", run["case_id"])
                require(run["envelope"] == parsed["envelope"], "ORACLE_STORED_ENVELOPE_MISMATCH", run["case_id"])
            vector.append({key: copy.deepcopy(run[key]) for key in ("case_id", "classification", "reason_code", "envelope")})
        vector.sort(key=lambda record: record["case_id"])
        computed_vectors[order] = vector
        digest = sha256_bytes(canonical_bytes(vector))
        require(vector == oracle["canonical_order_check"][f"{order}_vector"], "ORACLE_CANONICAL_VECTOR_MISMATCH", order)
        require(digest == oracle["canonical_order_check"][f"{order}_sha256"], "ORACLE_CANONICAL_HASH_MISMATCH", order)
        require(oracle["counts"][order] == {"total": 22, "Preserved": 20, "Violated": 0, "Unknown": 2}, "ORACLE_COUNT_MISMATCH", order)
    require(computed_vectors["normal"] == computed_vectors["inverted"], "ORACLE_ORDER_DEPENDENCE", "normal != inverted")
    require(oracle["canonical_order_check"]["byte_identical"] is True, "ORACLE_ORDER_FLAG_MISMATCH", "byte_identical")
    sentinel = cases["C23-dictionary-crystalline-span-sentinel"]
    require(oracle["crystalline_sentinel"]["case_id"] == sentinel["id"], "ORACLE_SENTINEL_ID_MISMATCH", sentinel["id"])
    require(oracle["crystalline_sentinel"]["contract_expected"] == sentinel["expected"], "ORACLE_SENTINEL_MISMATCH", sentinel["id"])
    require(oracle["crystalline_sentinel"]["vanilla_execution"] == "NOT_RUN_NON_DECISORY", "ORACLE_SENTINEL_EXECUTED", sentinel["id"])
    return {
        "raw_samples_tested": parser_samples,
        "normal_counts": oracle["counts"]["normal"],
        "inverted_counts": oracle["counts"]["inverted"],
        "canonical_sha256": oracle["canonical_order_check"]["normal_sha256"],
        "sentinel": "SEPARATE_NOT_RUN_NON_DECISORY",
    }


def preflight(raw: dict[str, bytes], data: dict[str, Any]) -> list[dict[str, Any]]:
    manifest = data["p1301r2-manifest.json"]
    contract = data["p1301r2-contract.json"]
    oracle = data["p1301r2-oracle-suite.json"]
    mutants = data["p1301r2-mutants.json"]
    plan = data["p1301r2-mutation-plan.json"]
    gates: list[dict[str, Any]] = []

    # P01: every supplied input is byte-pinned; plan's four frozen inputs also agree.
    require({name: sha256_bytes(content) for name, content in raw.items()} == INPUT_HASHES, "INPUT_HASH_MISMATCH", "seven-input freeze")
    for entry in plan["frozen_inputs"]:
        require(INPUT_HASHES[pathlib.Path(entry["path"]).name] == entry["sha256"], "INPUT_HASH_MISMATCH", entry["path"])
    gates.append({"id": "P01-hash-freeze", "result": "PASS", "input_count": 7})

    require(contract["schema_version"] == "tekt-observable-contract-v2" and contract["step"] == "P1301" and contract["revision"] == 2, "CONTRACT_IDENTITY_MISMATCH", "schema/step/revision")
    require(contract["manifest"] == {"path": "00_nucleo/diagnosticos/p1301r2-manifest.json", "sha256": INPUT_HASHES["p1301r2-manifest.json"]}, "CONTRACT_IDENTITY_MISMATCH", "manifest")
    gates.append({"id": "P02-contract-identity", "result": "PASS"})

    contract_ids = [item["id"] for item in contract["mutants"]]
    mutant_ids = [item["id"] for item in mutants["mutants"]]
    run_ids = plan["run_matrix"]["mutant_ids"]
    require(contract_ids == mutant_ids == run_ids and len(set(run_ids)) == 12, "MUTANT_SET_MISMATCH", repr(run_ids))
    require([item["mutant_id"] for item in plan["expected_discrimination"]] == run_ids, "MUTANT_SET_MISMATCH", "expected_discrimination")
    gates.append({"id": "P03-mutant-bijection", "result": "PASS", "mutant_count": 12})

    contract_mutants = {item["id"]: item for item in contract["mutants"]}
    for mutant in mutants["mutants"]:
        require(mutant["kill_cases"] == contract_mutants[mutant["id"]]["witness_cases"], "WITNESS_SET_MISMATCH", mutant["id"])
    gates.append({"id": "P04-witness-preservation", "result": "PASS"})

    cases = contract["cases"]
    ids = [case["id"] for case in cases]
    require(len(ids) == 23 and len(set(ids)) == 23, "ORDER_SET_OR_SEQUENCE_MISMATCH", "case ids")
    for order in ("normal", "inverted"):
        require(mutants["execution_orders"][order] == contract["execution_orders"][order], "ORDER_SET_OR_SEQUENCE_MISMATCH", order)
        require(sorted(contract["execution_orders"][order]) == sorted(ids), "ORDER_SET_OR_SEQUENCE_MISMATCH", order)
    require(contract["execution_orders"]["inverted"] == list(reversed(contract["execution_orders"]["normal"])), "ORDER_SET_OR_SEQUENCE_MISMATCH", "reverse")
    gates.append({"id": "P05-order-preservation", "result": "PASS", "case_count": 23})

    used_operations: list[str] = []
    for mutant in mutants["mutants"]:
        for operation in mutant["transformation"]["operations"]:
            op = operation["op"]
            used_operations.append(op)
            if "pointer" in operation:
                require(allowed_pointer(operation["pointer"]), "NON_COMMON_SURFACE_MUTATION", f"{mutant['id']} {operation['pointer']}")
            require("raw_stderr" not in json.dumps(operation), "NON_COMMON_SURFACE_MUTATION", mutant["id"])
    gates.append({"id": "P06-envelope-only-boundary", "result": "PASS"})

    used = set(used_operations)
    declared = set(mutants["application_model"]["operation_semantics"])
    require(used == declared == IMPLEMENTED_OPERATIONS, "UNSUPPORTED_MUTATION_OPERATION", f"used={sorted(used)}, declared={sorted(declared)}, implemented={sorted(IMPLEMENTED_OPERATIONS)}")
    gates.append({"id": "P07-operation-closure", "result": "PASS", "operations": sorted(used), "unsupported_count": 0})

    # Full cross-artifact invariant audit.
    for document in (manifest, contract, oracle, mutants, plan):
        require(document["step"] == "P1301" and document["revision"] == 2, "CROSS_ARTIFACT_IDENTITY_MISMATCH", document.get("schema_version", "?"))
        require(document["protocol_regime"] == "complete" and document["attestation_language"] == ATTESTATION, "ATTESTATION_MISMATCH", document.get("schema_version", "?"))
    require(manifest["technical_isolation_attested"] is False, "ATTESTATION_MISMATCH", "manifest technical isolation")
    require(contract["logical_case_count"] == 23 and contract["vanilla_case_count"] == 22 and contract["opaque_case_count"] == 2 and contract["crystalline_sentinel_count"] == 1, "CONTRACT_COUNT_MISMATCH", "logical partitions")
    require(contract["common_envelope"]["fields_in_exact_comparison_order"] == FIELDS, "ENVELOPE_FIELD_ORDER_MISMATCH", "contract")
    require(contract["cli_surface"]["only_command_shape"] == ["eval", "<expression>", "--format", "json"], "CLI_SURFACE_MISMATCH", "shape")
    require(set(contract["cli_surface"]["forbidden_arguments"]) == FORBIDDEN_ARGUMENTS, "CLI_SURFACE_MISMATCH", "forbidden")
    require(manifest["common_cli_surface"]["forbidden_harness_arguments"] == contract["cli_surface"]["forbidden_arguments"], "CLI_SURFACE_MISMATCH", "manifest/contract")
    require(manifest["prompts"] == [{**prompt, "consumer": manifest["prompts"][index]["consumer"]} for index, prompt in enumerate(contract["frozen_inputs"]["prompts"])], "PROMPT_FREEZE_MISMATCH", "manifest/contract")
    require(contract["frozen_inputs"]["vanilla"]["sha256"] == manifest["baseline"]["vanilla_sha256"] == oracle["baseline"]["sha256"], "BASELINE_IDENTITY_MISMATCH", "vanilla")
    require(contract["frozen_inputs"]["vanilla"]["ratified_upstream_commit"] == manifest["baseline"]["ratified_upstream_commit"] == oracle["baseline"]["ratified_upstream_commit"], "BASELINE_IDENTITY_MISMATCH", "commit")
    require(oracle["frozen_inputs"]["manifest"]["sha256"] == INPUT_HASHES["p1301r2-manifest.json"], "ORACLE_INPUT_MISMATCH", "manifest")
    require(oracle["frozen_inputs"]["contract"]["sha256"] == INPUT_HASHES["p1301r2-contract.json"], "ORACLE_INPUT_MISMATCH", "contract")
    require(oracle["frozen_inputs"]["contract_author_receipt"]["sha256"] == INPUT_HASHES["p1301r2-contract-author-receipt.md"], "ORACLE_INPUT_MISMATCH", "receipt")
    for entry in mutants["frozen_inputs"]:
        require(INPUT_HASHES[pathlib.Path(entry["path"]).name] == entry["sha256"], "MUTANT_INPUT_MISMATCH", entry["path"])
    require(INPUT_HASHES["p1301r2-contract.json"].encode() in raw["p1301r2-contract-author-receipt.md"], "AUTHOR_RECEIPT_MISMATCH", "contract hash")
    require(INPUT_HASHES["p1301r2-oracle-suite.json"].encode() in raw["p1301r2-oracle-author-receipt.md"], "AUTHOR_RECEIPT_MISMATCH", "oracle hash")
    require(ATTESTATION.encode("utf-8") in raw["p1301r2-contract-author-receipt.md"] and ATTESTATION.encode("utf-8") in raw["p1301r2-oracle-author-receipt.md"], "ATTESTATION_MISMATCH", "author receipts")
    require(plan["run_matrix"] == {
        "mutant_ids": run_ids,
        "orders": ["normal", "inverted"],
        "repetitions_per_order": 2,
        "total_mutant_runs": 48,
        "baseline_control_runs": 4,
        "total_planned_runs": 52,
        "parallelism": plan["run_matrix"]["parallelism"],
        "raw_stderr": plan["run_matrix"]["raw_stderr"],
    }, "RUN_MATRIX_MISMATCH", "matrix")
    require(contract["discrimination_gate"]["required_killed_mutants"] == plan["gate"]["required_killed_mutants"] == 12, "GATE_POLICY_MISMATCH", "kills")
    require(contract["discrimination_gate"]["required_mutation_score"] == plan["gate"]["required_mutation_score"] == 1.0, "GATE_POLICY_MISMATCH", "score")
    require(plan["gate"]["survivors_allowed"] == plan["gate"]["invalid_mutants_allowed"] == plan["gate"]["baseline_regressions_allowed"] == plan["gate"]["undeclared_unknown_allowed"] == 0, "GATE_POLICY_MISMATCH", "zero allowances")
    require(contract["calibration_budget"]["remaining_contract_revisions_after_r2"] == 0 and contract["calibration_budget"]["harness_redesign_budget"] == "EXHAUSTED", "CALIBRATION_BUDGET_MISMATCH", "r2")
    oracle_check = audit_oracle(contract, oracle)
    require(oracle["oracle_verdict"]["overall"] == "PASS", "ORACLE_VERDICT_MISMATCH", "overall")
    gates.append({"id": "P08-full-input-invariants", "result": "PASS", "oracle": oracle_check})
    return gates


def nominal_corpus(contract: dict[str, Any]) -> dict[str, dict[str, Any]]:
    corpus: dict[str, dict[str, Any]] = {}
    for case in contract["cases"]:
        if case["candidate_mode"] == "bilateral_declared_opaque":
            corpus[case["id"]] = {
                "case_id": case["id"],
                "profile": case["profile"],
                "classification": "Unknown",
                "reason_code": case["expected"]["reason_code"],
                "envelope": None,
                "declared_diagnostic_count": 2,
            }
        else:
            corpus[case["id"]] = {
                "case_id": case["id"],
                "profile": case["profile"],
                "classification": "Preserved",
                "reason_code": None,
                "envelope": exact_envelope(case["expected"]),
            }
    return corpus


def executable_ids(order: list[str], contract_cases: dict[str, dict[str, Any]]) -> list[str]:
    return [case_id for case_id in order if contract_cases[case_id]["candidate_mode"] != "vanilla_reference_only"]


def vector_for(corpus: dict[str, dict[str, Any]], ids: list[str]) -> list[dict[str, Any]]:
    vector = []
    for case_id in ids:
        record = corpus[case_id]
        vector.append({key: copy.deepcopy(record.get(key)) for key in ("case_id", "classification", "reason_code", "envelope")})
    vector.sort(key=lambda record: record["case_id"])
    return vector


def selected_ids(mutant: dict[str, Any], mutants: dict[str, Any], order: list[str], cases: dict[str, dict[str, Any]]) -> list[str]:
    selector = mutant["transformation"]["selector"]
    if "case_group" in selector:
        chosen = mutants["case_groups"].get(selector["case_group"])
        require(chosen is not None, "INVALID_SELECTOR", mutant["id"])
        return [case_id for case_id in order if case_id in chosen]
    if "case_ids" in selector:
        chosen = selector["case_ids"]
        require(len(chosen) == len(set(chosen)) and all(case_id in cases for case_id in chosen), "INVALID_SELECTOR", mutant["id"])
        return [case_id for case_id in order if case_id in chosen]
    if selector.get("all_executable_ordered_case_records") is True:
        excluded_modes = set(selector.get("exclude_candidate_modes", []))
        return [case_id for case_id in order if cases[case_id]["candidate_mode"] not in excluded_modes]
    raise GateError("INVALID_SELECTOR", mutant["id"])


def verify_precondition(mutant: dict[str, Any], corpus: dict[str, dict[str, Any]], selected: list[str], mutants: dict[str, Any]) -> None:
    pre = mutant["precondition"]
    allowed = {
        "case_group", "case_id", "classification", "envelope.message_template",
        "span_is_non_null", "whole_access_span_differs_for_kill_cases", "message_prefix",
        "std_three_field_cases_unchanged", "pointer", "equals", "all", "profiles_exactly",
        "corresponding_default_cases_unchanged", "reason_code", "declared_diagnostic_count",
        "orders", "reset_state_per_order", "at_least_two_distinct_non_null_original_spans",
    }
    require(set(pre) <= allowed, "INVALID_PRECONDITION", f"{mutant['id']} keys")
    if "case_id" in pre:
        require(selected == [pre["case_id"]], "INVALID_PRECONDITION", f"{mutant['id']} case_id")
    if "case_group" in pre:
        require(set(selected) == set(mutants["case_groups"][pre["case_group"]]), "INVALID_PRECONDITION", f"{mutant['id']} group")
    if "classification" in pre:
        require(all(corpus[case_id]["classification"] == pre["classification"] for case_id in selected), "INVALID_PRECONDITION", f"{mutant['id']} classification")
    if "reason_code" in pre:
        require(all(corpus[case_id]["reason_code"] == pre["reason_code"] for case_id in selected), "INVALID_PRECONDITION", f"{mutant['id']} reason")
    if "declared_diagnostic_count" in pre:
        require(all(corpus[case_id].get("declared_diagnostic_count") == pre["declared_diagnostic_count"] for case_id in selected), "INVALID_PRECONDITION", f"{mutant['id']} cardinality")
    if "pointer" in pre:
        require(pointer_get(corpus[pre["case_id"]], pre["pointer"]) == pre["equals"], "INVALID_PRECONDITION", f"{mutant['id']} pointer")
    for assertion in pre.get("all", []):
        require(pointer_get(corpus[pre["case_id"]], assertion["pointer"]) == assertion["equals"], "INVALID_PRECONDITION", f"{mutant['id']} all")
    if "envelope.message_template" in pre:
        template = pre["envelope.message_template"]
        for case_id in selected:
            params = mutants["case_parameters"][case_id]
            require(corpus[case_id]["envelope"]["message"] == template.format(**params), "INVALID_PRECONDITION", f"{mutant['id']} template")
    if pre.get("span_is_non_null"):
        require(all(corpus[case_id]["envelope"]["span_bytes"] is not None for case_id in selected), "INVALID_PRECONDITION", f"{mutant['id']} spans")
    if pre.get("whole_access_span_differs_for_kill_cases"):
        require(all(corpus[case_id]["envelope"]["span_bytes"] != mutants["case_parameters"][case_id]["whole_access_span"] for case_id in mutant["kill_cases"]), "INVALID_PRECONDITION", f"{mutant['id']} kill spans")
    if "message_prefix" in pre:
        require(all(corpus[case_id]["envelope"]["message"].startswith(pre["message_prefix"]) for case_id in selected), "INVALID_PRECONDITION", f"{mutant['id']} prefix")
    if pre.get("std_three_field_cases_unchanged"):
        require(all(corpus[case_id]["classification"] == "Preserved" for case_id in mutants["case_groups"]["std_module_missing"]), "INVALID_PRECONDITION", f"{mutant['id']} std controls")
    if "profiles_exactly" in pre:
        require(sorted(set(corpus[case_id]["profile"] for case_id in selected)) == sorted(pre["profiles_exactly"]), "INVALID_PRECONDITION", f"{mutant['id']} profiles")
    if pre.get("corresponding_default_cases_unchanged"):
        require(all(corpus[case_id]["classification"] == "Preserved" for case_id in ("V01-std-hsl-default", "V05-std-hsv-default", "V09-std-linear-rgb-default")), "INVALID_PRECONDITION", f"{mutant['id']} defaults")
    if "orders" in pre:
        require(pre["orders"] == ["normal", "inverted"] and pre.get("reset_state_per_order") is True, "INVALID_PRECONDITION", f"{mutant['id']} orders")
    if pre.get("at_least_two_distinct_non_null_original_spans"):
        spans = {tuple(corpus[case_id]["envelope"]["span_bytes"]) for case_id in selected if corpus[case_id]["envelope"] is not None and corpus[case_id]["envelope"]["span_bytes"] is not None}
        require(len(spans) >= 2, "INVALID_PRECONDITION", f"{mutant['id']} distinct spans")


def apply_mutant(mutant: dict[str, Any], corpus: dict[str, dict[str, Any]], selected: list[str], mutants: dict[str, Any]) -> list[dict[str, Any]]:
    changed: list[dict[str, Any]] = []
    for operation in mutant["transformation"]["operations"]:
        op = operation["op"]
        require(op in IMPLEMENTED_OPERATIONS, "UNSUPPORTED_MUTATION_OPERATION", op)
        if op == "carry_previous_span":
            previous = operation["initial"]
            require(previous is None and operation["update_source"] == "original /envelope/span_bytes" and operation["skip_null_spans"] is True, "INVALID_MUTATION_OPERATION", mutant["id"])
            for case_id in selected:
                record = corpus[case_id]
                if record["envelope"] is None or record["envelope"]["span_bytes"] is None:
                    continue
                original = copy.deepcopy(record["envelope"]["span_bytes"])
                if previous is not None:
                    record["envelope"]["span_bytes"] = copy.deepcopy(previous)
                    if original != previous:
                        changed.append({"case_id": case_id, "pointer": "/envelope/span_bytes", "old": original, "new": copy.deepcopy(previous)})
                previous = original
            continue
        for case_id in selected:
            record = corpus[case_id]
            if op == "promote_unknown":
                old = {"classification": record["classification"], "reason_code": record["reason_code"]}
                require(record["classification"] == "Unknown" and record["envelope"] is None, "INVALID_MUTATION_OPERATION", mutant["id"])
                record["classification"] = operation["new_classification"]
                record["reason_code"] = operation["new_reason_code"]
                record["selected_diagnostic_index"] = operation["selected_diagnostic_index"]
                changed.append({"case_id": case_id, "pointer": "/classification,/reason_code,/selected_diagnostic_index", "old": old, "new": {"classification": record["classification"], "reason_code": record["reason_code"], "selected_diagnostic_index": record["selected_diagnostic_index"]}})
                continue
            pointer = operation["pointer"]
            old = copy.deepcopy(pointer_get(record, pointer))
            if op == "replace":
                require(old == operation["old"], "INVALID_MUTATION_OPERATION", f"{mutant['id']} old")
                new = operation["new"]
            elif op == "replace_template":
                params = mutants["case_parameters"].get(case_id)
                require(params is not None, "INVALID_MUTATION_OPERATION", f"{mutant['id']} params")
                try:
                    new = operation["value"].format(**params)
                except KeyError as exc:
                    raise GateError("INVALID_MUTATION_OPERATION", f"{mutant['id']} template {exc}") from exc
            elif op == "replace_from_case_parameter":
                params = mutants["case_parameters"].get(case_id)
                require(params is not None and operation["parameter"] in params, "INVALID_MUTATION_OPERATION", f"{mutant['id']} parameter")
                new = params[operation["parameter"]]
            elif op == "append":
                require(isinstance(old, list), "INVALID_MUTATION_OPERATION", f"{mutant['id']} append target")
                new = old + [copy.deepcopy(operation["value"])]
            else:
                raise GateError("UNSUPPORTED_MUTATION_OPERATION", op)
            require(old != new, "NO_OP_TRANSFORMATION", f"{mutant['id']} {case_id} {pointer}")
            pointer_set(record, pointer, new)
            changed.append({"case_id": case_id, "pointer": pointer, "old": old, "new": copy.deepcopy(new)})
    require(bool(changed), "NO_OP_TRANSFORMATION", mutant["id"])
    return changed


def validate_non_equivalence(mutant: dict[str, Any], nominal: dict[str, dict[str, Any]], corpus: dict[str, dict[str, Any]]) -> dict[str, Any]:
    validation = mutant["non_equivalence_validation"]
    mode = validation["mode"]
    if mode == "ordered_run_divergence":
        return {"result": "DEFERRED_CROSS_ORDER"}
    for comparison in validation["comparisons"]:
        case_id = comparison["case_id"]
        if mode == "all_listed_cases_differ":
            pointer = comparison["pointer"]
            original = pointer_get(nominal[case_id], pointer)
            observed = pointer_get(corpus[case_id], pointer)
            if "contract_value" in comparison:
                require(original == comparison["contract_value"], "NON_EQUIVALENCE_FAILURE", f"{mutant['id']} contract value")
            require(observed == comparison["mutant_value"] and observed != original, "NON_EQUIVALENCE_FAILURE", f"{mutant['id']} {case_id}")
        elif mode == "opaque_classification_must_be_rejected":
            require(nominal[case_id]["classification"] == comparison["contract_classification"], "NON_EQUIVALENCE_FAILURE", f"{mutant['id']} contract class")
            require(corpus[case_id]["classification"] == comparison["mutant_classification"] and corpus[case_id]["classification"] != nominal[case_id]["classification"], "NON_EQUIVALENCE_FAILURE", f"{mutant['id']} mutant class")
        else:
            raise GateError("UNKNOWN_NON_EQUIVALENCE_MODE", mode)
    for pointer in validation.get("retained_fields", []):
        for case_id in mutant["kill_cases"]:
            require(pointer_get(corpus[case_id], pointer) == pointer_get(nominal[case_id], pointer), "NON_EQUIVALENCE_FAILURE", f"{mutant['id']} retained {pointer}")
    return {"result": "PASS", "mode": mode}


def classify_record(record: dict[str, Any], case: dict[str, Any]) -> tuple[str, list[dict[str, Any]]]:
    witnesses: list[dict[str, Any]] = []
    if case["candidate_mode"] == "bilateral_declared_opaque":
        accepted = record["classification"] == "Unknown" and record["reason_code"] == case["expected"]["reason_code"] and record["envelope"] is None
        if accepted:
            return "Unknown", []
        for field, expected, observed in (
            ("classification", "Unknown", record["classification"]),
            ("reason_code", case["expected"]["reason_code"], record["reason_code"]),
        ):
            if expected != observed:
                witnesses.append({"case_id": case["id"], "field": field, "expected": expected, "observed": observed})
        return "Violated", witnesses
    if record["classification"] == "Unknown" or record["envelope"] is None:
        return "FAIL", [{"case_id": case["id"], "field": "classification", "expected": "Preserved or Violated with envelope", "observed": record["classification"]}]
    for field in FIELDS:
        expected = case["expected"][field]
        observed = record["envelope"][field]
        if expected != observed:
            witnesses.append({"case_id": case["id"], "field": field, "expected": expected, "observed": observed})
    return ("Preserved" if not witnesses else "Violated"), witnesses


def classify_corpus(corpus: dict[str, dict[str, Any]], ids: list[str], cases: dict[str, dict[str, Any]]) -> tuple[dict[str, str], list[dict[str, Any]], dict[str, int]]:
    classifications: dict[str, str] = {}
    witnesses: list[dict[str, Any]] = []
    counts = {"Preserved": 0, "Violated": 0, "Unknown": 0, "FAIL": 0}
    for case_id in ids:
        result, case_witnesses = classify_record(corpus[case_id], cases[case_id])
        classifications[case_id] = result
        counts[result] += 1
        witnesses.extend(case_witnesses)
    return classifications, witnesses, counts


def run_discrimination(raw: dict[str, bytes], data: dict[str, Any]) -> dict[str, Any]:
    started = time.monotonic()
    gates = preflight(raw, data)
    contract = data["p1301r2-contract.json"]
    mutants_doc = data["p1301r2-mutants.json"]
    plan = data["p1301r2-mutation-plan.json"]
    cases = case_index(contract)
    nominal = nominal_corpus(contract)
    runs: list[dict[str, Any]] = []
    input_hashes = copy.deepcopy(INPUT_HASHES)

    baseline_hashes: dict[str, list[str]] = {"normal": [], "inverted": []}
    baseline_counts: dict[str, list[dict[str, int]]] = {"normal": [], "inverted": []}
    for order_name in plan["run_matrix"]["orders"]:
        order = contract["execution_orders"][order_name]
        ids = executable_ids(order, cases)
        for repetition in range(1, plan["run_matrix"]["repetitions_per_order"] + 1):
            corpus = copy.deepcopy(nominal)
            classifications, witnesses, counts = classify_corpus(corpus, ids, cases)
            vector = vector_for(corpus, ids)
            digest = sha256_bytes(canonical_bytes(vector))
            result = "PASS" if counts == {"Preserved": 20, "Violated": 0, "Unknown": 2, "FAIL": 0} and not witnesses else "FAIL"
            baseline_hashes[order_name].append(digest)
            baseline_counts[order_name].append(counts)
            runs.append({
                "run_kind": "baseline",
                "order": order_name,
                "repetition": repetition,
                "input_artifact_hashes": input_hashes,
                "precondition": "PASS",
                "changed_cases_and_pointers": [],
                "non_equivalence": "NOT_APPLICABLE",
                "case_classifications": classifications,
                "public_witnesses": witnesses,
                "canonical_vector_sha256": digest,
                "result": result,
            })
    baseline_repetition_equal = all(len(set(values)) == 1 for values in baseline_hashes.values())
    baseline_order_equal = baseline_hashes["normal"][0] == baseline_hashes["inverted"][0]
    require(baseline_repetition_equal and baseline_order_equal, "BASELINE_ORDER_OR_REPETITION_FAILURE", repr(baseline_hashes))
    require(all(run["result"] == "PASS" for run in runs), "BASELINE_REGRESSION", "baseline controls")

    by_id = {mutant["id"]: mutant for mutant in mutants_doc["mutants"]}
    mutant_run_indexes: dict[str, dict[str, list[int]]] = {}
    mutant_validity: dict[str, bool] = {}
    for mutant_id in plan["run_matrix"]["mutant_ids"]:
        mutant = by_id[mutant_id]
        mutant_run_indexes[mutant_id] = {"normal": [], "inverted": []}
        mutant_validity[mutant_id] = True
        for order_name in plan["run_matrix"]["orders"]:
            order = contract["execution_orders"][order_name]
            ids = executable_ids(order, cases)
            for repetition in range(1, plan["run_matrix"]["repetitions_per_order"] + 1):
                corpus = copy.deepcopy(nominal)
                selected = selected_ids(mutant, mutants_doc, order, cases)
                verify_precondition(mutant, corpus, selected, mutants_doc)
                changed = apply_mutant(mutant, corpus, selected, mutants_doc)
                non_equivalence = validate_non_equivalence(mutant, nominal, corpus)
                classifications, witnesses, counts = classify_corpus(corpus, ids, cases)
                vector = vector_for(corpus, ids)
                digest = sha256_bytes(canonical_bytes(vector))
                if mutant_id == "M12-order-dependent-state-leak":
                    result = "PENDING_CROSS_ORDER"
                else:
                    killed = all(classifications.get(case_id) == "Violated" for case_id in mutant["kill_cases"])
                    result = "KILLED" if killed and non_equivalence["result"] == "PASS" else "SURVIVED"
                run = {
                    "run_kind": "mutant",
                    "mutant_id": mutant_id,
                    "order": order_name,
                    "repetition": repetition,
                    "input_artifact_hashes": input_hashes,
                    "precondition": "PASS",
                    "changed_cases_and_pointers": changed,
                    "non_equivalence": non_equivalence,
                    "case_classifications": classifications,
                    "classification_counts": counts,
                    "public_witnesses": witnesses,
                    "canonical_vector_sha256": digest,
                    "result": result,
                }
                runs.append(run)
                index = len(runs) - 1
                mutant_run_indexes[mutant_id][order_name].append(index)

    mutant_results: list[dict[str, Any]] = []
    for mutant_id in plan["run_matrix"]["mutant_ids"]:
        mutant = by_id[mutant_id]
        indexes = mutant_run_indexes[mutant_id]
        normal_runs = [runs[index] for index in indexes["normal"]]
        inverted_runs = [runs[index] for index in indexes["inverted"]]
        repetition_equal = all(group[0]["canonical_vector_sha256"] == group[1]["canonical_vector_sha256"] for group in (normal_runs, inverted_runs))
        require(repetition_equal, "MUTANT_NONDETERMINISM", mutant_id)
        cross_order_equal = normal_runs[0]["canonical_vector_sha256"] == inverted_runs[0]["canonical_vector_sha256"]
        cross_witnesses: list[dict[str, Any]] = []
        if mutant_id == "M12-order-dependent-state-leak":
            require(not cross_order_equal, "ORDER_DEPENDENCE_MUTANT_SURVIVED", mutant_id)
            for boundary in mutant["non_equivalence_validation"]["boundary_witnesses"]:
                relevant = normal_runs[0] if boundary["order"] == "normal" else inverted_runs[0]
                changed_match = [change for change in relevant["changed_cases_and_pointers"] if change["case_id"] == boundary["case_id"] and change["pointer"] == "/envelope/span_bytes"]
                require(len(changed_match) == 1 and changed_match[0]["old"] == boundary["contract_span"] and changed_match[0]["new"] == boundary["mutant_span"], "ORDER_DEPENDENCE_WITNESS_MISMATCH", repr(boundary))
                cross_witnesses.append({**boundary, "observed": changed_match[0]})
            for index in indexes["normal"] + indexes["inverted"]:
                runs[index]["non_equivalence"] = {"result": "PASS", "mode": "ordered_run_divergence", "normal_sha256": normal_runs[0]["canonical_vector_sha256"], "inverted_sha256": inverted_runs[0]["canonical_vector_sha256"]}
                runs[index]["result"] = "KILLED"
            result = "KILLED"
        else:
            require(cross_order_equal, "UNEXPECTED_MUTANT_ORDER_DEPENDENCE", mutant_id)
            result = "KILLED" if all(run["result"] == "KILLED" for run in normal_runs + inverted_runs) else "SURVIVED"
        mutant_results.append({
            "mutant_id": mutant_id,
            "axis": mutant["axis"],
            "valid": mutant_validity[mutant_id],
            "result": result,
            "repetition_equal": repetition_equal,
            "cross_order_equal": cross_order_equal,
            "cross_order_witnesses": cross_witnesses,
            "run_count": 4,
        })

    valid = sum(1 for result in mutant_results if result["valid"])
    killed = sum(1 for result in mutant_results if result["valid"] and result["result"] == "KILLED")
    survivors = [result["mutant_id"] for result in mutant_results if result["result"] == "SURVIVED"]
    invalid = [result["mutant_id"] for result in mutant_results if not result["valid"]]
    score = killed / valid if valid else 0.0
    require(len(runs) == 52, "RUN_COUNT_MISMATCH", str(len(runs)))
    pass_gate = valid == 12 and killed == 12 and score == 1.0 and not survivors and not invalid
    require(pass_gate, "DISCRIMINATION_GATE_FAILURE", f"valid={valid}, killed={killed}, score={score}")
    elapsed_ms = round((time.monotonic() - started) * 1000, 3)
    return {
        "schema_version": "tekt-discrimination-receipt-v2",
        "step": "P1301",
        "revision": 2,
        "status": "PASS",
        "verdict": plan["gate"]["verdict_if_all_pass"],
        "attestation_language": ATTESTATION,
        "scope_limit": plan["receipt_requirements"]["seal_language"],
        "inputs": input_hashes,
        "runner": {"path": "00_nucleo/diagnosticos/p1301r2-contract-runner.py", "sha256": sha256_file(pathlib.Path(__file__).resolve())},
        "preflight": gates,
        "self_check": self_check(raw, data, include_preflight=False),
        "run_matrix": {
            "baseline_control_runs": 4,
            "mutant_runs": 48,
            "total_actual_runs": len(runs),
            "mutants_executed_once_per_matrix_slot": True,
            "orders": ["normal", "inverted"],
            "repetitions_per_order": 2,
        },
        "baseline": {
            "counts_per_run": baseline_counts,
            "positive_control_vector_per_order": {"normal": 20, "inverted": 20},
            "opaque_control_vector_per_order": {"normal": 2, "inverted": 2},
            "crystalline_sentinel": {"case_id": "C23-dictionary-crystalline-span-sentinel", "status": "PRESERVED_SEPARATE_CONTRACT_CONTROL", "expected_span_bytes": [10, 21]},
            "normal_inverted_identity": baseline_order_equal,
            "repetition_identity": baseline_repetition_equal,
            "normal_sha256": baseline_hashes["normal"][0],
            "inverted_sha256": baseline_hashes["inverted"][0],
        },
        "mutation": {
            "valid_mutants": valid,
            "killed_mutants": killed,
            "survivors": survivors,
            "invalid_mutants": invalid,
            "mutation_score": score,
            "operation_sets": {
                "used": sorted(IMPLEMENTED_OPERATIONS),
                "declared": sorted(mutants_doc["application_model"]["operation_semantics"]),
                "implemented": sorted(IMPLEMENTED_OPERATIONS),
                "unsupported_count": 0,
            },
            "results": mutant_results,
        },
        "runs": runs,
        "cost": {"elapsed_milliseconds": elapsed_ms, "process_executions": 0, "contract_record_executions": len(runs)},
        "provenance": {
            "executed_at": dt.datetime.now().astimezone().isoformat(),
            "repository_head": data["p1301r2-manifest.json"]["repository"]["head"],
            "working_tree": data["p1301r2-manifest.json"]["repository"]["working_tree"],
            "repository_state_source": "copied from frozen manifest; no additional repository state was read",
            "allowed_repository_reads": sorted(INPUT_HASHES),
            "forbidden_context_not_read": ["candidate implementation", "candidate Rust tests", "productive code", "P1301 revision-1 artifacts", "00_nucleo/materialization/", "00_nucleo/context/"],
            "candidate_built_or_read": False,
        },
    }


def self_check(raw: dict[str, bytes], data: dict[str, Any], include_preflight: bool = True) -> dict[str, Any]:
    contract = data["p1301r2-contract.json"]
    oracle = data["p1301r2-oracle-suite.json"]
    parser = audit_oracle(contract, oracle)
    normal_case = case_index(contract)["V01-std-hsl-default"]
    base = build_argv("/candidate", normal_case, contract)
    rejected: list[str] = []
    for forbidden in sorted(FORBIDDEN_ARGUMENTS):
        try:
            validate_argv(base + [forbidden, "x"], "/candidate", normal_case["expression"], [])
        except GateError as exc:
            require(exc.reason_code == "INVALID_CONTRACT_EXECUTION", "SELF_CHECK_FAILURE", forbidden)
            rejected.append(forbidden)
        else:
            raise GateError("SELF_CHECK_FAILURE", f"accepted {forbidden}")
    result: dict[str, Any] = {
        "status": "PASS",
        "input_hash_count": 7,
        "parser": parser,
        "forbidden_arguments_rejected": rejected,
        "semantic_translation": "none",
    }
    if include_preflight:
        result["preflight"] = preflight(raw, data)
    return result


def run_candidate(binary: pathlib.Path, order_name: str, raw: dict[str, bytes], data: dict[str, Any]) -> dict[str, Any]:
    preflight(raw, data)
    contract = data["p1301r2-contract.json"]
    cases = case_index(contract)
    require(binary.is_file(), "CANDIDATE_BINARY_NOT_FOUND", str(binary))
    binary_hash = sha256_file(binary)
    ids = executable_ids(contract["execution_orders"][order_name], cases)
    require(len(ids) == 22 and "C23-dictionary-crystalline-span-sentinel" in ids and "V20-dictionary-vanilla-reference" not in ids, "CANDIDATE_CASE_PARTITION_MISMATCH", repr(ids))
    records: list[dict[str, Any]] = []
    classifications: dict[str, str] = {}
    witnesses: list[dict[str, Any]] = []
    counts = {"Preserved": 0, "Violated": 0, "Unknown": 0, "FAIL": 0}
    for case_id in ids:
        case = cases[case_id]
        argv = build_argv(str(binary), case, contract)
        try:
            process = subprocess.run(argv, stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=False)
            parsed = parse_human(case["expression"], process.returncode, process.stdout, process.stderr)
            if parsed["kind"] == "Envelope":
                record = {"case_id": case_id, "profile": case["profile"], "classification": "Preserved", "reason_code": None, "envelope": parsed["envelope"]}
            else:
                record = {"case_id": case_id, "profile": case["profile"], "classification": "Unknown", "reason_code": parsed["reason_code"], "envelope": None}
            result, case_witnesses = classify_record(record, case)
            raw_stderr: str | None
            raw_stderr_base64: str | None
            try:
                raw_stderr = process.stderr.decode("utf-8")
                raw_stderr_base64 = None
            except UnicodeDecodeError:
                raw_stderr = None
                raw_stderr_base64 = base64.b64encode(process.stderr).decode("ascii")
            records.append({
                "case_id": case_id,
                "candidate_mode": case["candidate_mode"],
                "argv": argv,
                "exit_code_raw": process.returncode,
                "stdout_utf8": process.stdout.decode("utf-8", errors="strict") if parsed.get("reason_code") != "NON_UTF8_PROCESS_STREAM" else None,
                "raw_stderr": raw_stderr,
                "raw_stderr_base64_if_non_utf8": raw_stderr_base64,
                "raw_stderr_sha256": sha256_bytes(process.stderr),
                "adapter_result": parsed,
                "contract_classification": result,
                "public_witnesses": case_witnesses,
            })
            classifications[case_id] = result
            counts[result] += 1
            witnesses.extend(case_witnesses)
        except OSError as exc:
            records.append({"case_id": case_id, "argv": argv, "execution_error": str(exc), "contract_classification": "FAIL"})
            classifications[case_id] = "FAIL"
            counts["FAIL"] += 1
            witnesses.append({"case_id": case_id, "field": "execution", "expected": "successful process execution", "observed": str(exc)})
    sentinel = next(record for record in records if record["case_id"] == "C23-dictionary-crystalline-span-sentinel")
    bilateral_ids = [case_id for case_id in ids if cases[case_id]["candidate_mode"] == "bilateral_exact"]
    opaque_ids = [case_id for case_id in ids if cases[case_id]["candidate_mode"] == "bilateral_declared_opaque"]
    passed = all(classifications[case_id] == "Preserved" for case_id in bilateral_ids) and all(classifications[case_id] == "Unknown" for case_id in opaque_ids) and classifications[sentinel["case_id"]] == "Preserved"
    return {
        "schema_version": "tekt-candidate-contract-execution-v2",
        "step": "P1301",
        "revision": 2,
        "status": "PASS" if passed else "FAIL",
        "attestation_language": ATTESTATION,
        "candidate_binary": {"path": str(binary), "sha256": binary_hash},
        "order": order_name,
        "common_cli_only": True,
        "forbidden_arguments_observed": [],
        "semantic_translation": "none; syntactic adapter only",
        "candidate_case_count": len(ids),
        "bilateral_exact_count": len(bilateral_ids),
        "opaque_count": len(opaque_ids),
        "sentinel": {"case_id": sentinel["case_id"], "classification": sentinel["contract_classification"], "separate_contract_control": True},
        "counts": counts,
        "classifications": classifications,
        "public_witnesses": witnesses,
        "runs": records,
        "inputs": copy.deepcopy(INPUT_HASHES),
        "runner_sha256": sha256_file(pathlib.Path(__file__).resolve()),
    }


def write_result(result: dict[str, Any], output: pathlib.Path | None) -> None:
    rendered = json.dumps(result, ensure_ascii=False, indent=2) + "\n"
    if output is None:
        sys.stdout.write(rendered)
        return
    protected = {(HERE / name).resolve() for name in INPUT_HASHES}
    protected.add(pathlib.Path(__file__).resolve())
    require(output.resolve() not in protected, "PROTECTED_OUTPUT_PATH", str(output))
    output.write_text(rendered, encoding="utf-8")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="mode", required=True)
    check = sub.add_parser("self-check", help="audit inputs, argv guard, and oracle parser samples")
    check.add_argument("--output", type=pathlib.Path)
    discriminate = sub.add_parser("discriminate", help="execute the frozen 52-run mutation matrix")
    discriminate.add_argument("--output", type=pathlib.Path)
    candidate = sub.add_parser("candidate", help="run a candidate through the frozen common CLI contract")
    candidate.add_argument("--binary", required=True, type=pathlib.Path)
    candidate.add_argument("--order", choices=("normal", "inverted"), default="normal")
    candidate.add_argument("--output", type=pathlib.Path)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        raw, data = load_inputs()
        if args.mode == "self-check":
            result = self_check(raw, data)
        elif args.mode == "discriminate":
            result = run_discrimination(raw, data)
        else:
            result = run_candidate(args.binary.resolve(), args.order, raw, data)
        write_result(result, args.output)
        return 0 if result.get("status") == "PASS" else 1
    except GateError as exc:
        failure = {
            "schema_version": "tekt-runner-failure-v2",
            "step": "P1301",
            "revision": 2,
            "status": "FAIL",
            "verdict": "DO_NOT_SEAL",
            "reason_code": exc.reason_code,
            "detail": exc.detail,
            "attestation_language": ATTESTATION,
        }
        output = getattr(args, "output", None)
        write_result(failure, output)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
