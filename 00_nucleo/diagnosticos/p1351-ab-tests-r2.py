#!/usr/bin/env python3
"""P1351 independent A/B tests (revision r2).

This file is authored from the P1351 step and the public P1350 blocker only.
It intentionally does not import a candidate during ``--selftest``.  A later
verifier supplies the candidate path as the sole positional argument.

Candidate API (all functions are required):

    project_executor(raw: Mapping) -> Mapping
    project_boundary(raw: Mapping) -> Mapping
    project_meta(raw: Mapping) -> Mapping
    supervise_real_case() -> Mapping
    main_process_edges() -> Sequence[str]       # static probe

The first three functions are pure projections.  ``supervise_real_case`` is
the only live test and must create a disposable session under /dev/shm.
"""

from __future__ import annotations

import argparse
import importlib.util
import json
import sys
from collections.abc import Mapping, Sequence
from pathlib import Path
from typing import Any, Callable


PATH_KINDS = {"EXECUTOR", "PUBLIC_REFUSAL"}
TRANSPORTS = {"PIDFD", "WAITPID_ENOSYS", None}
EXECUTOR_KEYS = {"path_kind", "identity_transport", "pid", "pgid", "sid", "starttime_ticks", "terminal_wait_observed"}
META_EVIDENCE = {"nonce": "n-1351", "challenge": "c-1351", "source": "oracle"}


def _executor(path_kind: str = "EXECUTOR", transport: Any = "PIDFD", *, identity: bool = True) -> dict[str, Any]:
    values = {"path_kind": path_kind, "identity_transport": transport,
              "pid": 41001 if identity else None, "pgid": 41001 if identity else None,
              "sid": 41001 if identity else None, "starttime_ticks": 987654 if identity else None,
              "terminal_wait_observed": True}
    return values


def _boundary(executor: Mapping[str, Any] | None = None, **extra: Any) -> dict[str, Any]:
    value: dict[str, Any] = {
        "schema": "p1351-boundary-result-r1",
        "path_kind": "EXECUTOR",
        "identity_transport": "PIDFD",
        "executor": dict(executor or _executor()),
        "nonce": "n-1351",
        "challenge": "c-1351",
        "meta_evidence": dict(META_EVIDENCE),
        "cleanup": {"group_absent": True, "zombies_absent": True,
                     "descriptors_closed": True, "shm_removed": True},
        "journal": {"case_closed": True, "group_absent_before_result": True},
    }
    value.update(extra)
    return value


def _assert_executor_shape(value: Any) -> None:
    assert isinstance(value, Mapping), "executor projection is not a mapping"
    assert set(value) == EXECUTOR_KEYS, f"executor keys differ: {sorted(value)}"
    kind, transport = value["path_kind"], value["identity_transport"]
    assert kind in PATH_KINDS, f"invalid path_kind: {kind!r}"
    assert transport in TRANSPORTS, f"invalid identity_transport: {transport!r}"
    if kind == "PUBLIC_REFUSAL":
        assert transport is None, "PUBLIC_REFUSAL must have null identity_transport"
        assert all(value[key] is None for key in ("pid", "pgid", "sid", "starttime_ticks")), "refusal carries identity"
    else:
        assert transport in {"PIDFD", "WAITPID_ENOSYS"}, "EXECUTOR needs a transport"
        assert all(type(value[key]) is int and value[key] > 0 for key in ("pid", "pgid", "sid", "starttime_ticks")), "invalid executor identity"
        assert value["sid"] == value["pid"] and value["pgid"] == value["pid"], "SID/PGID mismatch"
    assert value["terminal_wait_observed"] is True, "terminal wait not observed"


def _assert_boundary(value: Any, expected_kind: str, expected_transport: Any) -> None:
    assert isinstance(value, Mapping), "boundary result is not a mapping"
    assert value.get("schema") == "p1351-boundary-result-r1", "wrong boundary schema"
    assert value.get("path_kind") == expected_kind
    assert value.get("identity_transport") == expected_transport
    _assert_executor_shape(value.get("executor"))
    assert value.get("nonce") == "n-1351" and value.get("challenge") == "c-1351", "nonce/challenge changed"
    assert value.get("meta_evidence") == META_EVIDENCE, "meta_evidence changed"
    cleanup, journal = value.get("cleanup"), value.get("journal")
    assert isinstance(cleanup, Mapping) and all(cleanup.get(k) is True for k in ("group_absent", "zombies_absent", "descriptors_closed", "shm_removed")), "cleanup incomplete"
    assert isinstance(journal, Mapping) and journal.get("case_closed") is True and journal.get("group_absent_before_result") is True, "journal released before group absence"


def _assert_real(value: Any) -> None:
    assert isinstance(value, Mapping), "real result is not a mapping"
    assert value.get("schema") == "p1351-boundary-result-r1", "wrong boundary schema"
    assert value.get("path_kind") == "EXECUTOR"
    transport = value.get("identity_transport")
    assert transport in {"PIDFD", "WAITPID_ENOSYS"}
    _assert_executor_shape(value.get("executor"))
    # Live runs must bind their own fresh non-empty tokens, not fixture literals.
    nonce, challenge = value.get("nonce"), value.get("challenge")
    assert isinstance(nonce, str) and nonce and isinstance(challenge, str) and challenge
    assert value.get("meta_evidence") == {"nonce": nonce, "challenge": challenge, "source": "oracle"}
    cleanup, journal = value.get("cleanup"), value.get("journal")
    assert isinstance(cleanup, Mapping) and all(cleanup.get(k) is True for k in ("group_absent", "zombies_absent", "descriptors_closed", "shm_removed"))
    assert isinstance(journal, Mapping) and journal.get("case_closed") is True and journal.get("group_absent_before_result") is True
    assert value.get("process_residual") is False, "process residual reported"
    assert value.get("journal_present") is False, "journal residual reported"
    assert value.get("group_present") is False, "group residual reported"


def _reject(fn: Callable[[], None]) -> None:
    """A negative fixture passes only when the independent predicate rejects it."""
    try:
        fn()
    except AssertionError:
        return
    raise AssertionError("invalid fixture was accepted")


def _same_meta(value: Mapping[str, Any]) -> None:
    assert value["meta_evidence"] == META_EVIDENCE


def _distinct(value_a: Any, value_b: Any) -> None:
    assert value_a != value_b


def _assert_empty(value: Any) -> None:
    assert isinstance(value, Sequence) and not isinstance(value, (str, bytes))
    assert len(value) == 0, f"main-process runtime edge(s): {list(value)!r}"


def _assert_repeat(first: Any, second: Any) -> None:
    assert isinstance(first, Mapping) and isinstance(second, Mapping)
    assert first != second, "repeat reused observable state"
    assert isinstance(first.get("nonce"), str) and isinstance(second.get("nonce"), str)
    assert isinstance(first.get("challenge"), str) and isinstance(second.get("challenge"), str)
    assert first["nonce"] != second["nonce"], "repeat reused nonce"
    assert first["challenge"] != second["challenge"], "repeat reused challenge"
    assert first.get("process_residual") is False and second.get("process_residual") is False


def pure_tests() -> list[tuple[str, Callable[[], None]]]:
    tests: list[tuple[str, Callable[[], None]]] = []

    def t(name: str, fn: Callable[[], None]) -> None:
        tests.append((name, fn))

    t("fixture_executor_pidfd_schema", lambda: _assert_executor_shape(_executor("EXECUTOR", "PIDFD")))
    t("fixture_executor_waitpid_schema", lambda: _assert_executor_shape(_executor("EXECUTOR", "WAITPID_ENOSYS")))
    t("fixture_public_refusal_schema", lambda: _assert_executor_shape(_executor("PUBLIC_REFUSAL", None, identity=False)))

    # The four values below are deliberately placed on the wrong axis.
    t("wrong_path_pidfd", lambda: _reject(lambda: _assert_executor_shape(_executor("PIDFD", None))))
    t("wrong_path_waitpid_enosys", lambda: _reject(lambda: _assert_executor_shape(_executor("WAITPID_ENOSYS", None))))
    t("wrong_transport_executor", lambda: _reject(lambda: _assert_executor_shape(_executor("PUBLIC_REFUSAL", "EXECUTOR", identity=False))))
    t("wrong_transport_refusal", lambda: _reject(lambda: _assert_executor_shape(_executor("PUBLIC_REFUSAL", "PUBLIC_REFUSAL", identity=False))))

    t("reject_fallback_without_enosys", lambda: _reject(lambda: _assert_executor_shape(_executor("EXECUTOR", "WAITPID", identity=True))))
    t("reject_invalid_pid", lambda: _reject(lambda: _assert_executor_shape({**_executor(), "pid": 0})))
    t("reject_invalid_sid", lambda: _reject(lambda: _assert_executor_shape({**_executor(), "sid": 77})))
    t("reject_missing_terminal_wait", lambda: _reject(lambda: _assert_executor_shape({**_executor(), "terminal_wait_observed": False})))
    t("reject_refusal_identity", lambda: _reject(lambda: _assert_executor_shape(_executor("PUBLIC_REFUSAL", None, identity=True))))
    t("reject_executor_null_transport", lambda: _reject(lambda: _assert_executor_shape(_executor("EXECUTOR", None))))

    t("nonce_challenge_meta_are_bound", lambda: _assert_boundary(_boundary(), "EXECUTOR", "PIDFD"))
    t("journal_requires_group_absent", lambda: _reject(lambda: _assert_boundary(_boundary(journal={"case_closed": True, "group_absent_before_result": False}), "EXECUTOR", "PIDFD")))
    t("cleanup_requires_zombies_absent", lambda: _reject(lambda: _assert_boundary(_boundary(cleanup={"group_absent": True, "zombies_absent": False, "descriptors_closed": True, "shm_removed": True}), "EXECUTOR", "PIDFD")))
    t("refusal_has_no_process_identity", lambda: _assert_boundary(_boundary(_executor("PUBLIC_REFUSAL", None, identity=False), path_kind="PUBLIC_REFUSAL", identity_transport=None), "PUBLIC_REFUSAL", None))
    t("refusal_transport_is_null", lambda: _reject(lambda: _assert_boundary(_boundary(_executor("PUBLIC_REFUSAL", None, identity=False), path_kind="PUBLIC_REFUSAL", identity_transport="PIDFD"), "PUBLIC_REFUSAL", None)))
    t("meta_evidence_exact", lambda: _reject(lambda: _assert_boundary(_boundary(meta_evidence={"nonce": "changed"}), "EXECUTOR", "PIDFD")))
    t("fresh_nonce_not_reused", lambda: _distinct("n-1351", "n-1351-repeat"))
    t("fresh_challenge_not_reused", lambda: _distinct("c-1351", "c-1351-repeat"))
    t("no_main_process_edge_fixture", lambda: _distinct([], ["fork"]))
    return tests


def load_candidate(path: Path) -> Any:
    if path.is_symlink():
        raise RuntimeError("candidate path must not be a symlink")
    spec = importlib.util.spec_from_file_location("p1351_candidate_under_test", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("candidate cannot be loaded")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def candidate_tests(module: Any) -> list[tuple[str, Callable[[], None]]]:
    tests: list[tuple[str, Callable[[], None]]] = []

    def call(name: str, *args: Any) -> Any:
        fn = getattr(module, name, None)
        if not callable(fn):
            raise AssertionError(f"missing required API: {name}")
        return fn(*args)

    def projected_executor(raw: Mapping[str, Any], kind: str, transport: Any) -> None:
        value = call("project_executor", raw)
        _assert_executor_shape(value)
        assert value["path_kind"] == kind and value["identity_transport"] == transport

    tests.append(("candidate_executor_pidfd", lambda: projected_executor(_executor("EXECUTOR", "PIDFD"), "EXECUTOR", "PIDFD")))
    tests.append(("candidate_executor_waitpid_enosys", lambda: projected_executor(_executor("EXECUTOR", "WAITPID_ENOSYS"), "EXECUTOR", "WAITPID_ENOSYS")))
    tests.append(("candidate_executor_public_refusal", lambda: projected_executor(_executor("PUBLIC_REFUSAL", None, identity=False), "PUBLIC_REFUSAL", None)))
    tests.append(("candidate_public_refusal_no_process", lambda: _assert_boundary(call("project_boundary", _boundary(_executor("PUBLIC_REFUSAL", None, identity=False), path_kind="PUBLIC_REFUSAL", identity_transport=None)), "PUBLIC_REFUSAL", None)))
    tests.append(("candidate_nonce_challenge_meta", lambda: _assert_boundary(call("project_boundary", _boundary()), "EXECUTOR", "PIDFD")))
    tests.append(("candidate_meta_projection", lambda: _same_meta(call("project_meta", {"meta_evidence": META_EVIDENCE, "nonce": "n-1351", "challenge": "c-1351"}))))
    tests.append(("candidate_real_supervision_journal_group_absent", lambda: _assert_real(call("supervise_real_case"))))
    tests.append(("candidate_no_main_process_bypass", lambda: _assert_empty(call("main_process_edges"))))
    tests.append(("candidate_repeat_fresh_state", lambda: _assert_repeat(call("supervise_real_case"), call("supervise_real_case"))))
    return tests


def run(tests: Sequence[tuple[str, Callable[[], None]]], *, stop_on_residual: bool) -> tuple[int, int, bool]:
    passed = failed = 0
    residual = False
    for name, fn in tests:
        try:
            fn()
        except BaseException as exc:  # accumulate independent failures
            failed += 1
            print(f"FAIL {name}: {type(exc).__name__}: {exc}")
            if "residual" in str(exc).lower() or "group_present" in str(exc).lower():
                residual = True
                if stop_on_residual:
                    print("STOP residual process/resource reported")
                    break
        else:
            passed += 1
            print(f"PASS {name}")
    return passed, failed, residual


def main(argv: Sequence[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("candidate", nargs="?", type=Path, help="candidate module; omitted for pure selftest")
    parser.add_argument("--selftest", action="store_true", help="validate suite fixtures without loading a candidate")
    args = parser.parse_args(argv)
    if args.selftest and args.candidate:
        parser.error("--selftest does not accept a candidate")
    tests = pure_tests()
    if args.selftest or args.candidate is None:
        passed, failed, _ = run(tests, stop_on_residual=False)
        print(json.dumps({"mode": "selftest", "passed": passed, "failed": failed, "total": len(tests)}, sort_keys=True))
        return 0 if failed == 0 else 1
    module = load_candidate(args.candidate)
    passed, failed, residual = run(tests + candidate_tests(module), stop_on_residual=True)
    print(json.dumps({"mode": "candidate", "passed": passed, "failed": failed, "total": len(tests) + len(candidate_tests(module)), "residual_stop": residual}, sort_keys=True))
    return 0 if failed == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
