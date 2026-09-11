#!/usr/bin/env python3
"""Frozen, target-blind adversarial suite for P1350.

Authorship executes only ``--author-check``.  Future integration must supply
whole-file-pinned targets and an independent verifier driver.  Boundary attacks
are isolated protocol experiments outside the semantic mutation denominator;
they do not add another public adapter entrypoint.
"""

from __future__ import annotations

import hashlib
import importlib.util
import json
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Protocol


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"
SEQUENCE_SHA256 = "e958b9c5e939a4b06b9e8594966001004de814e49e0141034d5351f1dbf4a907"
CASE_DEADLINE_NS = 12_000_000_000
TERM_GRACE_NS = 250_000_000
CLEANUP_NS = 2_000_000_000
SINGLE_CASE_MAX_NS = CASE_DEADLINE_NS + TERM_GRACE_NS + CLEANUP_NS

PINS = {
    "step_p1350": (ROOT / "00_nucleo/materialization/typst-passo-1350.md", "e09cf72e1e81c414343f8ec400e8d1e242910ed9a8a56ed7aaa96308a36d9ebf"),
    "contract_spec_p1350": (DIAG / "p1350-contract-spec-r1.json", "bdc3f3c0a6ef7baec3b2cb2e31c88c099dc5a49b18e97ae0a0fcc86535286e82"),
    "contract_binding_p1350": (DIAG / "p1350-contract-binding-r1.json", "257114e18af3ef0d968294e5e9b8326bd963eb7fa92a1d2c489c6e05f388fcfb"),
    "contract_receipt_p1350": (DIAG / "p1350-contract-receipt-r1.json", "e741751e0641c46be2e8aa634cc96b0ee400cd3b6d5693b25eec66cba283008d"),
    "verifier_blocker_p1349": (DIAG / "p1349-verifier-blocker-r2.json", "16780a65c46ba958b6e1535184ea039d022f558a01fb384cc0c110c261d55417"),
    "contract_spec_p1349": (DIAG / "p1349-contract-spec-r1.json", "9b3363fe2c60d634a7547357217299a2fc610e04f3c0116b667ce23caf555335"),
    "contract_binding_p1349": (DIAG / "p1349-contract-binding-r1.json", "3db25f33a5af2716b432788cd06e8ec527567584b803def215bd45cad06e29bd"),
    "contract_receipt_p1349": (DIAG / "p1349-contract-receipt-r1.json", "6baf1c42baaea917035e0d54729dba9430c5799c5cd61212f5f31dbce33b6ef6"),
    "adversary_suite_p1349": (DIAG / "p1349-adversary-suite-r1.py", "9f7320db878b16f7be0436b364b34a2f3fdc7ea6a8b34cfb50da0a38930a1703"),
    "adversary_authorship_p1349": (DIAG / "p1349-adversary-authorship-r1.md", "ea364c7194fa2c3f9b15da2b53cac261cb4576e925713a2329e3d7b7484818b9"),
    "adversary_authorship_receipt_p1349": (DIAG / "p1349-adversary-authorship-receipt-r1.json", "ec08d9406e3fbd993d750236e454b6d3f7210c11a1eddc31b1770f558af7b6a2"),
}


def sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def canonical(value: Any) -> bytes:
    return (json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")) + "\n").encode()


def strict_json(raw: bytes, label: str) -> Any:
    def pairs(items: list[tuple[str, Any]]) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key, value in items:
            if key in result:
                raise RuntimeError(f"DUPLICATE_KEY: {label}:{key}")
            result[key] = value
        return result

    try:
        return json.loads(raw.decode("utf-8", "strict"), object_pairs_hook=pairs)
    except (UnicodeError, json.JSONDecodeError) as exc:
        raise RuntimeError(f"SCHEMA: {label}") from exc


def attack(identifier: str, group: str, operation: str, blocker_code: str, *, executor_instances: int = 1, **parameters: Any) -> dict[str, Any]:
    dynamic = group != "static_bypass"
    return {
        "attack_id": identifier,
        "cost": {
            "complete_runs": 0,
            "executor_instances_max": executor_instances if dynamic else 0,
            "full_runs": 0,
            "wall_ns_max": SINGLE_CASE_MAX_NS * executor_instances if dynamic else 0,
        },
        "execution_scope": "isolated_boundary_invocation",
        "expected_blocker_code": blocker_code,
        "expected_domain": "integration_blocker",
        "expected_schema": "p1349-integration-blocker-r1",
        "expected_verdict": "INTEGRATION_BLOCKED",
        "group": group,
        "operation": operation,
        "parameters": parameters,
        "score_eligible": False,
        "valid": True,
    }


STATIC_BYPASS_ATTACKS = [
    attack("P1350-B01-raw-inherited-entry-from-batch", "static_bypass", "call_raw_inherited_from_batch", "SUPERVISOR_AUTHORITY", inherited=True),
    attack("P1350-B02-disable-supervisor-or-direct-fallback", "static_bypass", "inject_disable_or_fallback_control", "SUPERVISOR_AUTHORITY", sources=["flag", "environment", "case", "default"]),
    attack("P1350-B03-unbound-or-duplicate-operation", "static_bypass", "corrupt_route_coverage", "SUPERVISOR_AUTHORITY", mutations=["omit", "duplicate"]),
    attack("P1350-B04-route-selected-before-handshake", "static_bypass", "select_route_before_authenticated_ack", "SUPERVISOR_AUTHORITY"),
]

DYNAMIC_SUPERVISOR_ATTACKS = [
    attack("P1350-D01-stop-before-handshake", "dynamic_supervisor", "stop_executor_before_handshake", "SUPERVISOR_TIMEOUT"),
    attack("P1350-D02-stop-after-handshake", "dynamic_supervisor", "stop_executor_after_handshake", "SUPERVISOR_TIMEOUT"),
    attack("P1350-D03-ignore-deadline", "dynamic_supervisor", "ignore_case_deadline", "SUPERVISOR_TIMEOUT", maximum_case_duration_ns=CASE_DEADLINE_NS),
    attack("P1350-D04-ignore-term", "dynamic_supervisor", "ignore_term_until_kill", "SUPERVISOR_TIMEOUT", term_grace_ns=TERM_GRACE_NS, kill_required=True),
    attack("P1350-D05-descendant-ptrace-stop", "dynamic_supervisor", "leave_registered_descendant_ptrace_stop", "ORPHAN_PROCESS", executor_instances=2),
    attack("P1350-D06-sid-pgid-divergence", "dynamic_supervisor", "diverge_sid_or_pgid", "SUPERVISOR_AUTHORITY", require="SID==PGID==PID"),
    attack("P1350-D07-pid-starttime-divergence", "dynamic_supervisor", "diverge_pid_or_starttime", "SUPERVISOR_AUTHORITY"),
    attack("P1350-D08-pidfd-divergence", "dynamic_supervisor", "swap_retained_pidfd", "SUPERVISOR_AUTHORITY"),
    attack("P1350-D09-unregistered-fork", "dynamic_supervisor", "fork_without_registration_ack", "SUPERVISOR_AUTHORITY", executor_instances=2),
    attack("P1350-D10-orphan-child", "dynamic_supervisor", "orphan_registered_child", "ORPHAN_PROCESS", executor_instances=2),
    attack("P1350-D11-reap-wrong-pid", "dynamic_supervisor", "substitute_waitpid_target", "SUPERVISOR_AUTHORITY", executor_instances=2),
    attack("P1350-D12-missing-frame", "dynamic_supervisor", "emit_invalid_frame", "FRAME_AUTHORITY", mutation="missing"),
    attack("P1350-D13-partial-frame", "dynamic_supervisor", "emit_invalid_frame", "FRAME_AUTHORITY", mutation="partial"),
    attack("P1350-D14-duplicate-frame", "dynamic_supervisor", "emit_invalid_frame", "FRAME_AUTHORITY", mutation="duplicate"),
    attack("P1350-D15-noncanonical-frame", "dynamic_supervisor", "emit_invalid_frame", "FRAME_AUTHORITY", mutation="noncanonical", variants=["trailing_bytes", "nonempty_stderr"]),
    attack("P1350-D16-oversized-frame", "dynamic_supervisor", "emit_invalid_frame", "FRAME_AUTHORITY", mutation="oversized", bytes=65537),
    attack("P1350-D17-missing-capability", "dynamic_supervisor", "mutate_handshake_capability", "SUPERVISOR_AUTHORITY", mutation="missing"),
    attack("P1350-D18-replayed-capability", "dynamic_supervisor", "mutate_handshake_capability", "SUPERVISOR_AUTHORITY", mutation="replay", executor_instances=2),
    attack("P1350-D19-cross-case-capability", "dynamic_supervisor", "mutate_handshake_capability", "SUPERVISOR_AUTHORITY", mutation="cross_case_swap", executor_instances=2),
    attack("P1350-D20-adapter-exception", "dynamic_supervisor", "raise_adapter_exception", "ADAPTER_EXCEPTION", restoration_required=True),
    attack("P1350-D21-checker-error", "dynamic_supervisor", "raise_checker_error", "CHECKER_ERROR", cleanup_required=True),
]

JOURNAL_ATTACKS = [
    attack("P1350-J01-truncated-line", "journal", "mutate_journal", "JOURNAL_AUTHORITY", mutation="truncate_or_missing_lf"),
    attack("P1350-J02-broken-hash-chain", "journal", "mutate_journal", "JOURNAL_AUTHORITY", mutation="previous_or_current_hash"),
    attack("P1350-J03-event-out-of-order", "journal", "mutate_journal", "JOURNAL_AUTHORITY", mutation="event_order"),
    attack("P1350-J04-two-active-cases", "journal", "mutate_journal", "JOURNAL_AUTHORITY", mutation="concurrent_case_start", executor_instances=2),
    attack("P1350-J05-case-without-closure", "journal", "mutate_journal", "JOURNAL_AUTHORITY", mutation="omit_case_result"),
    attack("P1350-J06-result-before-group-absent", "journal", "mutate_journal", "JOURNAL_AUTHORITY", mutation="premature_case_result"),
    attack("P1350-J07-premature-run-complete", "journal", "mutate_journal", "JOURNAL_AUTHORITY", mutation="premature_or_duplicate_run_complete"),
    attack("P1350-J08-executor-writes-journal", "journal", "mutate_journal_writer", "JOURNAL_AUTHORITY", writer="executor_or_descendant"),
]

FRESHNESS_ATTACKS = [
    attack("P1350-F01-repeat-reuses-registry", "freshness", "reuse_state_between_cases", "CLEANUP_FAILURE", state="registry", executor_instances=2),
    attack("P1350-F02-reverse-reuses-fd", "freshness", "reuse_state_between_cases", "CLEANUP_FAILURE", state="fd", executor_instances=2),
    attack("P1350-F03-process-or-monkeypatch-residue", "freshness", "leave_cross_case_residue", "ORPHAN_PROCESS", states=["process", "monkeypatch"], executor_instances=2),
]

BOUNDARY_ATTACKS = STATIC_BYPASS_ATTACKS + DYNAMIC_SUPERVISOR_ATTACKS + JOURNAL_ATTACKS + FRESHNESS_ATTACKS


@dataclass(frozen=True)
class FrozenTargets:
    checker: Path
    checker_sha256: str
    caller: Path
    caller_sha256: str
    delivery_receipt: Path
    delivery_receipt_sha256: str


class VerifierDriver(Protocol):
    def route_coverage(self, targets: FrozenTargets) -> dict[str, Any]: ...
    def exercise_boundary(self, case: dict[str, Any], targets: FrozenTargets) -> dict[str, Any]: ...
    def exercise_meta(self, case: dict[str, Any], targets: FrozenTargets) -> dict[str, Any]: ...
    def exercise_focal(self, case: dict[str, Any], targets: FrozenTargets, exact_ids: list[str]) -> dict[str, Any]: ...


def load_p1349_suite() -> Any:
    path, digest = PINS["adversary_suite_p1349"]
    if path.is_symlink() or sha256(path.read_bytes()) != digest:
        raise RuntimeError("PROTECTED_INPUT: P1349 adversary suite")
    spec = importlib.util.spec_from_file_location("p1349_frozen_adversary_baseline", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("PROTECTED_INPUT: P1349 suite loader")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def inherited_slices() -> dict[str, list[dict[str, Any]]]:
    base = load_p1349_suite()
    slices = base.frozen_classification_slices()
    return {
        "meta": [dict(item) for item in base.META_TESTS],
        "x14_and_supervisor_variants": [dict(item) for item in slices["x14"] + slices["new_variants"]],
        "other_p1348": [dict(item) for item in slices["other_p1348"]],
        "p1347_regressions": [dict(item) for item in slices["p1347_regressions"]],
        "p1346_replays": [dict(item) for item in slices["p1346_replays"]],
        "controls": [dict(item) for item in slices["controls"]],
    }


def route_coverage_manifest() -> list[dict[str, str]]:
    slices = inherited_slices()
    new_operations = {item["adapter_operation"] for item in slices["x14_and_supervisor_variants"]}
    classification_cases = [item for key, values in slices.items() if key != "meta" for item in values]
    all_operations = {item["adapter_operation"] for item in classification_cases}
    inherited_operations = all_operations - new_operations
    if len(new_operations) != 12 or len(inherited_operations) != 48 or len(all_operations) != 60:
        raise RuntimeError("PROTECTED_INPUT: expected 12 + 48 = 60 operation routes")
    return [
        {"operation": operation, "route": "P1350_LOCAL" if operation in new_operations else "INHERITED_P1348_P1347_P1346"}
        for operation in sorted(all_operations)
    ]


def validate_authorship() -> dict[str, Any]:
    for label, (path, digest) in PINS.items():
        if path.is_symlink() or sha256(path.read_bytes()) != digest:
            raise RuntimeError(f"PROTECTED_INPUT: {label}")
    binding = strict_json(PINS["contract_binding_p1350"][0].read_bytes(), "P1350 binding")
    blocker = strict_json(PINS["verifier_blocker_p1349"][0].read_bytes(), "P1349 blocker")
    base = load_p1349_suite()
    base_summary = base.validate_authorship()
    slices = inherited_slices()
    route_manifest = route_coverage_manifest()
    group_counts = {
        "static_bypass": len(STATIC_BYPASS_ATTACKS),
        "dynamic_supervisor": len(DYNAMIC_SUPERVISOR_ATTACKS),
        "journal": len(JOURNAL_ATTACKS),
        "freshness": len(FRESHNESS_ATTACKS),
    }
    if group_counts != binding["attacks_controls_cardinality_binding"]["mandatory_attack_groups"]:
        raise RuntimeError("SCHEMA: mandatory P1350 attack groups")
    if len(BOUNDARY_ATTACKS) != 36 or len({item["attack_id"] for item in BOUNDARY_ATTACKS}) != 36:
        raise RuntimeError("SCHEMA: boundary attack identity")
    expected_slices = {"meta": 2, "x14_and_supervisor_variants": 12, "other_p1348": 22, "p1347_regressions": 39, "p1346_replays": 181, "controls": 11}
    if {key: len(value) for key, value in slices.items()} != expected_slices:
        raise RuntimeError("PROTECTED_INPUT: inherited slice cardinalities")
    if base_summary["focal_valid_negatives_bound"] != 254 or base_summary["sequence_sha256"] != SEQUENCE_SHA256:
        raise RuntimeError("PROTECTED_INPUT: P1349 semantic denominator")
    coverage = binding["public_boundary_binding"]["coverage_counts"]
    if coverage != {"declared": 60, "new": 12, "inherited": 48, "supervised": 60, "unbound": 0, "duplicate": 0, "direct_inherited_edges": 0}:
        raise RuntimeError("PROTECTED_INPUT: P1350 route coverage contract")
    if blocker["terminal_observation"]["observed_descendant_count"] != 6 or blocker["terminal_observation"]["minimum_main_elapsed_seconds_at_last_snapshot"] < 869:
        raise RuntimeError("PROTECTED_INPUT: P1349 public blocker observation")
    return {
        "authorship_cost": {"focal_runs": 0, "full_runs": 0, "ptrace_runs": 0, "target_processes": 0},
        "boundary_attacks": len(BOUNDARY_ATTACKS),
        "boundary_future_wall_ns_max_sequential": sum(item["cost"]["wall_ns_max"] for item in BOUNDARY_ATTACKS),
        "candidate_read_or_executed": False,
        "controls_bound": 11,
        "focal_valid_negatives_bound": 254,
        "full_runs": 0,
        "inherited_operation_routes": 48,
        "meta_tests_bound": 2,
        "new_operation_routes": 12,
        "oracle_checker_adapter_read_or_executed": False,
        "p1346_replays_bound": 181,
        "p1347_regressions_bound": 39,
        "route_manifest_sha256": sha256(canonical(route_manifest)),
        "supervised_operation_routes": len(route_manifest),
        "verdict": "AUTHORED_NOT_EXECUTED",
    }


def load_driver(path: Path, digest: str) -> VerifierDriver:
    if path.is_symlink() or sha256(path.read_bytes()) != digest:
        raise RuntimeError("AUTHORITY_ROOT: frozen verifier driver")
    spec = importlib.util.spec_from_file_location("p1350_frozen_verifier_driver", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("AUTHORITY_ROOT: driver loader")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    required = ["route_coverage", "exercise_boundary", "exercise_meta", "exercise_focal"]
    if any(not hasattr(module, name) for name in required):
        raise RuntimeError("SCHEMA: P1350 verifier-driver surface")
    return module


def boundary_matches(case: dict[str, Any], observed: dict[str, Any]) -> bool:
    return (
        observed.get("schema") == case["expected_schema"]
        and observed.get("case_id") == case["attack_id"]
        and observed.get("blocker_code") == case["expected_blocker_code"]
        and observed.get("verdict") == case["expected_verdict"]
        and "classification" not in observed
        and "reason_code" not in observed
        and observed.get("cleanup", {}).get("group_absent") is True
        and observed.get("cleanup", {}).get("zombies_absent") is True
    )


def classification_matches(case: dict[str, Any], observed: dict[str, Any]) -> bool:
    expected_reason = case.get("expected_reason_code")
    return (
        observed.get("schema") == "p1349-classification-result-r1"
        and observed.get("case_id") == case["case_id"]
        and observed.get("classification") == case["expected_classification"]
        and (expected_reason is None or observed.get("reason_code") == expected_reason)
        and "blocker_code" not in observed
        and "meta_status" not in observed
    )


def run_with_driver(targets: FrozenTargets, driver_path: Path, driver_sha256: str) -> dict[str, Any]:
    """Future verifier entrypoint; never called by authorship mode."""
    validate_authorship()
    for path, digest in [(targets.checker, targets.checker_sha256), (targets.caller, targets.caller_sha256), (targets.delivery_receipt, targets.delivery_receipt_sha256)]:
        if path.is_symlink() or sha256(path.read_bytes()) != digest:
            raise RuntimeError("AUTHORITY_ROOT: frozen target drift")
    driver = load_driver(driver_path, driver_sha256)
    expected_routes = route_coverage_manifest()
    observed_coverage = driver.route_coverage(targets)
    if observed_coverage.get("routes") != expected_routes or observed_coverage.get("declared") != 60 or observed_coverage.get("supervised") != 60 or observed_coverage.get("direct_inherited_edges") != 0:
        return {"full_runs": 0, "verdict": "STATIC_COVERAGE_STOP"}
    boundary_records = [driver.exercise_boundary(case, targets) for case in BOUNDARY_ATTACKS]
    boundary_failures = [case["attack_id"] for case, observed in zip(BOUNDARY_ATTACKS, boundary_records) if not boundary_matches(case, observed)]
    result: dict[str, Any] = {"boundary": boundary_records, "boundary_failures": boundary_failures, "full_runs": 0}
    if boundary_failures:
        result["verdict"] = "BOUNDARY_GATE_STOP"
        return result
    base = load_p1349_suite()
    slices = inherited_slices()
    meta_records = [driver.exercise_meta(case, targets) for case in slices["meta"]]
    meta_failures = [case["case_id"] for case, observed in zip(slices["meta"], meta_records) if not base.meta_matches(case, observed)]
    result["meta"] = meta_records
    if meta_failures:
        result.update(meta_failures=meta_failures, verdict="META_GATE_STOP")
        return result
    exact_ids = base.exact_language_ids()
    stages = [
        ("x14_and_supervisor_variants", slices["x14_and_supervisor_variants"]),
        ("other_p1348", slices["other_p1348"]),
        ("p1347_regressions", slices["p1347_regressions"]),
        ("p1346_replays", slices["p1346_replays"]),
        ("controls", slices["controls"]),
    ]
    for label, cases in stages:
        records = [driver.exercise_focal(case, targets, exact_ids) for case in cases]
        failures = [case["case_id"] for case, observed in zip(cases, records) if not classification_matches(case, observed)]
        result[label] = records
        if failures:
            result.update(failures=failures, verdict="CLASSIFICATION_GATE_STOP")
            return result
    result["verdict"] = "ZERO_SURVIVORS_PENDING_INDEPENDENT_COUNTING"
    return result


def main() -> int:
    if sys.argv[1:] != ["--author-check"]:
        raise RuntimeError("SCHEMA: authorship CLI accepts exactly --author-check")
    sys.stdout.buffer.write(canonical(validate_authorship()))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
