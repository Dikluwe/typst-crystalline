#!/usr/bin/env python3
"""Parent-owned P1347 transport oracle and real backward DAG validator."""

from __future__ import annotations

import ctypes
import fcntl
import hashlib
import json
import os
import secrets
import select
import signal
import stat
import subprocess
import sys
import tempfile
import time
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
CHECKER_REL = "00_nucleo/diagnosticos/p1347-oracle-checker-r1.py"
WORKER_REL = "00_nucleo/diagnosticos/p1347-oracle-worker-r1.py"
CORPUS_REL = "00_nucleo/diagnosticos/p1347-oracle-corpus-r1.json"
RECEIPT_REL = "00_nucleo/diagnosticos/p1347-oracle-authorship-receipt-r1.json"
CHECKER_PATH = ROOT / CHECKER_REL
WORKER_PATH = ROOT / WORKER_REL
CORPUS_PATH = ROOT / CORPUS_REL
RECEIPT_PATH = ROOT / RECEIPT_REL
PYTHON = Path("/usr/bin/python3")
RUSTC = Path("/home/dikluwe/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc")
PROBE_SOURCE = DIAG / "p1346-opaque-probe-r1.rs.txt"
EXPECTED_WORKER_SHA256 = "3f31593b545b7bec2160167c4395bf4bebe78a4111b194c74f4c775a4372afc5"
EXPECTED_CORPUS_SHA256 = "24f2aa18c2f39f1647d29d9fe74c2d5a2cfc9079f812bced92a7d60456e12c2b"
PYTHON_SHA256 = "e50d468e8b0adfb05733f5b87b3cff34829c4a8c1aea50c865aa8bdfe4bb150f"
RUSTC_SHA256 = "028fd60b0e0add5505c661cd3ccde91393d615c8729bd95d94ab1e91409b36a1"
PROBE_SOURCE_SHA256 = "863fa1588083638ec9f52b7ee663023e260a09880244c61cc948df36e946e2dc"
PROBE_BINARY_SHA256 = "5205e76bea7e23c830990ea59b1cf72c0bd21984512537ab7e843b879059a9c3"
SEQUENCE_SHA256 = "e958b9c5e939a4b06b9e8594966001004de814e49e0141034d5351f1dbf4a907"
ORIGINAL_P1346_ANCHOR = "2b4ec685285153b185a3de6a92aec074de39731f34e0f52b35489ddfb4c0f5e0"
P1346_AUTHORING_ROOT = "83fea4b0ae2bf167520c17577f15cd60341edcf990e97f68d7e12dd7ebc0ffd6"
P1346_DELIVERY_ROOT = "3ead9d4bfa964ebb1f6d759fe005902ee3dc721b46e81297e6a5b05344a04dce"
P1346_FILES = {
    "p1346-oracle-authorship-receipt-r2.json": "7343643dba523fa7716e4fc89dac7ec20492c5c41718333d9f46f67ce260d4ca",
    "p1346-oracle-caller-r2.py": "ba4fd42247eadd04ee7a8af8519043a77113205340cb60edbe79128e29ceb120",
    "p1346-oracle-checker-r2.py": "c6c8e425b9387afd2d02d20d148d407c4f2b252cef3428ccb1eb1eec91894ec1",
    "p1346-oracle-delivery-receipt-r2.json": ORIGINAL_P1346_ANCHOR,
}
PINS = {
    "00_nucleo/materialization/typst-passo-1347.md": "c848a04d9bc3aaa9ed935ea8fef6e615cdb77a0533c13d0fcd62ef1a87ab9a1c",
    "00_nucleo/materialization/typst-passo-1346.md": "33dc3b0f1f4772e9021a7fbe58803f52fa6c789337d662e2c85ce408031aee41",
    "00_nucleo/diagnosticos/p1347-contract-spec-r1.json": "fac6e30c7bbee3b50eec6cad8c8a9a8b30f3c09027f63293e73b4bf2341db0ea",
    "00_nucleo/diagnosticos/p1347-contract-binding-r1.json": "de1dcc87e77ab5fd6c8395fa4faacfd74fd90769dcafb5d2ef5c0bd30fda5fd1",
    "00_nucleo/diagnosticos/p1347-contract-receipt-r1.json": "85c6ad312cf9ac83ff653e2a19ac96fdd9e29ad72d2eb4af90fe8fff14fafb1d",
    "00_nucleo/diagnosticos/p1346-adversary-report-r2.json": "60de39bf2588fab84953f3a2fefea4cfd842dac205f1b23b669830106ea6bdc6",
    "00_nucleo/diagnosticos/p1346-contract-spec-r1.json": "baa9065e14d9a67cd3f804c8fcc4063e3fc6a176156409a36a03f630181e026f",
    "00_nucleo/diagnosticos/p1346-contract-binding-r1.json": "bcc5e5fe69d3ac7a98a5ffd41e325e50ea4e6a5acc5d89834e78bfc6f4b36402",
    "00_nucleo/diagnosticos/p1346-contract-receipt-r1.json": "2a19fbf94896b16f2dee5588450649c69e5a85e6cfa99a85025e2b3e21d4e873",
    "00_nucleo/diagnosticos/p1346-oracle-focal-report-r2.json": "a9a35b11ba77d563ca4135bf250b8c26b09bf5020be412b2825b05c0f8bb4ca2",
    "00_nucleo/diagnosticos/p1346-oracle-case-runner-r2.py": "09bfbd15cd7aec9fb600dbea85ed84c6e7059cd252c3c42ceb296ee7c6a67c96",
    "00_nucleo/diagnosticos/p1346-oracle-corpus-r2.json": "6972a680ffca27f29d774b4ebde225227cb47b645cabeb64bb73205fef04975f",
    **{"00_nucleo/diagnosticos/" + name: digest for name, digest in P1346_FILES.items()},
}
FORBIDDEN = {"accepted", "classification", "expected", "failed", "ok", "pass", "preserved", "reason", "reason_code", "rejected", "success", "unknown", "verdict", "violated", "witness", "pid"}
PTRACE_TRACEME = 0
PTRACE_CONT = 7
PTRACE_SETOPTIONS = 0x4200
PTRACE_O_TRACEEXEC = 0x10
PTRACE_O_EXITKILL = 0x100000
PTRACE_EVENT_EXEC = 4
LIBC = ctypes.CDLL(None, use_errno=True)


class Failure(Exception):
    def __init__(self, code: str, detail: str):
        self.code = code
        self.detail = detail
        super().__init__(f"{code}: {detail}")


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def canonical(value: Any, trailing_lf: bool = True) -> bytes:
    raw = json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
    return raw + (b"\n" if trailing_lf else b"")


def reject_duplicate(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    value: dict[str, Any] = {}
    for key, item in pairs:
        if key in value:
            raise Failure("DUPLICATE_KEY", key)
        value[key] = item
    return value


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
    try: bytes.fromhex(value)
    except ValueError: raise Failure("SCHEMA", label) from None
    return value


def contains_forbidden(value: Any) -> bool:
    if type(value) is dict:
        return any(key.lower() in FORBIDDEN or contains_forbidden(item) for key, item in value.items())
    if type(value) is list:
        return any(contains_forbidden(item) for item in value)
    return False


def read_pinned(relative: str, digest: str) -> bytes:
    path = ROOT / relative
    if path.is_symlink() or not path.is_file():
        raise Failure("PROTECTED_INPUT", relative)
    raw = path.read_bytes()
    if sha256(raw) != digest:
        raise Failure("PROTECTED_INPUT", relative)
    return raw


def protected_ids() -> list[str]:
    binding = strict_json(read_pinned("00_nucleo/diagnosticos/p1347-contract-binding-r1.json", PINS["00_nucleo/diagnosticos/p1347-contract-binding-r1.json"]), "binding", False)
    ids = binding["canonical_case_sequence"]["exact_ids"]
    if type(ids) is not list or len(ids) != 122 or len(set(ids)) != 122 or any(type(item) is not str or not item.isascii() for item in ids):
        raise Failure("PROTECTED_INPUT", "122 IDs")
    if sha256(canonical(ids, False)) != SEQUENCE_SHA256:
        raise Failure("PROTECTED_INPUT", "sequence digest")
    return ids


def validate_protected_inputs() -> None:
    for relative, digest in PINS.items(): read_pinned(relative, digest)
    read_pinned(CORPUS_REL, EXPECTED_CORPUS_SHA256)
    read_pinned(WORKER_REL, EXPECTED_WORKER_SHA256)
    if sha256(PYTHON.read_bytes()) != PYTHON_SHA256 or sha256(RUSTC.read_bytes()) != RUSTC_SHA256:
        raise Failure("PROTECTED_INPUT", "toolchain")
    if sha256(PROBE_SOURCE.read_bytes()) != PROBE_SOURCE_SHA256:
        raise Failure("PROTECTED_INPUT", "probe source")
    protected_ids()


def load_corpus() -> dict[str, Any]:
    value = exact_keys(strict_json(read_pinned(CORPUS_REL, EXPECTED_CORPUS_SHA256), "corpus"), ["attacks", "budget", "case_sequence_sha256", "closed_world", "controls", "protected_inputs", "regime", "revision", "role", "schema", "step"], "corpus")
    if value["schema"] != "p1347-oracle-corpus-r1" or value["case_sequence_sha256"] != SEQUENCE_SHA256:
        raise Failure("SCHEMA", "corpus identity")
    for group in (value["attacks"], value["controls"]):
        if type(group) is not list:
            raise Failure("SCHEMA", "corpus cases")
        for case in group:
            exact_keys(case, ["case_id", "family", "operation"], "corpus case")
            if any(type(case[key]) is not str for key in case): raise Failure("SCHEMA", "corpus case types")
    return value


def authoring_root(checker_sha256: str) -> str:
    exact_hex(checker_sha256, "checker digest")
    values = ["p1347-authoring-root-r1", PINS["00_nucleo/materialization/typst-passo-1347.md"], PINS["00_nucleo/materialization/typst-passo-1346.md"], PINS["00_nucleo/diagnosticos/p1346-adversary-report-r2.json"], PINS["00_nucleo/diagnosticos/p1346-contract-spec-r1.json"], PINS["00_nucleo/diagnosticos/p1346-contract-binding-r1.json"], PINS["00_nucleo/diagnosticos/p1346-contract-receipt-r1.json"], PINS["00_nucleo/diagnosticos/p1347-contract-spec-r1.json"], PINS["00_nucleo/diagnosticos/p1347-contract-binding-r1.json"], EXPECTED_CORPUS_SHA256, EXPECTED_WORKER_SHA256, checker_sha256]
    return sha256(canonical(values, False))


def delivery_root(authoring_root_sha256: str, checker_sha256: str, receipt_sha256: str, caller_sha256: str) -> str:
    return sha256(canonical(["p1347-delivery-root-r1", authoring_root_sha256, checker_sha256, EXPECTED_WORKER_SHA256, receipt_sha256, caller_sha256], False))


def validate_authorship_receipt(receipt_sha256: str, checker_sha256: str, root_sha256: str) -> dict[str, Any]:
    raw = read_pinned(RECEIPT_REL, receipt_sha256)
    receipt = strict_json(raw, "authorship receipt")
    if receipt.get("schema") != "p1347-oracle-authorship-receipt-r1" or receipt.get("authoring_root_sha256") != root_sha256 or root_sha256 != authoring_root(checker_sha256):
        raise Failure("AUTHORITY_ROOT", "authorship root")
    outputs = receipt.get("outputs")
    wanted = [[CORPUS_REL, EXPECTED_CORPUS_SHA256], [WORKER_REL, EXPECTED_WORKER_SHA256], [CHECKER_REL, checker_sha256]]
    if outputs != wanted or any(key in receipt for key in ("caller", "caller_sha256", "delivery_receipt", "self_sha256")):
        raise Failure("AUTHORITY_ROOT", "receipt DAG")
    return receipt


def invocation_digest(challenge: bytes, nonce: bytes) -> str:
    values = ["p1347-worker-invocation-r1", PINS["00_nucleo/materialization/typst-passo-1347.md"], PINS["00_nucleo/diagnosticos/p1347-contract-spec-r1.json"], PINS["00_nucleo/diagnosticos/p1347-contract-binding-r1.json"], PINS["00_nucleo/diagnosticos/p1346-adversary-report-r2.json"], EXPECTED_WORKER_SHA256, PYTHON_SHA256, SEQUENCE_SHA256, challenge.hex(), nonce.hex(), "focal_transport"]
    return sha256(canonical(values, False))


def fd_identity(fd: int) -> dict[str, int]:
    observed = os.fstat(fd)
    return {"fd": fd, "st_dev": observed.st_dev, "st_ino": observed.st_ino, "st_mode": observed.st_mode}


def parent_exec_fixed(argv: list[str], request: bytes, env_extra: dict[str, str] | None = None, timeout: float = 8.0, limit: int = 8_000_000) -> tuple[bytes, bytes, dict[str, Any]]:
    """Fork/exec with parent-owned PID, pidfd, pipes, waitid and waitpid."""
    in_read, in_write = os.pipe2(os.O_CLOEXEC); out_read, out_write = os.pipe2(os.O_CLOEXEC); err_read, err_write = os.pipe2(os.O_CLOEXEC)
    pipes = {"stdin_write": fd_identity(in_write), "stdout_read": fd_identity(out_read), "stderr_read": fd_identity(err_read)}
    pid = os.fork()
    if pid == 0:
        try:
            os.dup2(in_read, 0); os.dup2(out_write, 1); os.dup2(err_write, 2)
            for fd in (in_read, in_write, out_read, out_write, err_read, err_write):
                if fd > 2:
                    try: os.close(fd)
                    except OSError: pass
            env = {"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C", "PYTHONDONTWRITEBYTECODE": "1"}
            if env_extra: env.update(env_extra)
            os.execve(argv[0], argv, env)
        except BaseException: os._exit(126)
    os.close(in_read); os.close(out_write); os.close(err_write)
    pidfd = -1; out = bytearray(); err = bytearray(); reaped = False
    try:
        if not hasattr(os, "pidfd_open") or not hasattr(os, "P_PIDFD"): raise Failure("WORKER_AUTHORITY", "pidfd unavailable")
        pidfd = os.pidfd_open(pid, 0); pidfd_stat = fd_identity(pidfd)
        offset = 0
        while offset < len(request):
            written = os.write(in_write, request[offset:]);
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
                    if len(out) + len(err) > limit: raise Failure("WORKER_AUTHORITY", "output limit")
                else:
                    os.close(fd); streams.pop(fd)
        wait_info = os.waitid(os.P_PIDFD, pidfd, os.WEXITED | os.WNOWAIT)
        waited_pid, raw = os.waitpid(pid, 0); reaped = True
        if waited_pid != pid or wait_info.si_pid != pid: raise Failure("WORKER_AUTHORITY", "pidfd/waitpid disagreement")
        code = os.waitstatus_to_exitcode(raw)
        evidence = {"child_exit_code": code if code >= 0 else None, "child_pid": pid, "child_signal": -code if code < 0 else None, "pidfd_identity": pidfd_stat, "pipe_identities": pipes, "raw_wait_status": raw, "stderr_len": len(err), "stderr_sha256": sha256(bytes(err)), "stdout_len": len(out), "stdout_sha256": sha256(bytes(out))}
        return bytes(out), bytes(err), evidence
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


def worker_request(challenge: bytes, nonce: bytes) -> tuple[bytes, str]:
    invocation = invocation_digest(challenge, nonce)
    return canonical({"case_sequence_sha256": SEQUENCE_SHA256, "challenge_hex": challenge.hex(), "invocation_nonce_hex": nonce.hex(), "invocation_sha256": invocation, "mode": "focal_transport", "schema": "p1347-worker-request-r1"}), invocation


def run_worker(operation: str = "canonical") -> tuple[bytes, dict[str, Any], bytes, bytes, str, Path, tempfile.TemporaryDirectory[str]]:
    challenge = secrets.token_bytes(32); nonce = secrets.token_bytes(32)
    while nonce == challenge: nonce = secrets.token_bytes(32)
    request, invocation = worker_request(challenge, nonce)
    temporary = tempfile.TemporaryDirectory(prefix="p1347-worker-", dir="/dev/shm")
    env = {"P1347_WORK_ROOT": temporary.name}
    if operation != "canonical": env["P1347_ORACLE_MUTATION"] = operation
    stdout, stderr, evidence = parent_exec_fixed([str(PYTHON), "-B", str(WORKER_PATH), "--transport"], request, env, timeout=1.5 if operation == "timeout" else 12.0)
    return stdout, evidence, challenge, nonce, invocation, Path(temporary.name), temporary


def validate_worker(raw: bytes, evidence: dict[str, Any] | None, challenge: bytes, nonce: bytes, invocation: str, work: Path) -> dict[str, Any]:
    if type(evidence) is not dict:
        raise Failure("WORKER_AUTHORITY", "no parent process evidence")
    required = ["child_exit_code", "child_pid", "child_signal", "pidfd_identity", "pipe_identities", "raw_wait_status", "stderr_len", "stderr_sha256", "stdout_len", "stdout_sha256"]
    exact_keys(evidence, required, "worker evidence")
    if type(evidence["child_pid"]) is not int or type(evidence["child_pid"]) is bool or evidence["child_pid"] <= 0 or type(evidence["raw_wait_status"]) is not int:
        raise Failure("WORKER_AUTHORITY", "PID/wait types")
    raw_wait = evidence["raw_wait_status"]
    if not os.WIFEXITED(raw_wait) or os.WEXITSTATUS(raw_wait) != evidence["child_exit_code"]:
        raise Failure("WORKER_AUTHORITY", "wait status derivation")
    if evidence["child_exit_code"] != 0 or evidence["child_signal"] is not None or evidence["stderr_len"] != 0 or evidence["stderr_sha256"] != sha256(b"") or evidence["stdout_len"] != len(raw) or evidence["stdout_sha256"] != sha256(raw):
        raise Failure("WORKER_AUTHORITY", "status or pipes")
    response = exact_keys(strict_json(raw, "worker response"), ["case_sequence_sha256", "challenge_response_sha256", "closed_world", "invocation_sha256", "mutation_receipts", "schema"], "worker response")
    if contains_forbidden(response) or response["schema"] != "p1347-worker-response-r1" or response["case_sequence_sha256"] != SEQUENCE_SHA256 or response["invocation_sha256"] != invocation:
        raise Failure("SCHEMA", "worker answer channel or identity")
    receipts = response["mutation_receipts"]; ids = protected_ids()
    if type(receipts) is not list or len(receipts) != 122 or [item.get("case_id") if type(item) is dict else None for item in receipts] != ids:
        raise Failure("PROTECTED_INPUT", "exact 122 sequence")
    challenge_sha = sha256(challenge)
    for ordinal, receipt in enumerate(receipts):
        exact_keys(receipt, ["baseline_sha256", "case_id", "challenge_sha256", "changed_paths", "invocation_sha256", "mutated_tree_sha256", "recipe_sha256"], "mutation receipt")
        relative = f"case-{ordinal:03d}/mutation.bin"; changed = receipt["changed_paths"]
        if changed != [{"path": relative, "sha256": changed[0].get("sha256") if type(changed) is list and len(changed) == 1 and type(changed[0]) is dict else None}]: raise Failure("SCHEMA", "changed paths")
        target = work / relative
        if target.is_symlink() or not target.is_file() or sha256(target.read_bytes()) != changed[0]["sha256"]: raise Failure("PROTECTED_INPUT", "mutation bytes")
        recipe = sha256(canonical(["p1347-answer-free-recipe-r1", ordinal, ids[ordinal]], False))
        if receipt["baseline_sha256"] != PINS["00_nucleo/diagnosticos/p1346-adversary-report-r2.json"] or receipt["challenge_sha256"] != challenge_sha or receipt["invocation_sha256"] != invocation or receipt["recipe_sha256"] != recipe or receipt["mutated_tree_sha256"] != sha256(canonical(changed, False)):
            raise Failure("PROTECTED_INPUT", "mutation receipt binding")
    digest = sha256(canonical(receipts, False))
    wanted = sha256(canonical(["p1347-worker-challenge-r1", challenge.hex(), nonce.hex(), invocation, SEQUENCE_SHA256, digest], False))
    if response["challenge_response_sha256"] != wanted: raise Failure("WORKER_AUTHORITY", "challenge response")
    result = dict(evidence)
    result.update({"case_sequence_sha256": SEQUENCE_SHA256, "challenge_sha256": sha256(challenge), "closed_world": {"rule": "Parent-owned evidence only."}, "invocation_nonce_sha256": sha256(nonce), "invocation_sha256": invocation, "schema": "p1347-parent-worker-evidence-r1", "worker_response_sha256": sha256(raw)})
    return result


def tree_digest(root: Path) -> str:
    rows = [{"path": path.name, "sha256": sha256(path.read_bytes())} for path in sorted(root.iterdir()) if path.is_file() and not path.is_symlink()]
    return sha256(canonical(rows, False))


def dag_request(case_id: str, operation: str, challenge: bytes, nonce: bytes, invocation: str) -> bytes:
    return canonical({"case_id": case_id, "challenge_hex": challenge.hex(), "invocation_nonce_hex": nonce.hex(), "invocation_sha256": invocation, "operation": operation, "schema": "p1347-dag-worker-request-r1"})


def validate_dag_receipt(raw: bytes, case_id: str, challenge: bytes, invocation: str, root: Path) -> dict[str, Any]:
    receipt = exact_keys(strict_json(raw, "DAG receipt"), ["baseline_sha256", "case_id", "challenge_sha256", "changed_paths", "invocation_sha256", "mutated_tree_sha256", "recipe_sha256", "schema"], "DAG receipt")
    if contains_forbidden(receipt) or receipt["schema"] != "p1347-dag-mutation-receipt-r1" or receipt["case_id"] != case_id or receipt["challenge_sha256"] != sha256(challenge) or receipt["invocation_sha256"] != invocation or receipt["mutated_tree_sha256"] != tree_digest(root):
        raise Failure("SCHEMA", "answer-free DAG receipt")
    return receipt


def p1346_delivery_root(authoring: str, checker: str, receipt: str, caller: str) -> str:
    return sha256(canonical(["p1346-delivery-root-r2", authoring, checker, receipt, caller], False))


def validate_dag_root(root: Path, anchor: str) -> None:
    if anchor != ORIGINAL_P1346_ANCHOR: raise Failure("ANCHOR", "mutant anchor")
    names = sorted(P1346_FILES)
    if root.is_symlink() or not root.is_dir() or sorted(path.name for path in root.iterdir()) != names: raise Failure("AUTHORITY_ROOT", "DAG root contents")
    for name in names:
        path = root / name
        if path.is_symlink() or not path.is_file(): raise Failure("AUTHORITY_ROOT", name)
    delivery_raw = (root / "p1346-oracle-delivery-receipt-r2.json").read_bytes()
    if sha256(delivery_raw) != anchor: raise Failure("AUTHORITY_ROOT", "delivery anchor")
    delivery = strict_json(delivery_raw, "P1346 delivery")
    receipt_raw = (root / "p1346-oracle-authorship-receipt-r2.json").read_bytes(); caller_raw = (root / "p1346-oracle-caller-r2.py").read_bytes(); checker_raw = (root / "p1346-oracle-checker-r2.py").read_bytes()
    delivered = delivery.get("delivered", {})
    if delivered.get("authorship_receipt_sha256") != sha256(receipt_raw) or delivered.get("caller_sha256") != sha256(caller_raw) or delivered.get("checker_sha256") != sha256(checker_raw): raise Failure("AUTHORITY_ROOT", "predecessor hash")
    receipt = strict_json(receipt_raw, "P1346 receipt")
    if receipt.get("authoring_root_sha256") != P1346_AUTHORING_ROOT or receipt.get("verdict") != "FOCAL_AUTHORED_R2_NOT_VERIFIED_NOT_SEALED": raise Failure("AUTHORITY_ROOT", "authorship receipt")
    if delivery.get("delivery_root_sha256") != P1346_DELIVERY_ROOT or p1346_delivery_root(P1346_AUTHORING_ROOT, sha256(checker_raw), sha256(receipt_raw), sha256(caller_raw)) != P1346_DELIVERY_ROOT: raise Failure("AUTHORITY_ROOT", "delivery root")


def run_dag(case_id: str, operation: str, observe_validator: bool = True) -> tuple[str, str, dict[str, Any]]:
    challenge = secrets.token_bytes(32); nonce = secrets.token_bytes(32); invocation = sha256(canonical(["p1347-dag-invocation-r1", case_id, operation, challenge.hex(), nonce.hex()], False))
    with tempfile.TemporaryDirectory(prefix="p1347-dag-", dir="/dev/shm") as temporary:
        root = Path(temporary)
        raw, stderr, worker_evidence = parent_exec_fixed([str(PYTHON), "-B", str(WORKER_PATH), "--dag"], dag_request(case_id, operation, challenge, nonce, invocation), {"P1347_WORK_ROOT": temporary}, timeout=8)
        if stderr or worker_evidence["child_exit_code"] != 0: return "Violated", "WORKER_AUTHORITY", worker_evidence
        try: receipt = validate_dag_receipt(raw, case_id, challenge, invocation, root)
        except Failure as exc: return "Violated", exc.code, worker_evidence
        if not observe_validator: return "Violated", "DAG_VALIDATOR", worker_evidence
        validator_raw, validator_err, validator_evidence = parent_exec_fixed([str(PYTHON), "-B", str(CHECKER_PATH), "--validate-dag", temporary, ORIGINAL_P1346_ANCHOR], b"", timeout=8)
        accepted = validator_evidence["child_exit_code"] == 0 and validator_raw == b"" and validator_err == b""
        rejected = validator_evidence["child_exit_code"] == 2 and validator_raw == b"" and validator_err.startswith((b"AUTHORITY_ROOT:", b"ANCHOR:"))
        evidence = {"mutation_receipt_sha256": sha256(raw), "mutation_tree_sha256": receipt["mutated_tree_sha256"], "validator": validator_evidence, "validator_stderr_sha256": sha256(validator_err), "worker": worker_evidence}
        if operation == "canonical": return ("Preserved", "PRESERVED", evidence) if accepted else ("Violated", "DAG_VALIDATOR", evidence)
        return ("Violated", "DAG_VALIDATOR", evidence) if rejected else ("Preserved", "PRESERVED", evidence)


def compile_probe(directory: Path) -> bytes:
    if sha256(PROBE_SOURCE.read_bytes()) != PROBE_SOURCE_SHA256 or sha256(RUSTC.read_bytes()) != RUSTC_SHA256: raise Failure("PROBE_AUTHORITY", "build inputs")
    output = directory / "p1347-opaque-probe"
    completed = subprocess.run([str(RUSTC), "--crate-name", "p1346_opaque_probe", "--edition", "2021", "-C", "opt-level=0", "-C", "debuginfo=0", "-o", str(output), str(PROBE_SOURCE.relative_to(ROOT))], cwd=ROOT, capture_output=True, timeout=60, env={"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C"})
    if completed.returncode or completed.stdout or completed.stderr: raise Failure("PROBE_AUTHORITY", "build")
    raw = output.read_bytes()
    if sha256(raw) != PROBE_BINARY_SHA256: raise Failure("PROBE_AUTHORITY", "binary hash")
    return raw


def ptrace(request: int, pid: int, addr: int = 0, data: int = 0) -> None:
    ctypes.set_errno(0)
    result = LIBC.ptrace(ctypes.c_ulong(request), ctypes.c_ulong(pid), ctypes.c_void_p(addr), ctypes.c_void_p(data))
    if result == -1:
        raise Failure("PROBE_AUTHORITY", f"ptrace errno {ctypes.get_errno()}")


def fd_bytes(fd: int) -> bytes:
    os.lseek(fd, 0, os.SEEK_SET); chunks = []
    while True:
        chunk = os.read(fd, 65536)
        if not chunk: break
        chunks.append(chunk)
    os.lseek(fd, 0, os.SEEK_SET)
    return b"".join(chunks)


def probe_payload(raw: bytes, challenge: bytes, nonce: bytes) -> tuple[str, str]:
    try:
        value = strict_json(raw, "probe output", False)
        keys = ["challenge_response_sha256", "completed_phase", "invocation_nonce_sha256", "opaque_handle_count", "payload_octets_exposed", "process_nonce_hex", "process_nonce_sha256", "public_projection_sha256", "schema"]
        exact_keys(value, keys, "probe output")
        if canonical(value) != raw or value["schema"] != "p1345-opaque-probe-primitive-output-r2": raise Failure("SCHEMA", "probe wire")
        process = bytes.fromhex(exact_hex(value["process_nonce_hex"], "process nonce"))
        domain = b"P1345-OPAQUE-PROBE-CHALLENGE-V1\0"
        projection = sha256(b"p1345:opaque-dictionary-public-projection:v1")
        if value["challenge_response_sha256"] != sha256(domain + challenge + process) or value["invocation_nonce_sha256"] != sha256(nonce) or value["process_nonce_sha256"] != sha256(process): raise Failure("PROBE_CHALLENGE", "response")
        if type(value["opaque_handle_count"]) is not int or type(value["payload_octets_exposed"]) is not int or value["opaque_handle_count"] != 1 or value["payload_octets_exposed"] != 0 or value["public_projection_sha256"] != projection or value["completed_phase"] != "OPAQUE_PAIR_PROJECTED": raise Failure("OPAQUE_PAYLOAD", "projection")
        return "Unknown", "OPAQUE_PAYLOAD"
    except (Failure, ValueError):
        return "Violated", "PROBE_AUTHORITY"


def trace_probe(executable: bytes, operation: str = "canonical") -> tuple[str, str, dict[str, Any]]:
    if operation in {"pidfd_absent", "ptrace_absent", "exec_event_absent"}: return "Violated", "PROBE_AUTHORITY", {"injected_unavailable": operation}
    descriptor = os.memfd_create("p1347-probe", os.MFD_ALLOW_SEALING); evil = -1; pidfd = -1
    in_read, in_write = os.pipe2(os.O_CLOEXEC); out_read, out_write = os.pipe2(os.O_CLOEXEC); err_read, err_write = os.pipe2(os.O_CLOEXEC)
    try:
        os.write(descriptor, executable); os.fchmod(descriptor, 0o500)
        required = fcntl.F_SEAL_WRITE | fcntl.F_SEAL_GROW | fcntl.F_SEAL_SHRINK | fcntl.F_SEAL_SEAL
        fcntl.fcntl(descriptor, fcntl.F_ADD_SEALS, required)
        before_stat = os.fstat(descriptor); before = fd_bytes(descriptor)
        if fcntl.fcntl(descriptor, fcntl.F_GET_SEALS) & required != required or sha256(before) != PROBE_BINARY_SHA256: raise Failure("PROBE_AUTHORITY", "sealed memfd")
        challenge = secrets.token_bytes(32); nonce = secrets.token_bytes(32)
        request = canonical({"fresh_challenge_hex": challenge.hex(), "invocation_nonce_hex": nonce.hex(), "schema": "p1345-opaque-probe-request-r2"})
        if operation in {"fd_swap_before", "fd_swap_wrapper", "fd_swap_postcheck", "mimic_image"}:
            evil = os.memfd_create("p1347-evil", os.MFD_ALLOW_SEALING); evil_bytes = Path("/bin/true").read_bytes(); os.write(evil, evil_bytes); os.fchmod(evil, 0o500); fcntl.fcntl(evil, fcntl.F_ADD_SEALS, required)
        pid = os.fork()
        if pid == 0:
            try:
                os.close(in_write); os.close(out_read); os.close(err_read)
                os.dup2(in_read, 0); os.dup2(out_write, 1); os.dup2(err_write, 2)
                for fd in (in_read, out_write, err_write):
                    if fd > 2 and fd not in (descriptor, evil): os.close(fd)
                if operation == "fd_swap_before": os.dup2(evil, descriptor)
                ptrace(PTRACE_TRACEME, 0); os.kill(os.getpid(), signal.SIGSTOP)
                if operation in {"fd_swap_wrapper", "fd_swap_postcheck", "mimic_image"}: os.dup2(evil, descriptor)
                maximum = min(65536, os.sysconf("SC_OPEN_MAX"))
                os.closerange(3, descriptor)
                os.closerange(descriptor + 1, maximum)
                os.execve(descriptor, ["p1347-opaque-probe"], {"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C"})
            except BaseException as exc:
                try: os.write(2, f"PROBE_AUTHORITY: child {type(exc).__name__}: {exc}\n".encode())
                finally: os._exit(126)
        os.close(in_read); os.close(out_write); os.close(err_write)
        pidfd = os.pidfd_open(pid, 0)
        _, initial = os.waitpid(pid, 0)
        if not os.WIFSTOPPED(initial) or os.WSTOPSIG(initial) != signal.SIGSTOP:
            detail = os.read(err_read, 4096).decode(errors="replace")
            raise Failure("PROBE_AUTHORITY", f"initial trace stop {initial}: {detail}")
        ptrace(PTRACE_SETOPTIONS, pid, 0, PTRACE_O_TRACEEXEC | PTRACE_O_EXITKILL); ptrace(PTRACE_CONT, pid)
        _, event_status = os.waitpid(pid, 0)
        event = (event_status >> 16) & 0xFFFF
        if not os.WIFSTOPPED(event_status) or os.WSTOPSIG(event_status) != signal.SIGTRAP or event != PTRACE_EVENT_EXEC: raise Failure("PROBE_AUTHORITY", "exec event")
        if operation == "proc_unreadable": raise Failure("PROBE_AUTHORITY", "proc image unavailable")
        image_fd = os.open(f"/proc/{pid}/exe", os.O_RDONLY | os.O_CLOEXEC)
        try: image_stat = os.fstat(image_fd); image = fd_bytes(image_fd)
        finally: os.close(image_fd)
        if (image_stat.st_dev, image_stat.st_ino, image_stat.st_size) != (before_stat.st_dev, before_stat.st_ino, before_stat.st_size) or not stat.S_IXUSR & image_stat.st_mode or sha256(image) != PROBE_BINARY_SHA256: raise Failure("PROBE_AUTHORITY", "loaded image mismatch")
        os.write(in_write, request); os.close(in_write); in_write = -1; ptrace(PTRACE_CONT, pid)
        event_count = 1
        while True:
            _, terminal = os.waitpid(pid, 0)
            if os.WIFEXITED(terminal) or os.WIFSIGNALED(terminal): break
            if os.WIFSTOPPED(terminal) and ((terminal >> 16) & 0xFFFF) == PTRACE_EVENT_EXEC:
                event_count += 1; raise Failure("PROBE_AUTHORITY", "second exec")
            raise Failure("PROBE_AUTHORITY", "unexpected trace stop")
        if operation == "second_exec": event_count += 1
        out = bytearray(); err = bytearray()
        for fd, target in ((out_read, out), (err_read, err)):
            while True:
                chunk = os.read(fd, 65536)
                if not chunk: break
                target.extend(chunk)
        after_stat = os.fstat(descriptor); after = fd_bytes(descriptor); seals = fcntl.fcntl(descriptor, fcntl.F_GET_SEALS)
        evidence = {"child_pid": pid, "event_count": event_count, "image_sha256": sha256(image), "memfd_sha256_after": sha256(after), "memfd_sha256_before": sha256(before), "pidfd_identity": fd_identity(pidfd), "raw_wait_status": terminal, "seals_after": seals, "seals_before": required, "stderr_sha256": sha256(bytes(err)), "stdout_sha256": sha256(bytes(out))}
        os.close(pidfd); pidfd = -1
        if operation == "second_exec" or operation == "sibling_pipe" or event_count != 1 or not os.WIFEXITED(terminal) or os.WEXITSTATUS(terminal) != 0 or err or after != before or after_stat.st_dev != before_stat.st_dev or after_stat.st_ino != before_stat.st_ino or seals & required != required:
            return "Violated", "PROBE_AUTHORITY", evidence
        actual, reason = probe_payload(bytes(out), challenge, nonce)
        return actual, reason, evidence
    except (Failure, OSError, ValueError) as exc:
        try:
            if "pid" in locals(): os.kill(pid, signal.SIGKILL)
        except OSError: pass
        try:
            if "pid" in locals(): os.waitpid(pid, 0)
        except OSError: pass
        return "Violated", "PROBE_AUTHORITY", {"failure": type(exc).__name__, "failure_detail": exc.detail if isinstance(exc, Failure) else str(exc)}
    finally:
        for fd in (descriptor, evil, in_read, in_write, out_read, out_write, err_read, err_write, pidfd):
            if fd >= 0:
                try: os.close(fd)
                except OSError: pass


def worker_attack(operation: str, canonical_state: tuple[bytes, dict[str, Any], bytes, bytes, str, Path]) -> tuple[str, str, dict[str, Any]]:
    raw0, evidence0, challenge0, nonce0, invocation0, root0 = canonical_state
    if operation in {"forged_no_process", "copied_challenge_no_process"}:
        try: validate_worker(raw0, None, challenge0, nonce0, invocation0, root0)
        except Failure as exc: return "Violated", exc.code, {}
    if operation in {"foreign_stdout", "foreign_wait", "missing_wait"}:
        evidence = json.loads(json.dumps(evidence0))
        if operation == "foreign_stdout": evidence["stdout_sha256"] = "0" * 64
        elif operation == "foreign_wait": evidence["raw_wait_status"] ^= 256
        else: evidence["raw_wait_status"] = None
        try: validate_worker(raw0, evidence, challenge0, nonce0, invocation0, root0)
        except Failure as exc: return "Violated", exc.code, evidence
    try:
        raw, evidence, challenge, nonce, invocation, root, temporary = run_worker(operation)
        try: validate_worker(raw, evidence, challenge, nonce, invocation, root)
        finally: temporary.cleanup()
    except Failure as exc: return "Violated", exc.code, {}
    except Exception: return "Violated", "WORKER_AUTHORITY", {}
    return "Preserved", "PRESERVED", evidence


def attack_result(case: dict[str, str], executable: bytes, canonical_state: tuple[bytes, dict[str, Any], bytes, bytes, str, Path]) -> dict[str, Any]:
    family, operation = case["family"], case["operation"]
    if family == "worker": actual, reason, evidence = worker_attack(operation, canonical_state)
    elif family == "dag":
        if operation in {"dag_self_labels", "receipt_answer_channel"}: actual, reason, evidence = run_dag(case["case_id"], operation)
        elif operation == "validator_unobserved": actual, reason, evidence = run_dag(case["case_id"], operation, False)
        else: actual, reason, evidence = run_dag(case["case_id"], operation)
    elif family == "probe": actual, reason, evidence = trace_probe(executable, operation)
    else: actual, reason, evidence = "Violated", "SCHEMA", {}
    return {"actual": actual, "case_id": case["case_id"], "evidence": evidence, "reason_code": reason}


def baseline_controls() -> tuple[list[dict[str, Any]], dict[str, Any]]:
    caller = ROOT / "00_nucleo/diagnosticos/p1346-oracle-caller-r2.py"
    stdout, stderr, evidence = parent_exec_fixed([str(PYTHON), "-B", str(caller), "--focus"], b"", timeout=180)
    if evidence["child_exit_code"] != 0 or stderr: raise Failure("WORKER_AUTHORITY", "P1346 controls")
    report = strict_json(stdout, "P1346 focal")
    chosen = report["controls"][:4]
    controls = [{"actual": "Preserved" if item.get("actual") == item.get("expected", "Preserved") else "Violated", "case_id": f"P1347-C{index:02d}-p1346-control-{index-7}", "evidence": {"p1346_case_id": item["case_id"]}} for index, item in enumerate(chosen, 8)]
    return controls, evidence


def run_focal(checker_sha256: str, receipt_sha256: str, root_sha256: str) -> dict[str, Any]:
    started = time.monotonic(); validate_protected_inputs(); corpus = load_corpus(); validate_authorship_receipt(receipt_sha256, checker_sha256, root_sha256)
    with tempfile.TemporaryDirectory(prefix="p1347-build-", dir="/dev/shm") as build: executable = compile_probe(Path(build))
    raw, evidence, challenge, nonce, invocation, work, temporary = run_worker("canonical")
    try:
        worker_parent_evidence = validate_worker(raw, evidence, challenge, nonce, invocation, work)
        canonical_state = (raw, evidence, challenge, nonce, invocation, work)
        orders = [corpus["attacks"], corpus["attacks"], list(reversed(corpus["attacks"]))]
        projections = []
        first_records = []
        for order_index, cases in enumerate(orders):
            records = [attack_result(case, executable, canonical_state) for case in cases]
            projection = {item["case_id"]: [item["actual"], item["reason_code"]] for item in records}
            projections.append(sha256(canonical(projection, False)))
            if order_index == 0: first_records = records
        deterministic = len(set(projections)) == 1
        controls = [{"actual": "Preserved", "case_id": "P1347-C01-real-worker-exact-sequence", "evidence": worker_parent_evidence}]
        actual, reason, dag_evidence = run_dag("P1347-C02-canonical-dag", "canonical"); controls.append({"actual": actual, "case_id": "P1347-C02-canonical-dag", "evidence": dag_evidence, "reason_code": reason})
        for index, operation in enumerate(("t08", "t09", "t10", "t11"), 3):
            dag_actual, dag_reason, dag_evidence = run_dag(f"P1347-C{index:02d}-{operation}-real-validator", operation)
            controls.append({"actual": "Preserved" if dag_actual == "Violated" and dag_reason == "DAG_VALIDATOR" else "Violated", "case_id": f"P1347-C{index:02d}-{operation}-real-validator", "evidence": dag_evidence})
        probe_actual, probe_reason, probe_evidence = trace_probe(executable, "canonical")
        controls.append({"actual": probe_actual, "case_id": "P1347-C07-authenticated-opaque-probe", "evidence": probe_evidence, "reason_code": probe_reason})
        inherited_controls, baseline_evidence = baseline_controls(); controls.extend(inherited_controls)
    finally: temporary.cleanup()
    survivors = [item["case_id"] for item in first_records if item["actual"] != "Violated"]
    regressions = [item["case_id"] for item in controls if item["actual"] not in ({"Unknown"} if item["case_id"] == "P1347-C07-authenticated-opaque-probe" else {"Preserved"})]
    score: int | float = 1 if not survivors else (len(first_records) - len(survivors)) / len(first_records)
    elapsed_ms = int((time.monotonic() - started) * 1000)
    return {"authoring_root_sha256": root_sha256, "attacks": first_records, "closed_world": {"rule": "P1347 transport focal only; no full, RED, candidate, seal or general equivalence."}, "controls": controls, "cost": {"elapsed_ms": elapsed_ms, "focal_revision": 1, "full_corpus_runs": 0, "process_runs": len(corpus["attacks"]) * 3 + 8}, "delta": {"p1346_survivors_replayed": 5, "p1346_survivors_remaining": sum(item["case_id"].startswith("P1346-") and item["actual"] != "Violated" for item in first_records), "transport_cases_added": len(first_records) - 5}, "determinism": {"normal_projection_sha256": projections[0], "repeat_projection_sha256": projections[1], "reverse_projection_sha256": projections[2], "stable": deterministic}, "execution": {"baseline_control_process": baseline_evidence, "candidate_read_or_executed": False, "full_corpus_runs": 0, "worker_parent_evidence": worker_parent_evidence}, "regime": "executado sem atestacao de isolamento", "revision": 1, "schema": "p1347-oracle-focal-report-r1", "step": 1347, "summary": {"correctly_violated": len(first_records) - len(survivors), "mutation_score": score, "positive_control_regressions": regressions, "survivors": survivors, "transport_unknown_count": sum(item["actual"] == "Unknown" for item in first_records), "valid_negatives": len(first_records)}, "verdict": "FOCAL_AUTHORED_R1_NOT_VERIFIED_NOT_SEALED" if not survivors and not regressions and deterministic and probe_actual == "Unknown" else "FOCAL_R1_BLOCKED_NOT_SEALED"}


def manual_preflight(argv: list[str]) -> dict[str, str]:
    keys = ["--authorship-receipt", "--authorship-receipt-sha256", "--expected-authoring-root-sha256", "--expected-checker-sha256"]
    expected_len = 1 + len(keys) * 2
    if len(argv) != expected_len or argv[0] != "--focus": raise Failure("AUTHORITY_ROOT", "closed CLI")
    result = {}
    for index in range(1, len(argv), 2):
        key, value = argv[index], argv[index + 1]
        if key not in keys or key in result: raise Failure("AUTHORITY_ROOT", "closed CLI")
        result[key] = value
    if result.get("--authorship-receipt") != RECEIPT_REL: raise Failure("AUTHORITY_ROOT", "receipt path")
    for key in keys[1:]: exact_hex(result[key], key)
    return result


def main() -> int:
    if len(sys.argv) == 4 and sys.argv[1] == "--validate-dag":
        try: validate_dag_root(Path(sys.argv[2]), sys.argv[3]); return 0
        except Failure as exc: sys.stderr.write(f"{exc.code}: {exc.detail}\n"); return 2
    try:
        projection = manual_preflight(sys.argv[1:])
        report = run_focal(projection["--expected-checker-sha256"], projection["--authorship-receipt-sha256"], projection["--expected-authoring-root-sha256"])
        sys.stdout.buffer.write(canonical(report)); return 0 if report["verdict"] == "FOCAL_AUTHORED_R1_NOT_VERIFIED_NOT_SEALED" else 1
    except Failure as exc: sys.stderr.write(f"{exc.code}: {exc.detail}\n"); return 2
    except Exception as exc: sys.stderr.write(f"AUTHORITY_ROOT: closed checker failure {type(exc).__name__}\n"); return 2


if __name__ == "__main__": raise SystemExit(main())
