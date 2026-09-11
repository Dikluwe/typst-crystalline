#!/usr/bin/env python3
"""P1351 boundary compatibility projection.

This module is deliberately a projection layer.  It never fabricates process
identity or cleanup evidence: the R3 supervisor/harness remain the authority
for those observations.  ``path_kind`` describes the public path while
``identity_transport`` records the wait/identity transport used by it.
"""

from __future__ import annotations

import hashlib
import importlib.util
import os
import ast
import secrets
import tempfile
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
SUPERVISOR_REL = "00_nucleo/diagnosticos/p1350-oracle-supervisor-r3.py"
HARNESS_REL = "00_nucleo/diagnosticos/p1350-oracle-boundary-harness-r3.py"
SUPERVISOR_SHA256 = "1d662c6fa98f0b00fdf8a56c4112c9be1863469a1b1d2eb1de1fdf3d8bb032c6"
HARNESS_SHA256 = "7de050f2f86612306e4dac934fee8f4f2730a98f75103551233a9139f8e6500e"
PATH_KINDS = ("EXECUTOR", "PUBLIC_REFUSAL")
IDENTITY_TRANSPORTS = ("PIDFD", "WAITPID_ENOSYS", None)
EXECUTOR_KEYS = ("path_kind", "identity_transport", "pgid", "pid", "sid", "starttime_ticks", "terminal_wait_observed")


class CompatibilityFailure(ValueError):
    """Invalid or contradictory boundary evidence."""


def _load(relative: str, expected: str, name: str) -> Any:
    path = ROOT / relative
    raw = path.read_bytes()
    if path.is_symlink() or hashlib.sha256(raw).hexdigest() != expected:
        raise RuntimeError(f"AUTHORITY_ROOT: drift {relative}")
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"AUTHORITY_ROOT: loader {relative}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _identity(value: Any) -> bool:
    return type(value) is int and value > 0


def validate_executor_axes(executor: Any, cleanup: Any) -> dict[str, Any]:
    """Validate and project executor evidence without adding observations."""
    if type(executor) is not dict or type(cleanup) is not dict:
        raise CompatibilityFailure("executor/cleanup evidence must be objects")
    if set(executor) not in (set(EXECUTOR_KEYS), {"path_kind", "pgid", "pid", "sid", "starttime_ticks", "terminal_wait_observed"}):
        raise CompatibilityFailure("unexpected executor keys")
    explicit_axes = "identity_transport" in executor
    transport = executor.get("identity_transport")
    old_transport = executor.get("path_kind")
    if explicit_axes and old_transport not in PATH_KINDS:
        raise CompatibilityFailure("transport value placed on path_kind axis")
    if transport is None:
        transport = old_transport if old_transport in ("PIDFD", "WAITPID_ENOSYS") else None
    elif transport not in ("PIDFD", "WAITPID_ENOSYS"):
        raise CompatibilityFailure("invalid identity_transport")
    if old_transport not in ("PIDFD", "WAITPID_ENOSYS", "EXECUTOR", "PUBLIC_REFUSAL"):
        raise CompatibilityFailure("invalid path_kind")
    identity_present = all(_identity(executor.get(key)) for key in ("pid", "pgid", "sid", "starttime_ticks"))
    terminal = executor.get("terminal_wait_observed") is True
    clean = all(cleanup.get(key) is True for key in ("descriptors_closed", "group_absent", "patches_restored", "registry_destroyed", "shm_removed", "zombies_absent"))
    if identity_present and terminal and clean:
        if transport not in ("PIDFD", "WAITPID_ENOSYS"):
            raise CompatibilityFailure("executor requires an identity transport")
        return {"path_kind": "EXECUTOR", "identity_transport": transport, **{key: executor[key] for key in EXECUTOR_KEYS[2:]}}
    if any(executor.get(key) is not None for key in ("pid", "pgid", "sid", "starttime_ticks")):
        raise CompatibilityFailure("partial or invalid executor identity")
    if transport is not None or executor.get("terminal_wait_observed") is not True:
        raise CompatibilityFailure("refusal cannot carry executor transport")
    return {"path_kind": "PUBLIC_REFUSAL", "identity_transport": None, "pgid": None, "pid": None, "sid": None, "starttime_ticks": None, "terminal_wait_observed": True}


def validate_and_project(result: Any) -> dict[str, Any]:
    """Validate a P1350 result and return the corrected public projection."""
    if type(result) is not dict or type(result.get("dynamic_evidence")) is not dict:
        raise CompatibilityFailure("boundary result shape")
    dynamic = result["dynamic_evidence"]
    if type(dynamic.get("cleanup")) is not dict or type(dynamic.get("executor")) is not dict:
        raise CompatibilityFailure("dynamic evidence shape")
    projected_executor = validate_executor_axes(dynamic["executor"], dynamic["cleanup"])
    projected = dict(result)
    projected_dynamic = dict(dynamic)
    projected_dynamic["executor"] = projected_executor
    projected["dynamic_evidence"] = projected_dynamic
    return projected


def project_boundary_result(result: Any) -> dict[str, Any]:
    return validate_and_project(result)


def project_executor(raw: Any) -> dict[str, Any]:
    """Project the public executor axes from a boundary-shaped mapping."""
    if type(raw) is not dict:
        raise CompatibilityFailure("executor input must be a mapping")
    cleanup = raw.get("cleanup", {key: True for key in ("descriptors_closed", "group_absent", "patches_restored", "registry_destroyed", "shm_removed", "zombies_absent")})
    return validate_executor_axes(raw, cleanup)


def project_boundary(raw: Any) -> dict[str, Any]:
    """Produce the closed P1351 result while retaining bound evidence."""
    if type(raw) is not dict:
        raise CompatibilityFailure("boundary input must be a mapping")
    executor = project_executor(raw.get("executor", raw))
    cleanup = raw.get("cleanup")
    journal = raw.get("journal")
    if type(cleanup) is not dict or any(cleanup.get(key) is not True for key in ("group_absent", "zombies_absent", "descriptors_closed", "shm_removed")):
        raise CompatibilityFailure("cleanup evidence incomplete")
    if type(journal) is not dict or journal.get("case_closed") is not True or journal.get("group_absent_before_result") is not True:
        raise CompatibilityFailure("journal does not prove terminal group absence")
    if type(raw.get("nonce")) is not str or type(raw.get("challenge")) is not str or type(raw.get("meta_evidence")) is not dict:
        raise CompatibilityFailure("binding evidence missing")
    result = dict(raw)
    result.update({"schema": "p1351-boundary-result-r1", "path_kind": executor["path_kind"], "identity_transport": executor["identity_transport"], "executor": executor})
    return result


def project_meta(raw: Any) -> dict[str, Any]:
    if type(raw) is not dict or type(raw.get("meta_evidence")) is not dict:
        raise CompatibilityFailure("meta evidence missing")
    return {"meta_evidence": dict(raw["meta_evidence"]), "nonce": raw.get("nonce"), "challenge": raw.get("challenge")}


def supervise_real_case() -> dict[str, Any]:
    raw = run_supervised_case()
    dynamic = raw["dynamic_evidence"]
    executor = project_executor(dynamic["executor"])
    nonce = raw.pop("_nonce")
    challenge = raw.pop("_challenge")
    cleanup = dynamic["cleanup"]
    result = {"schema": "p1351-boundary-result-r1", "path_kind": executor["path_kind"], "identity_transport": executor["identity_transport"], "executor": executor,
              "nonce": nonce, "challenge": challenge, "meta_evidence": {"nonce": nonce, "challenge": challenge, "source": "oracle"},
              "cleanup": {"group_absent": cleanup["group_absent"], "zombies_absent": cleanup["zombies_absent"], "descriptors_closed": cleanup["descriptors_closed"], "shm_removed": cleanup["shm_removed"]},
              "journal": {"case_closed": True, "group_absent_before_result": cleanup["group_absent"]},
              "process_residual": False, "journal_present": False, "group_present": False}
    return result


def main_process_edges() -> list[str]:
    """Statically report forbidden direct runtime edges in this module."""
    tree = ast.parse(Path(__file__).read_text())
    forbidden = {"fork", "system", "popen", "run", "Popen", "check_call", "check_output"}
    edges = []
    for node in ast.walk(tree):
        if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute) and node.func.attr in forbidden:
            edges.append(f"{node.func.attr}@{node.lineno}")
    return sorted(set(edges))


def run_supervised_case(*, attack_id: str = "P1350-B01-raw-inherited-entry", operation: str = "raw_inherited_entry", expected: str = "FRAME_AUTHORITY") -> dict[str, Any]:
    """Run one disposable R3 case; all child execution stays in the harness."""
    supervisor = _load(SUPERVISOR_REL, SUPERVISOR_SHA256, "p1351_supervisor_r3")
    harness = _load(HARNESS_REL, HARNESS_SHA256, "p1351_harness_r3")
    harness.bind_supervisor(supervisor)
    fd, path = tempfile.mkstemp(prefix="p1351-boundary-", suffix=".jsonl", dir="/dev/shm")
    os.close(fd)
    journal_fd = os.open(path, os.O_RDWR | os.O_APPEND | os.O_CLOEXEC)
    journal = supervisor.RunJournal(journal_fd, secrets.token_hex(32), 1)
    try:
        request = {"attack_id": attack_id, "attack_nonce": secrets.token_hex(32), "challenge": secrets.token_hex(32)}
        raw = harness.run_boundary_experiment(request, [attack_id, "compat", operation, expected], journal, None)
        raw["_nonce"], raw["_challenge"] = request["attack_nonce"], request["challenge"]
        return validate_and_project(raw)
    finally:
        journal.closed = True
        try:
            os.close(journal_fd)
        finally:
            try:
                os.unlink(path)
            except FileNotFoundError:
                pass


def _selftest() -> None:
    cleanup = {key: True for key in ("descriptors_closed", "group_absent", "patches_restored", "registry_destroyed", "shm_removed", "zombies_absent")}
    executor = {"path_kind": "PIDFD", "pgid": 4, "pid": 4, "sid": 4, "starttime_ticks": 9, "terminal_wait_observed": True}
    assert validate_executor_axes(executor, cleanup)["path_kind"] == "EXECUTOR"
    assert validate_executor_axes({"path_kind": "PUBLIC_REFUSAL", "pgid": None, "pid": None, "sid": None, "starttime_ticks": None, "terminal_wait_observed": True}, cleanup)["identity_transport"] is None
    try:
        validate_executor_axes({"path_kind": "PIDFD", "identity_transport": None, "pgid": None, "pid": None, "sid": None, "starttime_ticks": None, "terminal_wait_observed": False}, cleanup)
    except CompatibilityFailure:
        pass
    else:
        raise AssertionError("transport leaked onto path_kind axis")


if __name__ == "__main__":
    _selftest()
