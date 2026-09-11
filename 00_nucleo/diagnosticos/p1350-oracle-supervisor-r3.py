#!/usr/bin/env python3
"""P1350 Oracle R3 supervisor facade.

The frozen R2 supervisor remains the transport implementation.  This revision
adds only the closed public projection required by the boundary result; it does
not alter deadlines, process-group cleanup, route coverage, or reason codes.
"""

from __future__ import annotations

import hashlib
import importlib.util
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
BASE_REL = "00_nucleo/diagnosticos/p1350-oracle-supervisor-r2.py"
BASE_SHA256 = "dc61799da0d71d73c43be0a9251289e94365979de4fc5809d7a359ef1e4adb1d"
PUBLIC_KEYS = ["attempt_finished_ns", "attempt_started_ns", "journal_fdatasync_observed", "parent_pid", "public_invocation_observed", "run_id"]


def _load_base() -> Any:
    path = ROOT / BASE_REL
    raw = path.read_bytes()
    if path.is_symlink() or hashlib.sha256(raw).hexdigest() != BASE_SHA256:
        raise RuntimeError("AUTHORITY_ROOT: P1350 R2 supervisor drift")
    spec = importlib.util.spec_from_file_location("p1350_supervisor_r2_for_r3", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("AUTHORITY_ROOT: P1350 R2 supervisor loader")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


_BASE = _load_base()

# Preserve the entire frozen transport surface.  Function globals remain owned
# by the pinned R2 module, so this facade cannot silently change its semantics.
for _name, _value in vars(_BASE).items():
    if not _name.startswith("__") and _name not in globals():
        globals()[_name] = _value


def public_boundary_supervisor(
    *,
    attempt_finished_ns: int,
    attempt_started_ns: int,
    journal_fdatasync_observed: bool,
    parent_pid: int,
    public_invocation_observed: bool,
    run_id: str,
) -> dict[str, Any]:
    """Build the exact public supervisor evidence without synthetic defaults."""
    value = {
        "attempt_finished_ns": attempt_finished_ns,
        "attempt_started_ns": attempt_started_ns,
        "journal_fdatasync_observed": journal_fdatasync_observed,
        "parent_pid": parent_pid,
        "public_invocation_observed": public_invocation_observed,
        "run_id": run_id,
    }
    if list(value) != PUBLIC_KEYS or type(attempt_started_ns) is not int or type(attempt_finished_ns) is not int or attempt_finished_ns < attempt_started_ns or type(parent_pid) is not int or parent_pid <= 0 or type(run_id) is not str or not run_id or journal_fdatasync_observed is not True or public_invocation_observed is not True:
        raise SupervisorFailure("SUPERVISOR_AUTHORITY", "boundary public supervisor evidence")
    return value
