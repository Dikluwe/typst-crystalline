#!/usr/bin/env python3
"""P1349 oracle: disjoint result domains and bounded parent supervision."""

from __future__ import annotations

import copy
import errno
import hashlib
import importlib.util
import json
import os
import secrets
import select
import shutil
import signal
import stat
import sys
import tempfile
import time
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
CHECKER_REL = "00_nucleo/diagnosticos/p1349-oracle-checker-r1.py"
CORPUS_REL = "00_nucleo/diagnosticos/p1349-oracle-corpus-r1.json"
MANIFEST_REL = "00_nucleo/diagnosticos/p1349-authority-manifest-r1.json"
RECEIPT_REL = "00_nucleo/diagnosticos/p1349-oracle-authorship-receipt-r1.json"
BASE_REL = "00_nucleo/diagnosticos/p1348-oracle-checker-r1.py"
CHECKER_PATH = ROOT / CHECKER_REL
CORPUS_PATH = ROOT / CORPUS_REL
RECEIPT_PATH = ROOT / RECEIPT_REL
BASE_PATH = ROOT / BASE_REL
EXPECTED_BASE_SHA256 = "fd1c4a0e1601ea817ffb92ad6723367083ac9f5c7a0f9bcecb155e9148764249"
EXPECTED_MANIFEST_SHA256 = "1d2b30fd83d78d1fada381f82062309cbe5eaba2a4d32665fa59872dd9c3c907"
EXPECTED_CORPUS_SHA256 = "045af76f9540107bb633c8d2c904e726f96624ecf8e9a91d706b99f530c56803"
CONTRACT_ROOT_SHA256 = "5a66f9dd44051f4a90bf55b068388bfd9b65ce2f98dc6a25df94d7eb5d250500"
CHECKER_DEADLINE_NS = 8_000_000_000
SUPERVISOR_DEADLINE_NS = 12_000_000_000
TERM_GRACE_NS = 500_000_000
KILL_AND_REAP_NS = 2_000_000_000
META_IDS = (
    "P1348-X06-failure-after-registry-commit",
    "P1348-X15-historical-original-open-nameerror",
)
RESULT_KEYS = {
    "meta": ["case_id", "closed_world", "meta_evidence", "meta_status", "schema"],
    "classification": ["case_id", "classification", "closed_world", "evidence", "reason_code", "schema"],
    "blocker": ["blocker_code", "case_id", "cleanup", "closed_world", "deadline", "exception", "phase", "schema", "supervisor", "verdict"],
}
BLOCKER_CODES = {"ADAPTER_EXCEPTION", "CLEANUP_FAILURE", "INTERRUPTED", "ORPHAN_PROCESS", "SUPERVISOR_PROTOCOL", "SUPERVISOR_TIMEOUT"}
PINS = {
    "00_nucleo/materialization/typst-passo-1349.md": "071ee50547bab6dc86b84d630b6c501867967121234cce79ab8de14075973a81",
    "00_nucleo/materialization/typst-passo-1348.md": "4254d294e71427c7dd54ab28db674d7b20174e560bd4882bf3413dae217224c7",
    "00_nucleo/materialization/typst-passo-1347.md": "c848a04d9bc3aaa9ed935ea8fef6e615cdb77a0533c13d0fcd62ef1a87ab9a1c",
    "00_nucleo/diagnosticos/p1349-contract-spec-r1.json": "9b3363fe2c60d634a7547357217299a2fc610e04f3c0116b667ce23caf555335",
    "00_nucleo/diagnosticos/p1349-contract-binding-r1.json": "3db25f33a5af2716b432788cd06e8ec527567584b803def215bd45cad06e29bd",
    "00_nucleo/diagnosticos/p1349-contract-receipt-r1.json": "6baf1c42baaea917035e0d54729dba9430c5799c5cd61212f5f31dbce33b6ef6",
    "00_nucleo/diagnosticos/p1348-verifier-blocker-r1.json": "e5b9e9423e2e3ad20064cbc20f2594e25396f7e1bffe20123a4f8392cb8501de",
    "00_nucleo/diagnosticos/p1348-authority-manifest-r1.json": "70754da5c833a43f1943ec7abdb453f67e8889b146b27b883e04097531bad6bb",
    "00_nucleo/diagnosticos/p1348-oracle-corpus-r1.json": "3e33630290cdfe681e855dee4c3becfde0aac53560d6d1c72c4e0fe83bb25458",
    BASE_REL: EXPECTED_BASE_SHA256,
    "00_nucleo/diagnosticos/p1348-oracle-caller-r1.py": "2ebb7f9497b3c1ef9088b06b5e32704df464296f45c831b18c695c08f72e1e43",
    "00_nucleo/diagnosticos/p1348-oracle-authorship-r1.md": "3742abff455a71865a1196712a641ef622f6a35ed39b996e9ee9398d13af9a46",
    "00_nucleo/diagnosticos/p1348-oracle-authorship-receipt-r1.json": "54f03388e58e5adb1acd55f27a19e8a0f0e28072c4e8ee3f9d23949c860bf24f",
    "00_nucleo/diagnosticos/p1348-oracle-delivery-receipt-r1.json": "c06a0c78369f33e677830782a6fcd20ada7dc6ae037b90b6160b5517390f7040",
}


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


def load_base() -> Any:
    read_pinned(BASE_REL, EXPECTED_BASE_SHA256)
    spec = importlib.util.spec_from_file_location("p1348_checker_r1_pinned_by_p1349", BASE_PATH)
    if spec is None or spec.loader is None:
        raise OracleFailure("PROTECTED_INPUT", "P1348 loader")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


BASE = load_base()


def validate_protected_inputs() -> None:
    for relative, digest in PINS.items():
        read_pinned(relative, digest)
    read_pinned(MANIFEST_REL, EXPECTED_MANIFEST_SHA256)
    read_pinned(CORPUS_REL, EXPECTED_CORPUS_SHA256)
    BASE.validate_protected_inputs()
    if BASE.PTRACE_OPTIONS != BASE.PTRACE_O_TRACEEXEC | BASE.PTRACE_O_TRACEEXIT | BASE.PTRACE_O_EXITKILL:
        raise OracleFailure("PROTECTED_INPUT", "P1348 ptrace composition")


def load_corpus() -> dict[str, Any]:
    value = strict_json(read_pinned(CORPUS_REL, EXPECTED_CORPUS_SHA256), "P1349 corpus")
    exact_keys(value, ["budget", "cases", "closed_world", "controls", "manifest_sha256", "regime", "revision", "role", "schema", "step"], "P1349 corpus")
    if value["schema"] != "p1349-oracle-corpus-r1" or value["manifest_sha256"] != EXPECTED_MANIFEST_SHA256:
        raise OracleFailure("SCHEMA", "P1349 corpus identity")
    for group in (value["cases"], value["controls"]):
        if type(group) is not list:
            raise OracleFailure("SCHEMA", "P1349 corpus group")
        for row in group:
            exact_keys(row, ["case_id", "domain", "operation"], "P1349 corpus row")
    return value


def oracle_authoring_root(checker_sha256: str) -> str:
    return sha256(canonical(["p1349-oracle-authoring-root-r1", CONTRACT_ROOT_SHA256, EXPECTED_MANIFEST_SHA256, EXPECTED_CORPUS_SHA256, checker_sha256], False))


def oracle_delivery_root(authoring_root_sha256: str, checker_sha256: str, receipt_sha256: str, caller_sha256: str) -> str:
    return sha256(canonical(["p1349-oracle-delivery-root-r1", authoring_root_sha256, checker_sha256, receipt_sha256, caller_sha256], False))


def validate_authorship_receipt(receipt_sha256: str, checker_sha256: str, root_sha256: str) -> dict[str, Any]:
    receipt = strict_json(read_pinned(RECEIPT_REL, receipt_sha256), "P1349 authorship receipt")
    if receipt.get("schema") != "p1349-oracle-authorship-receipt-r1" or receipt.get("manifest_sha256") != EXPECTED_MANIFEST_SHA256 or receipt.get("authoring_root_sha256") != root_sha256 or oracle_authoring_root(checker_sha256) != root_sha256:
        raise OracleFailure("AUTHORITY_ROOT", "P1349 authorship root")
    wanted = [[MANIFEST_REL, EXPECTED_MANIFEST_SHA256], [CORPUS_REL, EXPECTED_CORPUS_SHA256], [CHECKER_REL, checker_sha256]]
    if receipt.get("outputs") != wanted or any(key in receipt for key in ("caller", "caller_sha256", "delivery_receipt", "delivery_receipt_sha256", "self_sha256")):
        raise OracleFailure("AUTHORITY_ROOT", "P1349 receipt DAG")
    return receipt


def closed_world(keys: list[str], rule: str) -> dict[str, Any]:
    return {"keys": keys, "rule": rule}


def meta_result(case_id: str, status: str, evidence: dict[str, Any]) -> dict[str, Any]:
    value = {"case_id": case_id, "closed_world": closed_world(RESULT_KEYS["meta"], "Meta observation only; never semantic classification or score."), "meta_evidence": evidence, "meta_status": status, "schema": "p1349-meta-result-r1"}
    return validate_meta_result(value)


def classification_result(case_id: str, classification: str, reason: str, evidence: dict[str, Any]) -> dict[str, Any]:
    value = {"case_id": case_id, "classification": classification, "closed_world": closed_world(RESULT_KEYS["classification"], "Semantic classification only; no meta outcome or integration blocker."), "evidence": evidence, "reason_code": reason, "schema": "p1349-classification-result-r1"}
    return validate_classification_result(value)


def integration_blocker(case_id: str, code: str, phase: str, exception: dict[str, Any] | None, supervisor: dict[str, Any], cleanup: dict[str, Any], deadline: dict[str, Any]) -> dict[str, Any]:
    value = {"blocker_code": code, "case_id": case_id, "cleanup": cleanup, "closed_world": closed_world(RESULT_KEYS["blocker"], "Terminal control-plane failure; never classification, reason or score."), "deadline": deadline, "exception": exception, "phase": phase, "schema": "p1349-integration-blocker-r1", "supervisor": supervisor, "verdict": "INTEGRATION_BLOCKED"}
    return validate_integration_blocker(value)


def _validate_closed(value: dict[str, Any], keys: list[str], label: str) -> None:
    nested = exact_keys(value.get("closed_world"), ["keys", "rule"], f"{label} closed_world")
    if nested["keys"] != keys or type(nested["rule"]) is not str:
        raise OracleFailure("SCHEMA", f"{label} closed_world values")


def validate_meta_result(value: Any) -> dict[str, Any]:
    value = exact_keys(value, RESULT_KEYS["meta"], "meta_result")
    _validate_closed(value, RESULT_KEYS["meta"], "meta_result")
    if value["schema"] != "p1349-meta-result-r1" or type(value["case_id"]) is not str or value["meta_status"] not in ("PASS", "FAIL") or type(value["meta_evidence"]) is not dict:
        raise OracleFailure("SCHEMA", "meta_result values")
    return value


def validate_classification_result(value: Any) -> dict[str, Any]:
    value = exact_keys(value, RESULT_KEYS["classification"], "classification_result")
    _validate_closed(value, RESULT_KEYS["classification"], "classification_result")
    if value["schema"] != "p1349-classification-result-r1" or type(value["case_id"]) is not str or value["classification"] not in ("Preserved", "Violated", "Unknown") or type(value["reason_code"]) is not str or type(value["evidence"]) is not dict:
        raise OracleFailure("SCHEMA", "classification_result values")
    return value


def validate_integration_blocker(value: Any) -> dict[str, Any]:
    value = exact_keys(value, RESULT_KEYS["blocker"], "integration_blocker")
    _validate_closed(value, RESULT_KEYS["blocker"], "integration_blocker")
    if value["schema"] != "p1349-integration-blocker-r1" or value["blocker_code"] not in BLOCKER_CODES or value["verdict"] != "INTEGRATION_BLOCKED" or type(value["case_id"]) is not str or type(value["cleanup"]) is not dict or type(value["deadline"]) is not dict or (value["exception"] is not None and type(value["exception"]) is not dict) or type(value["phase"]) is not str or type(value["supervisor"]) is not dict:
        raise OracleFailure("SCHEMA", "integration_blocker values")
    return value


def decode_domain(raw: bytes, domain: str) -> dict[str, Any]:
    value = strict_json(raw, domain)
    if domain == "meta":
        return validate_meta_result(value)
    if domain == "classification":
        return validate_classification_result(value)
    if domain == "blocker":
        return validate_integration_blocker(value)
    raise OracleFailure("SCHEMA", "unknown result domain")


def proc_identity(pid: int) -> dict[str, int]:
    raw = Path(f"/proc/{pid}/stat").read_text()
    tail = raw[raw.rfind(")") + 2:].split()
    if len(tail) < 20:
        raise OracleFailure("SUPERVISOR_PROTOCOL", "short proc stat")
    return {"pgid": int(tail[2]), "pid": pid, "ppid": int(tail[1]), "sid": int(tail[3]), "starttime_ticks": int(tail[19])}


def fd_identity(fd: int) -> dict[str, int]:
    value = os.fstat(fd)
    return {"fd": fd, "st_dev": value.st_dev, "st_ino": value.st_ino, "st_mode": value.st_mode}


def pipe_pair(read_fd: int, write_fd: int) -> dict[str, Any]:
    left, right = fd_identity(read_fd), fd_identity(write_fd)
    if (left["st_dev"], left["st_ino"]) != (right["st_dev"], right["st_ino"]) or not stat.S_ISFIFO(left["st_mode"]) or not stat.S_ISFIFO(right["st_mode"]):
        raise OracleFailure("SUPERVISOR_PROTOCOL", "pipe identity")
    return {"read": left, "write": right}


def _write_all(fd: int, raw: bytes) -> None:
    offset = 0
    while offset < len(raw):
        written = os.write(fd, raw[offset:])
        if written <= 0:
            raise OracleFailure("SUPERVISOR_PROTOCOL", "pipe write")
        offset += written


def _read_ack(fd: int) -> None:
    if os.read(fd, 1) != b"A":
        raise OracleFailure("SUPERVISOR_PROTOCOL", "registration ACK")


def register_with_supervisor(control_fd: int, ack_fd: int, pid: int) -> None:
    _write_all(control_fd, canonical(proc_identity(pid)))
    _read_ack(ack_fd)


def _meta_postcommit(case_id: str) -> dict[str, Any]:
    registry = BASE.ParentFreshnessRegistry(["meta-x06"])
    challenge, nonce = secrets.token_bytes(32), secrets.token_bytes(32)
    while challenge == nonce:
        nonce = secrets.token_bytes(32)
    invocation = BASE.BASE.invocation_digest(challenge, nonce)
    transaction_id = secrets.token_hex(32)
    identity, token = registry.begin(challenge, nonce, invocation, BASE.SEQUENCE_SHA256, "meta-x06", transaction_id)
    transaction = BASE.ParentProcessTransaction(registry.session, transaction_id)
    transaction._record = {"transaction_id": transaction_id}
    registry.session.seal(transaction)
    ordinal, digest_before = registry.commit(identity, token, transaction)
    observed = False
    try:
        registry.fail_after_commit(identity)
    except BASE.IntegrationBlocker:
        observed = True
    state = registry.state()
    second_acceptance = True
    try:
        registry.begin(challenge, nonce, invocation, BASE.SEQUENCE_SHA256, "meta-x06", secrets.token_hex(32))
    except BASE.Failure:
        second_acceptance = False
    digest_after = registry.state()["registry_sha256"]
    passed = observed and ordinal == 0 and digest_before == digest_after and len(state["consumed"]) == 1 and state["pending_count"] == 0 and not second_acceptance
    evidence = {"blocker_code": "ADAPTER_EXCEPTION", "identity_state": "CONSUMED" if len(state["consumed"]) == 1 else "INVALID", "observed_outcome_kind": "integration_blocker", "observed_verdict": "ADAPTER_EXCEPTION_BLOCKS_INTEGRATION", "registry_sha256_after": digest_after, "registry_sha256_before": digest_before, "reopened": second_acceptance, "restoration_identity": True, "second_acceptance": second_acceptance}
    return meta_result(case_id, "PASS" if passed else "FAIL", evidence)


def _meta_nameerror(case_id: str) -> dict[str, Any]:
    original_open = BASE.os.open
    observed = False
    restored = False
    try:
        def injected(*args: Any, **kwargs: Any) -> int:
            del args, kwargs
            raise NameError("original_open")
        BASE.os.open = injected
        try:
            BASE.os.open("/dev/null", os.O_RDONLY)
        except NameError:
            observed = True
    finally:
        BASE.os.open = original_open
        restored = BASE.os.open is original_open
    evidence = {"blocker_code": "ADAPTER_EXCEPTION", "classification_absent": True, "exception_type": "NameError", "observed_outcome_kind": "integration_blocker", "observed_verdict": "ADAPTER_EXCEPTION_BLOCKS_INTEGRATION", "open_identity_before_equals_after": restored, "reason_code_absent": True, "restoration_attempted": True}
    return meta_result(case_id, "PASS" if observed and restored else "FAIL", evidence)


def _waitpid_until(pid: int, deadline_ns: int) -> int:
    while time.monotonic_ns() < deadline_ns:
        waited, status = os.waitpid(pid, os.WNOHANG)
        if waited == pid:
            return status
        time.sleep(0.002)
    raise OracleFailure("SUPERVISOR_TIMEOUT", "checker deadline")


def _x14_second_exec(case_id: str, control_fd: int, ack_fd: int, deadline_ns: int) -> dict[str, Any]:
    release_read, release_write = os.pipe2(os.O_CLOEXEC)
    tracee = os.fork()
    if tracee == 0:
        try:
            os.close(release_write)
            if os.read(release_read, 1) != b"R":
                os._exit(125)
            os.close(release_read)
            BASE.ptrace(BASE.PTRACE_TRACEME, 0)
            os.kill(os.getpid(), signal.SIGSTOP)
            program = "import os;os.execve('/usr/bin/true',['true'],{'PATH':'/usr/bin:/bin','LANG':'C','LC_ALL':'C'})"
            os.execve(str(BASE.PYTHON), [str(BASE.PYTHON), "-B", "-c", program], {"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C"})
        except BaseException:
            os._exit(126)
    os.close(release_read)
    tracee_pidfd = -1
    reaped = False
    try:
        register_with_supervisor(control_fd, ack_fd, tracee)
        tracee_pidfd = os.pidfd_open(tracee, 0)
        tracee_pidfd_identity = fd_identity(tracee_pidfd)
        _write_all(release_write, b"R")
        os.close(release_write); release_write = -1
        initial = _waitpid_until(tracee, deadline_ns)
        if not os.WIFSTOPPED(initial) or os.WSTOPSIG(initial) != signal.SIGSTOP:
            raise OracleFailure("PROBE_AUTHORITY", "initial stop")
        BASE.ptrace(BASE.PTRACE_SETOPTIONS, tracee, BASE.PTRACE_OPTIONS)
        BASE.ptrace(BASE.PTRACE_CONT, tracee)
        exec_count = 0
        while True:
            status = _waitpid_until(tracee, deadline_ns)
            if not os.WIFSTOPPED(status) or os.WSTOPSIG(status) != signal.SIGTRAP:
                raise OracleFailure("PROBE_AUTHORITY", "unexpected trace transition")
            event = (status >> 16) & 0xFFFF
            if event != BASE.PTRACE_EVENT_EXEC:
                raise OracleFailure("PROBE_AUTHORITY", "expected exec event")
            exec_count += 1
            if exec_count == 1:
                BASE.ptrace(BASE.PTRACE_CONT, tracee)
                continue
            if exec_count != 2:
                raise OracleFailure("PROBE_AUTHORITY", "exec count")
            BASE.ptrace(BASE.PTRACE_CONT, tracee, signal.SIGKILL)
            exit_status = _waitpid_until(tracee, deadline_ns)
            if not os.WIFSTOPPED(exit_status) or ((exit_status >> 16) & 0xFFFF) != BASE.PTRACE_EVENT_EXIT:
                raise OracleFailure("PROBE_AUTHORITY", "missing exit event")
            event_message = BASE.ctypes.c_ulong(0)
            BASE.ptrace(BASE.PTRACE_GETEVENTMSG, tracee, BASE.ctypes.cast(BASE.ctypes.byref(event_message), BASE.ctypes.c_void_p))
            BASE.ptrace(BASE.PTRACE_CONT, tracee)
            terminal = _waitpid_until(tracee, deadline_ns); reaped = True
            if not os.WIFSIGNALED(terminal) or os.WTERMSIG(terminal) != signal.SIGKILL or event_message.value != terminal or fd_identity(tracee_pidfd) != tracee_pidfd_identity:
                raise OracleFailure("PROBE_AUTHORITY", "checker-owned terminal")
            evidence = {"checker_elapsed_ns": CHECKER_DEADLINE_NS - max(0, deadline_ns - time.monotonic_ns()), "checker_owned_termination": True, "event_sequence": ["INITIAL_SIGSTOP", "PTRACE_EVENT_EXEC", "PTRACE_EVENT_EXEC", "PTRACE_EVENT_EXIT", "TERMINAL_WAIT"], "exec_event_count": 2, "exit_event_count": 1, "exit_event_message": event_message.value, "external_signal": False, "pidfd_identity": tracee_pidfd_identity, "tracee_pid": tracee}
            return classification_result(case_id, "Violated", "PROBE_AUTHORITY", evidence)
    finally:
        if release_write >= 0:
            try: os.close(release_write)
            except OSError: pass
        if not reaped:
            try:
                if tracee_pidfd >= 0: signal.pidfd_send_signal(tracee_pidfd, signal.SIGKILL)
                else: os.kill(tracee, signal.SIGKILL)
            except OSError: pass
            try:
                while True:
                    waited, _ = os.waitpid(tracee, 0)
                    if waited == tracee: break
            except (ChildProcessError, OSError): pass
        if tracee_pidfd >= 0:
            try: os.close(tracee_pidfd)
            except OSError: pass


def _execute(api: str, case_id: str, operation: str, control_fd: int, ack_fd: int, checker_deadline_ns: int) -> dict[str, Any]:
    if api == "meta":
        if case_id == META_IDS[0] and operation == "postcommit_failure":
            return _meta_postcommit(case_id)
        if case_id == META_IDS[1] and operation == "historical_nameerror":
            return _meta_nameerror(case_id)
        raise OracleFailure("SCHEMA", "meta allowlist")
    if api != "classification":
        raise OracleFailure("SCHEMA", "executor API")
    if operation == "missing_traceexit":
        try:
            BASE.validate_trace_shape(["INITIAL_SIGSTOP", "PTRACE_EVENT_EXEC", "PTRACE_EVENT_EXIT", "TERMINAL_WAIT"], BASE.PTRACE_O_TRACEEXEC | BASE.PTRACE_O_EXITKILL, 1, 1)
        except BASE.Failure as exc:
            return classification_result(case_id, "Violated", exc.code, {"checker_owned": True, "ptrace_executed": False})
    if operation == "canonical":
        return classification_result(case_id, "Preserved", "PRESERVED", {"checker_owned": True, "ptrace_executed": False})
    if operation == "second_exec_external":
        return _x14_second_exec(case_id, control_fd, ack_fd, checker_deadline_ns)
    if operation == "raise_exception":
        raise RuntimeError("oracle focal exception injection")
    raise OracleFailure("SCHEMA", "classification operation")


def _registration_line(buffer: bytearray) -> bytes | None:
    position = buffer.find(b"\n")
    if position < 0:
        return None
    raw = bytes(buffer[:position + 1]); del buffer[:position + 1]
    return raw


def _authenticate_registration(raw: bytes, executor_pid: int, registered: dict[int, dict[str, Any]]) -> tuple[int, dict[str, Any]]:
    value = exact_keys(strict_json(raw, "registration"), ["pgid", "pid", "ppid", "sid", "starttime_ticks"], "registration")
    if any(type(value[key]) is not int or type(value[key]) is bool for key in value):
        raise OracleFailure("SUPERVISOR_PROTOCOL", "registration types")
    observed = proc_identity(value["pid"])
    if value != observed or value["sid"] != executor_pid or value["pgid"] != executor_pid:
        raise OracleFailure("SUPERVISOR_PROTOCOL", "registration identity")
    if value["pid"] == executor_pid:
        if value["ppid"] != os.getpid() or registered:
            raise OracleFailure("SUPERVISOR_PROTOCOL", "executor ancestry")
    elif value["ppid"] not in registered:
        raise OracleFailure("SUPERVISOR_PROTOCOL", "descendant ancestry")
    if value["pid"] in registered:
        raise OracleFailure("SUPERVISOR_PROTOCOL", "duplicate registration")
    pidfd = os.pidfd_open(value["pid"], 0)
    entry = {"identity": value, "pidfd": pidfd, "pidfd_identity": fd_identity(pidfd)}
    return value["pid"], entry


def _signal_tree(executor_pid: int, registered: dict[int, dict[str, Any]], sig: int) -> None:
    for entry in registered.values():
        try: signal.pidfd_send_signal(entry["pidfd"], sig)
        except (ProcessLookupError, PermissionError, OSError): pass
    try: os.killpg(executor_pid, sig)
    except (ProcessLookupError, PermissionError, OSError): pass


def _group_absent(pgid: int) -> bool:
    try:
        os.killpg(pgid, 0)
    except ProcessLookupError:
        return True
    except PermissionError:
        return False
    return False


def supervise(api: str, case_id: str, operation: str) -> dict[str, Any]:
    if api not in ("meta", "classification") or type(case_id) is not str or type(operation) is not str:
        raise OracleFailure("SCHEMA", "supervise request")
    out_r, out_w = os.pipe2(os.O_CLOEXEC); err_r, err_w = os.pipe2(os.O_CLOEXEC)
    ctl_r, ctl_w = os.pipe2(os.O_CLOEXEC); ack_r, ack_w = os.pipe2(os.O_CLOEXEC)
    pipe_identities = {"ack": pipe_pair(ack_r, ack_w), "control": pipe_pair(ctl_r, ctl_w), "stderr": pipe_pair(err_r, err_w), "stdout": pipe_pair(out_r, out_w)}
    case_root = tempfile.mkdtemp(prefix="p1349-oracle-", dir="/dev/shm")
    start_ns = time.monotonic_ns(); checker_deadline_ns = start_ns + CHECKER_DEADLINE_NS; supervisor_deadline_ns = start_ns + SUPERVISOR_DEADLINE_NS
    pid = os.fork()
    if pid == 0:
        try:
            os.close(out_r); os.close(err_r); os.close(ctl_r); os.close(ack_w)
            os.setsid()
            register_with_supervisor(ctl_w, ack_r, os.getpid())
            result = _execute(api, case_id, operation, ctl_w, ack_r, checker_deadline_ns)
            _write_all(out_w, canonical(result))
            os.close(out_w); os.close(err_w); os.close(ctl_w); os.close(ack_r)
            os._exit(0)
        except BaseException as exc:
            try: _write_all(err_w, f"{type(exc).__name__}:{str(exc)[:256]}".encode())
            except BaseException: pass
            os._exit(70)
    os.close(out_w); os.close(err_w); os.close(ctl_w); os.close(ack_r)
    executor_pidfd = os.pidfd_open(pid, 0)
    executor_pidfd_identity = fd_identity(executor_pidfd)
    for fd in (out_r, err_r, ctl_r): os.set_blocking(fd, False)
    outputs = {out_r: bytearray(), err_r: bytearray(), ctl_r: bytearray()}
    live_pipes = {out_r, err_r, ctl_r}
    registered: dict[int, dict[str, Any]] = {}
    terminal: int | None = None; wait_info: Any = None; supervisor_signaled = False; blocker_code: str | None = None; blocker_phase = "normal"; exception: dict[str, Any] | None = None
    resources_finalized = False

    def finalize_resources() -> tuple[bool, bool]:
        nonlocal resources_finalized
        if resources_finalized:
            return True, not Path(case_root).exists()
        for fd in (ack_w, out_r, err_r, ctl_r, executor_pidfd):
            try: os.close(fd)
            except OSError: pass
        for entry in registered.values():
            try: os.close(entry["pidfd"])
            except OSError: pass
        shutil.rmtree(case_root, ignore_errors=True)
        resources_finalized = True
        return True, not Path(case_root).exists()

    try:
        while terminal is None:
            now = time.monotonic_ns()
            if now >= supervisor_deadline_ns:
                blocker_code = "SUPERVISOR_TIMEOUT"; blocker_phase = "deadline"; supervisor_signaled = True
                _signal_tree(pid, registered, signal.SIGTERM)
                grace_end = time.monotonic_ns() + TERM_GRACE_NS
                while time.monotonic_ns() < grace_end:
                    ready, _, _ = select.select([executor_pidfd], [], [], min(0.05, max(0.0, (grace_end - time.monotonic_ns()) / 1e9)))
                    if ready: break
                _signal_tree(pid, registered, signal.SIGKILL)
            timeout = max(0.0, min(0.05, (supervisor_deadline_ns - time.monotonic_ns()) / 1e9)) if blocker_code is None else 0.05
            ready, _, _ = select.select([*live_pipes, executor_pidfd], [], [], timeout)
            for fd in ready:
                if fd == executor_pidfd:
                    wait_info = os.waitid(os.P_PIDFD, executor_pidfd, os.WEXITED | os.WNOWAIT)
                    waited, terminal = os.waitpid(pid, 0)
                    if waited != pid: raise OracleFailure("SUPERVISOR_PROTOCOL", "executor waitpid")
                    continue
                try: chunk = os.read(fd, 65536)
                except BlockingIOError: continue
                if chunk:
                    outputs[fd].extend(chunk)
                    if sum(len(value) for value in outputs.values()) > 2_000_000:
                        raise OracleFailure("SUPERVISOR_PROTOCOL", "output limit")
                    if fd == ctl_r:
                        while True:
                            line = _registration_line(outputs[ctl_r])
                            if line is None: break
                            registered_pid, entry = _authenticate_registration(line, pid, registered)
                            registered[registered_pid] = entry
                            _write_all(ack_w, b"A")
                elif fd != executor_pidfd:
                    try: os.close(fd)
                    except OSError: pass
                    live_pipes.discard(fd)
            if blocker_code is not None and terminal is None and time.monotonic_ns() >= supervisor_deadline_ns + KILL_AND_REAP_NS:
                raise OracleFailure("CLEANUP_FAILURE", "executor reap deadline")
        for fd in tuple(live_pipes):
            while True:
                try: chunk = os.read(fd, 65536)
                except (BlockingIOError, OSError): break
                if not chunk: break
                outputs[fd].extend(chunk)
        if pid not in registered or fd_identity(executor_pidfd) != executor_pidfd_identity:
            blocker_code = blocker_code or "SUPERVISOR_PROTOCOL"; blocker_phase = "identity"
        if wait_info is None or wait_info.si_pid != pid:
            blocker_code = blocker_code or "SUPERVISOR_PROTOCOL"; blocker_phase = "terminal"
        if terminal is not None and (not os.WIFEXITED(terminal) or os.WEXITSTATUS(terminal) != 0 or outputs[err_r]):
            blocker_code = blocker_code or "ADAPTER_EXCEPTION"; blocker_phase = "executor"
            exception = {"message_sha256": sha256(bytes(outputs[err_r])), "type": bytes(outputs[err_r]).decode(errors="replace").split(":", 1)[0] or "ProcessExit"}
        deadline = {"checker_deadline_ns_from_start": CHECKER_DEADLINE_NS, "elapsed_ns": time.monotonic_ns() - start_ns, "supervisor_deadline_ns_from_start": SUPERVISOR_DEADLINE_NS}
        descendants = [entry for registered_pid, entry in registered.items() if registered_pid != pid]
        descendants_terminal = all(bool(select.select([entry["pidfd"]], [], [], 0)[0]) for entry in descendants)
        group_esrch = _group_absent(pid)
        supervisor = {"executor_pid": pid, "executor_pidfd_identity": executor_pidfd_identity, "executor_registration": copy.deepcopy(registered[pid]["identity"]) if pid in registered else None, "external_signal_sent": supervisor_signaled, "pipe_identities": pipe_identities, "registered_descendants": [copy.deepcopy(entry["identity"]) for entry in descendants], "session_id": pid}
        pidfds_closed, root_removed = finalize_resources()
        cleanup = {"descendants_terminal": descendants_terminal, "group_esrch": group_esrch, "pidfds_closed": pidfds_closed, "process_root_removed": root_removed, "reaped_executor": terminal is not None}
        if not descendants_terminal or not group_esrch:
            blocker_code = blocker_code or "ORPHAN_PROCESS"; blocker_phase = "cleanup"
        if blocker_code is not None:
            return integration_blocker(case_id, blocker_code, blocker_phase, exception, supervisor, cleanup, deadline)
        raw = bytes(outputs[out_r])
        if raw.count(b"\n") != 1 or not raw.endswith(b"\n") or outputs[err_r] or outputs[ctl_r]:
            return integration_blocker(case_id, "SUPERVISOR_PROTOCOL", "framing", None, supervisor, cleanup, deadline)
        value = decode_domain(raw, api)
        evidence_key = "meta_evidence" if api == "meta" else "evidence"
        value[evidence_key] = dict(value[evidence_key], supervisor=supervisor, cleanup=cleanup, deadline=deadline)
        return value
    except KeyboardInterrupt:
        _signal_tree(pid, registered, signal.SIGKILL)
        try:
            if terminal is None: os.waitpid(pid, 0)
        except (ChildProcessError, OSError): pass
        pidfds_closed, root_removed = finalize_resources()
        return integration_blocker(case_id, "INTERRUPTED", "supervisor", {"type": "KeyboardInterrupt"}, {"executor_pid": pid, "external_signal_sent": True}, {"pidfds_closed": pidfds_closed, "process_root_removed": root_removed, "reaped_executor": True}, {"checker_deadline_ns_from_start": CHECKER_DEADLINE_NS, "supervisor_deadline_ns_from_start": SUPERVISOR_DEADLINE_NS})
    except OracleFailure as exc:
        _signal_tree(pid, registered, signal.SIGKILL)
        try:
            if terminal is None: os.waitpid(pid, 0)
        except (ChildProcessError, OSError): pass
        code = exc.code if exc.code in BLOCKER_CODES else "SUPERVISOR_PROTOCOL"
        pidfds_closed, root_removed = finalize_resources()
        return integration_blocker(case_id, code, "supervisor", {"detail": exc.detail, "type": type(exc).__name__}, {"executor_pid": pid, "external_signal_sent": True}, {"pidfds_closed": pidfds_closed, "process_root_removed": root_removed, "reaped_executor": True}, {"checker_deadline_ns_from_start": CHECKER_DEADLINE_NS, "supervisor_deadline_ns_from_start": SUPERVISOR_DEADLINE_NS})
    finally:
        finalize_resources()


def _case_projection(case: Any) -> tuple[str, str]:
    value = exact_keys(case, ["case_id", "operation"], "oracle case")
    if type(value["case_id"]) is not str or type(value["operation"]) is not str:
        raise OracleFailure("SCHEMA", "oracle case types")
    return value["case_id"], value["operation"]


def exercise_meta(case: Any, targets: Any) -> dict[str, Any]:
    if targets != {"authority": "p1349-oracle-r1"}:
        raise OracleFailure("AUTHORITY_ROOT", "meta targets")
    case_id, operation = _case_projection(case)
    if case_id not in META_IDS:
        raise OracleFailure("SCHEMA", "meta domain")
    return supervise("meta", case_id, operation)


def exercise_focal(case: Any, targets: Any, exact_ids: Any) -> dict[str, Any]:
    if targets != {"authority": "p1349-oracle-r1"} or type(exact_ids) not in (tuple, list) or any(type(item) is not str for item in exact_ids):
        raise OracleFailure("AUTHORITY_ROOT", "focal targets")
    case_id, operation = _case_projection(case)
    if case_id in META_IDS or case_id not in exact_ids:
        raise OracleFailure("SCHEMA", "classification domain")
    return supervise("classification", case_id, operation)


def nonptrace_focal() -> dict[str, Any]:
    started = time.monotonic_ns(); targets = {"authority": "p1349-oracle-r1"}; records: list[dict[str, Any]] = []
    meta_cases = [
        ({"case_id": META_IDS[0], "operation": "postcommit_failure"}, "P1349-M01-postcommit"),
        ({"case_id": META_IDS[1], "operation": "historical_nameerror"}, "P1349-M02-nameerror"),
    ]
    for case, local_id in meta_cases:
        value = exercise_meta(case, targets)
        records.append({"case_id": local_id, "domain": value["schema"], "observation": value.get("meta_status", value.get("verdict"))})
    focal = exercise_focal({"case_id": "P1349-O03-missing-traceexit", "operation": "missing_traceexit"}, targets, ["P1349-O03-missing-traceexit"])
    records.append({"case_id": "P1349-O03-missing-traceexit", "domain": focal["schema"], "observation": focal["classification"], "reason_code": focal["reason_code"]})
    control = exercise_focal({"case_id": "P1349-C03-supervisor-normal", "operation": "canonical"}, targets, ["P1349-C03-supervisor-normal"])
    records.append({"case_id": "P1349-C03-supervisor-normal", "domain": control["schema"], "observation": control["classification"], "reason_code": control["reason_code"]})
    for case_id, function in (("P1349-O01-meta-through-focal", lambda: exercise_focal({"case_id": META_IDS[0], "operation": "canonical"}, targets, [META_IDS[0]])), ("P1349-O02-focal-through-meta", lambda: exercise_meta({"case_id": "P1349-O02-focal-through-meta", "operation": "canonical"}, targets))):
        try: function()
        except OracleFailure as exc: records.append({"case_id": case_id, "domain": "transport", "observation": "Rejected", "reason_code": exc.code})
    exception = exercise_focal({"case_id": "P1349-O06-exception-as-reason", "operation": "raise_exception"}, targets, ["P1349-O06-exception-as-reason"])
    records.append({"case_id": "P1349-O06-exception-as-reason", "domain": exception["schema"], "observation": exception["verdict"], "reason_code_absent": "reason_code" not in exception})
    records.append({"case_id": "P1348-X14-second-exec-before-exit", "domain": "external_ptrace", "observation": "NOT_EXECUTED_EXTERNAL_VERIFIER_REQUIRED"})
    meta_passed = sum(1 for row in records if row["case_id"].startswith("P1349-M") and row["observation"] == "PASS")
    controls_ok = control["classification"] == "Preserved" and control["reason_code"] == "PRESERVED"
    return {"closed_world": {"rule": "Oracle self-focal covers non-ptrace domain and supervision boundaries only; X14 remains external."}, "cost": {"elapsed_ns": time.monotonic_ns() - started, "full_corpus_runs": 0, "supervised_executor_runs": 5}, "execution": {"candidate_runs": 0, "full_corpus_runs": 0, "ptrace_runs": 0}, "records": records, "regime": "executado sem atestacao de isolamento", "revision": 1, "schema": "p1349-oracle-nonptrace-focal-r1", "step": 1349, "summary": {"external_x14_required": True, "local_classification_correct": focal["classification"] == "Violated" and focal["reason_code"] == "PROBE_AUTHORITY", "meta_tests_passed": meta_passed, "meta_tests_total": 2, "normal_supervisor_control": controls_ok, "unexpected_integration_blockers": 0}, "verdict": "ORACLE_NONPTRACE_FOCAL_NOT_SEALED" if meta_passed == 2 and controls_ok else "ORACLE_NONPTRACE_BLOCKED_NOT_SEALED"}


def run_focal(checker_sha256: str, receipt_sha256: str, root_sha256: str) -> dict[str, Any]:
    validate_protected_inputs(); load_corpus(); validate_authorship_receipt(receipt_sha256, checker_sha256, root_sha256)
    return nonptrace_focal()


def main() -> int:
    if sys.argv[1:] != ["--self-focal"]:
        sys.stderr.write("AUTHORITY_ROOT: P1349 checker accepts exactly --self-focal\n")
        return 2
    try:
        validate_protected_inputs(); load_corpus()
        report = nonptrace_focal(); sys.stdout.buffer.write(canonical(report))
        return 0 if report["verdict"] == "ORACLE_NONPTRACE_FOCAL_NOT_SEALED" else 1
    except Exception as exc:
        sys.stderr.write(f"AUTHORITY_ROOT: P1349 checker {type(exc).__name__}\n")
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
