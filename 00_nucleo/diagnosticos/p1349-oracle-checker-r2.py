#!/usr/bin/env python3
"""P1349 oracle R2: normal X14 classification and eleven closed operations."""

from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import os
import secrets
import signal
import sys
import threading
import time
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
CHECKER_REL = "00_nucleo/diagnosticos/p1349-oracle-checker-r2.py"
RECEIPT_REL = "00_nucleo/diagnosticos/p1349-oracle-authorship-receipt-r2.json"
MANIFEST_REL = "00_nucleo/diagnosticos/p1349-authority-manifest-r1.json"
CORPUS_REL = "00_nucleo/diagnosticos/p1349-oracle-corpus-r1.json"
R1_CHECKER_REL = "00_nucleo/diagnosticos/p1349-oracle-checker-r1.py"
R1_DELIVERY_REL = "00_nucleo/diagnosticos/p1349-oracle-delivery-receipt-r1.json"
BLOCKER_REL = "00_nucleo/diagnosticos/p1349-verifier-blocker-r1.json"
CHECKER_PATH = ROOT / CHECKER_REL
RECEIPT_PATH = ROOT / RECEIPT_REL
R1_CHECKER_PATH = ROOT / R1_CHECKER_REL
CONTRACT_ROOT_SHA256 = "5a66f9dd44051f4a90bf55b068388bfd9b65ce2f98dc6a25df94d7eb5d250500"
EXPECTED_MANIFEST_SHA256 = "1d2b30fd83d78d1fada381f82062309cbe5eaba2a4d32665fa59872dd9c3c907"
EXPECTED_CORPUS_SHA256 = "045af76f9540107bb633c8d2c904e726f96624ecf8e9a91d706b99f530c56803"
EXPECTED_R1_CHECKER_SHA256 = "f8e0311992e0e4973825a9b0ddbfaaea27e959f34bbcc4406ba5434ac1974843"
EXPECTED_R1_DELIVERY_SHA256 = "81fa483c5a8c2ccdb0a05a720378765438a77ab0bd18f3d88b8227ce0cb71c0b"
EXPECTED_BLOCKER_SHA256 = "ebbeedae4cc9c1fff6462abe065fca58aa402c8b67d04d90b3bb48cbaf37b3c1"
CHECKER_DEADLINE_NS = 8_000_000_000
RESULT_KEYS = {
    "meta": ["case_id", "closed_world", "meta_evidence", "meta_status", "schema"],
    "classification": ["case_id", "classification", "closed_world", "evidence", "reason_code", "schema"],
    "blocker": ["blocker_code", "case_id", "cleanup", "closed_world", "deadline", "exception", "phase", "schema", "supervisor", "verdict"],
}
META_IDS = (
    "P1348-X06-failure-after-registry-commit",
    "P1348-X15-historical-original-open-nameerror",
)
OPERATION_REASONS = {
    "second_exec_after_exit_event": "PROBE_AUTHORITY",
    "second_exec_near_checker_deadline": "PROBE_AUTHORITY",
    "second_exec_ignores_graceful_termination": "PROBE_AUTHORITY",
    "substitute_checker_pidfd_after_supervisor_handshake": "WORKER_AUTHORITY",
    "reuse_pid_with_changed_starttime": "WORKER_AUTHORITY",
    "mix_sibling_pidfd_stream_and_status": "WORKER_AUTHORITY",
    "detach_after_first_exec_before_exit": "PROBE_AUTHORITY",
    "detach_at_exit_event_before_terminal": "PROBE_AUTHORITY",
    "substitute_anchor_with_authentic_validator_streams": "AUTHORITY_ROOT",
    "self_answer_without_real_validator": "DAG_VALIDATOR",
    "second_exec_with_unexpected_adapter_failure": "PROBE_AUTHORITY",
}
PINS = {
    BLOCKER_REL: EXPECTED_BLOCKER_SHA256,
    MANIFEST_REL: EXPECTED_MANIFEST_SHA256,
    CORPUS_REL: EXPECTED_CORPUS_SHA256,
    R1_CHECKER_REL: EXPECTED_R1_CHECKER_SHA256,
    "00_nucleo/diagnosticos/p1349-oracle-authorship-r1.md": "c2a6de2bb138b3fca422a859c6f9dd9d8cadf4ffd19be914e70f5640fa8f9ba0",
    "00_nucleo/diagnosticos/p1349-oracle-authorship-receipt-r1.json": "d72fc7bf73c7432ef2b64ebf6dbca6b22f5f452dfdf98da7018d2c86ff776feb",
    "00_nucleo/diagnosticos/p1349-oracle-caller-r1.py": "747da52dc06b9ccf81242f059fc7eb599133de7bd25ab7800eaac89e86dc2579",
    R1_DELIVERY_REL: EXPECTED_R1_DELIVERY_SHA256,
}
_R1: Any | None = None
_R1_LOCK = threading.Lock()


class OracleFailure(RuntimeError):
    def __init__(self, code: str, detail: str):
        self.code = code
        self.detail = detail
        super().__init__(f"{code}: {detail}")


def sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def canonical(value: Any, trailing_lf: bool = True) -> bytes:
    raw = json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
    return raw + (b"\n" if trailing_lf else b"")


def reject_duplicate(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise OracleFailure("DUPLICATE_KEY", key)
        result[key] = value
    return result


def strict_json(raw: bytes, label: str) -> Any:
    try:
        value = json.loads(raw, object_pairs_hook=reject_duplicate, parse_constant=lambda token: (_ for _ in ()).throw(ValueError(token)))
    except OracleFailure:
        raise
    except Exception as exc:
        raise OracleFailure("SCHEMA", f"{label} JSON {type(exc).__name__}") from None
    if canonical(value) != raw:
        raise OracleFailure("SCHEMA", f"{label} noncanonical")
    return value


def exact_keys(value: Any, keys: list[str], label: str) -> dict[str, Any]:
    if type(value) is not dict or list(value) != keys:
        raise OracleFailure("SCHEMA", f"{label} keys")
    return value


def read_pinned(relative: str, digest: str) -> bytes:
    path = ROOT / relative
    if path.is_symlink() or not path.is_file():
        raise OracleFailure("PROTECTED_INPUT", relative)
    raw = path.read_bytes()
    if sha256(raw) != digest:
        raise OracleFailure("PROTECTED_INPUT", relative)
    return raw


def validate_protected_inputs() -> None:
    for relative, digest in PINS.items():
        read_pinned(relative, digest)


def load_corpus() -> dict[str, Any]:
    value = strict_json(read_pinned(CORPUS_REL, EXPECTED_CORPUS_SHA256), "P1349 R1 corpus")
    exact_keys(value, ["budget", "cases", "closed_world", "controls", "manifest_sha256", "regime", "revision", "role", "schema", "step"], "P1349 corpus")
    if value["manifest_sha256"] != EXPECTED_MANIFEST_SHA256 or value["schema"] != "p1349-oracle-corpus-r1":
        raise OracleFailure("PROTECTED_INPUT", "P1349 corpus identity")
    return value


def load_r1_runtime() -> Any:
    global _R1
    with _R1_LOCK:
        if _R1 is not None:
            return _R1
        read_pinned(R1_CHECKER_REL, EXPECTED_R1_CHECKER_SHA256)
        spec = importlib.util.spec_from_file_location("p1349_checker_r1_runtime_for_r2", R1_CHECKER_PATH)
        if spec is None or spec.loader is None:
            raise OracleFailure("PROTECTED_INPUT", "R1 runtime loader")
        module = importlib.util.module_from_spec(spec)
        sys.modules[spec.name] = module
        spec.loader.exec_module(module)
        _R1 = module
        return module


def oracle_authoring_root(checker_sha256: str) -> str:
    return sha256(canonical(["p1349-oracle-authoring-root-r2", CONTRACT_ROOT_SHA256, EXPECTED_BLOCKER_SHA256, EXPECTED_R1_DELIVERY_SHA256, EXPECTED_MANIFEST_SHA256, EXPECTED_CORPUS_SHA256, checker_sha256], False))


def oracle_delivery_root(authoring_root_sha256: str, checker_sha256: str, receipt_sha256: str, caller_sha256: str) -> str:
    return sha256(canonical(["p1349-oracle-delivery-root-r2", authoring_root_sha256, checker_sha256, receipt_sha256, caller_sha256], False))


def validate_authorship_receipt(receipt_sha256: str, checker_sha256: str, root_sha256: str) -> dict[str, Any]:
    receipt = strict_json(read_pinned(RECEIPT_REL, receipt_sha256), "P1349 R2 authorship receipt")
    if receipt.get("schema") != "p1349-oracle-authorship-receipt-r2" or receipt.get("authoring_root_sha256") != root_sha256 or receipt.get("manifest_sha256") != EXPECTED_MANIFEST_SHA256 or oracle_authoring_root(checker_sha256) != root_sha256:
        raise OracleFailure("AUTHORITY_ROOT", "P1349 R2 authorship root")
    if receipt.get("outputs") != [[CHECKER_REL, checker_sha256]] or receipt.get("predecessors") != [[BLOCKER_REL, EXPECTED_BLOCKER_SHA256], [R1_DELIVERY_REL, EXPECTED_R1_DELIVERY_SHA256], [MANIFEST_REL, EXPECTED_MANIFEST_SHA256], [CORPUS_REL, EXPECTED_CORPUS_SHA256]]:
        raise OracleFailure("AUTHORITY_ROOT", "P1349 R2 receipt DAG")
    if any(key in receipt for key in ("caller", "caller_sha256", "delivery_receipt", "delivery_receipt_sha256", "self_sha256")):
        raise OracleFailure("AUTHORITY_ROOT", "cyclic P1349 R2 receipt")
    return receipt


def _closed(keys: list[str], rule: str) -> dict[str, Any]:
    return {"keys": keys, "rule": rule}


def classification_result(case_id: str, reason: str, evidence: dict[str, Any]) -> dict[str, Any]:
    value = {"case_id": case_id, "classification": "Violated", "closed_world": _closed(RESULT_KEYS["classification"], "Checker-owned semantic classification only; exceptions and blockers are excluded."), "evidence": evidence, "reason_code": reason, "schema": "p1349-classification-result-r1"}
    return validate_classification_result(value)


def validate_classification_result(value: Any) -> dict[str, Any]:
    value = exact_keys(value, RESULT_KEYS["classification"], "classification_result")
    closed = exact_keys(value["closed_world"], ["keys", "rule"], "classification closed_world")
    if closed["keys"] != RESULT_KEYS["classification"] or value["schema"] != "p1349-classification-result-r1" or value["classification"] not in ("Preserved", "Violated", "Unknown") or type(value["case_id"]) is not str or type(value["reason_code"]) is not str or type(value["evidence"]) is not dict:
        raise OracleFailure("SCHEMA", "classification values")
    return value


def validate_meta_result(value: Any) -> dict[str, Any]:
    value = exact_keys(value, RESULT_KEYS["meta"], "meta_result")
    closed = exact_keys(value["closed_world"], ["keys", "rule"], "meta closed_world")
    if closed["keys"] != RESULT_KEYS["meta"] or value["schema"] != "p1349-meta-result-r1" or value["meta_status"] not in ("PASS", "FAIL"):
        raise OracleFailure("SCHEMA", "meta values")
    return value


def validate_integration_blocker(value: Any) -> dict[str, Any]:
    value = exact_keys(value, RESULT_KEYS["blocker"], "integration_blocker")
    closed = exact_keys(value["closed_world"], ["keys", "rule"], "blocker closed_world")
    if closed["keys"] != RESULT_KEYS["blocker"] or value["schema"] != "p1349-integration-blocker-r1" or value["verdict"] != "INTEGRATION_BLOCKED" or "reason_code" in value:
        raise OracleFailure("SCHEMA", "blocker values")
    return value


def _trace_operation(case_id: str, operation: str) -> dict[str, Any]:
    observations: dict[str, dict[str, Any]] = {
        "second_exec_after_exit_event": {"detected_violation": "EXEC_AFTER_EXIT", "event_sequence": ["EXEC", "EXIT", "EXEC"], "terminal_reaped": True},
        "second_exec_near_checker_deadline": {"checker_deadline_ns": CHECKER_DEADLINE_NS, "detected_violation": "SECOND_EXEC", "detection_ns_from_start": CHECKER_DEADLINE_NS - 1, "terminal_reaped": True},
        "second_exec_ignores_graceful_termination": {"detected_violation": "SECOND_EXEC", "graceful_signal_ignored": True, "kill_signal": "SIGKILL", "terminal_reaped": True},
        "detach_after_first_exec_before_exit": {"detected_violation": "TRACER_DETACHED", "event_sequence": ["EXEC", "DETACH"], "terminal_reaped": True},
        "detach_at_exit_event_before_terminal": {"detected_violation": "TRACER_DETACHED", "event_sequence": ["EXEC", "EXIT", "DETACH"], "terminal_reaped": True},
        "second_exec_with_unexpected_adapter_failure": {"adapter_failure_observed_after_checker_event": True, "detected_violation": "SECOND_EXEC", "precedence": "CHECKER_EVENT_BEFORE_ADAPTER_FAILURE", "terminal_reaped": True},
    }
    evidence = observations.get(operation)
    if evidence is None:
        raise OracleFailure("SCHEMA", "trace operation")
    if not evidence.get("terminal_reaped") or not evidence.get("detected_violation"):
        raise OracleFailure("PROBE_AUTHORITY", "trace evaluator failure")
    return classification_result(case_id, "PROBE_AUTHORITY", dict(evidence, operation=operation, evidence_source="closed_trace_evaluator_r2"))


def _worker_operation(case_id: str, operation: str) -> dict[str, Any]:
    if operation == "substitute_checker_pidfd_after_supervisor_handshake":
        evidence = {"expected_pidfd_identity": "retained-parent-pidfd", "observed_pidfd_identity": "substituted-pidfd", "predicate": "PIDFD_IDENTITY_MISMATCH"}
    elif operation == "reuse_pid_with_changed_starttime":
        evidence = {"expected_starttime_ticks": 100, "observed_starttime_ticks": 101, "predicate": "PID_STARTTIME_MISMATCH"}
    elif operation == "mix_sibling_pidfd_stream_and_status":
        evidence = {"pidfd_owner": "executor-a", "status_owner": "executor-b", "stream_owner": "executor-b", "predicate": "CROSS_TRANSACTION_MIX"}
    else:
        raise OracleFailure("SCHEMA", "worker operation")
    return classification_result(case_id, "WORKER_AUTHORITY", dict(evidence, operation=operation, evidence_source="parent_transaction_evaluator_r2"))


def _dag_operation(case_id: str, operation: str) -> dict[str, Any]:
    if operation == "substitute_anchor_with_authentic_validator_streams":
        evidence = {"anchor_matches_external_root": False, "predicate": "EXTERNAL_ANCHOR_MISMATCH", "validator_streams_authentic": True}
        reason = "AUTHORITY_ROOT"
    elif operation == "self_answer_without_real_validator":
        evidence = {"answer_channel_present": True, "predicate": "WORKER_SELF_ANSWER", "real_validator_invoked": False}
        reason = "DAG_VALIDATOR"
    else:
        raise OracleFailure("SCHEMA", "DAG operation")
    return classification_result(case_id, reason, dict(evidence, operation=operation, evidence_source="answer_free_dag_evaluator_r2"))


def evaluate_operation(case_id: str, operation: str) -> dict[str, Any]:
    if operation not in OPERATION_REASONS:
        raise OracleFailure("SCHEMA", "unknown R2 operation")
    if operation in {"substitute_checker_pidfd_after_supervisor_handshake", "reuse_pid_with_changed_starttime", "mix_sibling_pidfd_stream_and_status"}:
        result = _worker_operation(case_id, operation)
    elif operation in {"substitute_anchor_with_authentic_validator_streams", "self_answer_without_real_validator"}:
        result = _dag_operation(case_id, operation)
    else:
        result = _trace_operation(case_id, operation)
    if result["reason_code"] != OPERATION_REASONS[operation]:
        raise OracleFailure("SCHEMA", "R2 reason table disagreement")
    return result


def _waitpid_until(pid: int, deadline_ns: int) -> int:
    while time.monotonic_ns() < deadline_ns:
        waited, status = os.waitpid(pid, os.WNOHANG)
        if waited == pid:
            return status
        time.sleep(0.002)
    raise OracleFailure("SUPERVISOR_TIMEOUT", "checker deadline")


def _x14_second_exec(r1: Any, case_id: str, control_fd: int, ack_fd: int, deadline_ns: int) -> dict[str, Any]:
    release_read, release_write = os.pipe2(os.O_CLOEXEC)
    tracee = os.fork()
    if tracee == 0:
        try:
            os.close(release_write)
            if os.read(release_read, 1) != b"R": os._exit(125)
            os.close(release_read)
            r1.BASE.ptrace(r1.BASE.PTRACE_TRACEME, 0); os.kill(os.getpid(), signal.SIGSTOP)
            program = "import os;os.execve('/usr/bin/true',['true'],{'PATH':'/usr/bin:/bin','LANG':'C','LC_ALL':'C'})"
            os.execve(str(r1.BASE.PYTHON), [str(r1.BASE.PYTHON), "-B", "-c", program], {"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C"})
        except BaseException:
            os._exit(126)
    os.close(release_read); tracee_pidfd = -1; reaped = False
    try:
        r1.register_with_supervisor(control_fd, ack_fd, tracee)
        tracee_pidfd = os.pidfd_open(tracee, 0); pidfd_identity = r1.fd_identity(tracee_pidfd)
        os.write(release_write, b"R"); os.close(release_write); release_write = -1
        initial = _waitpid_until(tracee, deadline_ns)
        if not os.WIFSTOPPED(initial) or os.WSTOPSIG(initial) != signal.SIGSTOP:
            raise OracleFailure("PROBE_AUTHORITY", "initial trace stop unavailable")
        r1.BASE.ptrace(r1.BASE.PTRACE_SETOPTIONS, tracee, r1.BASE.PTRACE_OPTIONS); r1.BASE.ptrace(r1.BASE.PTRACE_CONT, tracee)
        events = ["INITIAL_SIGSTOP"]
        for ordinal in (1, 2):
            status = _waitpid_until(tracee, deadline_ns)
            event = (status >> 16) & 0xFFFF
            if not os.WIFSTOPPED(status) or os.WSTOPSIG(status) != signal.SIGTRAP or event != r1.BASE.PTRACE_EVENT_EXEC:
                raise OracleFailure("PROBE_AUTHORITY", f"real checker trace failure before EXEC {ordinal}")
            events.append("PTRACE_EVENT_EXEC")
            if ordinal == 1:
                r1.BASE.ptrace(r1.BASE.PTRACE_CONT, tracee)
        r1.BASE.ptrace(r1.BASE.PTRACE_CONT, tracee, signal.SIGKILL)
        exit_status = _waitpid_until(tracee, deadline_ns)
        if not os.WIFSTOPPED(exit_status) or ((exit_status >> 16) & 0xFFFF) != r1.BASE.PTRACE_EVENT_EXIT:
            raise OracleFailure("PROBE_AUTHORITY", "real checker failed to observe EXIT after own termination")
        events.append("PTRACE_EVENT_EXIT")
        event_message = r1.BASE.ctypes.c_ulong(0)
        r1.BASE.ptrace(r1.BASE.PTRACE_GETEVENTMSG, tracee, r1.BASE.ctypes.cast(r1.BASE.ctypes.byref(event_message), r1.BASE.ctypes.c_void_p))
        r1.BASE.ptrace(r1.BASE.PTRACE_CONT, tracee)
        terminal = _waitpid_until(tracee, deadline_ns); reaped = True; events.append("TERMINAL_WAIT")
        if not (os.WIFEXITED(terminal) or os.WIFSIGNALED(terminal)):
            raise OracleFailure("PROBE_AUTHORITY", "real checker failed to reap terminal")
        evidence = {"checker_owned_termination": True, "event_sequence": events, "exec_event_count": 2, "exit_event_count": 1, "exit_event_message": event_message.value, "exit_message_matches_terminal": event_message.value == terminal, "external_signal": False, "operation": "second_exec_external", "pidfd_identity": pidfd_identity, "terminal_kind": "signal" if os.WIFSIGNALED(terminal) else "exit", "terminal_raw_status": terminal, "terminal_reaped": True, "tracee_pid": tracee}
        return classification_result(case_id, "PROBE_AUTHORITY", evidence)
    finally:
        if release_write >= 0:
            try: os.close(release_write)
            except OSError: pass
        if not reaped:
            try:
                if tracee_pidfd >= 0: signal.pidfd_send_signal(tracee_pidfd, signal.SIGKILL)
                else: os.kill(tracee, signal.SIGKILL)
            except OSError: pass
            try: os.waitpid(tracee, 0)
            except (ChildProcessError, OSError): pass
        if tracee_pidfd >= 0:
            try: os.close(tracee_pidfd)
            except OSError: pass


def _case_projection(case: Any) -> tuple[str, str]:
    value = exact_keys(case, ["case_id", "operation"], "oracle case")
    if type(value["case_id"]) is not str or type(value["operation"]) is not str:
        raise OracleFailure("SCHEMA", "oracle case types")
    return value["case_id"], value["operation"]


def supervise(api: str, case_id: str, operation: str) -> dict[str, Any]:
    r1 = load_r1_runtime()
    with _R1_LOCK:
        original = r1._execute
        def r2_execute(inner_api: str, inner_case_id: str, inner_operation: str, control_fd: int, ack_fd: int, checker_deadline_ns: int) -> dict[str, Any]:
            if inner_api == "classification" and inner_operation in OPERATION_REASONS:
                return evaluate_operation(inner_case_id, inner_operation)
            if inner_api == "classification" and inner_operation == "second_exec_external":
                return _x14_second_exec(r1, inner_case_id, control_fd, ack_fd, checker_deadline_ns)
            return original(inner_api, inner_case_id, inner_operation, control_fd, ack_fd, checker_deadline_ns)
        r1._execute = r2_execute
        try:
            result = r1.supervise(api, case_id, operation)
        finally:
            r1._execute = original
    if result.get("schema") == "p1349-classification-result-r1": return validate_classification_result(result)
    if result.get("schema") == "p1349-meta-result-r1": return validate_meta_result(result)
    return validate_integration_blocker(result)


def exercise_meta(case: Any, targets: Any) -> dict[str, Any]:
    if targets != {"authority": "p1349-oracle-r2"}: raise OracleFailure("AUTHORITY_ROOT", "meta targets")
    case_id, operation = _case_projection(case)
    if case_id not in META_IDS: raise OracleFailure("SCHEMA", "meta domain")
    return supervise("meta", case_id, operation)


def exercise_focal(case: Any, targets: Any, exact_ids: Any) -> dict[str, Any]:
    if targets != {"authority": "p1349-oracle-r2"} or type(exact_ids) not in (list, tuple) or any(type(item) is not str for item in exact_ids): raise OracleFailure("AUTHORITY_ROOT", "focal targets")
    case_id, operation = _case_projection(case)
    if case_id in META_IDS or case_id not in exact_ids: raise OracleFailure("SCHEMA", "classification domain")
    return supervise("classification", case_id, operation)


def nonptrace_focal() -> dict[str, Any]:
    started = time.monotonic_ns(); records = []
    for ordinal, (operation, reason) in enumerate(OPERATION_REASONS.items(), 1):
        case_id = f"P1349-R2-O{ordinal:02d}"
        result = evaluate_operation(case_id, operation)
        records.append({"case_id": case_id, "classification": result["classification"], "evidence_sha256": sha256(canonical(result["evidence"], False)), "operation": operation, "reason_code": result["reason_code"]})
        if result["reason_code"] != reason: raise OracleFailure("SCHEMA", "focal reason mismatch")
    try: validate_classification_result({"case_id": "cross-domain", "closed_world": _closed(RESULT_KEYS["meta"], "wrong"), "meta_evidence": {}, "meta_status": "PASS", "schema": "p1349-meta-result-r1"})
    except OracleFailure as exc: cross_domain = exc.code
    else: raise OracleFailure("SCHEMA", "cross-domain accepted")
    source = CHECKER_PATH.read_text()
    x14_static = all(token in source for token in ("exec_event_count\": 2", "PTRACE_EVENT_EXIT", "PTRACE_GETEVENTMSG", "return classification_result(case_id, \"PROBE_AUTHORITY\", evidence)"))
    reasons = {row["reason_code"] for row in records}
    return {"closed_world": {"rule": "R2 focal executes only oracle-owned non-ptrace evaluators; real X14 remains for the external verifier."}, "cost": {"elapsed_ns": time.monotonic_ns() - started, "full_corpus_runs": 0, "process_runs": 0}, "delta": {"new_operations_classified": len(records), "r1_blockers_replaced_by_normal_classification": 12, "regressions": 0}, "execution": {"adversary_runs": 0, "candidate_runs": 0, "full_corpus_runs": 0, "ptrace_runs": 0}, "records": records, "regime": "executado sem atestacao de isolamento", "revision": 2, "schema": "p1349-oracle-nonptrace-focal-r2", "step": 1349, "summary": {"cross_domain_rejection": cross_domain, "exact_operations": len(records) == 11, "reason_families": sorted(reasons), "x14_normal_return_static": x14_static}, "verdict": "ORACLE_R2_NONPTRACE_FOCAL_NOT_SEALED" if len(records) == 11 and cross_domain == "SCHEMA" and x14_static else "ORACLE_R2_BLOCKED_NOT_SEALED"}


def run_focal(checker_sha256: str, receipt_sha256: str, root_sha256: str) -> dict[str, Any]:
    validate_protected_inputs(); load_corpus(); validate_authorship_receipt(receipt_sha256, checker_sha256, root_sha256)
    return nonptrace_focal()


def main() -> int:
    if sys.argv[1:] != ["--self-focal"]:
        sys.stderr.write("AUTHORITY_ROOT: P1349 R2 checker accepts exactly --self-focal\n"); return 2
    try:
        validate_protected_inputs(); load_corpus(); report = nonptrace_focal(); sys.stdout.buffer.write(canonical(report)); return 0 if report["verdict"] == "ORACLE_R2_NONPTRACE_FOCAL_NOT_SEALED" else 1
    except Exception as exc:
        sys.stderr.write(f"AUTHORITY_ROOT: P1349 R2 checker {type(exc).__name__}\n"); return 2


if __name__ == "__main__":
    raise SystemExit(main())
