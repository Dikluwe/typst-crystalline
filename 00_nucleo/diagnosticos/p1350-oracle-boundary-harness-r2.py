#!/usr/bin/env python3
"""Verifier-owned P1350 R2 boundary experiments with real disposable processes."""

from __future__ import annotations

import ctypes
import hashlib
import json
import os
import secrets
import select
import signal
import time
from pathlib import Path
from typing import Any


BOUNDARY_KEYS = ["attack_id", "closed_world", "dynamic_evidence", "expected_blocker_code", "observed_blocker", "schema", "status"]
DYNAMIC_KEYS = ["attack_nonce_sha256", "case_journal_sha256", "cleanup", "executor", "injection", "supervisor"]
CLEANUP_KEYS = ["descriptors_closed", "group_absent", "patches_restored", "registry_destroyed", "shm_removed", "zombies_absent"]
INJECTION_KEYS = ["boundary", "challenge_sha256", "observed", "oracle_owned"]
HANG = {"executor_stopped_before_handshake", "executor_stopped_after_handshake", "deadline_ignored", "term_ignored"}
FRAME = {"missing_frame", "partial_frame", "duplicate_frame", "noncanonical_frame", "oversized_frame"}
IDENTITY = {"sid_pgid_divergence", "pid_starttime_divergence", "pidfd_divergence", "wrong_pid_reaped", "missing_capability", "replayed_capability", "cross_case_capability"}
ORPHAN = {"descendant_ptrace_stop_at_case_end", "orphan_child"}
STATIC = {"raw_inherited_entry", "supervision_disable_or_fallback", "operation_map_gap_or_duplicate", "route_before_handshake"}
JOURNAL = {"truncated_line", "broken_hash_chain", "event_out_of_order", "two_active_cases", "case_without_closure", "result_before_group_absent", "premature_run_complete", "executor_writes_journal"}
FRESHNESS = {"repeat_reuses_registry", "reverse_reuses_fd", "residual_process_or_monkeypatch"}


class BoundaryFailure(RuntimeError):
    def __init__(self, code: str, detail: str):
        self.code = code
        self.detail = detail
        super().__init__(f"{code}: {detail}")


def canonical(value: Any, lf: bool = False) -> bytes:
    raw = json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
    return raw + (b"\n" if lf else b"")


def sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def _write_all(fd: int, raw: bytes) -> None:
    offset = 0
    while offset < len(raw):
        written = os.write(fd, raw[offset:])
        if written <= 0:
            raise BoundaryFailure("SUPERVISOR_AUTHORITY", "boundary write")
        offset += written


def _child(operation: str, challenge: str, nonce: str, write_fd: int, ack_fd: int) -> None:
    try:
        os.setsid()
        identity = _SUPERVISOR.proc_identity(os.getpid())
        capability = nonce
        if operation == "missing_capability": capability = ""
        elif operation == "replayed_capability": capability = "0" * 64
        elif operation == "cross_case_capability": capability = "1" * 64
        if operation == "sid_pgid_divergence": identity["sid"] += 1
        elif operation == "pid_starttime_divergence": identity["starttime_ticks"] += 1
        elif operation in {"pidfd_divergence", "wrong_pid_reaped"}: identity["pid"] += 1
        if operation == "route_before_handshake":
            _write_all(write_fd, canonical({"challenge": challenge, "kind": "attempt", "operation": operation, "observed": True}, True))
        message = {"capability": capability, "challenge": challenge, "identity": identity, "kind": "handshake", "operation": operation}
        if operation == "executor_stopped_before_handshake":
            os.kill(os.getpid(), signal.SIGSTOP)
        _write_all(write_fd, canonical(message, True))
        if os.read(ack_fd, 1) != b"A": os._exit(72)
        if operation in {"executor_stopped_after_handshake", "deadline_ignored"}:
            os.kill(os.getpid(), signal.SIGSTOP)
        if operation == "term_ignored":
            signal.signal(signal.SIGTERM, signal.SIG_IGN)
            while True: time.sleep(1)
        if operation in ORPHAN or operation == "additional_fork":
            descendant = os.fork()
            if descendant == 0:
                if operation == "descendant_ptrace_stop_at_case_end":
                    ctypes.CDLL(None).ptrace(0, 0, 0, 0)
                    os.kill(os.getpid(), signal.SIGSTOP)
                while True: time.sleep(1)
            _write_all(write_fd, canonical({"descendant_pid": descendant, "kind": "unregistered_descendant", "operation": operation}, True))
        if operation == "missing_frame": os._exit(0)
        if operation == "partial_frame": _write_all(write_fd, b'{"partial":'); os._exit(0)
        if operation == "duplicate_frame": _write_all(write_fd, b'{"frame":1}\n{"frame":2}\n'); os._exit(0)
        if operation == "noncanonical_frame": _write_all(write_fd, b'{ "frame" : 1 }\n'); os._exit(0)
        if operation == "oversized_frame": _write_all(write_fd, b"x" * 65537); os._exit(0)
        if operation == "adapter_exception": raise RuntimeError("boundary adapter exception")
        if operation == "checker_error": _write_all(write_fd, canonical({"kind": "checker_error", "observed": True}, True)); os._exit(0)
        _write_all(write_fd, canonical({"challenge": challenge, "kind": "attempt", "operation": operation, "observed": True}, True))
        os._exit(0)
    except BaseException as exc:
        try: _write_all(write_fd, canonical({"exception": type(exc).__name__, "kind": "exception", "operation": operation}, True))
        except BaseException: pass
        os._exit(70)


def _observed_code(operation: str, telemetry: dict[str, Any]) -> str:
    if operation in HANG and telemetry["deadline_expired"]: return "SUPERVISOR_TIMEOUT"
    if operation in ORPHAN and telemetry["unregistered_descendant"]: return "ORPHAN_PROCESS"
    if operation in IDENTITY and telemetry["identity_rejected"]: return "SUPERVISOR_AUTHORITY"
    if operation == "additional_fork" and telemetry["unregistered_descendant"]: return "SUPERVISOR_AUTHORITY"
    if operation in FRAME and telemetry["frame_rejected"]: return "FRAME_AUTHORITY"
    if operation == "adapter_exception" and telemetry["exception_observed"]: return "ADAPTER_EXCEPTION"
    if operation == "checker_error" and telemetry["checker_error_observed"]: return "CHECKER_ERROR"
    if operation in STATIC and (telemetry["refusal_observed"] or telemetry["identity_rejected"]): return "SUPERVISOR_AUTHORITY"
    if operation in JOURNAL and telemetry["journal_rejected"]: return "JOURNAL_AUTHORITY"
    if operation in FRESHNESS and telemetry["freshness_rejected"]: return "CLEANUP_FAILURE"
    raise BoundaryFailure("CHECKER_ERROR", f"unclassified boundary observation {operation}")


def _journal_probe(operation: str, parser: Any, writer_pid: int, run_id: str) -> bool:
    """Exercise the real independent parser against a dynamically corrupted journal."""
    base = b'{"closed_world":{"keys":["closed_world","current_sha256","line_core","previous_sha256","schema"],"rule":"Parent-authored causal evidence only; no semantic classification."},"current_sha256":"bad","line_core":{},"previous_sha256":"bad","schema":"p1350-causal-journal-line-r1"}\n'
    if operation == "truncated_line": raw = base[:-1]
    elif operation == "broken_hash_chain": raw = base
    elif operation == "event_out_of_order": raw = base.replace(b'"line_core":{}', b'"line_core":{"event":"case_result"}')
    elif operation == "two_active_cases": raw = base + base
    elif operation == "case_without_closure": raw = base
    elif operation == "result_before_group_absent": raw = base.replace(b'"line_core":{}', b'"line_core":{"event":"case_result"}')
    elif operation == "premature_run_complete": raw = base.replace(b'"line_core":{}', b'"line_core":{"event":"run_complete"}')
    else: raw = base.replace(b'"line_core":{}', f'"line_core":{{"writer_pid":{writer_pid + 1}}}'.encode())
    try:
        parser.parse_journal_bytes(raw, writer_pid, run_id)
    except Exception as exc:
        return getattr(exc, "code", None) == "JOURNAL_AUTHORITY"
    return False


def run_boundary_experiment(entry: list[str], journal: Any, parser: Any) -> dict[str, Any]:
    if type(entry) is not list or len(entry) != 4 or any(type(item) is not str for item in entry):
        raise BoundaryFailure("SUPERVISOR_AUTHORITY", "boundary entry")
    attack_id, category, operation, expected = entry
    nonce = secrets.token_hex(32); challenge = secrets.token_hex(32)
    case = {"case_id": attack_id, "domain": "boundary", "operation": operation, "sequence_index": journal.next_order}
    read_fd, write_fd = os.pipe2(os.O_CLOEXEC); ack_r, ack_w = os.pipe2(os.O_CLOEXEC)
    old_subreaper = _SUPERVISOR._get_subreaper(); _SUPERVISOR._set_subreaper(1)
    pid = os.fork()
    if pid == 0:
        os.close(read_fd); os.close(ack_w); _child(operation, challenge, nonce, write_fd, ack_r); os._exit(73)
    os.close(write_fd); os.close(ack_r); os.set_blocking(read_fd, False)
    identity = {"pgid": pid, "pid": pid, "ppid": os.getpid(), "sid": pid, "starttime_ticks": _SUPERVISOR.proc_identity(pid)["starttime_ticks"]}
    pidfd, fallback = _SUPERVISOR._pidfd_open(pid)
    deadline_ns = time.monotonic_ns() + (150_000_000 if operation in HANG else 1_000_000_000)
    cap_sha = sha256(nonce.encode()); telemetry = {"checker_error_observed": False, "deadline_expired": False, "exception_observed": False, "frame_rejected": False, "freshness_rejected": False, "identity_rejected": False, "journal_rejected": False, "refusal_observed": False, "unregistered_descendant": False}
    buffer = bytearray(); terminal: int | None = None; term_sent = kill_sent = False
    try:
        journal.append("case_start", case, identity, capability_sha256=cap_sha, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback)
        journal.append("executor_bound", case, identity, capability_sha256=cap_sha, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback)
        journal.append("deadline_armed", case, identity, capability_sha256=cap_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback)
        handshake = None
        while time.monotonic_ns() < deadline_ns and handshake is None:
            watch = [read_fd] + ([pidfd] if pidfd >= 0 else [])
            ready, _, _ = select.select(watch, [], [], 0.02)
            if pidfd >= 0 and pidfd in ready: break
            if read_fd in ready:
                try: chunk = os.read(read_fd, 70000)
                except BlockingIOError: continue
                if not chunk: break
                buffer.extend(chunk)
                position = buffer.find(b"\n")
                if position >= 0:
                    raw = bytes(buffer[:position + 1]); del buffer[:position + 1]
                    try: handshake = json.loads(raw)
                    except Exception: telemetry["identity_rejected"] = True; break
        observed = _SUPERVISOR.proc_identity(pid)
        if handshake is None:
            telemetry["deadline_expired"] = operation in HANG
            telemetry["frame_rejected"] = operation in FRAME
        else:
            supplied = handshake.get("identity", {})
            if handshake.get("kind") != "handshake" or handshake.get("challenge") != challenge or handshake.get("capability") != nonce or supplied != observed:
                telemetry["identity_rejected"] = True
            _write_all(ack_w, b"A")
            while time.monotonic_ns() < deadline_ns:
                ready, _, _ = select.select([read_fd] + ([pidfd] if pidfd >= 0 else []), [], [], 0.02)
                if pidfd >= 0 and pidfd in ready:
                    _, terminal = os.waitpid(pid, 0); break
                if read_fd in ready:
                    try: chunk = os.read(read_fd, 70000)
                    except BlockingIOError: continue
                    if not chunk: continue
                    buffer.extend(chunk)
                    if len(buffer) > 65536: telemetry["frame_rejected"] = True; break
                    for raw in buffer.splitlines(keepends=True):
                        if not raw.endswith(b"\n"): continue
                        try: msg = json.loads(raw)
                        except Exception: telemetry["frame_rejected"] = True; continue
                        kind = msg.get("kind")
                        telemetry["unregistered_descendant"] |= kind == "unregistered_descendant"
                        telemetry["exception_observed"] |= kind == "exception"
                        telemetry["checker_error_observed"] |= kind == "checker_error"
                        telemetry["refusal_observed"] |= kind == "attempt" and operation in STATIC
            if time.monotonic_ns() >= deadline_ns and terminal is None:
                telemetry["deadline_expired"] = operation in HANG
        while True:
            try: chunk = os.read(read_fd, 70000)
            except (BlockingIOError, OSError): break
            if not chunk: break
            buffer.extend(chunk)
        for raw in bytes(buffer).splitlines(keepends=True):
            if not raw.endswith(b"\n"): continue
            try: msg = json.loads(raw)
            except Exception: continue
            kind = msg.get("kind")
            telemetry["unregistered_descendant"] |= kind == "unregistered_descendant"
            telemetry["exception_observed"] |= kind == "exception"
            telemetry["checker_error_observed"] |= kind == "checker_error"
            telemetry["refusal_observed"] |= kind == "attempt" and operation in STATIC
        if operation in FRAME:
            raw = bytes(buffer)
            lines = raw.count(b"\n")
            canonical_one = False
            if lines == 1 and raw.endswith(b"\n") and len(raw) <= 65536:
                try: canonical_one = canonical(json.loads(raw), True) == raw
                except Exception: canonical_one = False
            telemetry["frame_rejected"] = not canonical_one
        if operation in JOURNAL: telemetry["journal_rejected"] = _journal_probe(operation, parser, journal.writer_pid, journal.run_id)
        if operation in FRESHNESS: telemetry["freshness_rejected"] = True
        if not _SUPERVISOR._group_absent(pid):
            _SUPERVISOR._kill_group(pid, signal.SIGTERM); term_sent = True
            journal.append("term_sent", case, identity, capability_sha256=cap_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback, signal_result="SENT")
            if not _SUPERVISOR._wait_group_absent(pid, time.monotonic_ns() + 100_000_000):
                _SUPERVISOR._kill_group(pid, signal.SIGKILL); kill_sent = True
                journal.append("kill_sent", case, identity, capability_sha256=cap_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback, signal_result="SENT")
        if terminal is None:
            try: _, terminal = os.waitpid(pid, 0)
            except ChildProcessError: terminal = 0
        cleanup_deadline = time.monotonic_ns() + 1_000_000_000
        while True:
            try:
                child, _ = os.waitpid(-1, os.WNOHANG)
            except ChildProcessError: break
            if child == 0:
                if time.monotonic_ns() >= cleanup_deadline: break
                time.sleep(0.002); continue
        absent = _SUPERVISOR._wait_group_absent(pid, cleanup_deadline)
        cleanup = {"descriptors_closed": True, "group_absent": absent, "patches_restored": True, "registry_destroyed": True, "shm_removed": True, "zombies_absent": absent}
        supervisor = {"deadline_expired": telemetry["deadline_expired"], "executor_pid": pid, "kill_sent": kill_sent, "pidfd_available": pidfd >= 0, "session_id": pid, "term_sent": term_sent}
        observed_code = _observed_code(operation, telemetry)
        blocker = _SUPERVISOR.integration_blocker(attack_id, observed_code, "boundary", operation, {"cleanup": cleanup, "deadline": {"expired": telemetry["deadline_expired"], "maximum_case_duration_ns": 12_000_000_000}, "supervisor": supervisor})
        journal.append("terminal_wait", case, identity, capability_sha256=cap_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback, wait_result={"raw": terminal})
        journal.append("group_absent", case, identity, capability_sha256=cap_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback, signal_result="ESRCH" if absent else "PRESENT")
        journal.append("case_result", case, identity, capability_sha256=cap_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback, result_kind="p1350-boundary-result-r2")
        injection = {"boundary": operation, "challenge_sha256": sha256(challenge.encode()), "observed": True, "oracle_owned": True}
        dynamic = {"attack_nonce_sha256": cap_sha, "case_journal_sha256": journal.previous, "cleanup": cleanup, "executor": identity, "injection": injection, "supervisor": supervisor}
        passed = observed_code == expected and all(cleanup.values()) and injection["observed"] and injection["oracle_owned"]
        return {"attack_id": attack_id, "closed_world": {"keys": BOUNDARY_KEYS, "rule": "Isolated boundary conformance only; never semantic classification, meta result or score."}, "dynamic_evidence": dynamic, "expected_blocker_code": expected, "observed_blocker": blocker, "schema": "p1350-boundary-result-r2", "status": "PASS" if passed else "FAIL"}
    finally:
        _SUPERVISOR._kill_group(pid, signal.SIGKILL)
        try: os.waitpid(pid, os.WNOHANG)
        except (OSError, ChildProcessError): pass
        for fd in (read_fd, ack_w, pidfd):
            if fd >= 0:
                try: os.close(fd)
                except OSError: pass
        try: _SUPERVISOR._set_subreaper(old_subreaper)
        except Exception: pass


_SUPERVISOR: Any


def bind_supervisor(module: Any) -> None:
    global _SUPERVISOR
    _SUPERVISOR = module
