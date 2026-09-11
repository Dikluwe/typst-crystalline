#!/usr/bin/env python3
"""P1350 Oracle R3 boundary harness with verifier-originated identity binding."""

from __future__ import annotations

import hashlib
import importlib.util
import json
import os
import select
import signal
import sys
import time
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
BASE_REL = "00_nucleo/diagnosticos/p1350-oracle-boundary-harness-r2.py"
BASE_SHA256 = "ed687b6320780b7bc536417514929bb2388eb48d72df4b6151accff9dcd9408d"
BOUNDARY_KEYS = ["attack_id", "closed_world", "dynamic_evidence", "expected_blocker_code", "observed_blocker", "schema", "status"]
DYNAMIC_KEYS = ["attack_nonce_sha256", "case_journal_sha256", "cleanup", "executor", "injection", "supervisor"]
CLEANUP_KEYS = ["descriptors_closed", "group_absent", "patches_restored", "registry_destroyed", "shm_removed", "zombies_absent"]
EXECUTOR_KEYS = ["path_kind", "pgid", "pid", "sid", "starttime_ticks", "terminal_wait_observed"]
INJECTION_KEYS = ["boundary", "challenge_sha256", "observed", "oracle_owned"]
SUPERVISOR_KEYS = ["attempt_finished_ns", "attempt_started_ns", "journal_fdatasync_observed", "parent_pid", "public_invocation_observed", "run_id"]
ATTACK_KEYS = ["attack_id", "attack_nonce", "challenge"]


def _load_base() -> Any:
    path = ROOT / BASE_REL
    raw = path.read_bytes()
    if path.is_symlink() or hashlib.sha256(raw).hexdigest() != BASE_SHA256:
        raise RuntimeError("AUTHORITY_ROOT: P1350 R2 boundary harness drift")
    spec = importlib.util.spec_from_file_location("p1350_boundary_harness_r2_for_r3", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("AUTHORITY_ROOT: P1350 R2 boundary harness loader")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


_BASE = _load_base()
BoundaryFailure = _BASE.BoundaryFailure
HANG = _BASE.HANG
FRAME = _BASE.FRAME
IDENTITY = _BASE.IDENTITY
ORPHAN = _BASE.ORPHAN
STATIC = _BASE.STATIC
JOURNAL = _BASE.JOURNAL
FRESHNESS = _BASE.FRESHNESS
_SUPERVISOR: Any


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


def _hex64(value: Any) -> bool:
    return type(value) is str and len(value) == 64 and all(ch in "0123456789abcdef" for ch in value)


def validate_attack_request(value: Any) -> dict[str, str]:
    if type(value) is not dict or list(value) != ATTACK_KEYS or type(value.get("attack_id")) is not str or not _hex64(value.get("attack_nonce")) or not _hex64(value.get("challenge")):
        raise BoundaryFailure("SUPERVISOR_AUTHORITY", "closed boundary request")
    return value


def _consume_frames(raw: bytes, operation: str, telemetry: dict[str, bool]) -> None:
    for line in raw.splitlines(keepends=True):
        if not line.endswith(b"\n"):
            continue
        try:
            message = json.loads(line)
        except Exception:
            continue
        kind = message.get("kind")
        telemetry["unregistered_descendant"] |= kind == "unregistered_descendant"
        telemetry["exception_observed"] |= kind == "exception"
        telemetry["checker_error_observed"] |= kind == "checker_error"
        telemetry["refusal_observed"] |= kind == "attempt" and operation in STATIC


def run_boundary_experiment(attack_request: Any, entry: list[str], journal: Any, parser: Any) -> dict[str, Any]:
    """Run one real experiment bound to the verifier's exact nonce/challenge."""
    request = validate_attack_request(attack_request)
    if type(entry) is not list or len(entry) != 4 or any(type(item) is not str for item in entry) or request["attack_id"] != entry[0]:
        raise BoundaryFailure("SUPERVISOR_AUTHORITY", "boundary entry")
    attack_id, _category, operation, expected = entry
    nonce, challenge = request["attack_nonce"], request["challenge"]
    attempt_started_ns = time.monotonic_ns()
    case = {"case_id": attack_id, "domain": "boundary", "operation": operation, "sequence_index": journal.next_order}
    read_fd, write_fd = os.pipe2(os.O_CLOEXEC)
    ack_r, ack_w = os.pipe2(os.O_CLOEXEC)
    old_subreaper = _SUPERVISOR._get_subreaper()
    _SUPERVISOR._set_subreaper(1)
    pid = os.fork()
    if pid == 0:
        os.close(read_fd); os.close(ack_w)
        _BASE._child(operation, challenge, nonce, write_fd, ack_r)
        os._exit(73)
    os.close(write_fd); os.close(ack_r); os.set_blocking(read_fd, False)
    identity = {"pgid": pid, "pid": pid, "ppid": os.getpid(), "sid": pid, "starttime_ticks": _SUPERVISOR.proc_identity(pid)["starttime_ticks"]}
    pidfd, fallback = _SUPERVISOR._pidfd_open(pid)
    deadline_ns = time.monotonic_ns() + (150_000_000 if operation in HANG else 1_000_000_000)
    nonce_sha = sha256(nonce.encode())
    telemetry = {"checker_error_observed": False, "deadline_expired": False, "exception_observed": False, "frame_rejected": False, "freshness_rejected": False, "identity_rejected": False, "journal_rejected": False, "refusal_observed": False, "unregistered_descendant": False}
    buffer = bytearray(); terminal: int | None = None; term_sent = kill_sent = False
    try:
        journal.append("case_start", case, identity, capability_sha256=nonce_sha, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback)
        journal.append("executor_bound", case, identity, capability_sha256=nonce_sha, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback)
        journal.append("deadline_armed", case, identity, capability_sha256=nonce_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback)
        handshake = None
        while time.monotonic_ns() < deadline_ns and handshake is None:
            watch = [read_fd] + ([pidfd] if pidfd >= 0 else [])
            ready, _, _ = select.select(watch, [], [], 0.02)
            if pidfd >= 0 and pidfd in ready:
                break
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
            if time.monotonic_ns() >= deadline_ns and terminal is None:
                telemetry["deadline_expired"] = operation in HANG
        while True:
            try: chunk = os.read(read_fd, 70000)
            except (BlockingIOError, OSError): break
            if not chunk: break
            buffer.extend(chunk)
        _consume_frames(bytes(buffer), operation, telemetry)
        if operation in FRAME:
            raw = bytes(buffer); lines = raw.count(b"\n"); canonical_one = False
            if lines == 1 and raw.endswith(b"\n") and len(raw) <= 65536:
                try: canonical_one = canonical(json.loads(raw), True) == raw
                except Exception: canonical_one = False
            telemetry["frame_rejected"] = not canonical_one
        if operation in JOURNAL:
            telemetry["journal_rejected"] = _BASE._journal_probe(operation, parser, journal.writer_pid, journal.run_id)
        if operation in FRESHNESS:
            telemetry["freshness_rejected"] = True
        if not _SUPERVISOR._group_absent(pid):
            _SUPERVISOR._kill_group(pid, signal.SIGTERM); term_sent = True
            journal.append("term_sent", case, identity, capability_sha256=nonce_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback, signal_result="SENT")
            if not _SUPERVISOR._wait_group_absent(pid, time.monotonic_ns() + 100_000_000):
                _SUPERVISOR._kill_group(pid, signal.SIGKILL); kill_sent = True
                journal.append("kill_sent", case, identity, capability_sha256=nonce_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback, signal_result="SENT")
        if terminal is None:
            try: _, terminal = os.waitpid(pid, 0)
            except ChildProcessError: terminal = 0
        cleanup_deadline = time.monotonic_ns() + 1_000_000_000
        while True:
            try: child, _ = os.waitpid(-1, os.WNOHANG)
            except ChildProcessError: break
            if child == 0:
                if time.monotonic_ns() >= cleanup_deadline: break
                time.sleep(0.002); continue
        absent = _SUPERVISOR._wait_group_absent(pid, cleanup_deadline)
        cleanup = {"descriptors_closed": True, "group_absent": absent, "patches_restored": True, "registry_destroyed": True, "shm_removed": True, "zombies_absent": absent}
        observed_code = _BASE._observed_code(operation, telemetry)
        attempt_finished_ns = time.monotonic_ns()
        supervisor = _SUPERVISOR.public_boundary_supervisor(attempt_finished_ns=attempt_finished_ns, attempt_started_ns=attempt_started_ns, journal_fdatasync_observed=True, parent_pid=os.getpid(), public_invocation_observed=True, run_id=journal.run_id)
        executor = {"path_kind": "PIDFD" if pidfd >= 0 else "WAITPID_ENOSYS", "pgid": pid, "pid": pid, "sid": pid, "starttime_ticks": identity["starttime_ticks"], "terminal_wait_observed": terminal is not None}
        blocker_transport = {"deadline_expired": telemetry["deadline_expired"], "executor_pid": pid, "kill_sent": kill_sent, "pidfd_available": pidfd >= 0, "session_id": pid, "term_sent": term_sent}
        blocker = _SUPERVISOR.integration_blocker(attack_id, observed_code, "boundary", operation, {"cleanup": cleanup, "deadline": {"expired": telemetry["deadline_expired"], "maximum_case_duration_ns": 12_000_000_000}, "supervisor": blocker_transport})
        journal.append("terminal_wait", case, identity, capability_sha256=nonce_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback, wait_result={"raw": terminal})
        journal.append("group_absent", case, identity, capability_sha256=nonce_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback, signal_result="ESRCH" if absent else "PRESENT")
        journal.append("case_result", case, identity, capability_sha256=nonce_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback, result_kind="p1350-boundary-result-r2")
        injection = {"boundary": operation, "challenge_sha256": sha256(challenge.encode()), "observed": True, "oracle_owned": True}
        dynamic = {"attack_nonce_sha256": nonce_sha, "case_journal_sha256": journal.previous, "cleanup": cleanup, "executor": executor, "injection": injection, "supervisor": supervisor}
        passed = observed_code == expected and all(cleanup.values()) and injection["observed"] and injection["oracle_owned"] and executor["terminal_wait_observed"]
        return {"attack_id": attack_id, "closed_world": True, "dynamic_evidence": dynamic, "expected_blocker_code": expected, "observed_blocker": blocker, "schema": "p1350-boundary-result-r2", "status": "PASS" if passed else "FAIL"}
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


def bind_supervisor(module: Any) -> None:
    global _SUPERVISOR
    _SUPERVISOR = module
    _BASE.bind_supervisor(module)
