#!/usr/bin/env python3
"""P1348 operational verifier adapter, authored but not executed.

The adapter translates frozen adversarial operations into the frozen P1348 R1
caller/checker APIs.  It does not read expected answers and is not a verdict
authority.  Any adapter-side exception produces an integration-blocker outcome
with no classification or reason-code credit.
"""

from __future__ import annotations

import hashlib
import importlib.util
import json
import os
import secrets
import sys
import tempfile
from pathlib import Path
from typing import Any, Callable


RESULT_SCHEMA = "p1348-verifier-adapter-result-r1"
EVIDENCE_SCHEMA = "p1348-verifier-adapter-evidence-r1"
BLOCKER_SCHEMA = "p1348-adapter-exception-blocker-r1"
BLOCKER_VERDICT = "ADAPTER_EXCEPTION_BLOCKS_INTEGRATION"
REGIME = "executado sem atestacao de isolamento"
PTRACE_DETACH = 17

# Closed dispatch table. Parameterized variants retain their exact frozen case
# parameters and are never resolved from expected classifications or reasons.
OPERATION_ROUTES = {
    "adversarial_image": "probe",
    "alter_validator_stream_with_authority_root": "dag",
    "append_worker_stdout": "worker",
    "canonical_dag": "control",
    "canonical_traced_probe": "control",
    "canonical_worker": "control",
    "dag_coordinated_substitution": "dag",
    "dag_labels_without_validator": "dag",
    "dag_mutant_anchor": "dag",
    "dag_mutation_accepted_by_real_validator": "dag",
    "dag_receipt_answer_channel": "dag",
    "dag_valid_schema_self_answer": "dag",
    "dag_validator_without_parent_process": "dag",
    "detach_at_event": "probe",
    "detach_before_event": "probe",
    "detach_before_terminal_wait": "probe",
    "drop_parent_wait": "worker",
    "dup_over_after_child_check": "probe",
    "fabricate_terminal_status": "probe",
    "fail_after_registry_commit": "freshness",
    "fail_proc_exe": "probe",
    "fail_syscall": "probe",
    "fail_then_retry_before_commit": "freshness",
    "inject_registry": "freshness",
    "inject_second_exec": "probe",
    "inject_worker_pid": "worker",
    "mix_process_evidence": "parameterized_process",
    "mutate_id_sequence": "worker",
    "p1346_control_projection": "control",
    "raise_historical_original_open_nameerror": "adapter_meta",
    "real_dag_validator": "control",
    "remove_capability": "probe",
    "remove_trace_option": "probe",
    "replay_p1346_negative": "replay",
    "replay_worker_challenge": "freshness",
    "reuse_terminated_child_pid": "worker",
    "second_exec_after_attestation": "probe",
    "stdout_without_child": "worker",
    "substitute_pid_and_pidfd": "worker",
    "substitute_proc_exe_image": "probe",
    "substitute_process_identity": "worker",
    "suppress_exec_stop": "probe",
    "swap_fd_after_exec_stop": "probe",
    "swap_fd_before_exec": "probe",
    "swap_fd_inside_exec": "probe",
    "swap_validation_slots": "freshness",
    "truncate_worker_stdout": "worker",
    "validate_same_identity_twice": "freshness",
    "worker_exit": "worker",
    "worker_signal": "worker",
    "worker_timeout": "worker",
}

_CONTEXTS: dict[tuple[str, ...], tuple[Any, Any, str]] = {}
_PROBE_IMAGES: dict[tuple[str, ...], bytes] = {}
_P1346_REPORTS: dict[tuple[str, ...], dict[str, Any]] = {}


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
        return json.loads(
            raw.decode("utf-8", "strict"),
            object_pairs_hook=pairs,
            parse_constant=lambda token: (_ for _ in ()).throw(ValueError(token)),
        )
    except (UnicodeError, ValueError, json.JSONDecodeError) as exc:
        raise RuntimeError(f"SCHEMA: {label}") from exc


def _target_key(targets: Any) -> tuple[str, ...]:
    return (
        str(Path(targets.checker).resolve()),
        str(targets.checker_sha256),
        str(Path(targets.caller).resolve()),
        str(targets.caller_sha256),
        str(Path(targets.delivery_receipt).resolve()),
        str(targets.delivery_receipt_sha256),
    )


def _pinned(path: Path, expected: str, label: str) -> bytes:
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
    _pinned(checker_supplied, targets.checker_sha256, "checker")
    _pinned(caller_supplied, targets.caller_sha256, "caller")
    delivery = _strict_json(_pinned(delivery_supplied, targets.delivery_receipt_sha256, "delivery"), "delivery")
    checker_path = checker_supplied.resolve()
    caller_path = caller_supplied.resolve()
    if delivery.get("checker") != ["00_nucleo/diagnosticos/p1348-oracle-checker-r1.py", targets.checker_sha256]:
        raise RuntimeError("AUTHORITY_ROOT: delivered checker")
    if delivery.get("caller") != ["00_nucleo/diagnosticos/p1348-oracle-caller-r1.py", targets.caller_sha256]:
        raise RuntimeError("AUTHORITY_ROOT: delivered caller")
    receipt = delivery.get("oracle_receipt")
    if type(receipt) is not list or len(receipt) != 2 or type(receipt[1]) is not str:
        raise RuntimeError("AUTHORITY_ROOT: delivered authorship receipt")
    caller = _load_module(caller_path, f"p1348_adapter_caller_{targets.caller_sha256[:16]}")
    checker = caller.validate_chain()
    loaded_checker = Path(checker.__file__).resolve()
    if loaded_checker != checker_path or sha256(loaded_checker.read_bytes()) != targets.checker_sha256:
        raise RuntimeError("AUTHORITY_ROOT: caller returned alternate checker")
    computed_delivery = checker.oracle_delivery_root(
        delivery["authoring_root_sha256"], targets.checker_sha256, receipt[1], targets.caller_sha256
    )
    if delivery.get("delivery_root_sha256") != computed_delivery:
        raise RuntimeError("AUTHORITY_ROOT: delivery root")
    binding = sha256(canonical([*key, computed_delivery], False))
    _CONTEXTS[key] = checker, caller, binding
    return _CONTEXTS[key]


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
    return {"repr_sha256": sha256(repr(value).encode()), "type": type(value).__name__}


def _observation(
    classification: str,
    reason_code: str,
    evidence: Any,
    api: list[str],
    injection: str,
    observations: list[list[str]] | None = None,
) -> dict[str, Any]:
    if classification not in {"Preserved", "Violated", "Unknown"} or type(reason_code) is not str:
        raise RuntimeError("adapter observation schema")
    return {
        "api": api,
        "classification": classification,
        "evidence": _jsonable(evidence),
        "injection": injection,
        "observations": observations,
        "reason_code": reason_code,
    }


def _classification_result(case_id: str, binding: str, observed: dict[str, Any]) -> dict[str, Any]:
    evidence = {
        "api": observed["api"],
        "injection": observed["injection"],
        "observation": observed["evidence"],
        "schema": EVIDENCE_SCHEMA,
        "target_binding_sha256": binding,
    }
    return {
        "blocker": None,
        "case_id": case_id,
        "classification": observed["classification"],
        "evidence": evidence,
        "observations": observed["observations"],
        "outcome_kind": "classification",
        "postconditions": None,
        "reason_code": observed["reason_code"],
        "schema": RESULT_SCHEMA,
        "verdict": None,
    }


def _empty_restoration() -> dict[str, Any]:
    return {"attempted": False, "failures": [], "identity_matches": {}, "restored": True}


class AdapterFault(RuntimeError):
    def __init__(
        self,
        phase: str,
        cause: BaseException,
        restoration: dict[str, Any] | None = None,
        postconditions: dict[str, Any] | None = None,
    ) -> None:
        self.phase = phase
        self.cause = cause
        self.restoration = restoration or _empty_restoration()
        self.postconditions = postconditions
        super().__init__(str(cause))


def _blocker_result(case_id: str, targets: Any, fault: AdapterFault) -> dict[str, Any]:
    try:
        adapter_sha = sha256(Path(__file__).read_bytes())
    except Exception:
        adapter_sha = sha256(b"adapter-self-read-failed")
    inputs = {
        "caller_sha256": str(getattr(targets, "caller_sha256", "")),
        "checker_sha256": str(getattr(targets, "checker_sha256", "")),
        "delivery_receipt_sha256": str(getattr(targets, "delivery_receipt_sha256", "")),
    }
    blocker = {
        "adapter_sha256": adapter_sha,
        "case_id": case_id,
        "closed_world": {"rule": "Adapter exception blocks integration and supplies no case classification or reason."},
        "exception_message_sha256": sha256(str(fault.cause).encode()),
        "exception_phase": fault.phase,
        "exception_type": type(fault.cause).__name__,
        "inputs": inputs,
        "regime": REGIME,
        "restoration": fault.restoration,
        "revision": 1,
        "role": "p1348_verifier_adapter",
        "schema": BLOCKER_SCHEMA,
        "step": 1348,
        "verdict": BLOCKER_VERDICT,
    }
    return {
        "blocker": blocker,
        "case_id": case_id,
        "classification": None,
        "evidence": None,
        "observations": None,
        "outcome_kind": "integration_blocker",
        "postconditions": fault.postconditions,
        "reason_code": None,
        "schema": RESULT_SCHEMA,
        "verdict": BLOCKER_VERDICT,
    }


def _patch_call(
    patches: list[tuple[Any, str, Any, Any]],
    invoke: Callable[[], Any],
) -> tuple[Any, dict[str, Any]]:
    installed: list[tuple[Any, str, Any, str]] = []
    primary: tuple[str, BaseException] | None = None
    value: Any = None
    restoration = {"attempted": False, "failures": [], "identity_matches": {}, "restored": False}
    try:
        try:
            for owner, name, original, replacement in patches:
                if getattr(owner, name) is not original:
                    raise RuntimeError(f"patch capture drift:{name}")
                setattr(owner, name, replacement)
                installed.append((owner, name, original, name))
        except BaseException as exc:
            primary = "mutation", exc
        if primary is None:
            try:
                value = invoke()
            except BaseException as exc:
                primary = "invocation", exc
    finally:
        restoration["attempted"] = True
        for owner, name, original, label in reversed(installed):
            try:
                setattr(owner, name, original)
            except BaseException as exc:
                restoration["failures"].append(f"{label}:{type(exc).__name__}")
        for owner, name, original, label in installed:
            try:
                restoration["identity_matches"][label] = getattr(owner, name) is original
            except BaseException as exc:
                restoration["identity_matches"][label] = False
                restoration["failures"].append(f"{label}:identity:{type(exc).__name__}")
        restoration["restored"] = not restoration["failures"] and all(restoration["identity_matches"].values())
    if not restoration["restored"]:
        raise AdapterFault("restoration", RuntimeError("patch restoration failed"), restoration)
    if primary is not None:
        raise AdapterFault(primary[0], primary[1], restoration)
    return value, restoration


def _checker_failure(checker: Any, exc: BaseException, evidence: Any = None) -> dict[str, Any]:
    if not isinstance(exc, checker.Failure):
        raise exc
    return _observation(
        "Violated", exc.code, {"failure_detail": exc.detail, "state": evidence},
        ["caller.validate_chain", "checker failure"], "checker-observed failure",
    )


def _slot(case_id: str, order: str, suffix: str = "one") -> str:
    if order not in {"normal", "repeat", "reverse"}:
        raise RuntimeError("invalid execution order")
    slot = f"{order}:{case_id}:{suffix}"
    if not slot.isascii():
        raise RuntimeError("non-ASCII schedule slot")
    return slot


def _cleanup_run(run: Any) -> None:
    if run is None:
        return
    try:
        run.cleanup()
    except BaseException as exc:
        raise AdapterFault("cleanup", exc)


def _worker_once(
    checker: Any,
    case_id: str,
    order: str,
    operation: str = "canonical",
    mutate: Callable[[Any], None] | None = None,
    challenge: bytes | None = None,
    nonce: bytes | None = None,
) -> dict[str, Any]:
    registry = checker.ParentFreshnessRegistry([_slot(case_id, order)])
    run = None
    try:
        try:
            run = checker.start_worker(registry, _slot(case_id, order), operation, challenge, nonce)
            if mutate is not None:
                mutate(run)
            evidence = checker.validate_worker(run)
            return _observation(
                "Preserved", "PRESERVED", evidence,
                ["caller.validate_chain", "checker.start_worker", "checker.validate_worker"],
                f"worker operation:{operation}",
            )
        except BaseException as exc:
            return _checker_failure(checker, exc, registry.state())
    finally:
        _cleanup_run(run)


def _worker_operation(
    checker: Any, case_id: str, order: str, operation: str, parameters: dict[str, Any]
) -> dict[str, Any]:
    if operation == "stdout_without_child":
        try:
            checker.validate_worker(None)
            return _observation("Preserved", "PRESERVED", {}, ["checker.validate_worker"], "missing worker handle")
        except BaseException as exc:
            return _checker_failure(checker, exc)

    if operation == "mutate_id_sequence":
        mapped = {
            "omit": "omit_id", "extra": "extra_id", "duplicate": "duplicate_id",
            "swap": "reorder_id", "unicode_confusable": "unicode_id",
        }.get(parameters.get("mutation"))
        if mapped is None:
            raise RuntimeError("unknown ID-sequence mutation")
        return _worker_once(checker, case_id, order, mapped)

    if operation == "inject_worker_pid":
        value = parameters.get("value")
        if type(value) is bool:
            mapped = "pid_bool"
        elif type(value) is int and value == 0:
            mapped = "pid_zero"
        elif type(value) is int and value < 0:
            mapped = "pid_negative"
        elif type(value) is str:
            mapped = "pid_string"
        else:
            raise RuntimeError("unknown payload PID mutation")
        return _worker_once(checker, case_id, order, mapped)

    if operation in {"substitute_process_identity", "substitute_pid_and_pidfd"}:
        def mutate(run: Any) -> None:
            run.transaction._record["child_pid"] = os.getpid()
            if operation == "substitute_pid_and_pidfd":
                identity = dict(run.transaction._record["pidfd_identity"])
                identity["st_ino"] = int(identity["st_ino"]) + 1
                run.transaction._record["pidfd_identity"] = identity

        return _worker_once(checker, case_id, order, "canonical", mutate)

    if operation == "reuse_terminated_child_pid":
        prior_registry = checker.ParentFreshnessRegistry([_slot(case_id, order, "prior")])
        current_registry = checker.ParentFreshnessRegistry([_slot(case_id, order, "current")])
        prior = current = None
        try:
            prior = checker.start_worker(prior_registry, _slot(case_id, order, "prior"))
            checker.validate_worker(prior)
            stale = prior.transaction.session.verify(prior.transaction)
            current = checker.start_worker(current_registry, _slot(case_id, order, "current"))
            current.transaction._record["child_pid"] = stale["child_pid"]
            current.transaction._record["pidfd_identity"] = stale["pidfd_identity"]
            try:
                checker.validate_worker(current)
                return _observation("Preserved", "PRESERVED", {}, ["checker.validate_worker"], "terminated child PID/pidfd substitution")
            except BaseException as exc:
                return _checker_failure(checker, exc, current_registry.state())
        finally:
            cleanup_fault = None
            for run in (current, prior):
                try:
                    _cleanup_run(run)
                except AdapterFault as exc:
                    cleanup_fault = cleanup_fault or exc
            if cleanup_fault is not None:
                raise cleanup_fault

    if operation == "drop_parent_wait":
        def mutate(run: Any) -> None:
            run.transaction._record["raw_wait_status"] = None

        return _worker_once(checker, case_id, order, "canonical", mutate)

    if operation == "mix_process_evidence":
        foreign = parameters.get("foreign")
        if foreign == "stdout":
            return _worker_once(checker, case_id, order, "canonical", lambda run: setattr(run, "raw", b"foreign\n"))
        if foreign == "stderr":
            return _worker_once(checker, case_id, order, "canonical", lambda run: setattr(run, "stderr", b"foreign\n"))
        if foreign == "wait_status":
            def mutate(run: Any) -> None:
                run.transaction._record["raw_wait_status"] ^= 0x100
            return _worker_once(checker, case_id, order, "canonical", mutate)
        raise RuntimeError("unknown worker process-evidence source")

    if operation == "truncate_worker_stdout":
        return _worker_once(checker, case_id, order, "partial_stdout")
    if operation == "append_worker_stdout":
        return _worker_once(checker, case_id, order, "extra_stdout")
    if operation in {"worker_timeout", "worker_signal", "worker_exit"}:
        mapped = {"worker_timeout": "timeout", "worker_signal": "signal", "worker_exit": "unexpected_exit"}[operation]
        return _worker_once(checker, case_id, order, mapped)
    raise RuntimeError(f"unmapped worker operation:{operation}")


def _double_validation(checker: Any, case_id: str, order: str) -> dict[str, Any]:
    slot = _slot(case_id, order)
    registry = checker.ParentFreshnessRegistry([slot])
    run = None
    observations: list[list[str]] = []
    details: list[Any] = []
    try:
        run = checker.start_worker(registry, slot)
        for _ in range(2):
            try:
                details.append(checker.validate_worker(run))
                observations.append(["Preserved", "PRESERVED"])
            except BaseException as exc:
                if not isinstance(exc, checker.Failure):
                    raise
                observations.append(["Violated", exc.code])
                details.append({"failure_detail": exc.detail})
        classification, reason = observations[-1]
        return _observation(
            classification, reason, {"registry": registry.state(), "steps": details},
            ["checker.start_worker", "checker.validate_worker", "checker.validate_worker"],
            "same PendingWorkerRun validated twice in one registry", observations,
        )
    finally:
        _cleanup_run(run)


def _fail_then_retry(checker: Any, case_id: str, order: str) -> dict[str, Any]:
    slot = _slot(case_id, order)
    registry = checker.ParentFreshnessRegistry([slot])
    challenge, nonce = secrets.token_bytes(32), secrets.token_bytes(32)
    while challenge == nonce:
        nonce = secrets.token_bytes(32)
    first = retry = None
    observations: list[list[str]] = []
    details: list[Any] = []
    try:
        first = checker.start_worker(registry, slot, "partial_stdout", challenge, nonce)
        try:
            checker.validate_worker(first)
            observations.append(["Preserved", "PRESERVED"])
        except BaseException as exc:
            if not isinstance(exc, checker.Failure):
                raise
            observations.append(["Violated", exc.code])
            details.append({"first_failure": exc.detail, "state_after_failure": registry.state()})
        _cleanup_run(first)
        first = None
        retry = checker.start_worker(registry, slot, "canonical", challenge, nonce)
        try:
            details.append(checker.validate_worker(retry))
            observations.append(["Preserved", "PRESERVED"])
        except BaseException as exc:
            if not isinstance(exc, checker.Failure):
                raise
            observations.append(["Violated", exc.code])
            details.append({"retry_failure": exc.detail})
        return _observation(
            observations[0][0], observations[0][1], {"registry": registry.state(), "steps": details},
            ["checker.start_worker", "checker.validate_worker", "checker.start_worker", "checker.validate_worker"],
            "precommit framing failure, rollback, same-identity retry", observations,
        )
    finally:
        cleanup_fault = None
        for run in (retry, first):
            try:
                _cleanup_run(run)
            except AdapterFault as exc:
                cleanup_fault = cleanup_fault or exc
        if cleanup_fault is not None:
            raise cleanup_fault


def _postcommit_blocker(checker: Any, case_id: str, order: str) -> None:
    slot = _slot(case_id, order)
    registry = checker.ParentFreshnessRegistry([slot])
    run = None
    try:
        run = checker.start_worker(registry, slot)
        checker.validate_worker(run)
        try:
            registry.fail_after_commit(run.identity)
        except checker.IntegrationBlocker as exc:
            reopened = False
            try:
                replay = checker.start_worker(registry, slot, "canonical", run.challenge, run.nonce)
            except checker.Failure:
                reopened = False
            else:
                reopened = True
                _cleanup_run(replay)
            state = registry.state()
            consumed = any(item["identity_sha256"] == run.identity for item in state["consumed"])
            raise AdapterFault(
                "postcommit", exc, _empty_restoration(),
                {"identity": "CONSUMED" if consumed else "NOT_CONSUMED", "reopened": reopened},
            )
    finally:
        _cleanup_run(run)


def _inject_registry(checker: Any, case_id: str, order: str) -> dict[str, Any]:
    observations: list[list[str]] = []
    details: list[Any] = []
    payload = _worker_once(
        checker, case_id + "-payload", order, "canonical",
        lambda run: setattr(run, "raw", run.raw[:-2] + b',"registry":{}}\n'),
    )
    observations.append([payload["classification"], payload["reason_code"]])
    details.append(payload["evidence"])

    slot = _slot(case_id, order, "adapter")
    registry = checker.ParentFreshnessRegistry([slot])
    run = None
    try:
        run = checker.start_worker(registry, slot)
        run.registry = checker.ParentFreshnessRegistry([slot])
        try:
            checker.validate_worker(run)
            observations.append(["Preserved", "PRESERVED"])
        except BaseException as exc:
            if not isinstance(exc, checker.Failure):
                raise
            observations.append(["Violated", exc.code])
            details.append({"adapter_registry_failure": exc.detail})
    finally:
        _cleanup_run(run)
    classification, reason = observations[0]
    return _observation(
        classification, reason, details,
        ["checker.validate_worker payload registry", "checker.validate_worker substituted registry"],
        "payload and adapter registry substitution", observations,
    )


def _freshness_operation(
    checker: Any, case_id: str, order: str, operation: str
) -> dict[str, Any]:
    if operation in {"replay_worker_challenge", "validate_same_identity_twice"}:
        return _double_validation(checker, case_id, order)
    if operation == "swap_validation_slots":
        first, second = _slot(case_id, order, "first"), _slot(case_id, order, "second")
        registry = checker.ParentFreshnessRegistry([first, second])
        try:
            checker.start_worker(registry, second)
            return _observation("Preserved", "PRESERVED", registry.state(), ["checker.start_worker"], "second schedule slot before first")
        except BaseException as exc:
            return _checker_failure(checker, exc, registry.state())
    if operation == "fail_then_retry_before_commit":
        return _fail_then_retry(checker, case_id, order)
    if operation == "fail_after_registry_commit":
        _postcommit_blocker(checker, case_id, order)
        raise RuntimeError("postcommit blocker not raised")
    if operation == "inject_registry":
        return _inject_registry(checker, case_id, order)
    raise RuntimeError(f"unmapped freshness operation:{operation}")


def _dag_operation(
    checker: Any, case_id: str, operation: str, parameters: dict[str, Any]
) -> dict[str, Any]:
    mapped = {
        "dag_labels_without_validator": ("dag_self_labels", True),
        "dag_mutation_accepted_by_real_validator": ("extra_file", True),
        "dag_mutant_anchor": ("mutant_anchor", True),
        "dag_validator_without_parent_process": ("validator_unobserved", False),
        "dag_receipt_answer_channel": ("receipt_answer_channel", True),
        "dag_coordinated_substitution": ("mutant_anchor", True),
        "dag_valid_schema_self_answer": ("dag_self_labels", True),
    }
    if operation == "alter_validator_stream_with_authority_root":
        original_capture = checker.capture_process

        def altered_capture(*args: Any, **kwargs: Any) -> Any:
            transaction, stdout, stderr = original_capture(*args, **kwargs)
            schedule_slot = args[2] if len(args) > 2 else kwargs.get("schedule_slot")
            if schedule_slot == "dag-validator":
                stdout += b"foreign"
                stderr += b"foreign"
            return transaction, stdout, stderr

        value, restoration = _patch_call(
            [(checker, "capture_process", original_capture, altered_capture)],
            lambda: checker.run_dag(case_id, "mutant_anchor"),
        )
        actual, reason, evidence = value
        evidence = {"checker": evidence, "restoration": restoration}
        return _observation(actual, reason, evidence, ["checker.run_dag"], "validator stdout/stderr altered after capture")
    if operation not in mapped:
        raise RuntimeError(f"unmapped DAG operation:{operation}")
    worker_operation, observe = mapped[operation]
    actual, reason, evidence = checker.run_dag(case_id, worker_operation, observe)
    return _observation(
        actual, reason, evidence, ["caller.validate_chain", "checker.run_dag"],
        f"DAG worker:{worker_operation}; validator observed:{observe}",
    )


def _probe_image(checker: Any, key: tuple[str, ...]) -> bytes:
    if key not in _PROBE_IMAGES:
        with tempfile.TemporaryDirectory(prefix="p1348-adapter-build-", dir="/dev/shm") as temporary:
            _PROBE_IMAGES[key] = checker.BASE.compile_probe(Path(temporary))
    return _PROBE_IMAGES[key]


def _probe_observation(value: tuple[str, str, dict[str, Any]], injection: str, restoration: Any = None) -> dict[str, Any]:
    actual, reason, evidence = value
    if restoration is not None:
        evidence = {"checker": evidence, "restoration": restoration}
    return _observation(actual, reason, evidence, ["caller.validate_chain", "checker.trace_probe"], injection)


def _probe_patched(
    checker: Any,
    executable: bytes,
    patches: list[tuple[Any, str, Any, Any]],
    injection: str,
) -> dict[str, Any]:
    value, restoration = _patch_call(patches, lambda: checker.trace_probe(executable, "canonical"))
    return _probe_observation(value, injection, restoration)


def _exec_substitution(checker: Any, executable: bytes, injection: str, fail: bool = False) -> dict[str, Any]:
    original_execve = checker.os.execve
    original_open = checker.os.open

    def substituted_execve(path: Any, argv: Any, env: Any) -> Any:
        if type(path) is int:
            if fail:
                raise OSError("injected integer-FD exec failure")
            evil = original_open("/bin/true", os.O_RDONLY | os.O_CLOEXEC)
            try:
                return original_execve(evil, argv, env)
            finally:
                os.close(evil)
        return original_execve(path, argv, env)

    return _probe_patched(
        checker, executable,
        [(checker.os, "execve", original_execve, substituted_execve)], injection,
    )


def _swap_before_exec(checker: Any, executable: bytes, injection: str) -> dict[str, Any]:
    original_memfd_create = checker.os.memfd_create
    original_ptrace = checker.ptrace
    original_open = checker.os.open
    state: dict[str, int] = {}

    def tracked_memfd(name: str, flags: int = 0) -> int:
        fd = original_memfd_create(name, flags)
        if name == "p1348-probe":
            state["probe_fd"] = fd
        return fd

    def swap_at_traceme(request: int, pid: int, data: Any = 0) -> None:
        original_ptrace(request, pid, data)
        if request == checker.PTRACE_TRACEME and "probe_fd" in state:
            evil = original_open("/bin/true", os.O_RDONLY | os.O_CLOEXEC)
            try:
                os.dup2(evil, state["probe_fd"])
            finally:
                os.close(evil)

    return _probe_patched(
        checker, executable,
        [(checker.os, "memfd_create", original_memfd_create, tracked_memfd),
         (checker, "ptrace", original_ptrace, swap_at_traceme)],
        injection,
    )


def _swap_after_exec_stop(checker: Any, executable: bytes) -> dict[str, Any]:
    # Historical NameError repair: open is captured in this exact operation
    # scope, installed with the other patches, and restored/identity-checked by
    # the same finally boundary before this function returns.
    original_open = checker.os.open
    original_memfd_create = checker.os.memfd_create
    original_write = checker.os.write
    state: dict[str, int] = {}

    def passthrough_open(path: Any, flags: int, *args: Any) -> int:
        return original_open(path, flags, *args)

    def tracked_memfd(name: str, flags: int = 0) -> int:
        fd = original_memfd_create(name, flags)
        if name == "p1348-probe":
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

    return _probe_patched(
        checker, executable,
        [(checker.os, "open", original_open, passthrough_open),
         (checker.os, "memfd_create", original_memfd_create, tracked_memfd),
         (checker.os, "write", original_write, swap_before_request)],
        "retained probe FD dup-over after exec-stop authentication and before request",
    )


def _detach_before_exit(checker: Any, executable: bytes, injection: str) -> dict[str, Any]:
    original_ptrace = checker.ptrace
    state = {"continues": 0}

    def detach_on_second_continue(request: int, pid: int, data: Any = 0) -> None:
        if request == checker.PTRACE_CONT:
            state["continues"] += 1
            if state["continues"] == 2:
                original_ptrace(PTRACE_DETACH, pid, 0)
                return
        original_ptrace(request, pid, data)

    return _probe_patched(
        checker, executable,
        [(checker, "ptrace", original_ptrace, detach_on_second_continue)], injection,
    )


def _detach_at_exec_event(checker: Any, executable: bytes) -> dict[str, Any]:
    original_open = checker.os.open
    original_ptrace = checker.ptrace

    def detach_then_open(path: Any, flags: int, *args: Any) -> int:
        if isinstance(path, str) and path.startswith("/proc/") and path.endswith("/exe"):
            pid = int(path.split("/")[2])
            original_ptrace(PTRACE_DETACH, pid, 0)
        return original_open(path, flags, *args)

    return _probe_patched(
        checker, executable,
        [(checker.os, "open", original_open, detach_then_open)],
        "PTRACE_DETACH while stopped at authenticated exec event",
    )


def _probe_operation(
    checker: Any, key: tuple[str, ...], operation: str, parameters: dict[str, Any]
) -> dict[str, Any]:
    executable = _probe_image(checker, key)
    if operation in {"swap_fd_before_exec", "dup_over_after_child_check"}:
        return _swap_before_exec(checker, executable, operation)
    if operation in {"swap_fd_inside_exec", "adversarial_image"}:
        return _exec_substitution(checker, executable, operation)
    if operation == "swap_fd_after_exec_stop":
        return _swap_after_exec_stop(checker, executable)
    if operation in {"second_exec_after_attestation", "inject_second_exec"}:
        return _probe_observation(checker.trace_probe(executable, "second_exec"), "checker second-exec hook")
    if operation in {"detach_before_terminal_wait", "detach_before_event"}:
        return _detach_before_exit(checker, executable, operation)
    if operation == "detach_at_event":
        return _detach_at_exec_event(checker, executable)
    if operation == "remove_trace_option":
        return _probe_observation(checker.trace_probe(executable, "missing_traceexit"), "remove PTRACE_O_TRACEEXIT")

    if operation == "remove_capability":
        capability = parameters.get("capability")
        if capability == "pidfd_open":
            original = checker.os.pidfd_open
            def unavailable_pidfd(pid: int, flags: int = 0) -> int:
                raise OSError("injected pidfd unavailable")
            return _probe_patched(checker, executable, [(checker.os, "pidfd_open", original, unavailable_pidfd)], "remove pidfd_open")
        if capability == "ptrace":
            original = checker.ptrace
            def unavailable_ptrace(request: int, pid: int, data: Any = 0) -> None:
                raise checker.Failure("PROBE_AUTHORITY", "injected ptrace unavailable")
            return _probe_patched(checker, executable, [(checker, "ptrace", original, unavailable_ptrace)], "remove ptrace")
        if capability == "execve_fd":
            return _exec_substitution(checker, executable, "remove integer-FD exec", True)
        raise RuntimeError("unknown removed probe capability")

    if operation == "fail_syscall":
        syscall = parameters.get("syscall")
        if syscall == "pidfd_open":
            original = checker.os.pidfd_open
            def failed_pidfd(pid: int, flags: int = 0) -> int:
                raise OSError("injected pidfd_open failure")
            return _probe_patched(checker, executable, [(checker.os, "pidfd_open", original, failed_pidfd)], "pidfd_open failure")
        if syscall == "PTRACE_SETOPTIONS":
            original = checker.ptrace
            def failed_options(request: int, pid: int, data: Any = 0) -> None:
                if request == checker.PTRACE_SETOPTIONS:
                    raise checker.Failure("PROBE_AUTHORITY", "injected PTRACE_SETOPTIONS failure")
                original(request, pid, data)
            return _probe_patched(checker, executable, [(checker, "ptrace", original, failed_options)], "PTRACE_SETOPTIONS failure")
        if syscall == "F_GET_SEALS":
            original = checker.fcntl.fcntl
            def failed_seals(fd: int, command: int, *args: Any) -> Any:
                if command == checker.fcntl.F_GET_SEALS:
                    raise OSError("injected F_GET_SEALS failure")
                return original(fd, command, *args)
            return _probe_patched(checker, executable, [(checker.fcntl, "fcntl", original, failed_seals)], "F_GET_SEALS failure")
        raise RuntimeError("unknown failed syscall")

    if operation == "suppress_exec_stop":
        return _exec_substitution(checker, executable, "child exits before exec event", True)
    if operation == "fail_proc_exe":
        original = checker.os.open
        def failed_open(path: Any, flags: int, *args: Any) -> int:
            if isinstance(path, str) and path.startswith("/proc/") and path.endswith("/exe"):
                raise OSError("injected proc image unreadable")
            return original(path, flags, *args)
        return _probe_patched(checker, executable, [(checker.os, "open", original, failed_open)], "/proc/<pid>/exe unreadable")
    if operation == "substitute_proc_exe_image":
        original = checker.os.open
        def substituted_open(path: Any, flags: int, *args: Any) -> int:
            if isinstance(path, str) and path.startswith("/proc/") and path.endswith("/exe"):
                return original("/bin/true", flags)
            return original(path, flags, *args)
        return _probe_patched(checker, executable, [(checker.os, "open", original, substituted_open)], "/proc/<pid>/exe -> /bin/true")
    if operation in {"mix_process_evidence", "fabricate_terminal_status"}:
        original = checker.os.waitpid
        state = {"calls": 0}
        def foreign_wait(pid: int, options: int) -> tuple[int, int]:
            state["calls"] += 1
            if state["calls"] == 3:
                return (pid + 1, 0) if operation == "mix_process_evidence" else (pid, 0)
            return original(pid, options)
        return _probe_patched(checker, executable, [(checker.os, "waitpid", original, foreign_wait)], operation)
    raise RuntimeError(f"unmapped probe operation:{operation}")


def _adapter_meta_operation(checker: Any) -> None:
    original_open = checker.os.open
    def passthrough_open(path: Any, flags: int, *args: Any) -> int:
        return original_open(path, flags, *args)
    _unused, restoration = _patch_call(
        [(checker.os, "open", original_open, passthrough_open)],
        lambda: None,
    )
    # This frozen case injects the historical adapter defect itself.  The
    # harmless open swap has already crossed the same finally boundary, so the
    # blocker can prove restoration instead of merely claiming it.
    raise AdapterFault(
        "mutation",
        NameError("historical original_open was undefined"),
        restoration,
    )


def _p1346_report(checker: Any, key: tuple[str, ...]) -> dict[str, Any]:
    if key not in _P1346_REPORTS:
        session = checker.ParentSession()
        caller = checker.ROOT / "00_nucleo/diagnosticos/p1346-oracle-caller-r2.py"
        transaction, stdout, stderr = checker.capture_process(
            session, secrets.token_hex(32), "p1346-replay-caller",
            [str(checker.PYTHON), "-B", str(caller), "--focus"], b"", timeout=180,
        )
        checker.validate_transaction(transaction, stdout, stderr)
        report = checker.BASE.strict_json(stdout, "P1346 focal")
        if type(report) is not dict or type(report.get("attacks")) is not list or type(report.get("controls")) is not list:
            raise RuntimeError("P1346 focal report schema")
        report["_adapter_transaction_id"] = transaction.transaction_id
        _P1346_REPORTS[key] = report
    return _P1346_REPORTS[key]


def _replay(checker: Any, key: tuple[str, ...], source_case_id: Any) -> dict[str, Any]:
    if type(source_case_id) is not str:
        raise RuntimeError("replay source_case_id schema")
    report = _p1346_report(checker, key)
    matches = [row for row in report["attacks"] if type(row) is dict and row.get("case_id") == source_case_id]
    if len(matches) != 1:
        raise RuntimeError("P1346 replay identity")
    row = matches[0]
    actual, reason = row.get("actual"), row.get("reason_code")
    if actual not in {"Preserved", "Violated", "Unknown"} or type(reason) is not str:
        raise RuntimeError("P1346 replay observation schema")
    evidence = {
        "caller_transaction_id": report["_adapter_transaction_id"],
        "source_case_id": source_case_id,
        "source_record_sha256": sha256(canonical(row, False)),
    }
    return _observation(actual, reason, evidence, ["checker.capture_process", "P1346 caller --focus"], "exact source-case projection")


def _control(
    checker: Any, key: tuple[str, ...], case_id: str, order: str, operation: str, case: dict[str, Any]
) -> dict[str, Any]:
    if operation == "canonical_worker":
        return _worker_once(checker, case_id, order)
    if operation == "canonical_dag":
        actual, reason, evidence = checker.run_dag(case_id, "canonical")
        return _observation(actual, reason, evidence, ["checker.run_dag"], "canonical DAG")
    if operation == "real_dag_validator":
        mutation = case.get("mutation")
        if mutation not in {"T08", "T09", "T10", "T11"}:
            raise RuntimeError("real DAG mutation schema")
        actual, reason, evidence = checker.run_dag(case_id, mutation.lower())
        return _observation(actual, reason, evidence, ["checker.run_dag"], f"real validator:{mutation.lower()}")
    if operation == "canonical_traced_probe":
        return _probe_observation(checker.trace_probe(_probe_image(checker, key), "canonical"), "canonical EXEC-to-EXIT probe")
    if operation == "p1346_control_projection":
        ordinal = case.get("ordinal")
        if type(ordinal) is not int or type(ordinal) is bool or ordinal not in range(4):
            raise RuntimeError("P1346 control ordinal schema")
        report = _p1346_report(checker, key)
        row = report["controls"][ordinal]
        actual = "Preserved" if row.get("actual") == row.get("expected", "Preserved") else "Violated"
        reason = "PRESERVED" if actual == "Preserved" else str(row.get("reason_code", "PROTECTED_INPUT"))
        evidence = {"source_case_id": row.get("case_id"), "source_record_sha256": sha256(canonical(row, False))}
        return _observation(actual, reason, evidence, ["P1346 caller --focus"], f"control projection:{ordinal}")
    raise RuntimeError(f"unmapped control operation:{operation}")


def exercise(case: dict[str, Any], targets: Any, order: str) -> dict[str, Any]:
    """Execute one frozen case, or return a classification-free blocker."""
    case_id = case.get("case_id") if type(case) is dict else "<invalid-case>"
    phase = "import"
    try:
        case_id, operation, parameters = _case(case)
        phase = "setup"
        key = _target_key(targets)
        checker, _caller, binding = _context(targets)
        if order not in {"normal", "repeat", "reverse"}:
            raise RuntimeError("invalid order")
        route = OPERATION_ROUTES.get(operation)
        phase = "mutation"
        if route == "control":
            observed = _control(checker, key, case_id, order, operation, case)
        elif route == "replay":
            observed = _replay(checker, key, parameters.get("source_case_id"))
        elif route == "dag":
            observed = _dag_operation(checker, case_id, operation, parameters)
        elif route == "probe" or (route == "parameterized_process" and parameters.get("foreign") == "probe_stdout_and_status"):
            observed = _probe_operation(checker, key, operation, parameters)
        elif route == "worker" or route == "parameterized_process":
            observed = _worker_operation(checker, case_id, order, operation, parameters)
        elif route == "freshness":
            observed = _freshness_operation(checker, case_id, order, operation)
        elif route == "adapter_meta":
            _adapter_meta_operation(checker)
            raise RuntimeError("adapter meta operation returned")
        else:
            raise RuntimeError(f"unmapped adapter operation:{operation}")
        phase = "result"
        return _classification_result(case_id, binding, observed)
    except AdapterFault as fault:
        return _blocker_result(str(case_id), targets, fault)
    except BaseException as exc:
        return _blocker_result(str(case_id), targets, AdapterFault(phase, exc))
