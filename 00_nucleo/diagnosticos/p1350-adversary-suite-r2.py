#!/usr/bin/env python3
"""Frozen P1350 R2 adversarial authorship.

This file is deliberately blind to every P1350 R2 oracle, checker, caller,
supervisor, journal parser and adapter.  Its CLI accepts only
``--author-check``.  A later independent verifier may call ``run_with_driver``
with whole-file-pinned targets and a verifier-owned driver.

The three identity domains are intentionally different types and parameters:
122 LANGUAGE_ID values, 60 FOCAL_OPERATION values and 36 BOUNDARY_ATTACK
values.  Boundary PASS is recomputed from fresh dynamic evidence; a static
answer containing the expected blocker label is never sufficient.
"""

from __future__ import annotations

import hashlib
import importlib.util
import json
import secrets
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Protocol


ROOT = Path(__file__).resolve().parents[2]
DIAG = ROOT / "00_nucleo/diagnosticos"

LANGUAGE_ID_SEQUENCE_SHA256 = "e958b9c5e939a4b06b9e8594966001004de814e49e0141034d5351f1dbf4a907"
BOUNDARY_ATTACK_MANIFEST_SHA256 = "d2e85d5936557936120754b647d41e88349ab19169ee4dd3fc967151bb68d8d8"
FOCAL_OPERATION_IDS_SHA256 = "997e107d263c2ffa4b649ac9a7f8320dc673f56956800b06cb4316bab3baabd1"

CASE_DEADLINE_NS = 12_000_000_000
TERM_GRACE_NS = 250_000_000
CLEANUP_DEADLINE_NS = 2_000_000_000
SINGLE_EXECUTOR_WALL_NS_MAX = CASE_DEADLINE_NS + TERM_GRACE_NS + CLEANUP_DEADLINE_NS
BOUNDARY_SEQUENTIAL_WALL_NS_MAX = 598_500_000_000

PINS = {
    "step_p1350": (ROOT / "00_nucleo/materialization/typst-passo-1350.md", "e09cf72e1e81c414343f8ec400e8d1e242910ed9a8a56ed7aaa96308a36d9ebf"),
    "contract_spec_p1350_r2": (DIAG / "p1350-contract-spec-r2.json", "17c3b02d2f98ca12d6f89b369b78983663a8f8af6becf08daf47227b8cb42b90"),
    "contract_binding_p1350_r2": (DIAG / "p1350-contract-binding-r2.json", "c341b0db70cebe933bbf6dea9fc19b2122f66534c4488ca4dc00ac9f3ab8f4dd"),
    "contract_receipt_p1350_r2": (DIAG / "p1350-contract-receipt-r2.json", "667e268009baf59cc7fe9f43df2dca55bfa69fbe2484ee06e7175d8165994d7f"),
    "adapter_blocker_p1350_r1": (DIAG / "p1350-adapter-integration-blocker-r1.json", "3f2494f92d10eed22a9d8d0d3647ca9fa532f82f235415fcca93074174c2e341"),
    "adversary_suite_p1350_r1": (DIAG / "p1350-adversary-suite-r1.py", "f6cbd4badd8ae824aec3b73f03fac2fdb169922341ba408c9297ae0ab66f6382"),
    "adversary_authorship_p1350_r1": (DIAG / "p1350-adversary-authorship-r1.md", "d830e822d99f807d73b95b7278d06bf209725eb87e00adf2766bdeefc0296b8e"),
    "adversary_receipt_p1350_r1": (DIAG / "p1350-adversary-authorship-receipt-r1.json", "7ef1d7e894e25b17647929ef28086e138d0c1ad84e8a5ef900eb3454f652b4e6"),
    "adversary_suite_p1349": (DIAG / "p1349-adversary-suite-r1.py", "9f7320db878b16f7be0436b364b34a2f3fdc7ea6a8b34cfb50da0a38930a1703"),
    "adversary_authorship_p1349": (DIAG / "p1349-adversary-authorship-r1.md", "ea364c7194fa2c3f9b15da2b53cac261cb4576e925713a2329e3d7b7484818b9"),
    "adversary_receipt_p1349": (DIAG / "p1349-adversary-authorship-receipt-r1.json", "ec08d9406e3fbd993d750236e454b6d3f7210c11a1eddc31b1770f558af7b6a2"),
}


def sha256(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def rfc8785(value: Any) -> bytes:
    """RFC8785-equivalent for the closed ASCII/integer/bool values used here."""
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode("utf-8")


def canonical_line(value: Any) -> bytes:
    return rfc8785(value) + b"\n"


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


def is_lower_sha256(value: Any) -> bool:
    return isinstance(value, str) and len(value) == 64 and all(char in "0123456789abcdef" for char in value)


# Independently frozen operation allowlist.  Routes are deliberately absent:
# only the later adapter may author the 60-entry operation-to-route map.
FOCAL_OPERATION_IDS = [
    "adversarial_image",
    "alter_validator_stream_with_authority_root",
    "append_worker_stdout",
    "canonical_dag",
    "canonical_traced_probe",
    "canonical_worker",
    "dag_coordinated_substitution",
    "dag_labels_without_validator",
    "dag_mutant_anchor",
    "dag_mutation_accepted_by_real_validator",
    "dag_receipt_answer_channel",
    "dag_valid_schema_self_answer",
    "dag_validator_without_parent_process",
    "detach_after_first_exec_before_exit",
    "detach_at_event",
    "detach_at_exit_event_before_terminal",
    "detach_before_event",
    "detach_before_terminal_wait",
    "drop_parent_wait",
    "dup_over_after_child_check",
    "fabricate_terminal_status",
    "fail_proc_exe",
    "fail_syscall",
    "fail_then_retry_before_commit",
    "inject_registry",
    "inject_second_exec",
    "inject_worker_pid",
    "mix_process_evidence",
    "mix_sibling_pidfd_stream_and_status",
    "mutate_id_sequence",
    "p1346_control_projection",
    "real_dag_validator",
    "remove_capability",
    "remove_trace_option",
    "replay_p1346_negative",
    "replay_worker_challenge",
    "reuse_pid_with_changed_starttime",
    "reuse_terminated_child_pid",
    "second_exec_after_attestation",
    "second_exec_after_exit_event",
    "second_exec_ignores_graceful_termination",
    "second_exec_near_checker_deadline",
    "second_exec_with_unexpected_adapter_failure",
    "self_answer_without_real_validator",
    "stdout_without_child",
    "substitute_anchor_with_authentic_validator_streams",
    "substitute_checker_pidfd_after_supervisor_handshake",
    "substitute_pid_and_pidfd",
    "substitute_proc_exe_image",
    "substitute_process_identity",
    "suppress_exec_stop",
    "swap_fd_after_exec_stop",
    "swap_fd_before_exec",
    "swap_fd_inside_exec",
    "swap_validation_slots",
    "truncate_worker_stdout",
    "validate_same_identity_twice",
    "worker_exit",
    "worker_signal",
    "worker_timeout",
]


# Exact contract-owned tuple order.  These entries, not R1's prose aliases,
# define the R2 boundary namespace and its pinned manifest digest.
BOUNDARY_ATTACK_ENTRIES = [
    ["P1350-B01-raw-inherited-entry", "STATIC_BYPASS", "raw_inherited_entry", "SUPERVISOR_AUTHORITY"],
    ["P1350-B02-supervision-disable-or-fallback", "STATIC_BYPASS", "supervision_disable_or_fallback", "SUPERVISOR_AUTHORITY"],
    ["P1350-B03-operation-map-gap-or-duplicate", "STATIC_BYPASS", "operation_map_gap_or_duplicate", "SUPERVISOR_AUTHORITY"],
    ["P1350-B04-route-before-handshake", "STATIC_BYPASS", "route_before_handshake", "SUPERVISOR_AUTHORITY"],
    ["P1350-D01-executor-stopped-before-handshake", "DYNAMIC_SUPERVISOR", "executor_stopped_before_handshake", "SUPERVISOR_TIMEOUT"],
    ["P1350-D02-executor-stopped-after-handshake", "DYNAMIC_SUPERVISOR", "executor_stopped_after_handshake", "SUPERVISOR_TIMEOUT"],
    ["P1350-D03-deadline-ignored", "DYNAMIC_SUPERVISOR", "deadline_ignored", "SUPERVISOR_TIMEOUT"],
    ["P1350-D04-term-ignored", "DYNAMIC_SUPERVISOR", "term_ignored", "SUPERVISOR_TIMEOUT"],
    ["P1350-D05-descendant-ptrace-stop-at-case-end", "DYNAMIC_SUPERVISOR", "descendant_ptrace_stop_at_case_end", "ORPHAN_PROCESS"],
    ["P1350-D06-sid-pgid-divergence", "DYNAMIC_SUPERVISOR", "sid_pgid_divergence", "SUPERVISOR_AUTHORITY"],
    ["P1350-D07-pid-starttime-divergence", "DYNAMIC_SUPERVISOR", "pid_starttime_divergence", "SUPERVISOR_AUTHORITY"],
    ["P1350-D08-pidfd-divergence", "DYNAMIC_SUPERVISOR", "pidfd_divergence", "SUPERVISOR_AUTHORITY"],
    ["P1350-D09-additional-fork", "DYNAMIC_SUPERVISOR", "additional_fork", "SUPERVISOR_AUTHORITY"],
    ["P1350-D10-orphan-child", "DYNAMIC_SUPERVISOR", "orphan_child", "ORPHAN_PROCESS"],
    ["P1350-D11-wrong-pid-reaped", "DYNAMIC_SUPERVISOR", "wrong_pid_reaped", "SUPERVISOR_AUTHORITY"],
    ["P1350-D12-missing-frame", "DYNAMIC_SUPERVISOR", "missing_frame", "FRAME_AUTHORITY"],
    ["P1350-D13-partial-frame", "DYNAMIC_SUPERVISOR", "partial_frame", "FRAME_AUTHORITY"],
    ["P1350-D14-duplicate-frame", "DYNAMIC_SUPERVISOR", "duplicate_frame", "FRAME_AUTHORITY"],
    ["P1350-D15-noncanonical-frame", "DYNAMIC_SUPERVISOR", "noncanonical_frame", "FRAME_AUTHORITY"],
    ["P1350-D16-oversized-frame", "DYNAMIC_SUPERVISOR", "oversized_frame", "FRAME_AUTHORITY"],
    ["P1350-D17-missing-capability", "DYNAMIC_SUPERVISOR", "missing_capability", "SUPERVISOR_AUTHORITY"],
    ["P1350-D18-replayed-capability", "DYNAMIC_SUPERVISOR", "replayed_capability", "SUPERVISOR_AUTHORITY"],
    ["P1350-D19-cross-case-capability", "DYNAMIC_SUPERVISOR", "cross_case_capability", "SUPERVISOR_AUTHORITY"],
    ["P1350-D20-adapter-exception", "DYNAMIC_SUPERVISOR", "adapter_exception", "ADAPTER_EXCEPTION"],
    ["P1350-D21-checker-error", "DYNAMIC_SUPERVISOR", "checker_error", "CHECKER_ERROR"],
    ["P1350-J01-truncated-line", "JOURNAL", "truncated_line", "JOURNAL_AUTHORITY"],
    ["P1350-J02-broken-hash-chain", "JOURNAL", "broken_hash_chain", "JOURNAL_AUTHORITY"],
    ["P1350-J03-event-out-of-order", "JOURNAL", "event_out_of_order", "JOURNAL_AUTHORITY"],
    ["P1350-J04-two-active-cases", "JOURNAL", "two_active_cases", "JOURNAL_AUTHORITY"],
    ["P1350-J05-case-without-closure", "JOURNAL", "case_without_closure", "JOURNAL_AUTHORITY"],
    ["P1350-J06-result-before-group-absent", "JOURNAL", "result_before_group_absent", "JOURNAL_AUTHORITY"],
    ["P1350-J07-premature-run-complete", "JOURNAL", "premature_run_complete", "JOURNAL_AUTHORITY"],
    ["P1350-J08-executor-writes-journal", "JOURNAL", "executor_writes_journal", "JOURNAL_AUTHORITY"],
    ["P1350-F01-repeat-reuses-registry", "FRESHNESS", "repeat_reuses_registry", "CLEANUP_FAILURE"],
    ["P1350-F02-reverse-reuses-fd", "FRESHNESS", "reverse_reuses_fd", "CLEANUP_FAILURE"],
    ["P1350-F03-residual-process-or-monkeypatch", "FRESHNESS", "residual_process_or_monkeypatch", "CLEANUP_FAILURE"],
]


@dataclass(frozen=True)
class FrozenTargets:
    checker: Path
    checker_sha256: str
    caller: Path
    caller_sha256: str
    supervisor: Path
    supervisor_sha256: str
    journal_parser: Path
    journal_parser_sha256: str
    execution_delivery: Path
    execution_delivery_sha256: str


@dataclass(frozen=True)
class RunContext:
    """Opaque placeholder: only a later checker may create the real value."""

    value: Any


class VerifierDriver(Protocol):
    def bind_verified_run(self, targets: FrozenTargets) -> RunContext: ...
    def exercise_boundary(self, attack: dict[str, Any], targets: FrozenTargets, boundary_attack_manifest_sha256: str, run_context: RunContext) -> dict[str, Any]: ...
    def exercise_meta(self, case: dict[str, Any], targets: FrozenTargets, language_id_sequence_sha256: str, run_context: RunContext) -> dict[str, Any]: ...
    def exercise_focal(self, case: dict[str, Any], targets: FrozenTargets, language_id_sequence_sha256: str, focal_operation_route_map_sha256: str, run_context: RunContext) -> dict[str, Any]: ...


def load_module(path: Path, digest: str, module_name: str) -> Any:
    if path.is_symlink() or sha256(path.read_bytes()) != digest:
        raise RuntimeError(f"PROTECTED_INPUT: {module_name}")
    spec = importlib.util.spec_from_file_location(module_name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"PROTECTED_INPUT: {module_name} loader")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def load_p1349_suite() -> Any:
    path, digest = PINS["adversary_suite_p1349"]
    return load_module(path, digest, "p1349_frozen_adversary_baseline_for_p1350_r2")


def language_id_sequence() -> list[str]:
    ids = load_p1349_suite().exact_language_ids()
    if len(ids) != 122 or len(set(ids)) != 122 or any(not isinstance(item, str) or not item or not item.isascii() for item in ids):
        raise RuntimeError("PROTECTED_INPUT: LANGUAGE_ID sequence shape")
    if sha256(rfc8785(ids)) != LANGUAGE_ID_SEQUENCE_SHA256:
        raise RuntimeError("PROTECTED_INPUT: LANGUAGE_ID sequence digest")
    return list(ids)


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


def operation_ids_digest(ids: list[str]) -> str:
    return sha256(rfc8785(["p1350-focal-operation-ids-r2", ids]))


def boundary_manifest_digest(entries: list[list[str]]) -> str:
    return sha256(rfc8785(["p1350-boundary-attack-manifest-r2", entries]))


def boundary_attack(entry: list[str]) -> dict[str, Any]:
    attack_id, category, boundary, expected = entry
    nonce = secrets.token_bytes(32)
    challenge = secrets.token_bytes(32)
    return {
        "attack_id": attack_id,
        "category": category,
        "injection_boundary": boundary,
        "expected_blocker_code": expected,
        "experiment": {
            "attack_nonce": nonce.hex(),
            "attack_nonce_sha256": sha256(nonce),
            "challenge": challenge.hex(),
            "challenge_sha256": sha256(challenge),
            "fresh_process_required": True,
            "static_source_evidence_sufficient": False,
        },
    }


def exact_keys(value: Any, keys: list[str]) -> bool:
    return isinstance(value, dict) and sorted(value) == keys


def positive_int(value: Any) -> bool:
    return isinstance(value, int) and not isinstance(value, bool) and value > 0


def boundary_result_matches(attack: dict[str, Any], observed: dict[str, Any], seen: dict[str, set[Any]]) -> bool:
    """Recompute PASS; never accept a driver's bare status or expected label."""
    if not exact_keys(observed, ["attack_id", "closed_world", "dynamic_evidence", "expected_blocker_code", "observed_blocker", "schema", "status"]):
        return False
    if observed.get("schema") != "p1350-boundary-result-r2" or observed.get("status") != "PASS":
        return False
    if observed.get("attack_id") != attack["attack_id"] or observed.get("expected_blocker_code") != attack["expected_blocker_code"]:
        return False
    if observed.get("closed_world") is not True:
        return False
    forbidden = {"classification", "focal_correctly_violated", "meta_status", "mutation_score", "reason_code"}
    if forbidden.intersection(observed):
        return False

    evidence = observed.get("dynamic_evidence")
    if not exact_keys(evidence, ["attack_nonce_sha256", "case_journal_sha256", "cleanup", "executor", "injection", "supervisor"]):
        return False
    if evidence["attack_nonce_sha256"] != attack["experiment"]["attack_nonce_sha256"]:
        return False
    if not is_lower_sha256(evidence["case_journal_sha256"]):
        return False

    injection = evidence.get("injection")
    if not exact_keys(injection, ["boundary", "challenge_sha256", "observed", "oracle_owned"]):
        return False
    if injection != {
        "boundary": attack["injection_boundary"],
        "challenge_sha256": attack["experiment"]["challenge_sha256"],
        "observed": True,
        "oracle_owned": True,
    }:
        return False

    cleanup = evidence.get("cleanup")
    cleanup_keys = ["descriptors_closed", "group_absent", "patches_restored", "registry_destroyed", "shm_removed", "zombies_absent"]
    if not exact_keys(cleanup, cleanup_keys) or any(cleanup[key] is not True for key in cleanup_keys):
        return False

    supervisor = evidence.get("supervisor")
    supervisor_keys = ["attempt_finished_ns", "attempt_started_ns", "journal_fdatasync_observed", "parent_pid", "public_invocation_observed", "run_id"]
    if not exact_keys(supervisor, supervisor_keys):
        return False
    if not positive_int(supervisor["parent_pid"]) or not positive_int(supervisor["attempt_started_ns"]) or not positive_int(supervisor["attempt_finished_ns"]):
        return False
    if supervisor["attempt_finished_ns"] <= supervisor["attempt_started_ns"]:
        return False
    if supervisor["journal_fdatasync_observed"] is not True or supervisor["public_invocation_observed"] is not True:
        return False
    if not isinstance(supervisor["run_id"], str) or not supervisor["run_id"]:
        return False

    executor = evidence.get("executor")
    executor_keys = ["path_kind", "pgid", "pid", "sid", "starttime_ticks", "terminal_wait_observed"]
    if not exact_keys(executor, executor_keys) or executor["path_kind"] not in {"EXECUTOR", "PUBLIC_REFUSAL"}:
        return False
    if executor["terminal_wait_observed"] is not True:
        return False
    if executor["path_kind"] == "EXECUTOR":
        if not all(positive_int(executor[key]) for key in ["pid", "pgid", "sid", "starttime_ticks"]):
            return False
        if executor["sid"] != executor["pid"] or executor["pgid"] != executor["pid"]:
            return False
    elif any(executor[key] is not None for key in ["pid", "pgid", "sid", "starttime_ticks"]):
        return False

    blocker = observed.get("observed_blocker")
    if not isinstance(blocker, dict) or blocker.get("schema") != "p1350-integration-blocker-r1":
        return False
    if blocker.get("blocker_code") != attack["expected_blocker_code"] or forbidden.intersection(blocker):
        return False

    uniques = {
        "nonce": evidence["attack_nonce_sha256"],
        "challenge": injection["challenge_sha256"],
        "journal": evidence["case_journal_sha256"],
        "run": supervisor["run_id"],
    }
    if executor["path_kind"] == "EXECUTOR":
        uniques["executor"] = (executor["pid"], executor["starttime_ticks"])
    if any(value in seen[name] for name, value in uniques.items()):
        return False
    for name, value in uniques.items():
        seen[name].add(value)
    return True


def classification_matches(case: dict[str, Any], observed: dict[str, Any]) -> bool:
    expected_reason = case.get("expected_reason_code")
    return (
        observed.get("schema") == "p1349-classification-result-r1"
        and observed.get("case_id") == case["case_id"]
        and observed.get("classification") == case["expected_classification"]
        and (expected_reason is None or observed.get("reason_code") == expected_reason)
        and "meta_status" not in observed
        and "blocker_code" not in observed
        and "verdict" not in observed
    )


def meta_matches(case: dict[str, Any], observed: dict[str, Any]) -> bool:
    return (
        observed.get("schema") == "p1349-meta-result-r1"
        and observed.get("case_id") == case["case_id"]
        and observed.get("meta_status") == case["expected_meta_status"]
        and observed.get("meta_evidence") == case["expected_meta_evidence"]
        and not {"classification", "reason_code", "blocker_code", "verdict"}.intersection(observed)
    )


def validate_authorship() -> dict[str, Any]:
    for label, (path, digest) in PINS.items():
        if path.is_symlink() or sha256(path.read_bytes()) != digest:
            raise RuntimeError(f"PROTECTED_INPUT: {label}")

    spec = strict_json(PINS["contract_spec_p1350_r2"][0].read_bytes(), "P1350 R2 spec")
    binding = strict_json(PINS["contract_binding_p1350_r2"][0].read_bytes(), "P1350 R2 binding")
    receipt = strict_json(PINS["contract_receipt_p1350_r2"][0].read_bytes(), "P1350 R2 receipt")
    blocker = strict_json(PINS["adapter_blocker_p1350_r1"][0].read_bytes(), "P1350 R1 adapter blocker")
    baseline = load_p1349_suite()
    baseline_summary = baseline.validate_authorship()

    ids = language_id_sequence()
    slices = inherited_slices()
    expected_slices = {"meta": 2, "x14_and_supervisor_variants": 12, "other_p1348": 22, "p1347_regressions": 39, "p1346_replays": 181, "controls": 11}
    if {key: len(value) for key, value in slices.items()} != expected_slices:
        raise RuntimeError("PROTECTED_INPUT: inherited retest cardinalities")
    if baseline_summary["focal_valid_negatives_bound"] != 254 or baseline_summary["sequence_sha256"] != LANGUAGE_ID_SEQUENCE_SHA256:
        raise RuntimeError("PROTECTED_INPUT: semantic denominator or language sequence")

    derived_operations = sorted({item["adapter_operation"] for key, values in slices.items() if key != "meta" for item in values})
    local_operations = {item["adapter_operation"] for item in slices["x14_and_supervisor_variants"]}
    if derived_operations != FOCAL_OPERATION_IDS or len(local_operations) != 12 or len(set(FOCAL_OPERATION_IDS) - local_operations) != 48:
        raise RuntimeError("PROTECTED_INPUT: FOCAL_OPERATION allowlist")
    if operation_ids_digest(FOCAL_OPERATION_IDS) != FOCAL_OPERATION_IDS_SHA256:
        raise RuntimeError("PROTECTED_INPUT: FOCAL_OPERATION digest")

    if spec["boundary_attack_entries"] != BOUNDARY_ATTACK_ENTRIES:
        raise RuntimeError("PROTECTED_INPUT: boundary manifest entries")
    if boundary_manifest_digest(BOUNDARY_ATTACK_ENTRIES) != BOUNDARY_ATTACK_MANIFEST_SHA256:
        raise RuntimeError("PROTECTED_INPUT: BOUNDARY_ATTACK digest")
    categories = {name: sum(1 for entry in BOUNDARY_ATTACK_ENTRIES if entry[1] == name) for name in ["STATIC_BYPASS", "DYNAMIC_SUPERVISOR", "JOURNAL", "FRESHNESS"]}
    if categories != {"STATIC_BYPASS": 4, "DYNAMIC_SUPERVISOR": 21, "JOURNAL": 8, "FRESHNESS": 3}:
        raise RuntimeError("SCHEMA: boundary category cardinalities")

    domains = spec["identity_domains"]
    if domains["language_id_sequence"]["sequence_sha256"] != LANGUAGE_ID_SEQUENCE_SHA256:
        raise RuntimeError("PROTECTED_INPUT: contract language digest")
    if domains["boundary_attack_manifest"]["manifest_sha256"] != BOUNDARY_ATTACK_MANIFEST_SHA256:
        raise RuntimeError("PROTECTED_INPUT: contract boundary digest")
    if binding["domain_separation_binding"]["forbidden_parameter_names"] != ["exact_ids", "exact_ids_sha256"]:
        raise RuntimeError("SCHEMA: forbidden shared identity parameters")
    if receipt["domain_pins"]["shared_exact_ids_parameter_allowed"] is not False:
        raise RuntimeError("SCHEMA: receipt domain separation")
    if len(blocker["measured_incompatibilities"]) != 3 or blocker["verdict"] != "BLOCKED_NOT_EXECUTABLE":
        raise RuntimeError("PROTECTED_INPUT: measured R1 integration blocker")

    return {
        "authorship_cost": {"boundary_runs": 0, "candidate_runs": 0, "focal_runs": 0, "full_runs": 0, "meta_runs": 0, "ptrace_runs": 0, "target_processes": 0},
        "boundary_attack_manifest_sha256": BOUNDARY_ATTACK_MANIFEST_SHA256,
        "boundary_attacks_bound": len(BOUNDARY_ATTACK_ENTRIES),
        "boundary_category_counts": categories,
        "boundary_dynamic_evidence_required": True,
        "boundary_sequential_wall_ns_max": BOUNDARY_SEQUENTIAL_WALL_NS_MAX,
        "controls_bound": 11,
        "focal_operation_ids_sha256": FOCAL_OPERATION_IDS_SHA256,
        "focal_operations_bound": len(FOCAL_OPERATION_IDS),
        "focal_valid_negatives_bound": 254,
        "inherited_operations_bound": 48,
        "language_id_sequence_sha256": LANGUAGE_ID_SEQUENCE_SHA256,
        "language_ids_bound": len(ids),
        "local_operations_bound": 12,
        "meta_tests_bound": 2,
        "oracle_checker_adapter_r2_read_or_executed": False,
        "p1346_replays_bound": 181,
        "p1347_regressions_bound": 39,
        "r1_preserved": True,
        "shared_exact_ids_parameter_allowed": False,
        "verdict": "AUTHORED_NOT_EXECUTED",
    }


def run_with_driver(driver: VerifierDriver, targets: FrozenTargets, focal_operation_route_map_sha256: str) -> dict[str, Any]:
    """Future verifier entrypoint; unreachable from the authorship CLI."""
    validate_authorship()
    if not is_lower_sha256(focal_operation_route_map_sha256):
        raise RuntimeError("PROTECTED_INPUT: focal operation route-map digest")
    for path, digest in [
        (targets.checker, targets.checker_sha256),
        (targets.caller, targets.caller_sha256),
        (targets.supervisor, targets.supervisor_sha256),
        (targets.journal_parser, targets.journal_parser_sha256),
        (targets.execution_delivery, targets.execution_delivery_sha256),
    ]:
        if path.is_symlink() or sha256(path.read_bytes()) != digest:
            raise RuntimeError("AUTHORITY_ROOT: frozen target drift")

    context = driver.bind_verified_run(targets)
    result: dict[str, Any] = {"boundary": [], "boundary_failures": [], "full_runs": 0, "unexpected_integration_blockers": 0}
    seen: dict[str, set[Any]] = {"nonce": set(), "challenge": set(), "journal": set(), "run": set(), "executor": set()}
    for entry in BOUNDARY_ATTACK_ENTRIES:
        attack = boundary_attack(entry)
        observed = driver.exercise_boundary(attack, targets, BOUNDARY_ATTACK_MANIFEST_SHA256, context)
        result["boundary"].append(observed)
        if not boundary_result_matches(attack, observed, seen):
            result["boundary_failures"].append(attack["attack_id"])
            result["verdict"] = "BOUNDARY_TERMINAL_STOP"
            return result

    slices = inherited_slices()
    meta_records = [driver.exercise_meta(case, targets, LANGUAGE_ID_SEQUENCE_SHA256, context) for case in slices["meta"]]
    meta_failures = [case["case_id"] for case, observed in zip(slices["meta"], meta_records) if not meta_matches(case, observed)]
    result.update(meta=meta_records, meta_failures=meta_failures)
    if meta_failures:
        result["verdict"] = "META_TERMINAL_STOP"
        return result

    stages = [
        ("x14_and_supervisor_variants", slices["x14_and_supervisor_variants"]),
        ("other_p1348", slices["other_p1348"]),
        ("p1347_regressions", slices["p1347_regressions"]),
        ("p1346_replays", slices["p1346_replays"]),
        ("controls", slices["controls"]),
    ]
    for label, cases in stages:
        records = [
            driver.exercise_focal(case, targets, LANGUAGE_ID_SEQUENCE_SHA256, focal_operation_route_map_sha256, context)
            for case in cases
        ]
        failures = [case["case_id"] for case, observed in zip(cases, records) if not classification_matches(case, observed)]
        blockers = sum(1 for observed in records if observed.get("schema") == "p1350-integration-blocker-r1" or "blocker_code" in observed)
        result[label] = records
        result["unexpected_integration_blockers"] += blockers
        if failures or blockers:
            result["failures"] = failures
            result["verdict"] = "FOCAL_TERMINAL_STOP"
            return result
    result["verdict"] = "ZERO_SURVIVORS_PENDING_INDEPENDENT_COUNTING"
    return result


def main() -> int:
    if sys.argv[1:] != ["--author-check"]:
        raise RuntimeError("SCHEMA: authorship CLI accepts exactly --author-check")
    sys.stdout.buffer.write(canonical_line(validate_authorship()))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
