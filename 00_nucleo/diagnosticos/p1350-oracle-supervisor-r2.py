#!/usr/bin/env python3
"""P1350 R2 mandatory supervisor with a distinct 60-operation route domain."""

from __future__ import annotations

import ctypes
import errno
import hashlib
import hmac
import importlib.util
import json
import os
import secrets
import select
import shutil
import signal
import stat
import tempfile
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Any


MAX_CASE_NS = 12_000_000_000
TERM_GRACE_NS = 250_000_000
CLEANUP_NS = 2_000_000_000
MAX_FRAME = 65_536
GENESIS = "0" * 64
LINE_SCHEMA = "p1350-causal-journal-line-r1"
ENVELOPE_KEYS = ["closed_world", "current_sha256", "line_core", "previous_sha256", "schema"]
CORE_KEYS = ["capability_sha256", "case_id", "deadline_ns", "domain", "event", "event_ordinal", "operation", "order", "pgid", "pid", "pidfd_available", "pidfd_fallback_errno", "result_kind", "run_id", "sid", "signal_result", "starttime_ticks", "wait_result", "writer_pid"]
BLOCKER_CODES = {"ADAPTER_EXCEPTION", "AUTHORITY_UNAVAILABLE", "CHECKER_ERROR", "CLEANUP_FAILURE", "EXTERNAL_CLEANUP_REQUIRED", "FRAME_AUTHORITY", "INTERRUPTED", "JOURNAL_AUTHORITY", "ORPHAN_PROCESS", "SUPERVISOR_AUTHORITY", "SUPERVISOR_TIMEOUT"}
RESULT_KEYS = {
    "classification": ["case_id", "classification", "closed_world", "evidence", "reason_code", "schema"],
    "blocker": ["blocker_code", "case_id", "cleanup", "closed_world", "deadline", "exception", "phase", "schema", "supervisor", "verdict"],
}
PR_GET_CHILD_SUBREAPER = 37
PR_SET_CHILD_SUBREAPER = 36
LIBC = ctypes.CDLL(None, use_errno=True)


class SupervisorFailure(RuntimeError):
    def __init__(self, code: str, detail: str):
        self.code = code if code in BLOCKER_CODES else "CHECKER_ERROR"
        self.detail = detail
        super().__init__(f"{self.code}: {detail}")


def canonical(value: Any, lf: bool = False) -> bytes:
    raw = json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
    return raw + (b"\n" if lf else b"")


def sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def _write_all(fd: int, raw: bytes) -> None:
    offset = 0
    while offset < len(raw):
        size = os.write(fd, raw[offset:])
        if size <= 0:
            raise SupervisorFailure("SUPERVISOR_AUTHORITY", "short write")
        offset += size


def _duplicates(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise SupervisorFailure("FRAME_AUTHORITY", f"duplicate key {key}")
        result[key] = value
    return result


def strict_json(raw: bytes, label: str, code: str = "FRAME_AUTHORITY") -> Any:
    try:
        value = json.loads(raw, object_pairs_hook=_duplicates, parse_constant=lambda token: (_ for _ in ()).throw(ValueError(token)))
    except SupervisorFailure as exc:
        raise SupervisorFailure(code, exc.detail) from None
    except Exception as exc:
        raise SupervisorFailure(code, f"{label} JSON {type(exc).__name__}") from None
    if canonical(value, True) != raw:
        raise SupervisorFailure(code, f"{label} noncanonical")
    return value


def proc_identity(pid: int) -> dict[str, int]:
    try:
        raw = Path(f"/proc/{pid}/stat").read_text()
        tail = raw[raw.rfind(")") + 2:].split()
        return {"pgid": int(tail[2]), "pid": pid, "ppid": int(tail[1]), "sid": int(tail[3]), "starttime_ticks": int(tail[19])}
    except Exception:
        raise SupervisorFailure("SUPERVISOR_AUTHORITY", f"unavailable proc identity {pid}") from None


def fd_identity(fd: int) -> dict[str, int]:
    value = os.fstat(fd)
    return {"fd": fd, "st_dev": value.st_dev, "st_ino": value.st_ino, "st_mode": value.st_mode}


def _pidfd_open(pid: int) -> tuple[int, int | None]:
    try:
        return os.pidfd_open(pid, 0), None
    except OSError as exc:
        if exc.errno == errno.ENOSYS:
            return -1, errno.ENOSYS
        raise SupervisorFailure("AUTHORITY_UNAVAILABLE", f"pidfd_open errno {exc.errno}") from None


def _set_subreaper(enabled: int) -> None:
    if LIBC.prctl(PR_SET_CHILD_SUBREAPER, enabled, 0, 0, 0) != 0:
        raise SupervisorFailure("AUTHORITY_UNAVAILABLE", f"prctl set errno {ctypes.get_errno()}")


def _get_subreaper() -> int:
    value = ctypes.c_int()
    if LIBC.prctl(PR_GET_CHILD_SUBREAPER, ctypes.byref(value), 0, 0, 0) != 0:
        raise SupervisorFailure("AUTHORITY_UNAVAILABLE", f"prctl get errno {ctypes.get_errno()}")
    return value.value


def _closed(keys: list[str], rule: str) -> dict[str, Any]:
    return {"keys": keys, "rule": rule}


def integration_blocker(case_id: str, code: str, phase: str, detail: str, evidence: dict[str, Any]) -> dict[str, Any]:
    code = code if code in BLOCKER_CODES else "CHECKER_ERROR"
    return {
        "blocker_code": code,
        "case_id": case_id,
        "cleanup": evidence.get("cleanup", {}),
        "closed_world": _closed(RESULT_KEYS["blocker"], "Terminal P1350 control-plane failure; never classification, reason or score."),
        "deadline": evidence.get("deadline", {}),
        "exception": {"detail_sha256": sha256(detail.encode()), "type": "SupervisorFailure"},
        "phase": phase,
        "schema": "p1349-integration-blocker-r1",
        "supervisor": evidence.get("supervisor", {}),
        "verdict": "INTEGRATION_BLOCKED",
    }


class RunJournal:
    """Single-writer state. The verifier owns/opened fd; only this parent writes it."""

    def __init__(self, fd: int, run_id: str, expected_cases: int | None):
        if type(fd) is not int or fd < 0 or type(run_id) is not str or not run_id or (expected_cases is not None and (type(expected_cases) is not int or expected_cases < 1)):
            raise SupervisorFailure("JOURNAL_AUTHORITY", "run context")
        self.fd = fd
        self.run_id = run_id
        self.expected_cases = expected_cases
        self.writer_pid = os.getpid()
        self.previous = GENESIS
        self.event_ordinal = 0
        self.next_order = 0
        self.closed = False

    def append(self, event: str, case: dict[str, Any] | None, identity: dict[str, Any] | None, **fields: Any) -> str:
        if self.closed or os.getpid() != self.writer_pid:
            raise SupervisorFailure("JOURNAL_AUTHORITY", "writer authority")
        core = {
            "capability_sha256": fields.get("capability_sha256"),
            "case_id": None if case is None else case["case_id"],
            "deadline_ns": fields.get("deadline_ns"),
            "domain": None if case is None else case["domain"],
            "event": event,
            "event_ordinal": self.event_ordinal,
            "operation": None if case is None else case["operation"],
            "order": self.next_order if case is None else case["sequence_index"],
            "pgid": None if identity is None else identity["pgid"],
            "pid": None if identity is None else identity["pid"],
            "pidfd_available": fields.get("pidfd_available"),
            "pidfd_fallback_errno": fields.get("pidfd_fallback_errno"),
            "result_kind": fields.get("result_kind"),
            "run_id": self.run_id,
            "sid": None if identity is None else identity["sid"],
            "signal_result": fields.get("signal_result"),
            "starttime_ticks": None if identity is None else identity["starttime_ticks"],
            "wait_result": fields.get("wait_result"),
            "writer_pid": self.writer_pid,
        }
        if list(core) != CORE_KEYS:
            raise SupervisorFailure("JOURNAL_AUTHORITY", "core construction")
        current = sha256(canonical(["p1350-journal-line-r1", self.previous, core]))
        envelope = {
            "closed_world": _closed(ENVELOPE_KEYS, "Parent-authored causal evidence only; no semantic classification."),
            "current_sha256": current,
            "line_core": core,
            "previous_sha256": self.previous,
            "schema": LINE_SCHEMA,
        }
        raw = canonical(envelope, True)
        if len(raw) > 4096:
            raise SupervisorFailure("JOURNAL_AUTHORITY", "line limit")
        _write_all(self.fd, raw)
        os.fdatasync(self.fd)
        self.previous = current
        self.event_ordinal += 1
        if event == "case_result":
            self.next_order += 1
        return current

    def finish(self) -> str:
        if self.expected_cases is not None and self.next_order != self.expected_cases:
            raise SupervisorFailure("JOURNAL_AUTHORITY", "run cardinality")
        digest = self.append("run_complete", None, None, result_kind="complete")
        self.closed = True
        return digest


@dataclass(frozen=True)
class AdapterPlan:
    adapter_path: str
    adapter_sha256: str
    route_map_path: str
    route_map_sha256: str
    route_map_domain_sha256: str
    authority_root_sha256: str


def _read_pinned(root: Path, relative: str, digest: str) -> bytes:
    path = root / relative
    if path.is_symlink() or not path.is_file():
        raise SupervisorFailure("AUTHORITY_UNAVAILABLE", f"missing target {relative}")
    raw = path.read_bytes()
    if sha256(raw) != digest:
        raise SupervisorFailure("AUTHORITY_UNAVAILABLE", f"target drift {relative}")
    return raw


def validate_route_map(root: Path, plan: AdapterPlan) -> dict[str, Any]:
    value = strict_json(_read_pinned(root, plan.route_map_path, plan.route_map_sha256), "route map", "AUTHORITY_UNAVAILABLE")
    keys = ["closed_world", "entries", "inherited_count", "language_id_sequence_sha256", "local_count", "operation_ids_sha256", "schema", "total_count"]
    if type(value) is not dict or list(value) != keys or value["schema"] != "p1350-focal-operation-route-map-r2":
        raise SupervisorFailure("SUPERVISOR_AUTHORITY", "route map schema")
    bindings = value["entries"]
    if type(bindings) is not list or len(bindings) != 60:
        raise SupervisorFailure("SUPERVISOR_AUTHORITY", "route coverage cardinality")
    seen: set[str] = set(); inherited = local = 0; ordered: list[str] = []
    for row in bindings:
        if type(row) is not dict or list(row) != ["operation_id", "route"] or row["route"] not in ("INHERITED_P1348_P1347_P1346", "P1350_LOCAL") or row["operation_id"] in seen or type(row["operation_id"]) is not str:
            raise SupervisorFailure("SUPERVISOR_AUTHORITY", "route binding")
        seen.add(row["operation_id"]); ordered.append(row["operation_id"])
        inherited += row["route"] == "INHERITED_P1348_P1347_P1346"
        local += row["route"] == "P1350_LOCAL"
    operation_ids_sha256 = sha256(canonical(["p1350-focal-operation-ids-r2", ordered]))
    route_map_sha256 = sha256(canonical(["p1350-focal-operation-route-map-r2", value["language_id_sequence_sha256"], bindings]))
    if ordered != sorted(ordered, key=lambda item: item.encode("ascii")) or value["operation_ids_sha256"] != operation_ids_sha256 or value["total_count"] != 60 or value["inherited_count"] != 48 or value["local_count"] != 12 or inherited != 48 or local != 12 or route_map_sha256 != plan.route_map_domain_sha256:
        raise SupervisorFailure("SUPERVISOR_AUTHORITY", "route coverage 60/60")
    return value


def _close_unrelated(keep: set[int]) -> None:
    try:
        fds = [int(item.name) for item in Path("/proc/self/fd").iterdir() if item.name.isdigit()]
    except OSError:
        fds = list(range(3, 1024))
    for fd in fds:
        if fd > 2 and fd not in keep:
            try:
                os.close(fd)
            except OSError:
                pass


def register_descendant(control_fd: int, ack_fd: int, pid: int) -> None:
    _write_all(control_fd, canonical({"kind": "descendant", "identity": proc_identity(pid)}, True))
    if os.read(ack_fd, 1) != b"A":
        raise SupervisorFailure("SUPERVISOR_AUTHORITY", "descendant ACK")


def _child_main(root: Path, case: dict[str, Any], plan: AdapterPlan, boot_r: int, ctl_w: int, ack_r: int, out_w: int, err_w: int) -> None:
    try:
        os.setsid()
        os.dup2(out_w, 1); os.dup2(err_w, 2)
        _close_unrelated({boot_r, ctl_w, ack_r, out_w, err_w, 1, 2})
        capability = os.read(boot_r, 65)
        if len(capability) != 65 or not capability.endswith(b"\n"):
            os._exit(71)
        identity = proc_identity(os.getpid())
        handshake = {"capability": capability[:-1].decode(), "case_id": case["case_id"], "operation": case["operation"], "pgid": identity["pgid"], "pid": identity["pid"], "schema": "p1350-executor-handshake-r1", "sid": identity["sid"], "starttime_ticks": identity["starttime_ticks"]}
        _write_all(ctl_w, canonical({"kind": "handshake", "value": handshake}, True))
        if os.read(ack_r, 1) != b"A":
            os._exit(72)
        route_map = strict_json(_read_pinned(root, plan.route_map_path, plan.route_map_sha256), "child route map", "SUPERVISOR_AUTHORITY")
        matches = [row for row in route_map["entries"] if row["operation_id"] == case["operation"]]
        if case["domain"] == "meta":
            matches = [{"operation_id": case["operation"], "route": "P1350_LOCAL"}]
        if len(matches) != 1:
            raise SupervisorFailure("SUPERVISOR_AUTHORITY", "child route selection")
        _read_pinned(root, plan.adapter_path, plan.adapter_sha256)
        path = root / plan.adapter_path
        spec = importlib.util.spec_from_file_location(f"p1350_adapter_{os.getpid()}", path)
        if spec is None or spec.loader is None:
            raise SupervisorFailure("AUTHORITY_UNAVAILABLE", "adapter loader")
        adapter = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(adapter)
        entry_name = "execute_meta_in_executor" if case["domain"] == "meta" else "execute_in_executor"
        entry = getattr(adapter, entry_name, None)
        if not callable(entry):
            raise SupervisorFailure("ADAPTER_EXCEPTION", "adapter entry")
        context = {"ack_fd": ack_r, "authority_root_sha256": plan.authority_root_sha256, "control_fd": ctl_w, "register_descendant": register_descendant}
        result = entry(dict(case), context) if case["domain"] == "meta" else entry(dict(case), dict(matches[0]), context)
        _write_all(out_w, canonical(result, True))
        os._exit(0)
    except BaseException as exc:
        try:
            _write_all(err_w, f"{type(exc).__name__}:{str(exc)[:256]}".encode())
        except BaseException:
            pass
        os._exit(70)


def _read_lines(buffer: bytearray) -> list[bytes]:
    lines: list[bytes] = []
    while True:
        position = buffer.find(b"\n")
        if position < 0:
            return lines
        lines.append(bytes(buffer[:position + 1])); del buffer[:position + 1]


def _signal(pidfd: int, pid: int, sig: int) -> str:
    try:
        if pidfd >= 0:
            signal.pidfd_send_signal(pidfd, sig)
        else:
            os.kill(pid, sig)
        return "SENT"
    except ProcessLookupError:
        return "ESRCH"
    except OSError as exc:
        return f"ERRNO_{exc.errno}"


def _kill_group(pgid: int, sig: int) -> str:
    try:
        os.killpg(pgid, sig)
        return "SENT"
    except ProcessLookupError:
        return "ESRCH"
    except OSError as exc:
        return f"ERRNO_{exc.errno}"


def _group_absent(pgid: int) -> bool:
    return _kill_group(pgid, 0) == "ESRCH"


def _wait_group_absent(pgid: int, deadline_ns: int) -> bool:
    while time.monotonic_ns() < deadline_ns:
        if _group_absent(pgid):
            return True
        time.sleep(0.002)
    return _group_absent(pgid)


def _supervise_case(root: Path, case: dict[str, Any], plan: AdapterPlan, journal: RunJournal) -> tuple[dict[str, Any], dict[str, Any]]:
    """The only dynamic route: fork -> authenticated child -> adapter import/route."""
    started = time.monotonic_ns()
    case_root = tempfile.mkdtemp(prefix="p1350-oracle-", dir="/dev/shm")
    boot_r, boot_w = os.pipe2(os.O_CLOEXEC); ctl_r, ctl_w = os.pipe2(os.O_CLOEXEC)
    ack_r, ack_w = os.pipe2(os.O_CLOEXEC); out_r, out_w = os.pipe2(os.O_CLOEXEC); err_r, err_w = os.pipe2(os.O_CLOEXEC)
    old_subreaper = _get_subreaper(); _set_subreaper(1)
    pid = os.fork()
    if pid == 0:
        os.close(boot_w); os.close(ctl_r); os.close(ack_w); os.close(out_r); os.close(err_r)
        _child_main(root, case, plan, boot_r, ctl_w, ack_r, out_w, err_w)
        os._exit(73)
    os.close(boot_r); os.close(ctl_w); os.close(ack_r); os.close(out_w); os.close(err_w)
    retained = [boot_w, ctl_r, ack_w, out_r, err_r]
    pidfd = -1; fallback_errno: int | None = None; terminal: int | None = None
    registered: dict[int, tuple[dict[str, int], int]] = {}
    result_raw = bytearray(); stderr_raw = bytearray(); ctl_buffer = bytearray()
    identity = {"pid": pid, "ppid": os.getpid(), "pgid": pid, "sid": pid, "starttime_ticks": proc_identity(pid)["starttime_ticks"]}
    capability = ""; deadline_ns = started + MAX_CASE_NS; blocker: tuple[str, str] | None = None
    frame_line_sha: str | None = None; term_result: str | None = None; kill_result: str | None = None
    try:
        pidfd, fallback_errno = _pidfd_open(pid)
        secret, nonce = secrets.token_bytes(32), secrets.token_bytes(32)
        capability = hmac.new(secret, canonical(["p1350-executor-capability-r1", journal.run_id, case["sequence_index"], case["case_id"], case["operation"], pid, identity["starttime_ticks"], nonce.hex()]), hashlib.sha256).hexdigest()
        cap_sha = sha256(capability.encode())
        journal.append("case_start", case, identity, capability_sha256=cap_sha, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback_errno)
        _write_all(boot_w, capability.encode() + b"\n"); os.close(boot_w); retained.remove(boot_w)
        for fd in (ctl_r, out_r, err_r): os.set_blocking(fd, False)
        handshake_ok = False
        while not handshake_ok and time.monotonic_ns() < deadline_ns:
            ready, _, _ = select.select([ctl_r, err_r, pidfd] if pidfd >= 0 else [ctl_r, err_r], [], [], 0.05)
            if pidfd >= 0 and pidfd in ready:
                blocker = ("SUPERVISOR_AUTHORITY", "executor terminal before handshake"); break
            for fd in ready:
                if fd == err_r:
                    stderr_raw.extend(os.read(err_r, MAX_FRAME + 1)); continue
                chunk = os.read(ctl_r, MAX_FRAME + 1)
                if not chunk:
                    blocker = ("SUPERVISOR_AUTHORITY", "control EOF before handshake"); break
                ctl_buffer.extend(chunk)
                for raw in _read_lines(ctl_buffer):
                    message = strict_json(raw, "control handshake", "SUPERVISOR_AUTHORITY")
                    if type(message) is not dict or list(message) != ["kind", "value"] or message["kind"] != "handshake":
                        raise SupervisorFailure("SUPERVISOR_AUTHORITY", "first control frame")
                    hs = message["value"]
                    if type(hs) is not dict or list(hs) != ["capability", "case_id", "operation", "pgid", "pid", "schema", "sid", "starttime_ticks"]:
                        raise SupervisorFailure("SUPERVISOR_AUTHORITY", "handshake schema")
                    observed = proc_identity(pid)
                    if hs != {"capability": capability, "case_id": case["case_id"], "operation": case["operation"], "pgid": pid, "pid": pid, "schema": "p1350-executor-handshake-r1", "sid": pid, "starttime_ticks": identity["starttime_ticks"]} or observed != identity:
                        raise SupervisorFailure("SUPERVISOR_AUTHORITY", "handshake identity")
                    handshake_ok = True
        if not handshake_ok and blocker is None:
            blocker = ("SUPERVISOR_TIMEOUT", "handshake deadline")
        if handshake_ok:
            journal.append("executor_bound", case, identity, capability_sha256=cap_sha, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback_errno)
            deadline_ns = time.monotonic_ns() + MAX_CASE_NS
            journal.append("deadline_armed", case, identity, capability_sha256=cap_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback_errno)
            _write_all(ack_w, b"A")
        live = {ctl_r, out_r, err_r}
        while terminal is None and blocker is None:
            if time.monotonic_ns() >= deadline_ns:
                blocker = ("SUPERVISOR_TIMEOUT", "case deadline"); break
            watch = list(live) + ([pidfd] if pidfd >= 0 else [])
            ready, _, _ = select.select(watch, [], [], 0.05)
            for fd in ready:
                if fd == pidfd:
                    _, terminal = os.waitpid(pid, 0); break
                try: chunk = os.read(fd, MAX_FRAME + 1)
                except BlockingIOError: continue
                if not chunk:
                    live.discard(fd); continue
                if fd == out_r:
                    result_raw.extend(chunk)
                    if len(result_raw) > MAX_FRAME: blocker = ("FRAME_AUTHORITY", "oversized frame")
                elif fd == err_r:
                    stderr_raw.extend(chunk)
                    if len(stderr_raw) > MAX_FRAME: blocker = ("FRAME_AUTHORITY", "oversized stderr")
                else:
                    ctl_buffer.extend(chunk)
                    for raw in _read_lines(ctl_buffer):
                        message = strict_json(raw, "descendant registration", "SUPERVISOR_AUTHORITY")
                        if type(message) is not dict or list(message) != ["identity", "kind"] or message["kind"] != "descendant":
                            raise SupervisorFailure("SUPERVISOR_AUTHORITY", "descendant frame")
                        observed = proc_identity(message["identity"]["pid"])
                        if observed != message["identity"] or observed["sid"] != pid or observed["pgid"] != pid or observed["ppid"] not in ({pid} | set(registered)) or observed["pid"] in registered:
                            raise SupervisorFailure("SUPERVISOR_AUTHORITY", "descendant identity")
                        child_pidfd, child_fallback = _pidfd_open(observed["pid"])
                        if child_fallback is not None:
                            raise SupervisorFailure("AUTHORITY_UNAVAILABLE", "descendant pidfd ENOSYS unsupported")
                        registered[observed["pid"]] = (observed, child_pidfd); _write_all(ack_w, b"A")
            if pidfd < 0 and terminal is None:
                waited, status = os.waitpid(pid, os.WNOHANG)
                if waited == pid: terminal = status
        for fd, target in ((out_r, result_raw), (err_r, stderr_raw), (ctl_r, ctl_buffer)):
            while True:
                try:
                    chunk = os.read(fd, MAX_FRAME + 1)
                except (BlockingIOError, OSError):
                    break
                if not chunk:
                    break
                target.extend(chunk)
                if len(target) > MAX_FRAME:
                    blocker = blocker or ("FRAME_AUTHORITY", "post-terminal output limit")
        if result_raw and len(result_raw) <= MAX_FRAME and not stderr_raw:
            try:
                value = strict_json(bytes(result_raw), "executor frame")
                if type(value) is not dict or value.get("schema") not in ("p1349-classification-result-r1", "p1349-meta-result-r1", "p1349-integration-blocker-r1"):
                    raise SupervisorFailure("FRAME_AUTHORITY", "frame domain")
                frame_line_sha = sha256(bytes(result_raw))
                journal.append("frame_received", case, identity, capability_sha256=cap_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback_errno, result_kind=value["schema"])
            except SupervisorFailure as exc:
                blocker = blocker or (exc.code, exc.detail)
        elif blocker is None:
            blocker = ("ADAPTER_EXCEPTION", "executor failed without canonical frame") if stderr_raw else ("FRAME_AUTHORITY", "missing frame")
        if not _group_absent(pid):
            term_result = _kill_group(pid, signal.SIGTERM)
            for child_pid, (_, child_fd) in registered.items(): _signal(child_fd, child_pid, signal.SIGTERM)
            journal.append("term_sent", case, identity, capability_sha256=cap_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback_errno, signal_result=term_result)
            if not _wait_group_absent(pid, time.monotonic_ns() + TERM_GRACE_NS):
                kill_result = _kill_group(pid, signal.SIGKILL)
                for child_pid, (_, child_fd) in registered.items(): _signal(child_fd, child_pid, signal.SIGKILL)
                journal.append("kill_sent", case, identity, capability_sha256=cap_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback_errno, signal_result=kill_result)
        cleanup_deadline = time.monotonic_ns() + CLEANUP_NS
        if terminal is None:
            while time.monotonic_ns() < cleanup_deadline:
                waited, status = os.waitpid(pid, os.WNOHANG)
                if waited == pid: terminal = status; break
                time.sleep(0.002)
        reaped: list[int] = [pid] if terminal is not None else []
        for child_pid in registered:
            while time.monotonic_ns() < cleanup_deadline:
                try:
                    waited, _ = os.waitpid(child_pid, os.WNOHANG)
                except ChildProcessError:
                    waited = child_pid
                if waited == child_pid: reaped.append(child_pid); break
                time.sleep(0.002)
        group_absent = _wait_group_absent(pid, cleanup_deadline)
        if terminal is None: blocker = blocker or ("CLEANUP_FAILURE", "executor not reaped")
        if not group_absent: blocker = blocker or ("ORPHAN_PROCESS", "process group remains")
        wait_value = None if terminal is None else {"raw": terminal, "signaled": os.WIFSIGNALED(terminal), "exited": os.WIFEXITED(terminal)}
        journal.append("terminal_wait", case, identity, capability_sha256=cap_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback_errno, wait_result=wait_value)
        journal.append("group_absent", case, identity, capability_sha256=cap_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback_errno, signal_result="ESRCH" if group_absent else "PRESENT")
        for fd in retained + ([pidfd] if pidfd >= 0 else []):
            try: os.close(fd)
            except OSError: pass
        for _, child_fd in registered.values():
            try: os.close(child_fd)
            except OSError: pass
        shutil.rmtree(case_root, ignore_errors=True)
        shm_removed = not Path(case_root).exists()
        _set_subreaper(old_subreaper)
        if not shm_removed: blocker = blocker or ("CLEANUP_FAILURE", "case root remains")
        cleanup = {"descriptors_closed": True, "group_absent": group_absent, "kill_sent": kill_result is not None, "patches_restored": True, "reaped_pids": sorted(reaped), "registry_destroyed": True, "shm_removed": shm_removed, "term_sent": term_result is not None, "zombies_absent": terminal is not None and len(reaped) == 1 + len(registered)}
        deadline = {"armed_ns": deadline_ns - MAX_CASE_NS, "cleanup_complete_ns": time.monotonic_ns(), "expired": blocker is not None and blocker[0] == "SUPERVISOR_TIMEOUT", "maximum_case_duration_ns": MAX_CASE_NS, "monotonic_start_ns": started, "term_grace_ns": TERM_GRACE_NS}
        supervisor = {"capability_sha256": cap_sha, "executor_pid": pid, "pidfd_available": pidfd >= 0, "pidfd_fallback_errno": fallback_errno, "registered_descendants": sorted(registered), "session_id": pid, "starttime_ticks": identity["starttime_ticks"]}
        evidence = {"cleanup": cleanup, "deadline": deadline, "frame_sha256": frame_line_sha, "supervisor": supervisor}
        result = integration_blocker(case["case_id"], blocker[0], "supervisor", blocker[1], evidence) if blocker else value
        result_kind = result["schema"] if type(result) is dict else "invalid"
        journal.append("case_result", case, identity, capability_sha256=cap_sha, deadline_ns=deadline_ns, pidfd_available=pidfd >= 0, pidfd_fallback_errno=fallback_errno, result_kind=result_kind)
        return result, evidence
    except KeyboardInterrupt:
        _kill_group(pid, signal.SIGKILL)
        try: os.waitpid(pid, 0)
        except (OSError, ChildProcessError): pass
        raise SupervisorFailure("INTERRUPTED", "supervisor interrupted") from None
    except SupervisorFailure:
        _kill_group(pid, signal.SIGKILL)
        try: os.waitpid(pid, 0)
        except (OSError, ChildProcessError): pass
        raise
    finally:
        for fd in retained + ([pidfd] if pidfd >= 0 else []):
            try: os.close(fd)
            except OSError: pass
        for _, child_fd in registered.values():
            try: os.close(child_fd)
            except OSError: pass
        shutil.rmtree(case_root, ignore_errors=True)
        try: _set_subreaper(old_subreaper)
        except SupervisorFailure: pass
