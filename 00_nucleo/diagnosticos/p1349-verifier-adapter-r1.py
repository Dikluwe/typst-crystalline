#!/usr/bin/env python3
"""P1349 operational adapter authorship; never a verdict authority.

The two public APIs preserve the frozen domain split.  Adapter exceptions and
unavailable oracle operations always become a closed integration blocker; they
are never converted to a case reason.
"""

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
META_IDS = (
    "P1348-X06-failure-after-registry-commit",
    "P1348-X15-historical-original-open-nameerror",
)
P1348_ADAPTER = DIAG / "p1348-verifier-adapter-r1.py"
P1348_ADAPTER_SHA256 = "bcec1c05eed907ec3ea8f45c55940ee5244fe9498c731e66458b9817d946edfa"
P1348_TARGETS = {
    "checker": (DIAG / "p1348-oracle-checker-r1.py", "fd1c4a0e1601ea817ffb92ad6723367083ac9f5c7a0f9bcecb155e9148764249"),
    "caller": (DIAG / "p1348-oracle-caller-r1.py", "2ebb7f9497b3c1ef9088b06b5e32704df464296f45c831b18c695c08f72e1e43"),
    "delivery_receipt": (DIAG / "p1348-oracle-delivery-receipt-r1.json", "c06a0c78369f33e677830782a6fcd20ada7dc6ae037b90b6160b5517390f7040"),
}

# Every P1349 supervisor operation is sent through checker.supervise.
# Only X14 has a frozen checker implementation.  The remaining eleven stay
# explicit so the checker returns a terminal blocker instead of adapter-made
# classification when its operation surface is absent.
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

# The inherited operations are delegated byte-for-byte to the pinned P1348
# adapter.  X06/X15 are deliberately absent because their P1349 operations live
# only in META_OPERATION_MAP.
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

META_OPERATION_MAP = {
    "postcommit_fault_in_disposable_process": "postcommit_failure",
    "historical_nameerror_in_disposable_process": "historical_nameerror",
}

_CONTEXTS: dict[tuple[str, ...], tuple[Any, Any, str]] = {}


def sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


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


def _canonical(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


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
    checker_path = Path(targets.checker)
    caller_path = Path(targets.caller)
    delivery_path = Path(targets.delivery_receipt)
    _pinned(checker_path, targets.checker_sha256, "P1349 checker")
    _pinned(caller_path, targets.caller_sha256, "P1349 caller")
    delivery = _strict_json(_pinned(delivery_path, targets.delivery_receipt_sha256, "P1349 delivery"), "P1349 delivery")
    if delivery.get("checker") != ["00_nucleo/diagnosticos/p1349-oracle-checker-r1.py", targets.checker_sha256]:
        raise RuntimeError("AUTHORITY_ROOT: P1349 delivered checker")
    if delivery.get("caller") != ["00_nucleo/diagnosticos/p1349-oracle-caller-r1.py", targets.caller_sha256]:
        raise RuntimeError("AUTHORITY_ROOT: P1349 delivered caller")
    receipt = delivery.get("oracle_receipt")
    if type(receipt) is not list or len(receipt) != 2 or type(receipt[1]) is not str:
        raise RuntimeError("AUTHORITY_ROOT: P1349 delivered receipt")
    caller = _load(caller_path.resolve(), f"p1349_adapter_caller_{targets.caller_sha256[:16]}")
    checker = caller.validate_chain()
    loaded = Path(checker.__file__).resolve()
    if loaded != checker_path.resolve() or sha256(loaded.read_bytes()) != targets.checker_sha256:
        raise RuntimeError("AUTHORITY_ROOT: P1349 caller alternate checker")
    wanted_root = checker.oracle_delivery_root(
        delivery["authoring_root_sha256"], targets.checker_sha256, receipt[1], targets.caller_sha256
    )
    if delivery.get("delivery_root_sha256") != wanted_root:
        raise RuntimeError("AUTHORITY_ROOT: P1349 delivery root")
    _pinned(P1348_ADAPTER, P1348_ADAPTER_SHA256, "P1348 adapter")
    base_adapter = _load(P1348_ADAPTER, f"p1348_adapter_pinned_by_p1349_{P1348_ADAPTER_SHA256[:16]}")
    binding = sha256(("\0".join((*key, wanted_root, P1348_ADAPTER_SHA256))).encode())
    _CONTEXTS[key] = checker, base_adapter, binding
    return _CONTEXTS[key]


def _case(case: Any, domain: str) -> tuple[str, str]:
    if type(case) is not dict or case.get("domain") != domain:
        raise RuntimeError(f"SCHEMA: {domain} case")
    case_id, operation = case.get("case_id"), case.get("adapter_operation")
    if type(case_id) is not str or type(operation) is not str:
        raise RuntimeError(f"SCHEMA: {domain} identity")
    return case_id, operation


def _empty_cleanup() -> dict[str, Any]:
    return {"adapter_restored": True, "failures": [], "process_root_removed": True}


def _blocker(case_id: str, phase: str, exc: BaseException, code: str = "ADAPTER_EXCEPTION") -> dict[str, Any]:
    keys = ["blocker_code", "case_id", "cleanup", "closed_world", "deadline", "exception", "phase", "schema", "supervisor", "verdict"]
    return {
        "blocker_code": code,
        "case_id": case_id,
        "cleanup": _empty_cleanup(),
        "closed_world": {"keys": keys, "rule": "Terminal adapter/control-plane failure; never classification, reason or score."},
        "deadline": {"checker_deadline_ns_from_start": 8_000_000_000, "supervisor_deadline_ns_from_start": 12_000_000_000},
        "exception": {"message_sha256": sha256(str(exc).encode()), "type": type(exc).__name__},
        "phase": phase,
        "schema": "p1349-integration-blocker-r1",
        "supervisor": {"adapter_sha256": sha256(Path(__file__).read_bytes()), "external_signal_sent": False, "regime": REGIME},
        "verdict": "INTEGRATION_BLOCKED",
    }


def _tree_empty(evidence: dict[str, Any]) -> bool:
    cleanup = evidence.get("cleanup")
    return bool(
        type(cleanup) is dict
        and cleanup.get("descendants_terminal") is True
        and cleanup.get("group_esrch") is True
        and cleanup.get("reaped_executor") is True
        and cleanup.get("pidfds_closed") is True
        and cleanup.get("process_root_removed") is True
    )


def _translate_meta(checker: Any, case_id: str, value: dict[str, Any]) -> dict[str, Any]:
    if value.get("schema") == "p1349-integration-blocker-r1":
        return value
    checker.validate_meta_result(value)
    raw = value["meta_evidence"]
    if case_id == META_IDS[0]:
        digest_unchanged = raw.get("registry_sha256_before") == raw.get("registry_sha256_after")
        postconditions = {
            "identity_state": raw.get("identity_state"),
            "process_tree_empty": _tree_empty(raw),
            "registry_commit_ordinal_unchanged": value["meta_status"] == "PASS" and digest_unchanged,
            "registry_digest_unchanged": digest_unchanged,
            "reopened": raw.get("reopened"),
            "restoration_identity": raw.get("restoration_identity"),
            "second_acceptance": raw.get("second_acceptance"),
        }
        observed = {
            "blocker_code": raw.get("blocker_code"),
            "outcome_kind": raw.get("observed_outcome_kind"),
            "verdict": raw.get("observed_verdict"),
        }
        passed = value["meta_status"] == "PASS" and observed == {
            "blocker_code": "ADAPTER_EXCEPTION", "outcome_kind": "integration_blocker",
            "verdict": "ADAPTER_EXCEPTION_BLOCKS_INTEGRATION",
        } and postconditions == {
            "identity_state": "CONSUMED", "process_tree_empty": True,
            "registry_commit_ordinal_unchanged": True, "registry_digest_unchanged": True,
            "reopened": False, "restoration_identity": True, "second_acceptance": False,
        }
    elif case_id == META_IDS[1]:
        restored = raw.get("restoration_attempted") is True and raw.get("open_identity_before_equals_after") is True
        observed = {
            "blocker_code": raw.get("blocker_code"),
            "exception_type": raw.get("exception_type"),
            "outcome_kind": raw.get("observed_outcome_kind"),
            "verdict": raw.get("observed_verdict"),
        }
        postconditions = {
            "classification_absent": raw.get("classification_absent"),
            "open_captured_before_patching": restored,
            "open_identity_before_equals_after": raw.get("open_identity_before_equals_after"),
            "process_tree_empty": _tree_empty(raw),
            "reason_code_absent": raw.get("reason_code_absent"),
            "restoration_completed": restored,
        }
        passed = value["meta_status"] == "PASS" and observed == {
            "blocker_code": "ADAPTER_EXCEPTION", "exception_type": "NameError",
            "outcome_kind": "integration_blocker", "verdict": "ADAPTER_EXCEPTION_BLOCKS_INTEGRATION",
        } and all(postconditions.values())
    else:
        raise RuntimeError("SCHEMA: meta allowlist")
    return checker.meta_result(
        case_id, "PASS" if passed else "FAIL",
        {"observed_blocker": observed, "postconditions": postconditions},
    )


def _p1348_targets() -> Any:
    for label, (path, digest) in P1348_TARGETS.items():
        _pinned(path, digest, f"P1348 {label}")
    return SimpleNamespace(
        checker=P1348_TARGETS["checker"][0], checker_sha256=P1348_TARGETS["checker"][1],
        caller=P1348_TARGETS["caller"][0], caller_sha256=P1348_TARGETS["caller"][1],
        delivery_receipt=P1348_TARGETS["delivery_receipt"][0],
        delivery_receipt_sha256=P1348_TARGETS["delivery_receipt"][1],
    )


def _translate_p1348(checker: Any, case_id: str, value: dict[str, Any], binding: str) -> dict[str, Any]:
    if value.get("outcome_kind") == "classification":
        classification, reason = value.get("classification"), value.get("reason_code")
        if classification not in ("Preserved", "Violated", "Unknown") or type(reason) is not str:
            raise RuntimeError("SCHEMA: P1348 classification")
        return checker.classification_result(
            case_id, classification, reason,
            {"p1348_evidence": value.get("evidence"), "p1348_target_binding_sha256": binding},
        )
    if value.get("outcome_kind") == "integration_blocker":
        nested = value.get("blocker") if type(value.get("blocker")) is dict else {}
        return _blocker(case_id, "p1348_adapter", RuntimeError(str(nested.get("exception_type", "P1348 blocker"))))
    raise RuntimeError("SCHEMA: P1348 result domain")


def exercise_meta(case: dict[str, Any], targets: Any) -> dict[str, Any]:
    case_id = case.get("case_id") if type(case) is dict else "<invalid-case>"
    phase = "schema"
    try:
        case_id, operation = _case(case, "meta")
        if case_id not in META_IDS or operation not in META_OPERATION_MAP:
            raise RuntimeError("SCHEMA: meta allowlist")
        if META_IDS.index(case_id) != tuple(META_OPERATION_MAP).index(operation):
            raise RuntimeError("SCHEMA: meta operation binding")
        phase = "setup"
        checker, _base_adapter, _binding = _context(targets)
        phase = "supervisor"
        value = checker.exercise_meta(
            {"case_id": case_id, "operation": META_OPERATION_MAP[operation]},
            {"authority": "p1349-oracle-r1"},
        )
        phase = "meta_translation"
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
        phase = "setup"
        checker, base_adapter, binding = _context(targets)
        if operation in SUPERVISOR_OPERATION_MAP:
            phase = "p1349_supervisor"
            # The frozen checker is the sole source of termination,
            # classification, reason and supervisor/blocker evidence.
            return checker.supervise("classification", case_id, SUPERVISOR_OPERATION_MAP[operation])
        if operation not in INHERITED_OPERATIONS:
            raise RuntimeError(f"SCHEMA: unmapped focal operation:{operation}")
        phase = "p1348_delegate"
        value = base_adapter.exercise(case, _p1348_targets(), "normal")
        return _translate_p1348(checker, case_id, value, binding)
    except BaseException as exc:
        return _blocker(str(case_id), phase, exc)
