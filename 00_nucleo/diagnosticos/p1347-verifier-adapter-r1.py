#!/usr/bin/env python3
"""Operational adapter from the frozen P1347 adversary suite to oracle R2.

This module is intentionally an adapter, not a verdict authority.  It never
imports the adversary suite, never reads an expected result, and never runs a
focal on import.  ``exercise`` performs exactly one requested operation and
reports the classification actually produced by the frozen checker/caller
APIs.  Later suite orchestration owns comparison with the frozen expectations.
"""

from __future__ import annotations

import atexit
import copy
import hashlib
import importlib.util
import json
import os
import secrets
import signal
import sys
import tempfile
from pathlib import Path
from typing import Any, Callable


RESULT_SCHEMA = "p1347-verifier-adapter-result-r1"
EVIDENCE_SCHEMA = "p1347-verifier-adapter-evidence-r1"
SEQUENCE_SHA256 = "e958b9c5e939a4b06b9e8594966001004de814e49e0141034d5351f1dbf4a907"
PTRACE_DETACH = 17

# Closed dispatch table.  Variants sharing an operation name remain distinct
# through their frozen parameters (for example ID mutation kind or syscall).
OPERATION_ROUTES = {
    "adversarial_image": "probe",
    "append_worker_stdout": "worker",
    "canonical_dag": "control",
    "canonical_traced_probe": "control",
    "canonical_worker": "control",
    "dag_coordinated_substitution": "dag",
    "dag_labels_without_validator": "dag",
    "dag_mutant_anchor": "dag",
    "dag_mutation_accepted_by_real_validator": "dag",
    "dag_receipt_answer_channel": "dag",
    "dag_validator_without_parent_process": "dag",
    "detach_before_terminal_wait": "probe",
    "drop_parent_wait": "worker",
    "dup_over_after_child_check": "probe",
    "fail_proc_exe": "probe",
    "fail_syscall": "probe",
    "inject_worker_pid": "worker",
    "mix_process_evidence": "parameterized_process",
    "mutate_id_sequence": "worker",
    "p1346_control_projection": "control",
    "real_dag_validator": "control",
    "remove_capability": "probe",
    "replay_p1346_negative": "replay",
    "replay_worker_challenge": "worker",
    "second_exec_after_attestation": "probe",
    "stdout_without_child": "worker",
    "substitute_proc_exe_image": "probe",
    "substitute_process_identity": "worker",
    "suppress_exec_stop": "probe",
    "swap_fd_after_exec_stop": "probe",
    "swap_fd_before_exec": "probe",
    "swap_fd_inside_exec": "probe",
    "truncate_worker_stdout": "worker",
    "worker_exit": "worker",
    "worker_signal": "worker",
    "worker_timeout": "worker",
}

_CONTEXTS: dict[tuple[str, ...], tuple[Any, Any, str]] = {}
_CANONICAL_WORKERS: dict[tuple[str, ...], tuple[Any, ...]] = {}
_PROBE_IMAGES: dict[tuple[str, ...], bytes] = {}
_P1346_REPORTS: dict[tuple[str, ...], dict[str, Any]] = {}
_TEMPORARIES: list[tempfile.TemporaryDirectory[str]] = []


def sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def canonical(value: Any, trailing_lf: bool = True) -> bytes:
    raw = json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
    return raw + (b"\n" if trailing_lf else b"")


def _strict_json(raw: bytes, label: str) -> Any:
    def pairs(items: list[tuple[str, Any]]) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key, value in items:
            if key in result:
                raise RuntimeError(f"DUPLICATE_KEY: {label}:{key}")
            result[key] = value
        return result

    try:
        value = json.loads(
            raw.decode("utf-8", "strict"),
            object_pairs_hook=pairs,
            parse_constant=lambda token: (_ for _ in ()).throw(ValueError(token)),
        )
    except (UnicodeError, ValueError, json.JSONDecodeError) as exc:
        raise RuntimeError(f"SCHEMA: {label}") from exc
    if canonical(value) != raw:
        raise RuntimeError(f"SCHEMA: {label}:noncanonical")
    return value


def _target_key(targets: Any) -> tuple[str, ...]:
    return (
        str(Path(targets.checker).resolve()),
        str(targets.checker_sha256),
        str(Path(targets.caller).resolve()),
        str(targets.caller_sha256),
        str(Path(targets.delivery_receipt).resolve()),
        str(targets.delivery_receipt_sha256),
    )


def _pinned_file(path: Path, expected: str, label: str) -> bytes:
    if path.is_symlink() or not path.is_file():
        raise RuntimeError(f"AUTHORITY_ROOT: {label}:path")
    raw = path.read_bytes()
    if sha256(raw) != expected:
        raise RuntimeError(f"AUTHORITY_ROOT: {label}:sha256")
    return raw


def _load_module(path: Path, name: str) -> Any:
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"AUTHORITY_ROOT: loader:{path.name}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def _context(targets: Any) -> tuple[Any, Any, str]:
    key = _target_key(targets)
    if key in _CONTEXTS:
        return _CONTEXTS[key]

    checker_supplied = Path(targets.checker)
    caller_supplied = Path(targets.caller)
    delivery_supplied = Path(targets.delivery_receipt)
    _pinned_file(checker_supplied, targets.checker_sha256, "checker")
    _pinned_file(caller_supplied, targets.caller_sha256, "caller")
    delivery_raw = _pinned_file(delivery_supplied, targets.delivery_receipt_sha256, "delivery receipt")
    checker_path = checker_supplied.resolve()
    caller_path = caller_supplied.resolve()
    delivery = _strict_json(delivery_raw, "delivery receipt")
    delivered = delivery.get("delivered")
    if type(delivered) is not dict:
        raise RuntimeError("AUTHORITY_ROOT: delivery schema")
    if (
        delivered.get("checker_sha256") != targets.checker_sha256
        or delivered.get("caller_sha256") != targets.caller_sha256
        or delivered.get("authorship_receipt_sha256") is None
    ):
        raise RuntimeError("AUTHORITY_ROOT: delivered targets")

    caller = _load_module(caller_path, f"p1347_adapter_caller_{targets.caller_sha256[:16]}")
    checker = caller.validate_chain()
    loaded_path = Path(checker.__file__).resolve()
    if loaded_path != checker_path or sha256(loaded_path.read_bytes()) != targets.checker_sha256:
        raise RuntimeError("AUTHORITY_ROOT: caller returned alternate checker")
    computed_delivery = checker.delivery_root(
        delivery["authoring_root_sha256"],
        targets.checker_sha256,
        delivered["authorship_receipt_sha256"],
        targets.caller_sha256,
    )
    if delivery.get("delivery_root_sha256") != computed_delivery:
        raise RuntimeError("AUTHORITY_ROOT: external delivery root")
    binding = sha256(canonical([*key, computed_delivery], False))
    _CONTEXTS[key] = (checker, caller, binding)
    return _CONTEXTS[key]


def _cleanup() -> None:
    while _TEMPORARIES:
        temporary = _TEMPORARIES.pop()
        try:
            temporary.cleanup()
        except Exception:
            pass


atexit.register(_cleanup)


def _exact_ids(checker: Any, supplied: Any) -> list[str]:
    if type(supplied) is not list or len(supplied) != 122:
        raise RuntimeError("PROTECTED_INPUT: exact_ids cardinality")
    ids = list(supplied)
    if any(type(item) is not str or not item.isascii() for item in ids) or len(set(ids)) != 122:
        raise RuntimeError("PROTECTED_INPUT: exact_ids identity")
    if sha256(canonical(ids, False)) != SEQUENCE_SHA256 or ids != checker.protected_ids():
        raise RuntimeError("PROTECTED_INPUT: exact_ids sequence")
    return ids


def _case(case: Any) -> tuple[str, str, dict[str, Any]]:
    if type(case) is not dict:
        raise RuntimeError("SCHEMA: case object")
    case_id = case.get("case_id")
    operation = case.get("adapter_operation")
    parameters = case.get("parameters", {})
    if type(case_id) is not str or type(operation) is not str or type(parameters) is not dict:
        raise RuntimeError("SCHEMA: case identity/operation/parameters")
    return case_id, operation, parameters


def _jsonable(value: Any) -> Any:
    if value is None or type(value) in (str, int, float, bool):
        return value
    if type(value) is bytes:
        return {"bytes_len": len(value), "bytes_sha256": sha256(value)}
    if isinstance(value, Path):
        return str(value)
    if type(value) is dict:
        return {str(key): _jsonable(item) for key, item in value.items()}
    if type(value) in (list, tuple):
        return [_jsonable(item) for item in value]
    return {"type": type(value).__name__, "repr_sha256": sha256(repr(value).encode())}


def _result(
    case_id: str,
    classification: str,
    reason_code: str,
    binding: str,
    api: list[str],
    injection: str,
    observation: Any,
) -> dict[str, Any]:
    if classification not in {"Preserved", "Violated", "Unknown"}:
        classification, reason_code = "Violated", "SCHEMA"
    if type(reason_code) is not str:
        classification, reason_code = "Violated", "SCHEMA"
    evidence = {
        "api": api,
        "injection": injection,
        "observation": _jsonable(observation),
        "schema": EVIDENCE_SCHEMA,
        "target_binding_sha256": binding,
    }
    return {
        "case_id": case_id,
        "classification": classification,
        "evidence": evidence,
        "reason_code": reason_code,
        "schema": RESULT_SCHEMA,
    }


def _failure(checker: Any, exc: BaseException) -> tuple[str, str, dict[str, Any]]:
    if isinstance(exc, checker.Failure):
        return "Violated", exc.code, {"failure_detail": exc.detail}
    detail = str(exc)
    for code in ("AUTHORITY_ROOT", "PROTECTED_INPUT", "DUPLICATE_KEY", "SCHEMA"):
        if detail.startswith(code + ":"):
            return "Violated", code, {"failure_detail": detail.split(":", 1)[1].strip()}
    return "Violated", "AUTHORITY_ROOT", {
        "failure_type": type(exc).__name__,
        "failure_repr_sha256": sha256(repr(exc).encode()),
    }


def _canonical_worker(checker: Any, key: tuple[str, ...]) -> tuple[Any, ...]:
    if key not in _CANONICAL_WORKERS:
        raw, evidence, challenge, nonce, invocation, root, temporary = checker.run_worker("canonical")
        checker.validate_worker(raw, evidence, challenge, nonce, invocation, root)
        _TEMPORARIES.append(temporary)
        _CANONICAL_WORKERS[key] = (raw, evidence, challenge, nonce, invocation, root)
    return _CANONICAL_WORKERS[key]


def _validate_worker_variant(
    checker: Any,
    state: tuple[Any, ...],
    mutate: Callable[[bytes, dict[str, Any]], tuple[bytes, dict[str, Any]]],
) -> tuple[str, str, dict[str, Any]]:
    raw, evidence, challenge, nonce, invocation, root = state
    changed_raw, changed_evidence = mutate(raw, copy.deepcopy(evidence))
    try:
        accepted = checker.validate_worker(changed_raw, changed_evidence, challenge, nonce, invocation, root)
        return "Preserved", "PRESERVED", accepted
    except BaseException as exc:
        return _failure(checker, exc)


def _with_stdout_evidence(checker: Any, raw: bytes, evidence: dict[str, Any]) -> tuple[bytes, dict[str, Any]]:
    evidence["stdout_len"] = len(raw)
    evidence["stdout_sha256"] = checker.sha256(raw)
    return raw, evidence


def _worker_operation(checker: Any, key: tuple[str, ...], operation: str, parameters: dict[str, Any]) -> tuple[str, str, dict[str, Any], str]:
    state = _canonical_worker(checker, key)
    raw0, evidence0, challenge0, nonce0, invocation0, root0 = state

    if operation == "stdout_without_child":
        try:
            checker.validate_worker(raw0, None, challenge0, nonce0, invocation0, root0)
            return "Preserved", "PRESERVED", {}, "remove parent-owned process evidence"
        except BaseException as exc:
            actual, reason, evidence = _failure(checker, exc)
            return actual, reason, evidence, "remove parent-owned process evidence"

    if operation == "mutate_id_sequence":
        def mutate(raw: bytes, evidence: dict[str, Any]) -> tuple[bytes, dict[str, Any]]:
            response = checker.strict_json(raw, "adapter worker response")
            receipts = response["mutation_receipts"]
            mutation = parameters.get("mutation")
            if mutation == "omit":
                receipts.pop(int(parameters.get("ordinal", 61)))
            elif mutation == "extra":
                receipts.append(dict(receipts[-1], case_id=parameters.get("value", "P1347-EXTRA")))
            elif mutation == "duplicate":
                ordinal = int(parameters.get("ordinal", 1))
                source = 0 if ordinal != 0 else 1
                receipts[ordinal] = dict(receipts[ordinal], case_id=receipts[source]["case_id"])
            elif mutation == "swap":
                left, right = parameters.get("ordinals", [0, 1])
                receipts[left], receipts[right] = receipts[right], receipts[left]
            elif mutation == "unicode_confusable":
                ordinal = int(parameters.get("ordinal", 0))
                receipts[ordinal] = dict(receipts[ordinal], case_id=parameters["replacement"])
            else:
                raise RuntimeError("SCHEMA: unknown ID mutation")
            return _with_stdout_evidence(checker, checker.canonical(response), evidence)

        actual, reason, evidence = _validate_worker_variant(checker, state, mutate)
        return actual, reason, evidence, f"mutation_receipts sequence:{parameters.get('mutation')}"

    if operation == "inject_worker_pid":
        def mutate(raw: bytes, evidence: dict[str, Any]) -> tuple[bytes, dict[str, Any]]:
            response = checker.strict_json(raw, "adapter worker response")
            response["pid"] = parameters.get("value")
            return _with_stdout_evidence(checker, checker.canonical(response), evidence)

        actual, reason, evidence = _validate_worker_variant(checker, state, mutate)
        return actual, reason, evidence, "worker response top-level pid"

    if operation == "substitute_process_identity":
        def mutate(raw: bytes, evidence: dict[str, Any]) -> tuple[bytes, dict[str, Any]]:
            evidence["child_pid"] = os.getpid()
            return raw, evidence

        actual, reason, evidence = _validate_worker_variant(checker, state, mutate)
        return actual, reason, evidence, "parent evidence child_pid -> sibling adapter PID"

    if operation == "drop_parent_wait":
        def mutate(raw: bytes, evidence: dict[str, Any]) -> tuple[bytes, dict[str, Any]]:
            evidence["raw_wait_status"] = None
            return raw, evidence

        actual, reason, evidence = _validate_worker_variant(checker, state, mutate)
        return actual, reason, evidence, "parent evidence raw_wait_status removed"

    if operation == "mix_process_evidence":
        foreign = parameters.get("foreign")
        if foreign == "probe_stdout_and_status":
            return (*_probe_operation(checker, key, "mix_process_evidence", parameters)[:3], "probe stdout/status from sibling")

        def mutate(raw: bytes, evidence: dict[str, Any]) -> tuple[bytes, dict[str, Any]]:
            if foreign == "stdout":
                evidence["stdout_sha256"] = "0" * 64
            elif foreign == "stderr":
                evidence["stderr_len"] = 1
                evidence["stderr_sha256"] = checker.sha256(b"x")
            elif foreign == "wait_status":
                evidence["raw_wait_status"] ^= 0x100
            else:
                raise RuntimeError("SCHEMA: unknown foreign process evidence")
            return raw, evidence

        actual, reason, evidence = _validate_worker_variant(checker, state, mutate)
        return actual, reason, evidence, f"parent evidence foreign:{foreign}"

    if operation in {"truncate_worker_stdout", "append_worker_stdout"}:
        def mutate(raw: bytes, evidence: dict[str, Any]) -> tuple[bytes, dict[str, Any]]:
            if operation == "truncate_worker_stdout":
                changed = raw[: len(raw) // 2]
            else:
                changed = raw + bytes.fromhex(str(parameters.get("bytes", "0a")))
            return _with_stdout_evidence(checker, changed, evidence)

        actual, reason, evidence = _validate_worker_variant(checker, state, mutate)
        return actual, reason, evidence, operation

    if operation == "replay_worker_challenge":
        try:
            first = checker.validate_worker(raw0, copy.deepcopy(evidence0), challenge0, nonce0, invocation0, root0)
            second = checker.validate_worker(raw0, copy.deepcopy(evidence0), challenge0, nonce0, invocation0, root0)
            return "Preserved", "PRESERVED", {
                "first_evidence_sha256": sha256(canonical(first, False)),
                "second_evidence_sha256": sha256(canonical(second, False)),
                "same_challenge_reused": True,
            }, "revalidate identical challenge/nonce/invocation evidence"
        except BaseException as exc:
            actual, reason, evidence = _failure(checker, exc)
            return actual, reason, evidence, "revalidate identical challenge/nonce/invocation evidence"

    direct = {
        "worker_timeout": "timeout",
        "worker_signal": "signal",
        "worker_exit": "unexpected_exit",
    }
    if operation in direct:
        try:
            raw, evidence, challenge, nonce, invocation, root, temporary = checker.run_worker(direct[operation])
            try:
                accepted = checker.validate_worker(raw, evidence, challenge, nonce, invocation, root)
                return "Preserved", "PRESERVED", accepted, f"worker mutation:{direct[operation]}"
            finally:
                temporary.cleanup()
        except BaseException as exc:
            actual, reason, evidence = _failure(checker, exc)
            return actual, reason, evidence, f"worker mutation:{direct[operation]}"

    raise RuntimeError(f"SCHEMA: unmapped worker operation:{operation}")


def _dag_operation(checker: Any, case_id: str, operation: str) -> tuple[str, str, dict[str, Any], str]:
    mapped = {
        "dag_labels_without_validator": ("dag_self_labels", True),
        "dag_mutation_accepted_by_real_validator": ("extra_file", True),
        "dag_mutant_anchor": ("mutant_anchor", True),
        "dag_validator_without_parent_process": ("validator_unobserved", False),
        "dag_receipt_answer_channel": ("receipt_answer_channel", True),
        "dag_coordinated_substitution": ("mutant_anchor", True),
    }
    if operation not in mapped:
        raise RuntimeError(f"SCHEMA: unmapped DAG operation:{operation}")
    worker_operation, observe = mapped[operation]
    actual, reason, evidence = checker.run_dag(case_id, worker_operation, observe)
    return actual, reason, evidence, f"DAG worker:{worker_operation}; parent validator observed:{observe}"


def _probe_image(checker: Any, key: tuple[str, ...]) -> bytes:
    if key not in _PROBE_IMAGES:
        with tempfile.TemporaryDirectory(prefix="p1347-adapter-build-", dir="/dev/shm") as temporary:
            _PROBE_IMAGES[key] = checker.compile_probe(Path(temporary))
    return _PROBE_IMAGES[key]


def _patched_probe(checker: Any, executable: bytes, patches: list[tuple[Any, str, Any]], injection: str) -> tuple[str, str, dict[str, Any], str]:
    saved: list[tuple[Any, str, Any]] = []
    try:
        for owner, name, replacement in patches:
            saved.append((owner, name, getattr(owner, name)))
            setattr(owner, name, replacement)
        actual, reason, evidence = checker.trace_probe(executable, "canonical")
        return actual, reason, evidence, injection
    finally:
        for owner, name, original in reversed(saved):
            setattr(owner, name, original)


def _probe_operation(checker: Any, key: tuple[str, ...], operation: str, parameters: dict[str, Any]) -> tuple[str, str, dict[str, Any], str]:
    executable = _probe_image(checker, key)
    direct = {
        "swap_fd_before_exec": "fd_swap_before",
        "dup_over_after_child_check": "fd_swap_postcheck",
        "second_exec_after_attestation": "second_exec",
        "adversarial_image": "mimic_image",
        "suppress_exec_stop": "exec_event_absent",
        "fail_proc_exe": "proc_unreadable",
        "mix_process_evidence": "sibling_pipe",
    }
    if operation == "swap_fd_inside_exec":
        mapped = "fd_swap_postcheck" if parameters.get("inherited") else "fd_swap_wrapper"
        actual, reason, evidence = checker.trace_probe(executable, mapped)
        return actual, reason, evidence, f"trace probe:{mapped}"
    if operation in direct:
        mapped = direct[operation]
        actual, reason, evidence = checker.trace_probe(executable, mapped)
        return actual, reason, evidence, f"trace probe:{mapped}"

    if operation == "remove_capability":
        capability = parameters.get("capability")
        if capability == "pidfd_open":
            actual, reason, evidence = checker.trace_probe(executable, "pidfd_absent")
            return actual, reason, evidence, "pidfd_open capability removed"
        if capability == "ptrace":
            actual, reason, evidence = checker.trace_probe(executable, "ptrace_absent")
            return actual, reason, evidence, "ptrace capability removed"
        if capability == "execve_fd":
            original_execve = checker.os.execve

            def unavailable_execve(path: Any, argv: Any, env: Any) -> Any:
                if type(path) is int:
                    raise OSError("injected FD exec unavailable")
                return original_execve(path, argv, env)

            return _patched_probe(checker, executable, [(checker.os, "execve", unavailable_execve)], "os.execve integer-FD failure")
        raise RuntimeError("SCHEMA: unknown removed capability")

    if operation == "fail_syscall":
        syscall = parameters.get("syscall")
        if syscall == "pidfd_open":
            def failed_pidfd(pid: int, flags: int = 0) -> int:
                raise OSError("injected pidfd_open failure")

            return _patched_probe(checker, executable, [(checker.os, "pidfd_open", failed_pidfd)], "os.pidfd_open raises")
        if syscall == "PTRACE_SETOPTIONS":
            original_ptrace = checker.ptrace

            def failed_options(request: int, pid: int, addr: int = 0, data: int = 0) -> None:
                if request == checker.PTRACE_SETOPTIONS:
                    raise checker.Failure("PROBE_AUTHORITY", "injected PTRACE_SETOPTIONS failure")
                original_ptrace(request, pid, addr, data)

            return _patched_probe(checker, executable, [(checker, "ptrace", failed_options)], "ptrace PTRACE_SETOPTIONS raises")
        if syscall == "F_GET_SEALS":
            original_fcntl = checker.fcntl.fcntl

            def failed_seals(fd: int, command: int, *args: Any) -> Any:
                if command == checker.fcntl.F_GET_SEALS:
                    raise OSError("injected F_GET_SEALS failure")
                return original_fcntl(fd, command, *args)

            return _patched_probe(checker, executable, [(checker.fcntl, "fcntl", failed_seals)], "fcntl F_GET_SEALS raises")
        raise RuntimeError("SCHEMA: unknown failed syscall")

    if operation == "substitute_proc_exe_image":
        original_open = checker.os.open

        def substituted_open(path: Any, flags: int, *args: Any) -> int:
            if isinstance(path, str) and path.startswith("/proc/") and path.endswith("/exe"):
                return original_open("/bin/true", flags)
            return original_open(path, flags, *args)

        return _patched_probe(checker, executable, [(checker.os, "open", substituted_open)], "/proc/<pid>/exe open -> /bin/true")

    if operation == "swap_fd_after_exec_stop":
        original_memfd_create = checker.os.memfd_create
        original_write = checker.os.write
        state: dict[str, int] = {}

        def tracked_memfd(name: str, flags: int = 0) -> int:
            fd = original_memfd_create(name, flags)
            if name == "p1347-probe":
                state["probe_fd"] = fd
            return fd

        def swap_before_request(fd: int, data: bytes) -> int:
            if data.startswith(b'{"schema":"p1345-opaque-probe-request-r2"') and "probe_fd" in state:
                evil = original_open("/bin/true", os.O_RDONLY | os.O_CLOEXEC)
                try:
                    os.dup2(evil, state["probe_fd"])
                finally:
                    os.close(evil)
            return original_write(fd, data)

        return _patched_probe(
            checker,
            executable,
            [(checker.os, "memfd_create", tracked_memfd), (checker.os, "write", swap_before_request)],
            "retained probe FD dup-over after exec-stop authentication, before request",
        )

    if operation == "detach_before_terminal_wait":
        original_ptrace = checker.ptrace
        state = {"continues": 0}

        def detach_on_second_continue(request: int, pid: int, addr: int = 0, data: int = 0) -> None:
            if request == checker.PTRACE_CONT:
                state["continues"] += 1
                if state["continues"] == 2:
                    original_ptrace(PTRACE_DETACH, pid, 0, 0)
                    return
            original_ptrace(request, pid, addr, data)

        return _patched_probe(checker, executable, [(checker, "ptrace", detach_on_second_continue)], "PTRACE_DETACH at authenticated exec-stop before terminal wait")

    raise RuntimeError(f"SCHEMA: unmapped probe operation:{operation}")


def _p1346_report(checker: Any, key: tuple[str, ...]) -> dict[str, Any]:
    if key not in _P1346_REPORTS:
        caller = checker.ROOT / "00_nucleo/diagnosticos/p1346-oracle-caller-r2.py"
        stdout, stderr, evidence = checker.parent_exec_fixed(
            [str(checker.PYTHON), "-B", str(caller), "--focus"], b"", timeout=180
        )
        if evidence["child_exit_code"] != 0 or stderr:
            raise checker.Failure("WORKER_AUTHORITY", "P1346 replay caller")
        report = checker.strict_json(stdout, "P1346 focal")
        if type(report) is not dict or type(report.get("attacks")) is not list or type(report.get("controls")) is not list:
            raise checker.Failure("SCHEMA", "P1346 focal report")
        report["_adapter_parent_evidence"] = evidence
        _P1346_REPORTS[key] = report
    return _P1346_REPORTS[key]


def _replay(checker: Any, key: tuple[str, ...], source_case_id: Any) -> tuple[str, str, dict[str, Any], str]:
    if type(source_case_id) is not str:
        raise RuntimeError("SCHEMA: replay source_case_id")
    report = _p1346_report(checker, key)
    matches = [row for row in report["attacks"] if type(row) is dict and row.get("case_id") == source_case_id]
    if len(matches) != 1:
        raise checker.Failure("PROTECTED_INPUT", "P1346 replay identity")
    row = matches[0]
    actual = row.get("actual")
    reason = row.get("reason_code")
    if actual not in {"Preserved", "Violated", "Unknown"} or type(reason) is not str:
        raise checker.Failure("SCHEMA", "P1346 replay result")
    evidence = {
        "p1346_parent_evidence": report["_adapter_parent_evidence"],
        "source_case_id": source_case_id,
        "source_record_sha256": sha256(canonical(row, False)),
    }
    return actual, reason, evidence, "P1346 pinned caller --focus record projection"


def _control(checker: Any, key: tuple[str, ...], case_id: str, operation: str, case: dict[str, Any]) -> tuple[str, str, dict[str, Any], str]:
    if operation == "canonical_worker":
        raw, evidence, challenge, nonce, invocation, root = _canonical_worker(checker, key)
        accepted = checker.validate_worker(raw, copy.deepcopy(evidence), challenge, nonce, invocation, root)
        return "Preserved", "PRESERVED", accepted, "canonical run_worker + validate_worker"
    if operation == "canonical_dag":
        actual, reason, evidence = checker.run_dag(case_id, "canonical")
        return actual, reason, evidence, "canonical run_dag"
    if operation == "real_dag_validator":
        mutation = case.get("mutation")
        if mutation not in {"T08", "T09", "T10", "T11"}:
            raise RuntimeError("SCHEMA: real DAG mutation")
        actual, reason, evidence = checker.run_dag(case_id, mutation.lower())
        return actual, reason, evidence, f"run_dag:{mutation.lower()}"
    if operation == "canonical_traced_probe":
        actual, reason, evidence = checker.trace_probe(_probe_image(checker, key), "canonical")
        return actual, reason, evidence, "canonical traced direct-FD probe"
    if operation == "p1346_control_projection":
        ordinal = case.get("ordinal")
        if type(ordinal) is not int or type(ordinal) is bool or ordinal not in range(4):
            raise RuntimeError("SCHEMA: P1346 control ordinal")
        report = _p1346_report(checker, key)
        row = report["controls"][ordinal]
        actual = "Preserved" if row.get("actual") == row.get("expected", "Preserved") else "Violated"
        reason = "PRESERVED" if actual == "Preserved" else str(row.get("reason_code", "PROTECTED_INPUT"))
        evidence = {
            "p1346_parent_evidence": report["_adapter_parent_evidence"],
            "source_case_id": row.get("case_id"),
            "source_record_sha256": sha256(canonical(row, False)),
        }
        return actual, reason, evidence, f"P1346 control projection ordinal:{ordinal}"
    raise RuntimeError(f"SCHEMA: unmapped control operation:{operation}")


def exercise(case: dict[str, Any], targets: Any, exact_ids: list[str]) -> dict[str, Any]:
    """Execute one frozen adversarial/replay/control case against frozen R2.

    The input mapping is treated as immutable.  No expected classification or
    reason is read.  Every returned object has exactly five top-level fields.
    """
    case_id = case.get("case_id") if type(case) is dict else "<invalid-case>"
    try:
        case_id, operation, parameters = _case(case)
        key = _target_key(targets)
        checker, _caller, binding = _context(targets)
        _exact_ids(checker, exact_ids)

        route = OPERATION_ROUTES.get(operation)
        if route == "control":
            actual, reason, evidence, injection = _control(checker, key, case_id, operation, case)
            api = ["caller.validate_chain", "checker control API"]
        elif route == "replay":
            actual, reason, evidence, injection = _replay(checker, key, parameters.get("source_case_id"))
            api = ["caller.validate_chain", "checker.parent_exec_fixed", "checker.strict_json"]
        elif route == "dag":
            actual, reason, evidence, injection = _dag_operation(checker, case_id, operation)
            api = ["caller.validate_chain", "checker.run_dag"]
        elif route == "probe" or (route == "parameterized_process" and parameters.get("foreign") == "probe_stdout_and_status"):
            actual, reason, evidence, injection = _probe_operation(checker, key, operation, parameters)
            api = ["caller.validate_chain", "checker.compile_probe", "checker.trace_probe"]
        elif route == "worker" or route == "parameterized_process":
            actual, reason, evidence, injection = _worker_operation(checker, key, operation, parameters)
            api = ["caller.validate_chain", "checker.run_worker", "checker.validate_worker"]
        else:
            raise RuntimeError(f"SCHEMA: unmapped adapter operation:{operation}")
        return _result(case_id, actual, reason, binding, api, injection, evidence)
    except BaseException as exc:
        try:
            checker
        except UnboundLocalError:
            class _NoChecker:
                class Failure(Exception):
                    pass
            checker = _NoChecker()
        actual, reason, evidence = _failure(checker, exc)
        fallback_binding = sha256(canonical([str(getattr(targets, "checker_sha256", "")), str(getattr(targets, "caller_sha256", "")), str(getattr(targets, "delivery_receipt_sha256", ""))], False))
        return _result(str(case_id), actual, reason, fallback_binding, ["adapter.exercise"], "fail closed before/within dispatch", evidence)
