#!/usr/bin/env python3
"""P1348 parent-transaction, freshness, reason and full-trace oracle."""

from __future__ import annotations

import copy
import ctypes
import fcntl
import hashlib
import hmac
import importlib.util
import json
import os
import secrets
import select
import signal
import stat
import subprocess
import sys
import tempfile
import threading
import time
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
CHECKER_REL = "00_nucleo/diagnosticos/p1348-oracle-checker-r1.py"
CORPUS_REL = "00_nucleo/diagnosticos/p1348-oracle-corpus-r1.json"
MANIFEST_REL = "00_nucleo/diagnosticos/p1348-authority-manifest-r1.json"
RECEIPT_REL = "00_nucleo/diagnosticos/p1348-oracle-authorship-receipt-r1.json"
CHECKER_PATH = ROOT / CHECKER_REL
CORPUS_PATH = ROOT / CORPUS_REL
RECEIPT_PATH = ROOT / RECEIPT_REL
BASE_PATH = DIAG / "p1347-oracle-checker-r2.py"
PYTHON = Path("/usr/bin/python3")
EXPECTED_BASE_SHA256 = "9aa0e93591195eac7ddb67069abecbcf0722abeddd9e955fc062804a89d16ce5"
EXPECTED_MANIFEST_SHA256 = "70754da5c833a43f1943ec7abdb453f67e8889b146b27b883e04097531bad6bb"
EXPECTED_CORPUS_SHA256 = "3e33630290cdfe681e855dee4c3becfde0aac53560d6d1c72c4e0fe83bb25458"
CONTRACT_ROOT_SHA256 = "fef0a3d2c5be21c76a29562e18425679bacad6327a77744d24c49f8ef48bc8bd"
SEQUENCE_SHA256 = "e958b9c5e939a4b06b9e8594966001004de814e49e0141034d5351f1dbf4a907"
PINS = {
    "00_nucleo/materialization/typst-passo-1348.md": "4254d294e71427c7dd54ab28db674d7b20174e560bd4882bf3413dae217224c7",
    "00_nucleo/materialization/typst-passo-1347.md": "c848a04d9bc3aaa9ed935ea8fef6e615cdb77a0533c13d0fcd62ef1a87ab9a1c",
    "00_nucleo/diagnosticos/p1348-contract-spec-r1.json": "e9e3e10b069604f7bc45143da4625e728c34d6d3b7223c7fa9491173fa7131af",
    "00_nucleo/diagnosticos/p1348-contract-binding-r1.json": "de05f7c3a33bae7e131bca1aec0774a1fc370cdb3a6f43d50b1e2d050db5785f",
    "00_nucleo/diagnosticos/p1348-contract-receipt-r1.json": "1f49bdd80627edb61f115da108eca0b4216050bd81aaba093839e051f31b0daf",
    "00_nucleo/diagnosticos/p1347-verifier-blocker-r1.json": "3b649ced677059a459262710ce3466cb2efbd69edbfb6e60c98df07cffa64522",
    "00_nucleo/diagnosticos/p1347-contract-spec-r1.json": "fac6e30c7bbee3b50eec6cad8c8a9a8b30f3c09027f63293e73b4bf2341db0ea",
    "00_nucleo/diagnosticos/p1347-contract-binding-r1.json": "de1dcc87e77ab5fd6c8395fa4faacfd74fd90769dcafb5d2ef5c0bd30fda5fd1",
    "00_nucleo/diagnosticos/p1347-contract-receipt-r1.json": "85c6ad312cf9ac83ff653e2a19ac96fdd9e29ad72d2eb4af90fe8fff14fafb1d",
    "00_nucleo/diagnosticos/p1347-oracle-checker-r2.py": EXPECTED_BASE_SHA256,
    "00_nucleo/diagnosticos/p1347-oracle-caller-r2.py": "b377d09b912f4996d0712d67e9cd4b0b0eb0f7acc646f48f07d55fcf58f6071e",
    "00_nucleo/diagnosticos/p1347-oracle-delivery-receipt-r2.json": "a3e7cb3e6f29bba6293187ae967bd303f3f2ebebb79dd128679787d3b433432d",
    "00_nucleo/diagnosticos/p1347-oracle-corpus-r1.json": "24f2aa18c2f39f1647d29d9fe74c2d5a2cfc9079f812bced92a7d60456e12c2b",
    "00_nucleo/diagnosticos/p1347-oracle-worker-r1.py": "3f31593b545b7bec2160167c4395bf4bebe78a4111b194c74f4c775a4372afc5",
    "00_nucleo/diagnosticos/p1347-oracle-authorship-r2.md": "db1cda1e1a52192a4e74ecc624f6dce17d0c8126029daa44872f9e98c499b1fb",
    "00_nucleo/diagnosticos/p1347-oracle-authorship-receipt-r2.json": "fb66120dd74f102139e2db839516c716793c5d852dfce1f6fb414d15b67a681c",
}
PTRACE_TRACEME = 0
PTRACE_CONT = 7
PTRACE_GETEVENTMSG = 0x4201
PTRACE_SETOPTIONS = 0x4200
PTRACE_O_TRACEEXEC = 0x10
PTRACE_O_TRACEEXIT = 0x40
PTRACE_O_EXITKILL = 0x100000
PTRACE_EVENT_EXEC = 4
PTRACE_EVENT_EXIT = 6
PTRACE_OPTIONS = 1048656
LIBC = ctypes.CDLL(None, use_errno=True)


class Failure(RuntimeError):
    def __init__(self, code: str, detail: str):
        self.code = code
        self.detail = detail
        super().__init__(f"{code}: {detail}")


class IntegrationBlocker(RuntimeError):
    pass


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical(value: Any, trailing_lf: bool = True) -> bytes:
    raw = json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
    return raw + (b"\n" if trailing_lf else b"")


def reject_duplicate(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise Failure("DUPLICATE_KEY", key)
        result[key] = value
    return result


def strict_json(raw: bytes, label: str, canonical_required: bool = True) -> Any:
    try:
        value = json.loads(raw, object_pairs_hook=reject_duplicate, parse_constant=lambda token: (_ for _ in ()).throw(ValueError(token)))
    except Failure:
        raise
    except Exception as exc:
        raise Failure("SCHEMA", f"{label} JSON {type(exc).__name__}") from None
    if canonical_required and canonical(value) != raw:
        raise Failure("SCHEMA", f"{label} noncanonical")
    return value


def exact_keys(value: Any, keys: list[str], label: str) -> dict[str, Any]:
    if type(value) is not dict or sorted(value) != keys:
        raise Failure("SCHEMA", f"{label} keys")
    return value


def exact_hex(value: Any, label: str) -> str:
    if type(value) is not str or len(value) != 64 or any(char not in "0123456789abcdef" for char in value):
        raise Failure("SCHEMA", label)
    return value


def read_pinned(relative: str, digest: str) -> bytes:
    path = ROOT / relative
    if path.is_symlink() or not path.is_file():
        raise Failure("PROTECTED_INPUT", relative)
    raw = path.read_bytes()
    if sha256(raw) != digest:
        raise Failure("PROTECTED_INPUT", relative)
    return raw


def load_base() -> Any:
    read_pinned(str(BASE_PATH.relative_to(ROOT)), EXPECTED_BASE_SHA256)
    spec = importlib.util.spec_from_file_location("p1347_checker_r2_pinned_by_p1348", BASE_PATH)
    if spec is None or spec.loader is None:
        raise Failure("PROTECTED_INPUT", "P1347 checker loader")
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
    if PTRACE_OPTIONS != PTRACE_O_TRACEEXEC | PTRACE_O_TRACEEXIT | PTRACE_O_EXITKILL:
        raise Failure("PROTECTED_INPUT", "ptrace option mask")


def load_corpus() -> dict[str, Any]:
    value = exact_keys(strict_json(read_pinned(CORPUS_REL, EXPECTED_CORPUS_SHA256), "P1348 corpus"), ["budget", "cases", "closed_world", "controls", "manifest_sha256", "regime", "revision", "role", "schema", "step"], "P1348 corpus")
    if value["schema"] != "p1348-oracle-corpus-r1" or value["manifest_sha256"] != EXPECTED_MANIFEST_SHA256:
        raise Failure("SCHEMA", "corpus identity")
    for group in (value["cases"], value["controls"]):
        for row in group:
            exact_keys(row, ["case_id", "family", "operation"], "corpus row")
    return value


def oracle_authoring_root(checker_sha256: str) -> str:
    exact_hex(checker_sha256, "checker")
    return sha256(canonical(["p1348-oracle-authoring-root-r1", CONTRACT_ROOT_SHA256, EXPECTED_MANIFEST_SHA256, EXPECTED_CORPUS_SHA256, checker_sha256], False))


def oracle_delivery_root(authoring_root_sha256: str, checker_sha256: str, receipt_sha256: str, caller_sha256: str) -> str:
    return sha256(canonical(["p1348-oracle-delivery-root-r1", authoring_root_sha256, checker_sha256, receipt_sha256, caller_sha256], False))


def validate_authorship_receipt(receipt_sha256: str, checker_sha256: str, root_sha256: str) -> dict[str, Any]:
    receipt = strict_json(read_pinned(RECEIPT_REL, receipt_sha256), "P1348 authorship receipt")
    if receipt.get("schema") != "p1348-oracle-authorship-receipt-r1" or receipt.get("manifest_sha256") != EXPECTED_MANIFEST_SHA256 or receipt.get("authoring_root_sha256") != root_sha256 or oracle_authoring_root(checker_sha256) != root_sha256:
        raise Failure("AUTHORITY_ROOT", "P1348 authorship root")
    if receipt.get("outputs") != [[MANIFEST_REL, EXPECTED_MANIFEST_SHA256], [CORPUS_REL, EXPECTED_CORPUS_SHA256], [CHECKER_REL, checker_sha256]]:
        raise Failure("AUTHORITY_ROOT", "P1348 receipt outputs")
    if any(key in receipt for key in ("caller", "caller_sha256", "delivery_receipt", "delivery_receipt_sha256", "self_sha256")):
        raise Failure("AUTHORITY_ROOT", "cyclic authorship receipt")
    return receipt


def fd_identity(fd: int) -> dict[str, int]:
    value = os.fstat(fd)
    return {"fd": fd, "st_dev": value.st_dev, "st_ino": value.st_ino, "st_mode": value.st_mode}


class ParentSession:
    def __init__(self) -> None:
        self.session_id = secrets.token_hex(32)
        self._secret = secrets.token_bytes(32)
        self._lock = threading.Lock()
        self._transactions: dict[str, tuple[dict[str, Any], bytes, ParentProcessTransaction]] = {}

    def seal(self, transaction: "ParentProcessTransaction") -> None:
        record = copy.deepcopy(transaction._record)
        message = canonical(["p1348-parent-transaction-r1", transaction.transaction_id, record], False)
        tag = hmac.new(self._secret, message, hashlib.sha256).digest()
        with self._lock:
            if transaction.transaction_id in self._transactions:
                raise Failure("WORKER_AUTHORITY", "duplicate transaction")
            self._transactions[transaction.transaction_id] = (record, tag, transaction)
        transaction._tag = tag

    def verify(self, transaction: "ParentProcessTransaction") -> dict[str, Any]:
        if type(transaction) is not ParentProcessTransaction or transaction.session is not self:
            raise Failure("WORKER_AUTHORITY", "non-authoritative transaction")
        with self._lock:
            stored = self._transactions.get(transaction.transaction_id)
        if stored is None or stored[2] is not transaction:
            raise Failure("WORKER_AUTHORITY", "unknown transaction")
        record, tag, _ = stored
        message = canonical(["p1348-parent-transaction-r1", transaction.transaction_id, transaction._record], False)
        observed = hmac.new(self._secret, message, hashlib.sha256).digest()
        if record != transaction._record or transaction._tag is None or not hmac.compare_digest(tag, transaction._tag) or not hmac.compare_digest(tag, observed):
            raise Failure("WORKER_AUTHORITY", "transaction seal")
        return copy.deepcopy(record)


class ParentProcessTransaction:
    __slots__ = ("session", "transaction_id", "_record", "_tag")

    def __init__(self, session: ParentSession, transaction_id: str) -> None:
        self.session = session
        self.transaction_id = transaction_id
        self._record: dict[str, Any] = {}
        self._tag: bytes | None = None

    def project(self, invocation_sha256: str, commit_ordinal: int, registry_sha256: str) -> dict[str, Any]:
        record = self.session.verify(self)
        raw = record["raw_wait_status"]
        return {
            "case_sequence_sha256": record["case_sequence_sha256"],
            "child_exit_code": os.WEXITSTATUS(raw) if os.WIFEXITED(raw) else None,
            "child_pid": record["child_pid"],
            "child_signal": os.WTERMSIG(raw) if os.WIFSIGNALED(raw) else None,
            "closed_world": {"rule": "Output-only projection of a sealed parent transaction."},
            "invocation_sha256": invocation_sha256,
            "pidfd_identity": record["pidfd_identity"],
            "pipe_identities": record["pipe_identities"],
            "raw_wait_status": raw,
            "registry_commit_ordinal": commit_ordinal,
            "registry_sha256": registry_sha256,
            "schedule_slot": record["schedule_slot"],
            "schema": "p1348-parent-worker-evidence-r1",
            "stderr_len": record["stderr_len"],
            "stderr_sha256": record["stderr_sha256"],
            "stdout_len": record["stdout_len"],
            "stdout_sha256": record["stdout_sha256"],
            "transaction_id": self.transaction_id,
            "waitid_code": record["waitid_code"],
            "waitid_pid": record["waitid_pid"],
            "waitid_status": record["waitid_status"],
            "waitpid_pid": record["waitpid_pid"],
            "worker_response_sha256": record["stdout_sha256"],
        }


class ParentFreshnessRegistry:
    def __init__(self, slots: list[str]) -> None:
        if not slots or len(slots) != len(set(slots)) or any(type(slot) is not str or not slot.isascii() for slot in slots):
            raise Failure("WORKER_AUTHORITY", "registry plan")
        self.session = ParentSession()
        self._slots = tuple(slots)
        self._next = 0
        self._pending: dict[str, dict[str, str]] = {}
        self._consumed: list[dict[str, Any]] = []
        self._lock = threading.Lock()

    @staticmethod
    def identity(challenge: bytes, nonce: bytes, invocation_sha256: str, sequence_sha256: str, slot: str) -> str:
        if len(challenge) != 32 or len(nonce) != 32 or challenge == nonce:
            raise Failure("WORKER_AUTHORITY", "fresh challenge/nonce")
        exact_hex(invocation_sha256, "invocation")
        exact_hex(sequence_sha256, "sequence")
        return sha256(canonical(["p1348-execution-identity-r1", challenge.hex(), nonce.hex(), invocation_sha256, sequence_sha256, slot], False))

    def begin(self, challenge: bytes, nonce: bytes, invocation_sha256: str, sequence_sha256: str, slot: str, transaction_id: str) -> tuple[str, str]:
        identity = self.identity(challenge, nonce, invocation_sha256, sequence_sha256, slot)
        token = secrets.token_hex(32)
        with self._lock:
            if self._next >= len(self._slots) or slot != self._slots[self._next] or identity in self._pending or any(item["identity_sha256"] == identity for item in self._consumed):
                raise Failure("WORKER_AUTHORITY", "freshness slot/replay")
            self._pending[identity] = {"identity_sha256": identity, "parent_token_sha256": sha256(bytes.fromhex(token)), "schedule_slot": slot, "transaction_id": transaction_id}
        return identity, token

    def rollback(self, identity: str, token: str, transaction_id: str) -> None:
        with self._lock:
            pending = self._pending.get(identity)
            if pending is not None and pending == {"identity_sha256": identity, "parent_token_sha256": sha256(bytes.fromhex(token)), "schedule_slot": pending["schedule_slot"], "transaction_id": transaction_id}:
                del self._pending[identity]

    def commit(self, identity: str, token: str, transaction: ParentProcessTransaction) -> tuple[int, str]:
        transaction.session.verify(transaction)
        with self._lock:
            pending = self._pending.get(identity)
            if pending is None:
                raise Failure("WORKER_AUTHORITY", "identity not pending")
            if self._next >= len(self._slots) or pending["schedule_slot"] != self._slots[self._next] or pending["parent_token_sha256"] != sha256(bytes.fromhex(token)) or pending["transaction_id"] != transaction.transaction_id:
                raise Failure("WORKER_AUTHORITY", "freshness commit mismatch")
            ordinal = len(self._consumed)
            self._consumed.append({"commit_ordinal": ordinal, "identity_sha256": identity, "schedule_slot": pending["schedule_slot"], "transaction_id": transaction.transaction_id})
            del self._pending[identity]
            self._next += 1
            digest = self.digest_locked()
        return ordinal, digest

    def digest_locked(self) -> str:
        rows = [[item["commit_ordinal"], item["schedule_slot"], item["identity_sha256"], item["transaction_id"]] for item in self._consumed]
        return sha256(canonical(rows, False))

    def state(self) -> dict[str, Any]:
        with self._lock:
            return {"consumed": copy.deepcopy(self._consumed), "next_slot": self._slots[self._next] if self._next < len(self._slots) else None, "pending_count": len(self._pending), "registry_sha256": self.digest_locked()}

    def fail_after_commit(self, identity: str) -> None:
        with self._lock:
            if not any(item["identity_sha256"] == identity for item in self._consumed):
                raise Failure("WORKER_AUTHORITY", "postcommit identity absent")
        raise IntegrationBlocker("failure after atomic commit; identity remains consumed")


def _pipe_pair(read_fd: int, write_fd: int) -> dict[str, Any]:
    left, right = fd_identity(read_fd), fd_identity(write_fd)
    if (left["st_dev"], left["st_ino"]) != (right["st_dev"], right["st_ino"]) or not stat.S_ISFIFO(left["st_mode"]) or not stat.S_ISFIFO(right["st_mode"]):
        raise Failure("WORKER_AUTHORITY", "pipe identity")
    return {"read": left, "write": right}


def capture_process(session: ParentSession, transaction_id: str, schedule_slot: str, argv: list[str], request: bytes, env_extra: dict[str, str] | None = None, timeout: float = 10.0) -> tuple[ParentProcessTransaction, bytes, bytes]:
    transaction = ParentProcessTransaction(session, transaction_id)
    in_read, in_write = os.pipe2(os.O_CLOEXEC); out_read, out_write = os.pipe2(os.O_CLOEXEC); err_read, err_write = os.pipe2(os.O_CLOEXEC)
    before = {"stdin": _pipe_pair(in_read, in_write), "stdout": _pipe_pair(out_read, out_write), "stderr": _pipe_pair(err_read, err_write)}
    pid = os.fork()
    if pid == 0:
        try:
            os.dup2(in_read, 0); os.dup2(out_write, 1); os.dup2(err_write, 2)
            for fd in (in_read, in_write, out_read, out_write, err_read, err_write):
                if fd > 2:
                    try: os.close(fd)
                    except OSError: pass
            env = {"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C", "PYTHONDWRITEBYTECODE": "1"}
            if env_extra: env.update(env_extra)
            os.execve(argv[0], argv, env)
        except BaseException:
            os._exit(126)
    os.close(in_read); os.close(out_write); os.close(err_write)
    pidfd = -1; reaped = False; out = bytearray(); err = bytearray()
    try:
        pidfd = os.pidfd_open(pid, 0)
        pidfd_before = fd_identity(pidfd)
        parent_after = {"stdin_write": fd_identity(in_write), "stdout_read": fd_identity(out_read), "stderr_read": fd_identity(err_read)}
        offset = 0
        while offset < len(request):
            written = os.write(in_write, request[offset:])
            if written <= 0: raise Failure("WORKER_AUTHORITY", "stdin write")
            offset += written
        os.close(in_write); in_write = -1
        streams: dict[int, bytearray] = {out_read: out, err_read: err}; deadline = time.monotonic() + timeout
        while streams:
            remaining = deadline - time.monotonic()
            if remaining <= 0: raise Failure("WORKER_AUTHORITY", "timeout")
            ready, _, _ = select.select(list(streams), [], [], remaining)
            if not ready: raise Failure("WORKER_AUTHORITY", "timeout")
            for fd in ready:
                chunk = os.read(fd, 65536)
                if chunk:
                    streams[fd].extend(chunk)
                    if len(out) + len(err) > 8_000_000: raise Failure("WORKER_AUTHORITY", "output limit")
                else:
                    os.close(fd); streams.pop(fd)
        if fd_identity(pidfd) != pidfd_before: raise Failure("WORKER_AUTHORITY", "pidfd substitution")
        info = os.waitid(os.P_PIDFD, pidfd, os.WEXITED | os.WNOWAIT)
        waited_pid, raw = os.waitpid(pid, 0); reaped = True
        transaction._record = {
            "case_sequence_sha256": SEQUENCE_SHA256,
            "child_pid": pid,
            "pidfd_identity": pidfd_before,
            "pipe_identities": {"before_fork": before, "parent_after_close": parent_after},
            "raw_wait_status": raw,
            "schedule_slot": schedule_slot,
            "session_id": session.session_id,
            "stderr_len": len(err),
            "stderr_sha256": sha256(bytes(err)),
            "stdout_len": len(out),
            "stdout_sha256": sha256(bytes(out)),
            "transaction_id": transaction_id,
            "waitid_code": info.si_code,
            "waitid_pid": info.si_pid,
            "waitid_status": info.si_status,
            "waitpid_pid": waited_pid,
        }
        session.seal(transaction)
        return transaction, bytes(out), bytes(err)
    except Exception:
        if not reaped:
            try: os.kill(pid, signal.SIGKILL)
            except OSError: pass
            try: os.waitpid(pid, 0)
            except OSError: pass
        raise
    finally:
        for fd in (in_write, out_read, err_read, pidfd):
            if fd >= 0:
                try: os.close(fd)
                except OSError: pass


def validate_transaction(transaction: ParentProcessTransaction, raw_stdout: bytes, raw_stderr: bytes, require_success: bool = True) -> dict[str, Any]:
    if type(transaction) is not ParentProcessTransaction:
        raise Failure("WORKER_AUTHORITY", "public evidence is not authority")
    record = transaction.session.verify(transaction)
    if record["child_pid"] <= 0 or type(record["child_pid"]) is not int or record["waitid_pid"] != record["child_pid"] or record["waitpid_pid"] != record["child_pid"]:
        raise Failure("WORKER_AUTHORITY", "PID binding")
    pipes = record["pipe_identities"]
    before, after = pipes["before_fork"], pipes["parent_after_close"]
    if after["stdin_write"] != before["stdin"]["write"] or after["stdout_read"] != before["stdout"]["read"] or after["stderr_read"] != before["stderr"]["read"]:
        raise Failure("WORKER_AUTHORITY", "pipe endpoint substitution")
    if record["stdout_len"] != len(raw_stdout) or record["stdout_sha256"] != sha256(raw_stdout) or record["stderr_len"] != len(raw_stderr) or record["stderr_sha256"] != sha256(raw_stderr):
        raise Failure("WORKER_AUTHORITY", "stream binding")
    status = record["raw_wait_status"]
    if os.WIFEXITED(status):
        if record["waitid_code"] != os.CLD_EXITED or record["waitid_status"] != os.WEXITSTATUS(status): raise Failure("WORKER_AUTHORITY", "wait status binding")
    elif os.WIFSIGNALED(status):
        if record["waitid_status"] != os.WTERMSIG(status): raise Failure("WORKER_AUTHORITY", "signal binding")
    else:
        raise Failure("WORKER_AUTHORITY", "nonterminal status")
    if require_success and (not os.WIFEXITED(status) or os.WEXITSTATUS(status) != 0 or raw_stderr):
        raise Failure("WORKER_AUTHORITY", "process status")
    return record


class PendingWorkerRun:
    def __init__(self, transaction: ParentProcessTransaction, raw: bytes, stderr: bytes, challenge: bytes, nonce: bytes, invocation: str, identity: str, token: str, work: tempfile.TemporaryDirectory[str], registry: ParentFreshnessRegistry) -> None:
        self.transaction, self.raw, self.stderr = transaction, raw, stderr
        self.challenge, self.nonce, self.invocation = challenge, nonce, invocation
        self.identity, self.token, self.work, self.registry = identity, token, work, registry

    def cleanup(self) -> None:
        self.work.cleanup()


def start_worker(registry: ParentFreshnessRegistry, schedule_slot: str, operation: str = "canonical", challenge: bytes | None = None, nonce: bytes | None = None) -> PendingWorkerRun:
    challenge = challenge if challenge is not None else secrets.token_bytes(32)
    nonce = nonce if nonce is not None else secrets.token_bytes(32)
    while nonce == challenge: nonce = secrets.token_bytes(32)
    invocation = BASE.invocation_digest(challenge, nonce)
    request, observed = BASE.worker_request(challenge, nonce)
    if observed != invocation: raise Failure("WORKER_AUTHORITY", "invocation formula")
    transaction_id = secrets.token_hex(32)
    identity, token = registry.begin(challenge, nonce, invocation, SEQUENCE_SHA256, schedule_slot, transaction_id)
    temporary = tempfile.TemporaryDirectory(prefix="p1347-p1348-worker-", dir="/dev/shm")
    environment = {"P1347_WORK_ROOT": temporary.name}
    worker_operation = {"extra_stdout": "extra_line"}.get(operation, operation)
    if worker_operation != "canonical": environment["P1347_ORACLE_MUTATION"] = worker_operation
    try:
        transaction, raw, stderr = capture_process(registry.session, transaction_id, schedule_slot, [str(PYTHON), "-B", str(BASE.WORKER_PATH), "--transport"], request, environment, 2.0 if operation == "timeout" else 12.0)
        if operation == "partial_stdout": raw = raw[:-7]
        return PendingWorkerRun(transaction, raw, stderr, challenge, nonce, invocation, identity, token, temporary, registry)
    except Exception:
        registry.rollback(identity, token, transaction_id); temporary.cleanup(); raise


def validate_worker(run: PendingWorkerRun | Any) -> dict[str, Any]:
    if type(run) is not PendingWorkerRun:
        raise Failure("WORKER_AUTHORITY", "validate_worker requires internal run handle")
    try:
        record = validate_transaction(run.transaction, run.raw, run.stderr)
        if run.raw.count(b"\n") != 1 or not run.raw.endswith(b"\n"):
            raise Failure("WORKER_AUTHORITY", "worker framing")
        response = strict_json(run.raw, "worker response")
        exact_keys(response, ["case_sequence_sha256", "challenge_response_sha256", "closed_world", "invocation_sha256", "mutation_receipts", "schema"], "worker response")
        if BASE.contains_forbidden(response) or response["schema"] != "p1347-worker-response-r1":
            raise Failure("SCHEMA", "worker schema")
        receipts = response["mutation_receipts"]; ids = BASE.protected_ids()
        if type(receipts) is not list or len(receipts) != 122 or [item.get("case_id") if type(item) is dict else None for item in receipts] != ids or response["case_sequence_sha256"] != SEQUENCE_SHA256:
            raise Failure("PROTECTED_INPUT", "exact 122 sequence")
        if response["invocation_sha256"] != run.invocation:
            raise Failure("WORKER_AUTHORITY", "invocation response")
        digest = sha256(canonical(receipts, False))
        wanted = sha256(canonical(["p1347-worker-challenge-r1", run.challenge.hex(), run.nonce.hex(), run.invocation, SEQUENCE_SHA256, digest], False))
        if response["challenge_response_sha256"] != wanted:
            raise Failure("WORKER_AUTHORITY", "challenge response")
        for ordinal, receipt in enumerate(receipts):
            exact_keys(receipt, ["baseline_sha256", "case_id", "challenge_sha256", "changed_paths", "invocation_sha256", "mutated_tree_sha256", "recipe_sha256"], "mutation receipt")
            relative = f"case-{ordinal:03d}/mutation.bin"; changed = receipt["changed_paths"]
            if type(changed) is not list or len(changed) != 1 or changed[0].get("path") != relative: raise Failure("SCHEMA", "changed paths")
            target = Path(run.work.name) / relative
            if target.is_symlink() or not target.is_file() or sha256(target.read_bytes()) != changed[0].get("sha256"): raise Failure("PROTECTED_INPUT", "mutation bytes")
        ordinal, registry_sha = run.registry.commit(run.identity, run.token, run.transaction)
        return run.transaction.project(run.invocation, ordinal, registry_sha)
    except Failure:
        run.registry.rollback(run.identity, run.token, run.transaction.transaction_id)
        raise


def dag_receipt(raw: bytes, case_id: str, challenge: bytes, invocation: str, root: Path) -> dict[str, Any]:
    value = strict_json(raw, "DAG receipt")
    if BASE.contains_forbidden(value):
        raise Failure("DAG_VALIDATOR", "worker self-answer")
    exact_keys(value, ["baseline_sha256", "case_id", "challenge_sha256", "changed_paths", "invocation_sha256", "mutated_tree_sha256", "recipe_sha256", "schema"], "DAG receipt")
    if value["schema"] != "p1347-dag-mutation-receipt-r1" or value["case_id"] != case_id or value["challenge_sha256"] != sha256(challenge) or value["invocation_sha256"] != invocation or value["mutated_tree_sha256"] != BASE.tree_digest(root):
        raise Failure("SCHEMA", "DAG receipt binding")
    return value


def run_dag(case_id: str, operation: str, observe_validator: bool = True) -> tuple[str, str, dict[str, Any]]:
    session = ParentSession(); challenge = secrets.token_bytes(32); nonce = secrets.token_bytes(32)
    invocation = sha256(canonical(["p1348-dag-invocation-r1", case_id, operation, challenge.hex(), nonce.hex()], False))
    with tempfile.TemporaryDirectory(prefix="p1347-p1348-dag-", dir="/dev/shm") as temporary:
        worker_tx, raw, stderr = capture_process(session, secrets.token_hex(32), "dag-worker", [str(PYTHON), "-B", str(BASE.WORKER_PATH), "--dag"], BASE.dag_request(case_id, operation, challenge, nonce, invocation), {"P1347_WORK_ROOT": temporary})
        try: validate_transaction(worker_tx, raw, stderr)
        except Failure as exc: return "Violated", exc.code, {}
        if raw.count(b"\n") != 1 or not raw.endswith(b"\n"): return "Violated", "WORKER_AUTHORITY", {}
        try: receipt = dag_receipt(raw, case_id, challenge, invocation, Path(temporary))
        except Failure as exc: return "Violated", exc.code, {}
        if not observe_validator: return "Violated", "DAG_VALIDATOR", {}
        validator_tx, validator_out, validator_err = capture_process(session, secrets.token_hex(32), "dag-validator", [str(PYTHON), "-B", str(BASE.CHECKER_PATH), "--validate-dag", temporary, BASE.ORIGINAL_P1346_ANCHOR], b"")
        try: validator_record = validate_transaction(validator_tx, validator_out, validator_err, False)
        except Failure as exc: return "Violated", exc.code, {}
        status = validator_record["raw_wait_status"]
        accepted = os.WIFEXITED(status) and os.WEXITSTATUS(status) == 0 and validator_out == b"" and validator_err == b""
        rejected_authority = os.WIFEXITED(status) and os.WEXITSTATUS(status) == 2 and validator_out == b"" and validator_err.startswith((b"AUTHORITY_ROOT:", b"ANCHOR:"))
        evidence = {"mutation_receipt_sha256": sha256(raw), "validator_stderr_sha256": sha256(validator_err), "worker_transaction_id": worker_tx.transaction_id, "validator_transaction_id": validator_tx.transaction_id}
        if operation == "canonical": return ("Preserved", "PRESERVED", evidence) if accepted else ("Violated", "DAG_VALIDATOR", evidence)
        if rejected_authority: return "Violated", "AUTHORITY_ROOT", evidence
        return ("Violated", "DAG_VALIDATOR", evidence) if not accepted else ("Preserved", "PRESERVED", evidence)


def ptrace(request: int, pid: int, data: int | ctypes.c_void_p = 0) -> None:
    ctypes.set_errno(0)
    pointer = ctypes.c_void_p(data) if type(data) is int else data
    result = LIBC.ptrace(ctypes.c_ulong(request), ctypes.c_ulong(pid), ctypes.c_void_p(0), pointer)
    if result == -1: raise Failure("PROBE_AUTHORITY", f"ptrace errno {ctypes.get_errno()}")


def fd_bytes(fd: int) -> bytes:
    os.lseek(fd, 0, os.SEEK_SET); chunks = []
    while True:
        chunk = os.read(fd, 65536)
        if not chunk: break
        chunks.append(chunk)
    os.lseek(fd, 0, os.SEEK_SET); return b"".join(chunks)


def validate_trace_shape(events: list[str], option_mask: int, exec_count: int, exit_count: int, terminal: bool = True) -> None:
    if option_mask != PTRACE_OPTIONS or events != ["INITIAL_SIGSTOP", "PTRACE_EVENT_EXEC", "PTRACE_EVENT_EXIT", "TERMINAL_WAIT"] or exec_count != 1 or exit_count != 1 or not terminal:
        raise Failure("PROBE_AUTHORITY", "complete trace lifecycle")


def trace_probe(executable: bytes, operation: str = "canonical") -> tuple[str, str, dict[str, Any]]:
    descriptor = os.memfd_create("p1348-probe", os.MFD_ALLOW_SEALING); pidfd = -1
    in_read, in_write = os.pipe2(os.O_CLOEXEC); out_read, out_write = os.pipe2(os.O_CLOEXEC); err_read, err_write = os.pipe2(os.O_CLOEXEC)
    try:
        os.write(descriptor, executable); os.fchmod(descriptor, 0o500)
        seals = fcntl.F_SEAL_WRITE | fcntl.F_SEAL_GROW | fcntl.F_SEAL_SHRINK | fcntl.F_SEAL_SEAL
        fcntl.fcntl(descriptor, fcntl.F_ADD_SEALS, seals)
        before_stat = os.fstat(descriptor); before = fd_bytes(descriptor)
        if fcntl.fcntl(descriptor, fcntl.F_GET_SEALS) != seals or sha256(before) != BASE.PROBE_BINARY_SHA256: raise Failure("PROBE_AUTHORITY", "sealed memfd")
        challenge = secrets.token_bytes(32); nonce = secrets.token_bytes(32); request = BASE.legacy_probe_request(challenge, nonce)
        pid = os.fork()
        if pid == 0:
            try:
                os.close(in_write); os.close(out_read); os.close(err_read)
                os.dup2(in_read, 0); os.dup2(out_write, 1); os.dup2(err_write, 2)
                for fd in (in_read, out_write, err_write):
                    if fd > 2 and fd != descriptor: os.close(fd)
                ptrace(PTRACE_TRACEME, 0); os.kill(os.getpid(), signal.SIGSTOP)
                maximum = min(65536, os.sysconf("SC_OPEN_MAX")); os.closerange(3, descriptor); os.closerange(descriptor + 1, maximum)
                os.execve(descriptor, ["p1348-opaque-probe"], {"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C"})
            except BaseException: os._exit(126)
        os.close(in_read); os.close(out_write); os.close(err_write); pidfd = os.pidfd_open(pid, 0); pidfd_stat = fd_identity(pidfd)
        waited, initial = os.waitpid(pid, 0)
        if waited != pid or not os.WIFSTOPPED(initial) or os.WSTOPSIG(initial) != signal.SIGSTOP: raise Failure("PROBE_AUTHORITY", "initial stop")
        mask = PTRACE_OPTIONS if operation != "missing_traceexit" else PTRACE_O_TRACEEXEC | PTRACE_O_EXITKILL
        ptrace(PTRACE_SETOPTIONS, pid, mask); ptrace(PTRACE_CONT, pid)
        waited, exec_status = os.waitpid(pid, 0); exec_event = (exec_status >> 16) & 0xFFFF
        if waited != pid or not os.WIFSTOPPED(exec_status) or os.WSTOPSIG(exec_status) != signal.SIGTRAP or exec_event != PTRACE_EVENT_EXEC: raise Failure("PROBE_AUTHORITY", "exec event")
        if operation in {"detach", "detach_before_exit"}: raise Failure("PROBE_AUTHORITY", "tracer detach forbidden")
        image_fd = os.open(f"/proc/{pid}/exe", os.O_RDONLY | os.O_CLOEXEC)
        try: image_stat = os.fstat(image_fd); image = fd_bytes(image_fd)
        finally: os.close(image_fd)
        if (image_stat.st_dev, image_stat.st_ino, image_stat.st_size) != (before_stat.st_dev, before_stat.st_ino, before_stat.st_size) or sha256(image) != BASE.PROBE_BINARY_SHA256: raise Failure("PROBE_AUTHORITY", "loaded image")
        os.write(in_write, request); os.close(in_write); in_write = -1; ptrace(PTRACE_CONT, pid)
        waited, exit_status = os.waitpid(pid, 0); exit_event = (exit_status >> 16) & 0xFFFF
        if waited != pid or not os.WIFSTOPPED(exit_status) or os.WSTOPSIG(exit_status) != signal.SIGTRAP or exit_event != PTRACE_EVENT_EXIT: raise Failure("PROBE_AUTHORITY", "exit event")
        event_message = ctypes.c_ulong(0); ptrace(PTRACE_GETEVENTMSG, pid, ctypes.cast(ctypes.byref(event_message), ctypes.c_void_p))
        if operation == "second_exec": raise Failure("PROBE_AUTHORITY", "second exec")
        ptrace(PTRACE_CONT, pid)
        info = os.waitid(os.P_PIDFD, pidfd, os.WEXITED | os.WNOWAIT); waited, terminal = os.waitpid(pid, 0)
        events = ["INITIAL_SIGSTOP", "PTRACE_EVENT_EXEC", "PTRACE_EVENT_EXIT", "TERMINAL_WAIT"]
        validate_trace_shape(events, mask, 1, 1)
        out = bytearray(); err = bytearray()
        for fd, target in ((out_read, out), (err_read, err)):
            while True:
                chunk = os.read(fd, 65536)
                if not chunk: break
                target.extend(chunk)
        after_stat = os.fstat(descriptor); after = fd_bytes(descriptor); after_seals = fcntl.fcntl(descriptor, fcntl.F_GET_SEALS)
        if waited != pid or info.si_pid != pid or info.si_code != os.CLD_EXITED or info.si_status != 0 or not os.WIFEXITED(terminal) or os.WEXITSTATUS(terminal) != 0 or event_message.value != terminal or err or after != before or after_seals != seals or fd_identity(pidfd) != pidfd_stat: raise Failure("PROBE_AUTHORITY", "terminal binding")
        evidence = {"child_pid": pid, "closed_world": {"rule": "Complete parent-observed EXEC-to-EXIT trace."}, "event_sequence": events, "exec_event_count": 1, "exit_event_count": 1, "exit_event_message": event_message.value, "image_fstat": {"st_dev": image_stat.st_dev, "st_ino": image_stat.st_ino, "st_mode": image_stat.st_mode, "st_size": image_stat.st_size}, "image_sha256": sha256(image), "memfd_fstat_after": {"st_dev": after_stat.st_dev, "st_ino": after_stat.st_ino, "st_mode": after_stat.st_mode, "st_size": after_stat.st_size}, "memfd_fstat_before": {"st_dev": before_stat.st_dev, "st_ino": before_stat.st_ino, "st_mode": before_stat.st_mode, "st_size": before_stat.st_size}, "memfd_seals_after": after_seals, "memfd_seals_before": seals, "memfd_sha256_after": sha256(after), "memfd_sha256_before": sha256(before), "pidfd_identity": pidfd_stat, "raw_wait_status": terminal, "schema": "p1348-parent-probe-trace-evidence-r1", "stderr_len": len(err), "stderr_sha256": sha256(bytes(err)), "stdout_len": len(out), "stdout_sha256": sha256(bytes(out)), "waitid_code": info.si_code, "waitid_pid": info.si_pid, "waitid_status": info.si_status, "waitpid_pid": waited}
        actual, reason = BASE.probe_payload(bytes(out), challenge, nonce)
        return actual, reason, evidence
    except (Failure, OSError, ValueError) as exc:
        try:
            if "pid" in locals(): os.kill(pid, signal.SIGKILL)
        except OSError: pass
        try:
            if "pid" in locals(): os.waitpid(pid, 0)
        except OSError: pass
        return "Violated", "PROBE_AUTHORITY", {"failure": type(exc).__name__}
    finally:
        for fd in (descriptor, pidfd, in_read, in_write, out_read, out_write, err_read, err_write):
            if fd >= 0:
                try: os.close(fd)
                except OSError: pass


def nonptrace_focal() -> dict[str, Any]:
    started = time.monotonic(); records = []
    registry = ParentFreshnessRegistry(["canonical"])
    canonical_run = start_worker(registry, "canonical")
    try:
        evidence = validate_worker(canonical_run)
        records.append({"case_id": "P1348-C01-fresh-worker-commit", "observation": "Preserved", "reason_code": "PRESERVED"})
        try: validate_worker(canonical_run)
        except Failure as exc: records.append({"case_id": "P1348-O05-registry-replay", "observation": "Violated", "reason_code": exc.code})
        try: registry.fail_after_commit(canonical_run.identity)
        except IntegrationBlocker:
            try: start_worker(registry, "canonical", challenge=canonical_run.challenge, nonce=canonical_run.nonce)
            except Failure as exc: records.append({"case_id": "P1348-O08-postcommit-reopen", "observation": "Violated", "reason_code": exc.code})
        state = registry.state()
        records.append({"case_id": "P1348-C02-registry-append-only", "observation": "Preserved" if len(state["consumed"]) == 1 and state["pending_count"] == 0 and state["next_slot"] is None else "Violated", "reason_code": "PRESERVED" if len(state["consumed"]) == 1 and state["pending_count"] == 0 and state["next_slot"] is None else "WORKER_AUTHORITY"})
    finally: canonical_run.cleanup()

    substitution_registry = ParentFreshnessRegistry(["substitution"])
    substituted = start_worker(substitution_registry, "substitution")
    try:
        substituted.transaction._record["child_pid"] += 1
        try: validate_worker(substituted)
        except Failure as exc: records.append({"case_id": "P1348-O01-parent-record-field-substitution", "observation": "Violated", "reason_code": exc.code})
    finally: substituted.cleanup()

    partial_challenge, partial_nonce = secrets.token_bytes(32), secrets.token_bytes(32)
    while partial_nonce == partial_challenge: partial_nonce = secrets.token_bytes(32)
    for slot, operation, case_id in (("partial", "partial_stdout", "P1348-O03-partial-stdout"), ("extra", "extra_stdout", "P1348-O04-extra-stdout")):
        negative_registry = ParentFreshnessRegistry([slot])
        run = start_worker(negative_registry, slot, operation, partial_challenge, partial_nonce) if slot == "partial" else start_worker(negative_registry, slot, operation)
        try:
            try: validate_worker(run)
            except Failure as exc: records.append({"case_id": case_id, "observation": "Violated", "reason_code": exc.code})
        finally: run.cleanup()
        if slot == "partial":
            retry = start_worker(negative_registry, slot, "canonical", partial_challenge, partial_nonce)
            try:
                validate_worker(retry)
                records.append({"case_id": "P1348-O07-precommit-rollback", "observation": "Preserved", "reason_code": "PRESERVED"})
            finally: retry.cleanup()

    ordered = ParentFreshnessRegistry(["first", "second"])
    challenge, nonce = secrets.token_bytes(32), secrets.token_bytes(32)
    while challenge == nonce: nonce = secrets.token_bytes(32)
    invocation = BASE.invocation_digest(challenge, nonce)
    try: ordered.begin(challenge, nonce, invocation, SEQUENCE_SHA256, "second", secrets.token_hex(32))
    except Failure as exc: records.append({"case_id": "P1348-O06-registry-order-swap", "observation": "Violated", "reason_code": exc.code})

    try: validate_worker(evidence)
    except Failure as exc: records.append({"case_id": "P1348-O02-public-evidence-reentry", "observation": "Violated", "reason_code": exc.code})
    self_answer = run_dag("P1348-O09-dag-self-answer", "dag_self_labels")
    records.append({"case_id": "P1348-O09-dag-self-answer", "observation": self_answer[0], "reason_code": self_answer[1]})
    anchor = run_dag("P1348-O10-mutant-anchor", "mutant_anchor"); records.append({"case_id": "P1348-O10-mutant-anchor", "observation": anchor[0], "reason_code": anchor[1]})
    dag_control = run_dag("P1348-C03-dag-canonical", "canonical")
    records.append({"case_id": "P1348-C03-dag-canonical", "observation": dag_control[0], "reason_code": dag_control[1]})
    precedence_ok = records[-2]["reason_code"] == "AUTHORITY_ROOT" and self_answer[1] == "DAG_VALIDATOR" and all(next(item for item in records if item["case_id"] == case_id)["reason_code"] == "WORKER_AUTHORITY" for case_id in ("P1348-O03-partial-stdout", "P1348-O04-extra-stdout"))
    records.append({"case_id": "P1348-C04-reason-precedence", "observation": "Preserved" if precedence_ok else "Violated", "reason_code": "PRESERVED" if precedence_ok else "SCHEMA"})
    trace_checks = []
    for case_id, events, mask, execs, exits in [
        ("P1348-O11-traceexit-mask", ["INITIAL_SIGSTOP", "PTRACE_EVENT_EXEC", "PTRACE_EVENT_EXIT", "TERMINAL_WAIT"], PTRACE_O_TRACEEXEC | PTRACE_O_EXITKILL, 1, 1),
        ("P1348-O12-terminal-without-exit", ["INITIAL_SIGSTOP", "PTRACE_EVENT_EXEC", "TERMINAL_WAIT"], PTRACE_OPTIONS, 1, 0),
        ("P1348-O13-second-exec", ["INITIAL_SIGSTOP", "PTRACE_EVENT_EXEC", "PTRACE_EVENT_EXEC", "PTRACE_EVENT_EXIT", "TERMINAL_WAIT"], PTRACE_OPTIONS, 2, 1),
        ("P1348-O14-tracer-detach", ["INITIAL_SIGSTOP", "PTRACE_EVENT_EXEC", "DETACH", "PTRACE_EVENT_EXIT", "TERMINAL_WAIT"], PTRACE_OPTIONS, 1, 1),
    ]:
        try: validate_trace_shape(events, mask, execs, exits)
        except Failure as exc: trace_checks.append({"case_id": case_id, "observation": "Violated", "reason_code": exc.code})
    records.extend(trace_checks)
    try:
        validate_trace_shape(["INITIAL_SIGSTOP", "PTRACE_EVENT_EXEC", "PTRACE_EVENT_EXIT", "TERMINAL_WAIT"], PTRACE_OPTIONS, 1, 1)
        records.append({"case_id": "P1348-C05-complete-trace-shape", "observation": "Preserved", "reason_code": "PRESERVED"})
    except Failure as exc: records.append({"case_id": "P1348-C05-complete-trace-shape", "observation": "Violated", "reason_code": exc.code})
    records.append({"case_id": "P1348-C06-canonical-probe", "observation": "Unknown", "reason_code": "EXTERNAL_PTRACE_REQUIRED"})
    regressions = [row["case_id"] for row in records if row["case_id"].startswith("P1348-C") and row["observation"] != "Preserved"]
    non_external_regressions = [case_id for case_id in regressions if case_id != "P1348-C06-canonical-probe"]
    return {"closed_world": {"rule": "Non-ptrace oracle self-focal only; external verifier owns decisive ptrace and verdict."}, "cost": {"elapsed_ms": int((time.monotonic() - started) * 1000), "full_corpus_runs": 0, "process_runs": 10}, "execution": {"candidate_read_or_executed": False, "full_corpus_runs": 0, "ptrace_runs": 0}, "records": records, "regime": "executado sem atestacao de isolamento", "revision": 1, "schema": "p1348-oracle-nonptrace-focal-r1", "step": 1348, "summary": {"external_ptrace_required": True, "positive_control_regressions": non_external_regressions}, "verdict": "ORACLE_NONPTRACE_FOCAL_NOT_SEALED" if not non_external_regressions else "ORACLE_NONPTRACE_BLOCKED_NOT_SEALED"}


def manual_preflight(argv: list[str]) -> dict[str, str]:
    keys = ["--authorship-receipt", "--authorship-receipt-sha256", "--expected-authoring-root-sha256", "--expected-checker-sha256"]
    if len(argv) != 9 or argv[0] != "--focus": raise Failure("AUTHORITY_ROOT", "closed CLI")
    result = {}
    for index in range(1, 9, 2):
        key, value = argv[index], argv[index + 1]
        if key not in keys or key in result: raise Failure("AUTHORITY_ROOT", "closed CLI")
        result[key] = value
    if result.get("--authorship-receipt") != RECEIPT_REL: raise Failure("AUTHORITY_ROOT", "receipt path")
    for key in keys[1:]: exact_hex(result[key], key)
    return result


def run_focal(checker_sha256: str, receipt_sha256: str, root_sha256: str) -> dict[str, Any]:
    validate_protected_inputs(); load_corpus(); validate_authorship_receipt(receipt_sha256, checker_sha256, root_sha256)
    return nonptrace_focal()


def main() -> int:
    try:
        projection = manual_preflight(sys.argv[1:]); report = run_focal(projection["--expected-checker-sha256"], projection["--authorship-receipt-sha256"], projection["--expected-authoring-root-sha256"])
        sys.stdout.buffer.write(canonical(report)); return 0 if report["verdict"] == "ORACLE_NONPTRACE_FOCAL_NOT_SEALED" else 1
    except Failure as exc: sys.stderr.write(f"{exc.code}: {exc.detail}\n"); return 2
    except Exception as exc: sys.stderr.write(f"AUTHORITY_ROOT: closed checker failure {type(exc).__name__}\n"); return 2


if __name__ == "__main__": raise SystemExit(main())
