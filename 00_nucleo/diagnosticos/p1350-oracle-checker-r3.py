#!/usr/bin/env python3
"""P1350 Oracle R3: final focal repair of boundary and meta bindings."""

from __future__ import annotations

import hashlib
import importlib.util
import os
import secrets
import sys
import tempfile
import time
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
CHECKER_REL = "00_nucleo/diagnosticos/p1350-oracle-checker-r3.py"
RECEIPT_REL = "00_nucleo/diagnosticos/p1350-oracle-authorship-receipt-r3.json"
BASE_REL = "00_nucleo/diagnosticos/p1350-oracle-checker-r2.py"
MANIFEST_REL = "00_nucleo/diagnosticos/p1350-authority-manifest-r3.json"
CORPUS_REL = "00_nucleo/diagnosticos/p1350-oracle-corpus-r2.json"
BOUNDARY_MANIFEST_REL = "00_nucleo/diagnosticos/p1350-oracle-boundary-manifest-r2.json"
LANGUAGE_REL = "00_nucleo/diagnosticos/p1350-oracle-language-id-sequence-r2.json"
SUPERVISOR_REL = "00_nucleo/diagnosticos/p1350-oracle-supervisor-r3.py"
PARSER_REL = "00_nucleo/diagnosticos/p1350-oracle-journal-parser-r2.py"
HARNESS_REL = "00_nucleo/diagnosticos/p1350-oracle-boundary-harness-r3.py"
COMPOSER_REL = "00_nucleo/diagnosticos/p1350-oracle-delivery-composer-r3.py"
SELF_ADAPTER_REL = "00_nucleo/diagnosticos/p1350-oracle-selftest-adapter-r2.py"
SELF_MAP_REL = "00_nucleo/diagnosticos/p1350-oracle-selftest-route-map-r2.json"
CONTRACT_ROOT_SHA256 = "a07ef8d9f088b25aa6184ff0bbc8193c7c55151797a75da7139588fe4e2a3a7e"
LANGUAGE_SHA256 = "e958b9c5e939a4b06b9e8594966001004de814e49e0141034d5351f1dbf4a907"
BOUNDARY_SHA256 = "d2e85d5936557936120754b647d41e88349ab19169ee4dd3fc967151bb68d8d8"
EXPECTED = {
    BASE_REL: "d8c8eae84c0843f1b530caa15098d724695194e135b5201e7f53abeaad936024",
    MANIFEST_REL: "0f95a9097d41b60cfe7c0be6cfb874a4098416d57832c0d0fabe84e6eac31876",
    CORPUS_REL: "ac00c878512be78fd520590949e28296442fdc58bc3cd7c9b7f7c9dec5d67a3b",
    BOUNDARY_MANIFEST_REL: "8b46f6903854710f0117fdf704ae9161c479928c941ac0d83bbbaa8412c1d4c1",
    LANGUAGE_REL: "5bf7ebfef7457636594d20eeba449986b6a42214b0608112d6e7dce86459c569",
    SUPERVISOR_REL: "1d662c6fa98f0b00fdf8a56c4112c9be1863469a1b1d2eb1de1fdf3d8bb032c6",
    PARSER_REL: "24632052d47dc42368040b4e2476b689295f4c89d43b0fee64418e8f0d7f39e8",
    HARNESS_REL: "7de050f2f86612306e4dac934fee8f4f2730a98f75103551233a9139f8e6500e",
    COMPOSER_REL: "3888ae81248a81bedd90f3fd6d295a9a13d8c346491b3ac3b5be90af115e2af5",
    SELF_ADAPTER_REL: "64b34e0d8d3a992ef979fa3a37d0def646c0a027ef94131d392fa9e03cee0862",
    SELF_MAP_REL: "4d1323cc9985cbe1f55335f81f9b5f1faafcfe3b71d9cd4541ea3bdb264875b9",
}
PINS = {
    "00_nucleo/materialization/typst-passo-1350.md": "e09cf72e1e81c414343f8ec400e8d1e242910ed9a8a56ed7aaa96308a36d9ebf",
    "00_nucleo/diagnosticos/p1350-contract-spec-r2.json": "17c3b02d2f98ca12d6f89b369b78983663a8f8af6becf08daf47227b8cb42b90",
    "00_nucleo/diagnosticos/p1350-contract-binding-r2.json": "c341b0db70cebe933bbf6dea9fc19b2122f66534c4488ca4dc00ac9f3ab8f4dd",
    "00_nucleo/diagnosticos/p1350-contract-receipt-r2.json": "667e268009baf59cc7fe9f43df2dca55bfa69fbe2484ee06e7175d8165994d7f",
    "00_nucleo/diagnosticos/p1350-adapter-integration-blocker-r2.json": "9ed548a8e36bf89815bdce5de0ee9625f57c76c371c88a459c2f055ad442e629",
    "00_nucleo/diagnosticos/p1350-oracle-delivery-receipt-r2.json": "c10c3b81321b62f63c104174b3c9e844026dcaf16020dea86bf970af9065abb6",
    **EXPECTED,
}
BOUNDARY_KEYS = ["attack_id", "closed_world", "dynamic_evidence", "expected_blocker_code", "observed_blocker", "schema", "status"]
META_IDS = {"P1348-X06-failure-after-registry-commit", "P1348-X15-historical-original-open-nameerror"}


def sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def _load(relative: str, digest: str, name: str) -> Any:
    path = ROOT / relative; raw = path.read_bytes()
    if path.is_symlink() or sha256(raw) != digest: raise RuntimeError(f"AUTHORITY_ROOT: drift {relative}")
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None: raise RuntimeError(f"AUTHORITY_ROOT: loader {relative}")
    module = importlib.util.module_from_spec(spec); sys.modules[name] = module; spec.loader.exec_module(module); return module


BASE = _load(BASE_REL, EXPECTED[BASE_REL], "p1350_checker_r2_for_r3")
SUPERVISOR = _load(SUPERVISOR_REL, EXPECTED[SUPERVISOR_REL], "p1350_supervisor_r3_pinned")
PARSER = _load(PARSER_REL, EXPECTED[PARSER_REL], "p1350_parser_r2_for_r3")
HARNESS = _load(HARNESS_REL, EXPECTED[HARNESS_REL], "p1350_harness_r3_pinned")
HARNESS.bind_supervisor(SUPERVISOR)
BASE.SUPERVISOR = SUPERVISOR
OracleFailure = BASE.OracleFailure
RunContext = BASE.RunContext
canonical = BASE.canonical
_strict_json = BASE._strict_json


def validate_protected_inputs() -> None:
    for relative, digest in PINS.items(): BASE.read_pinned(relative, digest)


def load_corpus() -> dict[str, Any]:
    return BASE.load_corpus()


def oracle_authoring_root(checker_sha256: str) -> str:
    order = (MANIFEST_REL, CORPUS_REL, BOUNDARY_MANIFEST_REL, LANGUAGE_REL, SUPERVISOR_REL, PARSER_REL, HARNESS_REL, COMPOSER_REL, SELF_ADAPTER_REL, SELF_MAP_REL)
    return sha256(canonical(["p1350-oracle-authoring-root-r3", CONTRACT_ROOT_SHA256, [EXPECTED[key] for key in order], checker_sha256], False))


def oracle_delivery_root(authoring_root_sha256: str, checker_sha256: str, receipt_sha256: str, caller_sha256: str) -> str:
    return sha256(canonical(["p1350-oracle-delivery-root-r3", authoring_root_sha256, checker_sha256, EXPECTED[SUPERVISOR_REL], EXPECTED[PARSER_REL], EXPECTED[HARNESS_REL], EXPECTED[COMPOSER_REL], receipt_sha256, caller_sha256], False))


def validate_authorship_receipt(receipt_sha256: str, checker_sha256: str, root_sha256: str) -> dict[str, Any]:
    receipt = _strict_json(BASE.read_pinned(RECEIPT_REL, receipt_sha256), "R3 authorship receipt")
    if receipt.get("schema") != "p1350-oracle-authorship-receipt-r3" or receipt.get("authoring_root_sha256") != root_sha256 or oracle_authoring_root(checker_sha256) != root_sha256:
        raise OracleFailure("AUTHORITY_ROOT", "R3 authorship root")
    order = (MANIFEST_REL, CORPUS_REL, BOUNDARY_MANIFEST_REL, LANGUAGE_REL, SUPERVISOR_REL, PARSER_REL, HARNESS_REL, COMPOSER_REL, SELF_ADAPTER_REL, SELF_MAP_REL)
    expected = [[path, EXPECTED[path]] for path in order] + [[CHECKER_REL, checker_sha256]]
    if receipt.get("outputs") != expected or any(key in receipt for key in ("caller", "delivery_receipt", "self_sha256")):
        raise OracleFailure("AUTHORITY_ROOT", "R3 authorship DAG")
    return receipt


def bind_verified_run(execution_delivery_path: str, execution_delivery_whole_file_sha256: str) -> RunContext:
    validate_protected_inputs(); load_corpus()
    delivery = _strict_json(BASE.read_pinned(execution_delivery_path, execution_delivery_whole_file_sha256), "execution delivery")
    keys = ["adapter", "adapter_authorship_receipt", "adversary", "adversary_authorship_receipt", "authority_root_sha256", "boundary_attack_manifest", "checker", "focal_operation_route_map", "journal_parser", "language_id_sequence", "oracle_authorship_receipt", "oracle_caller", "schema", "supervisor"]
    if type(delivery) is not dict or list(delivery) != keys or delivery["schema"] != "p1350-execution-delivery-receipt-r2": raise OracleFailure("AUTHORITY_ROOT", "execution delivery schema")
    plain = {key: BASE._pin_from_delivery(delivery[key], key) for key in ("adapter", "adapter_authorship_receipt", "adversary", "adversary_authorship_receipt", "checker", "journal_parser", "oracle_authorship_receipt", "oracle_caller", "supervisor")}
    checker_sha = sha256((ROOT / CHECKER_REL).read_bytes())
    if plain["checker"][:2] != (CHECKER_REL, checker_sha) or plain["supervisor"][:2] != (SUPERVISOR_REL, EXPECTED[SUPERVISOR_REL]) or plain["journal_parser"][:2] != (PARSER_REL, EXPECTED[PARSER_REL]) or plain["oracle_authorship_receipt"][0] != RECEIPT_REL:
        raise OracleFailure("AUTHORITY_ROOT", "R3 execution pins")
    language_domain, route_domain, boundary_domain = delivery["language_id_sequence"], delivery["focal_operation_route_map"], delivery["boundary_attack_manifest"]
    language_pin = language_domain.get("artifact") if type(language_domain) is dict else None
    route_pin = route_domain.get("artifact") if type(route_domain) is dict else None
    boundary_pin = boundary_domain.get("artifact") if type(boundary_domain) is dict else None
    language_ids, _ = BASE._load_language(*BASE._pin_from_delivery(language_pin, "language")[:2])
    boundary_entries, _ = BASE._load_boundary(*BASE._pin_from_delivery(boundary_pin, "boundary")[:2])
    if language_domain.get("count") != 122 or language_domain.get("sequence_sha256") != LANGUAGE_SHA256 or boundary_domain.get("total_count") != 36 or boundary_domain.get("manifest_sha256") != BOUNDARY_SHA256:
        raise OracleFailure("AUTHORITY_ROOT", "delivery domains")
    route_relative, route_whole_sha, _ = BASE._pin_from_delivery(route_pin, "route map")
    plan = SUPERVISOR.AdapterPlan(plain["adapter"][0], plain["adapter"][1], route_relative, route_whole_sha, route_domain.get("focal_operation_route_map_sha256"), delivery["authority_root_sha256"])
    route_map = SUPERVISOR.validate_route_map(ROOT, plan)
    if route_domain.get("total_count") != 60 or route_domain.get("local_count") != 12 or route_domain.get("inherited_count") != 48 or route_domain.get("operation_ids_sha256") != route_map["operation_ids_sha256"] or route_map["language_id_sequence_sha256"] != LANGUAGE_SHA256:
        raise OracleFailure("AUTHORITY_ROOT", "route domains")
    oracle_hashes = [plain[key][1] for key in ("checker", "oracle_caller", "supervisor", "journal_parser")]
    root_values = ["p1350-execution-authority-root-r2", CONTRACT_ROOT_SHA256, oracle_hashes, plain["oracle_authorship_receipt"][1], [plain["adversary"][1]], plain["adversary_authorship_receipt"][1], plain["adapter"][1], route_whole_sha, plain["adapter_authorship_receipt"][1], LANGUAGE_SHA256, route_domain["focal_operation_route_map_sha256"], BOUNDARY_SHA256]
    if sha256(canonical(root_values, False)) != delivery["authority_root_sha256"]: raise OracleFailure("AUTHORITY_ROOT", "execution root")
    fd, path = tempfile.mkstemp(prefix="p1350-r3-journal-", suffix=".jsonl", dir="/dev/shm"); os.close(fd); fd = os.open(path, os.O_RDWR | os.O_APPEND | os.O_CLOEXEC)
    journal = SUPERVISOR.RunJournal(fd, secrets.token_hex(32), None)
    return RunContext(BASE._BOUND_TOKEN, delivery["authority_root_sha256"], {row[0]: row for row in boundary_entries}, BOUNDARY_SHA256, route_domain["focal_operation_route_map_sha256"], LANGUAGE_SHA256, tuple(language_ids), journal, fd, path, plan, frozenset(row["operation_id"] for row in route_map["entries"]), execution_delivery_whole_file_sha256)


def exercise_focal(case: Any, targets: Any, language_id_sequence_sha256: str, focal_operation_route_map_sha256: str, run_context: Any) -> dict[str, Any]:
    return BASE.exercise_focal(case, targets, language_id_sequence_sha256, focal_operation_route_map_sha256, run_context)


def exercise_meta(case: Any, targets: Any, language_id_sequence_sha256: str, run_context: Any) -> dict[str, Any]:
    context = BASE._context(run_context)
    if language_id_sequence_sha256 != context.language_id_sequence_sha256 or type(case) is not dict or list(case) != ["case_id", "operation", "payload"] or case["case_id"] not in META_IDS or type(targets) is not dict:
        raise OracleFailure("PROTECTED_INPUT", "meta domain parameters")
    internal = {"case_id": case["case_id"], "domain": "meta", "operation": case["operation"], "payload": {"case": case["payload"], "targets": targets}, "sequence_index": context.journal.next_order}
    try: value, _parent_envelope = SUPERVISOR._supervise_case(ROOT, internal, context.plan, context.journal)
    except SUPERVISOR.SupervisorFailure as exc: raise OracleFailure(exc.code, exc.detail) from None
    # Deliberately return the inherited object unchanged.  Parent transport
    # evidence is already durable in context.journal and is not meta_evidence.
    return BASE._validate_result(value, case["case_id"], True)


def exercise_boundary(attack: Any, targets: Any, boundary_attack_manifest_sha256: str, run_context: Any) -> dict[str, Any]:
    context = BASE._context(run_context)
    request = HARNESS.validate_attack_request(attack)
    attack_id = request["attack_id"]
    if boundary_attack_manifest_sha256 != context.boundary_attack_manifest_sha256 or attack_id not in context.boundary_entries or type(targets) is not dict:
        raise OracleFailure("PROTECTED_INPUT", "boundary domain parameters")
    try: value = HARNESS.run_boundary_experiment(request, context.boundary_entries[attack_id], context.journal, PARSER)
    except Exception as exc:
        code = getattr(exc, "code", "CHECKER_ERROR")
        return SUPERVISOR.integration_blocker(attack_id, code, "boundary_harness", str(exc), {"cleanup": {}, "deadline": {}, "supervisor": {}})
    if type(value) is not dict or list(value) != BOUNDARY_KEYS or value["schema"] != "p1350-boundary-result-r2" or value["attack_id"] != attack_id or value["status"] not in ("PASS", "FAIL") or value["closed_world"] is not True:
        raise OracleFailure("FRAME_AUTHORITY", "boundary result")
    dynamic = value["dynamic_evidence"]
    if type(dynamic) is not dict or list(dynamic) != HARNESS.DYNAMIC_KEYS or list(dynamic["cleanup"]) != HARNESS.CLEANUP_KEYS or list(dynamic["executor"]) != HARNESS.EXECUTOR_KEYS or list(dynamic["injection"]) != HARNESS.INJECTION_KEYS or list(dynamic["supervisor"]) != HARNESS.SUPERVISOR_KEYS:
        raise OracleFailure("FRAME_AUTHORITY", "boundary evidence")
    if dynamic["attack_nonce_sha256"] != sha256(request["attack_nonce"].encode()) or dynamic["injection"]["challenge_sha256"] != sha256(request["challenge"].encode()):
        raise OracleFailure("FRAME_AUTHORITY", "boundary challenge binding")
    return value


finish_run = BASE.finish_run


def _self_context(expected_cases: int) -> RunContext:
    context = BASE._self_context(expected_cases)
    BASE.SUPERVISOR = SUPERVISOR
    return context


def nonptrace_focal() -> dict[str, Any]:
    validate_protected_inputs(); load_corpus(); started = time.monotonic_ns(); context = _self_context(2)
    try:
        expected_meta = {"oracle_selftest": True, "operation": "postcommit_failure"}
        meta = exercise_meta({"case_id": "P1348-X06-failure-after-registry-commit", "operation": "postcommit_failure", "payload": {}}, {"authority": "p1350-oracle-r3"}, LANGUAGE_SHA256, context)
        attack = {"attack_id": "P1350-B01-raw-inherited-entry", "attack_nonce": secrets.token_hex(32), "challenge": secrets.token_hex(32)}
        boundary = exercise_boundary(attack, {"authority": "p1350-oracle-r3"}, BOUNDARY_SHA256, context)
        receipt = finish_run(context)
        dynamic = boundary.get("dynamic_evidence", {})
        passed = meta.get("meta_evidence") == expected_meta and "parent_supervisor" not in meta.get("meta_evidence", {}) and boundary.get("status") == "PASS" and dynamic.get("attack_nonce_sha256") == sha256(attack["attack_nonce"].encode()) and dynamic.get("injection", {}).get("challenge_sha256") == sha256(attack["challenge"].encode()) and dynamic.get("closed_world") is None and boundary.get("closed_world") is True and receipt["case_result_count"] == 2 and receipt["process_groups_absent"] == 2
        return {"closed_world": {"rule": "R3 focal exercises only the two repaired links and non-adversarial controls."}, "cost": {"elapsed_ns": time.monotonic_ns() - started, "supervised_case_runs": 2}, "execution": {"adversary_runs": 0, "boundary_runs": 1, "candidate_runs": 0, "full_corpus_runs": 0, "meta_runs": 1, "ptrace_runs": 0}, "regime": "executado sem atestacao de isolamento", "revision": 3, "schema": "p1350-oracle-nonptrace-focal-r3", "summary": {"boundary_challenge_bound": boundary.get("status") == "PASS", "domain_counts": [122, 60, 36], "journal_case_results": receipt["case_result_count"], "meta_projection_preserved": meta.get("meta_evidence") == expected_meta, "process_groups_absent": receipt["process_groups_absent"]}, "verdict": "ORACLE_R3_NONPTRACE_FOCAL_NOT_SEALED" if passed else "ORACLE_R3_BLOCKED_NOT_SEALED"}
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
        sys.stderr.write("AUTHORITY_ROOT: P1350 R3 checker accepts exactly --self-focal\n"); return 2
    try:
        report = nonptrace_focal(); sys.stdout.buffer.write(canonical(report)); return 0 if report["verdict"] == "ORACLE_R3_NONPTRACE_FOCAL_NOT_SEALED" else 1
    except Exception as exc:
        sys.stderr.write(f"{getattr(exc, 'code', 'AUTHORITY_ROOT')}: P1350 R3 checker {type(exc).__name__}\n"); return 2


if __name__ == "__main__":
    raise SystemExit(main())
