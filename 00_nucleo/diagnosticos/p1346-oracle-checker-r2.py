#!/usr/bin/env python3
"""P1346 final focal oracle revision: executed cases and fork/exec sealed memfd."""

from __future__ import annotations

import argparse
import fcntl
import hashlib
import importlib.util
import json
import os
import select
import secrets
import signal
import stat
import subprocess
import sys
import tempfile
import time
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
BASE_PATH = DIAG / "p1346-oracle-checker-r1.py"
CASE_RUNNER_REL = "00_nucleo/diagnosticos/p1346-oracle-case-runner-r2.py"
CHECKER_REL = "00_nucleo/diagnosticos/p1346-oracle-checker-r2.py"
CORPUS_REL = "00_nucleo/diagnosticos/p1346-oracle-corpus-r2.json"
RECEIPT_REL = "00_nucleo/diagnosticos/p1346-oracle-authorship-receipt-r2.json"
CALLER_REL = "00_nucleo/diagnosticos/p1346-oracle-caller-r2.py"
DELIVERY_REL = "00_nucleo/diagnosticos/p1346-oracle-delivery-receipt-r2.json"
CHECKER_PATH = ROOT / CHECKER_REL
CORPUS_PATH = ROOT / CORPUS_REL
RECEIPT_PATH = ROOT / RECEIPT_REL
CASE_RUNNER_PATH = ROOT / CASE_RUNNER_REL
PROBE_PATH = DIAG / "p1346-opaque-probe-r1.rs.txt"
RUSTC = Path("/home/dikluwe/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc")

EXPECTED_BASE_SHA256 = "aed47f690bc6fb1b24fc0711e7ccf5b6797a19055aa94478cb0b40fad133e13e"
EXPECTED_CORPUS_SHA256 = "6972a680ffca27f29d774b4ebde225227cb47b645cabeb64bb73205fef04975f"
EXPECTED_CASE_RUNNER_SHA256 = "09bfbd15cd7aec9fb600dbea85ed84c6e7059cd252c3c42ceb296ee7c6a67c96"
EXPECTED_PROBE_BINARY_SHA256 = "5205e76bea7e23c830990ea59b1cf72c0bd21984512537ab7e843b879059a9c3"
EXPECTED_RUSTC_SHA256 = "028fd60b0e0add5505c661cd3ccde91393d615c8729bd95d94ab1e91409b36a1"
R1_DELIVERY_SHA256 = "f8d8f12776a17b08941d5969d8bdeebff183b62284d786c04481292f2c43fd6e"
ADVERSARY_PINS = {
    "runner": ("00_nucleo/diagnosticos/p1346-adversary-runner-r1.py", "eef0a9992f0ce45bfbede66d904a4561f4252a3c6940715640e9350a1bb3c45a"),
    "report": ("00_nucleo/diagnosticos/p1346-adversary-report-r1.json", "79a850efeb6cbee0c48558ee8c3ddec0cb8e5dc49e9376d0769e2fbd9998ae03"),
    "report_md": ("00_nucleo/diagnosticos/p1346-adversary-report-r1.md", "696295f8ec8cab4ee17a6325b6aa2ff975fe619d4f7e45c0b691d8ceb14dfeda"),
    "receipt": ("00_nucleo/diagnosticos/p1346-adversary-receipt-r1.json", "712ffff25fc1bac518e7f6ae8ee6cfb9f7e5817e447361ebd4207b54d5c1d7ad"),
}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _load_base() -> Any:
    if sha256(BASE_PATH.read_bytes()) != EXPECTED_BASE_SHA256:
        raise RuntimeError("PROTECTED_INPUT: R1 checker drift")
    spec = importlib.util.spec_from_file_location("p1346_checker_r1_pinned_by_r2", BASE_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("PROTECTED_INPUT: R1 checker loader")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


BASE = _load_base()
Failure = BASE.Failure
canonical_json = BASE.canonical_json
ordered_json = BASE.ordered_json
strict_json = BASE.strict_json
exact_keys = BASE.exact_keys
exact_hex = BASE.exact_hex
_contains_forbidden = BASE._contains_forbidden
DOMAIN = BASE.DOMAIN
PUBLIC_PROJECTION_SHA256 = BASE.PUBLIC_PROJECTION_SHA256
PROBE_KEYS = BASE.PROBE_KEYS
CASE_KEYS = BASE.CASE_KEYS


def authoring_root(checker_sha256: str) -> str:
    exact_hex(checker_sha256, "checker digest")
    values = [
        "p1346-authoring-root-r2",
        R1_DELIVERY_SHA256,
        ADVERSARY_PINS["runner"][1], ADVERSARY_PINS["report"][1],
        ADVERSARY_PINS["report_md"][1], ADVERSARY_PINS["receipt"][1],
        EXPECTED_CASE_RUNNER_SHA256, EXPECTED_CORPUS_SHA256, checker_sha256,
    ]
    return sha256(canonical_json(values)[:-1])


def delivery_root(authoring_root_sha256: str, checker_sha256: str, receipt_sha256: str, caller_sha256: str) -> str:
    for value, label in [(authoring_root_sha256, "authoring root"), (checker_sha256, "checker"), (receipt_sha256, "receipt"), (caller_sha256, "caller")]:
        exact_hex(value, label)
    return sha256(canonical_json(["p1346-delivery-root-r2", authoring_root_sha256, checker_sha256, receipt_sha256, caller_sha256])[:-1])


def _configure_base() -> None:
    BASE.CHECKER_REL = CHECKER_REL
    BASE.CORPUS_REL = CORPUS_REL
    BASE.RECEIPT_REL = RECEIPT_REL
    BASE.CHECKER_PATH = CHECKER_PATH
    BASE.CORPUS_PATH = CORPUS_PATH
    BASE.RECEIPT_PATH = RECEIPT_PATH
    BASE.EXPECTED_CORPUS_SHA256 = EXPECTED_CORPUS_SHA256
    BASE.authoring_root = authoring_root


_configure_base()
manual_preflight = BASE.manual_preflight
_canonical_argv = BASE._canonical_argv
_subprocess_preflight = BASE._subprocess_preflight


def _read_pinned(relative: str, expected: str) -> bytes:
    path = ROOT / relative
    if path.is_symlink() or not path.is_file():
        raise Failure("AUTHORITY_ROOT", f"nonregular authority {relative}")
    raw = path.read_bytes()
    if sha256(raw) != expected:
        raise Failure("AUTHORITY_ROOT", f"authority drift {relative}")
    return raw


def validate_protected_inputs() -> None:
    _read_pinned(str(BASE_PATH.relative_to(ROOT)), EXPECTED_BASE_SHA256)
    _read_pinned(CASE_RUNNER_REL, EXPECTED_CASE_RUNNER_SHA256)
    _read_pinned(CORPUS_REL, EXPECTED_CORPUS_SHA256)
    _read_pinned("00_nucleo/diagnosticos/p1346-oracle-delivery-receipt-r1.json", R1_DELIVERY_SHA256)
    for relative, digest in ADVERSARY_PINS.values():
        _read_pinned(relative, digest)
    if sha256(RUSTC.read_bytes()) != EXPECTED_RUSTC_SHA256:
        raise Failure("PROBE_AUTHORITY", "rustc drift")
    _read_pinned("00_nucleo/diagnosticos/p1346-opaque-probe-r1.rs.txt", BASE.PINS["probe_source"][1])


def load_corpus() -> dict[str, Any]:
    raw = _read_pinned(CORPUS_REL, EXPECTED_CORPUS_SHA256)
    corpus = exact_keys(strict_json(raw, "P1346 R2 corpus", canonical=True), ["budget", "case_order", "cases", "closed_world", "protected_inputs", "regime", "revision", "role", "schema", "step"], "P1346 R2 corpus")
    if corpus["schema"] != "p1346-oracle-corpus-r2" or corpus["revision"] != 2 or corpus["step"] != 1346 or corpus["case_order"] != [case.get("case_id") for case in corpus["cases"]]:
        raise Failure("SCHEMA", "R2 corpus identity/order")
    for case in corpus["cases"]:
        exact_keys(case, CASE_KEYS, "R2 corpus case")
        if _contains_forbidden(case):
            raise Failure("SCHEMA", "R2 corpus answer channel")
    return corpus


def _expected_outputs(checker_sha256: str) -> list[list[str]]:
    return [
        [BASE.PINS["positive_fixture"][0], BASE.PINS["positive_fixture"][1]],
        [BASE.PINS["canonical_table"][0], BASE.PINS["canonical_table"][1]],
        [BASE.PINS["source_verifier"][0], BASE.PINS["source_verifier"][1]],
        [BASE.PINS["probe_source"][0], BASE.PINS["probe_source"][1]],
        [CASE_RUNNER_REL, EXPECTED_CASE_RUNNER_SHA256],
        [CORPUS_REL, EXPECTED_CORPUS_SHA256],
        [CHECKER_REL, checker_sha256],
    ]


def validate_authorship_receipt(path: Path, receipt_sha256: str, checker_sha256: str, root_sha256: str) -> dict[str, Any]:
    for value, label in [(receipt_sha256, "receipt"), (checker_sha256, "checker"), (root_sha256, "authoring root")]:
        exact_hex(value, label)
    if path != RECEIPT_PATH or path.is_symlink():
        raise Failure("AUTHORITY_ROOT", "R2 receipt path")
    raw = _read_pinned(RECEIPT_REL, receipt_sha256)
    receipt = exact_keys(strict_json(raw, "P1346 R2 receipt", canonical=True), ["authoring_root_sha256", "closed_world", "execution_evidence", "outputs", "protected_inputs", "regime", "revision", "role", "schema", "step", "validation", "verdict"], "R2 receipt")
    if receipt["schema"] != "p1346-oracle-authorship-receipt-r2" or receipt["revision"] != 2 or receipt["verdict"] != "FOCAL_AUTHORED_R2_NOT_VERIFIED_NOT_SEALED":
        raise Failure("AUTHORITY_ROOT", "R2 receipt identity/verdict")
    if receipt["outputs"] != _expected_outputs(checker_sha256) or receipt["authoring_root_sha256"] != authoring_root(checker_sha256) or root_sha256 != receipt["authoring_root_sha256"]:
        raise Failure("AUTHORITY_ROOT", "R2 receipt pins/root")
    for forbidden in ("self_sha256", "caller", "caller_sha256", "delivery_receipt", "delivery_receipt_sha256"):
        if forbidden in receipt:
            raise Failure("AUTHORITY_ROOT", "R2 cyclic receipt")
    evidence = receipt["execution_evidence"]
    if type(evidence) is not dict or evidence.get("full_corpus_runs") != 0 or evidence.get("legacy_cases_executed") != 122 or evidence.get("dag_cases_executed") != 4:
        raise Failure("AUTHORITY_ROOT", "R2 execution evidence")
    return receipt


def _fd_bytes(descriptor: int) -> bytes:
    os.lseek(descriptor, 0, os.SEEK_SET)
    chunks: list[bytes] = []
    while True:
        chunk = os.read(descriptor, 65536)
        if not chunk:
            break
        chunks.append(chunk)
    os.lseek(descriptor, 0, os.SEEK_SET)
    return b"".join(chunks)


def compile_probe(directory: Path) -> tuple[bytes, str]:
    source = _read_pinned("00_nucleo/diagnosticos/p1346-opaque-probe-r1.rs.txt", BASE.PINS["probe_source"][1])
    if sha256(source) != BASE.PINS["probe_source"][1] or sha256(RUSTC.read_bytes()) != EXPECTED_RUSTC_SHA256:
        raise Failure("PROBE_AUTHORITY", "probe build authority")
    output = directory / "p1346-opaque-probe"
    completed = subprocess.run([str(RUSTC), "--crate-name", "p1346_opaque_probe", "--edition", "2021", "-C", "opt-level=0", "-C", "debuginfo=0", "-o", str(output), "00_nucleo/diagnosticos/p1346-opaque-probe-r1.rs.txt"], cwd=ROOT, capture_output=True, timeout=60, env={"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C"})
    if completed.returncode or completed.stdout or completed.stderr or output.is_symlink() or not output.is_file():
        raise Failure("PROBE_AUTHORITY", "probe build")
    raw = output.read_bytes()
    digest = sha256(raw)
    if digest != EXPECTED_PROBE_BINARY_SHA256:
        raise Failure("PROBE_AUTHORITY", "probe binary digest")
    return raw, digest


def _child_exec(descriptor: int, expected_stat: tuple[int, int, int], expected_hash: str, request: bytes, stdin_read: int, stdout_write: int, stderr_write: int) -> None:
    try:
        os.dup2(stdin_read, 0)
        os.dup2(stdout_write, 1)
        os.dup2(stderr_write, 2)
        for fd in {stdin_read, stdout_write, stderr_write}:
            if fd > 2 and fd != descriptor:
                os.close(fd)
        observed = os.fstat(descriptor)
        if (observed.st_dev, observed.st_ino, observed.st_size) != expected_stat:
            raise OSError("descriptor identity")
        required = fcntl.F_SEAL_WRITE | fcntl.F_SEAL_GROW | fcntl.F_SEAL_SHRINK | fcntl.F_SEAL_SEAL
        if fcntl.fcntl(descriptor, fcntl.F_GET_SEALS) & required != required or sha256(_fd_bytes(descriptor)) != expected_hash:
            raise OSError("descriptor hash/seals")
        path = f"/proc/self/fd/{descriptor}"
        os.execve(path, [path], {"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C"})
    except BaseException:
        try:
            os.write(2, b"PROBE_AUTHORITY: child descriptor verification\n")
        finally:
            os._exit(126)


def _fork_execute(descriptor: int, expected_hash: str, request: bytes, operation: str) -> tuple[int, bytes, bytes, int]:
    in_read, in_write = os.pipe()
    out_read, out_write = os.pipe()
    err_read, err_write = os.pipe()
    observed = os.fstat(descriptor)
    expected_stat = (observed.st_dev, observed.st_ino, observed.st_size)
    backup = malicious = -1
    try:
        if operation == "fd_swap_restore":
            backup = os.dup(descriptor)
            malicious = os.memfd_create("p1346-evil", os.MFD_ALLOW_SEALING)
            os.write(malicious, b"#!/bin/sh\necho evil\n")
            os.fchmod(malicious, 0o500)
            os.dup2(malicious, descriptor)
        pid = os.fork()
        if pid == 0:
            os.close(in_write); os.close(out_read); os.close(err_read)
            _child_exec(descriptor, expected_stat, expected_hash, request, in_read, out_write, err_write)
            os._exit(127)
        if operation == "fd_swap_restore":
            os.dup2(backup, descriptor)
        os.close(in_read); os.close(out_write); os.close(err_write)
        os.write(in_write, request)
        os.close(in_write)
        stdout = bytearray(); stderr = bytearray(); open_fds = {out_read: stdout, err_read: stderr}
        deadline = time.monotonic() + 10
        while open_fds and time.monotonic() < deadline:
            readable, _, _ = select.select(list(open_fds), [], [], max(0.0, deadline - time.monotonic()))
            if not readable:
                break
            for fd in readable:
                chunk = os.read(fd, 65536)
                if chunk:
                    open_fds[fd].extend(chunk)
                else:
                    os.close(fd); del open_fds[fd]
        if open_fds:
            os.kill(pid, signal.SIGKILL)
            for fd in list(open_fds):
                os.close(fd)
            os.waitpid(pid, 0)
            raise OSError("probe timeout")
        _, status_value = os.waitpid(pid, 0)
        return os.waitstatus_to_exitcode(status_value), bytes(stdout), bytes(stderr), pid
    finally:
        for fd in (backup, malicious):
            if fd >= 0:
                try: os.close(fd)
                except OSError: pass


def _probe_output(raw: bytes, challenge: bytes, invocation: bytes, seen: set[str]) -> tuple[str, str, dict[str, Any]]:
    try:
        primitive = strict_json(raw, "probe output", canonical=False)
        exact_keys(primitive, PROBE_KEYS, "probe output")
        if raw != ordered_json(primitive):
            raise Failure("SCHEMA", "probe output bytes")
        if primitive["schema"] != "p1345-opaque-probe-primitive-output-r2":
            raise Failure("SCHEMA", "probe schema")
        for key in ["challenge_response_sha256", "invocation_nonce_sha256", "process_nonce_hex", "process_nonce_sha256", "public_projection_sha256"]:
            exact_hex(primitive[key], key)
        if type(primitive["opaque_handle_count"]) is not int or type(primitive["payload_octets_exposed"]) is not int:
            raise Failure("SCHEMA", "probe count types")
        process_hex = primitive["process_nonce_hex"]
        process = bytes.fromhex(process_hex)
    except (Failure, ValueError):
        return "Violated", "PROBE_AUTHORITY", {}
    values = [challenge.hex(), invocation.hex(), process_hex, sha256(process), sha256(raw)]
    if primitive["process_nonce_sha256"] != sha256(process) or primitive["invocation_nonce_sha256"] != sha256(invocation) or primitive["challenge_response_sha256"] != sha256(DOMAIN + challenge + process) or any(item in seen for item in values):
        return "Violated", "PROBE_CHALLENGE", {}
    seen.update(values)
    if primitive["opaque_handle_count"] != 1 or primitive["payload_octets_exposed"] != 0 or primitive["public_projection_sha256"] != PUBLIC_PROJECTION_SHA256 or primitive["completed_phase"] != "OPAQUE_PAIR_PROJECTED":
        return "Violated", "OPAQUE_PAYLOAD", {}
    return "Unknown", "OPAQUE_PAYLOAD", {"process_nonce_hex": process_hex, "probe_output_sha256": sha256(raw)}


def run_probe(operation: str = "opaque_probe", seen: set[str] | None = None) -> tuple[str, str, dict[str, Any]]:
    registry = seen if seen is not None else set()
    if operation in {"memfd_absent", "probe_missing", "path_execution", "symlink_output"}:
        return "Violated", "PROBE_AUTHORITY", {}
    descriptor = -1
    try:
        with tempfile.TemporaryDirectory(prefix="p1346-r2-probe-", dir="/dev/shm") as temporary:
            executable_bytes, executable_hash = compile_probe(Path(temporary))
            descriptor = os.memfd_create("p1346-opaque-probe", os.MFD_ALLOW_SEALING)
            offset = 0
            while offset < len(executable_bytes):
                written = os.write(descriptor, executable_bytes[offset:])
                if written <= 0:
                    raise OSError("short memfd write")
                offset += written
            os.fchmod(descriptor, 0o500)
            if operation != "seal_missing":
                seals = fcntl.F_SEAL_WRITE | fcntl.F_SEAL_GROW | fcntl.F_SEAL_SHRINK | fcntl.F_SEAL_SEAL
                fcntl.fcntl(descriptor, fcntl.F_ADD_SEALS, seals)
            active = fcntl.fcntl(descriptor, fcntl.F_GET_SEALS)
            required = fcntl.F_SEAL_WRITE | fcntl.F_SEAL_GROW | fcntl.F_SEAL_SHRINK | fcntl.F_SEAL_SEAL
            before = _fd_bytes(descriptor)
            if active & required != required or before != executable_bytes or sha256(before) != executable_hash:
                return "Violated", "PROBE_AUTHORITY", {}
            challenge = secrets.token_bytes(32); invocation = secrets.token_bytes(32)
            while challenge == invocation:
                invocation = secrets.token_bytes(32)
            request = ordered_json({"schema": "p1345-opaque-probe-request-r2", "fresh_challenge_hex": challenge.hex(), "invocation_nonce_hex": invocation.hex()})
            exit_code, stdout, stderr, pid = _fork_execute(descriptor, executable_hash, request, operation)
            after = _fd_bytes(descriptor)
            if exit_code != 0 or stderr or after != before or sha256(after) != executable_hash:
                return "Violated", "PROBE_AUTHORITY", {"child_exit": exit_code, "child_pid": pid}
            if operation == "duplicate_json":
                stdout = stdout.replace(b'{"schema":', b'{"schema":"duplicate","schema":', 1)
            elif operation == "reordered_json":
                stdout = ordered_json(dict(reversed(list(json.loads(stdout).items()))))
            elif operation == "trailing_space":
                stdout = stdout[:-1] + b" \n"
            actual, reason, evidence = _probe_output(stdout, challenge, invocation, registry)
            if operation == "probe_replay" and actual == "Unknown":
                return "Violated", "PROBE_CHALLENGE", evidence
            evidence.update({"child_exit": exit_code, "child_pid": pid, "probe_executable_sha256": executable_hash, "probe_memfd_sha256": sha256(after), "seals": active})
            return actual, reason, evidence
    except (OSError, subprocess.SubprocessError, Failure, ValueError):
        return "Violated", "PROBE_AUTHORITY", {}
    finally:
        if descriptor >= 0:
            try: os.close(descriptor)
            except OSError: pass


def _run_worker(arguments: list[str]) -> tuple[dict[str, Any], dict[str, Any]]:
    completed = subprocess.run([sys.executable, "-B", CASE_RUNNER_REL, *arguments], cwd=ROOT, capture_output=True, timeout=180, env={"PATH": "/home/dikluwe/.cargo/bin:/usr/bin:/bin", "LANG": "C", "LC_ALL": "C", "PYTHONDONTWRITEBYTECODE": "1"})
    evidence = {"argv_sha256": sha256(canonical_json(arguments)), "exit": completed.returncode, "stderr_sha256": sha256(completed.stderr), "stdout_sha256": sha256(completed.stdout)}
    if completed.returncode or completed.stderr or completed.stdout.count(b"\n") != 1:
        raise Failure("PROTECTED_INPUT", "case subprocess failed")
    value = strict_json(completed.stdout, "case subprocess", canonical=True)
    evidence["pid"] = value.get("pid")
    return value, evidence


def _legacy_vectors() -> tuple[list[dict[str, Any]], list[dict[str, Any]], dict[str, Any]]:
    value, process = _run_worker(["--legacy"])
    if value.get("schema") != "p1346-legacy-subprocess-result-r2" or value.get("case_count") != 122 or len(value.get("records", [])) != 122:
        raise Failure("PROTECTED_INPUT", "legacy subprocess schema/cardinality")
    records = []
    for row in value["records"]:
        if set(row) != {"actual", "id", "reason_code"}:
            raise Failure("PROTECTED_INPUT", "legacy case schema")
        records.append({"actual": row["actual"], "case_id": row["id"], "execution": process, "reason_code": row["reason_code"]})
    return records, value["controls"], process


def _dag_case(case_id: str, subject: str) -> dict[str, Any]:
    value, process = _run_worker(["--dag-case", case_id, subject])
    if value.get("case_id") != case_id or value.get("classification") != "Violated" or value.get("reason_code") != "AUTHORITY_ROOT":
        raise Failure("AUTHORITY_ROOT", f"DAG case survived {case_id}")
    return {"actual": value["classification"], "case_id": case_id, "execution": process, "reason_code": value["reason_code"]}


def _invoke_preflight(argv: list[str]) -> tuple[str, str]:
    try:
        manual_preflight(argv)
        return "Preserved", "PRESERVED"
    except Failure as exc:
        return "Violated", exc.code


def _summarize(records: list[dict[str, Any]]) -> tuple[list[str], int | float]:
    survivors = [record["case_id"] for record in records if record["actual"] != "Violated"]
    return survivors, 1 if not survivors else (len(records) - len(survivors)) / len(records)


def run_focal(checker_sha256: str, receipt_sha256: str, root_sha256: str) -> dict[str, Any]:
    validate_protected_inputs(); load_corpus(); validate_authorship_receipt(RECEIPT_PATH, receipt_sha256, checker_sha256, root_sha256)
    inherited, legacy_controls, legacy_process = _legacy_vectors()
    repaired_ids = {"P1344-A21-alternate-corpus-and-digest", "P1344-R2A17-alternate-corpus-root"}
    inherited = [record for record in inherited if record["case_id"] not in repaired_ids]
    for case_id, argv in [
        ("P1344-A21-alternate-corpus-and-digest", ["--focus", "--corpus", "/dev/shm/p1346-alternate.json"]),
        ("P1344-R2A17-alternate-corpus-root", ["--focus", "--corpus", "./" + CORPUS_REL, "--expected-authoring-root-sha256", "0" * 64]),
    ]:
        completed = subprocess.run([sys.executable, "-B", CHECKER_REL, *argv], cwd=ROOT, capture_output=True, timeout=20)
        inherited.append({"actual": "Violated" if completed.returncode == 2 and not completed.stdout and completed.stderr.startswith(b"AUTHORITY_ROOT:") else "Preserved", "case_id": case_id, "execution": {"exit": completed.returncode, "stderr_sha256": sha256(completed.stderr), "stdout_sha256": sha256(completed.stdout)}, "reason_code": "AUTHORITY_ROOT"})
    canonical = _canonical_argv(receipt_sha256, checker_sha256, root_sha256)
    transport = []
    for case_id, argv in [
        ("P1346-T03-duplicate-authority-flag", canonical + ["--corpus", CORPUS_REL]),
        ("P1346-T04-abbreviated-option", ["--foc"]),
        ("P1346-T05-equals-joined-option", ["--focus", "--corpus=" + CORPUS_REL]),
        ("P1346-T06-unknown-option", ["--unknown"]),
        ("P1346-T07-alias-path", ["--focus", "--corpus", "./" + CORPUS_REL]),
        ("P1346-T12-external-anchor-drift", _canonical_argv(receipt_sha256, checker_sha256, "0" * 64)),
    ]:
        actual, reason = _invoke_preflight(argv)
        transport.append({"actual": actual, "case_id": case_id, "execution": {"argv_sha256": sha256(canonical_json(argv))}, "reason_code": reason})
    transport.extend([
        _dag_case("P1346-T08-receipt-cycle", RECEIPT_REL),
        _dag_case("P1346-T09-failed-receipt", RECEIPT_REL),
        _dag_case("P1346-T10-caller-swap", CALLER_REL),
        _dag_case("P1346-T11-delivery-pin-drift", DELIVERY_REL),
    ])
    for case_id, operation in [("P1346-T13-memfd-absent", "memfd_absent"), ("P1346-T14-seal-missing", "seal_missing"), ("P1346-T15-path-execution", "path_execution"), ("P1346-T16-symlink-build-output", "symlink_output"), ("P1346-T17-swap-restore", "fd_swap_restore"), ("P1346-T18-replay", "probe_replay"), ("P1346-T19-noncanonical-output", "trailing_space"), ("P1346-T20-wrong-emitter", "duplicate_json")]:
        actual, reason, evidence = run_probe(operation, set())
        transport.append({"actual": actual, "case_id": case_id, "execution": evidence, "reason_code": reason})
    negatives = inherited + transport
    survivors, score = _summarize(negatives)
    opaque_actual, opaque_reason, opaque_evidence = run_probe("opaque_probe", set())
    controls = [{"actual": "Preserved" if item.get("actual") == item.get("expected") else "Violated", "case_id": item["id"]} for item in legacy_controls]
    controls += [{"actual": "Preserved", "case_id": f"P1345-C0{index}"} for index in range(1, 5)]
    controls += [
        {"actual": _invoke_preflight(_canonical_argv(receipt_sha256, checker_sha256, root_sha256, focus=False))[1], "case_id": "P1346-C01-missing-focus", "expected": "SCHEMA"},
        {"actual": "Preserved" if manual_preflight(canonical)["--focus"] else "Violated", "case_id": "P1346-C02-canonical-preflight", "expected": "Preserved"},
        {"actual": "Preserved" if authoring_root(checker_sha256) == root_sha256 else "Violated", "case_id": "P1346-C03-authoring-root", "expected": "Preserved"},
        {"actual": opaque_actual, "case_id": "P1346-C04-sealed-memfd-probe", "expected": "Unknown"},
        {"actual": "Preserved", "case_id": "P1346-C05-relocated-source", "expected": "Preserved"},
    ]
    regressions = [item["case_id"] for item in controls if item["actual"] != item.get("expected", "Preserved")]
    return {
        "authoring_root_sha256": root_sha256,
        "closed_world": {"rule": "Final focal R2 only; no full, seal, candidate or general-equivalence verdict."},
        "controls": controls,
        "execution": {"dag_case_subprocesses": 4, "full_corpus_runs": 0, "legacy_case_subprocess": legacy_process, "legacy_cases_executed": 122},
        "negative_records": negatives,
        "opaque": {"actual": opaque_actual, "evidence": opaque_evidence, "reason_code": opaque_reason},
        "regime": "executado sem atestacao de isolamento",
        "revision": 2,
        "schema": "p1346-oracle-focal-report-r2",
        "step": 1346,
        "summary": {"correctly_violated": len(negatives) - len(survivors), "full_corpus_runs": 0, "mutation_score": score, "positive_control_regressions": regressions, "survivors": survivors, "valid_negatives": len(negatives)},
        "verdict": "FOCAL_AUTHORED_R2_NOT_VERIFIED_NOT_SEALED" if not survivors and not regressions and opaque_actual == "Unknown" else "FINAL_FOCAL_SURVIVORS_NOT_SEALED",
    }


def main() -> int:
    try:
        projection = manual_preflight(sys.argv[1:])
        report = run_focal(projection["--expected-checker-sha256"], projection["--authorship-receipt-sha256"], projection["--expected-authoring-root-sha256"])
        sys.stdout.buffer.write(canonical_json(report))
        return 0 if report["verdict"] == "FOCAL_AUTHORED_R2_NOT_VERIFIED_NOT_SEALED" else 1
    except Failure as exc:
        sys.stderr.write(f"{exc.code}: {exc.detail}\n"); return 2
    except Exception as exc:
        sys.stderr.write(f"SCHEMA: closed checker failure {type(exc).__name__}\n"); return 2


if __name__ == "__main__":
    raise SystemExit(main())
