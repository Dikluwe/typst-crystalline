#!/usr/bin/env python3
"""P1350 oracle: mandatory supervised boundary and causal-journal checks."""

from __future__ import annotations

import hashlib
import importlib.util
import json
import os
import secrets
import sys
import tempfile
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
CHECKER_REL = "00_nucleo/diagnosticos/p1350-oracle-checker-r1.py"
RECEIPT_REL = "00_nucleo/diagnosticos/p1350-oracle-authorship-receipt-r1.json"
MANIFEST_REL = "00_nucleo/diagnosticos/p1350-authority-manifest-r1.json"
CORPUS_REL = "00_nucleo/diagnosticos/p1350-oracle-corpus-r1.json"
SUPERVISOR_REL = "00_nucleo/diagnosticos/p1350-oracle-supervisor-r1.py"
PARSER_REL = "00_nucleo/diagnosticos/p1350-oracle-journal-parser-r1.py"
SELF_ADAPTER_REL = "00_nucleo/diagnosticos/p1350-oracle-selftest-adapter-r1.py"
SELF_MAP_REL = "00_nucleo/diagnosticos/p1350-oracle-selftest-route-map-r1.json"
CONTRACT_ROOT_SHA256 = "f1d0b459dbc673afbd761630b906f7444bb6f8816a3fd935d13297c8e5c3d718"
EXPECTED_MANIFEST_SHA256 = "06075bdc5d46ce45c3848bc0929a33288e43650ffda7e2fddf36eae91d3522ea"
EXPECTED_CORPUS_SHA256 = "09132f2c7cd8c0deff6dfab1a7f439a2a1cec92a771434a2b25e7e60b7514053"
EXPECTED_SUPERVISOR_SHA256 = "5efd5ada7d675bd97ed091aff15128aacfb50b97c1733a85a89578f1359ce4e4"
EXPECTED_PARSER_SHA256 = "9bed8434754f7f002924307fcddcf5baa5e751536a7356a4eb9b3cbb8ec93413"
EXPECTED_SELF_ADAPTER_SHA256 = "86da667a2e8cbcd85cd9613e2ba209a433a7cebcfdc8788aec532dab2662593f"
EXPECTED_SELF_MAP_SHA256 = "9357890b61e34d2489639181a7ec2702419dcb2174e27605ace6530e9f1af10e"
PINS = {
    "00_nucleo/materialization/typst-passo-1350.md": "e09cf72e1e81c414343f8ec400e8d1e242910ed9a8a56ed7aaa96308a36d9ebf",
    "00_nucleo/diagnosticos/p1350-contract-spec-r1.json": "bdc3f3c0a6ef7baec3b2cb2e31c88c099dc5a49b18e97ae0a0fcc86535286e82",
    "00_nucleo/diagnosticos/p1350-contract-binding-r1.json": "257114e18af3ef0d968294e5e9b8326bd963eb7fa92a1d2c489c6e05f388fcfb",
    "00_nucleo/diagnosticos/p1350-contract-receipt-r1.json": "e741751e0641c46be2e8aa634cc96b0ee400cd3b6d5693b25eec66cba283008d",
    "00_nucleo/diagnosticos/p1349-verifier-blocker-r2.json": "16780a65c46ba958b6e1535184ea039d022f558a01fb384cc0c110c261d55417",
    "00_nucleo/diagnosticos/p1349-authority-manifest-r1.json": "1d2b30fd83d78d1fada381f82062309cbe5eaba2a4d32665fa59872dd9c3c907",
    "00_nucleo/diagnosticos/p1349-oracle-corpus-r1.json": "045af76f9540107bb633c8d2c904e726f96624ecf8e9a91d706b99f530c56803",
    "00_nucleo/diagnosticos/p1349-oracle-checker-r2.py": "999971b4e25c6a44ae76542d0115d8304c3a8f28afbaf26bc1c9347db6edc267",
    "00_nucleo/diagnosticos/p1349-oracle-authorship-receipt-r2.json": "d95113d55015300108e5571fe29d6b83b17a34bc37aa7dfa01be04e934fb6ab8",
    "00_nucleo/diagnosticos/p1349-oracle-caller-r2.py": "4445f1a23a66ad2a0c19a67769cfb043541106bb81ded6e1a1e263d0da974c07",
    "00_nucleo/diagnosticos/p1349-oracle-delivery-receipt-r2.json": "219f6c23c2ba95592536fb5ab174a0d8f5243c76fab35a4057042f63234a4524",
    MANIFEST_REL: EXPECTED_MANIFEST_SHA256,
    CORPUS_REL: EXPECTED_CORPUS_SHA256,
    SUPERVISOR_REL: EXPECTED_SUPERVISOR_SHA256,
    PARSER_REL: EXPECTED_PARSER_SHA256,
    SELF_ADAPTER_REL: EXPECTED_SELF_ADAPTER_SHA256,
    SELF_MAP_REL: EXPECTED_SELF_MAP_SHA256,
}
CLASSIFICATION_KEYS = ["case_id", "classification", "closed_world", "evidence", "reason_code", "schema"]
BLOCKER_KEYS = ["blocker_code", "case_id", "cleanup", "closed_world", "deadline", "exception", "phase", "schema", "supervisor", "verdict"]
REQUEST_KEYS = ["case_id", "exact_ids_sha256", "operation", "run_id", "schema", "sequence_index", "targets_sha256"]
_BOUND_TOKEN = object()


class OracleFailure(RuntimeError):
    def __init__(self, code: str, detail: str):
        self.code = code
        self.detail = detail
        super().__init__(f"{code}: {detail}")


def sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def canonical(value: Any, lf: bool = True) -> bytes:
    raw = json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
    return raw + (b"\n" if lf else b"")


def read_pinned(relative: str, digest: str) -> bytes:
    path = ROOT / relative
    if path.is_symlink() or not path.is_file():
        raise OracleFailure("PROTECTED_INPUT", relative)
    raw = path.read_bytes()
    if sha256(raw) != digest:
        raise OracleFailure("PROTECTED_INPUT", relative)
    return raw


def _load(relative: str, digest: str, name: str) -> Any:
    read_pinned(relative, digest)
    spec = importlib.util.spec_from_file_location(name, ROOT / relative)
    if spec is None or spec.loader is None:
        raise OracleFailure("AUTHORITY_ROOT", f"loader {relative}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


SUPERVISOR = _load(SUPERVISOR_REL, EXPECTED_SUPERVISOR_SHA256, "p1350_supervisor_pinned")
PARSER = _load(PARSER_REL, EXPECTED_PARSER_SHA256, "p1350_journal_parser_pinned")


def validate_protected_inputs() -> None:
    for relative, digest in PINS.items():
        read_pinned(relative, digest)


def _strict_file(relative: str, digest: str, schema: str) -> dict[str, Any]:
    raw = read_pinned(relative, digest)
    try:
        value = json.loads(raw)
    except Exception:
        raise OracleFailure("PROTECTED_INPUT", f"JSON {relative}") from None
    if canonical(value) != raw or type(value) is not dict or value.get("schema") != schema:
        raise OracleFailure("PROTECTED_INPUT", f"canonical {relative}")
    return value


def load_corpus() -> dict[str, Any]:
    value = _strict_file(CORPUS_REL, EXPECTED_CORPUS_SHA256, "p1350-oracle-corpus-r1")
    if value.get("manifest_sha256") != EXPECTED_MANIFEST_SHA256 or value.get("coverage") != {"declared": 60, "direct_inherited_edges": 0, "duplicate": 0, "inherited": 48, "local": 12, "supervised": 60, "unbound": 0}:
        raise OracleFailure("PROTECTED_INPUT", "corpus coverage")
    return value


def oracle_authoring_root(checker_sha256: str) -> str:
    return sha256(canonical(["p1350-oracle-authoring-root-r1", CONTRACT_ROOT_SHA256, EXPECTED_MANIFEST_SHA256, EXPECTED_CORPUS_SHA256, EXPECTED_SUPERVISOR_SHA256, EXPECTED_PARSER_SHA256, EXPECTED_SELF_ADAPTER_SHA256, EXPECTED_SELF_MAP_SHA256, checker_sha256], False))


def oracle_delivery_root(authoring_root_sha256: str, checker_sha256: str, receipt_sha256: str, caller_sha256: str) -> str:
    return sha256(canonical(["p1350-oracle-delivery-root-r1", authoring_root_sha256, checker_sha256, EXPECTED_SUPERVISOR_SHA256, EXPECTED_PARSER_SHA256, receipt_sha256, caller_sha256], False))


def validate_authorship_receipt(receipt_sha256: str, checker_sha256: str, root_sha256: str) -> dict[str, Any]:
    raw = read_pinned(RECEIPT_REL, receipt_sha256)
    try: receipt = json.loads(raw)
    except Exception: raise OracleFailure("AUTHORITY_ROOT", "authorship JSON") from None
    if canonical(receipt) != raw or receipt.get("schema") != "p1350-oracle-authorship-receipt-r1" or receipt.get("authoring_root_sha256") != root_sha256 or oracle_authoring_root(checker_sha256) != root_sha256:
        raise OracleFailure("AUTHORITY_ROOT", "authorship root")
    expected = [[MANIFEST_REL, EXPECTED_MANIFEST_SHA256], [CORPUS_REL, EXPECTED_CORPUS_SHA256], [SUPERVISOR_REL, EXPECTED_SUPERVISOR_SHA256], [PARSER_REL, EXPECTED_PARSER_SHA256], [SELF_ADAPTER_REL, EXPECTED_SELF_ADAPTER_SHA256], [SELF_MAP_REL, EXPECTED_SELF_MAP_SHA256], [CHECKER_REL, checker_sha256]]
    if receipt.get("outputs") != expected or any(key in receipt for key in ("caller", "caller_sha256", "delivery_receipt", "delivery_receipt_sha256", "self_sha256")):
        raise OracleFailure("AUTHORITY_ROOT", "authorship DAG")
    return receipt


@dataclass
class RunContext:
    token: object
    journal: Any
    plan: Any
    exact_ids: list[str]
    targets_sha256: str


def _plan_digest(plan: Any) -> str:
    return sha256(canonical({"adapter_path": plan.adapter_path, "adapter_sha256": plan.adapter_sha256, "authority_root_sha256": plan.authority_root_sha256, "route_map_path": plan.route_map_path, "route_map_sha256": plan.route_map_sha256}, False))


def bind_verified_run(delivery_path: str, delivery_sha256: str, authority_root_sha256: str, journal_fd: int, run_id: str, expected_cases: int, exact_ids: list[str]) -> RunContext:
    """Bind a future externally anchored adapter delivery; no callable crosses this API."""
    validate_protected_inputs(); load_corpus()
    raw = read_pinned(delivery_path, delivery_sha256)
    try: delivery = json.loads(raw)
    except Exception: raise OracleFailure("AUTHORITY_ROOT", "execution delivery JSON") from None
    keys = ["adapter", "adversary", "authority_root_sha256", "checker", "journal_parser", "route_map", "schema", "supervisor"]
    if canonical(delivery) != raw or type(delivery) is not dict or list(delivery) != keys or delivery["schema"] != "p1350-execution-delivery-receipt-r1" or delivery["authority_root_sha256"] != authority_root_sha256:
        raise OracleFailure("AUTHORITY_ROOT", "execution delivery")
    if delivery["checker"] != [CHECKER_REL, sha256((ROOT / CHECKER_REL).read_bytes())] or delivery["supervisor"] != [SUPERVISOR_REL, EXPECTED_SUPERVISOR_SHA256] or delivery["journal_parser"] != [PARSER_REL, EXPECTED_PARSER_SHA256]:
        raise OracleFailure("AUTHORITY_ROOT", "oracle pins in execution delivery")
    plan = SUPERVISOR.AdapterPlan(delivery["adapter"][0], delivery["adapter"][1], delivery["route_map"][0], delivery["route_map"][1], authority_root_sha256)
    SUPERVISOR.validate_route_map(ROOT, plan, exact_ids)
    journal = SUPERVISOR.RunJournal(journal_fd, run_id, expected_cases)
    return RunContext(_BOUND_TOKEN, journal, plan, list(exact_ids), _plan_digest(plan))


def _validate_result(value: Any, case_id: str) -> dict[str, Any]:
    if type(value) is not dict:
        raise OracleFailure("FRAME_AUTHORITY", "result object")
    if value.get("schema") == "p1349-classification-result-r1":
        if list(value) != CLASSIFICATION_KEYS or value["case_id"] != case_id or value["classification"] not in ("Preserved", "Violated", "Unknown") or type(value["evidence"]) is not dict or type(value["reason_code"]) is not str:
            raise OracleFailure("FRAME_AUTHORITY", "classification schema")
        if value["classification"] == "Unknown" and value["reason_code"] != "OPAQUE_PAYLOAD":
            raise OracleFailure("FRAME_AUTHORITY", "Unknown allowlist")
        return value
    if value.get("schema") == "p1349-integration-blocker-r1" and list(value) == BLOCKER_KEYS and value["case_id"] == case_id and value["verdict"] == "INTEGRATION_BLOCKED" and value["blocker_code"] in SUPERVISOR.BLOCKER_CODES:
        return value
    raise OracleFailure("FRAME_AUTHORITY", "result domain")


def exercise_focal(case: Any, targets: Any, exact_ids: Any) -> dict[str, Any]:
    """Exactly one public focal execution entry; every route reaches supervise_case."""
    if not isinstance(targets, RunContext) or targets.token is not _BOUND_TOKEN or exact_ids != targets.exact_ids:
        raise OracleFailure("AUTHORITY_ROOT", "unbound focal context")
    if type(case) is not dict or list(case) != REQUEST_KEYS:
        raise OracleFailure("SUPERVISOR_AUTHORITY", "focal request keys")
    expected_ids = sha256(canonical(exact_ids, False))
    if case["schema"] != "p1350-focal-request-r1" or case["run_id"] != targets.journal.run_id or case["exact_ids_sha256"] != expected_ids or case["targets_sha256"] != targets.targets_sha256 or case["case_id"] not in exact_ids or case["sequence_index"] != targets.journal.next_order or type(case["operation"]) is not str:
        raise OracleFailure("SUPERVISOR_AUTHORITY", "focal request binding")
    internal = {"case_id": case["case_id"], "domain": "classification", "operation": case["operation"], "sequence_index": case["sequence_index"]}
    try:
        value, parent = SUPERVISOR._supervise_case(ROOT, internal, targets.plan, targets.journal)
    except SUPERVISOR.SupervisorFailure as exc:
        raise OracleFailure(exc.code, exc.detail) from None
    value = _validate_result(value, case["case_id"])
    if value["schema"] == "p1349-classification-result-r1":
        value = dict(value); value["evidence"] = dict(value["evidence"], parent_supervisor=parent)
    else:
        value = dict(value); value["cleanup"] = parent["cleanup"]; value["deadline"] = parent["deadline"]; value["supervisor"] = parent["supervisor"]
    return _validate_result(value, case["case_id"])


def finish_run(targets: RunContext) -> str:
    if not isinstance(targets, RunContext) or targets.token is not _BOUND_TOKEN:
        raise OracleFailure("AUTHORITY_ROOT", "unbound run")
    return targets.journal.finish()


def _self_context(fd: int, run_id: str, exact_ids: list[str]) -> RunContext:
    plan = SUPERVISOR.AdapterPlan(SELF_ADAPTER_REL, EXPECTED_SELF_ADAPTER_SHA256, SELF_MAP_REL, EXPECTED_SELF_MAP_SHA256, oracle_authoring_root(sha256((ROOT / CHECKER_REL).read_bytes())))
    SUPERVISOR.validate_route_map(ROOT, plan, exact_ids)
    return RunContext(_BOUND_TOKEN, SUPERVISOR.RunJournal(fd, run_id, 2), plan, exact_ids, _plan_digest(plan))


def _request(context: RunContext, case_id: str, operation: str) -> dict[str, Any]:
    return {"case_id": case_id, "exact_ids_sha256": sha256(canonical(context.exact_ids, False)), "operation": operation, "run_id": context.journal.run_id, "schema": "p1350-focal-request-r1", "sequence_index": context.journal.next_order, "targets_sha256": context.targets_sha256}


def nonptrace_focal() -> dict[str, Any]:
    validate_protected_inputs(); load_corpus(); started = time.monotonic_ns()
    ids = [f"P1350-SELF-{ordinal:03d}" for ordinal in range(1, 61)]
    run_id = secrets.token_hex(32)
    fd, path = tempfile.mkstemp(prefix="p1350-oracle-journal-", suffix=".jsonl", dir="/dev/shm")
    os.close(fd)
    fd = os.open(path, os.O_RDWR | os.O_APPEND | os.O_CLOEXEC)
    records: list[dict[str, Any]] = []
    try:
        context = _self_context(fd, run_id, ids)
        preserved = exercise_focal(_request(context, ids[0], "canonical"), context, ids)
        records.append({"case_id": ids[0], "outcome": preserved["classification"], "reason": preserved["reason_code"]})
        blocked = exercise_focal(_request(context, ids[1], "raise_exception"), context, ids)
        records.append({"case_id": ids[1], "outcome": blocked["verdict"], "reason": blocked["blocker_code"]})
        final_line = finish_run(context)
        os.fsync(fd); os.close(fd); fd = -1
        raw = Path(path).read_bytes()
        parsed = PARSER.parse_journal_bytes(raw, os.getpid(), run_id, 2)
        syntax = {"coverage_60_of_60": True, "direct_inherited_edges": 0, "journal_final_line_matches": parsed["final_line_sha256"] == final_line, "journal_lines": parsed["line_count"], "process_groups_absent": parsed["process_groups_absent"]}
        passed = preserved["classification"] == "Preserved" and preserved["reason_code"] == "PRESERVED" and blocked["blocker_code"] == "ADAPTER_EXCEPTION" and syntax["journal_final_line_matches"] and parsed["process_groups_absent"] == 2
        return {"closed_world": {"rule": "Oracle-owned non-ptrace controls only; adversary, candidate, productive full and ptrace are not executed."}, "cost": {"elapsed_ns": time.monotonic_ns() - started, "supervised_case_runs": 2}, "execution": {"adversary_runs": 0, "candidate_runs": 0, "full_corpus_runs": 0, "ptrace_runs": 0}, "records": records, "regime": "executado sem atestacao de isolamento", "revision": 1, "schema": "p1350-oracle-nonptrace-focal-r1", "step": 1350, "summary": syntax, "verdict": "ORACLE_R1_NONPTRACE_FOCAL_NOT_SEALED" if passed else "ORACLE_R1_BLOCKED_NOT_SEALED"}
    finally:
        if fd >= 0:
            try: os.close(fd)
            except OSError: pass
        try: os.unlink(path)
        except OSError: pass


def run_focal(checker_sha256: str, receipt_sha256: str, root_sha256: str) -> dict[str, Any]:
    validate_protected_inputs(); load_corpus(); validate_authorship_receipt(receipt_sha256, checker_sha256, root_sha256)
    return nonptrace_focal()


def main() -> int:
    if sys.argv[1:] != ["--self-focal"]:
        sys.stderr.write("AUTHORITY_ROOT: P1350 checker accepts exactly --self-focal\n"); return 2
    try:
        report = nonptrace_focal(); sys.stdout.buffer.write(canonical(report)); return 0 if report["verdict"] == "ORACLE_R1_NONPTRACE_FOCAL_NOT_SEALED" else 1
    except Exception as exc:
        code = getattr(exc, "code", "AUTHORITY_ROOT"); sys.stderr.write(f"{code}: P1350 checker {type(exc).__name__}\n"); return 2


if __name__ == "__main__":
    raise SystemExit(main())
