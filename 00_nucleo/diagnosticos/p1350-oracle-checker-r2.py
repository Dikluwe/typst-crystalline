#!/usr/bin/env python3
"""P1350 Oracle R2: disjoint language, focal-route and boundary domains."""

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
CHECKER_REL = "00_nucleo/diagnosticos/p1350-oracle-checker-r2.py"
RECEIPT_REL = "00_nucleo/diagnosticos/p1350-oracle-authorship-receipt-r2.json"
MANIFEST_REL = "00_nucleo/diagnosticos/p1350-authority-manifest-r2.json"
CORPUS_REL = "00_nucleo/diagnosticos/p1350-oracle-corpus-r2.json"
BOUNDARY_MANIFEST_REL = "00_nucleo/diagnosticos/p1350-oracle-boundary-manifest-r2.json"
LANGUAGE_REL = "00_nucleo/diagnosticos/p1350-oracle-language-id-sequence-r2.json"
SUPERVISOR_REL = "00_nucleo/diagnosticos/p1350-oracle-supervisor-r2.py"
PARSER_REL = "00_nucleo/diagnosticos/p1350-oracle-journal-parser-r2.py"
HARNESS_REL = "00_nucleo/diagnosticos/p1350-oracle-boundary-harness-r2.py"
COMPOSER_REL = "00_nucleo/diagnosticos/p1350-oracle-delivery-composer-r2.py"
SELF_ADAPTER_REL = "00_nucleo/diagnosticos/p1350-oracle-selftest-adapter-r2.py"
SELF_MAP_REL = "00_nucleo/diagnosticos/p1350-oracle-selftest-route-map-r2.json"
CONTRACT_ROOT_SHA256 = "a07ef8d9f088b25aa6184ff0bbc8193c7c55151797a75da7139588fe4e2a3a7e"
LANGUAGE_SHA256 = "e958b9c5e939a4b06b9e8594966001004de814e49e0141034d5351f1dbf4a907"
BOUNDARY_SHA256 = "d2e85d5936557936120754b647d41e88349ab19169ee4dd3fc967151bb68d8d8"
EXPECTED = {
    MANIFEST_REL: "a850acd48879c814dad90e27e07437e7de2146696d6118357dc9bee89f724a29",
    CORPUS_REL: "ac00c878512be78fd520590949e28296442fdc58bc3cd7c9b7f7c9dec5d67a3b",
    BOUNDARY_MANIFEST_REL: "8b46f6903854710f0117fdf704ae9161c479928c941ac0d83bbbaa8412c1d4c1",
    LANGUAGE_REL: "5bf7ebfef7457636594d20eeba449986b6a42214b0608112d6e7dce86459c569",
    SUPERVISOR_REL: "dc61799da0d71d73c43be0a9251289e94365979de4fc5809d7a359ef1e4adb1d",
    PARSER_REL: "24632052d47dc42368040b4e2476b689295f4c89d43b0fee64418e8f0d7f39e8",
    HARNESS_REL: "ed687b6320780b7bc536417514929bb2388eb48d72df4b6151accff9dcd9408d",
    COMPOSER_REL: "4a4969f3da9d7851affc9ae72ab5694964975ff83e99148ce0dfd3d022d0abd2",
    SELF_ADAPTER_REL: "64b34e0d8d3a992ef979fa3a37d0def646c0a027ef94131d392fa9e03cee0862",
    SELF_MAP_REL: "4d1323cc9985cbe1f55335f81f9b5f1faafcfe3b71d9cd4541ea3bdb264875b9",
}
PINS = {
    "00_nucleo/materialization/typst-passo-1350.md": "e09cf72e1e81c414343f8ec400e8d1e242910ed9a8a56ed7aaa96308a36d9ebf",
    "00_nucleo/diagnosticos/p1350-contract-spec-r2.json": "17c3b02d2f98ca12d6f89b369b78983663a8f8af6becf08daf47227b8cb42b90",
    "00_nucleo/diagnosticos/p1350-contract-binding-r2.json": "c341b0db70cebe933bbf6dea9fc19b2122f66534c4488ca4dc00ac9f3ab8f4dd",
    "00_nucleo/diagnosticos/p1350-contract-receipt-r2.json": "667e268009baf59cc7fe9f43df2dca55bfa69fbe2484ee06e7175d8165994d7f",
    "00_nucleo/diagnosticos/p1350-adapter-integration-blocker-r1.json": "3f2494f92d10eed22a9d8d0d3647ca9fa532f82f235415fcca93074174c2e341",
    "00_nucleo/diagnosticos/p1350-oracle-delivery-receipt-r1.json": "f4d1800e0223cc7d9dbd8225fbf47217d30c53d7da70acba4a9c908fe9966a6c",
    **EXPECTED,
}
CLASSIFICATION_KEYS = ["case_id", "classification", "closed_world", "evidence", "reason_code", "schema"]
META_KEYS = ["case_id", "closed_world", "meta_evidence", "meta_status", "schema"]
BLOCKER_KEYS = ["blocker_code", "case_id", "cleanup", "closed_world", "deadline", "exception", "phase", "schema", "supervisor", "verdict"]
BOUNDARY_KEYS = ["attack_id", "closed_world", "dynamic_evidence", "expected_blocker_code", "observed_blocker", "schema", "status"]
META_IDS = {"P1348-X06-failure-after-registry-commit", "P1348-X15-historical-original-open-nameerror"}
_BOUND_TOKEN = object()


class OracleFailure(RuntimeError):
    def __init__(self, code: str, detail: str):
        self.code = code; self.detail = detail
        super().__init__(f"{code}: {detail}")


def canonical(value: Any, lf: bool = True) -> bytes:
    raw = json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
    return raw + (b"\n" if lf else b"")


def sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def read_pinned(relative: str, digest: str) -> bytes:
    path = ROOT / relative
    if path.is_symlink() or not path.is_file(): raise OracleFailure("PROTECTED_INPUT", relative)
    raw = path.read_bytes()
    if sha256(raw) != digest: raise OracleFailure("PROTECTED_INPUT", relative)
    return raw


def _load(relative: str, digest: str, name: str) -> Any:
    read_pinned(relative, digest)
    spec = importlib.util.spec_from_file_location(name, ROOT / relative)
    if spec is None or spec.loader is None: raise OracleFailure("AUTHORITY_ROOT", f"loader {relative}")
    module = importlib.util.module_from_spec(spec); sys.modules[name] = module; spec.loader.exec_module(module)
    return module


SUPERVISOR = _load(SUPERVISOR_REL, EXPECTED[SUPERVISOR_REL], "p1350_supervisor_r2_pinned")
PARSER = _load(PARSER_REL, EXPECTED[PARSER_REL], "p1350_parser_r2_pinned")
HARNESS = _load(HARNESS_REL, EXPECTED[HARNESS_REL], "p1350_boundary_harness_r2_pinned")
HARNESS.bind_supervisor(SUPERVISOR)


def validate_protected_inputs() -> None:
    for relative, digest in PINS.items(): read_pinned(relative, digest)


def _strict_json(raw: bytes, label: str) -> Any:
    def reject(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        value: dict[str, Any] = {}
        for key, item in pairs:
            if key in value: raise OracleFailure("SCHEMA", f"{label} duplicate {key}")
            value[key] = item
        return value
    try: value = json.loads(raw, object_pairs_hook=reject, parse_constant=lambda token: (_ for _ in ()).throw(ValueError(token)))
    except OracleFailure: raise
    except Exception as exc: raise OracleFailure("SCHEMA", f"{label} JSON {type(exc).__name__}") from None
    if canonical(value) != raw: raise OracleFailure("SCHEMA", f"{label} noncanonical")
    return value


def _load_language(relative: str = LANGUAGE_REL, digest: str = EXPECTED[LANGUAGE_REL]) -> tuple[list[str], dict[str, Any]]:
    value = _strict_json(read_pinned(relative, digest), "language sequence")
    if type(value) is not dict or list(value) != ["closed_world", "count", "ids", "schema", "sequence_sha256"] or value["schema"] != "p1350-language-id-sequence-r2" or value["count"] != 122 or len(value["ids"]) != 122 or len(set(value["ids"])) != 122 or any(type(item) is not str or not item or not item.isascii() for item in value["ids"]) or sha256(canonical(value["ids"], False)) != LANGUAGE_SHA256 or value["sequence_sha256"] != LANGUAGE_SHA256:
        raise OracleFailure("PROTECTED_INPUT", "language domain")
    return value["ids"], value


def _load_boundary(relative: str = BOUNDARY_MANIFEST_REL, digest: str = EXPECTED[BOUNDARY_MANIFEST_REL]) -> tuple[list[list[str]], dict[str, Any]]:
    value = _strict_json(read_pinned(relative, digest), "boundary manifest")
    counts = {"DYNAMIC_SUPERVISOR": 21, "FRESHNESS": 3, "JOURNAL": 8, "STATIC_BYPASS": 4}
    implemented = HARNESS.HANG | HARNESS.FRAME | HARNESS.IDENTITY | HARNESS.ORPHAN | HARNESS.STATIC | HARNESS.JOURNAL | HARNESS.FRESHNESS | {"additional_fork", "adapter_exception", "checker_error"}
    operations = {row[2] for row in value.get("entries", []) if type(row) is list and len(row) == 4}
    if type(value) is not dict or list(value) != ["category_counts", "closed_world", "entries", "manifest_sha256", "schema", "total_count"] or value["schema"] != "p1350-boundary-attack-manifest-r2" or value["total_count"] != 36 or value["category_counts"] != counts or sha256(canonical(["p1350-boundary-attack-manifest-r2", value["entries"]], False)) != BOUNDARY_SHA256 or value["manifest_sha256"] != BOUNDARY_SHA256 or operations != implemented:
        raise OracleFailure("PROTECTED_INPUT", "boundary domain")
    return value["entries"], value


def load_corpus() -> dict[str, Any]:
    value = _strict_json(read_pinned(CORPUS_REL, EXPECTED[CORPUS_REL]), "corpus")
    if value.get("schema") != "p1350-oracle-corpus-r2" or value.get("manifest_sha256") != EXPECTED[MANIFEST_REL] or value.get("domain_counts") != {"boundary_attacks": 36, "focal_operations": 60, "language_ids": 122}:
        raise OracleFailure("PROTECTED_INPUT", "corpus")
    return value


def oracle_authoring_root(checker_sha256: str) -> str:
    ordered = [EXPECTED[key] for key in (MANIFEST_REL, CORPUS_REL, BOUNDARY_MANIFEST_REL, LANGUAGE_REL, SUPERVISOR_REL, PARSER_REL, HARNESS_REL, COMPOSER_REL, SELF_ADAPTER_REL, SELF_MAP_REL)]
    return sha256(canonical(["p1350-oracle-authoring-root-r2", CONTRACT_ROOT_SHA256, ordered, checker_sha256], False))


def oracle_delivery_root(authoring_root_sha256: str, checker_sha256: str, receipt_sha256: str, caller_sha256: str) -> str:
    return sha256(canonical(["p1350-oracle-delivery-root-r2", authoring_root_sha256, checker_sha256, EXPECTED[SUPERVISOR_REL], EXPECTED[PARSER_REL], EXPECTED[HARNESS_REL], EXPECTED[COMPOSER_REL], receipt_sha256, caller_sha256], False))


def validate_authorship_receipt(receipt_sha256: str, checker_sha256: str, root_sha256: str) -> dict[str, Any]:
    receipt = _strict_json(read_pinned(RECEIPT_REL, receipt_sha256), "authorship receipt")
    if receipt.get("schema") != "p1350-oracle-authorship-receipt-r2" or receipt.get("authoring_root_sha256") != root_sha256 or oracle_authoring_root(checker_sha256) != root_sha256:
        raise OracleFailure("AUTHORITY_ROOT", "authorship root")
    expected_outputs = [[path, EXPECTED[path]] for path in (MANIFEST_REL, CORPUS_REL, BOUNDARY_MANIFEST_REL, LANGUAGE_REL, SUPERVISOR_REL, PARSER_REL, HARNESS_REL, COMPOSER_REL, SELF_ADAPTER_REL, SELF_MAP_REL)] + [[CHECKER_REL, checker_sha256]]
    if receipt.get("outputs") != expected_outputs or any(key in receipt for key in ("caller", "caller_sha256", "delivery_receipt", "delivery_receipt_sha256", "self_sha256")):
        raise OracleFailure("AUTHORITY_ROOT", "authorship DAG")
    return receipt


def _pin_from_delivery(value: Any, label: str) -> tuple[str, str, bytes]:
    if type(value) is not list or len(value) != 2 or any(type(item) is not str for item in value): raise OracleFailure("AUTHORITY_ROOT", f"{label} pin")
    return value[0], value[1], read_pinned(value[0], value[1])


@dataclass
class RunContext:
    token: object
    authority_root_sha256: str
    boundary_entries: dict[str, list[str]]
    boundary_attack_manifest_sha256: str
    focal_operation_route_map_sha256: str
    language_id_sequence_sha256: str
    language_ids: tuple[str, ...]
    journal: Any
    journal_fd: int
    journal_path: str
    plan: Any
    route_operation_ids: frozenset[str]
    delivery_sha256: str
    finished: bool = False


def bind_verified_run(execution_delivery_path: str, execution_delivery_whole_file_sha256: str) -> RunContext:
    validate_protected_inputs(); load_corpus()
    delivery = _strict_json(read_pinned(execution_delivery_path, execution_delivery_whole_file_sha256), "execution delivery")
    keys = ["adapter", "adapter_authorship_receipt", "adversary", "adversary_authorship_receipt", "authority_root_sha256", "boundary_attack_manifest", "checker", "focal_operation_route_map", "journal_parser", "language_id_sequence", "oracle_authorship_receipt", "oracle_caller", "schema", "supervisor"]
    if type(delivery) is not dict or list(delivery) != keys or delivery["schema"] != "p1350-execution-delivery-receipt-r2": raise OracleFailure("AUTHORITY_ROOT", "execution delivery schema")
    plain = {}
    for key in ("adapter", "adapter_authorship_receipt", "adversary", "adversary_authorship_receipt", "checker", "journal_parser", "oracle_authorship_receipt", "oracle_caller", "supervisor"):
        plain[key] = _pin_from_delivery(delivery[key], key)
    if plain["checker"][:2] != (CHECKER_REL, sha256((ROOT / CHECKER_REL).read_bytes())) or plain["supervisor"][:2] != (SUPERVISOR_REL, EXPECTED[SUPERVISOR_REL]) or plain["journal_parser"][:2] != (PARSER_REL, EXPECTED[PARSER_REL]):
        raise OracleFailure("AUTHORITY_ROOT", "oracle execution pins")
    language_domain, route_domain, boundary_domain = delivery["language_id_sequence"], delivery["focal_operation_route_map"], delivery["boundary_attack_manifest"]
    language_pin = language_domain.get("artifact") if type(language_domain) is dict else None
    route_pin = route_domain.get("artifact") if type(route_domain) is dict else None
    boundary_pin = boundary_domain.get("artifact") if type(boundary_domain) is dict else None
    language_ids, _ = _load_language(*_pin_from_delivery(language_pin, "language")[:2])
    boundary_entries, _ = _load_boundary(*_pin_from_delivery(boundary_pin, "boundary")[:2])
    if language_domain.get("count") != 122 or language_domain.get("sequence_sha256") != LANGUAGE_SHA256 or boundary_domain.get("total_count") != 36 or boundary_domain.get("manifest_sha256") != BOUNDARY_SHA256:
        raise OracleFailure("AUTHORITY_ROOT", "delivery domain pins")
    route_relative, route_whole_sha, _ = _pin_from_delivery(route_pin, "route map")
    plan = SUPERVISOR.AdapterPlan(plain["adapter"][0], plain["adapter"][1], route_relative, route_whole_sha, route_domain.get("focal_operation_route_map_sha256"), delivery["authority_root_sha256"])
    route_map = SUPERVISOR.validate_route_map(ROOT, plan)
    if route_domain.get("total_count") != 60 or route_domain.get("local_count") != 12 or route_domain.get("inherited_count") != 48 or route_domain.get("operation_ids_sha256") != route_map["operation_ids_sha256"] or route_map["language_id_sequence_sha256"] != LANGUAGE_SHA256:
        raise OracleFailure("AUTHORITY_ROOT", "route delivery pins")
    oracle_hashes = [plain[key][1] for key in ("checker", "oracle_caller", "supervisor", "journal_parser")]
    root_values = ["p1350-execution-authority-root-r2", CONTRACT_ROOT_SHA256, oracle_hashes, plain["oracle_authorship_receipt"][1], [plain["adversary"][1]], plain["adversary_authorship_receipt"][1], plain["adapter"][1], route_whole_sha, plain["adapter_authorship_receipt"][1], LANGUAGE_SHA256, route_domain["focal_operation_route_map_sha256"], BOUNDARY_SHA256]
    if sha256(canonical(root_values, False)) != delivery["authority_root_sha256"]: raise OracleFailure("AUTHORITY_ROOT", "execution root")
    fd, journal_path = tempfile.mkstemp(prefix="p1350-r2-journal-", suffix=".jsonl", dir="/dev/shm"); os.close(fd)
    journal_fd = os.open(journal_path, os.O_RDWR | os.O_APPEND | os.O_CLOEXEC)
    journal = SUPERVISOR.RunJournal(journal_fd, secrets.token_hex(32), None)
    return RunContext(_BOUND_TOKEN, delivery["authority_root_sha256"], {row[0]: row for row in boundary_entries}, BOUNDARY_SHA256, route_domain["focal_operation_route_map_sha256"], LANGUAGE_SHA256, tuple(language_ids), journal, journal_fd, journal_path, plan, frozenset(row["operation_id"] for row in route_map["entries"]), execution_delivery_whole_file_sha256)


def _context(value: Any) -> RunContext:
    if not isinstance(value, RunContext) or value.token is not _BOUND_TOKEN or value.finished: raise OracleFailure("AUTHORITY_ROOT", "opaque RunContext")
    return value


def _validate_result(value: Any, case_id: str, allow_meta: bool = False) -> dict[str, Any]:
    if type(value) is not dict: raise OracleFailure("FRAME_AUTHORITY", "result object")
    schema = value.get("schema")
    if schema == "p1349-classification-result-r1" and list(value) == CLASSIFICATION_KEYS and value["case_id"] == case_id and value["classification"] in ("Preserved", "Violated", "Unknown") and type(value["evidence"]) is dict and type(value["reason_code"]) is str:
        if value["classification"] == "Unknown" and value["reason_code"] != "OPAQUE_PAYLOAD": raise OracleFailure("FRAME_AUTHORITY", "Unknown allowlist")
        return value
    if allow_meta and schema == "p1349-meta-result-r1" and list(value) == META_KEYS and value["case_id"] == case_id and value["meta_status"] in ("PASS", "FAIL") and type(value["meta_evidence"]) is dict: return value
    if schema == "p1349-integration-blocker-r1" and list(value) == BLOCKER_KEYS and value["case_id"] == case_id and value["verdict"] == "INTEGRATION_BLOCKED" and value["blocker_code"] in SUPERVISOR.BLOCKER_CODES: return value
    raise OracleFailure("FRAME_AUTHORITY", "result domain")


def exercise_focal(case: Any, targets: Any, language_id_sequence_sha256: str, focal_operation_route_map_sha256: str, run_context: Any) -> dict[str, Any]:
    context = _context(run_context)
    if language_id_sequence_sha256 != context.language_id_sequence_sha256 or focal_operation_route_map_sha256 != context.focal_operation_route_map_sha256 or type(case) is not dict or list(case) != ["case_id", "operation_id", "payload"] or type(targets) is not dict:
        raise OracleFailure("PROTECTED_INPUT", "focal domain parameters")
    if type(case["case_id"]) is not str or case["operation_id"] not in context.route_operation_ids: raise OracleFailure("SUPERVISOR_AUTHORITY", "focal operation")
    internal = {"case_id": case["case_id"], "domain": "classification", "operation": case["operation_id"], "payload": {"case": case["payload"], "targets": targets}, "sequence_index": context.journal.next_order}
    try: value, parent = SUPERVISOR._supervise_case(ROOT, internal, context.plan, context.journal)
    except SUPERVISOR.SupervisorFailure as exc: raise OracleFailure(exc.code, exc.detail) from None
    value = _validate_result(value, case["case_id"])
    if value["schema"] == "p1349-classification-result-r1": value = dict(value); value["evidence"] = dict(value["evidence"], parent_supervisor=parent)
    return value


def exercise_meta(case: Any, targets: Any, language_id_sequence_sha256: str, run_context: Any) -> dict[str, Any]:
    context = _context(run_context)
    if language_id_sequence_sha256 != context.language_id_sequence_sha256 or type(case) is not dict or list(case) != ["case_id", "operation", "payload"] or case["case_id"] not in META_IDS or type(targets) is not dict:
        raise OracleFailure("PROTECTED_INPUT", "meta domain parameters")
    internal = {"case_id": case["case_id"], "domain": "meta", "operation": case["operation"], "payload": {"case": case["payload"], "targets": targets}, "sequence_index": context.journal.next_order}
    try: value, parent = SUPERVISOR._supervise_case(ROOT, internal, context.plan, context.journal)
    except SUPERVISOR.SupervisorFailure as exc: raise OracleFailure(exc.code, exc.detail) from None
    value = _validate_result(value, case["case_id"], True)
    if value["schema"] == "p1349-meta-result-r1": value = dict(value); value["meta_evidence"] = dict(value["meta_evidence"], parent_supervisor=parent)
    return value


def exercise_boundary(attack: Any, targets: Any, boundary_attack_manifest_sha256: str, run_context: Any) -> dict[str, Any]:
    context = _context(run_context)
    if boundary_attack_manifest_sha256 != context.boundary_attack_manifest_sha256 or type(attack) is not str or attack not in context.boundary_entries or type(targets) is not dict:
        raise OracleFailure("PROTECTED_INPUT", "boundary domain parameters")
    try: value = HARNESS.run_boundary_experiment(context.boundary_entries[attack], context.journal, PARSER)
    except Exception as exc:
        code = getattr(exc, "code", "CHECKER_ERROR")
        return SUPERVISOR.integration_blocker(attack, code, "boundary_harness", str(exc), {"cleanup": {}, "deadline": {}, "supervisor": {}})
    if type(value) is not dict or list(value) != BOUNDARY_KEYS or value["schema"] != "p1350-boundary-result-r2" or value["attack_id"] != attack or value["status"] not in ("PASS", "FAIL") or any(key in value for key in ("classification", "reason_code", "meta_status", "mutation_score")):
        raise OracleFailure("FRAME_AUTHORITY", "boundary result")
    dynamic = value["dynamic_evidence"]
    if type(dynamic) is not dict or list(dynamic) != HARNESS.DYNAMIC_KEYS or list(dynamic["cleanup"]) != HARNESS.CLEANUP_KEYS or list(dynamic["injection"]) != HARNESS.INJECTION_KEYS:
        raise OracleFailure("FRAME_AUTHORITY", "boundary evidence")
    return value


def finish_run(run_context: Any) -> dict[str, Any]:
    context = _context(run_context); final_line = context.journal.finish(); os.fsync(context.journal_fd); os.close(context.journal_fd); context.journal_fd = -1; context.finished = True
    raw = Path(context.journal_path).read_bytes(); parsed = PARSER.parse_journal_bytes(raw, os.getpid(), context.journal.run_id, context.journal.next_order)
    return {"case_result_count": parsed["case_result_count"], "case_start_count": parsed["case_start_count"], "closed_world": {"rule": "Closed journal identity and cleanup counts only; no semantic classifications."}, "final_line_sha256": final_line, "genesis_sha256": PARSER.GENESIS, "journal_path_identity": context.journal_path, "journal_sha256": parsed["journal_sha256"], "line_count": parsed["line_count"], "parser_sha256": EXPECTED[PARSER_REL], "process_groups_absent": parsed["process_groups_absent"], "run_complete_count": parsed["run_complete_count"], "run_id": parsed["run_id"], "schema": "p1350-run-journal-receipt-r1", "unexpected_integration_blockers": 0, "writer_pid": parsed["writer_pid"]}


def _self_context(expected_cases: int) -> RunContext:
    language_ids, _ = _load_language(); entries, _ = _load_boundary(); route_map = _strict_json(read_pinned(SELF_MAP_REL, EXPECTED[SELF_MAP_REL]), "self map")
    plan = SUPERVISOR.AdapterPlan(SELF_ADAPTER_REL, EXPECTED[SELF_ADAPTER_REL], SELF_MAP_REL, EXPECTED[SELF_MAP_REL], "c4a2535ec2525e4855c913219afb907772cfde2340e4c0f29e99a5ab4a03e57c", oracle_authoring_root(sha256((ROOT / CHECKER_REL).read_bytes())))
    SUPERVISOR.validate_route_map(ROOT, plan)
    fd, path = tempfile.mkstemp(prefix="p1350-r2-self-", suffix=".jsonl", dir="/dev/shm"); os.close(fd); fd = os.open(path, os.O_RDWR | os.O_APPEND | os.O_CLOEXEC)
    journal = SUPERVISOR.RunJournal(fd, secrets.token_hex(32), expected_cases)
    return RunContext(_BOUND_TOKEN, plan.authority_root_sha256, {row[0]: row for row in entries}, BOUNDARY_SHA256, plan.route_map_domain_sha256, LANGUAGE_SHA256, tuple(language_ids), journal, fd, path, plan, frozenset(row["operation_id"] for row in route_map["entries"]), "SELF_FOCAL")


def nonptrace_focal() -> dict[str, Any]:
    validate_protected_inputs(); load_corpus(); started = time.monotonic_ns(); context = _self_context(6); records = []
    try:
        case_id = context.language_ids[0]
        focal = exercise_focal({"case_id": case_id, "operation_id": "p1350-self-op-001", "payload": {}}, {"authority": "p1350-oracle-r2"}, LANGUAGE_SHA256, context.focal_operation_route_map_sha256, context)
        records.append({"id": "focal", "observed": focal["classification"], "reason": focal["reason_code"]})
        meta = exercise_meta({"case_id": "P1348-X06-failure-after-registry-commit", "operation": "postcommit_failure", "payload": {}}, {"authority": "p1350-oracle-r2"}, LANGUAGE_SHA256, context)
        records.append({"id": "meta", "observed": meta["meta_status"]})
        for attack_id in ("P1350-B01-raw-inherited-entry", "P1350-D12-missing-frame", "P1350-J01-truncated-line", "P1350-F01-repeat-reuses-registry"):
            result = exercise_boundary(attack_id, {"authority": "p1350-oracle-r2"}, BOUNDARY_SHA256, context)
            records.append({"id": attack_id, "observed": result.get("status", result.get("verdict")), "reason": result.get("expected_blocker_code", result.get("blocker_code"))})
        receipt = finish_run(context)
        boundary_pass = sum(row["observed"] == "PASS" for row in records if row["id"].startswith("P1350-"))
        passed = focal["classification"] == "Preserved" and meta["meta_status"] == "PASS" and boundary_pass == 4 and receipt["case_result_count"] == 6 and receipt["process_groups_absent"] == 6
        return {"closed_world": {"rule": "Oracle-owned non-ptrace R2 controls only; adversary, full, candidate and ptrace are not executed."}, "cost": {"elapsed_ns": time.monotonic_ns() - started, "supervised_case_runs": 6}, "execution": {"adversary_runs": 0, "boundary_runs": 4, "candidate_runs": 0, "full_corpus_runs": 0, "ptrace_runs": 0}, "records": records, "regime": "executado sem atestacao de isolamento", "revision": 2, "schema": "p1350-oracle-nonptrace-focal-r2", "step": 1350, "summary": {"boundary_controls_passed": boundary_pass, "boundary_controls_total": 4, "domain_counts": [122, 60, 36], "journal_case_results": receipt["case_result_count"], "process_groups_absent": receipt["process_groups_absent"]}, "verdict": "ORACLE_R2_NONPTRACE_FOCAL_NOT_SEALED" if passed else "ORACLE_R2_BLOCKED_NOT_SEALED"}
    finally:
        if context.journal_fd >= 0:
            try: os.close(context.journal_fd)
            except OSError: pass
        try: os.unlink(context.journal_path)
        except OSError: pass


def run_focal(checker_sha256: str, receipt_sha256: str, root_sha256: str) -> dict[str, Any]:
    validate_protected_inputs(); load_corpus(); validate_authorship_receipt(receipt_sha256, checker_sha256, root_sha256); return nonptrace_focal()


def main() -> int:
    if sys.argv[1:] != ["--self-focal"]:
        sys.stderr.write("AUTHORITY_ROOT: P1350 R2 checker accepts exactly --self-focal\n"); return 2
    try:
        report = nonptrace_focal(); sys.stdout.buffer.write(canonical(report)); return 0 if report["verdict"] == "ORACLE_R2_NONPTRACE_FOCAL_NOT_SEALED" else 1
    except Exception as exc:
        sys.stderr.write(f"{getattr(exc, 'code', 'AUTHORITY_ROOT')}: P1350 R2 checker {type(exc).__name__}\n"); return 2


if __name__ == "__main__":
    raise SystemExit(main())
