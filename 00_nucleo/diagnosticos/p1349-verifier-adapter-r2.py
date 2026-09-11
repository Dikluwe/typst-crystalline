#!/usr/bin/env python3
"""P1349 adapter R2, bound explicitly to the frozen Oracle R2 package."""

from __future__ import annotations

import hashlib
import importlib.util
import json
import sys
from pathlib import Path
from types import SimpleNamespace
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
REGIME = "executado sem atestacao de isolamento"
SEQUENCE_SHA256 = "e958b9c5e939a4b06b9e8594966001004de814e49e0141034d5351f1dbf4a907"
ORACLE_R2 = {
    "checker": (DIAG / "p1349-oracle-checker-r2.py", "999971b4e25c6a44ae76542d0115d8304c3a8f28afbaf26bc1c9347db6edc267"),
    "caller": (DIAG / "p1349-oracle-caller-r2.py", "4445f1a23a66ad2a0c19a67769cfb043541106bb81ded6e1a1e263d0da974c07"),
    "delivery_receipt": (DIAG / "p1349-oracle-delivery-receipt-r2.json", "219f6c23c2ba95592536fb5ab174a0d8f5243c76fab35a4057042f63234a4524"),
}
ORACLE_R2_DELIVERY_ROOT = "5c2b5e15959bfd23f9627d3391e66c0e46c607a962ad0bfc3315770c3c5f4b81"
ADAPTER_R1 = DIAG / "p1349-verifier-adapter-r1.py"
ADAPTER_R1_SHA256 = "db800da93aedf7c34ec3b0c4018661b35d61d2edbe709c3cfb48e8a108caa861"
META_IDS = (
    "P1348-X06-failure-after-registry-commit",
    "P1348-X15-historical-original-open-nameerror",
)
META_OPERATION_MAP = {
    "postcommit_fault_in_disposable_process": "postcommit_failure",
    "historical_nameerror_in_disposable_process": "historical_nameerror",
}
SUPERVISOR_OPERATION_MAP = {
    "inject_second_exec": "second_exec_external",
    "second_exec_after_exit_event": "second_exec_after_exit_event",
    "second_exec_near_checker_deadline": "second_exec_near_checker_deadline",
    "second_exec_ignores_graceful_termination": "second_exec_ignores_graceful_termination",
    "substitute_checker_pidfd_after_supervisor_handshake": "substitute_checker_pidfd_after_supervisor_handshake",
    "reuse_pid_with_changed_starttime": "reuse_pid_with_changed_starttime",
    "mix_sibling_pidfd_stream_and_status": "mix_sibling_pidfd_stream_and_status",
    "detach_after_first_exec_before_exit": "detach_after_first_exec_before_exit",
    "detach_at_exit_event_before_terminal": "detach_at_exit_event_before_terminal",
    "substitute_anchor_with_authentic_validator_streams": "substitute_anchor_with_authentic_validator_streams",
    "self_answer_without_real_validator": "self_answer_without_real_validator",
    "second_exec_with_unexpected_adapter_failure": "second_exec_with_unexpected_adapter_failure",
}
INHERITED_OPERATIONS = {
    "adversarial_image", "alter_validator_stream_with_authority_root", "append_worker_stdout",
    "canonical_dag", "canonical_traced_probe", "canonical_worker", "dag_coordinated_substitution",
    "dag_labels_without_validator", "dag_mutant_anchor", "dag_mutation_accepted_by_real_validator",
    "dag_receipt_answer_channel", "dag_valid_schema_self_answer", "dag_validator_without_parent_process",
    "detach_at_event", "detach_before_event", "detach_before_terminal_wait", "drop_parent_wait",
    "dup_over_after_child_check", "fabricate_terminal_status", "fail_proc_exe", "fail_syscall",
    "fail_then_retry_before_commit", "inject_registry", "inject_worker_pid", "mix_process_evidence",
    "mutate_id_sequence", "p1346_control_projection", "real_dag_validator", "remove_capability",
    "remove_trace_option", "replay_p1346_negative", "replay_worker_challenge", "reuse_terminated_child_pid",
    "second_exec_after_attestation", "stdout_without_child", "substitute_pid_and_pidfd",
    "substitute_proc_exe_image", "substitute_process_identity", "suppress_exec_stop",
    "swap_fd_after_exec_stop", "swap_fd_before_exec", "swap_fd_inside_exec", "swap_validation_slots",
    "truncate_worker_stdout", "validate_same_identity_twice", "worker_exit", "worker_signal", "worker_timeout",
}
_CONTEXTS: dict[tuple[str, ...], tuple[Any, Any, str]] = {}


def sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def _canonical(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def _strict_json(raw: bytes, label: str) -> Any:
    def pairs(items: list[tuple[str, Any]]) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key, value in items:
            if key in result:
                raise RuntimeError(f"DUPLICATE_KEY: {label}:{key}")
            result[key] = value
        return result

    try:
        return json.loads(raw.decode("utf-8", "strict"), object_pairs_hook=pairs)
    except (UnicodeError, json.JSONDecodeError) as exc:
        raise RuntimeError(f"SCHEMA: {label}") from exc


def _pinned(path: Path, expected: str, label: str) -> bytes:
    if path.is_symlink() or not path.is_file():
        raise RuntimeError(f"AUTHORITY_ROOT: {label}:path")
    raw = path.read_bytes()
    if sha256(raw) != expected:
        raise RuntimeError(f"AUTHORITY_ROOT: {label}:sha256")
    return raw


def _load(path: Path, name: str) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"AUTHORITY_ROOT: loader:{path.name}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def _target_key(targets: Any) -> tuple[str, ...]:
    return (
        str(Path(targets.checker).resolve()), str(targets.checker_sha256),
        str(Path(targets.caller).resolve()), str(targets.caller_sha256),
        str(Path(targets.delivery_receipt).resolve()), str(targets.delivery_receipt_sha256),
    )


def _context(targets: Any) -> tuple[Any, Any, str]:
    key = _target_key(targets)
    if key in _CONTEXTS:
        return _CONTEXTS[key]
    supplied = {
        "checker": (Path(targets.checker), str(targets.checker_sha256)),
        "caller": (Path(targets.caller), str(targets.caller_sha256)),
        "delivery_receipt": (Path(targets.delivery_receipt), str(targets.delivery_receipt_sha256)),
    }
    for label, (wanted_path, wanted_digest) in ORACLE_R2.items():
        path, digest = supplied[label]
        if path.resolve() != wanted_path.resolve() or digest != wanted_digest:
            raise RuntimeError(f"AUTHORITY_ROOT: explicit Oracle R2 {label}")
        _pinned(path, digest, f"Oracle R2 {label}")
    delivery = _strict_json(
        ORACLE_R2["delivery_receipt"][0].read_bytes(), "Oracle R2 delivery"
    )
    if delivery.get("checker") != ["00_nucleo/diagnosticos/p1349-oracle-checker-r2.py", ORACLE_R2["checker"][1]]:
        raise RuntimeError("AUTHORITY_ROOT: Oracle R2 delivered checker")
    if delivery.get("caller") != ["00_nucleo/diagnosticos/p1349-oracle-caller-r2.py", ORACLE_R2["caller"][1]]:
        raise RuntimeError("AUTHORITY_ROOT: Oracle R2 delivered caller")
    if delivery.get("delivery_root_sha256") != ORACLE_R2_DELIVERY_ROOT:
        raise RuntimeError("AUTHORITY_ROOT: Oracle R2 declared delivery root")
    receipt = delivery.get("oracle_receipt")
    if type(receipt) is not list or len(receipt) != 2 or type(receipt[1]) is not str:
        raise RuntimeError("AUTHORITY_ROOT: Oracle R2 receipt edge")
    caller = _load(ORACLE_R2["caller"][0], f"p1349_adapter_r2_caller_{ORACLE_R2['caller'][1][:16]}")
    checker = caller.validate_chain()
    loaded = Path(checker.__file__).resolve()
    if loaded != ORACLE_R2["checker"][0].resolve() or sha256(loaded.read_bytes()) != ORACLE_R2["checker"][1]:
        raise RuntimeError("AUTHORITY_ROOT: Oracle R2 caller alternate checker")
    computed_root = checker.oracle_delivery_root(
        delivery["authoring_root_sha256"], ORACLE_R2["checker"][1], receipt[1], ORACLE_R2["caller"][1]
    )
    if computed_root != ORACLE_R2_DELIVERY_ROOT:
        raise RuntimeError("AUTHORITY_ROOT: Oracle R2 computed delivery root")
    _pinned(ADAPTER_R1, ADAPTER_R1_SHA256, "P1349 adapter R1")
    adapter_r1 = _load(ADAPTER_R1, f"p1349_adapter_r1_pinned_by_r2_{ADAPTER_R1_SHA256[:16]}")
    binding = sha256(_canonical([*key, ORACLE_R2_DELIVERY_ROOT, ADAPTER_R1_SHA256]))
    _CONTEXTS[key] = checker, adapter_r1, binding
    return _CONTEXTS[key]


def _case(case: Any, domain: str) -> tuple[str, str]:
    if type(case) is not dict or case.get("domain") != domain:
        raise RuntimeError(f"SCHEMA: {domain} case")
    case_id, operation = case.get("case_id"), case.get("adapter_operation")
    if type(case_id) is not str or type(operation) is not str:
        raise RuntimeError(f"SCHEMA: {domain} identity")
    return case_id, operation


def _blocker(case_id: str, phase: str, exc: BaseException) -> dict[str, Any]:
    keys = ["blocker_code", "case_id", "cleanup", "closed_world", "deadline", "exception", "phase", "schema", "supervisor", "verdict"]
    return {
        "blocker_code": "ADAPTER_EXCEPTION", "case_id": case_id,
        "cleanup": {"adapter_restored": True, "failures": [], "process_root_removed": True},
        "closed_world": {"keys": keys, "rule": "Terminal R2 adapter failure; never classification, reason or score."},
        "deadline": {"checker_deadline_ns_from_start": 8_000_000_000, "supervisor_deadline_ns_from_start": 12_000_000_000},
        "exception": {"message_sha256": sha256(str(exc).encode()), "type": type(exc).__name__},
        "phase": phase, "schema": "p1349-integration-blocker-r1",
        "supervisor": {"adapter_revision": 2, "external_signal_sent": False, "oracle_delivery_root_sha256": ORACLE_R2_DELIVERY_ROOT, "regime": REGIME},
        "verdict": "INTEGRATION_BLOCKED",
    }


def _tree_empty(evidence: dict[str, Any]) -> bool:
    cleanup = evidence.get("cleanup")
    return bool(type(cleanup) is dict and all(cleanup.get(key) is True for key in (
        "descendants_terminal", "group_esrch", "pidfds_closed", "process_root_removed", "reaped_executor"
    )))


def _meta_result(case_id: str, status: str, evidence: dict[str, Any]) -> dict[str, Any]:
    keys = ["case_id", "closed_world", "meta_evidence", "meta_status", "schema"]
    return {"case_id": case_id, "closed_world": {"keys": keys, "rule": "Meta observation only; never semantic classification or score."}, "meta_evidence": evidence, "meta_status": status, "schema": "p1349-meta-result-r1"}


def _translate_meta(checker: Any, case_id: str, value: dict[str, Any]) -> dict[str, Any]:
    if value.get("schema") == "p1349-integration-blocker-r1":
        return checker.validate_integration_blocker(value)
    checker.validate_meta_result(value)
    raw = value["meta_evidence"]
    if case_id == META_IDS[0]:
        digest_unchanged = raw.get("registry_sha256_before") == raw.get("registry_sha256_after")
        observed = {"blocker_code": raw.get("blocker_code"), "outcome_kind": raw.get("observed_outcome_kind"), "verdict": raw.get("observed_verdict")}
        postconditions = {
            "identity_state": raw.get("identity_state"), "process_tree_empty": _tree_empty(raw),
            "registry_commit_ordinal_unchanged": value["meta_status"] == "PASS" and digest_unchanged,
            "registry_digest_unchanged": digest_unchanged, "reopened": raw.get("reopened"),
            "restoration_identity": raw.get("restoration_identity"), "second_acceptance": raw.get("second_acceptance"),
        }
        passed = value["meta_status"] == "PASS" and observed == {"blocker_code": "ADAPTER_EXCEPTION", "outcome_kind": "integration_blocker", "verdict": "ADAPTER_EXCEPTION_BLOCKS_INTEGRATION"} and postconditions == {"identity_state": "CONSUMED", "process_tree_empty": True, "registry_commit_ordinal_unchanged": True, "registry_digest_unchanged": True, "reopened": False, "restoration_identity": True, "second_acceptance": False}
    elif case_id == META_IDS[1]:
        restored = raw.get("restoration_attempted") is True and raw.get("open_identity_before_equals_after") is True
        observed = {"blocker_code": raw.get("blocker_code"), "exception_type": raw.get("exception_type"), "outcome_kind": raw.get("observed_outcome_kind"), "verdict": raw.get("observed_verdict")}
        postconditions = {
            "classification_absent": raw.get("classification_absent"), "open_captured_before_patching": restored,
            "open_identity_before_equals_after": raw.get("open_identity_before_equals_after"),
            "process_tree_empty": _tree_empty(raw), "reason_code_absent": raw.get("reason_code_absent"),
            "restoration_completed": restored,
        }
        passed = value["meta_status"] == "PASS" and observed == {"blocker_code": "ADAPTER_EXCEPTION", "exception_type": "NameError", "outcome_kind": "integration_blocker", "verdict": "ADAPTER_EXCEPTION_BLOCKS_INTEGRATION"} and all(postconditions.values())
    else:
        raise RuntimeError("SCHEMA: meta allowlist")
    result = _meta_result(case_id, "PASS" if passed else "FAIL", {"observed_blocker": observed, "postconditions": postconditions})
    return checker.validate_meta_result(result)


def _r1_targets() -> Any:
    return SimpleNamespace(
        checker=DIAG / "p1349-oracle-checker-r1.py", checker_sha256="f8e0311992e0e4973825a9b0ddbfaaea27e959f34bbcc4406ba5434ac1974843",
        caller=DIAG / "p1349-oracle-caller-r1.py", caller_sha256="747da52dc06b9ccf81242f059fc7eb599133de7bd25ab7800eaac89e86dc2579",
        delivery_receipt=DIAG / "p1349-oracle-delivery-receipt-r1.json", delivery_receipt_sha256="81fa483c5a8c2ccdb0a05a720378765438a77ab0bd18f3d88b8227ce0cb71c0b",
    )


def exercise_meta(case: dict[str, Any], targets: Any) -> dict[str, Any]:
    case_id = case.get("case_id") if type(case) is dict else "<invalid-case>"
    phase = "schema"
    try:
        case_id, operation = _case(case, "meta")
        if case_id not in META_IDS or operation not in META_OPERATION_MAP or META_IDS.index(case_id) != tuple(META_OPERATION_MAP).index(operation):
            raise RuntimeError("SCHEMA: meta allowlist/binding")
        phase = "Oracle R2 setup"
        checker, _adapter_r1, _binding = _context(targets)
        phase = "Oracle R2 meta supervisor"
        value = checker.exercise_meta({"case_id": case_id, "operation": META_OPERATION_MAP[operation]}, {"authority": "p1349-oracle-r2"})
        return _translate_meta(checker, case_id, value)
    except BaseException as exc:
        return _blocker(str(case_id), phase, exc)


def exercise_focal(case: dict[str, Any], targets: Any, exact_ids: list[str]) -> dict[str, Any]:
    case_id = case.get("case_id") if type(case) is dict else "<invalid-case>"
    phase = "schema"
    try:
        case_id, operation = _case(case, "classification")
        if type(exact_ids) not in (list, tuple) or any(type(item) is not str or not item.isascii() for item in exact_ids):
            raise RuntimeError("SCHEMA: exact_ids")
        if len(exact_ids) != 122 or len(exact_ids) != len(set(exact_ids)) or sha256(_canonical(list(exact_ids))) != SEQUENCE_SHA256:
            raise RuntimeError("PROTECTED_INPUT: exact 122 language IDs")
        if case_id in META_IDS:
            raise RuntimeError("SCHEMA: focal domain")
        phase = "Oracle R2 setup"
        checker, adapter_r1, _binding = _context(targets)
        if operation in SUPERVISOR_OPERATION_MAP:
            phase = "Oracle R2 classification supervisor"
            return checker.supervise("classification", case_id, SUPERVISOR_OPERATION_MAP[operation])
        if operation not in INHERITED_OPERATIONS:
            raise RuntimeError(f"SCHEMA: unmapped focal operation:{operation}")
        phase = "pinned adapter R1 inherited route"
        value = adapter_r1.exercise_focal(case, _r1_targets(), list(exact_ids))
        if value.get("schema") == "p1349-classification-result-r1":
            return checker.validate_classification_result(value)
        if value.get("schema") == "p1349-integration-blocker-r1":
            return checker.validate_integration_blocker(value)
        raise RuntimeError("SCHEMA: adapter R1 inherited result")
    except BaseException as exc:
        return _blocker(str(case_id), phase, exc)
